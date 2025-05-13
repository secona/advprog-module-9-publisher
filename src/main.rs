use borsh::{BorshDeserialize, BorshSerialize};
use crosstown_bus::{CrosstownBus, HandleError, MessageHandler};

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct UserCreatedEventMessage {
    pub user_id: String,
    pub user_name: String,
}

pub struct UserCreatedHandler;

impl MessageHandler<UserCreatedEventMessage> for UserCreatedHandler {
    fn handle(&self, message: Box<UserCreatedEventMessage>) -> Result<(), HandleError> {
        println!("Message received on handler 1: {:?}", message);

        Ok(())
    }

    fn get_handler_action(&self) -> String {
        String::from("user_created")
    }
}

fn main() {
    let mut p = CrosstownBus::new_queue_publisher("amqp://guest:guest@localhost:5672".to_string()).unwrap();
    
    let _ = p.publish_event(String::from("user_created"), UserCreatedEventMessage {
        user_id: String::from("1"),
        user_name: String::from("2306152411-Amir"),
    });
    
    let _ = p.publish_event(String::from("user_created"), UserCreatedEventMessage {
        user_id: String::from("2"),
        user_name: String::from("2306152411-Budi"),
    });
    
    let _ = p.publish_event(String::from("user_created"), UserCreatedEventMessage {
        user_id: String::from("3"),
        user_name: String::from("2306152411-Cica"),
    });
    
    let _ = p.publish_event(String::from("user_created"), UserCreatedEventMessage {
        user_id: String::from("4"),
        user_name: String::from("2306152411-Dira"),
    });
    
    let _ = p.publish_event(String::from("user_created"), UserCreatedEventMessage {
        user_id: String::from("5"),
        user_name: String::from("2306152411-Emir"),
    });
}
