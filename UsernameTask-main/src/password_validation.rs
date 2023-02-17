use regex::Regex;


pub fn validate_password(password: &String) -> String {
    if password.is_empty() 
    {
        return String::from("Password not empty");

    } else if password.len() < 8 
    {
        return String::from("Password length is lass then 8");

    } else if password.len() > 16 
    {
        return String::from("Password length is more then 16");

    } else if Regex::new("^_|^[0-9]").unwrap().is_match(password.as_str()) 
    {
        return String::from("Password can not start with _ or number");

    } else if !Regex::new("[0-9]{1,}").unwrap().is_match(password.as_str()) 
    {
        return String::from("enter password at least one number");

    } else if !Regex::new("[A-Z]{1,}").unwrap().is_match(password.as_str())
     {
        return String::from("enter password at least one capital letter");
        
    } else if !Regex::new("[a-z]{1,}").unwrap().is_match(password.as_str())
     {
        return String::from("enter passwordat least one small letter");

    } else if !Regex::new("[~@$#!%^&*()]{1,}")
        .unwrap()
        .is_match(password.as_str())
    {
        return String::from("Password at least one spacial character:");
    }

    return "Valid".to_string();
}
