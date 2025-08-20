
use dioxus::prelude::*;
use openapi::apis::configuration::Configuration;
use openapi::apis::*;
use openapi::models as models;
use models::*;

use serde::{Deserialize, Serialize};

const API_VERSION: &str = "2025-05-20";

pub struct Client {
    config: Configuration,
}

impl Client {    
    fn new() -> Self {
        Self { config: Configuration::new() }
    }
    pub fn set_token(&mut self, token: String) {
        self.config.bearer_access_token = Some(token);
    }

    pub async fn list_spaces(&self) -> Result<PaginationPeriodPaginatedResponseApimodelSpace, Error<openapi::apis::spaces_api::ListSpacesError>> {
        openapi::apis::spaces_api::list_spaces(&self.config, API_VERSION, None, None)
            .await
    }
    pub async fn get_tasks(&self, space_id: &str) -> Result<PaginationPeriodPaginatedResponseApimodelObject, Error<openapi::apis::search_api::SearchSpaceError>> {
        let mut req = openapi::models::ApimodelPeriodSearchRequest::new();
        req.types = vec!["task".to_string()].into();

        openapi::apis::search_api::search_space(&self.config, API_VERSION, space_id, req, None, None)
            .await
    }
    pub async fn get_space(&self, space_id: &str) -> Result<ApimodelPeriodSpaceResponse, Error<openapi::apis::spaces_api::GetSpaceError>> {
        openapi::apis::spaces_api::get_space(&self.config, API_VERSION, space_id)
            .await
    }
    pub fn update_done_property(&self, space_id: String, object_id: String, done: bool) { // -> Result<ApimodelPeriodObjectResponse, Error<openapi::apis::objects_api::UpdateObjectError>> {
        let config = self.config.clone();
        spawn(async move {
            let mut prop = ApimodelPeriodCheckboxPropertyLinkValue::new();
            prop.key = "done".to_string().into();
            prop.checkbox = done.into();

            let mut req= ApimodelPeriodUpdateObjectRequest::new();
            req.properties = Some(vec![ApimodelPeriodPropertyLinkWithValue::ApimodelPeriodCheckboxPropertyLinkValue(Box::new(prop))]);
            
            println!("{:#?}", req);
            let res = openapi::apis::objects_api::update_object(&config, API_VERSION, &space_id, &object_id, req)
            .await;
            println!("{:#?}", res);
        });
    }
}

pub static API_CLIENT: GlobalSignal<Client> = Global::new(|| Client::new());


// Assuming you have all the necessary PropertyValue structs defined in your `models` module.
// Example:
// pub struct ApimodelPeriodTextPropertyValue { ... }
// pub struct ApimodelPeriodCheckboxPropertyValue { ... }

// This function maps a format enum to a value enum.
fn create_default_property_by_format(
    format: &models::ApimodelPeriodPropertyFormat,
) -> models::ApimodelPeriodPropertyWithValue {
    use models::ApimodelPeriodPropertyFormat as Format;
    use models::ApimodelPeriodPropertyWithValue as Value;

    match format {
        Format::PropertyFormatText => {
            Value::ApimodelPeriodTextPropertyValue(Box::new(models::ApimodelPeriodTextPropertyValue::default()))
        }
        Format::PropertyFormatNumber => {
            Value::ApimodelPeriodNumberPropertyValue(Box::new(models::ApimodelPeriodNumberPropertyValue::default()))
        }
        Format::PropertyFormatSelect => {
            Value::ApimodelPeriodSelectPropertyValue(Box::new(models::ApimodelPeriodSelectPropertyValue::default()))
        }
        Format::PropertyFormatMultiSelect => {
            Value::ApimodelPeriodMultiSelectPropertyValue(Box::new(models::ApimodelPeriodMultiSelectPropertyValue::default()))
        }
        Format::PropertyFormatDate => {
            Value::ApimodelPeriodDatePropertyValue(Box::new(models::ApimodelPeriodDatePropertyValue::default()))
        }
        Format::PropertyFormatFiles => {
            Value::ApimodelPeriodFilesPropertyValue(Box::new(models::ApimodelPeriodFilesPropertyValue::default()))
        }
        Format::PropertyFormatCheckbox => {
            Value::ApimodelPeriodCheckboxPropertyValue(Box::new(models::ApimodelPeriodCheckboxPropertyValue::default()))
        }
        Format::PropertyFormatUrl => {
            Value::ApimodelPeriodUrlPropertyValue(Box::new(models::ApimodelPeriodUrlPropertyValue::default()))
        }
        Format::PropertyFormatEmail => {
            Value::ApimodelPeriodEmailPropertyValue(Box::new(models::ApimodelPeriodEmailPropertyValue::default()))
        }
        Format::PropertyFormatPhone => {
            Value::ApimodelPeriodPhonePropertyValue(Box::new(models::ApimodelPeriodPhonePropertyValue::default()))
        }
        Format::PropertyFormatObjects => {
            Value::ApimodelPeriodObjectsPropertyValue(Box::new(models::ApimodelPeriodObjectsPropertyValue::default()))
        }
    }
}