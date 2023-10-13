mod motor;
mod state;

use std::{env, sync::{Arc, Mutex}};
use anyhow::Result;
use rclrs::RclrsError;
use motor::Motor;
use state::State;

fn main() -> Result<(), RclrsError> {
    let context = rclrs::Context::new(env::args())?;

    let node = rclrs::create_node(&context, "motor")?;

    let initial_state = State::new(0.05);
    let state = Arc::new(Mutex::new(initial_state));

    Motor::new(node.clone(), state.clone())?;

    rclrs::spin(node)
}