use std::sync::Arc;

use anyhow::Result;
use lighthouse_client::{Lighthouse, TokioWebSocket};
use tokio::sync::Mutex;
use tracing::debug;

use crate::model::State;

pub async fn run(lh: Lighthouse<TokioWebSocket>, shared_state: Arc<Mutex<State>>) -> Result<()> {
    loop {
        let frame = {
            let mut state = shared_state.lock().await;
            state.tick();
            state.render()
        };

        // Send the rendered model to the lighthouse
        lh.put_model(frame).await?;
        debug!("Sent frame");
    }
}
