mod start;
mod state;

use std::env;
use anyhow::Result;
use rclrs::RclrsError;
use start::Start;

fn main() -> Result<(), RclrsError> {
    let context = rclrs::Context::new(env::args())?;

    let node = rclrs::create_node(&context, "engine")?;

    Start::new(node.clone())?;

    rclrs::spin(node)
}