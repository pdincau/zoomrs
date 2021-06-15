mod config;
mod errors;

use crate::domain::config::Config;

pub enum Command {
    Add(Add),
    Join(Join),
    Remove(Remove),
}

struct Add {
    config: Config,
    alias: String,
    url: String,
}

struct Remove {
    config: Config,
    alias: String,
}

struct Join {
    config: Config,
    alias: String,
}