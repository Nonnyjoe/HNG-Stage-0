#[derive(Debug, Clone)]
pub struct Config {
    pub port: String,
    pub cat_fact_source_url: String,
    pub email: String,
    pub name: String,
}

impl Config {
    pub fn init() -> Config {
        let cat_fact_source_url = std::env::var("CAT_FACT_URL").expect("cat_fact_source_url must be set");
        let port = std::env::var("PORT").expect("PORT must be set");
        let email = std::env::var("EMAIL").expect("PORT must be set");
        let name = std::env::var("NAME").expect("PORT must be set");


        Config {
            cat_fact_source_url,
            email,
            name,
            port,
        }
    }
}

unsafe impl Send for Config {}
unsafe impl Sync for Config {}
