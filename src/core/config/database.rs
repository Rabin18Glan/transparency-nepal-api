use surrealdb::engine::remote::ws::{Ws, Client};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

pub type Db = Surreal<Client>;

pub async fn init_surreal(url: &str) -> Db {
    let db = Surreal::new::<Ws>(url).await
        .expect("Failed to connect to SurrealDB");
    
    db.signin(Root {
        username: "root",
        password: "root_password", // TODO: Move to Env
    }).await.expect("Failed to sign in to SurrealDB");
    
    db.use_ns("gorkhas").use_db("transparency").await
        .expect("Failed to select SurrealDB namespace/database");

    tracing::info!("SurrealDB connection established");
    db
}
