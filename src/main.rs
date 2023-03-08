mod clients;
mod settings;
mod api;

use std::{net::Ipv4Addr, ops::RangeInclusive, process};

use actix_web::{App, HttpServer, web::Data};
use clap::Parser;
use env_logger::{Builder, Target};
use log::{info, error};
use api::{v1};
use settings::{Settings, ServerSettings, UnifiSettings, SettingsBuilder};
use paperclip::actix::{OpenApiExt, web::scope};


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "127.0.0.1")]
    address: Ipv4Addr,
    #[arg(short, long, default_value_t = 8080, value_parser = validate_port)]
    port: u16
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    Builder::from_default_env().target(Target::Stdout).init();
    let settings_result = Settings::new();
    match settings_result {
        Ok(result) => {
            let s = &result.to_owned();

            HttpServer::new(move || {
                let settings = &result.clone();
                App::new()
                    .wrap_api()
                    .with_json_spec_at("/openapi.json")
                    .with_swagger_ui_at("/docs")
                    .app_data(Data::new(
                        settings.to_owned()
                    ))
                    .service(
                        scope("/v1/devices")
                        .service(v1::devices::routes::get_device_by_mac)
                        .service(v1::devices::routes::list_clients)
                    )
                    .service(
                        scope("/v1/animals")
                            .service(v1::animals::routes::get_dog)
                            .service(v1::animals::routes::create_dog)
                    )
                    .build()
            })
            .bind((s.get_server().get_address().to_owned(), s.get_server().get_port().to_owned()))?
            .run()
            .await
        }
        Err(error) => {
            error!("Configuration Error: {}", error);
            process::exit(1)
        }
    }
}