use crate::api::model::WrappedRoute;
use riven::consts::PlatformRoute;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    summoner: String,
    region: String,
}

impl Config {
    pub fn summoner(&self) -> &str {
        &self.summoner
    }

    pub fn region(&self) -> PlatformRoute {
        self.get_route()
    }

    pub fn set_region(&mut self, region: PlatformRoute) {
        self.region = region.to_string();
    }

    pub fn set_summoner(&mut self, sum: &str) {
        self.summoner = sum.to_string();
    }

    fn get_route(&self) -> PlatformRoute {
        if let Ok(p) = WrappedRoute::from_str(&self.region) {
            p.0
        } else {
            panic!("Unknown region value");
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            summoner: "".to_string(),
            region: "NA".to_string(),
        }
    }
}

//functions

pub fn load_config(path: &Path) -> Config {
    let path = path.join("config.ron");
    if path.exists() {
        match fs::read_to_string(&path) {
            Ok(f) => ron::from_str(&f).expect("Unable to parse config file"),
            Err(_) => {
                let def = Config::default();
                save_config(&path, &def);
                def
            }
        }
    } else {
        let def = Config::default();
        save_config(&path, &def);
        def
    }
}

pub fn save_config(path: &Path, cfg: &Config) {
    let raw = ron::to_string(cfg).expect("Unable to serialize config");
    fs::write(path, raw).unwrap();
}
