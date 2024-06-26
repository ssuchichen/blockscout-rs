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
pub struct TransactionActionAaveV3LiquidationCall {
    #[serde(rename = "data")]
    pub data: serde_json::Value,
    #[serde(rename = "protocol")]
    pub protocol: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl TransactionActionAaveV3LiquidationCall {
    pub fn new(
        data: serde_json::Value,
        protocol: String,
        r#type: String,
    ) -> TransactionActionAaveV3LiquidationCall {
        TransactionActionAaveV3LiquidationCall {
            data,
            protocol,
            r#type,
        }
    }
}
