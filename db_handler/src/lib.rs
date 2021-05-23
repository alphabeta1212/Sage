use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::ClientOptions,
    Client, Collection,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc, u64, u8};

#[derive(Debug, Serialize, Deserialize)]
pub struct Subscriber {
    _id: ObjectId,
    pub inspire: Vec<String>,
    pub management: Vec<String>,
    pub sports: Vec<String>,
    pub life: Vec<String>,
    pub funny: Vec<String>,
    pub love: Vec<String>,
    pub art: Vec<String>,
    pub students: Vec<String>,
    Server: String,
}
pub enum DbHandlerError {
    NO_COLLECTION,
    FAILED_CONNECTION,
    ADDITION_FAILED,
    FAILED_QUERY,
}
pub struct Database {
    collection: Option<Collection<Subscriber>>,
}

impl Database {
    pub fn new() -> Database {
        Database { collection: None }
    }
    pub async fn get_cached_data(&mut self) -> Option<Arc<Subscriber>> {
        let filter = doc! {"Server":"VBookworms"};
        if let Some(cursor) = &mut self.collection {
            let mut cursor = cursor
                .find(filter, None)
                .await
                .expect("filter execution failed");

            // Iterate over the results of the cursor.
            if let Some(field) = cursor.try_next().await.expect("Iterator finished!") {
                let ptr = Arc::new(field);
                Some(ptr.clone())
            } else {
                None
            }
        } else {
            None
        }
    }

    pub async fn make_connection(&mut self, uri: String) -> Result<(), DbHandlerError> {
        let client_options = ClientOptions::parse(uri).await;

        match client_options {
            Ok(mut client_options) => {
                client_options.app_name = Some("savant-monk".to_string());
                // Get a handle to the cluster
                if let Ok(client) = Client::with_options(client_options) {
                    client
                        .database("quote_subscribers")
                        .run_command(doc! {"ping": 1}, None)
                        .await
                        .unwrap();
                    //TODO Handle failure in Above Case
                    println!("Connected successfully.");
                    println!("DB names:");
                    // List the names of the databases in that cluster
                    for db_name in client
                        .list_database_names(None, None)
                        .await
                        .expect("No databases found")
                    {
                        println!("{}", db_name);
                    }

                    self.collection = Some(
                        client
                            .database("quote_subscribers")
                            .collection::<Subscriber>("subscribers"),
                    );
                    Ok(())
                } else {
                    Err(DbHandlerError::FAILED_CONNECTION)
                }
                // Ping the server to see if you can connect to the cluster
            }
            Err(why) => {
                println!("Failed to make a connection [{}]", why);
                Err(DbHandlerError::FAILED_CONNECTION)
            }
        }
        // Manually set an option
    }

    pub async fn add_subscriber_to(
        &self,
        quote_type: String,
        channel_id: u64,
    ) -> Result<(), DbHandlerError> {
        let filter = doc! {"Server":"VBookworms"};
        let update = doc! {"$addToSet" : {quote_type:format!("{}", channel_id)}};
        if let Some(doc) = &self.collection {
            match doc.update_one(filter.clone(), update, None).await {
                Ok(_) => {
                    println!("Successfully added element");
                }
                Err(why) => {
                    println!("Error, could not add");
                    return Err(DbHandlerError::ADDITION_FAILED);
                }
            };
            if let Ok(mut cursor) = doc.find(filter, None).await {
                // Iterate over the results of the cursor.
                while let Ok(Some(subscriber)) = cursor.try_next().await {
                    println!("{:?}", subscriber);
                }
                Ok(())
            } else {
                Err(DbHandlerError::FAILED_QUERY)
            }
        } else {
            Err(DbHandlerError::NO_COLLECTION)
        }
    }

    pub async fn remove_subscriber_from(
        &self,
        quote_type: String,
        channel_id: u64,
    ) -> Result<(), DbHandlerError> {
        let filter = doc! {"Server":"VBookworms"};
        let pull_obj = doc! { "$pull": { quote_type: format!("{}", channel_id) } };
        if let Some(doc) = &self.collection {
            match doc.update_one(filter.clone(), pull_obj, None).await {
                Ok(_) => {
                    if let Ok(mut cursor) = doc.find(filter, None).await {
                        // Iterate over the results of the cursor.
                        while let Ok(subscriber) = cursor.try_next().await {
                            println!("{:?}", subscriber);
                        }
                        Ok(())
                    } else {
                        Err(DbHandlerError::FAILED_QUERY)
                    }
                }
                Err(why) => return Err(DbHandlerError::FAILED_QUERY),
            }
        } else {
            return Err(DbHandlerError::NO_COLLECTION);
        }
    }
}

// #[tokio::main]
// async fn main() -> mongodb::error::Result<()> {
//     // Parse your connection string into an options struct
//     let mut client_options =
//         ClientOptions::parse("mongodb+srv://watermalone:vadodara@discord-data.cna4q.mongodb.net/quote_subscribers?retryWrites=true&w=majority")
//             .await?;
//     // Manually set an option
//     client_options.app_name = Some("savant-monk".to_string());
//     // Get a handle to the cluster
//     let client = Client::with_options(client_options)?;
//     // Ping the server to see if you can connect to the cluster
//     client
//         .database("quote_subscribers")
//         .run_command(doc! {"ping": 1}, None)
//         .await?;
//     println!("Connected successfully.");
//     // List the names of the databases in that cluster
//     for db_name in client.list_database_names(None, None).await? {
//         println!("{}", db_name);
//     }
//     // let opts = CreateCollectionOptions::builder();
//     // let opts = opts.size(18).build();
//     // db.create_collection("subscribers", opts).await?;
//     // for collection_name in client
//     //     .database("quote_subscribers")
//     //     .collection("subscribers")
//     //     .find("{'channel':['1234']}", None)
//     //     .await?
//     // {
//     //     println!("{}", collection_name);
//     // }
//     let connection = client
//         .database("quote_subscribers")
//         .collection::<Subscriber>("subscribers");
//     let filter = doc! {"name":"watermalone"};
//     let mut cursor = connection.find(filter.clone(), None).await?;
//     // Iterate over the results of the cursor.
//     while let Some(subscriber) = cursor.try_next().await? {
//         println!("Channel: {:?} {:?}", subscriber._id, subscriber.channel);
//     }
//     let update = doc! {"$addToSet" : {"channel":"5"}};
//     let update_result = connection.update_one(filter.clone(), update, None).await?;
//     let mut cursor = connection.find(filter.clone(), None).await?;
//     // Iterate over the results of the cursor.
//     while let Some(subscriber) = cursor.try_next().await? {
//         println!("Channel: {:?} {:?}", subscriber._id, subscriber.channel);
//     }
//     let mut cursor = connection.find(filter.clone(), None).await?;
//     // Iterate over the results of the cursor.
//     while let Some(subscriber) = cursor.try_next().await? {
//         println!("Channel: {:?} {:?}", subscriber._id, subscriber.channel);
//     }
//     let pull_obj = doc! { "$pull": { "channel": "5" } };
//     let update_result = connection
//         .update_one(filter.clone(), pull_obj, None)
//         .await?;
//     let mut cursor = connection.find(filter.clone(), None).await?
//     // Iterate over the results of the cursor.
//     while let Some(subscriber) = cursor.try_next().await? {
//         println!("Channel: {:?} {:?}", subscriber._id, subscriber.channel);
//     }
//     Ok(())
// }
