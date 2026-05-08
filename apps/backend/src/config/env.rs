use dotenvy::dotenv;

pub fn load_env() {
    match dotenv() {
        Ok(path) => println!(".env loaded from {:?}", path),
        Err(e) => println!("Failed to load .env: {}", e),
    }
}
