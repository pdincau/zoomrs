use domain::config::{Config, Room};

use crate::{
    domain::config_store::ConfigStore, infrastructure::file_config_store::FileConfigStore,
};

mod domain;
mod infrastructure;

fn main() {
    println!("Hello, world!");
    let command = std::env::args().nth(1).expect("no command given");
    let first_arg = std::env::args().nth(2);
    let second_arg = std::env::args().nth(3);
    let file_config_store = &FileConfigStore::new("/tmp/asdf");

    match command.as_str() {
        "add" => handle_add(
            first_arg.expect("no alias given"),
            second_arg.expect("no url given"),
            file_config_store,
        ),
        _ => panic!("Invalid command"),
    }
}

fn handle_add(
    alias: String,
    url: String,
    file_config_store: &dyn ConfigStore,
) -> () {
    let mut config = file_config_store.load().unwrap_or(Config::new());

    let room = Room::new(alias.as_str(), url.as_str());
    config.add(room);

    file_config_store.save(&config);
}
