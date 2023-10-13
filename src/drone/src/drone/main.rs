mod drone;
mod state;

use std::{env, sync::{Arc, Mutex}, collections::HashSet};
use anyhow::Result;
use rclrs::RclrsError;
use drone::Drone;
use state::State;

#[tokio::main]
async fn main() -> Result<(), RclrsError> {
    let context = rclrs::Context::new(env::args())?;

    let node = rclrs::create_node(&context, "drone")?;

    let mut motors = HashSet::new();
    motors.insert("motor1".to_string());
    motors.insert("motor2".to_string());
    motors.insert("motor3".to_string());
    motors.insert("motor4".to_string());

    let initial_state = State::new(1.5, motors);
    let state = Arc::new(Mutex::new(initial_state));

    Drone::new(node.clone(), state.clone())?;

    rclrs::spin(node)
}