use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct AppConfig {
    pub db_connection_string: String,
}

impl AppConfig {
    pub fn get() -> Result<Self, ConfigError> {
        let env = std::env::var("RUN_ENV").unwrap_or_else(|_| "dev".into());
        let s = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name("config/default.json"))
            // Add in the current environment file
            // Default to 'development' env
            // Note that this file is _optional_
            .add_source(File::with_name(&format!("config/{}.json", &env)).required(false))
            .build()?;

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_deserialize()

        /*

        s.set::<ENV>(
            "env",
            match env.as_str() {
                "prod" => ENV::Production,
                "Prod" => ENV::Production,
                "Production" => ENV::Production,
                "production" => ENV::Production,
                _ => ENV::Development,
            },
        );

        */
    }
}

// #[derive(Debug, Deserialize, Clone)]
// pub enum ENV {
//     Development,
//     Production,
// }
// impl fmt::Display for ENV {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             ENV::Development => write!(f, "Development"),
//             ENV::Production => write!(f, "Production"),
//         }
//     }
// }

// impl From<&str> for ENV {
//     fn from(env: &str) -> Self {
//         match env {
//             "Production" => ENV::Production,
//             _ => ENV::Development,
//         }
//     }
// }
