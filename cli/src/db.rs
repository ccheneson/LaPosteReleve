pub mod sqlite;

use std::sync::{Arc, Mutex};
use crate::models::{AccountActivity, AccountBalance, StatsAmountPerMonthByTag, StatsDetailedAmountPerMonthByTag, tagging::{ActivityToTags, TagsPattern}};
pub type ArcMutDB<T> = Arc<Mutex<T>>;


pub mod sqlite_connections {
    use std::path::Path;
    use rusqlite::{Connection, OpenFlags};

    pub fn remove_db_if_exist<P: AsRef<Path>>(db_path : P) -> Result<(), anyhow::Error> {  
        if db_path.as_ref().exists() {
            std::fs::remove_file(db_path)?;
        }
        Ok(())
    }

   
    pub fn from_file<P : AsRef<Path>>(file_db: P) -> anyhow::Result<Connection> {
        Connection::open_with_flags(file_db.as_ref(), OpenFlags::default())
        .map_err(|err| anyhow::anyhow!(err))
    }
}


pub trait DBActions {
    fn create_table(&self) -> anyhow::Result<usize>;
    fn insert_activities(&mut self,banking_statement: &[AccountActivity]) -> anyhow::Result<usize>;
    fn insert_balance(&self,balance: AccountBalance) -> anyhow::Result<usize>;
    fn get_activities(&self) -> anyhow::Result<Vec<AccountActivity>>;
    fn get_balance(&self) -> anyhow::Result<AccountBalance>;
    fn get_tag_patterns(&self) -> anyhow::Result<Vec<TagsPattern>>;
    fn insert_activity_tags(&mut self, activity_tags: &[ActivityToTags]) -> anyhow::Result<usize>;
    fn get_stats_tag_per_month(&self, tags: &[String]) -> anyhow::Result<Vec<StatsAmountPerMonthByTag>>;
    fn get_stats_detailed_amount_per_month(&self, tags: &[String]) -> anyhow::Result<Vec<StatsDetailedAmountPerMonthByTag>>;
}



#[cfg(test)]
pub mod tests {

    pub mod sqlite_connections {
        use rusqlite::Connection;
    
        pub fn in_memory() -> anyhow::Result<Connection> {
            Connection::open_in_memory()
            .map_err(|err| anyhow::anyhow!(err))        
        }
    }
    
}
