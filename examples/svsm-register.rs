use clap::Parser;
use log::{debug, error, info};
use reference_kbc::client_registration::ClientRegistration;
use reqwest::blocking::Client;
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
    /// ID of the workload
    #[clap(long)]
    workload: String,
    /// Pre-calculated measurement (hex encoded string - e.g. 8a60c0196d2e9f)
    #[clap(long)]
    measurement: String,
    /// Secret to share with the CVM
    #[clap(long)]
    passphrase: String,
}

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let config = ProxyArgs::parse();

    let cr = ClientRegistration::new(config.workload.clone());
    let registration = cr.register(&hex::decode(config.measurement)?, config.passphrase);

    info!(
        "Registering workload {0} at {1}",
        config.workload, config.url
    );

    let resp = Client::new()
        .post(config.url.clone() + "/kbs/v0/register_workload")
        .json(&registration)
        .send()
        .map_err(Error::HttpCommunication)?;

    debug!("register_workload - resp: {:#?}", resp);

    if resp.status().is_success() {
        info!(
            "Workload {0} successfully registered at {1}",
            config.workload, config.url
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
