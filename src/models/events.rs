use chrono::{NaiveDate, NaiveTime};
use mongodb::{
    Collection,
    bson::{DateTime, oid::ObjectId},
    results::InsertOneResult,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub start_time: DateTime,
    pub end_time: DateTime,
    pub days: Vec<u8>,
    pub event_type: String,
}

impl Event {
    pub async fn add_event(
        collection: &Collection<Event>,
        name: String,
        start_time: String,
        end_time: String,
        days: Vec<u8>,
        event_type: String,
    ) -> mongodb::error::Result<InsertOneResult> {
        let st = NaiveTime::parse_from_str(&start_time, "%H:%M").expect("Error Parsing time!");
        let et = NaiveTime::parse_from_str(&end_time, "%H:%M").expect("Error parsing Time!");

        let date = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();

        let start_dt = date.and_time(st);
        let start_ms = start_dt.and_utc().timestamp_millis();

        let end_dt = date.and_time(et);
        let end_ms = end_dt.and_utc().timestamp_millis();

        let start_bson_time = DateTime::from_millis(start_ms);
        let end_bson_time = DateTime::from_millis(end_ms);

        let new_event = Event {
            id: None,
            name: name.clone(),
            start_time: start_bson_time,
            end_time: end_bson_time,
            days,
            event_type,
        };

        collection.insert_one(new_event).await
    }
}
