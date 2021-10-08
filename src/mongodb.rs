use mongodb::{Database, Client, options::ClientOptions};

pub async fn get_mongo_db () -> Result<Database, mongodb::error::Error> {
  let client_options = ClientOptions::parse(
      "mongodb://localhost:27017"
  ).await?;

  let client = Client::with_options(client_options)?;

  let db = client.database("pokemon");

  Ok(db)
}