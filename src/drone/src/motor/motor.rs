use std::sync::{Arc, Mutex};
use anyhow::Result;
use rclrs::{Node, RclrsError};
use drone_interfaces::srv::{SetThrust, SetThrust_Request, SetThrust_Response};

use crate::state::State;

pub struct Motor {
    state: Arc<Mutex<State>>
}

impl Motor {
    pub fn new(node: Arc<Node>, initial_state: Arc<Mutex<State>>) -> Result<Arc<Motor>, RclrsError> {
        let motor = Arc::new( Self { state: initial_state });
        let internal = motor.clone();

        let service = node.create_service::<SetThrust, _>("set_thrust", 
              move |header, request| internal.set_thrust(header, request));

        service.map(|_| motor)
    }

    fn set_thrust(&self,
        _request_header: &rclrs::rmw_request_id_t,
        _request: SetThrust_Request) -> SetThrust_Response {

        let state = *self.state.lock().unwrap();
        state.set_thrust(_request.thrust);

        SetThrust_Response {
            success: true,
            message: String::from("")
         }
    }
}