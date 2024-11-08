use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    sql, Error, Surreal, Value,
};

use crate::constants::connection::set_environment_variable;

#[derive(Debug)]
pub struct Database {
    pub client: Surreal<Client>,
    pub name_space: String,
    pub db_name: String,
}

impl Database {
    pub async fn init() -> Result<Self, Error> {
        let db_address: String = set_environment_variable("DB_ADDRESS", "0.0.0.0:8000");
        let db_ns: String = set_environment_variable("DB_NS", "yay_leads");
        let db_name: String = set_environment_variable("DB_NAME", "yay_crm");

        let client = Surreal::new::<Ws>(db_address).await?;

        client
            .signin(Root {
                username: &set_environment_variable("DB_USER", "yay_root"),
                password: &set_environment_variable("DB_PASSWORD", "yay_root"),
            })
            .await?;

        client.use_ns(&db_ns).use_db(&db_name).await.unwrap();

        Ok(Database {
            client,
            name_space: String::from(&db_ns),
            db_name: String::from(&db_name),
        })
    }

    // pub async fn check_and_create_db(client: &Surreal) -> surrealdb::Result<()> {
    //     let result: surrealdb::Result<Value> = client.query("SHOW TABLES").await;
    //     match result {
    //         Ok(tables) => {
    //             if let Value::Array(arr) = tables {
    //                 if arr.is_empty() {
    //                     println!("No tables found. Creating DB structure");
    //                     // Table structure
    //                     client.query("CREATE TABLE users;").await?;
    //                 } else {
    //                     println!("Database already exists with tables: {:?}", tables);
    //                 }
    //             } else {
    //             }
    //         }
    //         Err(err) => {
    //             println!("Error querying tables: {:?}", err);
    //             println!("Creating the database structure.");
    //             client.query("CREATE TABLE users;").await?;
    //         }
    //     }

    //     Ok(())
    // }
}
