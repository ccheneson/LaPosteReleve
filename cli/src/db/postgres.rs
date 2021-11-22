use std::env;

use chrono::NaiveDate;
use diesel::{Connection, PgConnection, QueryDsl, QueryResult, query_dsl::methods::FilterDsl};
use crate::{db::postgres::postgres_models::{Activity, ActivityTag, ActivityTagWithId, ActivityWithId, Balance, TagsPattern, TagsPatternToTag}, models::*};
use super::{DBActions, DBConfig};
use crate::diesel::RunQueryDsl; // Needed for .execute 
use crate::diesel::BelongingToDsl; // Needed for .belonging_to 


pub(crate) mod postgres_models {
    use diesel::Insertable;

    pub(crate) mod schema {       
        table! {
            activities {
                id -> Integer,
                date -> Date,
                statement -> Text,
                amount -> Float4,
            }
        }
        table! {
            balances {
                id -> Integer,
                date -> Date,
                amount -> Float4,
            }
        }
        table! {
            tags {
                id -> Integer,
                tag -> Text,
            }
        }
        table! {
            tags_patterns {
                id -> Integer,
                tags_pattern -> Text,
            }
        }
        table! {
            tags_pattern_to_tags {
                id -> Integer,
                tags_pattern_id -> Integer,
                tags_id -> Integer,
            }
        }
        table! {
            activity_tags {
                id -> Integer,
                activity_id -> Integer,
                tags_pattern_id -> Integer,
            }
        }

        

        joinable!(tags_pattern_to_tags -> activity_tags(tags_pattern_id));
        allow_tables_to_appear_in_same_query!(tags_pattern_to_tags, activity_tags);
        


    }

    use self::schema::*;
    use chrono::NaiveDate;


    #[derive(Insertable, Debug)]
    #[table_name="activities"]
    pub struct Activity {
        pub date: NaiveDate,
        pub statement: String,
        pub amount: f32,
    }

    #[derive(Identifiable, Queryable, Debug)]
    #[table_name="activities"]
    pub struct ActivityWithId {
        pub id: i32,
        pub date: NaiveDate,
        pub statement: String,
        pub amount: f32,
    }
    
    #[derive(Queryable, Insertable)]
    pub struct Balance {
        pub date: NaiveDate,
        pub amount: f32,
    }

    #[derive(Queryable, Insertable)]
    #[table_name="balances"]
    pub struct BalanceWithId {
        pub id: i32,
        pub date: NaiveDate,
        pub amount: f32,
    }



    #[derive(Queryable, Associations, Debug)]
    #[table_name="tags_patterns"]
    pub struct TagsPattern {
        pub id: i32,
        pub tags_pattern: String,
    }

    #[derive(Queryable, Debug, Associations, Identifiable)]
    #[belongs_to(ActivityTagWithId, foreign_key="tags_pattern_id")]
    pub struct TagsPatternToTag {
        pub id: i32,
        pub tags_pattern_id: i32,
        pub tags_id: i32,
    }

    #[derive(Queryable)]
    pub struct Tag {
        pub id: i32,
        pub tag: String,
    }


    //#[belongs_to(parent = ActivityWithId, foreign_key="activity_id")]
    #[derive(Identifiable, Queryable, PartialEq, Associations, Debug)]
    #[belongs_to(ActivityWithId, foreign_key="activity_id")]
    #[table_name="activity_tags"]    
    pub struct ActivityTagWithId {
        pub id: i32,
        pub activity_id: i32,
        pub tags_pattern_id: i32,
    }

    #[derive(Insertable)]
    #[table_name="activity_tags"]
    pub struct ActivityTag {
        pub activity_id: i32,
        pub tags_pattern_id: i32,
    }

}


pub(crate) mod converters {
    use std::borrow::Borrow;

    use ordered_float::OrderedFloat;

    use crate::models::{AccountActivity, AccountBalance};
    use crate::models::tagging::ActivityToTags;
    
    use super::postgres_models::*;

    impl<T: Borrow<AccountActivity>> From<T> for Activity {
        fn from(a: T) -> Self {
            Self {
                date: a.borrow().date,
                statement: a.borrow().statement.to_string(),
                amount: *a.borrow().amount as f32,
            }
        }
    }

    impl<T: Borrow<AccountBalance>> From<T> for Balance {
        fn from(a: T) -> Self {
            Self {
                date: a.borrow().date,
                amount: *a.borrow().balance_euro as f32,
            }
        }
    }

    impl<T: Borrow<ActivityToTags>> From<T> for ActivityTag {
        fn from(a: T) -> Self {
            Self {
                activity_id: a.borrow().activity_id as i32,
                tags_pattern_id: a.borrow().tags_pattern_id as i32,
            }
        }
    }

    impl From<ActivityWithId> for AccountActivity {
        fn from(a: ActivityWithId) -> Self {
            // Self {
            //     row_id: Some(a.id as u32),
            //     date: a.date,
            //     statement: a.statement,
            //     amount : a.amount,
            //     tag_pattern_id: unimplemented!("")
            // }
            unimplemented!("")
        }
    }
    
}

