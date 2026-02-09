use mongodb::{Client, Collection};
use serde::{Deserialize, Serialize};

mod models;

use crate::models::events::Event;

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let uri: &str =
        "mongodb+srv://server:server123%40@cluster0.nuhlzny.mongodb.net/?appName=Cluster0";

    let client = Client::with_uri_str(uri).await?;

    let db = client.database("Calendar");
    let collection: Collection<Event> = db.collection("Events");

    let mut name = String::new();
    let mut event_type = String::new();

    println!("Enter event name:");
    std::io::stdin()
        .read_line(&mut name)
        .expect("Why can't you write name man... Its just a string- Thats embracing!");

    println!("Enter event type:");
    std::io::stdin()
        .read_line(&mut event_type)
        .expect("Why can't you write name man... Its just a string- Thats embracing!");

    Event::add_event(
        &collection,
        name.trim().to_string(),
        vec![2, 3],
        event_type.trim().to_string(),
    )
    .await?;

    Ok(())
}
