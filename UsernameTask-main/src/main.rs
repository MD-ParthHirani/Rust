extern crate regex;
mod utility;
use std::collections::HashMap;
mod password_validation;
mod username_validation;

#[derive(Debug)]
pub struct UserDetails {
    Username: String,
    Password: String,
    Name: String,
    Company: String,
    Id: u128,
}
static mut is_anyone_login: bool = false;

fn main() {
    let mut users: HashMap<String, UserDetails> = HashMap::new();

    let mut is_run_program = true;
    while is_run_program {
        println!(" \n\n 1. new account :  \n 2. check username  :  \n 3. login : \n 4. logout : \n 5. 7update details :  \n 6. delete account :  \n 7. exit : " );
        println!("_______________________________________ ");
        println!("Enter your choose number: ");
        let choice = utility::read_user_chois();

        match choice.as_str() {
            "1" => {
                utility::signup(&mut users);
            }
            "2" => {
                username_validation::check_username_availibility(&mut users);
            }
            "3" => {
                utility::login(&mut users);
            }
            "4" => {
                utility::logout();
            }
            "5" => {
                utility::update_details(&mut users);
            }
            "6" => {
                utility::delete_user(&mut users);
            }
            "7" => {
                is_run_program = false;
                println!("Exit you.");
            }
            _ => println!(" choose option"),
        }
    }
}
