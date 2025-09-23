use std::error::Error;

use crate::services::{get_users, login_success, logout};

pub fn handle_login(username: &str) -> Result<(), Box<dyn Error>> {
    println!("Username: {username}");
    if let Some(user) = get_users()
        .iter()
        .find(|u| u.username.eq_ignore_ascii_case(username))
    {
        println!("Please enter the password:");
        match rpassword::read_password() {
            Ok(password) => {
                if user.password == password {
                    //println!("Password: {password}");
                    login_success(&user.role)?;
                    println!("log in successfully");
                } else {
                    println!("Incorrect password.");
                }
            }
            Err(_) => {
                println!("not read the password");
            }
        }
    } else {
        println!("User not found.");
    }
    Ok(())
}

pub fn handle_logout() {
    logout();
    println!("Logged out successfully.")
}
