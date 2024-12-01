use clap::Parser;

#[derive(Debug, Clone, Default, Parser)]
pub struct Env {
    #[clap(env)]
    pub nats_url: String,

    #[clap(env)]
    pub port: String,
}
