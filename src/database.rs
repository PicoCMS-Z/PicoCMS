//TODO:エラー処理を正しくやろう

use std::env;
use std::sync::Arc;

use mongodb::{Client, Database};

#[derive(Clone)]
pub struct Db(pub(crate) Arc<Database>);

impl Db {
    pub async fn new() -> Db {
        let uri = env::var("MONGODB_URL").expect("MONGODB_URL is undefined.");
        let client = Client::with_uri_str(&uri)
            .await
            .expect("Could not connect to MongoDB.");
        let db = client.database("dev");
        Db(Arc::new(db))
    }
}