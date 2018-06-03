#[macro_use]
extern crate serenity;
#[macro_use]
extern crate slog;
extern crate sloggers;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::fs::File;
use serde_json::Error;
use sloggers::Build;
use sloggers::terminal::{TerminalLoggerBuilder, Destination};
use sloggers::types::Severity;
use serenity::client::{Client, validate_token};
use serenity::prelude::EventHandler;
use serenity::framework::standard::StandardFramework;

struct Handler;

impl EventHandler for Handler {}

#[derive(Deserialize, Debug)]
struct BotConf {
    token: String,
    prefix: String,
}

fn configure() -> Result<BotConf, Error> {
    let mut file = File::open("config.json").expect("Not found");
    let conf = serde_json::from_reader(file)?;
    Ok(conf)
}

fn main() {
    let config = configure().unwrap();
    let mut builder = TerminalLoggerBuilder::new();
    builder.level(Severity::Debug);
    builder.destination(Destination::Stderr);
    let logger = builder.build().unwrap();
    info!(logger, "hewwo!");
    info!(logger, "testing token...");
    assert!(validate_token(&config.token).is_ok(), "token is not valid");
    debug!(logger, "init client");
    let mut client = Client::new(&config.token, Handler)
        .expect("oopsie woopsie~!");
    client.with_framework(StandardFramework::new()
                          .configure(|c| c.prefix(&config.prefix))
                          .cmd("hi", hi));

    if let Err(why) = client.start() {
        error!(logger, "oh shit a error: {:?}", why);
    }
}

command!(hi(_ctx, msg) {
    let _ = msg.reply("hi, i'm a rust bot");
});
