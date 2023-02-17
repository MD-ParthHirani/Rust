extern crate regex;
mod data;
use std::collections::HashMap;

mod use_valid;

#[derive(Debug)]
pub struct UserDetails {
    Name: String,
    Company: String,
    Id: u128,
    Username: String,
    Password: String,
}
static mut is_anyone_login: bool = false;

fn main() {
    let mut users: HashMap<String, UserDetails> = HashMap::new();

    let mut run_program = true;
    while run_program {
        println!(" \n\n 1. login account :  \n 2. check username  :   \n 3. update details :  \n 4. delete account :  \n 5. exit : " );
        println!("_______________________________________ ");
        println!("Enter your choose number: ");
        let choice = data::read_user_chois();

        match choice.as_str() {
            "1" => {
                data::loginpage(&mut users);
            }
            "2" => {
                use_valid::check_user(&mut users);
            }
            
            "3" => {
                data::update_details(&mut users);
            }
            "4" => {
                data::delete_user(&mut users);
            }
            "5" => {
                run_program = false;
                println!("Exit you.");
            }
            _ => println!(" choose option"),
        }
    }
}

