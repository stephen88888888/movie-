use crate::services::AuthService;
use std::error::Error;

pub async fn handle_login(username: &str) -> Result<(), Box<dyn Error>> {
    println!("Username: {}", username);

    println!("Please enter the password:");
    let password = rpassword::read_password()?;

    match AuthService::login(username, &password) {
        Ok(Some(_session_id)) => {
            println!("Login successful!");
            if let Some(current_user) = AuthService::get_current_user() {
                println!("Current user: {}", current_user);
            }
        }
        Ok(None) => {
            println!("Invalid username or password.");
        }
        Err(e) => {
            println!("Login error: {}", e);
        }
    }

    Ok(())
}

pub async fn handle_logout() -> Result<(), Box<dyn Error>> {
    match AuthService::logout() {
        Ok(()) => {
            println!("Logout successful!");
        }
        Err(e) => {
            println!("Logout error: {}", e);
        }
    }
    Ok(())
}
