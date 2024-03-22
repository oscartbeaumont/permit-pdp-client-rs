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
pub struct KongAuthorizationInput {
    #[serde(rename = "request")]
    pub request: Box<models::KongAuthorizationInputRequest>,
    #[serde(rename = "client_ip", skip_serializing_if = "Option::is_none")]
    pub client_ip: Option<String>,
    #[serde(rename = "service", skip_serializing_if = "Option::is_none")]
    pub service: Option<Box<models::KongAuthorizationInputService>>,
    #[serde(rename = "route", skip_serializing_if = "Option::is_none")]
    pub route: Option<Box<models::KongAuthorizationInputRoute>>,
    #[serde(rename = "consumer", skip_serializing_if = "Option::is_none")]
    pub consumer: Option<Box<models::KongAuthorizationInputConsumer>>,
}

impl KongAuthorizationInput {
    pub fn new(request: models::KongAuthorizationInputRequest) -> KongAuthorizationInput {
        KongAuthorizationInput {
            request: Box::new(request),
            client_ip: None,
            service: None,
            route: None,
            consumer: None,
        }
    }
}

