use crate::{utility, UserDetails};
use regex::Regex;
use std::{collections::HashMap};

pub fn validate_username(username: &String) -> String {
    if username.is_empty()
     {
        return String::from("Username not empty");

    } else if Regex::new("^_|^[0-9]").unwrap().is_match(username.as_str())
     {
        return String::from("Username can not start with _ or number");

    } else if !Regex::new("[0-9]{1,}").unwrap().is_match(username.as_str()) 
    {
        return String::from("enter username at least one number");

    } else if !Regex::new("[A-Z]{1,}").unwrap().is_match(username.as_str())
     {
        return String::from("enter username least one capital letter");
        
    } else if !Regex::new("[a-z]{1,}").unwrap().is_match(username.as_str()) 
    {
        return String::from(" enter username at least one small letter");
        
    } else if !Regex::new("[~@$#!%^&*()]{1,}")
        .unwrap()
        .is_match(username.as_str())
    {
        return String::from(" least one spacial character:");
    }

    return "Valid".to_string();
}

pub fn check_username_availibility(users: &mut HashMap<String, UserDetails>) {
    let username = utility::read_input("username".to_string());
    if users.contains_key(&username) {
        println!("{} is already taken by another user.", &username);
    } else {
        println!("{} is available.", &username)
    }
}
