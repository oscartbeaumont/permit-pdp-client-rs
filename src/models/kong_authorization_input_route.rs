/*
 * Permit.io PDP
 *
 * The PDP (Policy decision point) container wraps Open Policy Agent (OPA) with a higher-level API intended for fine grained application-level authorization. The PDP automatically handles pulling policy updates in real-time from a centrally managed cloud-service (api.permit.io).
 *
 * The version of the OpenAPI document: 0.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct KongAuthorizationInputRoute {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "paths")]
    pub paths: Vec<String>,
    #[serde(rename = "protocols")]
    pub protocols: Vec<String>,
    #[serde(rename = "strip_path")]
    pub strip_path: bool,
    #[serde(rename = "created_at")]
    pub created_at: i32,
    #[serde(rename = "ws_id")]
    pub ws_id: uuid::Uuid,
    #[serde(rename = "request_buffering")]
    pub request_buffering: bool,
    #[serde(rename = "updated_at")]
    pub updated_at: i32,
    #[serde(rename = "preserve_host")]
    pub preserve_host: bool,
    #[serde(rename = "regex_priority")]
    pub regex_priority: i32,
    #[serde(rename = "response_buffering")]
    pub response_buffering: bool,
    #[serde(rename = "https_redirect_status_code")]
    pub https_redirect_status_code: i32,
    #[serde(rename = "path_handling")]
    pub path_handling: String,
    #[serde(rename = "service")]
    pub service: Box<models::KongAuthorizationInputRouteService>,
}

impl KongAuthorizationInputRoute {
    pub fn new(id: uuid::Uuid, paths: Vec<String>, protocols: Vec<String>, strip_path: bool, created_at: i32, ws_id: uuid::Uuid, request_buffering: bool, updated_at: i32, preserve_host: bool, regex_priority: i32, response_buffering: bool, https_redirect_status_code: i32, path_handling: String, service: models::KongAuthorizationInputRouteService) -> KongAuthorizationInputRoute {
        KongAuthorizationInputRoute {
            id,
            paths,
            protocols,
            strip_path,
            created_at,
            ws_id,
            request_buffering,
            updated_at,
            preserve_host,
            regex_priority,
            response_buffering,
            https_redirect_status_code,
            path_handling,
            service: Box::new(service),
        }
    }
}

