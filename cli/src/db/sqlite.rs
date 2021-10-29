use itertools::Itertools;
use ordered_float::OrderedFloat;
use rusqlite::{Connection, named_params,params_from_iter};
use crate::models::{AccountActivity, AccountBalance, StatsAmountPerMonthByTag, tagging::{ActivityToTags, TagsPattern}};

use super::DBActions;


pub struct SqliteDB {
    pub conn: Connection,
}

impl SqliteDB {
    pub fn new(conn: Connection) -> Self {        
        Self { conn }
    }

    #[allow(unused)]
    pub fn close_cnx(self) -> anyhow::Result<()> {
        self.conn.close().map_err(|err| anyhow::anyhow!(err.1))
    }
}

impl DBActions for SqliteDB {
    
    fn create_table(&self) -> anyhow::Result<usize> {
        self.conn.execute(
            "CREATE TABLE activities (
                      date            DATE NOT NULL,
                      statement       TEXT NOT NULL,
                      amount          NUMERIC NOT NULL,
                      PRIMARY KEY ( date, statement, amount)
                      );",
            [],
        ).and_then(|_|self.conn.execute(
            "CREATE TABLE balance (
                      date            DATE NOT NULL,
                      amount          NUMERIC NOT NULL,
                      PRIMARY KEY ( date, amount)
                      );",
            [],
        )).and_then(|_|self.conn.execute(
            "CREATE TABLE tags (
                      id              INTEGER NOT NULL,
                      tag             TEXT NOT NULL
                      );",
            [],
        )).and_then(|_|self.conn.execute(
            "CREATE TABLE tags_pattern (
                      id              INTEGER NOT NULL,
                      tags_pattern      TEXT NOT NULL
                      );",
            [],
        )).and_then(|_|self.conn.execute(
            "CREATE TABLE tags_pattern_to_tags (
                      tags_pattern_id   INTEGER NOT NULL,
                      tags_id   INTEGER NOT NULL
                      );",
            [],
        )).and_then(|_|self.conn.execute(
            "CREATE TABLE activities_tags (
                      activity_id       INTEGER NOT NULL,
                      tags_pattern_id   INTEGER NOT NULL,
                      PRIMARY KEY ( activity_id, tags_pattern_id)                   
                      );",
            [],
        )).and_then(|_|self.conn.execute(
            "INSERT INTO tags (id, tag)
                VALUES 
                (1, 'EDF'),
                (2, 'FREEMOBILE'), 
                (3, 'LOYER'),
                (4, 'PARIS'),                
                (5, 'RETRAIT'),
                (6, 'VIREMENT_BANCAIRE')
                ;",
            [],
        )).and_then(|_|self.conn.execute(
            "INSERT INTO tags_pattern (id, tags_pattern)
                VALUES 
                (1, 'EDF'),                
                (2, 'VIREMENT'),
                (3, 'RETRAIT'),
                (4, 'LOYER'),
                (5, 'FREE MOBILE')
             ;",
            [],
        )).and_then(|_|self.conn.execute(
            "INSERT INTO tags_pattern_to_tags (tags_pattern_id, tags_id)
                VALUES 
                (1, 1),
                (2, 6),
                (3, 5),
                (4, 3), (4, 4),
                (5, 2), (5, 4)
                ;",
            [],
        )).map_err(|err| anyhow::anyhow!(err))
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

}



#[cfg(test)]
mod tests {

    use crate::{db::{DBActions, sqlite::SqliteDB}, models::{AccountActivity, AccountBalance, tagging::TagsPattern}};
    use ordered_float::OrderedFloat;
    use chrono::NaiveDate;
    use crate::db::tests::sqlite_connections::in_memory;

    fn create_db() -> anyhow::Result<SqliteDB> {
        let connection = in_memory()?;
        Ok(SqliteDB { conn : connection })
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
