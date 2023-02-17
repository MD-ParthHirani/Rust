/*use regex::Regex;

fn main(){
    let text = "user.logins,host=webserver01,country=US:1|c";

    let re = Regex::new(r"([^,]+)((,[a-zA-Z0-9=]+)*)(:[0-9]+\|[c])").expect("failed to compile regex ");
    for cap in re.captures_iter(text){
        println!("Metric: {}",&cap[1]);
        println!("Tags: {}",&cap[2]);
        println!("Tags: {}",&cap[3]);
    } 

}*//* 
extern crate lazy_static;
extern crate regex;
extern crate serde;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

lazy_static! {
    static ref RE_USER_NAME: Regex = Regex::new(r"^[a-zA-Z0-9]{6,}$").unwrap();
    static ref RE_SPECIAL_CHAR: Regex = Regex::new("^.*?[@$!%*?&].*$").unwrap();
}

#[derive(Serialize, Deserialize, Validate)]
pub struct Student {
    #[validate(length(min = 3, message = "Name must be greater than 3 chars"))]
    pub name: String,
    #[validate(
        email,
        contains(pattern = "gmail", message = "Email must be valid gmail address")
    )]
    pub email: String,
    #[validate(range(min = 18, max = 22, message = "Age must be between 18 to 22"))]
    pub age: u32,
    #[validate(
        regex(
            path = "RE_USER_NAME",
            message = "Username must number and alphabets only and must be 6 characters long"
        )
    )]
    pub username: String,
    #[validate(
        custom(
            function = "validate_password",
            message = "Must Contain At Least One Upper Case, Lower Case and Number. Dont use spaces."
        ),
        regex(
            path = "RE_SPECIAL_CHAR",
            message = "Must Contain At Least One Special Character"
        )
    )]
    pub password: String,
}
fn validate_password(password: &str) -> Result<(), ValidationError> {
    let mut has_whitespace = false;
    let mut has_upper = false;
    let mut has_lower = false;
    let mut has_digit = false;

    for c in password.chars() {
        has_whitespace |= c.is_whitespace();
        has_lower |= c.is_lowercase();
        has_upper |= c.is_uppercase();
        has_digit |= c.is_digit(10);
    }
    if !has_whitespace && has_upper && has_lower && has_digit && password.len() >= 8 {
        Ok(())
    } else {
        return Err(ValidationError::new("Password Validation Failed"));
    }
}
pub async fn create_student(json: web::Json<Student>) -> impl Responder {
    let is_valid = json.validate();
    match is_valid {
        Ok(_) => HttpResponse::Ok().json("success"),
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting the server at http://127.0.0.1:8080/");
    HttpServer::new(|| App::new().route("/", web::post().to(create_student)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}*/
use regex::Regex;

fn extract_login(input: &str) -> Option<&str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?x)
            ^(?P<login>[^@\s]+)@
            ([[:word:]]+\.)*
            [[:word:]]+$
            ").unwrap();
    }
    RE.captures(input).and_then(|cap| {
        cap.name("login").map(|login| login.as_str())
    })
}

fn main() {
    assert_eq!(extract_login(r"I❤email@example.com"), Some(r"I❤email"));
    assert_eq!(
        extract_login(r"sdf+sdsfsd.as.sdsd@jhkk.d.rl"),
        Some(r"sdf+sdsfsd.as.sdsd")
    );
    assert_eq!(extract_login(r"More@Than@One@at.com"), None);
    assert_eq!(extract_login(r"Not an email@email"), None);
}
