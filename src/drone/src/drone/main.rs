mod take_off;

use std::env;
use anyhow::Result;
use rclrs::RclrsError;
use take_off::TakeOff;

fn main() -> Result<(), RclrsError> {
    let context = rclrs::Context::new(env::args())?;

    let node = rclrs::create_node(&context, "drone")?;

    TakeOff::new(node.clone())?;

    rclrs::spin(node)
}