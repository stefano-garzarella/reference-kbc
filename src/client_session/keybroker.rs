use kbs_types::{SnpAttestation, Tee};
use serde_json::{json, Value};

use crate::{
    client_session::ClientTee,
    lib::{fmt, Display, String, ToString},
};

pub enum SnpGeneration {
    Milan,
    Genoa,
}

impl Display for SnpGeneration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SnpGeneration::Milan => write!(f, "milan"),
            SnpGeneration::Genoa => write!(f, "genoa"),
        }
    }
}

pub struct KeybrokerClientSnp {
    attestation: SnpAttestation,
}

impl KeybrokerClientSnp {
    pub fn new(gen: SnpGeneration) -> Self {
        KeybrokerClientSnp {
            attestation: SnpAttestation {
                report: "".to_string(),
                gen: gen.to_string(),
            },
        }
    }

    pub fn update_report(&mut self, report: &[u8]) {
        self.attestation.report = hex::encode(report);
    }
}

impl ClientTee for KeybrokerClientSnp {
    fn version(&self) -> String {
        "0.1.0".to_string()
    }

    fn tee(&self) -> Tee {
        Tee::Snp
    }

    fn extra_params(&self) -> Value {
        json!("")
    }

    fn evidence(&self) -> Value {
        json!(self.attestation)
    }
}

#[cfg(test)]
mod tests {
    use core::str::FromStr;

    use super::*;
    use crate::client_session::*;

    #[test]
    fn test_session() {
        let mut snp = KeybrokerClientSnp::new(SnpGeneration::Milan);

        let mut cs = ClientSession::new();

        let request = cs.request(&snp).unwrap();
        assert_eq!(
            request,
            json!({
                "version": "0.1.0",
                "tee": "snp",
                "extra-params": json!("").to_string(),
            }),
        );

        let challenge = r#"
        {
            "nonce": "424242",
            "extra-params": ""
        }"#;
        let nonce = cs
            .challenge(serde_json::from_str(challenge).unwrap())
            .unwrap();
        assert_eq!(nonce, "424242".to_string());

        let report = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        snp.update_report(&report);

        let k_mod = BigUint::parse_bytes(b"98102316FFB6F426A242A619230E0F274AB9433DA04BB91B1A5792DDA8BC5DB86EE67F0F2E89A57716D1CF4469742BB1A9DD72BDA89CAA90CA7BF4D3D3DB1198BD61F12C7741ADC4426A88D1370412A936EC09340D3171B95AEAEDCE611C1E5F6C9E28EE212AE4C61F752978A596B153174DBF88D1125CA675AA7CFE23A8DD253546C68AEB2EE4A31D7FB66D9C7D665984C951158267A685E9C8D62BA7E62808D2B199926732C4BAF7C91A1630E5CB39CB96287032BA18D2642F743EDD09E0685657CF5063C095A9B05B2AAD214FBDE715644A9DE4C5C35C35BFE678F48A4083DA7D0D6C02604A3F0C9C03FD48E672F30D5B906BDE5958C9F4264A61B452211D", 16).unwrap();
        let k_mod_encoded = ClientSession::encode_key(&k_mod).unwrap();
        let k_exp = BigUint::from_str("12345").unwrap();
        let k_exp_encoded = ClientSession::encode_key(&k_exp).unwrap();

        let attestation = cs
            .attestation(k_mod_encoded.clone(), k_exp_encoded.clone(), &snp)
            .unwrap();
        assert_eq!(
            attestation,
            json!({
                "tee-pubkey": json!({
                    "alg": "RSA",
                    "kty": "RSA",
                    "n": k_mod_encoded,
                    "e": k_exp_encoded,
                }),
                "tee-evidence": json!({
                    "gen": "milan",
                    "report": hex::encode(report),
                }).to_string(),
            }),
        );

        let remote_secret = [9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        let data = {
            let resp = KbsResponse {
                protected: "".to_string(),
                encrypted_key: "".to_string(),
                iv: "".to_string(),
                ciphertext: hex::encode(remote_secret),
                tag: "".to_string(),
            };

            json!(resp)
        };
        let secret = cs.secret(data.to_string()).unwrap();
        assert_eq!(secret, remote_secret);
    }
}