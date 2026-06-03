use crate::protos::anytype_model::RelationFormat;
use crate::protos::anytype_model::block::*;
use crate::protos::anytype_model::object_type::*;
use crate::protos::client_commands_client::ClientCommandsClient;
use crate::protos::rpc::*;
use anyhow::Context;
use anyhow::Result;
use dioxus::prelude::*;
use tonic::Request;
use tonic::metadata::MetadataValue;
use tonic::service::interceptor::InterceptedService;
use tonic::transport::Channel;
pub static API_CLIENT: GlobalSignal<Option<Client>> = Signal::global(|| None);
#[derive(Clone, Debug)]
pub struct Client {
    pub client: ClientCommandsClient<InterceptedService<Channel, AuthInterceptor>>,
    pub account_id: String,
    pub tech_space_id: String,
    pub network_id: String,
}
use tonic::Status;
use tonic::service::Interceptor;
#[derive(Clone, Debug)]
pub struct AuthInterceptor {
    token: MetadataValue<tonic::metadata::Ascii>,
}
impl Interceptor for AuthInterceptor {
    fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, Status> {
        request.metadata_mut().insert("token", self.token.clone());
        Ok(request)
    }
}
fn extract_string(val: Option<&prost_types::Value>) -> String {
    if let Some(prost_types::Value {
        kind: Some(prost_types::value::Kind::StringValue(s)),
    }) = val
    {
        s.clone()
    } else {
        String::new()
    }
}
fn extract_number(val: Option<&prost_types::Value>) -> i32 {
    if let Some(prost_types::Value {
        kind: Some(prost_types::value::Kind::NumberValue(n)),
    }) = val
    {
        *n as i32
    } else {
        0
    }
}
impl Client {
    pub async fn init_new_account(root_path_str: String) -> Result<(String, Client)> {
        let addr = "127.0.0.1:31020";
        let channel = Channel::from_shared(format!("http://{}", addr))?
            .connect()
            .await
            .context("Failed to connect channel")?;
        let mut setup_client = ClientCommandsClient::new(channel.clone());
        let wallet_res = setup_client
            .wallet_create(wallet::create::Request {
                root_path: root_path_str.clone(),
                ..Default::default()
            })
            .await
            .context("Failed to create wallet")?
            .into_inner();
        let mnemonic = wallet_res.mnemonic;
        _ = setup_client
            .initial_set_parameters(initial::set_parameters::Request {
                platform: "android".to_string(),
                version: "0.0.1".to_string(),
                workdir: root_path_str.clone(),
                ..Default::default()
            })
            .await;
        let account_res = setup_client
            .account_create(account::create::Request {
                name: "My New Account".into(),
                store_path: root_path_str,
                network_mode: 0,
                disable_local_network_sync: false,
                ..Default::default()
            })
            .await
            .context("Failed to create account")?
            .into_inner();
        let account = account_res
            .account
            .ok_or(anyhow::anyhow!("Account data missing"))?;
        let tech_space_id = account.info.clone().unwrap_or_default().tech_space_id;
        let network_id = account.info.clone().unwrap_or_default().network_id;
        let session_res = setup_client
            .wallet_create_session(wallet::create_session::Request {
                auth: Some(wallet::create_session::request::Auth::Mnemonic(
                    mnemonic.clone(),
                )),
            })
            .await
            .context("Failed to create wallet session")?
            .into_inner();
        let meta_val = MetadataValue::try_from(&session_res.token)
            .map_err(|_| anyhow::anyhow!("Invalid token format"))?;
        let interceptor = AuthInterceptor { token: meta_val };
        let authenticated_client = ClientCommandsClient::with_interceptor(channel, interceptor);
        Ok((
            mnemonic,
            Self {
                client: authenticated_client,
                account_id: account.id,
                tech_space_id,
                network_id,
            },
        ))
    }
    /// Bootstraps the application from a saved mnemonic,
    /// populating the global API_CLIENT state along the way.
    pub async fn init_from_mnemonic(
        mnemonic: String,
        account_id: String,
        root_path_str: String,
    ) -> Result<Client> {
        let addr = "127.0.0.1:31020";
        let channel = Channel::from_shared(format!("http://{}", addr))?
            .connect()
            .await
            .context("Failed to connect channel")?;
        let mut setup_client = ClientCommandsClient::new(channel.clone());
        setup_client
            .wallet_recover(wallet::recover::Request {
                root_path: root_path_str.clone(),
                mnemonic: mnemonic.clone(),
                ..Default::default()
            })
            .await
            .context("Failed to recover wallet")?;
        _ = setup_client
            .initial_set_parameters(initial::set_parameters::Request {
                platform: "android".to_string(),
                version: "0.0.1".to_string(),
                workdir: root_path_str.clone(),
                ..Default::default()
            })
            .await;
        let account_res = setup_client
            .account_select(account::select::Request {
                root_path: root_path_str.clone(),
                id: account_id,
                ..Default::default()
            })
            .await
            .context("Failed to select account")?
            .into_inner();
        let account = account_res.account.context("Account data missing")?;
        let network_id = account.info.clone().unwrap_or_default().network_id;
        let tech_space_id = account.info.unwrap_or_default().tech_space_id;
        let session_res = setup_client
            .wallet_create_session(wallet::create_session::Request {
                auth: Some(wallet::create_session::request::Auth::Mnemonic(
                    mnemonic.clone(),
                )),
            })
            .await
            .context("Failed to create wallet session")?
            .into_inner();
        let meta_val = MetadataValue::try_from(&session_res.token)
            .map_err(|_| anyhow::anyhow!("Invalid token format"))?;
        let interceptor = AuthInterceptor { token: meta_val };
        let authenticated_client = ClientCommandsClient::with_interceptor(channel, interceptor);
        Ok(Self {
            client: authenticated_client,
            account_id: account.id,
            tech_space_id,
            network_id,
        })
    }
    /// Subscribes to the object search and parses out the target space IDs.
    pub async fn fetch_spaces(&self) -> Result<Vec<(String, String)>> {
        let mut grpc_client = self.client.clone();
        let req = Request::new(object::search_subscribe::Request {
            space_id: self.tech_space_id.clone(),
            sub_id: "space".to_string(),
            filters: vec![content::dataview::Filter {
                operator: 0,
                relation_key: "spaceLocalStatus".to_string(),
                condition: 1,
                value: Some(prost_types::Value {
                    kind: Some(prost_types::value::Kind::NumberValue(2.0)),
                }),
                ..Default::default()
            }],
            keys: vec!["targetSpaceId".to_string(), "name".to_string()],
            ..Default::default()
        });
        let response = grpc_client
            .object_search_subscribe(req)
            .await
            .context("Search subscribe error")?
            .into_inner();
        Ok(response
            .records
            .into_iter()
            .map(|record| {
                let id = extract_string(record.fields.get("targetSpaceId"));
                let name = extract_string(record.fields.get("name"));
                (id, name)
            })
            .collect())
    }
    pub async fn join_space_from_link(&self, url: &str) -> Result<String> {
        let mut client = self.client.clone();
        let invite = parse_invite_url(url)?;
        let preview_req = tonic::Request::new(space::invite_view::Request {
            invite_cid: invite.cid.clone(),
            invite_file_key: invite.key.clone(),
        });
        let preview_res: space::invite_view::Response =
            client.space_invite_view(preview_req).await?.into_inner();
        tracing::info!("preview: {:#?}", preview_res);
        if let Some(error) = preview_res.error {
            if error.code != 0 {
                anyhow::bail!(
                    "Failed to preview space invite (code {}): {}",
                    error.code,
                    error.description
                );
            }
        }
        let target_space_id = if !invite.space_id.is_empty() {
            invite.space_id
        } else {
            preview_res.space_id
        };
        let join_req = tonic::Request::new(space::join::Request {
            space_id: target_space_id,
            invite_cid: invite.cid,
            invite_file_key: invite.key,
            network_id: self.network_id.clone(),
        });
        let join_res = client.space_join(join_req).await?.into_inner();
        if let Some(error) = join_res.error {
            if error.code != 0 {
                anyhow::bail!(
                    "Failed to join space (code {}): {}",
                    error.code,
                    error.description
                );
            }
        }
        Ok(preview_res.space_name)
    }
    pub async fn fetch_sets(&self, space_id: &str) -> Result<Vec<(String, String, i32)>> {
        let mut grpc_client = self.client.clone();
        let req = Request::new(object::search::Request {
            space_id: space_id.to_string(),
            filters: vec![
                content::dataview::Filter {
                    operator: 0,
                    relation_key: "resolvedLayout".to_string(),
                    condition: 9,
                    value: Some(prost_types::Value {
                        kind: Some(prost_types::value::Kind::ListValue(
                            prost_types::ListValue {
                                values: vec![prost_types::Value {
                                    kind: Some(prost_types::value::Kind::NumberValue(
                                        Layout::Set as i32 as f64,
                                    )),
                                }],
                            },
                        )),
                    }),
                    ..Default::default()
                },
                content::dataview::Filter {
                    operator: 0,
                    relation_key: "isHidden".to_string(),
                    condition: 2,
                    value: Some(prost_types::Value {
                        kind: Some(prost_types::value::Kind::BoolValue(true)),
                    }),
                    ..Default::default()
                },
            ],
            keys: vec![
                "id".to_string(),
                "name".to_string(),
                "resolvedLayout".to_string(),
            ],
            ..Default::default()
        });
        let response = grpc_client
            .object_search(req)
            .await
            .context("ObjectSearch error")?
            .into_inner();
        Ok(response
            .records
            .into_iter()
            .filter_map(|record| {
                let id = extract_string(record.fields.get("id"));
                let name = extract_string(record.fields.get("name"));
                let layout = extract_number(record.fields.get("resolvedLayout"));
                Some((id, name, layout))
            })
            .collect())
    }
    pub async fn fetch_properties(
        &self,
        space_id: &str,
    ) -> Result<Vec<(String, String, String, RelationFormat)>> {
        let mut grpc_client = self.client.clone();
        let req = Request::new(object::search::Request {
            space_id: space_id.to_string(),
            filters: vec![
                content::dataview::Filter {
                    operator: 0,
                    relation_key: "resolvedLayout".to_string(),
                    condition: 1,
                    value: Some(prost_types::Value {
                        kind: Some(prost_types::value::Kind::NumberValue(5.0)),
                    }),
                    ..Default::default()
                },
                content::dataview::Filter {
                    operator: 0,
                    relation_key: "isHidden".to_string(),
                    condition: 2,
                    value: Some(prost_types::Value {
                        kind: Some(prost_types::value::Kind::BoolValue(true)),
                    }),
                    ..Default::default()
                },
            ],
            keys: vec![
                "id".to_string(),
                "name".to_string(),
                "relationKey".to_string(),
                "relationFormat".to_string(),
                "description".to_string(),
            ],
            ..Default::default()
        });
        let response = grpc_client
            .object_search(req)
            .await
            .context("ObjectSearch error")?
            .into_inner();
        if let Some(e) = &response.error {
            if e.code != 0 {
                anyhow::bail!("fetch_properties error ({}): {}", e.code, e.description);
            }
        }
        let properties = response
            .records
            .into_iter()
            .map(|record| {
                let id = extract_string(record.fields.get("id"));
                let name = extract_string(record.fields.get("name"));
                let key = extract_string(record.fields.get("relationKey"));
                let format =
                    RelationFormat::try_from(extract_number(record.fields.get("relationFormat")))
                        .unwrap_or(RelationFormat::Longtext);
                (id, name, key, format)
            })
            .collect();
        Ok(properties)
    }
    pub async fn get_list_name(&self, space_id: &str, list_id: &str) -> anyhow::Result<String> {
        let mut client = self.client.clone();
        let req = tonic::Request::new(object::show::Request {
            object_id: list_id.to_string(),
            space_id: space_id.to_string(),
            ..Default::default()
        });
        let resp = client
            .object_show(req)
            .await
            .context("get_list_name error")?
            .into_inner();
        if let Some(e) = &resp.error {
            if e.code != 0 {
                anyhow::bail!("ObjectShow error ({}): {}", e.code, e.description);
            }
        }
        let name = extract_string(
            resp.object_view.context("no details")?.details[0]
                .details
                .clone()
                .context("no details 2")?
                .fields
                .get("name"),
        );
        Ok(name)
    }
    pub async fn get_list_objects(&self, space_id: &str, list_id: &str) -> Result<Vec<String>> {
        let mut client = self.client.clone();
        let show_req = tonic::Request::new(object::show::Request {
            space_id: space_id.to_string(),
            object_id: list_id.to_string(),
            ..Default::default()
        });
        let show_res = client
            .object_show(show_req)
            .await
            .context("get_list_name error")?
            .into_inner();
        if let Some(e) = &show_res.error {
            if e.code != 0 {
                anyhow::bail!("ObjectShow failed ({}): {}", e.code, e.description);
            }
        }
        let details = show_res
            .object_view
            .as_ref()
            .and_then(|v| v.details.first())
            .and_then(|d| d.details.as_ref())
            .ok_or_else(|| anyhow::anyhow!("no details in ObjectShow response"))?;
        let set_of_ids: Vec<String> =
            match details.fields.get("setOf").and_then(|v| v.kind.as_ref()) {
                Some(prost_types::value::Kind::ListValue(list)) => list
                    .values
                    .iter()
                    .map(|v| extract_string(Some(v)))
                    .collect(),
                _ => vec![],
            };
        let mut source_keys = Vec::new();
        for type_id in &set_of_ids {
            let req = tonic::Request::new(object::show::Request {
                space_id: space_id.to_string(),
                object_id: type_id.clone(),
                ..Default::default()
            });
            let res = client.object_show(req).await?.into_inner();
            let unique_key = res
                .object_view
                .as_ref()
                .and_then(|v| v.details.first())
                .and_then(|d| d.details.as_ref())
                .and_then(|d| d.fields.get("uniqueKey"))
                .and_then(|v| {
                    if let Some(prost_types::value::Kind::StringValue(s)) = &v.kind {
                        Some(s.clone())
                    } else {
                        None
                    }
                })
                .unwrap_or_default();
            if !unique_key.is_empty() {
                source_keys.push(unique_key);
            }
        }
        let search_req = tonic::Request::new(object::search_subscribe::Request {
            space_id: space_id.to_string(),
            sub_id: format!("set-{}", list_id),
            source: source_keys,
            keys: vec!["id".to_string()],
            ..Default::default()
        });
        let search_res = client
            .object_search_subscribe(search_req)
            .await?
            .into_inner();
        if let Some(e) = &search_res.error {
            if e.code != 0 {
                anyhow::bail!(
                    "ObjectSearchSubscribe failed ({}): {}",
                    e.code,
                    e.description
                );
            }
        }
        let results = search_res
            .records
            .into_iter()
            .filter_map(|record| {
                let id = extract_string(record.fields.get("id"));
                Some(id)
            })
            .collect();
        Ok(results)
    }
    pub async fn get_object_properties(
        &self,
        space_id: &str,
        object_id: &str,
    ) -> Result<std::collections::HashMap<String, prost_types::Value>> {
        let mut client = self.client.clone();
        let show_res = client
            .object_show(tonic::Request::new(object::show::Request {
                space_id: space_id.to_string(),
                object_id: object_id.to_string(),
                ..Default::default()
            }))
            .await
            .context("get_object_properties: object_show failed")?
            .into_inner();
        if let Some(e) = &show_res.error {
            if e.code != 0 {
                anyhow::bail!("ObjectShow failed ({}): {}", e.code, e.description);
            }
        }
        let fields = show_res
            .object_view
            .as_ref()
            .and_then(|v| v.details.first())
            .and_then(|d| d.details.as_ref())
            .map(|d| d.fields.clone())
            .unwrap_or_default();
        Ok(fields.into_iter().collect())
    }
}
use url::Url;
pub struct ParsedInvite {
    pub cid: String,
    pub key: String,
    pub space_id: String,
}
pub fn parse_invite_url(invite_url: &str) -> Result<ParsedInvite> {
    if invite_url.is_empty() {
        anyhow::bail!("Invite URL is empty");
    }
    let u = Url::parse(invite_url)?;
    if u.scheme() == "anytype" {
        let mut cid = String::new();
        let mut key = String::new();
        let mut space_id = String::new();
        for (k, v) in u.query_pairs() {
            match k.as_ref() {
                "cid" => cid = v.to_string(),
                "key" => key = v.to_string(),
                "spaceId" => space_id = v.to_string(),
                _ => {}
            }
        }
        return Ok(ParsedInvite { cid, key, space_id });
    }
    if u.scheme() == "http" || u.scheme() == "https" {
        let cid = u.path().trim_start_matches('/').to_string();
        let key = u.fragment().unwrap_or("").to_string();
        return Ok(ParsedInvite {
            cid,
            key,
            space_id: String::new(),
        });
    }
    anyhow::bail!("Invalid invite url scheme: {}", invite_url)
}
