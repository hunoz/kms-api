pub mod dog {
    use paperclip::actix::Apiv2Schema;
    use serde_derive::{Serialize, Deserialize};
    use validator::{Validate};
    use getset::{Getters, Setters};

    #[derive(Serialize, Deserialize, Validate, Getters, Setters, Apiv2Schema)]
    #[get = "pub with_prefix"]
    #[set = "pub with_prefix"]
    #[serde(rename_all = "camelCase")]
    pub struct Dog {
        #[validate(length(min = 3))]
        breed: String,
        #[validate(length(min = 3))]
        color: String,
        #[validate(required)]
        is_vaccinated: Option<bool>,
    }

    impl Dog {
        pub fn bark(&self) {
            println!("Bark!");
        }

        pub fn new(breed: &str, color: &str, is_vaccinated: bool) -> Dog {
            Dog {
                breed: String::from(breed),
                color: String::from(color),
                is_vaccinated: Some(is_vaccinated)
            }
        }
    }
}

pub mod errors {
    use std::{collections::HashMap, borrow::Cow};
    use actix_web::{error, HttpResponse};
    use derive_more::{Display, Error};
    use paperclip::actix::Apiv2Schema;
    use serde_derive::{Serialize, Deserialize};
    use validator::ValidationErrors;
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

    pub fn parse_validation_errors(validation_errors: ValidationErrors ) -> HashMap<String, Vec<Cow<'static, str>>> {
        let mut error_map = HashMap::new();
        for (field_name, field_errors) in validation_errors.field_errors() {
            let error_codes: Vec<Cow<str>> = field_errors.iter().map(|error| error.code.clone()).collect();
            error_map.insert(field_name.to_string().to_case(Case::Camel), error_codes);
        }
        return error_map;
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