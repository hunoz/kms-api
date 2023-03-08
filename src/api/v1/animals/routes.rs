use crate::api::v1::animals::models::{errors::{Errors, parse_validation_errors}, responses::{Response}, dog::{Dog}};
use validator::Validate;
use paperclip::actix::{web::Json, api_v2_operation, get, post};

use actix_web::{Error, Result};

#[api_v2_operation]
#[get("/dog")]
pub async fn get_dog() -> Result<Json<Response<Dog>>, Error> {
    let data = Response {data: Dog::new("Labrador", "Black", true)};

    println!("{}", &data.data.get_breed());

    Err(Errors::NotFoundError.into())
    //Ok(web::Json(data))
}

#[api_v2_operation]
#[post("/dog")]
pub async fn create_dog(dog_info: Json<Dog>) -> Result<Json<Response<Dog>>, Error> {
    let mut dog = dog_info.0;

    dog.set_breed("Terrier".to_string());
    dog.bark();

    match dog.validate() {
        Ok(_) => Ok(
            Json(
                Response {
                    data: Dog::new(dog.get_breed(), dog.get_color(), dog.get_is_vaccinated().to_owned().unwrap())
                }
            )
        ),
        Err(e) => {
            Err(Errors::ValidationError { field_errors: parse_validation_errors(e) }.into())
        }
    }
}