use serde_json::json;
use surrealdb::engine::remote::ws::{Ws,Client};
use surrealdb::opt::auth::Root;
use surrealdb::{sql, Response};
use surrealdb::sql::Thing;
use surrealdb::Error;
use surrealdb::Surreal;
use serde::{Deserialize,Serialize};

#[derive(Serialize,Deserialize)]
pub struct  User{
    userid: String,
    address: String,
}

#[tokio::main]
async fn main() -> Result<Surreal<Client>, Error> {
    let db = Surreal::new::<Ws>("localhost:8000").await?;

    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;
    db.use_ns("kiimart").use_db("kiimart").await?;
    Ok(db)
}

pub async fn add_user_data(db:Surreal<Client>,username:String,address:String) -> Result<(),Error> {
    let user: Vec<User> = db
        .create("user")
        .content(User {
            userid: username.into(),
            address:address.into()
        })
        .await?;
    Ok(())
}

pub async fn fetch_user_data(db:Surreal<Client>,username:String) -> Result<Response,Error> {
    let people: Vec<User> = db.select("person").await.expect("REASON");
    let query = r#"
        SELECT address
        FROM type::table($table)
        where userid = $userid
    "#;
    let user = db.query(query).bind(("table", "person")).bind(("userid", username)).await?;
    println!("user: {:?}", user);
    Ok(user)
}
