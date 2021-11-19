use std::path::Path;

use itertools::Itertools;
use ordered_float::OrderedFloat;
use rusqlite::{Connection, OpenFlags, named_params, params_from_iter};
use crate::{db::InitTables, models::{AccountActivity, AccountBalance, StatsAmountPerMonthByTag, StatsDetailedAmountPerMonthByTag, tagging::{ActivityToTags, TagsPattern}}};
use super::{DBActions, DBConfig};


pub struct SqliteDB {
    conn: Connection,
    init_db_path: Option<String>,
}

impl SqliteDB {

    fn from_file<P: AsRef<Path>>(file_db: P) -> Self {
        let conn = 
            Connection::open_with_flags(
                                        file_db.as_ref(), 
                                        OpenFlags::default()
            )
            .map_err(|err| anyhow::anyhow!(err))
            .expect("Can not create DB as a file");
        Self {
            conn, init_db_path : None
        }
    }

    fn from_memory() -> Self {
        let conn = 
            Connection::open_in_memory()
            .map_err(|err| anyhow::anyhow!(err))
            .expect("Can not create DB in memory");
        Self {
            conn, init_db_path : None
        }
    }

    pub fn with_init_db_script(mut self, init_db_path: String) -> Self {
        self.init_db_path = Some(init_db_path);
        self
    }


    #[allow(unused)]
    pub fn close_cnx(self) -> anyhow::Result<()> {
        self.conn.close().map_err(|err| anyhow::anyhow!(err.1))
    }

    #[cfg(test)]
    pub fn connection(&self) -> &Connection {
        &self.conn
    }
}

impl DBActions for SqliteDB {

    fn from_config(conf: DBConfig) -> Self {
        match conf {
            DBConfig::File{ file_name } => SqliteDB::from_file(file_name),
            DBConfig::Memory => SqliteDB::from_memory(),
            DBConfig::RDBMS { .. } => unimplemented!("Not implemented for RDBMS yet")
        }
    }
    
    fn create_table(&self) -> anyhow::Result<usize> {
        let init_db_script = self.init_db_path.as_ref().ok_or(anyhow::anyhow!("Missing DB int script path"))?;
        let init_tables: InitTables = confy::load_path(init_db_script.as_str())?;

        self.conn
            .execute(init_tables.table_activities.as_str(),[],)
            .and_then(|_|
                self.conn.execute(init_tables.table_balance.as_str(),[],)
            )
            .and_then(|_|
                self.conn.execute(init_tables.table_tags.as_str(),[],)
            )
            .and_then(|_|
                self.conn.execute(init_tables.table_tags_pattern.as_str(),[],)
            )
            .and_then(|_|
                self.conn.execute(init_tables.table_tags_pattern_to_tags.as_str(),[],)
            )
            .and_then(|_|
                self.conn.execute(init_tables.table_activities_tags.as_str(),[],),
            )
            .and_then(|_|
                self.conn.execute(init_tables.predefined_tags.as_str(),[],),
            )
            .and_then(|_|
                self.conn.execute(init_tables.predefined_tags_pattern.as_str(),[],),
            )
            .and_then(|_|
                self.conn.execute(init_tables.predefined_tags_pattern_to_tags.as_str(),[],),
            )
            .map_err(|err| anyhow::anyhow!(err))
    }

