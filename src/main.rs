mod commands;
mod evt_handler;

use commands::api_calls::qod_api;
use commands::general::HELP;
use commands::ADMIN_GROUP;
use commands::EMOJI_GROUP;
use commands::GENERAL_GROUP;
use evt_handler::Handler;

use serenity::client::Client;
use serenity::{framework::standard::StandardFramework, model::id::ChannelId};
use std::collections::HashSet;
use std::env;
use std::io::Read;
use std::{fs::File, u64};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let api_token = env::var("API_TOKEN").unwrap();
    let mut client: Client;
    match &api_token.len() {
        0 => panic!("Token Not found!!"),
        _ => {
            client = Client::new(&api_token, Handler).expect("Error creating client");
        }
    };

    let owners = match client.cache_and_http.http.get_current_application_info() {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);
            owners
        }
        Err(why) => panic!("Could not access app info: {:?}", why),
    };
    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix("!").owners(owners))
            .help(&HELP)
            .group(&GENERAL_GROUP)
            .group(&EMOJI_GROUP)
            .group(&ADMIN_GROUP),
    );
    let client_ch = client.cache_and_http.clone();
    tokio::spawn(async move {
        loop {
            match qod_api::quote_of_the_day("funny").await {
                Ok(qod_tuple) => {
                    println!("Time for some quotes");
                    let mut file = File::open("sublist.txt").expect("Cannot open File");
                    let mut subscribers = String::new();
                    file.read_to_string(&mut subscribers)
                        .expect("Error reading file");
                    match subscribers.len() {
                        0 => println!("List not populated"),
                        _ => {
                            let (quote, author) = *qod_tuple;
                            let message = String::from(format!("{}\n-_{}_", quote, author));
                            for subs in subscribers.split("\n").into_iter() {
                                let chid: u64 = subs.parse::<u64>().expect("Not a u64 number");
                                let channel = ChannelId(chid);
                                println!("{}", subs);
                                match subs.len() {
                                    0 => panic!("subscribers Not found!!"),
                                    _ => {
                                        channel
                                            .say(
                                                &client_ch.http,
                                                // format!("{}\n-{}", *quote.0, *quote.1).to_string(),
                                                &message,
                                            )
                                            .expect("Failed to deliver message");
                                    }
                                };
                            }
                        }
                    };
                }
                Err(why) => println!("Error occurred: {}", why),
            };
            sleep(Duration::from_secs(60 * 60)).await; //86400 seconds in a day
        }
    });

    //Send client to quotes_task
    if let Err(msg) = client.start() {
        println!("Error : {:?}", msg);
    }
}
