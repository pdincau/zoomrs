use std::collections::hash_map::Entry::*;
use std::collections::HashMap;

use ZoomrsError::AlreadyAdded;

use crate::domain::errors::ZoomrsError;
use std::fmt::Error;

#[derive(Debug, Default, PartialEq)]
pub struct Config {
    data: HashMap<String, String>,
}

impl Config {
    pub fn get(&self, alias: String) -> Option<String> {
        self.data.get(&alias).map(|url| url.to_string())
    }

    pub fn add(&mut self, room: Room) -> Result<(), ZoomrsError> {
        match self.data.entry(room.alias.clone()) {
            Occupied(_) => Err(AlreadyAdded(room.alias)),
            Vacant(entry) => {
                entry.insert(room.url);
                Ok(())
            }
        }
    }

    pub fn search(&mut self, alias: String) -> Option<Room> {
        match self.data.entry(alias) {
            Occupied(entry) => Some(Room::new(entry.key(), entry.get())),
            Vacant(_) => None,
        }
    }

    pub fn delete(&mut self, alias: String) -> Result<(), ZoomrsError> {
        match self.data.remove(alias.as_str()) {
            None => Err(ZoomrsError::NotPresent(alias)),
            Some(_) => Ok(()),
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Config::default()
    }
}

#[derive(Debug, PartialEq)]
pub struct Room {
    pub alias: String,
    pub url: String,
}

impl Room {
    fn new(alias: &str, url: &str) -> Room {
        Room {
            alias: alias.to_string(),
            url: url.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use claim::assert_err;
    use claim::assert_ok;

    use crate::domain::config::Room;
    use crate::domain::*;

    #[test]
    fn default_is_empty() {
        let config = Config::default();

        assert_eq!(config, Config::new())
    }

    #[test]
    fn can_add_room() {
        let mut config = Config::default();

        let _ = config.add(Room::new("alias", "url"));

        assert_eq!(config.get("alias".to_string()), Some("url".to_string()));
    }

    #[test]
    fn cannot_add_room_twice() {
        let mut config = Config::default();

        let room = Room::new("alias", "url");
        let the_same_room = Room::new("alias", "url");

        let _ = config.add(room);

        let result = config.add(the_same_room);

        assert_err!(result);
    }

    #[test]
    fn can_search_room() {
        let mut config = Config::default();

        let room = Room::new("alias", "url");

        let _ = config.add(room);

        let expected_room = Room::new("alias", "url");

        assert_eq!(None, config.search("not_existing".to_string()));
        assert_eq!(Some(expected_room), config.search("alias".to_string()));
    }

    #[test]
    fn cannot_delete_missing_room() {
        let mut config = Config::default();

        assert_err!(config.delete("not_existing".to_string()));
    }

    #[test]
    fn can_delete_room() {
        let mut config = Config::default();

        let room = Room {
            alias: "alias".to_string(),
            url: "url".to_string(),
        };

        let _ = config.add(room);

        assert_ok!(config.delete("alias".to_string()));
    }
}
