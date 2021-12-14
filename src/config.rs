use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(Debug, Clone, Serialize, Deserialize, StructOpt)]
pub struct Config {
    #[structopt(
        long = "client-endpoint",
        env = "CLIENT_ENDPOINT",
        about = "REST API endpoint"
    )]
    pub endpoint: String,
    #[structopt(
        long = "client-token",
        env = "CLIENT_TOKEN",
        about = "API key for service"
    )]
    pub token: String,
}
