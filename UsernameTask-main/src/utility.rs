extern crate regex;
use rand::distributions::{Alphanumeric, DistString};

use crate::{is_anyone_login, password_validation, username_validation, UserDetails};

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

pub fn check_and_add_user(users: &mut HashMap<String, UserDetails>, username: &String) {
    if users.contains_key(username.as_str()) {
        println!("Username is already exist");
    } else {
        let mut is_read_password = true;

        while is_read_password {
            let password = &read_input("password".to_string());
            let password_msg = password_validation::validate_password(password);

            match password_msg.as_str() {
                "Valid" => {
                    let userdetails = UserDetails {
                        Username: username.to_string(),
                        Password: password.to_string(),
                        Name: String::from("Default"),
                        Company: String::from("Unknown"),
                        Id: 1234567890,
                    };

                    users.insert(username.to_string(), userdetails);

                    println!("Account is successfully generated using {}.", username);

                    is_read_password = false;
                }
                _ => println!("Error: {}", password_msg),
            }
        }
    }
}

pub fn signup(users: &mut HashMap<String, UserDetails>) {
    let username = read_input("username".to_string());
    let result = username_validation::validate_username(&username);

    match result.as_str() {
        "Valid" => check_and_add_user(users, &username),
        _ => {
            println!("Error: {}", &result)
        }
    }
}

pub fn login(users: &mut HashMap<String, UserDetails>) {
    unsafe {
        if is_anyone_login {
            println!("User already logged in")
        } else {
            let username = read_input("username".to_string());
            let password = read_input("password".to_string());

            match users.get(&username) {
                Some(value) => {
                    if password == value.Password {
                        is_anyone_login = true;
                        println!("Login successfully");
                    } else {
                        println!("Please check username and password");
                    }
                }
                None => println!("Please check username and password"),
            }
        }
    }
}

pub fn logout() {
    unsafe {
        if is_anyone_login {
            is_anyone_login = false;
            println!("You're successfully logged out");
        } else {
            println!("Please login before logout.")
        }
    }
}

pub fn validate_and_change_details(username: &String, users: &mut HashMap<String, UserDetails>) {
    match users.get(username) {
        Some(_value) => {
            let name = read_input("name".to_string());
            let company = read_input("address".to_string());
            let id = read_input("number".to_string());

            if !name.is_empty() {
                (users.get_mut(username).unwrap()).Name = name;
            }
            if !company.is_empty() {
                (users.get_mut(username).unwrap()).Company = company;
            }
            if !id.to_string().is_empty() && id.to_string().len() == 10 {
                (users.get_mut(username).unwrap()).Id = id.parse::<u128>().unwrap();
            } else {
                println!("Please number must be equals to 10");
            }

            println!("{:?}", users.get(username));

            println!("Details updated");
        }
        None => println!("Account not exist for {} username ", username),
    }
}

pub fn update_details(users: &mut HashMap<String, UserDetails>) {
    unsafe {
        if is_anyone_login {
            let username = read_input("username".to_string());
            validate_and_change_details(&username, users);
        } else {
            println!("Please login before going to change anything.")
        }
    }
}

pub fn delete_user(users: &mut HashMap<String, UserDetails>) {
    let username = read_input("username".to_string());
    let capcha = Alphanumeric.sample_string(&mut rand::thread_rng(), 8);

    match users.get(&username) {
        Some(_value) => {
            println!("Capcha : {}", capcha);
            let user_writen_capcha = read_input("capcha".to_string());

            if user_writen_capcha == capcha {
                users.remove(&username);
                println!("Account deleted successsfully");
                logout();
            } else {
                println!("Please enter correct capcha code");
            }
        }
        None => println!("Please check username and password"),
    }
}
