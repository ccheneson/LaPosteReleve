use chrono::NaiveDate;
use diesel::Insertable;

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

//needed for the get_activities query
allow_tables_to_appear_in_same_query!(activities, activity_tags);

//needed for the get_tag_patterns query
allow_tables_to_appear_in_same_query!(tags, tags_patterns, tags_pattern_to_tags);

#[derive(Insertable, Debug)]
#[table_name = "activities"]
pub struct Activity {
    pub date: NaiveDate,
    pub statement: String,
    pub amount: f32,
}

#[derive(Identifiable, Queryable, Debug)]
#[table_name = "activities"]
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
#[table_name = "balances"]
pub struct BalanceWithId {
    pub id: i32,
    pub date: NaiveDate,
    pub amount: f32,
}

#[derive(Queryable, Associations, Identifiable, Debug)]
#[table_name = "tags_patterns"]
pub struct TagsPattern {
    pub id: i32,
    pub tags_pattern: String,
}

#[derive(Queryable, Debug, Associations, Identifiable)]
#[table_name = "tags_pattern_to_tags"]
pub struct TagsPatternToTag {
    pub id: i32,
    pub tags_pattern_id: i32,
    pub tags_id: i32,
}

#[derive(Identifiable, Queryable, Associations)]
#[table_name = "tags"]
pub struct Tag {
    pub id: i32,
    pub tag: String,
}

#[derive(Identifiable, Queryable, PartialEq, Associations, Debug)]
#[table_name = "activity_tags"]
pub struct ActivityTagWithId {
    pub id: i32,
    pub activity_id: i32,
    pub tags_pattern_id: i32,
}

#[derive(Insertable, Queryable)]
#[table_name = "activity_tags"]
pub struct ActivityTag {
    pub activity_id: i32,
    pub tags_pattern_id: i32,
}
