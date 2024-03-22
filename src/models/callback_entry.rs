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

/// CallbackEntry : an entry in the callbacks register.  this schema is used by the callbacks api
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CallbackEntry {
    /// unique id to identify this callback (optional)
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// http/https url to call back on update
    #[serde(rename = "url")]
    pub url: String,
    /// optional http config for the target url (i.e: http method, headers, etc)
    #[serde(rename = "config", skip_serializing_if = "Option::is_none")]
    pub config: Option<Box<models::HttpFetcherConfig>>,
}

impl CallbackEntry {
    /// an entry in the callbacks register.  this schema is used by the callbacks api
    pub fn new(url: String) -> CallbackEntry {
        CallbackEntry {
            key: None,
            url,
            config: None,
        }
    }
}

