use crate::models::events::Event;
use chrono::{Datelike, Local, NaiveDate};
use futures::TryStreamExt;
use mongodb::{
    Collection,
    bson::{DateTime, doc},
};

pub async fn current_event(collection: &Collection<Event>) -> mongodb::error::Result<()> {
    let date = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();
    let now = Local::now().time();
    let now_day = Local::now().weekday().number_from_sunday();

    let dt = date.and_time(now);
    let bson_dt = dt.and_utc().timestamp_millis();

    let current_time = DateTime::from_millis(bson_dt);

    let mut cursor = collection
        .find(doc! {
            "start_time": {"$lte": current_time},
            "end_time": {"$gte": current_time},
            "days": now_day,
        })
        .await?;

    while let Some(event) = cursor.try_next().await? {
        println!("{:#?}", event);
    }

    Ok(())
}