pub struct Postgres {
    conn: PgConnection,
}

impl DBActions for Postgres {
    fn clean_db(&self) -> anyhow::Result<()> {
        unimplemented!("Cleaning DB is done from Diesel DB migration commands")
    }

    fn with_init_db_script(self, _: String) -> Self {
        unimplemented!("with_init_db_script: Init DB is done from Diesel DB migration commands")
    }

    fn from_config(conf: DBConfig) -> Self {
        match conf {
            DBConfig::RDBMS { url } => {
                let db_url = url.unwrap_or_else(||
                    env::var("DATABASE_URL").expect("Database URL unavailable from config nor env")
                );

                let conn = PgConnection::establish(db_url.as_str())
                    .expect(&format!("Error connecting to Postgres DB {}", db_url.as_str()));
                Postgres { conn }
            },
            _ => unimplemented!("{:?}", format!("DBConfig {} not implemented", conf).as_str())
        }
    }

    fn create_table(&self) -> anyhow::Result<usize> {
        unimplemented!("create_table: Init DB is done from Diesel DB migration commands")
    }

    fn insert_activities(
        &mut self,
        banking_statement: &[AccountActivity],
    ) -> anyhow::Result<usize> {

        use postgres_models::schema::activities;
        
        let converted: Vec<Activity> = banking_statement.into_iter().map(|a| a.into()).collect();

        let result: QueryResult<usize> = diesel::insert_into(activities::table).values(converted).execute(&self.conn);         

        result.map_err(|err| anyhow::anyhow!(err))
    }

    fn insert_balance(&self, balance: AccountBalance) -> anyhow::Result<usize> {
        use postgres_models::schema::balances;
        
        let converted: Balance = balance.into();

        let result: QueryResult<usize> = diesel::insert_into(balances::table).values(&converted).execute(&self.conn);         

        result.map_err(|err| anyhow::anyhow!(err))
    }

    fn get_activities(&self) -> anyhow::Result<Vec<AccountActivity>> {
        use postgres_models::schema::activities::*;
        use postgres_models::schema::activities;
        use postgres_models::schema::tags_pattern_to_tags;
        use postgres_models::schema::tags_pattern_to_tags::*;
        use postgres_models::schema::activity_tags::*;
        use postgres_models::schema::activity_tags;


        
        
        let activities: Vec<ActivityWithId>  = activities::table.select(activities::all_columns).load(&self.conn).unwrap();
        let a = activities.get(0).unwrap();
        
        // let activity_tags:Vec<ActivityTagWithId> =                 
        // ActivityTagWithId::belonging_to(&activities)        
        //     .select(activity_tags::all_columns)
        //     .load(&self.conn).unwrap();

        // let activity_tags:Vec<TagsPatternToTag> =                 
        //     ActivityTagWithId::belonging_to(&activities)
        //         .inner_join(tags_pattern_to_tags::table)
        //         .select(tags_pattern_to_tags::all_columns)
        //         .load(&self.conn).unwrap();

        //(i32,i32,i32),(i32,i32,i32)
        let activity2_tags:Vec<(i32,i32,i32,i32,i32,i32)> =
                ActivityTagWithId::belonging_to(&activities)
                    .inner_join(tags_pattern_to_tags::table)
                    .select((
                        activity_tags::id, 
                        activity_tags::activity_id, 
                        activity_tags::tags_pattern_id, 
                        tags_pattern_to_tags::id,
                        tags_pattern_to_tags::tags_pattern_id,
                        tags_pattern_to_tags::tags_id))
                    .load(&self.conn).unwrap();



            //tags_pattern_to_tags::table::select(self, selection)

        //let tags_id:Vec<ActivityTagWithId> = TagsPattern::belonging_to(&activity_tags).load(&self.conn).unwrap();

        println!("{:?}", activity2_tags);        
        //println!("{:?}", activity_tags);

        

        Ok(vec!())
    }

    fn get_balance(&self) -> anyhow::Result<AccountBalance> {
        todo!()
    }

    fn get_tag_patterns(&self) -> anyhow::Result<Vec<tagging::TagsPattern>> {
        todo!()
    }

    fn insert_activity_tags(
        &mut self,
        activity_tags: &[tagging::ActivityToTags],
    ) -> anyhow::Result<usize> {
        use postgres_models::schema::activity_tags;
        
        let converted: Vec<ActivityTag> = activity_tags.into_iter().map(|a| a.into()).collect();

        let result: QueryResult<usize> = diesel::insert_into(activity_tags::table).values(&converted).execute(&self.conn);         

        result.map_err(|err| anyhow::anyhow!(err))
    }

    fn get_stats_tag_per_month(
        &self,
        tags: &[String],
    ) -> anyhow::Result<Vec<StatsAmountPerMonthByTag>> {
        todo!()
    }

    fn get_stats_detailed_amount_per_month(
        &self,
        tags: &[String],
    ) -> anyhow::Result<Vec<StatsDetailedAmountPerMonthByTag>> {
        todo!()
    }
}
