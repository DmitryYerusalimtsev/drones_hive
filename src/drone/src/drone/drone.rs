use anyhow::Result;
use rclrs::{Node, RclrsError};
use tokio::runtime::Runtime;
use std::{sync::{Arc, Mutex}, collections::HashMap};
use std_srvs::srv::{Trigger, Trigger_Request, Trigger_Response};
use drone_interfaces::srv::{SetThrust, SetThrust_Request, SetThrust_Response};

use crate::state::State;

pub struct Drone {
    node: Arc<Node>,
    state: Arc<Mutex<State>>,
    runtime: Runtime
}

impl Drone {

    pub fn new(node: Arc<Node>, initial_state: Arc<Mutex<State>>, runtime: Runtime) -> Result<Arc<Drone>, RclrsError> {
        let drone = Arc::new( Self { node: node.clone(), state: initial_state, runtime });
        
        let to_drone = Arc::clone(&drone);
        let take_off = node.create_service::<Trigger, _>("take_off", 
              move |header, request| to_drone.take_off(header, request));

        let mbp_drone = Arc::clone(&drone);
        let move_by_path = node.create_service::<Trigger, _>("move_by_path", 
              move |header, request| mbp_drone.move_by_path(header, request));

        take_off
            .and(move_by_path)
            .map(|_| drone)
    }

    fn take_off(&self,
        _request_header: &rclrs::rmw_request_id_t,
        _request: Trigger_Request) -> Trigger_Response {
            
            let name = self.node.name();
            println!("{name}: Taking off.");

            let state = self.state.lock().unwrap();
            let thrust_per_motor = state.weight / state.motors.len() as f64;
            let target_thrust = state.motors.iter().map(|m| (m.clone(), thrust_per_motor)).collect();

            let results = self.set_thrust(target_thrust);
            
            let error_message = results.iter()
                .find(|(_, r)| !r.as_ref().unwrap().success)
                .map( |(_, r)| r.as_ref().unwrap().message.clone())
                .or(None);

            println!("{name}: Took off successfully.");

            Trigger_Response {
                success: error_message.is_none(),
                message: error_message.or(Some("".to_string())).unwrap()
            }
    }

    fn move_by_path(&self,
        _request_header: &rclrs::rmw_request_id_t,
        _request: Trigger_Request) -> Trigger_Response {
            
            let name = self.node.name();
            println!("{name}: Moving by specified path.");

            Trigger_Response {
                success: true,
                message: "".to_string()
            }
        }

    fn set_thrust(&self, target_thrust: HashMap<String, f64>) -> HashMap<String, Result<SetThrust_Response, RclrsError>> {
        target_thrust
            .into_iter()
            .map(|(motor, thrust)| {
                let node = Arc::clone(&self.node);
                let topic = motor.clone() + "/set_thrust";
                let handle = self.runtime.spawn(async move {
                    let client = node.create_client::<SetThrust>(&topic).unwrap();
                    client.call_async(SetThrust_Request { thrust }).await
                });
                (motor, handle)
            })
            .map(|(motor, handle)| {
                let result = self.runtime.block_on(handle).unwrap();
                (motor, result)
            })
            .collect()
    }
}
