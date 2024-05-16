use reqwest::Method;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{
    apis::{configuration, StatusCode},
    client::Client,
};

#[macro_export]
macro_rules! invoke {
    ($configuration:expr, $request:expr, $path:expr) => {
        crate::invoker::Invoker {}
            .invoke($configuration, Some($request), $path)
            .await
    };
}

#[macro_export]
macro_rules! invoke_without_request {
    ($configuration:expr, $path:expr) => {
        crate::invoker::Invoker {}
            .invoke_without_request($configuration, $path)
            .await
    };
}

pub struct Invoker;

impl Invoker {
    pub async fn invoke<T: Client, U: Serialize, V: DeserializeOwned>(
        &self,
        configuration: &configuration::Configuration<T>,
        request: Option<U>,
        path: &str,
    ) -> anyhow::Result<V> {
        let local_var_configuration = configuration;
        let local_var_client = &local_var_configuration.client;
        let local_var_uri_str = format!("{}{}", local_var_configuration.base_path, path);
        let mut local_var_req_builder =
            local_var_client.request(Method::POST, local_var_uri_str.as_str());
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let local_var_req = match request {
            Some(req) => local_var_req_builder.json(&req).build()?,
            None => local_var_req_builder.build()?,
        };
        let local_var_resp = local_var_client
            .execute(local_var_req, configuration.call_options.clone())
            .await?;
        let local_var_status = StatusCode::from(local_var_resp.status);
        let local_var_content = local_var_resp.body;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_slice(&local_var_content).map_err(anyhow::Error::from)
        } else {
            anyhow::bail!(serde_json::to_string_pretty(local_var_content.as_slice())?)
        }
    }
    pub async fn invoke_without_request<T: Client, V: DeserializeOwned>(
        &self,
        configuration: &configuration::Configuration<T>,
        path: &str,
    ) -> anyhow::Result<V> {
        self.invoke::<T, &str, V>(configuration, None, path).await
    }
}
