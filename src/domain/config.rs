use std::collections::hash_map::Entry::*;
use std::collections::HashMap;

use ZoomrsError::AlreadyAdded;

use crate::domain::errors::ZoomrsError;

#[derive(Debug, Default, PartialEq)]
pub struct Config {
    data: HashMap<String, String>
}

impl Config {
    pub fn get(&self, alias: String) -> Option<String> {
        self.data.get(&alias).map(|url| url.to_string())
    }

    pub fn add(&mut self, alias: String, url: String) -> Result<(), ZoomrsError> {
        match self.data.entry(alias.clone()) {
            Occupied(_) => Err(AlreadyAdded(alias)),
            Vacant(entry) => {
                let _ = entry.insert(url);
                Ok(())
            }
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Config::default()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use claim::assert_err;

    use crate::domain::*;
    use crate::domain::errors::ZoomrsError;

    #[test]
    fn default_is_empty() {
        let config = Config::default();

        assert_eq!(config, Config::new())
    }

    #[test]
    fn can_add_room() {
        let mut config = Config::default();

        config.add("alias".to_string(), "url".to_string());

        assert_eq!(config.get("alias".to_string()), Some("url".to_string()));
    }

    #[test]
    fn cannot_add_room_twice() {
        let mut config = Config::default();

        let _ = config.add("alias".to_string(), "url".to_string());

        let result = config.add("alias".to_string(), "url".to_string());

        assert_err!(result);
    }
}
