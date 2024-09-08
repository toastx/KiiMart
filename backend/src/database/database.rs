use serde_json::json;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::sql;
use surrealdb::sql::Thing;
use surrealdb::Error;
use surrealdb::Surreal;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let db = Surreal::new::<Ws>("localhost:8000").await?;

    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    // Select a specific namespace and database
    db.use_ns("namespace").use_db("database").await?;

    // Create a new person with a random ID
    let tobie: Vec<Person> = db
        .create("person")
        .content(Person {
            id: None,
            title: "Founder & CEO".into(),
            name: Name {
                first: "Tobie".into(),
                last: "Morgan Hitchcock".into(),
            },
            marketing: true,
        })
        .await?;

    // Create a new person with a specific ID
    let mut jaime: Option<Person> = db
        .create(("person", "jaime"))
        .content(Person {
            id: None,
            title: "Founder & COO".into(),
            name: Name {
                first: "Jaime".into(),
                last: "Morgan Hitchcock".into(),
            },
            marketing: false,
        })
        .await?;

    // Update a person record with a specific ID
    jaime = db
        .update(("person", "jaime"))
        .merge(json!({ "marketing": true }))
        .await?;

    // Select all people records
    let people: Vec<Person> = db.select("person").await?;

    // Perform a custom advanced query
    let query = r#"
        SELECT marketing, count()
        FROM type::table($table)
        GROUP BY marketing
    "#;

    let groups = db.query(sql).bind(("table", "person")).await?;

    // Delete all people up to but not including Jaime
    let people: Vec<Person> = db.delete("person").range(.."jaime").await?;

    // Delete all people
    let people: Vec<Person> = db.delete("person").await?;

    Ok(())
}
