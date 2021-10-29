use crate::actions::utils::group_by;
use crate::db::{ArcMutDB, DBActions};
use crate::errors::Errors;
use crate::models::tagging::TagsPattern;
use crate::models::{
    AccountActivity, AccountBalance, StatsAmountPerMonthByTag,
};

use chrono::{Datelike, NaiveDate};
use itertools::Itertools;
use ordered_float::OrderedFloat;
use serde::Serialize;


#[derive(Serialize)]
struct BalanceWWW<'a> {
    date: &'a NaiveDate,
    amount: &'a OrderedFloat<f64>,
}

#[derive(Serialize)]
struct AccountActivityWWW {
    month_index: u32,
    stats: AmountStatsWWW,
    activities: Vec<AccountActivity>,
}

#[derive(Serialize)]
struct AmountStatsWWW {
    amount_plus: OrderedFloat<f64>,
    amount_minus: OrderedFloat<f64>,
}

#[derive(Serialize)]
pub struct StatsAmountPerMonthByTagWWW<'a> {
    pub tags: &'a Vec<String>,
    pub data: &'a Vec<StatsAmountPerMonthByTag>,
}

#[derive(Serialize)]
pub struct StatsDetailedAmountPerMonthByTagWWW<'a> {
    pub labels: &'a Vec<u32>,
    pub data: &'a Vec<StatsDetailedWWW<'a>>,
}

#[derive(Serialize)]
pub struct StatsDetailedWWW<'a> {
    pub label: &'a str,
    pub data: Vec<OrderedFloat<f64>>,
}

#[derive(Serialize, Debug)]
pub struct TagsPatternWWW<'a> {
    pub pattern: &'a str,
    pub tags: Vec<&'a str>,
}

impl AmountStatsWWW {
    fn new() -> Self {
        Self {
            amount_plus: OrderedFloat(0.00),
            amount_minus: OrderedFloat(0.00),
        }
    }
    fn add_to_amount_plus(&mut self, value: OrderedFloat<f64>) {
        self.amount_plus = self.amount_plus + value;
    }
    fn add_to_amount_minus(&mut self, value: OrderedFloat<f64>) {
        self.amount_minus = self.amount_minus + value;
    }
}

pub async fn get_activities<T: DBActions>(
    db: ArcMutDB<T>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let activities: Vec<AccountActivity> = db
        .lock()
        .unwrap()
        .get_activities()
        .map_err(|err| Errors::DBError(err))?;

    //Group all activities per month
    let activities_by_month = activities.into_iter().group_by(|a| a.date.month());

    //Collect amount stats and activities per month
    let result = activities_by_month
        .into_iter()
        .map(|(month, group)| {
            let mut amounts = AmountStatsWWW::new();
            let mut account_activities: Vec<AccountActivity> = Vec::new();

            group.into_iter().for_each(|e| {
                if e.amount.ge(&OrderedFloat(0.0)) {
                    amounts.add_to_amount_plus(e.amount);
                } else {
                    amounts.add_to_amount_minus(e.amount);
                }
                account_activities.push(e);
            });

            AccountActivityWWW {
                month_index: month,
                stats: amounts,
                activities: account_activities,
            }
        })
        .collect::<Vec<AccountActivityWWW>>();
    Ok(warp::reply::json(&result))
}

pub async fn get_balance<T: DBActions>(
    db: ArcMutDB<T>,
) -> Result<impl warp::Reply, warp::Rejection> {
    // https://github.com/dtolnay/anyhow/issues/81#issuecomment-609171265. for lock().unwrap()
    let account_balance: AccountBalance = db
        .lock()
        .unwrap()
        .get_balance()
        .map_err(|err| Errors::DBError(err))?;
    let result = BalanceWWW {
        date: &account_balance.date,
        amount: &account_balance.balance_euro,
    };

    Ok(warp::reply::json(&result))
}

/**
 * Get the tag pattern ids and their associated tags text/pattern
 */
pub async fn get_tags<T: DBActions>(db: ArcMutDB<T>) -> Result<impl warp::Reply, warp::Rejection> {
    let tags_pattern: Vec<TagsPattern> = db
        .lock()
        .unwrap()
        .get_tag_patterns()
        .map_err(|err| Errors::DBError(err))?;


    let tags_pattern_grouped = group_by(
        &tags_pattern,
        |e: &TagsPattern| &e.id,
        |e: &TagsPattern| e.tag.as_str(),
    );

    Ok(warp::reply::json(&tags_pattern_grouped))
}

/**
 * Get stats by tag text
 */
pub async fn get_stats_tag_per_month<T: DBActions>(
    db: ArcMutDB<T>,
    tags: Vec<String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let stats = db
        .lock()
        .unwrap()
        .get_stats_tag_per_month(&tags)
        .map_err(|err| Errors::DBError(err))?;

    let result = StatsAmountPerMonthByTagWWW {
        tags: &tags,
        data: &stats,
    };

    Ok(warp::reply::json(&result))
}

/**
 * Get the list of all tag pattern and their associated tag text
 */
pub async fn get_tags_pattern<T: DBActions>(
    db: ArcMutDB<T>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let tags_pattern: Vec<TagsPattern> = db
        .lock()
        .unwrap()
        .get_tag_patterns()
        .map_err(|err| Errors::DBError(err))?;

    let grouped = group_by(
        &tags_pattern,
        |e: &TagsPattern| e.pattern.as_str(),
        |e: &TagsPattern| e.tag.as_str(),
    );

    let tags_pattern_grouped: Vec<TagsPatternWWW> = grouped
        .into_iter()
        .map(|e| TagsPatternWWW {
            pattern: e.0,
            tags: e.1,
        })
        .collect();

    Ok(warp::reply::json(&tags_pattern_grouped))
}

