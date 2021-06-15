use std::collections::HashMap;

#[derive(Debug, Default, PartialEq)]
pub struct Config {
    data: HashMap<String, String>
}

impl Config {
    pub fn get(&self, alias: String) -> Option<String> {
        self.data.get(&alias).map(|url| url.to_string())
    }

    pub fn add(&mut self, alias: String, url: String) {
        self.data.insert(alias, url);
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

    use crate::domain::*;

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
}
