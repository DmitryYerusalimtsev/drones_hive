mod drone;
mod state;

use std::{env, sync::{Arc, Mutex}, collections::HashSet};
use anyhow::Result;
use rclrs::RclrsError;
use drone::Drone;
use state::State;
use tokio::runtime::Handle;

#[tokio::main]
async fn main() -> Result<(), RclrsError> {
    let context = rclrs::Context::new(env::args())?;

    let node = rclrs::create_node(&context, "drone")?;

    let mut motors = HashSet::new();
    motors.insert(node.name() + "/motor_0");
    motors.insert(node.name() + "/motor_1");
    motors.insert(node.name() + "/motor_2");
    motors.insert(node.name() + "/motor_3");

    let initial_state = State::new(1.5, motors);
    let state = Arc::new(Mutex::new(initial_state));

    let runtime = Arc::new(Handle::current());

    Drone::new(node.clone(), state.clone(), runtime.clone())?;

    println!("Drone {} started.", node.name());

    let rclrs_spin = runtime.spawn_blocking(move || rclrs::spin(node));
    rclrs_spin.await.ok();
    
    Ok(())
}