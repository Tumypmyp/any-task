use dioxus::prelude::*;
use openapi::apis::configuration::Configuration;
use openapi::apis::*;
use openapi::models::*;
use time::UtcDateTime;
use time::format_description::well_known::Rfc3339;
const API_VERSION: &str = "2025-05-20";
pub static API_CLIENT: GlobalSignal<Client> = Global::new(|| Client::new());
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
    pub fn set_server(&mut self, server: String) {
        self.config.base_path = format!("http://{server}");
    }
    pub async fn list_spaces(
        &self,
    ) -> Result<
        PaginationPeriodPaginatedResponseApimodelSpace,
        Error<openapi::apis::spaces_api::ListSpacesError>,
    > {
        openapi::apis::spaces_api::list_spaces(&self.config, API_VERSION, None, None).await
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
        openapi::apis::tags_api::list_tags(&self.config, API_VERSION, space_id, property_id).await
    }
    pub async fn get_types(
        &self,
        space_id: &str,
        types: Vec<String>,
    ) -> Result<
        PaginationPeriodPaginatedResponseApimodelObject,
        Error<openapi::apis::search_api::SearchSpaceError>,
    > {
        let mut req = openapi::models::ApimodelPeriodSearchRequest::new();
        req.types = types.into();
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
    ) -> Result<ApimodelPeriodSpaceResponse, Error<openapi::apis::spaces_api::GetSpaceError>> {
        openapi::apis::spaces_api::get_space(&self.config, API_VERSION, &space_id()).await
    }
    pub async fn get_property(
        &self,
        space_id: &str,
        property_id: String,
    ) -> Result<
        ApimodelPeriodPropertyResponse,
        Error<openapi::apis::properties_api::GetPropertyError>,
    > {
        openapi::apis::properties_api::get_property(
            &self.config,
            API_VERSION,
            space_id,
            &property_id.to_string(),
        )
        .await
    }
    pub async fn get_object(
        &self,
        space_id: Signal<String>,
        object_id: Signal<String>,
    ) -> Result<ApimodelPeriodObjectResponse, Error<openapi::apis::objects_api::GetObjectError>>
    {
        openapi::apis::objects_api::get_object(
            &self.config,
            API_VERSION,
            &space_id(),
            &object_id(),
            None,
        )
        .await
    }
    pub async fn get_list_objects(
        &self,
        space_id: Signal<String>,
        list_id: Signal<String>,
        view_id: Signal<String>,
    ) -> Result<
        PaginationPeriodPaginatedResponseApimodelObject,
        Error<openapi::apis::lists_api::GetListObjectsError>,
    > {
        openapi::apis::lists_api::get_list_objects(
            &self.config,
            API_VERSION,
            &space_id(),
            &list_id(),
            &view_id(),
            None,
            None,
        )
        .await
    }
    pub fn update_done_property(&self, space_id: String, object_id: String, done: Option<bool>) {
        let config = self.config.clone();
        spawn(async move {
            let mut prop = ApimodelPeriodCheckboxPropertyLinkValue::new();
            prop.key = "done".to_string().into();
            prop.checkbox = done;
            let mut req = ApimodelPeriodUpdateObjectRequest::new();
            req.properties = Some(vec![
                ApimodelPeriodPropertyLinkWithValue::ApimodelPeriodCheckboxPropertyLinkValue(
                    Box::new(prop),
                ),
            ]);
            tracing::debug!("{:#?}", req);
            let res = openapi::apis::objects_api::update_object(
                &config,
                API_VERSION,
                &space_id,
                &object_id,
                req,
            )
            .await;
            tracing::debug!("{:#?}", res);
        });
    }
    pub fn update_datetime_property(
        &self,
        space_id: String,
        object_id: String,
        property_key: String,
        date: UtcDateTime,
    ) {
        let config = self.config.clone();
        spawn(async move {
            let mut prop = ApimodelPeriodDatePropertyLinkValue::new();
            prop.key = property_key.into();
            tracing::debug!("debug {:#?}", date);
            prop.date = date.format(&Rfc3339).unwrap().into();
            let mut req = ApimodelPeriodUpdateObjectRequest::new();
            req.properties = Some(vec![
                ApimodelPeriodPropertyLinkWithValue::ApimodelPeriodDatePropertyLinkValue(Box::new(
                    prop,
                )),
            ]);
            tracing::debug!("{:#?}", req);
            let res = openapi::apis::objects_api::update_object(
                &config,
                API_VERSION,
                &space_id,
                &object_id,
                req,
            )
            .await;
            tracing::debug!("{:#?}", res);
        });
    }
    pub async fn update_select_property(
        &self,
        space_id: String,
        object_id: String,
        property_key: String,
        option: Option<String>,
    ) {
        let mut prop = ApimodelPeriodSelectPropertyLinkValue::new();
        prop.key = property_key.into();
        prop.select = option;
        let mut req = ApimodelPeriodUpdateObjectRequest::new();
        req.properties = Some(vec![
            ApimodelPeriodPropertyLinkWithValue::ApimodelPeriodSelectPropertyLinkValue(Box::new(
                prop,
            )),
        ]);
        let res = openapi::apis::objects_api::update_object(
            &self.config,
            API_VERSION,
            &space_id,
            &object_id,
            req,
        )
        .await;
        tracing::debug!("{:#?}", res);
    }
}
