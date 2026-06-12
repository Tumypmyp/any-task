use crate::helpers::models::*;
use crate::protos::Event;
use crate::protos::StreamRequest;
use crate::protos::anytype_model::RelationFormat;
use crate::protos::anytype_model::block::content::dataview;
use crate::protos::anytype_model::block::*;
use crate::protos::anytype_model::object_type::*;
use crate::protos::client_commands_client::ClientCommandsClient;
use crate::protos::client_commands_client::*;
use crate::protos::event::Message;
use crate::protos::event::message::Value::*;
use crate::protos::rpc::*;
use anyhow::Context;
use anyhow::Result;
use dioxus::prelude::*;
use std::collections::HashMap;
use tonic::Request;
use tonic::metadata::MetadataValue;
use tonic::service::interceptor::InterceptedService;
use tonic::transport::Channel;
pub static API_CLIENT: GlobalSignal<Option<Client>> = Signal::global(|| None);
pub static RECONNECT_COUNT: GlobalSignal<u32> = Signal::global(|| 0);

#[derive(Clone, Debug)]
pub struct Client {
    pub client: ClientCommandsClient<InterceptedService<Channel, AuthInterceptor>>,
    pub account_id: String,
    pub tech_space_id: String,
    pub network_id: String,
    pub token: String,
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

pub fn extract_string(val: Option<&prost_types::Value>) -> String {
    if let Some(prost_types::Value {
        kind: Some(prost_types::value::Kind::StringValue(s)),
    }) = val
    {
        s.clone()
    } else {
        String::new()
    }
}
pub fn extract_number(val: Option<&prost_types::Value>) -> i32 {
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
                token: session_res.token.clone(),
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
            token: session_res.token.clone(),
        })
    }
    pub async fn listen_session_events(
        &mut self,
    ) -> Result<tonic::Response<tonic::codec::Streaming<Event>>, tonic::Status> {
        self.client
            .listen_session_events(StreamRequest {
                token: self.token.clone(),
            })
            .await
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
    pub async fn fetch_properties(&self, space_id: &str) -> Result<Vec<RelationInfo>> {
        let mut grpc_client = self.client.clone();
        let req = Request::new(object::search::Request {
            space_id: space_id.to_string(),
            filters: vec![content::dataview::Filter {
                operator: content::dataview::filter::Operator::No.into(),
                relation_key: "resolvedLayout".to_string(),
                condition: content::dataview::filter::Condition::Equal.into(),

                value: Some(prost_types::Value {
                    kind: Some(prost_types::value::Kind::NumberValue(5.0)),
                }),
                ..Default::default()
            }],
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
                RelationInfo {
                    name: name.clone(),
                    key: RelationKey(key.clone()),
                    // optional: OptionalInfo::Other,
                }
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
    /// Registers the spaces subscription and returns the initial snapshot.
    /// The subscription stays alive server-side until `unsubscribe_spaces` is called.
    pub async fn subscribe_spaces(&self) -> Result<object::search_subscribe::Response> {
        let mut grpc_client = self.client.clone();
        let req = Request::new(object::search_subscribe::Request {
            space_id: self.tech_space_id.clone(),
            sub_id: SPACES_SUB.to_string(),
            filters: vec![
                content::dataview::Filter {
                    relation_key: "resolvedLayout".to_string(),
                    condition: content::dataview::filter::Condition::Equal.into(),
                    value: Some(prost_types::Value {
                        kind: Some(prost_types::value::Kind::NumberValue(
                            Layout::SpaceView as i32 as f64,
                        )),
                    }),
                    ..Default::default()
                },
                // spaceAccountStatus NOT IN [SpaceDeleted(7), SpaceRemoving(10)]
                // content::dataview::Filter {
                //     relation_key: "spaceAccountStatus".to_string(),
                //     condition: content::dataview::filter::Condition::NotIn.into(),
                //     value: Some(prost_types::Value {
                //         kind: Some(prost_types::value::Kind::ListValue(
                //             prost_types::ListValue {
                //                 values: vec![
                //                     prost_types::Value {
                //                         kind: Some(prost_types::value::Kind::NumberValue(7.0)),
                //                     },
                //                     prost_types::Value {
                //                         kind: Some(prost_types::value::Kind::NumberValue(10.0)),
                //                     },
                //                 ],
                //             },
                //         )),
                //     }),
                //     ..Default::default()
                // },
            ],
            sorts: vec![/* spaceOrder asc if you want ordering */],
            keys: vec![
                "id".to_string(),
                "targetSpaceId".to_string(),
                "name".to_string(),
                "iconImage".to_string(),
                "iconOption".to_string(),
                "description".to_string(),
                "spaceLocalStatus".to_string(),
                "spaceOrder".to_string(),
            ],
            ..Default::default()
        });
        Ok(grpc_client
            .object_search_subscribe(req)
            .await
            .context("Search subscribe error")?
            .into_inner())
    }

    /// Cancels the spaces subscription.
    pub async fn unsubscribe_spaces(&self) -> Result<()> {
        let mut grpc_client = self.client.clone();
        let req = Request::new(object::search_unsubscribe::Request {
            sub_ids: vec![SPACES_SUB.to_string()],
        });
        grpc_client
            .object_search_unsubscribe(req)
            .await
            .context("Search unsubscribe error")?;
        Ok(())
    }
    pub async fn subscribe_sets(
        &self,
        space_id: String,
        sub_id: &str,
    ) -> Result<object::search_subscribe::Response> {
        let mut grpc_client = self.client.clone();
        let req = Request::new(object::search_subscribe::Request {
            space_id: space_id.to_string(),
            sub_id: sub_id.to_string(),
            filters: vec![content::dataview::Filter {
                relation_key: "resolvedLayout".to_string(),
                condition: content::dataview::filter::Condition::In.into(),
                value: Some(prost_types::Value {
                    kind: Some(prost_types::value::Kind::ListValue(
                        prost_types::ListValue {
                            values: vec![
                                prost_types::Value {
                                    kind: Some(prost_types::value::Kind::NumberValue(3.0)), // Set
                                },
                                prost_types::Value {
                                    kind: Some(prost_types::value::Kind::NumberValue(14.0)), // Collection
                                },
                            ],
                        },
                    )),
                }),
                ..Default::default()
            }],
            keys: vec![
                "id".to_string(),
                "name".to_string(),
                "resolvedLayout".to_string(),
                "iconEmoji".to_string(),
                "iconImage".to_string(),
            ],
            ..Default::default()
        });
        grpc_client
            .object_search_subscribe(req)
            .await
            .context("subscribe_sets error")
            .map(|r| r.into_inner())
    }

    pub async fn unsubscribe_sets(&self, sub_id: String) -> Result<()> {
        let mut grpc_client = self.client.clone();
        grpc_client
            .object_search_unsubscribe(Request::new(object::search_unsubscribe::Request {
                sub_ids: vec![sub_id.to_string()],
            }))
            .await
            .context("unsubscribe_sets error")?;
        Ok(())
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

fn get_string(v: prost_types::Value) -> String {
    match v.kind {
        Some(prost_types::value::Kind::StringValue(s)) => s,
        _ => String::new(),
    }
}
pub fn handle_msg(msg: Message) {
    match msg.value {
        // Fires before subscriptionAdd — store details by object id
        Some(ObjectDetailsSet(v)) if v.sub_ids.iter().any(|s| s == SPACES_SUB) => {
            let det = parse_space_details(&v.id, &v.details.unwrap_or_default().fields);
            SPACES.write().details.insert(v.id, det);
        }
        // Patch changed keys
        Some(ObjectDetailsAmend(v)) if v.sub_ids.iter().any(|s| s == SPACES_SUB) => {
            let mut state = SPACES.write();
            if let Some(det) = state.details.get_mut(&v.id) {
                tracing::debug!("amend: {:#?}", v.details);
                for kv in v.details {
                    match kv.key.as_str() {
                        "name" => det.name = get_string(kv.value.unwrap()),
                        "iconImage" => det.icon_image = get_string(kv.value.unwrap()),
                        "description" => det.description = get_string(kv.value.unwrap()),
                        "targetSpaceId" => det.target_space_id = get_string(kv.value.unwrap()),
                        _ => {}
                    }
                }
            }
        }
        // Insert into ordered list at the right position
        Some(SubscriptionAdd(v)) if v.sub_id == SPACES_SUB => {
            let mut state = SPACES.write();
            state.order.retain(|id| id != &v.id); // remove if already present
            if v.after_id.is_empty() {
                state.order.insert(0, v.id);
            } else {
                let pos = state
                    .order
                    .iter()
                    .position(|id| id == &v.after_id)
                    .map(|i| i + 1)
                    .unwrap_or(state.order.len());
                state.order.insert(pos, v.id);
            }
        }
        // Remove from both structures
        Some(SubscriptionRemove(v)) if v.sub_id == SPACES_SUB => {
            let mut state = SPACES.write();
            state.order.retain(|id| id != &v.id);
            state.details.remove(&v.id);
        }
        Some(ObjectDetailsSet(v)) if v.sub_ids.iter().any(|s| s.starts_with("sets-")) => {
            let det = SetDetails {
                object_id: v.id.clone(),
                name: extract_string(v.details.as_ref().and_then(|d| d.fields.get("name"))),
                layout: extract_number(
                    v.details
                        .as_ref()
                        .and_then(|d| d.fields.get("resolvedLayout")),
                ),
            };
            SETS.write().details.insert(v.id, det);
        }
        Some(ObjectDetailsAmend(v)) if v.sub_ids.iter().any(|s| s.starts_with("sets-")) => {
            let mut state = SETS.write();
            if let Some(det) = state.details.get_mut(&v.id) {
                for kv in v.details {
                    match kv.key.as_str() {
                        "name" => det.name = get_string(kv.value.unwrap()),
                        "resolvedLayout" => {
                            if let Some(prost_types::value::Kind::NumberValue(n)) =
                                kv.value.and_then(|v| v.kind)
                            {
                                det.layout = n as i32;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        Some(SubscriptionAdd(v)) if v.sub_id.starts_with("sets-") => {
            let mut state = SETS.write();
            state.order.retain(|id| id != &v.id);
            if v.after_id.is_empty() {
                state.order.insert(0, v.id);
            } else {
                let pos = state
                    .order
                    .iter()
                    .position(|id| id == &v.after_id)
                    .map(|i| i + 1)
                    .unwrap_or(state.order.len());
                state.order.insert(pos, v.id);
            }
        }
        Some(SubscriptionRemove(v)) if v.sub_id.starts_with("sets-") => {
            let mut state = SETS.write();
            state.order.retain(|id| id != &v.id);
            state.details.remove(&v.id);
        }
        // // objectDetailsSet — for any set-* subscription
        // Some(ObjectDetailsSet(v)) if v.sub_ids.iter().any(|s| s.starts_with("set-")) => {
        //     let mut states = LIST_STATES.write();
        //     for sub_id in v.sub_ids.iter().filter(|s| s.starts_with("set-")) {
        //         let state = states
        //             .entry(sub_id["set-".len()..].to_string())
        //             .or_default();
        //         let det = parse_object_details(&v.id, &v.details.as_ref().unwrap().fields);
        //         state.details.insert(v.id.clone(), det);
        //     }
        // }
        // // objectDetailsAmend
        // Some(ObjectDetailsAmend(v)) if v.sub_ids.iter().any(|s| s.starts_with("set-")) => {
        //     let mut states = LIST_STATES.write();
        //     for sub_id in v.sub_ids.iter().filter(|s| s.starts_with("set-")) {
        //         let list_id = &sub_id["set-".len()..];
        //         if let Some(state) = states.get_mut(list_id) {
        //             if let Some(det) = state.details.get_mut(&v.id) {
        //                 for kv in &v.details {
        //                     match kv.key.as_str() {
        //                         "name" => det.name = get_string(kv.value.clone().unwrap()),
        //                         _ => {}
        //                     }
        //                 }
        //             }
        //         }
        //     }
        // }
        // // subscriptionAdd
        // Some(SubscriptionAdd(v)) if v.sub_id.starts_with("set-") => {
        //     let list_id = v.sub_id["set-".len()..].to_string();
        //     let mut states = LIST_STATES.write();
        //     let state = states.entry(list_id).or_default();
        //     state.order.retain(|id| id != &v.id);
        //     if v.after_id.is_empty() {
        //         state.order.insert(0, v.id);
        //     } else {
        //         let pos = state
        //             .order
        //             .iter()
        //             .position(|id| id == &v.after_id)
        //             .map(|i| i + 1)
        //             .unwrap_or(state.order.len());
        //         state.order.insert(pos, v.id);
        //     }
        // }
        // // subscriptionRemove
        // Some(SubscriptionRemove(v)) if v.sub_id.starts_with("set-") => {
        //     let list_id = v.sub_id["set-".len()..].to_string();
        //     let mut states = LIST_STATES.write();
        //     if let Some(state) = states.get_mut(&list_id) {
        //         state.order.retain(|id| id != &v.id);
        //         state.details.remove(&v.id);
        //     }
        // }
        _ => {}
    }
}

pub static SPACES: GlobalSignal<SpacesState> = Signal::global(SpacesState::default);
pub const SPACES_SUB: &str = "spaces";
pub fn parse_space_details(
    object_id: &str,
    fields: &std::collections::BTreeMap<String, prost_types::Value>,
) -> SpaceDetails {
    SpaceDetails {
        object_id: object_id.to_string(),
        target_space_id: extract_string(fields.get("targetSpaceId")),
        name: extract_string(fields.get("name")),
        icon_image: extract_string(fields.get("iconImage")),
        description: extract_string(fields.get("description")),
    }
}

pub static SETS: GlobalSignal<SetsState> = Signal::global(SetsState::default);
pub fn sets_sub_id(space_id: String) -> String {
    format!("sets-{}", space_id)
}

pub static LIST_OBJECTS: GlobalSignal<ListObjectsState> = Signal::global(ListObjectsState::default);
