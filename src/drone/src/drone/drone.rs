use anyhow::Result;
use rclrs::{Node, RclrsError};
use tokio::runtime::Runtime;
use std::sync::{Arc, Mutex};
use std_srvs::srv::{Trigger, Trigger_Request, Trigger_Response};
use drone_interfaces::srv::{SetThrust, SetThrust_Request, SetThrust_Response};

use crate::state::State;

pub struct Drone {
    node: Arc<Node>,
    state: Arc<Mutex<State>>
}

impl Drone {

    pub async fn new(node: Arc<Node>, initial_state: Arc<Mutex<State>>) -> Result<Arc<Drone>, RclrsError> {
        let drone = Arc::new( Self { node: node.clone(), state: initial_state });
        let internal = drone.clone();

        let service = node.create_service::<Trigger, _>("take_off", 
              move |header, request| internal.take_off(header, request));

        service.map(|_| drone)
    }

    fn take_off(&self,
        _request_header: &rclrs::rmw_request_id_t,
        _request: Trigger_Request) -> Trigger_Response {
            
            let name = self.node.name();
            println!("{name}: Taking off.");

            let state: std::sync::MutexGuard<'_, State> = self.state.lock().unwrap();
            let thrust_per_motor = state.weight / state.motors.len() as f64;

            let mut sub_tasks = vec![];
            for motor in state.motors.clone() {
                let node = self.node.clone();

                let handle = tokio::spawn(async move {
                    let client = node.create_client::<SetThrust>(&motor).unwrap();
                    client.call_async(SetThrust_Request { thrust: thrust_per_motor }).await
                });

                sub_tasks.push(handle);
            }

            let response = async {
                let mut results = vec![];
                for handle in sub_tasks {
                    results.push(handle.await.unwrap());
                }

                let error_message = results.iter()
                    .find(|r| !r.as_ref().unwrap().success)
                    .map( |r: &std::result::Result<SetThrust_Response, RclrsError>| r.as_ref().unwrap().message.clone())
                    .or(None);

                Trigger_Response {
                    success: error_message.is_none(),
                    message: String::from("")
                }
            };

            Runtime::new().unwrap().block_on(response)
    }
}
