use dioxus::prelude::*;
use tonic::Request;
use tonic::metadata::MetadataValue;
// const API_VERSION: &str = "2025-11-08";
use crate::protos::anytype::client_commands_client::ClientCommandsClient;
use crate::protos::anytype::rpc::*;
use tonic::transport::Channel;

pub static API_CLIENT: GlobalSignal<Option<Client>> = Signal::global(|| None);

#[derive(Clone, Debug)]
pub struct Client {
    pub inner: ClientCommandsClient<Channel>,
    pub account_id: String,
    pub tech_space_id: String,
    pub token: String,
}
impl Client {
    pub async fn init_new_account(root_path_str: String) -> Result<(String, Client), String> {
        let addr = "127.0.0.1:31020";
        let mut client = ClientCommandsClient::connect(format!("http://{}", addr))
            .await
            .unwrap();
        let _ = client.app_shutdown(app::shutdown::Request::default()).await;

        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        let wallet_res = client
            .wallet_create(wallet::create::Request {
                root_path: root_path_str.clone(),
                ..Default::default()
            })
            .await
            .map_err(|e| e.to_string())?
            .into_inner();
        let mnemonic = wallet_res.mnemonic;

        _ = client
            .initial_set_parameters(initial::set_parameters::Request {
                platform: "android".to_string(),
                version: "0.0.1".to_string(),
                workdir: root_path_str.clone(),
                ..Default::default()
            })
            .await;

        let account_res = client
            .account_create(account::create::Request {
                name: "My New Account".into(),
                store_path: root_path_str,
                network_mode: 1,
                ..Default::default()
            })
            .await
            .map_err(|e| e.to_string())?
            .into_inner();

        let account = account_res.account.ok_or("Account data missing")?;
        let tech_space_id = account.info.unwrap_or_default().tech_space_id;
        let session_res = client
            .wallet_create_session(wallet::create_session::Request {
                auth: Some(wallet::create_session::request::Auth::Mnemonic(
                    mnemonic.clone(),
                )),
            })
            .await
            .map_err(|e| format!("Failed to create wallet session: {}", e))?
            .into_inner();
        Ok((
            mnemonic,
            Self {
                inner: client,
                account_id: account.id,
                tech_space_id: tech_space_id,
                token: session_res.token,
            },
        ))
    }
    /// Bootstraps the application from a saved mnemonic,
    /// populating the global API_CLIENT state along the way.
    pub async fn init_from_mnemonic(
        mnemonic: String,
        account_id: String,
        root_path_str: String,
    ) -> Result<Client, String> {
        let addr = "127.0.0.1:31020";

        let mut client = ClientCommandsClient::connect(format!("http://{}", addr))
            .await
            .map_err(|e| format!("Failed to connect to engine: {}", e))?;
        client
            .wallet_recover(wallet::recover::Request {
                root_path: root_path_str.clone(),
                mnemonic: mnemonic.clone(),
                ..Default::default()
            })
            .await
            .map_err(|e| e.to_string())?;

        _ = client
            .initial_set_parameters(initial::set_parameters::Request {
                platform: "android".to_string(),
                version: "0.0.1".to_string(),
                workdir: root_path_str.clone(),
                ..Default::default()
            })
            .await;

        let account_res = client
            .account_select(account::select::Request {
                root_path: root_path_str.clone(),
                id: account_id,
                ..Default::default()
            })
            .await
            .map_err(|e| e.to_string())?
            .into_inner();

        let account = account_res.account.ok_or("Account data missing")?;
        let tech_space_id = account.info.unwrap_or_default().tech_space_id;
        let session_res = client
            .wallet_create_session(wallet::create_session::Request {
                auth: Some(wallet::create_session::request::Auth::Mnemonic(
                    mnemonic.clone(),
                )),
            })
            .await
            .map_err(|e| format!("Failed to create wallet session: {}", e))?
            .into_inner();
        Ok(Self {
            inner: client,
            account_id: account.id,
            tech_space_id: tech_space_id,
            token: session_res.token,
        })
    }
    /// Subscribes to the object search and parses out the target space IDs.
    pub async fn fetch_spaces(&self) -> Result<Vec<String>, String> {
        let mut grpc_client = self.inner.clone();
        let tech_space_id = self.tech_space_id.clone();
        let mut req = Request::new(object::search_subscribe::Request {
            space_id: tech_space_id,
            sub_id: "space".to_string(),
            keys: vec!["targetSpaceId".to_string()],

            ..Default::default()
        });

        // 2. Inject raw token into lowercase "token" metadata key
        let meta_val = MetadataValue::try_from(&self.token.clone())
            .map_err(|_| "Failed to parse token into metadata value".to_string())?;
        req.metadata_mut().insert("token", meta_val);
        // 2. Fetch Spaces using the stored tech_space_id
        // let list_sub_res = grpc_client
        //     .object_search_subscribe(object::search_subscribe::Request {
        //         space_id: tech_space_id, // Passed automatically!
        //         sub_id: "space".to_string(),
        //         // filters: vec![
        //         //     // resolvedLayout == spaceView (58)
        //         //     anytype::model::block::content::dataview::Filter {
        //         //         operator: 0,          // No
        //         //         relation_key: "resolvedLayout".to_string(),
        //         //         condition: 1,         // Equal
        //         //         value: Some(prost_types::Value {
        //         //             kind: Some(prost_types::value::Kind::NumberValue(58.0)),
        //         //         }),
        //         //         ..Default::default()
        //         //     },
        //         //     // spaceLocalStatus == Ok (0)
        //         //     anytype::model::block::content::dataview::Filter {
        //         //         operator: 0,
        //         //         relation_key: "spaceLocalStatus".to_string(),
        //         //         condition: 1,
        //         //         value: Some(prost_types::Value {
        //         //             kind: Some(prost_types::value::Kind::NumberValue(0.0)),
        //         //         }),
        //         //         ..Default::default()
        //         //     },
        //         // ],
        //         keys: vec!["targetSpaceId".to_string()],
        //         ..Default::default()
        //     })
        //     .await
        //     .map_err(|e| e.to_string())?;
        let list_sub_res = grpc_client
            .object_search_subscribe(req)
            .await
            .map_err(|e| format!("Search subscribe error: {}", e))?
            .into_inner();
        let response = list_sub_res;
        let mut spaces = Vec::new();
        tracing::debug!("spaces found: {:#?}", response);

        for record in response.records {
            let id_field = record
                .fields
                .get("id")
                .or_else(|| record.fields.get("targetSpaceId"));

            if let Some(id_val) = id_field {
                if let Some(prost_types::value::Kind::StringValue(id_str)) = &id_val.kind {
                    spaces.push(id_str.clone());
                }
            }
        }

        Ok(spaces)
    }
}

