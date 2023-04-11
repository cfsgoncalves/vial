#[derive(Debug)]
pub struct Usermodel {
    pub email: String,
    pub password: String,
}

impl Usermodel {

    pub fn new(email: String, password: String) -> Self { 
        println!("Creating a new Usermodel");
        Self { email, password } 
    }

    pub fn create_user(&self){
        println!("Create the user with email: {:?}", self.email);
    }

    pub fn delete_user(&self, email: String){
        println!("Deleting the user with email: {email}");
    }

    pub fn get_user(&self, email: String) -> Usermodel{
        println!("Getting the user with email: {email}");
        return Usermodel { email: email, password: "foo".to_string() };
    }

    pub fn change_user_details(&self, old_user: Usermodel, new_user: Usermodel){
        println!("Changing Details of the user");
    }

    pub fn list_user_friends(&self) -> Vec<Usermodel>{
        println!("Getting all the user friends");
        let mut vec = Vec::new();
        vec.push(Usermodel{email: "foo".to_string(), password: "bar".to_string()});
        vec
    }
}