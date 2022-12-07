#[derive(Clone)]
pub struct Settings {
    pub replicate_token: String,
}

pub fn get_config() -> Settings {
    dotenvy::dotenv().ok();
    let replicate_token = dotenvy::var("REPLICATE_TOKEN").expect("REPLICATE_TOKEN must be set");
    Settings { replicate_token }
}
