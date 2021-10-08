mod mongodb;

#[tokio::main]
async fn main() {

    let db = match mongodb::get_mongo_db().await {
        Ok(database) => database,
        Err(err) => panic!("Error when getting DB: {:?}", err)
    };

    println!("db {:?}", db);
}
