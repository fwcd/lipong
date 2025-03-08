use std::{collections::HashMap, sync::Arc};

use anyhow::Result;
use futures::{prelude::*, Stream};
use lighthouse_client::protocol::{EventSource, GamepadButtonEvent, GamepadControlEvent, GamepadEvent, InputEvent, ServerMessage};
use tokio::sync::Mutex;
use tracing::info;

use crate::model::{State, PLAYER_COUNT};

pub async fn run<const W: usize, const H: usize>(
    mut stream: impl Stream<Item = lighthouse_client::Result<ServerMessage<InputEvent>>> + Unpin,
    shared_state: Arc<Mutex<State<W, H>>>,
) -> Result<()> {
    let mut gamepads: HashMap<EventSource, usize> = HashMap::new();

    while let Some(msg) = stream.next().await {
        let input_event = msg?.payload;

        match input_event {
            InputEvent::Key(key) if key.down => {
                let mut state = shared_state.lock().await;
                
                if let Some(dir) = key.wasd_direction() {
                    state.move_paddle(0, dir);
                } else if let Some(dir) = key.arrow_direction() {
                    state.move_paddle(1, dir);
                }
            },
            InputEvent::Gamepad(gamepad) if !matches!(gamepad, GamepadEvent { control: GamepadControlEvent::Button(GamepadButtonEvent { down: false, .. }), .. }) => {
                // TODO: Improve gamepad mechanics (e.g. keep moving paddle when key is held)

                let opt_dir = gamepad.direction();
                let source = gamepad.source;

                let i = gamepads.get(&source).cloned().unwrap_or_else(|| {
                    let i = gamepads.len() % PLAYER_COUNT;
                    info!("Registering new gamepad {:?} as player {}", &source, i + 1);
                    gamepads.insert(source, i);
                    i
                });

                if let Some(dir) = opt_dir {
                    let mut state = shared_state.lock().await;
                    state.move_paddle(i, dir);
                }
            },
            _ => {},
        }
    }

    Ok(())
}
