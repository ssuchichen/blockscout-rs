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
pub struct TokenCounters {
    #[serde(rename = "token_holders_count")]
    pub token_holders_count: String,
    #[serde(rename = "transfers_count")]
    pub transfers_count: String,
}

impl TokenCounters {
    pub fn new(token_holders_count: String, transfers_count: String) -> TokenCounters {
        TokenCounters {
            token_holders_count,
            transfers_count,
        }
    }
}
