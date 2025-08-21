use dioxus::prelude::*;
use models::*;
use openapi::apis::configuration::Configuration;
use openapi::apis::*;
use openapi::models;
const API_VERSION: &str = "2025-05-20";
pub struct Client {
    config: Configuration,
}
impl Client {
    fn new() -> Self {
        Self {
            config: Configuration::new(),
        }
    }
    pub fn set_token(&mut self, token: String) {
        self.config.bearer_access_token = Some(token);
    }
    pub async fn list_spaces(
        &self,
    ) -> Result<
        PaginationPeriodPaginatedResponseApimodelSpace,
        Error<openapi::apis::spaces_api::ListSpacesError>,
    > {
        openapi::apis::spaces_api::list_spaces(&self.config, API_VERSION, None, None)
            .await
    }
    pub async fn list_properties(
        &self,
        space_id: &str,
    ) -> Result<
        PaginationPeriodPaginatedResponseApimodelProperty,
        Error<openapi::apis::properties_api::ListPropertiesError>,
    > {
        openapi::apis::properties_api::list_properties(
                &self.config,
                API_VERSION,
                space_id,
                None,
                None,
            )
            .await
    }
    pub async fn list_select_property_options(
        &self,
        space_id: &str,
        property_id: &str,
    ) -> Result<
        openapi::models::PaginationPeriodPaginatedResponseApimodelTag,
        Error<openapi::apis::tags_api::ListTagsError>,
    > {
        openapi::apis::tags_api::list_tags(
                &self.config,
                API_VERSION,
                space_id,
                property_id,
            )
            .await
    }
    pub async fn get_tasks(
        &self,
        space_id: &str,
    ) -> Result<
        PaginationPeriodPaginatedResponseApimodelObject,
        Error<openapi::apis::search_api::SearchSpaceError>,
    > {
        let mut req = openapi::models::ApimodelPeriodSearchRequest::new();
        req.types = vec!["task".to_string()].into();
        openapi::apis::search_api::search_space(
                &self.config,
                API_VERSION,
                space_id,
                req,
                None,
                None,
            )
            .await
    }
    pub async fn get_space(
        &self,
        space_id: Signal<String>,
    ) -> Result<
        ApimodelPeriodSpaceResponse, 
        Error<openapi::apis::spaces_api::GetSpaceError>,
    > {
        openapi::apis::spaces_api::get_space(&self.config, API_VERSION, &space_id()).await    
    }
    
    pub fn get_property(
        &self,
        space_id: Signal<String>,
        property_id: Signal<String>,
    ) -> Option<ApimodelPeriodPropertyResponse> {
        let config = use_signal(|| self.config.clone());
        let resp = use_resource(move || async move {
            openapi::apis::properties_api::get_property(
                    &config(),
                    API_VERSION,
                    &space_id(),
                    &property_id(),
                )
                .await
        });
        match &*resp.read() {
            Some(Ok(p)) => Some(p.clone()),
            _ => None,
        }
    }
    pub fn update_done_property(&self, space_id: String, object_id: String, done: bool) {
        let config = self.config.clone();
        spawn(async move {
            let mut prop = ApimodelPeriodCheckboxPropertyLinkValue::new();
            prop.key = "done".to_string().into();
            prop.checkbox = done.into();
            let mut req = ApimodelPeriodUpdateObjectRequest::new();
            req.properties = Some(
                vec![
                    ApimodelPeriodPropertyLinkWithValue::ApimodelPeriodCheckboxPropertyLinkValue(
                        Box::new(prop),
                    ),
                ],
            );
            println!("{:#?}", req);
            let res = openapi::apis::objects_api::update_object(
                    &config,
                    API_VERSION,
                    &space_id,
                    &object_id,
                    req,
                )
                .await;
            println!("{:#?}", res);
        });
    }
}
pub fn get_property_id_by_key<'a>(space_id: String, key: &str) -> Option<String> {
    let properties = use_resource(move || {
        let id = space_id.clone();
        async move { API_CLIENT.read().list_properties(&id).await }
    });
    match &*properties.read() {
        Some(Ok(props)) => {
            for prop in props.data.clone().unwrap() {
                if prop.key.unwrap() == key {
                    return prop.id;
                }
            }
        }
        Some(Err(e)) => {
            println!("error: {:#?}", e);
        }
        _ => {}
    }
    None
}
pub static API_CLIENT: GlobalSignal<Client> = Global::new(|| Client::new());
