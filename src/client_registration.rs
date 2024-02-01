use openssl::base64::encode_block;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::lib::{Debug, String, ToString};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClientRegistration {
    policy: String,
    queries: Vec<String>,
    reference: String,
    resources: String,
}

impl ClientRegistration {
    pub fn new(policy: String, queries: Vec<String>, resources: String) -> Self {
        Self {
            policy,
            queries,
            reference: "".to_string(),
            resources,
        }
    }

    pub fn register(&mut self, measurement: &[u8]) -> Value {
        let encoded = encode_block(measurement);
        self.reference = format!("{{\"measurement\":\"{}\"}}", encoded);

        json!(self)
    }
}
