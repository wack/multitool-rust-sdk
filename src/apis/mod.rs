use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl<T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl<T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl<T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl<T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub fn parse_deep_object(prefix: &str, value: &serde_json::Value) -> Vec<(String, String)> {
    if let serde_json::Value::Object(object) = value {
        let mut params = vec![];

        for (key, value) in object {
            match value {
                serde_json::Value::Object(_) => params.append(&mut parse_deep_object(
                    &format!("{}[{}]", prefix, key),
                    value,
                )),
                serde_json::Value::Array(array) => {
                    for (i, value) in array.iter().enumerate() {
                        params.append(&mut parse_deep_object(
                            &format!("{}[{}][{}]", prefix, key, i),
                            value,
                        ));
                    }
                }
                serde_json::Value::String(s) => {
                    params.push((format!("{}[{}]", prefix, key), s.clone()))
                }
                _ => params.push((format!("{}[{}]", prefix, key), value.to_string())),
            }
        }

        return params;
    }

    unimplemented!("Only objects are supported with style=deepObject")
}

/// Internal use only
/// A content type supported by this client.
#[allow(dead_code)]
enum ContentType {
    Json,
    Text,
    Unsupported(String),
}

impl From<&str> for ContentType {
    fn from(content_type: &str) -> Self {
        if content_type.starts_with("application") && content_type.contains("json") {
            return Self::Json;
        } else if content_type.starts_with("text/plain") {
            return Self::Text;
        } else {
            return Self::Unsupported(content_type.to_string());
        }
    }
}

pub mod applications_api;
pub mod heartbeat_api;
pub mod response_code_metrics_api;
pub mod rollout_states_api;
pub mod rollouts_api;
pub mod users_api;
pub mod workspaces_api;

pub mod configuration;

use std::sync::Arc;

pub trait Api {
    fn applications_api(&self) -> &dyn applications_api::ApplicationsApi;
    fn heartbeat_api(&self) -> &dyn heartbeat_api::HeartbeatApi;
    fn response_code_metrics_api(&self) -> &dyn response_code_metrics_api::ResponseCodeMetricsApi;
    fn rollout_states_api(&self) -> &dyn rollout_states_api::RolloutStatesApi;
    fn rollouts_api(&self) -> &dyn rollouts_api::RolloutsApi;
    fn users_api(&self) -> &dyn users_api::UsersApi;
    fn workspaces_api(&self) -> &dyn workspaces_api::WorkspacesApi;
}

pub struct ApiClient {
    applications_api: Box<dyn applications_api::ApplicationsApi>,
    heartbeat_api: Box<dyn heartbeat_api::HeartbeatApi>,
    response_code_metrics_api: Box<dyn response_code_metrics_api::ResponseCodeMetricsApi>,
    rollout_states_api: Box<dyn rollout_states_api::RolloutStatesApi>,
    rollouts_api: Box<dyn rollouts_api::RolloutsApi>,
    users_api: Box<dyn users_api::UsersApi>,
    workspaces_api: Box<dyn workspaces_api::WorkspacesApi>,
}

impl ApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self {
            applications_api: Box::new(applications_api::ApplicationsApiClient::new(
                configuration.clone(),
            )),
            heartbeat_api: Box::new(heartbeat_api::HeartbeatApiClient::new(
                configuration.clone(),
            )),
            response_code_metrics_api: Box::new(
                response_code_metrics_api::ResponseCodeMetricsApiClient::new(configuration.clone()),
            ),
            rollout_states_api: Box::new(rollout_states_api::RolloutStatesApiClient::new(
                configuration.clone(),
            )),
            rollouts_api: Box::new(rollouts_api::RolloutsApiClient::new(configuration.clone())),
            users_api: Box::new(users_api::UsersApiClient::new(configuration.clone())),
            workspaces_api: Box::new(workspaces_api::WorkspacesApiClient::new(
                configuration.clone(),
            )),
        }
    }
}

impl Api for ApiClient {
    fn applications_api(&self) -> &dyn applications_api::ApplicationsApi {
        self.applications_api.as_ref()
    }
    fn heartbeat_api(&self) -> &dyn heartbeat_api::HeartbeatApi {
        self.heartbeat_api.as_ref()
    }
    fn response_code_metrics_api(&self) -> &dyn response_code_metrics_api::ResponseCodeMetricsApi {
        self.response_code_metrics_api.as_ref()
    }
    fn rollout_states_api(&self) -> &dyn rollout_states_api::RolloutStatesApi {
        self.rollout_states_api.as_ref()
    }
    fn rollouts_api(&self) -> &dyn rollouts_api::RolloutsApi {
        self.rollouts_api.as_ref()
    }
    fn users_api(&self) -> &dyn users_api::UsersApi {
        self.users_api.as_ref()
    }
    fn workspaces_api(&self) -> &dyn workspaces_api::WorkspacesApi {
        self.workspaces_api.as_ref()
    }
}

#[cfg(feature = "mockall")]
pub struct MockApiClient {
    pub applications_api_mock: applications_api::MockApplicationsApi,
    pub heartbeat_api_mock: heartbeat_api::MockHeartbeatApi,
    pub response_code_metrics_api_mock: response_code_metrics_api::MockResponseCodeMetricsApi,
    pub rollout_states_api_mock: rollout_states_api::MockRolloutStatesApi,
    pub rollouts_api_mock: rollouts_api::MockRolloutsApi,
    pub users_api_mock: users_api::MockUsersApi,
    pub workspaces_api_mock: workspaces_api::MockWorkspacesApi,
}

#[cfg(feature = "mockall")]
impl MockApiClient {
    pub fn new() -> Self {
        Self {
            applications_api_mock: applications_api::MockApplicationsApi::new(),
            heartbeat_api_mock: heartbeat_api::MockHeartbeatApi::new(),
            response_code_metrics_api_mock:
                response_code_metrics_api::MockResponseCodeMetricsApi::new(),
            rollout_states_api_mock: rollout_states_api::MockRolloutStatesApi::new(),
            rollouts_api_mock: rollouts_api::MockRolloutsApi::new(),
            users_api_mock: users_api::MockUsersApi::new(),
            workspaces_api_mock: workspaces_api::MockWorkspacesApi::new(),
        }
    }
}

#[cfg(feature = "mockall")]
impl Api for MockApiClient {
    fn applications_api(&self) -> &dyn applications_api::ApplicationsApi {
        &self.applications_api_mock
    }
    fn heartbeat_api(&self) -> &dyn heartbeat_api::HeartbeatApi {
        &self.heartbeat_api_mock
    }
    fn response_code_metrics_api(&self) -> &dyn response_code_metrics_api::ResponseCodeMetricsApi {
        &self.response_code_metrics_api_mock
    }
    fn rollout_states_api(&self) -> &dyn rollout_states_api::RolloutStatesApi {
        &self.rollout_states_api_mock
    }
    fn rollouts_api(&self) -> &dyn rollouts_api::RolloutsApi {
        &self.rollouts_api_mock
    }
    fn users_api(&self) -> &dyn users_api::UsersApi {
        &self.users_api_mock
    }
    fn workspaces_api(&self) -> &dyn workspaces_api::WorkspacesApi {
        &self.workspaces_api_mock
    }
}
