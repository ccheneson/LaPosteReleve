pub mod sqlite;

use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};
use crate::models::{AccountActivity, AccountBalance, StatsAmountPerMonthByTag, StatsDetailedAmountPerMonthByTag, tagging::{ActivityToTags, TagsPattern}};


pub type ArcMutDB<T> = Arc<Mutex<T>>;

pub mod utils {
    use std::path::Path;

    pub fn remove_db_if_exist<P: AsRef<Path>>(db_path : P) -> Result<(), anyhow::Error> {
        if db_path.as_ref().exists() {
            std::fs::remove_file(db_path)?;
        }
        Ok(())
    }

}

#[allow(unused)]
pub enum DBConfig {
    File { file_name: String},
    FileWithOverwrite { file_name: String},
    Memory,
    RDBMS {
        user: String,
        password: String,
        url: String   
    }
}

pub trait DBActions {
    fn with_init_db_script(self, init_db_path: String) -> Self;
    fn clean_db(&self) -> anyhow::Result<()>;
    fn from_config(conf: DBConfig) -> Self;
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



#[derive(Default, Debug, Serialize, Deserialize)]
struct InitTables {
    table_activities: String,
    table_balance: String,
    table_tags: String,
    table_tags_pattern: String,
    table_tags_pattern_to_tags: String,
    table_activities_tags: String,
    predefined_tags: String,
    predefined_tags_pattern: String,
    predefined_tags_pattern_to_tags: String
}
