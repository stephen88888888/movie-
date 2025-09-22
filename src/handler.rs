use std::error::Error;

pub fn handle_login(username: &str) -> Result<(), Box<dyn Error>> {
    println!("Username: {username}");

    Ok(())
}
