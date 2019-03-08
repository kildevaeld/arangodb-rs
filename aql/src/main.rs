extern crate arangodb;
use arangodb::{error::Result, lazy, models::User, rt, Future, Query};

fn main() -> Result<()> {
    let client = arangodb::Client::build()
        .endpoint("http://localhost:8529/_db/test_database2/")
        .auth(arangodb::Authorization::Simple(
            "test_user".to_string(),
            "password".to_string(),
        ))
        .build()?;

    // let fu = arangodb::CreateDatabase::new("test_database2")
    //     .user(User {
    //         active: true,
    //         passwd: Some("password".to_owned()),
    //         username: "test_user".to_owned(),
    //         extra: None,
    //         array: vec![],
    //     })
    //     .run(client)
    //     .map_err(|e| {
    //         println!("error: {:?}", e);
    //     })
    //     .and_then(|m| {
    //         println!("{:?}", m.1);
    //         arangodb::ListDatabases::new()
    //             .run(m.0)
    //             .map_err(|e| {
    //                 println!("error: {:?}", e);
    //             })
    //             .map(|m| {
    //                 println!("{:?}", m.1);
    //             })
    //     });

    // let fu = arangodb::ListCollections::new()
    //     .run(client)
    //     .map_err(|e| println!("{:?}", e))
    //     .map(|m| println!("{:?}", m.1));

    let fu = arangodb::CreateCursor::new("for doc in test_collection_rap return doc")
        .run(client)
        .map_err(|e| println!("{:?}", e))
        .map(|m| println!("{:?}", m.1));

    // let fu = arangodb::CreateCollection::new("test_collection_rap")
    //     .run(client)
    //     .map_err(|e| println!("{:?}", e))
    //     .map(|m| println!("{:?}", m.1));

    rt::run(fu);
    // let mut rt = rt::runtime::Runtime::new().unwrap();
    // rt.spawn(fu);
    // //drop(client);
    // rt.shutdown_on_idle().wait().unwrap();

    Ok(())
}
