use std::{fs::read_to_string, path::PathBuf};

use clap::Parser;
use log::{debug, error, info};
use reference_kbc::client_registration::ClientRegistration;
use reqwest::blocking::Client;
use serde_json::from_str;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("Communication with the HTTP server failed - {0}")]
    HttpCommunication(reqwest::Error),
    #[error("KBS is failing to register the SVSM workload")]
    RegistrationFailed,
}

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct ProxyArgs {
    /// HTTP url to KBS (e.g. http://server:4242)
    #[clap(long)]
    url: String,

    /// Secret to share with the CVM
    #[clap(long)]
    resources: PathBuf,

    /// Attestation appraisal policy
    #[clap(long)]
    policy: PathBuf,

    /// Appraisal policy queries
    #[clap(long)]
    queries: PathBuf,
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let test_measurement = [0u8; 1]; // A testing, non-usable measurement.

    let config = ProxyArgs::parse();

    let resources = read_to_string(config.resources).unwrap();
    let policy = read_to_string(config.policy).unwrap();
    let queries: Vec<String> = from_str(&read_to_string(config.queries).unwrap()).unwrap();

    let mut cr = ClientRegistration::new(policy, queries, resources);
    let registration = cr.register(&test_measurement);

    info!("Registering workload at {}", config.url);

    let resp = Client::new()
        .post(config.url.clone() + "/rvp/registration")
        .json(&registration)
        .send()
        .map_err(Error::HttpCommunication)?;

    debug!("register_workload - resp: {:#?}", resp);

    if resp.status().is_success() {
        info!(
            "Workload successfully registered at {} (replied {})",
            config.url,
            resp.text().unwrap()
        );
        Ok(())
    } else {
        error!(
            "KBS returned error {0} - {1}",
            resp.status(),
            resp.text().unwrap()
        );
        Err(Error::RegistrationFailed.into())
    }
}
