use chrono::NaiveDate;
use ordered_float::OrderedFloat;
use serde::Serialize;
use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Debug, Serialize)]
pub struct AccountActivity {
    pub row_id: Option<u32>,
    pub date: NaiveDate,
    pub statement: String,
    pub amount: OrderedFloat<f64>,
    pub tag_pattern_id: Option<u8>,
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct AccountBalance {
    pub row_id: Option<u32>,
    pub date: NaiveDate,
    pub balance_euro: OrderedFloat<f64>,
}

#[derive(Debug)]
pub struct BankingStatement {
    pub row_id: Option<u32>,
    pub balance: AccountBalance,
    pub activities: HashSet<AccountActivity>,
}

#[derive(Serialize, Debug)]
pub struct StatsAmountPerMonthByTag {
    pub amount: OrderedFloat<f64>,
    pub month: u8
}

#[derive(Serialize, Eq, PartialEq, Hash)]
pub struct StatsDetailedAmountPerMonthByTag {
    pub tag: String,
    pub amount: OrderedFloat<f64>,
    pub month: u32,
    pub month_year: u32
}

pub mod tagging {
    use serde::Serialize;

    #[derive(PartialEq, Serialize, Debug)]
    pub struct TagsPattern {
        pub id: u8,
        pub pattern: String,
        pub tag: String,
    }

    pub struct ActivityToTags {
        pub activity_id: u32,
        pub tags_pattern_id: u8,
    }
}
