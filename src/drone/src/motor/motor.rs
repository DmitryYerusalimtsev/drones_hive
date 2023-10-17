use std::sync::{Arc, Mutex};
use anyhow::Result;
use rclrs::{Node, RclrsError, Context};
use drone_interfaces::{msg::*, srv::*};
use tokio::runtime::Handle;
use tokio::time::{sleep, Duration};
use tokio::task::JoinHandle;

use crate::state::State;

pub struct Motor {
    node: Arc<Node>,
    context: Arc<Context>,
    state: Arc<Mutex<State>>,
    runtime: Arc<Handle>
}

impl Motor {
    pub fn new(node: Arc<Node>, context: Arc<Context>, initial_state: Arc<Mutex<State>>, runtime: Arc<Handle>) -> Result<Arc<Motor>, RclrsError> {

        let motor = Arc::new( Self { 
            node: node.clone(), 
            context: context, 
            state: initial_state, 
            runtime: runtime 
        });

        motor.start_state_publishing();

        let internal = Arc::clone(&motor);
        let service = node.create_service::<SetThrust, _>("set_thrust", 
              move |header, request| internal.set_thrust(header, request));

        service.map(|_| motor)
    }

    fn set_thrust(&self,
        _request_header: &rclrs::rmw_request_id_t,
        request: SetThrust_Request) -> SetThrust_Response {
        
        let state_mtx = Arc::clone(&self.state);
        let update_handle = self.runtime.spawn(async move {
            let state = *state_mtx.lock().unwrap();
            state.set_thrust(request.thrust).await
        });
        
        let update_result = self.runtime.block_on(update_handle);

        let message = match &update_result {
            Ok(_) => "".to_string(),
            Err(e) => e.to_string()
        };

        SetThrust_Response {
            success: update_result.is_ok(),
            message: message
        }
    }

    fn start_state_publishing(&self) -> JoinHandle<()> {
        let context = Arc::clone(&self.context);
        let node = Arc::clone(&self.node);
        let state_mtx = Arc::clone(&self.state);

        self.runtime.spawn(async move {
            let publisher = node.create_publisher::<MotorState>("state", rclrs::QOS_PROFILE_DEFAULT).unwrap();

            while context.ok() {
                let state = *state_mtx.lock().unwrap();
                let mut message = MotorState::default();
                message.rpm = state.rpm;
                message.thrust = state.thrust;
                publisher.publish(&message).unwrap();

                sleep(Duration::from_millis(100)).await;
            }
        })
    }
}