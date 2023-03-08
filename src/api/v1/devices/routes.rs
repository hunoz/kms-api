use std::{collections::HashMap, borrow::Cow, panic};

use crate::{api::v1::devices::models::{responses::Response, errors::Errors, device::Device, errors::create_error_response_for_mac_address}, clients::unifi::{UnifiApiClient, models::ListClientsResponse}, settings::Settings};
use eui48::MacAddress;

use actix_web::{web::{Path, Data}, Result, HttpResponse};
use log::{warn, info, error};
use paperclip::actix::{web::{Json}, api_v2_operation, get};


#[api_v2_operation]
#[get("/device/{mac_address}")]
pub async fn get_device_by_mac(path: Path<String>) -> Result<Json<Response<Device>>, actix_web::Error> {
    let mac_address = path.into_inner();
    let device: Device;

    // There's an odd error with some invalid MACs that the parser panics at
    let result = panic::catch_unwind(|| {
        MacAddress::parse_str(mac_address.clone().as_str());
    });

    if result.is_err() {
        warn!("Error parsing MAC address: {:?}", result.unwrap_err());
        let mut error_map = HashMap::new();
        error_map.insert("macAddress".to_owned(), vec![Cow::from("length")]);
        return Err(
            Errors::ValidationError { field_errors: error_map }.into()
        )
    }

    info!("No errors detected while parsing MAC address");

    match MacAddress::parse_str(mac_address.clone().as_str()) {
        Ok(mac) => {
            info!("Searching for device with MAC: {}", mac);
            device = Device::from_mac_address(mac);

            info!("Found device with name: {}", &device.get_hostname());

            Ok(
                Json(
                    Response {
                        data: device
                    }
                )
            )
        }
        Err(error) => {
            Err(
                Errors::ValidationError { field_errors: create_error_response_for_mac_address(error) }.into()
            )
        }
    }
}


#[api_v2_operation]
#[get("/list")]
async fn list_clients(data: Data<Settings>) -> HttpResponse {
    let unifi_settings = data.get_unifi();
    let mut client = UnifiApiClient::new(
        unifi_settings.get_base_url().clone(),
        unifi_settings.get_username().clone(),
        unifi_settings.get_password().clone()
    );
    
    let response = client.list_clients().await;
    let body: Result<ListClientsResponse, reqwest::Error>;

    match response.unwrap().error_for_status() {
        Ok(res) => {
            body = res.json().await;
        }
        Err(err) => {
            return HttpResponse::InternalServerError().json(err.to_string());
        }
    }

    match &body {
        Ok(response_body) => {
            for item in response_body.get_data() {
                info!("Item name: {} | Item MAC: {}",item.get_name().as_ref().unwrap_or(&"None".to_owned()), item.get_mac().as_ref().unwrap_or(&"None".to_owned()));
            }
            HttpResponse::Ok().json(body.unwrap())
        }
        Err(error) => {
            println!("{}", error);
            error!("Error: {}", error);
            HttpResponse::InternalServerError().finish()
        }
    }
}