// impl Client {
//     fn new() -> Self {
//         Self {
//             config: Configuration::new(),
//         }
//     }
//     pub fn set_api_key(&mut self, token: String) {
//         self.config.bearer_access_token = Some(token);
//     }
//     pub fn set_server(&mut self, server: String) {
//         self.config.base_path = format!("http://{server}");
//     }
//     /// Gets the current server address and port, stripping the protocol prefix.
//     pub fn get_server(&self) -> String {
//         let path = &self.config.base_path;
//         if path.starts_with("http://") {
//             path[7..].to_string()
//         } else {
//             path.clone()
//         }
//     }
//     /// Gets the current token. Returns an empty string if no token is set.
//     pub fn get_token(&self) -> String {
//         self.config.bearer_access_token.clone().unwrap_or_default()
//     }
//     pub async fn create_auth_challenge(
//         &self,
//     ) -> Result<CreateChallengeResponse, Error<openapi::apis::auth_api::CreateAuthChallengeError>>
//     {
//         openapi::apis::auth_api::create_auth_challenge(
//             &self.config,
//             API_VERSION,
//             openapi::models::CreateChallengeRequest {
//                 app_name: Some("AnyTask".to_string()),
//             },
//         )
//         .await
//     }
//     pub async fn create_api_key(
//         &self,
//         challenge_id: String,
//         code: String,
//     ) -> Result<CreateApiKeyResponse, Error<openapi::apis::auth_api::CreateApiKeyError>> {
//         tracing::debug!("create_api_key: {:#?} {} {}", self, challenge_id, code);
//         openapi::apis::auth_api::create_api_key(
//             &self.config,
//             API_VERSION,
//             openapi::models::CreateApiKeyRequest {
//                 challenge_id: Some(challenge_id.to_string()),
//                 code: Some(code.to_string()),
//             },
//         )
//         .await
//     }
//     pub async fn list_spaces(
//         &self,
//     ) -> Result<PaginatedResponseSpace, Error<openapi::apis::spaces_api::ListSpacesError>> {
//         openapi::apis::spaces_api::list_spaces(&self.config, API_VERSION, None, None).await
//     }
//     pub async fn get_views(
//         &self,
//         space_id: &str,
//         list_id: &str,
//     ) -> Result<PaginatedResponseView, Error<openapi::apis::lists_api::GetListViewsError>> {
//         openapi::apis::lists_api::get_list_views(
//             &self.config,
//             API_VERSION,
//             space_id,
//             list_id,
//             None,
//             None,
//         )
//         .await
//     }
//     pub async fn list_properties(
//         &self,
//         space_id: &str,
//     ) -> Result<PaginatedResponseProperty, Error<openapi::apis::properties_api::ListPropertiesError>>
//     {
//         openapi::apis::properties_api::list_properties(
//             &self.config,
//             API_VERSION,
//             space_id,
//             None,
//             None,
//         )
//         .await
//     }
//     pub async fn list_select_property_options(
//         &self,
//         space_id: &str,
//         property_id: &str,
//     ) -> Result<openapi::models::PaginatedResponseTag, Error<openapi::apis::tags_api::ListTagsError>>
//     {
//         openapi::apis::tags_api::list_tags(&self.config, API_VERSION, space_id, property_id).await
//     }
//     pub async fn get_types(
//         &self,
//         space_id: String,
//         types: Vec<String>,
//     ) -> Result<PaginatedResponseObject, Error<openapi::apis::search_api::SearchSpaceError>> {
//         let mut req = openapi::models::SearchRequest::new();
//         req.types = types.into();
//         openapi::apis::search_api::search_space(
//             &self.config,
//             API_VERSION,
//             &space_id,
//             req,
//             None,
//             None,
//         )
//         .await
//     }
//     pub async fn get_space(
//         &self,
//         space_id: String,
//     ) -> Result<SpaceResponse, Error<openapi::apis::spaces_api::GetSpaceError>> {
//         openapi::apis::spaces_api::get_space(&self.config, API_VERSION, &space_id).await
//     }
//     pub async fn get_property(
//         &self,
//         space_id: &str,
//         property_id: String,
//     ) -> Result<PropertyResponse, Error<openapi::apis::properties_api::GetPropertyError>> {
//         openapi::apis::properties_api::get_property(
//             &self.config,
//             API_VERSION,
//             space_id,
//             &property_id.to_string(),
//         )
//         .await
//     }
//     pub async fn get_object(
//         &self,
//         space_id: Signal<String>,
//         object_id: Signal<String>,
//     ) -> Result<ObjectResponse, Error<openapi::apis::objects_api::GetObjectError>> {
//         openapi::apis::objects_api::get_object(
//             &self.config,
//             API_VERSION,
//             &space_id(),
//             &object_id(),
//             None,
//         )
//         .await
//     }
//     pub async fn get_list_objects(
//         &self,
//         space_id: Signal<String>,
//         list_id: Signal<String>,
//         view_id: String,
//     ) -> Result<PaginatedResponseObject, Error<openapi::apis::lists_api::GetListObjectsError>> {
//         tracing::debug!(
//             "get list objects -> space: {}, list: {}, view: {}",
//             space_id(),
//             list_id(),
//             view_id.clone(),
//         );
//         openapi::apis::lists_api::get_list_objects(
//             &self.config,
//             API_VERSION,
//             &space_id(),
//             &list_id(),
//             &view_id.clone(),
//             None,
//             None,
//         )
//         .await
//     }
//     pub fn update_done_property(&self, space_id: String, object_id: String, done: Option<bool>) {
//         let config = self.config.clone();
//         spawn(async move {
//             let mut prop = CheckboxPropertyLinkValue::new();
//             prop.key = "done".to_string().into();
//             prop.checkbox = done;
//             let mut req = UpdateObjectRequest::new();
//             req.properties = Some(vec![PropertyLinkWithValue::CheckboxPropertyLinkValue(
//                 Box::new(prop),
//             )]);
//             tracing::debug!("{:#?}", req);
//             let res = openapi::apis::objects_api::update_object(
//                 &config,
//                 API_VERSION,
//                 &space_id,
//                 &object_id,
//                 req,
//             )
//             .await;
//             tracing::debug!("{:#?}", res);
//         });
//     }
//     pub fn update_datetime_property(
//         &self,
//         space_id: String,
//         object_id: String,
//         property_key: String,
//         date: UtcDateTime,
//     ) {
//         let config = self.config.clone();
//         spawn(async move {
//             let mut prop = DatePropertyLinkValue::new();
//             prop.key = property_key.into();
//             tracing::debug!("debug {:#?}", date);
//             prop.date = date.format(&Rfc3339).unwrap().into();
//             let mut req = UpdateObjectRequest::new();
//             req.properties = Some(vec![PropertyLinkWithValue::DatePropertyLinkValue(
//                 Box::new(prop),
//             )]);
//             tracing::debug!("{:#?}", req);
//             let res = openapi::apis::objects_api::update_object(
//                 &config,
//                 API_VERSION,
//                 &space_id,
//                 &object_id,
//                 req,
//             )
//             .await;
//             tracing::debug!("{:#?}", res);
//         });
//     }
//     pub async fn update_select_property(
//         &self,
//         space_id: String,
//         object_id: String,
//         property_key: String,
//         option: Option<String>,
//     ) {
//         let mut prop = SelectPropertyLinkValue::new();
//         prop.key = property_key.into();
//         prop.select = option;
//         let mut req = UpdateObjectRequest::new();
//         req.properties = Some(vec![PropertyLinkWithValue::SelectPropertyLinkValue(
//             Box::new(prop),
//         )]);
//         let res = openapi::apis::objects_api::update_object(
//             &self.config,
//             API_VERSION,
//             &space_id,
//             &object_id,
//             req,
//         )
//         .await;
//         tracing::debug!("{:#?}", res);
//     }
// }
