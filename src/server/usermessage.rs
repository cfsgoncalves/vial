pub struct UserMessage {
    message: String,
    from: String,
    to: Vec<String>
}

impl UserMessage {
    pub fn new(message: String, from: String, to: Vec<String>) -> Self { 
        println!("Creating a new UserMessage");
        Self { message, from, to } 
    }

    pub fn list_messages(init_date: String, end_date: String) -> Vec<UserMessage>{
        println!("Listing messages");
        todo!();
    }

    pub fn send_message(&self){
        println!("Sending messages");
        todo!();
    }

}
