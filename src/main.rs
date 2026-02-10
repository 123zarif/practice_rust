use dotenv::dotenv;
use mongodb::{Client, Collection};
use std::env;

mod add_event;
mod models;

use crate::add_event::add_event;
use crate::models::events::Event;

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    dotenv().ok();

    let api_uri = env::var("API_URI").expect("MONGODB_URI is not set in .env file");

    let client = Client::with_uri_str(api_uri).await?;

    let db = client.database("Calendar");
    let collection: Collection<Event> = db.collection("Events");

    let arg: Vec<String> = env::args().collect();
    let query: &String = &arg[1];

    if query == "add" {
        add_event(&collection).await?;
    }

    Ok(())
}
