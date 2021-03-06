/*
 * Imgur API
 *
 * Imgur's API exposes the entire Imgur infrastructure via a standardized programmatic interface. Using Imgur's API, you can do just about anything you can do on imgur.com, while using your programming language of choice.
 *
 * The version of the OpenAPI document: 0.4.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`generate_access_token`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenerateAccessTokenError {
    UnknownValue(serde_json::Value),
}


pub async fn generate_access_token(configuration: &configuration::Configuration, client_id: &str, client_secret: &str, grant_type: &str, refresh_token: &str) -> Result<crate::models::AccessToken, Error<GenerateAccessTokenError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/oauth/token", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    let mut local_var_form_params = std::collections::HashMap::new();
    local_var_form_params.insert("client_id", client_id.to_string());
    local_var_form_params.insert("client_secret", client_secret.to_string());
    local_var_form_params.insert("grant_type", grant_type.to_string());
    local_var_form_params.insert("refresh_token", refresh_token.to_string());
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GenerateAccessTokenError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

