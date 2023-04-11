use std::vec;
mod server;

fn main() {
    let me = server::usermodel::Usermodel::new(String::from("foo@bar.com"),String::from(""));

    let user_message = server::usermessage::UserMessage::new("String".to_string(),"String".to_string(), vec![String::from("value")]);
}
