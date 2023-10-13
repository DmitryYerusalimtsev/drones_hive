use anyhow::Result;
use rclrs::{Node, RclrsError};
use std::{time::Duration, sync::Arc};
use std_srvs::srv::{Trigger, Trigger_Request, Trigger_Response};

pub enum CommandResult {
    Completed
}

pub struct TakeOff {
    node: Arc<Node>
}

impl TakeOff {

    pub fn new(node: Arc<Node>) -> Result<Arc<TakeOff>, RclrsError> {
        let take_off = Arc::new( Self { node: node.clone() });
        let internal = take_off.clone();

        let service = node.create_service::<Trigger, _>("take_off", 
              move |header, request| internal.endpoint(header, request));

        service.map(|_| take_off)
    }

    fn take_off(&self) -> Result<CommandResult> {
        let name = self.node.name();
        println!("{name}: Starting engine.");

        std::thread::sleep(Duration::from_millis(2000));

        println!("{name}: Taking off.");

        Ok(CommandResult::Completed)
    }

    fn endpoint(&self,
        _request_header: &rclrs::rmw_request_id_t,
        _request: Trigger_Request) -> Trigger_Response {

        let result = self.take_off();

        match result {
            Ok(_) => 
                Trigger_Response {
                    success: true,
                    message: String::from("Command completed."),
                },
            
            Err(e) => 
                Trigger_Response {
                    success: false,
                    message: format!("Failed: {}", e),
                }
        }
    }
}
