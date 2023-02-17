extern crate regex;


use crate::{is_anyone_login, use_valid, UserDetails};

use std::{collections::HashMap, io};

pub fn read_input(type_of_input: String) -> String {
    println!("Enter {} :", type_of_input);
    let mut data = String::new();
    let _input = io::stdin().read_line(&mut data);
    return data.trim().to_string();
}

pub fn read_user_chois() -> String {
    let mut choice = String::new();
    let _input = io::stdin().read_line(&mut choice);
    return choice.trim().to_string();
}

pub fn check_and_add_user(users: &mut HashMap<String, UserDetails>, u_name: &String) {
    if users.contains_key(u_name.as_str()) {
        println!("Username is already exist");
    } else {
        let mut is_read_password = true;

        while is_read_password {
            let p_word = &read_input("password".to_string());
            let password_msg = use_valid::valid_pass(p_word);

            match password_msg.as_str() {
                "Valid" => {
                    let userdetails = UserDetails {
                        Username: u_name.to_string(),
                        Password: p_word.to_string(),
                        Name: String::from("Default"),
                        Company: String::from("Unknown"),
                        Id: 1234567890,
                    };

                    users.insert(u_name.to_string(), userdetails);

                    println!("Account is successfully generated using {}.", u_name);

                    is_read_password = false;
                }
                _ => println!("Error: {}", password_msg),
            }
        }
    }
}

pub fn loginpage(users: &mut HashMap<String, UserDetails>) {
    let u_name = read_input("username".to_string());
    let result = use_valid::valid_user(&u_name);

    match result.as_str() {
        "Valid" => check_and_add_user(users, &u_name),
        _ => {
            println!("Error: {}", &result)
        }
    }
}

pub fn validate_and_change_details(u_name: &String, users: &mut HashMap<String, UserDetails>) {
    match users.get(u_name) {
        Some(_value) => {
            let name = read_input("name".to_string());
            let company = read_input("address".to_string());
            let id = read_input("number".to_string());

            if !name.is_empty() {
                (users.get_mut(u_name).unwrap()).Name = name;
            }
            if !company.is_empty() {
                (users.get_mut(u_name).unwrap()).Company = company;
            }
            if !id.to_string().is_empty() && id.to_string().len() == 10 {
                (users.get_mut(u_name).unwrap()).Id = id.parse::<u128>().unwrap();
            } else {
                println!("Please number must be equals to 10");
            }

            println!("{:?}", users.get(u_name));

            println!("Details updated");
        }
        None => println!("Account not exist for {} username ", u_name),
    }
}

pub fn update_details(users: &mut HashMap<String, UserDetails>) {
    unsafe {
        
            let u_name = read_input("username".to_string());
            validate_and_change_details(&u_name, users);
       
            println!("Please login before going to change anything.")
        }
    }


pub fn delete_user(users: &mut HashMap<String, UserDetails>) {
    let u_name = read_input("username".to_string());

    if users.remove(&u_name).is_some() {
                users.remove(&u_name);
                println!("Account deleted successsfully");
                
       
    
        }
    
}
