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
pub struct BulkAuthorizationResult {
    #[serde(rename = "allow", skip_serializing_if = "Option::is_none")]
    pub allow: Option<Vec<models::AuthorizationResult>>,
}

impl BulkAuthorizationResult {
    pub fn new() -> BulkAuthorizationResult {
        BulkAuthorizationResult {
            allow: None,
        }
    }
}

