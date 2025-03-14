use anyhow::Result;
use clap::Parser;
use lighthouse_client::{protocol::{Authentication, LIGHTHOUSE_COLS, LIGHTHOUSE_ROWS}, Lighthouse, LIGHTHOUSE_URL};
use model::State;
use tracing::info;
use tokio::{sync::Mutex, task};
use std::sync::Arc;

mod controller;
mod model;
mod updater;

#[derive(Parser)]
#[command(version)]
struct Args {
    /// The username.
    #[arg(short, long, env = "LIGHTHOUSE_USER")]
    username: String,
    /// The API token.
    #[arg(short, long, env = "LIGHTHOUSE_TOKEN")]
    token: String,
    /// The server URL.
    #[arg(long, env = "LIGHTHOUSE_URL", default_value = LIGHTHOUSE_URL)]
    url: String,
    /// The ball's speed.
    #[arg(short, long, default_value_t = 0.75)]
    ball_speed: f64,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();
    _ = dotenvy::dotenv();

    let args = Args::parse();
    let auth = Authentication::new(&args.username, &args.token);
    let state = Arc::new(Mutex::new(State::<LIGHTHOUSE_COLS, LIGHTHOUSE_ROWS>::new(args.ball_speed)));

    let lh = Lighthouse::connect_with_tokio_to(&args.url, auth).await.unwrap();
    info!("Connected to the Lighthouse server");

    let input = lh.stream_input().await.unwrap();

    let updater_handle = task::spawn(updater::run(lh, state.clone()));
    let controller_handle = task::spawn(controller::run(input, state));

    updater_handle.await??;
    controller_handle.await??;

    Ok(())
}
