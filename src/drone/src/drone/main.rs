mod take_off;

use std::env;
use anyhow::Result;
use rclrs::RclrsError;

fn main() -> Result<(), RclrsError> {
    let context = rclrs::Context::new(env::args())?;

    let node = rclrs::create_node(&context, "drone")?;

    take_off::start_service(node.clone())?;

    rclrs::spin(node)
}