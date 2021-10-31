use crate::models::tagging::ActivityToTags;
use crate::db::{ArcMutDB, DBActions};



/**
 * Get all activities, get all predefined tags and flag all activities from the tag pattern
 */
pub fn tagging<T: DBActions>(arc_db : ArcMutDB<T>) -> anyhow::Result<usize> {
    let mut sqlite_db = arc_db.lock().unwrap();
    let tags_patterns = sqlite_db.get_tag_patterns()?;
    let activities = sqlite_db.get_activities()?;
    let mut activity_tags: Vec<ActivityToTags> = Vec::new();

    for activity in activities {
        for p in tags_patterns.iter() {
            if  activity.statement.to_lowercase().contains(&p.pattern.to_lowercase()) || 
                activity.amount.to_string().contains(&p.pattern.to_lowercase()){ //some activities only can be tagged from amount
                activity_tags.push(ActivityToTags{ activity_id : activity.row_id.unwrap(), tags_pattern_id : p.id});
            }
        }
    }

    let result = sqlite_db.insert_activity_tags(&activity_tags)?;
   
    Ok(result)
}


#[test]
fn test() -> anyhow::Result<()> {
    use crate::db::tests::sqlite_connections::in_memory;
    use std::sync::{Arc, Mutex};
    use crate::actions::csv2db::csv2db;    
    use crate::db::sqlite::SqliteDB;

    let conn = in_memory()?;
    let sqlite_db = SqliteDB::new(conn, Some("./init-db.toml".to_string()));

    let arc_db = Arc::new(Mutex::new(sqlite_db));
    csv2db("./data/", arc_db.clone())?;
    tagging(arc_db.clone())?;

    let db = arc_db.lock().unwrap();
    let check_query: usize = db.conn
        .query_row("
            select count(1)
            from activities a
            left join activities_tags at on at.activity_id = a.rowid
            left join tags_pattern_to_tags tptt on tptt.tags_pattern_id = at.tags_pattern_id
            left join tags t on t.id = tptt.tags_id
            where t.tag = ?1
            ",
            [ "FREEMOBILE" ],
            |row| row.get(0)
        )?;

    assert!(check_query > 0 , "Expected tag 'FREEMOBILE' not found");

    Ok(())

}