pub fn load_env() {
    dotenvy::dotenv().ok();
    println!("Environment loaded");
}
