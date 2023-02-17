use crate::{data, UserDetails};
use regex::Regex;
use std::{collections::HashMap};

pub fn valid_user(u_name: &String) -> String {
    if u_name.is_empty()
     {
        return String::from("Username not empty");

    } 
    else if !Regex::new("[0-9]{1,}").unwrap().is_match(u_name.as_str()) 
    {
        return String::from("enter username at least one number");

    } 
    else if !Regex::new("[a-z]{1,}").unwrap().is_match(u_name.as_str()) 
    {
        return String::from(" enter username at least one small letter");
        
    } else if !Regex::new("[~@$#!%^&*()]{1,}")
        .unwrap()
        .is_match(u_name.as_str())
    {
        return String::from(" least one spacial character:");
    }

    return "Valid".to_string();
}

pub fn check_user(users: &mut HashMap<String, UserDetails>) {
    let u_name = data::read_input("username".to_string());
    if users.contains_key(&u_name) {
        println!("{} is already taken by another user.", &u_name);
    } else {
        println!("{} is available.", &u_name)
    }
}


pub fn valid_pass(p_word: &String) -> String {
    if p_word.is_empty() 
    {
        return String::from("Password not empty");

    } else if p_word.len() < 8 
    {
        return String::from("Password length is lass then 8");

    } else if p_word.len() > 16 
    {
        return String::from("Password length is more then 16");

    } else if Regex::new("^_|^[0-9]").unwrap().is_match(p_word.as_str()) 
    {
        return String::from("Password can not start with _ or number");

    } else if !Regex::new("[0-9]{1,}").unwrap().is_match(p_word.as_str()) 
    {
        return String::from("enter password at least one number");

    } else if !Regex::new("[A-Z]{1,}").unwrap().is_match(p_word.as_str())
     {
        return String::from("enter password at least one capital letter");
        
    } else if !Regex::new("[a-z]{1,}").unwrap().is_match(p_word.as_str())
     {
        return String::from("enter passwordat least one small letter");

    } else if !Regex::new("[~@$#!%^&*()]{1,}")
        .unwrap()
        .is_match(p_word.as_str())
    {
        return String::from("Password at least one spacial character:");
    }

    return "Valid".to_string();
}
