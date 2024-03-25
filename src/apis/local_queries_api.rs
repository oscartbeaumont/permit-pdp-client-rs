/*
 * Permit.io PDP
 *
 * The PDP (Policy decision point) container wraps Open Policy Agent (OPA) with a higher-level API intended for fine grained application-level authorization. The PDP automatically handles pulling policy updates in real-time from a centrally managed cloud-service (api.permit.io).
 *
 * The version of the OpenAPI document: 0.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::{apis::ResponseContent, models};
use super::{Error, configuration};


/// struct for typed errors of method [`get_role_by_id_local_roles_role_id_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRoleByIdLocalRolesRoleIdGetError {
    Status404(models::Message),
    Status422(models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_role_by_name_local_roles_by_name_role_name_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRoleByNameLocalRolesByNameRoleNameGetError {
    Status404(models::Message),
    Status422(models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_user_local_users_user_id_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUserLocalUsersUserIdGetError {
    Status404(models::Message),
    Status422(models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_user_permissions_local_users_user_id_permissions_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUserPermissionsLocalUsersUserIdPermissionsGetError {
    Status404(models::Message),
    Status422(models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_user_roles_local_users_user_id_roles_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUserRolesLocalUsersUserIdRolesGetError {
    Status404(models::Message),
    Status422(models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_user_tenants_local_users_user_id_tenants_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUserTenantsLocalUsersUserIdTenantsGetError {
    Status404(models::Message),
    Status422(models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_roles_local_roles_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListRolesLocalRolesGetError {
    Status404(models::Message),
    Status422(models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_users_local_users_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListUsersLocalUsersGetError {
    Status404(models::Message),
    Status422(models::HttpValidationError),
    UnknownValue(serde_json::Value),
}


/// Get role (by the role id) directly from OPA cache.  If role is not found, returns 404. Possible reasons are either:  - role was never created via SDK or via the cloud console. - role was (very) recently created and the policy update did not propagate yet.
pub async fn get_role_by_id_local_roles_role_id_get(configuration: &configuration::Configuration, role_id: &str, authorization: Option<serde_json::Value>) -> Result<models::SyncedRole, Error<GetRoleByIdLocalRolesRoleIdGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/local/roles/{role_id}", local_var_configuration.base_path, role_id=crate::apis::urlencode(role_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = authorization {
        local_var_req_builder = local_var_req_builder.header("authorization", local_var_param_value.to_string());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetRoleByIdLocalRolesRoleIdGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get role (by the role name - case sensitive) directly from OPA cache.  If role is not found, returns 404. Possible reasons are either:  - role with such name was never created via SDK or via the cloud console. - role was (very) recently created and the policy update did not propagate yet.
pub async fn get_role_by_name_local_roles_by_name_role_name_get(configuration: &configuration::Configuration, role_name: &str, authorization: Option<serde_json::Value>) -> Result<models::SyncedRole, Error<GetRoleByNameLocalRolesByNameRoleNameGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/local/roles/by-name/{role_name}", local_var_configuration.base_path, role_name=crate::apis::urlencode(role_name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = authorization {
        local_var_req_builder = local_var_req_builder.header("authorization", local_var_param_value.to_string());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetRoleByNameLocalRolesByNameRoleNameGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get user data directly from OPA cache.  If user does not exist in OPA cache (i.e: not synced), returns 404.
pub async fn get_user_local_users_user_id_get(configuration: &configuration::Configuration, user_id: &str, authorization: Option<serde_json::Value>) -> Result<models::SyncedUser, Error<GetUserLocalUsersUserIdGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/local/users/{user_id}", local_var_configuration.base_path, user_id=crate::apis::urlencode(user_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = authorization {
        local_var_req_builder = local_var_req_builder.header("authorization", local_var_param_value.to_string());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetUserLocalUsersUserIdGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get roles **assigned to user** directly from OPA cache.  If user does not exist in OPA cache (i.e: not synced), returns 404.
pub async fn get_user_permissions_local_users_user_id_permissions_get(configuration: &configuration::Configuration, user_id: &str, authorization: Option<serde_json::Value>) -> Result<std::collections::HashMap<String, serde_json::Value>, Error<GetUserPermissionsLocalUsersUserIdPermissionsGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/local/users/{user_id}/permissions", local_var_configuration.base_path, user_id=crate::apis::urlencode(user_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = authorization {
        local_var_req_builder = local_var_req_builder.header("authorization", local_var_param_value.to_string());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetUserPermissionsLocalUsersUserIdPermissionsGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get roles **assigned to user** directly from OPA cache.  If user does not exist in OPA cache (i.e: not synced), returns 404.
pub async fn get_user_roles_local_users_user_id_roles_get(configuration: &configuration::Configuration, user_id: &str, authorization: Option<serde_json::Value>) -> Result<Vec<models::SyncedRole>, Error<GetUserRolesLocalUsersUserIdRolesGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/local/users/{user_id}/roles", local_var_configuration.base_path, user_id=crate::apis::urlencode(user_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = authorization {
        local_var_req_builder = local_var_req_builder.header("authorization", local_var_param_value.to_string());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetUserRolesLocalUsersUserIdRolesGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get tenants **assigned to user** directly from OPA cache. This endpoint only returns tenants that the user **has an assigned role in**. i.e: if the user is assigned to tenant \"tenant1\" but has no roles in that tenant, \"tenant1\" will not be returned by this endpoint.  If user does not exist in OPA cache (i.e: not synced), returns 404.
pub async fn get_user_tenants_local_users_user_id_tenants_get(configuration: &configuration::Configuration, user_id: &str, authorization: Option<serde_json::Value>) -> Result<Vec<String>, Error<GetUserTenantsLocalUsersUserIdTenantsGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/local/users/{user_id}/tenants", local_var_configuration.base_path, user_id=crate::apis::urlencode(user_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = authorization {
        local_var_req_builder = local_var_req_builder.header("authorization", local_var_param_value.to_string());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetUserTenantsLocalUsersUserIdTenantsGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get all roles stored in OPA cache.
pub async fn list_roles_local_roles_get(configuration: &configuration::Configuration, authorization: Option<serde_json::Value>) -> Result<Vec<models::SyncedRole>, Error<ListRolesLocalRolesGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/local/roles", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = authorization {
        local_var_req_builder = local_var_req_builder.header("authorization", local_var_param_value.to_string());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ListRolesLocalRolesGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get all users stored in OPA cache.  Be advised, if you have many (i.e: few hundreds or more) users this query might be expensive latency-wise.
pub async fn list_users_local_users_get(configuration: &configuration::Configuration, authorization: Option<serde_json::Value>) -> Result<Vec<models::SyncedUser>, Error<ListUsersLocalUsersGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/local/users", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = authorization {
        local_var_req_builder = local_var_req_builder.header("authorization", local_var_param_value.to_string());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ListUsersLocalUsersGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

