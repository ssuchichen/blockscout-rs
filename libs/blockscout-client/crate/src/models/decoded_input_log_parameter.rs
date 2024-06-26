/*
 * BlockScout API
 *
 * API for BlockScout web app
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: you@your-company.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DecodedInputLogParameter {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "indexed")]
    pub indexed: bool,
}

impl DecodedInputLogParameter {
    pub fn new(
        name: String,
        r#type: String,
        value: String,
        indexed: bool,
    ) -> DecodedInputLogParameter {
        DecodedInputLogParameter {
            name,
            r#type,
            value,
            indexed,
        }
    }
}
