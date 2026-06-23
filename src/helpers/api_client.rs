use std::collections::HashMap;

use crate::helpers::models::*;
use crate::protos::Event;
use crate::protos::StreamRequest;
use crate::protos::anytype_model::RelationFormat;
use crate::protos::anytype_model::SpaceStatus;
// use crate::protos::anytype_model::block::content::dataview;
use crate::protos::anytype_model::block::*;
use crate::protos::anytype_model::object_type::*;
use crate::protos::client_commands_client::ClientCommandsClient;
use crate::protos::event::Message;
use crate::protos::event::message::Value::*;
use crate::protos::event::object::details::*;
use crate::protos::event::object::subscription::*;
use crate::protos::rpc::*;
use anyhow::Context;
use anyhow::Result;
use dioxus::prelude::*;
// use std::collections::HashMap;
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
    pub async fn recover_from_mnemonic(mnemonic: String, root_path_str: String) -> Result<Client> {
        let addr = "127.0.0.1:31020";
        let channel = Channel::from_shared(format!("http://{}", addr))?
            .connect()
            .await
            .context("Failed to connect channel")?;
        let mut setup_client = ClientCommandsClient::new(channel.clone());

        // Step 1: Recover wallet (same as init_from_mnemonic)
        setup_client
            .wallet_recover(wallet::recover::Request {
                root_path: root_path_str.clone(),
                mnemonic: mnemonic.clone(),
                ..Default::default()
            })
            .await
            .context("Failed to recover wallet")?;

        let _ = setup_client
            .initial_set_parameters(initial::set_parameters::Request {
                platform: "android".to_string(),
                version: "0.0.1".to_string(),
                workdir: root_path_str.clone(),
                ..Default::default()
            })
            .await;

        // Step 2: Create session — needed before account_recover
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
        let mut auth_client =
            ClientCommandsClient::with_interceptor(channel.clone(), interceptor.clone());

        // Step 3: Open event stream BEFORE triggering account_recover
        // so we don't miss the Account.Show event
        let mut event_stream = auth_client
            .listen_session_events(StreamRequest {
                token: session_res.token.clone(),
            }) // adjust method name to match your proto
            .await
            .context("Failed to open event stream")?
            .into_inner();

        // Step 4: Trigger account discovery (empty request)
        auth_client
            .account_recover(account::recover::Request {})
            .await
            .context("Failed to trigger account recovery")?;

        // Step 5: Wait for the first Account.Show event
        let account_id = tokio::time::timeout(std::time::Duration::from_secs(30), async {
            loop {
                match event_stream.message().await {
                    Ok(Some(event)) => {
                        for msg in event.messages {
                            if let Some(AccountShow(show)) = msg.value {
                                if let Some(account) = show.account {
                                    return Ok::<String, anyhow::Error>(account.id);
                                }
                            }
                        }
                    }
                    Ok(None) => return Err(anyhow::anyhow!("Event stream closed unexpectedly")),
                    Err(e) => return Err(anyhow::anyhow!("Event stream error: {e}")),
                }
            }
        })
        .await
        .map_err(|_| anyhow::anyhow!("Account discovery timed out after 30s"))??;

        // Step 6: Select the discovered account (same as init_from_mnemonic)
        let account_res = auth_client
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

        Ok(Self {
            client: auth_client,
            account_id: account.id,
            tech_space_id,
            network_id,
            token: session_res.token.clone(),
        })
    }
    pub async fn listen_session_events(
        &self,
    ) -> Result<tonic::Response<tonic::codec::Streaming<Event>>, tonic::Status> {
        self.client
            .clone()
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
    pub async fn fetch_properties(
        &self,
        space_id: &str,
    ) -> Result<HashMap<RelationKey, RelationInfo>> {
        let mut grpc_client = self.client.clone();
        let req = Request::new(object::search::Request {
            space_id: space_id.to_string(),
            filters: vec![content::dataview::Filter {
                operator: content::dataview::filter::Operator::No.into(),
                relation_key: "resolvedLayout".to_string(),
                condition: content::dataview::filter::Condition::Equal.into(),
                value: Some(prost_types::Value {
                    kind: Some(prost_types::value::Kind::NumberValue(
                        Layout::Relation as i32 as f64,
                    )),
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
                let name = extract_string(record.fields.get("name"));
                let key = extract_string(record.fields.get("relationKey"));
                let format =
                    RelationFormat::try_from(extract_number(record.fields.get("relationFormat")))
                        .unwrap_or(RelationFormat::Longtext);
                (
                    RelationKey(key.clone()),
                    RelationInfo {
                        name: name.clone(),
                        key: RelationKey(key.clone()),
                        format,
                    },
                )
            })
            .collect::<HashMap<RelationKey, RelationInfo>>();
        Ok(properties)
    }

    pub async fn subscribe_spaces(&self) -> Result<()> {
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
                content::dataview::Filter {
                    relation_key: "spaceAccountStatus".to_string(),
                    condition: content::dataview::filter::Condition::NotIn.into(),
                    value: Some(prost_types::Value {
                        kind: Some(prost_types::value::Kind::ListValue(
                            prost_types::ListValue {
                                values: vec![
                                    prost_types::Value {
                                        kind: Some(prost_types::value::Kind::NumberValue(
                                            SpaceStatus::SpaceDeleted as i32 as f64,
                                        )),
                                    },
                                    prost_types::Value {
                                        kind: Some(prost_types::value::Kind::NumberValue(
                                            SpaceStatus::SpaceRemoving as i32 as f64,
                                        )),
                                    },
                                ],
                            },
                        )),
                    }),
                    ..Default::default()
                },
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
        let resp = grpc_client
            .object_search_subscribe(req)
            .await
            .context("Search subscribe error")?
            .into_inner();

        let mut state = SPACES.write();
        state.order.clear();
        state.details.clear();
        for record in resp.records {
            let id = extract_string(record.fields.get("id"));
            let det = parse_space_details(&id, &record.fields);
            state.order.push(det.object_id.clone());
            state.details.insert(det.object_id.clone(), det);
        }

        Ok(())
    }

    pub async fn subscribe_sets(&self, space_id: &str) -> Result<()> {
        let sub_id = SetsState::sub_id(space_id);
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
                "setOf".to_string(),
            ],
            ..Default::default()
        });
        let resp = grpc_client
            .object_search_subscribe(req)
            .await
            .context("subscribe_sets error")?
            .into_inner();
        let mut state = SETS.write();
        state.order.clear();
        state.details.clear();
        for record in resp.records {
            let id = extract_string(record.fields.get("id"));
            let det = SetDetails {
                object_id: id.clone(),
                name: extract_string(record.fields.get("name")),
                layout: extract_number(record.fields.get("resolvedLayout")),
            };
            state.order.push(id.clone());
            state.details.insert(id, det);
        }
        Ok(())
    }

    pub async fn subscribe_list_objects(
        &self,
        space_id: &str,
        list_id: &str,
        set_of: Vec<String>,
        keys: Vec<String>,
        filters: Vec<content::dataview::Filter>,
        sorts: Vec<content::dataview::Sort>,
    ) -> Result<object::search_subscribe::Response> {
        let mut client = self.client.clone();
        let mut all_keys = keys;
        if !all_keys.contains(&"id".to_string()) {
            all_keys.insert(0, "id".to_string());
        }

        client
            .object_search_subscribe(Request::new(object::search_subscribe::Request {
                space_id: space_id.to_string(),
                sub_id: format!("list-{}", list_id),
                source: set_of,
                keys: all_keys,
                filters,
                sorts,
                ..Default::default()
            }))
            .await
            .context("subscribe_list_objects failed")
            .map(|r| r.into_inner())
    }

    // pub async fn subscribe_set_meta(
    //     &self,
    //     space_id: &str,
    //     set_id: &str,
    // ) -> Result<object::subscribe_ids::Response> {
    //     let mut client = self.client.clone();
    //     client
    //         .object_subscribe_ids(Request::new(object::subscribe_ids::Request {
    //             space_id: space_id.to_string(),
    //             sub_id: format!("set-meta-{}", set_id),
    //             ids: vec![set_id.to_string()],
    //             keys: vec!["id".into(), "name".into(), "setOf".into()],
    //             ..Default::default()
    //         }))
    //         .await
    //         .context("subscribe_set_meta failed")
    //         .map(|r| r.into_inner())
    // }
    pub async fn object_open(&self, space_id: &str, object_id: &str) -> Result<()> {
        let resp = self
            .client
            .clone()
            .object_open(Request::new(object::open::Request {
                space_id: space_id.to_string(),
                object_id: object_id.to_string(),
                include_relations_as_dependent_objects: true,
                ..Default::default()
            }))
            .await
            .context("object_open failed")?
            .into_inner();

        let object_view = resp
            .object_view
            .ok_or_else(|| anyhow::anyhow!("missing object_view for object {}", object_id))?;

        let fields = object_view
            .details
            .first()
            .and_then(|d| d.details.as_ref())
            .map(|s| &s.fields);
        let name = fields
            .and_then(|f| f.get("name"))
            .map(|v| extract_string(Some(v)))
            .unwrap_or_default();
        let set_of = fields
            .and_then(|f| f.get("setOf"))
            .map(|v| extract_list_strings_from_value(Some(v)))
            .unwrap_or_default();
        let dv_block = object_view
            .blocks
            .iter()
            .find(|b| b.id == "dataview")
            .ok_or_else(|| anyhow::anyhow!("got no views"))?;
        let dv = match &dv_block.content {
            Some(ContentOneOf::Dataview(dv)) => dv,
            _ => return Err(anyhow::anyhow!("no dataview")),
        };
        let mut state = SET_META.write();
        state.name = name;
        state.set_of = set_of;
        state.id = object_id.to_string();
        state.views = dv.views.clone();
        state.active_view_id = if !dv.active_view.is_empty() {
            dv.active_view.clone()
        } else {
            dv.views.first().map(|v| v.id.clone()).unwrap_or_default()
        };
        Ok(())
    }

    pub async fn object_close(
        &self,
        space_id: &str,
        object_id: &str,
    ) -> Result<object::close::Response> {
        self.client
            .clone()
            .object_close(Request::new(object::close::Request {
                space_id: space_id.to_string(),
                object_id: object_id.to_string(),
                ..Default::default()
            }))
            .await
            .context("object_close failed")
            .map(|r| r.into_inner())
    }

    async fn unsubscribe(&self, sub_id: String) -> Result<()> {
        self.client
            .clone()
            .object_search_unsubscribe(Request::new(object::search_unsubscribe::Request {
                sub_ids: vec![sub_id.clone()],
            }))
            .await
            .context(format!("Failed to unsubscribe from {}", sub_id))?;
        Ok(())
    }

    pub async fn unsubscribe_spaces(&self) -> Result<()> {
        self.unsubscribe(SPACES_SUB.to_string()).await
    }
    pub async fn unsubscribe_sets(&self, space_id: &str) -> Result<()> {
        let sub_id = SetsState::sub_id(space_id);
        self.unsubscribe(sub_id).await
    }
    pub async fn unsubscribe_list_objects(&self, list_id: &str) -> Result<()> {
        self.unsubscribe(format!("list-{}", list_id)).await
    }
    // pub async fn unsubscribe_set_meta(&self, set_id: &str) -> Result<()> {
    //     self.unsubscribe(format!("set-meta-{}", set_id)).await
    // }
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

use crate::protos::event::block::dataview::view_update::*;
use crate::protos::event::block::dataview::*;
pub fn handle_msg(context_id: &str, msg: Message) {
    match msg.value {
        Some(ObjectDetailsSet(v)) => {
            if v.sub_ids.iter().any(|s| s == SPACES_SUB) {
                SPACES.write().handle_set(v);
            } else if v.sub_ids.iter().any(|s| SetsState::matches_sub_id(s)) {
                SETS.write().handle_set(v);
            } else if v.sub_ids.iter().any(|s| s.starts_with(LIST_SUB_PREFIX)) {
                let fields: std::collections::BTreeMap<String, prost_types::Value> = v
                    .details
                    .as_ref()
                    .map(|d| d.fields.clone().into_iter().collect())
                    .unwrap_or_default();
                let det = ObjectDetails {
                    id: v.id.clone(),
                    name: extract_string(fields.get("name")),
                    fields,
                };
                LIST_OBJECTS.write().details.insert(v.id, det);
            } else if v.sub_ids.is_empty() && SET_META.read().id.contains(context_id) {
                SET_META.write().handle_set(v);
            }
        }
        Some(ObjectDetailsAmend(v)) => {
            if v.sub_ids.iter().any(|s| s == SPACES_SUB) {
                SPACES.write().handle_amend(v);
            } else if v.sub_ids.iter().any(|s| SetsState::matches_sub_id(s)) {
                SETS.write().handle_amend(v);
            } else if v.sub_ids.iter().any(|s| s.starts_with(LIST_SUB_PREFIX)) {
                let mut state = LIST_OBJECTS.write();
                if let Some(det) = state.details.get_mut(&v.id) {
                    for kv in v.details {
                        let val = kv.value.unwrap_or_default();
                        match kv.key.as_str() {
                            "name" => det.name = get_string(val.clone()),
                            _ => {}
                        }
                        det.fields.insert(kv.key, val);
                    }
                }
            } else if v.sub_ids.is_empty() && SET_META.read().id.contains(context_id) {
                SET_META.write().handle_amend(v);
            }
        }
        Some(SubscriptionAdd(v)) => {
            if v.sub_id == SPACES_SUB {
                SPACES.write().handle_add(v);
            } else if SetsState::matches_sub_id(&v.sub_id) {
                SETS.write().handle_add(v);
            } else if v.sub_id.starts_with(LIST_SUB_PREFIX) {
                insert_ordered(&mut LIST_OBJECTS.write().order, v.id, &v.after_id);
            }
        }
        Some(SubscriptionRemove(v)) => {
            if v.sub_id == SPACES_SUB {
                SPACES.write().handle_remove(v);
            } else if SetsState::matches_sub_id(&v.sub_id) {
                SETS.write().handle_remove(v);
            } else if v.sub_id.starts_with(LIST_SUB_PREFIX) {
                let mut state = LIST_OBJECTS.write();
                state.order.retain(|id| id != &v.id);
                state.details.remove(&v.id);
            }
        }
        Some(BlockDataviewViewSet(v)) => {
            SET_META.write().handle_view_set(v);
        }
        Some(BlockDataviewViewDelete(v)) => {
            SET_META.write().handle_view_delete(v);
        }
        Some(BlockDataviewViewUpdate(v)) => {
            SET_META.write().handle_view_update(v);
        }
        Some(BlockDataviewViewOrder(v)) => {
            SET_META.write().handle_view_order(v);
        }
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

impl SpacesState {
    pub fn handle_add(&mut self, v: Add) {
        insert_ordered(&mut self.order, v.id, &v.after_id);
    }
    pub fn handle_remove(&mut self, v: Remove) {
        self.order.retain(|id| id != &v.id);
        self.details.remove(&v.id);
    }
    pub fn handle_set(&mut self, v: Set) {
        let det = parse_space_details(&v.id, &v.details.unwrap_or_default().fields);
        self.details.insert(v.id, det);
    }
    pub fn handle_amend(&mut self, v: Amend) {
        let Some(det) = self.details.get_mut(&v.id) else {
            tracing::warn!("got amend, but space was not loaded");
            return;
        };
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
pub static SETS: GlobalSignal<SetsState> = Signal::global(SetsState::default);

impl SetsState {
    pub fn matches_sub_id(sub_id: &str) -> bool {
        sub_id.starts_with("sets-")
    }
    pub fn sub_id(space_id: &str) -> String {
        format!("sets-{}", space_id)
    }
    pub fn handle_add(&mut self, v: Add) {
        insert_ordered(&mut self.order, v.id, &v.after_id);
    }
    pub fn handle_remove(&mut self, v: Remove) {
        self.order.retain(|id| id != &v.id);
        self.details.remove(&v.id);
    }
    pub fn handle_set(&mut self, v: Set) {
        let det = SetDetails {
            object_id: v.id.clone(),
            name: extract_string(v.details.as_ref().and_then(|d| d.fields.get("name"))),
            layout: extract_number(
                v.details
                    .as_ref()
                    .and_then(|d| d.fields.get("resolvedLayout")),
            ),
        };
        self.details.insert(v.id, det);
    }
    pub fn handle_amend(&mut self, v: Amend) {
        let Some(details) = self.details.get_mut(&v.id) else {
            tracing::warn!("got amend, but set was not loaded");
            return;
        };
        for kv in v.details {
            match kv.key.as_str() {
                "name" => details.name = get_string(kv.value.unwrap()),
                "resolvedLayout" => details.layout = extract_number((&kv.value).into()),
                _ => {}
            }
        }
    }
}

pub fn extract_list_strings(v: Option<&prost_types::Value>) -> Vec<String> {
    v.and_then(|v| {
        if let Some(prost_types::value::Kind::ListValue(lv)) = &v.kind {
            Some(
                lv.values
                    .iter()
                    .filter_map(|v| {
                        if let Some(prost_types::value::Kind::StringValue(s)) = &v.kind {
                            Some(s.clone())
                        } else {
                            None
                        }
                    })
                    .collect(),
            )
        } else {
            None
        }
    })
    .unwrap_or_default()
}

pub fn extract_list_strings_from_value(v: Option<&prost_types::Value>) -> Vec<String> {
    extract_list_strings(v)
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

pub const LIST_SUB_PREFIX: &str = "list";
pub static LIST_OBJECTS: GlobalSignal<ListObjectsState> = Signal::global(ListObjectsState::default);

pub static SET_META: GlobalSignal<SetMetaState> = Signal::global(SetMetaState::default);

impl SetMetaState {
    pub fn handle_set(&mut self, v: Set) {
        let Some(fields) = v.details.map(|d| d.fields) else {
            return;
        };
        self.name = extract_string(fields.get("name"));
        self.set_of = extract_list_strings(fields.get("setOf"));
    }
    pub fn handle_amend(&mut self, v: Amend) {
        for kv in v.details {
            match kv.key.as_str() {
                "name" => self.name = get_string(kv.value.unwrap_or_default()),
                "setOf" => self.set_of = extract_list_strings_from_value(kv.value.as_ref()),
                _ => {}
            }
        }
    }
    pub fn handle_view_set(&mut self, v: ViewSet) {
        let Some(new_view) = v.view else { return };

        if let Some(existing) = self.views.iter_mut().find(|view| view.id == v.view_id) {
            *existing = new_view;
        } else {
            self.views.push(new_view);
        }
    }
    pub fn handle_view_delete(&mut self, v: ViewDelete) {
        self.views.retain(|view| view.id != v.view_id);

        if self.active_view_id == v.view_id {
            self.active_view_id = self
                .views
                .first()
                .map(|view| view.id.clone())
                .unwrap_or_default();
        }
    }

    pub fn handle_view_update(&mut self, v: ViewUpdate) {
        let Some(view) = self.views.iter_mut().find(|view| view.id == v.view_id) else {
            tracing::error!("got update on nonexisting view: {}", v.view_id);
            return;
        };

        if let Some(f) = v.fields {
            view.name = f.name;
        }

        // --- Filters ---
        for change in v.filter {
            match change.operation {
                Some(filter::Operation::Add(add)) => {
                    let pos =
                        insert_pos_by_id(&add.after_id, view.filters.iter().map(|f| f.id.as_str()));
                    for (i, item) in add.items.into_iter().enumerate() {
                        view.filters.insert(pos + i, item);
                    }
                }
                Some(filter::Operation::Remove(rem)) => {
                    view.filters.retain(|f| !rem.ids.contains(&f.id));
                }
                Some(filter::Operation::Update(u)) => {
                    if let Some(f) = view.filters.iter_mut().find(|f| f.id == u.id) {
                        if let Some(item) = u.item {
                            *f = item;
                        }
                    }
                }
                Some(filter::Operation::Move(mv)) => {
                    let mut moved = Vec::with_capacity(mv.ids.len());

                    for target_id in &mv.ids {
                        if let Some(idx) = view.filters.iter().position(|s| s.id == *target_id) {
                            moved.push(view.filters.remove(idx));
                        }
                    }

                    let pos =
                        insert_pos_by_id(&mv.after_id, view.filters.iter().map(|s| s.id.as_str()));

                    for (i, item) in moved.into_iter().enumerate() {
                        view.filters.insert(pos + i, item);
                    }
                }
                None => {}
            }
        }

        // --- Sorts ---
        for change in v.sort {
            match change.operation {
                Some(sort::Operation::Add(add)) => {
                    let pos =
                        insert_pos_by_id(&add.after_id, view.sorts.iter().map(|s| s.id.as_str()));
                    for (i, item) in add.items.into_iter().enumerate() {
                        view.sorts.insert(pos + i, item);
                    }
                }
                Some(sort::Operation::Remove(rem)) => {
                    view.sorts.retain(|s| !rem.ids.contains(&s.id));
                }
                Some(sort::Operation::Update(u)) => {
                    if let Some(s) = view.sorts.iter_mut().find(|s| s.id == u.id) {
                        if let Some(item) = u.item {
                            *s = item;
                        }
                    }
                }
                Some(sort::Operation::Move(mv)) => {
                    let mut moved = Vec::with_capacity(mv.ids.len());

                    for target_id in &mv.ids {
                        if let Some(idx) = view.sorts.iter().position(|s| s.id == *target_id) {
                            moved.push(view.sorts.remove(idx));
                        }
                    }

                    let pos =
                        insert_pos_by_id(&mv.after_id, view.sorts.iter().map(|s| s.id.as_str()));

                    for (i, item) in moved.into_iter().enumerate() {
                        view.sorts.insert(pos + i, item);
                    }
                }
                None => {}
            }
        }
    }
    pub fn handle_view_order(&mut self, v: ViewOrder) {
        self.views.sort_by_cached_key(|view| {
            v.view_ids
                .iter()
                .position(|id| id == &view.id)
                .unwrap_or(usize::MAX)
        });
    }
}

fn insert_ordered(order: &mut Vec<String>, id: String, after_id: &str) {
    order.retain(|existing| existing != &id);
    if after_id.is_empty() {
        order.insert(0, id);
    } else {
        let pos = order
            .iter()
            .position(|existing| existing == after_id)
            .map(|i| i + 1)
            .unwrap_or(order.len());
        order.insert(pos, id);
    }
}
pub fn parse_object_details(
    object_id: &str,
    fields: &std::collections::BTreeMap<String, prost_types::Value>,
) -> ObjectDetails {
    ObjectDetails {
        id: object_id.to_string(),
        name: extract_string(fields.get("name")),
        fields: fields.clone(),
    }
}

fn get_string(v: prost_types::Value) -> String {
    match v.kind {
        Some(prost_types::value::Kind::StringValue(s)) => s,
        _ => String::new(),
    }
}

/// Finds the insertion index for a new item.
/// If `after_id` is empty, or if the ID is not found, it defaults to index 0.
fn insert_pos_by_id<'a>(after_id: &str, mut ids: impl Iterator<Item = &'a str>) -> usize {
    if after_id.is_empty() {
        return 0;
    }
    ids.position(|id| id == after_id)
        .map(|i| i + 1)
        .unwrap_or(0)
}
