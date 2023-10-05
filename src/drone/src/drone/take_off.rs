use anyhow::Result;
use rclrs::{Node, Service, RclrsError};
use core::CommandResult;
use std::{time::Duration, sync::Arc};
use std_srvs::srv::{Trigger, Trigger_Request, Trigger_Response};

pub(crate) struct TakeOff {
    pub node: Arc<Node>
}

impl TakeOff {

    fn take_off(&self) -> Result<CommandResult> {
        let name = self.node.name();
        println!("{name}: Starting engine.");

        std::thread::sleep(Duration::from_millis(2000));

        println!("{name}: Taking off.");

        Ok(CommandResult::Completed)
    }

    fn service(&self,
        _request_header: &rclrs::rmw_request_id_t,
        _request: Trigger_Request) -> Trigger_Response {

        let result = self.take_off();

        match result {
            Ok(CommandResult::Failed { reason }) => 
            Trigger_Response {
                success: true,
                message: format!("{}", reason),
            },

            Ok(_) => 
            Trigger_Response {
                success: true,
                message: String::from("Command completed."),
            },
            
            Err(e) => 
            Trigger_Response {
                success: false,
                message: format!("{}", e),
            }
        }
    }
}

pub fn start_service(node: Arc<Node>) -> Result<Arc<Service<Trigger>>, RclrsError> {
    let take_off = TakeOff { node: node.clone() };
    node.create_service::<Trigger, _>("take_off", 
        move |header, request| take_off.service(header, request))
}
