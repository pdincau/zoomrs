use crate::domain::config::Config;
use crate::domain::config_store::{ConfigStore, ConfigStoreError};
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub struct FileConfigStore {
    file: String,
}

impl FileConfigStore {
    pub fn new(file: String) -> Self {
        Self { file }
    }
}

impl ConfigStore for FileConfigStore {
    fn load(&self) -> Result<Config, ConfigStoreError> {
        let path = Path::new(&self.file);

        let file = File::open(&path)?;
        let config: Config = serde_json::from_reader(file)?;
        Ok(config)
    }

    fn save(&self, config: &Config) -> Result<(), ConfigStoreError> {
        let serialized_config = serde_json::to_string(config).unwrap();
        let path = Path::new(&self.file);

        let mut file = File::create(&path)?;
        file.write_all(serialized_config.as_bytes())?;

        Ok(())
    }
}

#[cfg(test)]
cappuccino::tests!({
    use claim::{assert_err, assert_ok};

    use super::FileConfigStore;
    use crate::domain::config::{Config, Room};
    use crate::domain::config_store::ConfigStore;

    use std::fs::File;
    use std::io::Write;
    use std::path::Path;

    it "fails to load the config when file does not exist" {
        let config_store = FileConfigStore::new("/tmp/not-existing".to_string());
        assert_err!(config_store.load());
    }

    it "fails to load the config when file is malformed" {
        let mut file = File::create(Path::new("/tmp/malformed-zoomrs-config"));
        file.unwrap().write_all("invalid".as_bytes());

        let config_store = FileConfigStore::new("/tmp/malformed-zoomrs-config".to_string());
        assert_err!(config_store.load());
    }

    it "saves and loads config from disk" {
        let mut config = Config::new();
        config.add(Room::new("alias", "url"));
        let config_store = FileConfigStore::new("/tmp/zoomrs-config".to_string());

        assert_ok!(config_store.save(&config));
        assert_eq!(config, config_store.load().unwrap());
    }
});
