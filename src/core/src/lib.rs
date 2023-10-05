// use chrono::{Utc, DateTime};

pub enum CommandResult {
    Accepted,
    Completed,
    Declined,
    Failed { reason: String }
}

// pub struct CommandResult {
//     pub result: String,
//     pub command: String,
//     pub timestamp: DateTime<Utc>,
//     pub reason: Option<String>
// }

// impl CommandResult {
//     pub fn new(command: String, result: String, reason: Option<String>) -> CommandResult {
//         Self { 
//             command: command, 
//             result: result,
//             timestamp: Utc::now(),
//             reason: reason
//         }
//     }

//     pub fn success(command: String, result: String) -> CommandResult {
//         Self { 
//             command: command, 
//             result: result,
//             timestamp: Utc::now(),
//             reason: None
//         }
//     }
// }