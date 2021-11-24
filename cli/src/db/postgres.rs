#[allow(unused)]
use std::env;
use chrono::NaiveDate;
use diesel::{Connection, NullableExpressionMethods, PgConnection, QueryDsl, QueryResult};
use ordered_float::OrderedFloat;
use crate::{db_schema::{Activity, ActivityTag, Balance, BalanceWithId}, models::{AccountActivity, AccountBalance, StatsAmountPerMonthByTag, StatsDetailedAmountPerMonthByTag, tagging}};
use super::{DBActions, DBConfig};
use crate::diesel::RunQueryDsl; // Needed for .execute 
use diesel::prelude::*; //for .on and .eq in left_join(....on(...))


pub(crate) mod converters {
    use std::borrow::Borrow;

    use crate::db_schema::{Activity, ActivityTag, Balance};
    use crate::models::{AccountActivity, AccountBalance};
    use crate::models::tagging::ActivityToTags;
    

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
    
}

pub struct Postgres {
    conn: PgConnection,
}

impl DBActions for Postgres {
    fn clean_db(&self) -> anyhow::Result<()> {
        println!("Cleaning DB is done from Diesel DB migration commands");
        Ok(())
    }

    fn with_init_db_script(self, _: String) -> Self {
        println!("with_init_db_script: Init DB is done from Diesel DB migration commands");
        self
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
        println!("create_table: Init DB is done from Diesel DB migration commands");
        Ok(0)
    }

    fn insert_activities(&mut self,banking_statement: &[AccountActivity]) -> anyhow::Result<usize> {

        use crate::db_schema::activities;
        
        let converted: Vec<Activity> = banking_statement.into_iter().map(|a| a.into()).collect();

        let result: QueryResult<usize> = diesel::insert_into(activities::table)
            .values(converted)
            .on_conflict((activities::date, activities::statement, activities::amount))
            .do_nothing()
            .execute(&self.conn);         

        result.map_err(|err| anyhow::anyhow!(err))
    }

    fn insert_balance(&self, balance: AccountBalance) -> anyhow::Result<usize> {
        use crate::db_schema::balances;
        
        let converted: Balance = balance.into();

        let result: QueryResult<usize> = diesel::insert_into(balances::table)
            .values(&converted)
            .on_conflict((balances::date, balances::amount))
            .do_nothing()
            .execute(&self.conn);         

        result.map_err(|err| anyhow::anyhow!(err))
    }

    fn get_activities(&self) -> anyhow::Result<Vec<AccountActivity>> {
        use crate::db_schema::activities;
        use crate::db_schema::activity_tags;

        let activities = activities::table
            .left_join(activity_tags::table.on(activity_tags::activity_id.eq(activities::id)))
            .select((activities::id, activities::date, activities::statement, activities::amount, activity_tags::tags_pattern_id.nullable()))
            .load::<(i32, NaiveDate, String , f32, Option<i32>)>(&self.conn)?
            .into_iter()
            .map(|r| AccountActivity {
                row_id: Some(r.0 as u32),
                date: r.1,
                statement: r.2,
                amount: OrderedFloat(r.3 as f64),
                tag_pattern_id: r.4.map(|v| v as u8),
            })
            .collect();
        Ok(activities)
    }

    fn get_balance(&self) -> anyhow::Result<AccountBalance> {
        use crate::db_schema::balances;

        let balance = balances::table
            .select(balances::all_columns)
            .get_result::<BalanceWithId>(&self.conn)
            .map(|r| AccountBalance {
                row_id : Some(r.id as u32),
                date : r.date,
                balance_euro : OrderedFloat(r.amount as f64)
            })?;
        Ok(balance)
    }

    fn get_tag_patterns(&self) -> anyhow::Result<Vec<tagging::TagsPattern>> {
        use crate::db_schema::tags_patterns;
        use crate::db_schema::tags_pattern_to_tags;        
        use crate::db_schema::tags;

        let result: Vec<tagging::TagsPattern> = tags_pattern_to_tags::table
        .inner_join(tags_patterns::table.on(tags_patterns::id.eq(tags_pattern_to_tags::tags_pattern_id)))
        .inner_join(tags::table.on(tags::id.eq(tags_pattern_to_tags::tags_id)))        
        .select((tags_patterns::id, tags_patterns::tags_pattern, tags::tag))
        .order_by(tags_patterns::id.asc())
        .load::<(i32, String, String)>(&self.conn)?
        .into_iter()
        .map(|r| tagging::TagsPattern {
            id: r.0 as u8,
            pattern: r.1,
            tag: r.2,
        })
        .collect();

        Ok(result)
    }

    fn insert_activity_tags(
        &mut self,
        activity_tags: &[tagging::ActivityToTags],
    ) -> anyhow::Result<usize> {
        use crate::db_schema::activity_tags;
        
        let converted: Vec<ActivityTag> = activity_tags.into_iter().map(|a| a.into()).collect();

        let result: QueryResult<usize> = diesel::insert_into(activity_tags::table)
            .values(&converted)
            .on_conflict((activity_tags::activity_id, activity_tags::tags_pattern_id))
            .do_nothing()
            .execute(&self.conn);

        result.map_err(|err| anyhow::anyhow!(err))
    }

    fn get_stats_tag_per_month(
        &self,
        tags: &[String],
    ) -> anyhow::Result<Vec<StatsAmountPerMonthByTag>> {
        //use crate::db_schema::tags;
        //use crate::db_schema::tags_pattern_to_tags;

        // let result: Vec<i32> = tags_pattern_to_tags::table
        //     .left_join(tags::table.on(tags::id.eq(tags_pattern_to_tags::tags_id)))
        //     .group_by(tags_pattern_to_tags::tags_pattern_id)
        //     .filter(tags::tag.eq("LOYER"))
        //     .select(
        //         tags_pattern_to_tags::tags_pattern_id
        //     )            
        //     .load(&self.conn)?;


        // println!("{:?}", result);
        
        todo!()
    }

    fn get_stats_detailed_amount_per_month(
        &self,
        tags: &[String],
    ) -> anyhow::Result<Vec<StatsDetailedAmountPerMonthByTag>> {
        todo!()
    }
}
