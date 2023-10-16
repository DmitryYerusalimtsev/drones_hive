mod motor;
mod state;

use std::{env, sync::{Arc, Mutex}};
use anyhow::Result;
use rclrs::RclrsError;
use motor::Motor;
use state::State;
use tokio::runtime::{Runtime, Builder};

fn main() -> Result<(), RclrsError> {
    let runtime: Runtime = Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    let context = Arc::new(rclrs::Context::new(env::args()).unwrap());

    let node = rclrs::create_node(context.as_ref(), "motor")?;

    let initial_state = State::new(0.05);
    let state = Arc::new(Mutex::new(initial_state));

    Motor::new(node.clone(), context, state.clone(), runtime)?;

    rclrs::spin(node)
}