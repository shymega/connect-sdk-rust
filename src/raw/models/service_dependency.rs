/*
 * 1Password Connect
 *
 * REST API interface for 1Password Connect.
 *
 * The version of the OpenAPI document: 1.3.0
 * Contact: support@1password.com
 * Generated by: https://openapi-generator.tech
 */

/// `ServiceDependency` : The state of a registered server dependency.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ServiceDependency {
    #[serde(rename = "service", skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Human-readable message for explaining the current state.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl ServiceDependency {
    /// The state of a registered server dependency.
    pub fn new() -> ServiceDependency {
        ServiceDependency {
            service: None,
            status: None,
            message: None,
        }
    }
}