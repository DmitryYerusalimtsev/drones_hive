mod motor;
mod state;

use std::{env, sync::{Arc, Mutex}};
use anyhow::Result;
use rclrs::RclrsError;
use motor::Motor;
use state::State;
use tokio::runtime::Handle;

#[tokio::main]
async fn main() -> Result<(), RclrsError> {

    let context = Arc::new(rclrs::Context::new(env::args()).unwrap());

    let node = rclrs::create_node(context.as_ref(), "motor")?;

    let initial_state = State::new(0.05);
    let state = Arc::new(Mutex::new(initial_state));

    let runtime = Arc::new(Handle::current());

    Motor::new(node.clone(), context, state.clone(), runtime.clone())?;

    println!("Motor {} started.", node.name());

    let rclrs_spin = runtime.spawn_blocking(move || rclrs::spin(node));
    rclrs_spin.await.ok();
    
    Ok(())
}