    fn insert_activities(&mut self, banking_activites: &[AccountActivity]) -> anyhow::Result<usize> {
        let mut result : usize = 0;
        let tx = self.conn.transaction()?;
        {
            let mut stmt = tx.prepare("
                INSERT INTO activities (date, statement, amount) VALUES (:d, :s, :a) ON CONFLICT(date, statement, amount) DO NOTHING 
            ")?;

            for activity in banking_activites {            
                result += 
                    stmt.execute(
                        named_params! { ":d" : activity.date, ":s" : activity.statement, ":a" : activity.amount.to_string()}
                    )
                    .map_err(|err| anyhow::anyhow!(err))?;
            }
        }
        tx.commit()?;
        Ok(result)
    }

    fn insert_balance(&self, balance: AccountBalance) -> anyhow::Result<usize> {
        let mut stmt = self.conn.prepare("
            INSERT INTO balance (date, amount) VALUES (:d, :a) ON CONFLICT(date, amount) DO NOTHING 
        ")?;

        let result =
                stmt.execute(
                    named_params! { ":d" : balance.date, ":a" : balance.balance_euro.to_string()}
                )
                .map_err(|err| anyhow::anyhow!(err))?;    

        Ok(result)
    }

    fn get_activities(&self) -> anyhow::Result<Vec<AccountActivity>> {
        let mut stmt = self.conn.prepare("
        SELECT a.rowid, a.date, a.statement, a.amount, at.tags_pattern_id
        FROM activities a
        LEFT JOIN activities_tags at ON at.activity_id = a.rowid
        ORDER BY date DESC
        ")?;
        let activities = stmt.query_map([], |row|
            Ok(AccountActivity {
                row_id : row.get(0)?,
                date : row.get(1)?,
                statement : row.get(2)?,
                amount : row.get(3).map(|e| OrderedFloat(e))?,
                tag_pattern_id: row.get(4).unwrap_or(None)
            })
        )?;

        let mut result:Vec<AccountActivity> = Vec::new();
        for activity in activities {
            result.push(activity.unwrap());
        }
        Ok(result)
    }

    fn get_balance(&self) -> anyhow::Result<AccountBalance> {
        let mut stmt = self.conn.prepare("SELECT rowid, date, amount  FROM balance")?;
        let mut row = stmt.raw_query();
        let row = row.next()?.unwrap();

        Ok(AccountBalance {                
            row_id : row.get(0)?,
            date : row.get(1)?,
            balance_euro : row.get(2).map(|e| OrderedFloat(e))?,
        })
    }

    fn get_tag_patterns(&self) -> anyhow::Result<Vec<TagsPattern>> {
        let mut stmt = self.conn.prepare("
        SELECT tp.id, tp.tags_pattern, t.tag
        FROM tags_pattern tp, tags_pattern_to_tags tptt, tags t
        WHERE tp.id = tptt.tags_pattern_id and t.id = tptt.tags_id
        ")?;
        let mut rows = stmt.query([])?;
        let mut tags_patterns = Vec::new();
        while let Some(row) = rows.next()? {
            tags_patterns.push(TagsPattern {
                id: row.get(0)?, 
                pattern: row.get(1)?,
                tag: row.get(2)?
            });
        }    
        Ok(tags_patterns)
    }

    fn insert_activity_tags(&mut self, activity_tags: &[ActivityToTags]) -> anyhow::Result<usize> {
        let mut result : usize = 0;
        let tx = self.conn.transaction()?;
        {
            let mut stmt = tx.prepare("
                INSERT INTO activities_tags (activity_id, tags_pattern_id) VALUES (:aid, :tpid) ON CONFLICT(activity_id, tags_pattern_id) DO NOTHING 
            ")?;

            for activity_tag in activity_tags {            
                result += 
                    stmt.execute(
                        named_params! { ":aid" : activity_tag.activity_id, ":tpid" : activity_tag.tags_pattern_id}
                    )
                    .map_err(|err| anyhow::anyhow!(err))?;
            }
        }
        tx.commit()?;
        Ok(result)
    }

    fn get_stats_tag_per_month(&self, tags: &[String]) -> anyhow::Result<Vec<StatsAmountPerMonthByTag>> {
        let where_clause = tags
            .iter()
            .map(|_| " tag = ?")
            .join(" or ");

        let sql = format!("
        SELECT ABS(SUM(a.amount)), cast(strftime('%m', a.date) as integer)
        FROM activities a
        LEFT JOIN activities_tags at ON at.activity_id = a.rowid
        WHERE at.tags_pattern_id in (
            select tptt.tags_pattern_id
            from tags_pattern_to_tags tptt
            left join tags t on tptt.tags_id = t.id
            where {}
            group by tptt.tags_pattern_id
            HAVING COUNT(tags_pattern_id) = {}
        )
        group by strftime('%m-%Y', a.date)
        ORDER BY date ASC 
        ", where_clause, tags.len()) ;

        let mut stmt = self.conn.prepare(&sql)?;        
        let mut rows = stmt.query(params_from_iter(tags))?;
        let mut stats = Vec::new();
        while let Some(row) = rows.next()? {
            stats.push(StatsAmountPerMonthByTag {
                amount: row.get(0).map(|e| OrderedFloat(e))?,
                month: row.get(1)?
            });
        }    
        Ok(stats)
    }

    fn get_stats_detailed_amount_per_month(&self, tags: &[String]) -> anyhow::Result<Vec<StatsDetailedAmountPerMonthByTag>> {
        let inner_where_clause = tags
            .iter()
            .map(|_| " tag = ?")
            .join(" or ");

        let where_clause = tags
            .iter()
            .map(|_| " tag <> ?")
            .join(" or ");

        let sql = format!("
       SELECT t.tag, ABS(a.amount), cast(strftime('%m', a.date) as integer), cast(strftime('%Y%m', a.date) as integer)
       FROM activities a
       LEFT JOIN activities_tags at ON at.activity_id = a.rowid
       LEFT JOIN tags_pattern_to_tags tptt ON tptt.tags_pattern_id = at.tags_pattern_id
       LEFT JOIN tags t ON tptt.tags_id = t.id
       WHERE at.tags_pattern_id in (
           select distinct(tptt.tags_pattern_id)
           from tags_pattern_to_tags tptt
           left join tags t on tptt.tags_id = t.id
           where {}
           group by tptt.tags_pattern_id
       ) and {}
       ORDER BY date ASC 
        ", inner_where_clause, where_clause) ;

        let mut stmt = self.conn.prepare(&sql)?;
        let tags = [tags,tags].concat();
        let mut rows = stmt.query(params_from_iter(tags))?;
        let mut stats = Vec::new();
        while let Some(row) = rows.next()? {
            stats.push(StatsDetailedAmountPerMonthByTag {
                tag: row.get(0)?,
                amount: row.get(1).map(|e| OrderedFloat(e))?,
                month: row.get(2)?,
                month_year: row.get(3)?
            });
        }    
        Ok(stats)
    }

}



#[cfg(test)]
mod tests {

    use crate::{db::{DBActions, DBConfig, sqlite::SqliteDB}, models::{AccountActivity, AccountBalance, tagging::TagsPattern}};
    use ordered_float::OrderedFloat;
    use chrono::NaiveDate;

    fn create_db() -> anyhow::Result<SqliteDB> {
        Ok(
            SqliteDB::from_config(DBConfig::Memory)
            .with_init_db_script("./data/init-db-test.toml".to_string())
        )
    }

    #[test]
    fn test_balance() -> anyhow::Result<()> {

        let db = create_db()?;
        db.create_table()?;

        let balance = AccountBalance {
            row_id: None,
            balance_euro: OrderedFloat(132.23),
            date : NaiveDate::from_ymd(2021, 11, 12)
        };
        db.insert_balance(balance)?;

           
        let test_balance: f64 = db.conn
        .query_row(
            "SELECT amount FROM balance", 
            [],
            |row| row.get(0)
        )?;

        assert_eq!(test_balance, 132.23, "Wrong amount found in balance");

        db.close_cnx()?;

        Ok(())
    }

    #[test]
    fn test_activity() -> anyhow::Result<()> {

        let mut db = create_db()?;
        db.create_table()?;


        let mut activities = vec!();
        activities.push(AccountActivity {
            row_id: None,
            date: NaiveDate::from_ymd(2021, 11, 01),
            statement: "I BOUGHT THIS".to_string(),
            amount: OrderedFloat(102.32),
            tag_pattern_id: None
        });
        activities.push(AccountActivity {
            row_id: None,
            date: NaiveDate::from_ymd(2021, 11, 02),
            statement: "I BOUGHT THAT with 'VIREMENT'".to_string(),
            amount: OrderedFloat(15.68),
            tag_pattern_id: None
        });
        activities.push(AccountActivity {
            row_id: None,
            date: NaiveDate::from_ymd(2021, 11, 02),
            statement: "I BOUGHT THAT with 'VIREMENT'".to_string(),
            amount: OrderedFloat(15.68),
            tag_pattern_id: None
        });

        db.insert_activities(&activities)?;
        
        
        let test_activity: f64 = db.conn
            .query_row(
                "SELECT amount FROM activities WHERE statement LIKE ?1 ", 
                [ "%I BOUGHT THAT%" ],
                |row| row.get(0)
            )?;

        let test_activity_count: usize = db.conn
            .query_row(
                "SELECT COUNT(*) FROM activities", 
                [],
                |row| row.get(0)
            )?;

        assert_eq!(test_activity, 15.68, "Wrong amount found in activities");
        assert_eq!(test_activity_count, 2, "Wrong number of activities (count)");
        
        db.close_cnx()?;

        Ok(())
    }

    #[test]
    fn test_tags_pattern() -> anyhow::Result<()> {

        let db = create_db()?;
        db.create_table()?;
   
        let tags_pattern = db.get_tag_patterns()?;

        assert!(
            tags_pattern.contains(&TagsPattern { id : 5, pattern : "FREE MOBILE".to_string(), tag: "FREEMOBILE".to_string()}), 
            "Tag Pattern not found"
        );

        db.close_cnx()?;

        Ok(())

    }

  
}
