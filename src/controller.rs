use std::sync::Arc;

use anyhow::Result;
use futures::{prelude::*, Stream};
use lighthouse_client::protocol::{InputEvent, KeyEvent, ServerMessage};
use tokio::sync::Mutex;
use tracing::info;

use crate::model::State;

pub async fn run<const W: usize, const H: usize>(
    mut stream: impl Stream<Item = lighthouse_client::Result<ServerMessage<InputEvent>>> + Unpin,
    shared_state: Arc<Mutex<State<W, H>>>,
) -> Result<()> {
    while let Some(msg) = stream.next().await {
        let input_event = msg?.payload;

        match input_event {
            InputEvent::Key(KeyEvent { code, down, .. }) => {
                info!("Got key input: {code:?} (down: {down})");

                #[allow(unused)]
                let mut state = shared_state.lock().await;
                // FIXME: Update game/app state here
            },
            _ => {},
        }
    }

    Ok(())
}
