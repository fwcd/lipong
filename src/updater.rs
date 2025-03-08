use std::sync::Arc;

use anyhow::Result;
use lighthouse_client::{Lighthouse, TokioWebSocket};
use tokio::{sync::Mutex, time};
use tracing::debug;

use crate::model::{State, TICK_INTERVAL};

pub async fn run<const W: usize, const H: usize>(
    lh: Lighthouse<TokioWebSocket>,
    shared_state: Arc<Mutex<State<W, H>>>,
) -> Result<()> {
    loop {
        let frame = {
            let mut state = shared_state.lock().await;
            state.tick();
            state.render()
        };

        // Send the rendered model to the lighthouse
        lh.put_model(frame).await?;
        debug!("Sent frame");

        time::sleep(TICK_INTERVAL).await;
    }
}
