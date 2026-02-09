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
    // pub start_time: DateTime,
    // pub end_time: DateTime,
    pub days: Vec<u8>,
    pub event_type: String,
}

impl Event {
    pub async fn add_event(
        collection: &Collection<Event>,
        name: String,
        // start_time: DateTime,
        // end_time: DateTime,
        days: Vec<u8>,
        event_type: String,
    ) -> mongodb::error::Result<InsertOneResult> {
        let new_event = Event {
            id: None,
            name: name.clone(),
            // start_time,
            // end_time,
            days,
            event_type,
        };

        println!("Tried Adding {}", name);

        collection.insert_one(new_event).await
    }
}
