use hmac::Hmac;
use jwt::VerifyWithKey;
use once_cell::sync::Lazy;
use poem::Request;
use poem_openapi::{auth::ApiKey, SecurityScheme};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::{
    io::{Read, Write},
    path::PathBuf,
    str::FromStr,
};

use crate::common::object::user::UserSign;

pub type ServerKey = Hmac<Sha256>;

/// ApiKey authorization
#[derive(SecurityScheme)]
#[oai(
    type = "api_key",
    key_name = "NRS-TOKEN",
    in = "header",
    checker = "api_checker"
)]
pub struct AppAuthorization(pub UserSign);

async fn api_checker(req: &Request, api_key: ApiKey) -> Option<UserSign> {
    let server_key = req.data::<ServerKey>().unwrap();
    VerifyWithKey::<UserSign>::verify_with_key(api_key.key.as_str(), server_key).ok()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Server {
    pub api_port: u16,
    pub log_file: PathBuf,
    pub api_key: String,
    pub salt: String,
    pub server_key: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Database {
    pub user_name: String,
    pub password: String,
    pub host: String,
    pub port: String,
    pub db: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Common {
    pub model_addr: String,
    pub log_level: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub common: Common,
    pub server: Server,
    pub database: Database,
}

impl Default for Config {
    /// dev config
    fn default() -> Self {
        Self {
            common: Common {
                model_addr: "model:8080".into(),
                log_level: "DEBUG".into(),
            },
            server: Server {
                api_port: 3000,
                log_file: PathBuf::from_str("server.log").unwrap(),
                api_key: String::from("SNn0TR#*N0f#JDMWsdmiwan3dj2d2k3d"),
                salt: String::from("Nekopara114514"),
                server_key: String::from("0237jfH#f3h289f3j0"),
            },
            database: Database {
                user_name: "news_recommender".into(),
                password: "nekopara".into(),
                host: "127.0.0.1".into(),
                port: "5432".into(),
                db: "news_recommend".into(),
            },
        }
    }
}

pub static CONFIG: once_cell::sync::Lazy<Config> = Lazy::new(|| {
    Config::parse_from_file(&PathBuf::from_str("config.toml").unwrap()).unwrap_or_default()
});

impl Config {
    pub fn parse_from_file(file: &PathBuf) -> anyhow::Result<Self> {
        let mut file = std::fs::File::open(file)?;
        let mut buf = String::with_capacity(file.metadata()?.len() as usize);
        file.read_to_string(&mut buf)?;
        Ok(toml::from_str(&buf)?)
    }

    pub fn write_to_file(&self, file: &PathBuf) -> anyhow::Result<()> {
        let content = toml::to_string_pretty(self)?;
        let mut file = std::fs::File::create(file)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }
}

#[test]
fn write_and_read_config() {
    let config = Config::default();
    let file = PathBuf::from_str("config.toml").unwrap();
    config.write_to_file(&file).unwrap();
    let config = Config::parse_from_file(&file).unwrap();
    println!("{config:?}");
}
