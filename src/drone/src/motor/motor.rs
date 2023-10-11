use core::CommandResult;
use std::{sync::{Arc, Mutex}, time::Duration};
use anyhow::Result;
use rclrs::{Node, RclrsError};
use drone_interfaces::srv::{SetThrust, SetThrust_Request, SetThrust_Response}

use crate::state::State;

pub struct Motor {
    node: Arc<Node>,
    state: Arc<Mutex<State>>
}

impl Motor {
    pub fn new(node: Arc<Node>, initial_state: State) -> Result<Arc<Motor>, RclrsError> {
        let state = Arc::new( initial_state);
        let motor = Arc::new( Self { node, state });

        let service = node.create_service::<Trigger, _>("start", 
              move |header, request| internal.endpoint(header, request));

        service.map(|_| motor)
    }
}