mod commands;
mod evt_handler;

use commands::general::HELP;
use commands::ADMIN_GROUP;
use commands::EMOJI_GROUP;
use commands::GENERAL_GROUP;
use evt_handler::Handler;

use serenity::client::Client;
use serenity::framework::standard::StandardFramework;

use commands::api_calls::qod_api;
use std::collections::HashSet;
use std::env;
#[tokio::main]
async fn main() {
    println!("Calling qod_Api");
    let api_token = env::var("API_TOKEN").unwrap();
    let qod = qod_api::quote_of_the_day("funny").await;
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
    if let Err(msg) = client.start() {
        println!("Error : {:?}", msg);
    }
    let (quote, author) = *qod.unwrap();
    println!("Quote:{}\nAuthor:{}", quote, author);
}
