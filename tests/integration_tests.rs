#[cfg(test)]
mod tests {
    use std::env; // ❌ Removed `dotenv::dotenv;`

    #[test]
    fn test_env_variables() {
        dotenv::dotenv().ok(); // ✅ This is fine; Rust will load .env if available

        if env::var("SPOTIFY_CLIENT_ID").is_err() {
            eprintln!("Skipping test: SPOTIFY_CLIENT_ID is not set");
            return;
        }

        assert!(env::var("SPOTIFY_CLIENT_SECRET").is_ok());
        assert!(env::var("SPOTIFY_REDIRECT_URI").is_ok());
    }
}
