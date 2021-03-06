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
pub struct ApiRequestResource {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
    #[serde(rename = "vault", skip_serializing_if = "Option::is_none")]
    pub vault: Option<Box<crate::raw::models::ApiRequestResourceVault>>,
    #[serde(rename = "item", skip_serializing_if = "Option::is_none")]
    pub item: Option<Box<crate::raw::models::ApiRequestResourceVault>>,
    #[serde(rename = "itemVersion", skip_serializing_if = "Option::is_none")]
    pub item_version: Option<i32>,
}

impl ApiRequestResource {
    pub fn new() -> ApiRequestResource {
        ApiRequestResource {
            _type: None,
            vault: None,
            item: None,
            item_version: None,
        }
    }
}

///
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize,
)]
#[allow(clippy::upper_case_acronyms)]
pub enum Type {
    #[serde(rename = "ITEM")]
    ITEM,
    #[serde(rename = "VAULT")]
    VAULT,
}

impl Default for Type {
    fn default() -> Type {
        Self::ITEM
    }
}
