use config::{Config, ConfigError, Environment, File};
use derive_builder::Builder;
use getset::Getters;
use log::{info, error};
use serde::Deserialize;

use std::{net::Ipv4Addr, ops::RangeInclusive, env, process};


#[derive(Debug, Deserialize, Getters, Clone)]
#[allow(unused)]
#[get = "pub with_prefix"]
pub struct ServerSettings {
    address: Ipv4Addr,
    port: u16
}

#[derive(Debug, Deserialize, Getters, Clone)]
#[allow(unused)]
#[get = "pub with_prefix"]
pub struct UnifiSettings {
    base_url: String,
    username: String,
    password: String
}


#[derive(Clone, Debug, Deserialize, Getters, Builder)]
#[allow(unused)]
#[builder(setter(into))]
#[get = "pub with_prefix"]
pub struct Settings {
    server: ServerSettings,
    unifi: UnifiSettings
}

const PORT_RANGE: RangeInclusive<usize> = 1024..=65535;

fn validate_port(n: &str) -> Result<u16, String> {
    let port: usize = n
        .parse()
        .map_err(|_| format!("`{n}` isn't a port number"))?;

    if PORT_RANGE.contains(&port) {
        Ok(port as u16)
    } else {
        Err(format!(
            "port not in range {}-{}",
            PORT_RANGE.start(),
            PORT_RANGE.end()
        ))
    }
}


impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let stage = env::var("STAGE").unwrap_or_else(|_| "dev".into());

        let config = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name("./config/default.toml"))
            // Add in the current environment file
            // Default to 'development' env
            // Note that this file is _optional_
            .add_source(
                File::with_name(&format!("./config/{}", stage))
                    .required(false),
            )
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            .add_source(Environment::with_prefix("kms"))
            .build()?;

        let port = config.get::<String>("server.port");
        match port {
            Ok(port_num) => {
                let is_valid = validate_port(port_num.as_str()).is_ok();

                if (!is_valid) {
                    error!("Configuration error: field \"port\" is not within range {}-{}", PORT_RANGE.start(), PORT_RANGE.end());
                    process::exit(1);
                }
            }
            Err(error) => {
                error!("Configuration Error: {}", error.to_string());
                process::exit(1);
            }
        }

        // You can deserialize (and thus freeze) the entire configuration as
        config.try_deserialize()
    }
}