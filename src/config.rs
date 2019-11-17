use std::env;

pub struct Config {
  pub db_path: String,
  pub raw_session_key_seed: String,
}

fn get_session_key_seed() -> Option<String> {
  match env::var("SESSION_KEY_SEED") {
    Ok(seed) => Some(seed),
    Err(_) => None,
  }
}

fn get_db_path() -> String {
  match env::var("DB_PATH") {
    Ok(path) => path,
    Err(_) => "./data/db".to_string(),
  }
}

impl Config {
  pub fn new() -> Result<Config, &'static str> {
    let db_path = get_db_path();

    let raw_session_key_seed = match get_session_key_seed() {
      Some(value) => value,
      None => { return Err("No session key seed specified"); }
    };

    Ok(Config {db_path, raw_session_key_seed})
  }
}
