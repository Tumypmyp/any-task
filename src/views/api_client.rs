
use dioxus::prelude::*;
use openapi::apis::configuration::Configuration;
use openapi::apis::*;
use openapi::models::*;

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

            let mut req= openapi::models::ApimodelPeriodUpdateObjectRequest::new();
            req.properties = Some(vec![ApimodelPeriodPropertyLinkWithValue::ApimodelPeriodCheckboxPropertyLinkValue(Box::new(prop))]);
            
            tracing::debug!("{:#?}", req);
            let res = openapi::apis::objects_api::update_object(&config, API_VERSION, &space_id, &object_id, req)
            .await;
            tracing::debug!("{:#?}", res);
        });
    }
}

pub static API_CLIENT: GlobalSignal<Client> = Global::new(|| Client::new());
