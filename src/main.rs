use mongodb::{
    Client, Collection,
    bson::{Document, doc},
};

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let uri = "mongodb+srv://server:server123%40@cluster0.nuhlzny.mongodb.net/?appName=Cluster0";

    let client = Client::with_uri_str(uri).await?;

    let db = client.database("Calendar");
    let cols: Collection<Document> = db.collection("Events");

    let mut events = cols.find(doc! {}).await?;

    while let Some(event) = events.try_next().await? {
        print!("{:#?}", event)
    }

    Ok(())
}
