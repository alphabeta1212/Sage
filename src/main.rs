mod commands;
mod evt_handler;

use commands::general::HELP;
use commands::ADMIN_GROUP;
use commands::EMOJI_GROUP;
use commands::GENERAL_GROUP;
use evt_handler::Handler;

use serenity::client::Client;
use serenity::framework::standard::StandardFramework;

use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open(".token.txt").expect("Cannot open File");
    let mut token = String::new();
    file.read_to_string(&mut token).expect("Error reading file");
    let mut client: Client;
    match token.len() {
        0 => panic!("Token Not found!!"),
        _ => {
            client = Client::new(&token, Handler).expect("Error creating client");
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
}
