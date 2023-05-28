/*
 * 1Password Connect
 *
 * REST API interface for 1Password Connect.
 *
 * The version of the OpenAPI document: 1.3.0
 * Contact: support@1password.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ItemVault {
    #[serde(rename = "id")]
    pub id: String,
}

impl ItemVault {
    pub fn new(id: String) -> ItemVault {
        ItemVault { id }
    }
}