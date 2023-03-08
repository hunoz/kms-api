pub mod device {
    use std::net::Ipv4Addr;

    use eui48::MacAddress;
    use paperclip::actix::Apiv2Schema;
    use crate::clients::dynamodb::DynamoDBClient;
    use serde_derive::{Serialize, Deserialize};
    use validator::{Validate};
    use getset::{Getters, Setters};
    use lazy_static::lazy_static;
    use regex::Regex;

    const CLIENT: DynamoDBClient = DynamoDBClient { client: ""  };

    lazy_static! {
        static ref MAC_ADDRESS_RE: Regex = Regex::new(r"^([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2})$").unwrap();
    }

    #[derive(Serialize, Deserialize, Validate, Getters, Setters, Apiv2Schema)]
    #[get = "pub with_prefix"]
    #[set = "pub with_prefix"]
    #[serde(rename_all = "camelCase")]
    pub struct Device {
        #[validate(length(min = 3))]
        hostname: String,
        #[validate(regex = "MAC_ADDRESS_RE")]
        mac_address: String,
        ip_address: Ipv4Addr
    }

    impl Device {
        pub fn from_mac_address(mac_address: MacAddress) -> Device {
            // We look up the device here
            CLIENT.client;
            Device {
                hostname: "fake".to_owned(),
                mac_address:mac_address.to_hex_string(),
                ip_address: Ipv4Addr::new(1, 1, 1, 1)
            }
        }
        pub fn new(hostname: &str, mac_address: MacAddress, ip_address: Ipv4Addr) -> Self {
            Device {
                hostname: hostname.to_owned(),
                mac_address: mac_address.to_hex_string(),
                ip_address: ip_address.to_owned()
            }
        }
    }
}

pub mod errors {
    use std::{collections::HashMap, borrow::Cow};
    use actix_web::{error, HttpResponse};
    use derive_more::{Display, Error};
    use eui48::ParseError;
    use paperclip::actix::Apiv2Schema;
    use serde_derive::{Serialize, Deserialize};
    use validator::{ValidationErrors, ValidationError};
    use convert_case::{Case, Casing};

    #[derive(Debug, Display, Error)]
    pub enum Errors {
        #[display(fmt = "Validation Error")]
        ValidationError { field_errors: HashMap<String, Vec<Cow<'static, str>>> },
        #[display(fmt = "Internal Server Exception")]
        InternalServerError,
        #[display(fmt = "Not Found")]
        NotFoundError,
        #[display(fmt = "Unauthorized")]
        UnauthorizedError,
        #[display(fmt = "Forbidden")]
        ForbiddenError
    }

    #[derive(Serialize, Deserialize, Apiv2Schema)]
    #[serde(rename_all = "camelCase")]
    pub struct ErrorMessage<T> {
        error_message: T,
    }

    impl error::ResponseError for Errors {
        fn error_response(&self) -> HttpResponse {
            match &*self {
                Errors::ValidationError { field_errors } => HttpResponse::BadRequest().json(ErrorMessage {
                    error_message: field_errors
                }),
                Errors::InternalServerError => HttpResponse::InternalServerError().json(ErrorMessage {
                    error_message: "Internal Server Error"
                }),
                Errors::NotFoundError => HttpResponse::NotFound().json(ErrorMessage {
                    error_message: "Resource Not Found"
                }),
                Errors::UnauthorizedError => HttpResponse::Unauthorized().json(ErrorMessage {
                    error_message: "Unauthorized"
                }),
                Errors::ForbiddenError => HttpResponse::Forbidden().json(ErrorMessage {
                    error_message: "Forbidden"
                })
            }
        }
    }

    fn parse_validation_errors(validation_errors: ValidationErrors ) -> HashMap<String, Vec<Cow<'static, str>>> {
        let mut error_map = HashMap::new();
        for (field_name, field_errors) in validation_errors.field_errors() {
            let error_codes: Vec<Cow<str>> = field_errors.iter().map(|error| error.code.clone()).collect();
            error_map.insert(field_name.to_string().to_case(Case::Camel), error_codes);
        }
        return error_map;
    }
    pub fn create_error_response_for_mac_address(error: ParseError) -> HashMap<String, Vec<Cow<'static, str>>> {
        let code  = if error.to_owned().to_string().contains("Invalid length") { "length" } else { "bytes" };   
        println!("Error: {}", error);
        let mut validation_errors = ValidationErrors::new();
        validation_errors.add("macAddress", ValidationError::new(code));
        parse_validation_errors(validation_errors)
    }
}

pub mod responses {
    use paperclip::actix::Apiv2Schema;
    use serde_derive::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Apiv2Schema)]
    pub struct Response<T> {
        pub data: T
    }
}