use rusty_leveldb::{DB, Options};
use serde::{Serialize, Deserialize};
use std::marker::{Send, Sync};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::config::Config;
use crate::crypto::Crypto;

#[derive(Serialize, Deserialize)]
pub struct PMState {
  pub data: String
}

#[derive(Serialize, Deserialize)]
pub struct Token {
  pub username: String,
  pub exp_time: u64,
  /* maybe: capabilities (ro/rw) */
}

pub struct UserDB {
  pub db: DB,
}

impl UserDB {
  pub fn new(config: &Config) -> UserDB {
    let mut options = Options::default();
    options.create_if_missing = true;

    let db = match DB::open(&config.db_path, options) {
        Ok(db) => { db },
        Err(e) => { panic!("failed to open database: {:?}", e) }
    };

    UserDB { db }
  }
}

unsafe impl Send for UserDB {}
unsafe impl Sync for UserDB {}

impl UserDB {
  pub fn register_user(&mut self, username: &str, password: &[u8]) -> Result<(), &'static str> {
    let user_key = format!("users:{}:password", username);

    if self.db.get(user_key.as_bytes()).is_some() {
      return Err("User already exists");
    }

    let pw_hash = Crypto::hash256(password);
    if self.db.put(user_key.as_bytes(), &pw_hash).is_err() {
      return Err("Internal database error");
    }

    match self.db.flush() {
      Ok(_) => Ok(()),
      Err(_) => Err("Internal database error"),
    }
  }

  pub fn get_user_pw_hash(&mut self, username: &str) -> Option<Vec<u8>> {
    let user_key = format!("users:{}:password", username);

    let raw_value = self.db.get(user_key.as_bytes())?;
    Some(raw_value)
  }

  pub fn generate_token(&mut self, crypto: &Crypto, username: &str) -> Result<String, &'static str> {
    let unix_time_now = match SystemTime::now().duration_since(UNIX_EPOCH) {
      Ok(t) => t.as_secs(),
      Err(_) => return Err("Internal error"),
    };

    let token = Token { username: username.to_string(), exp_time: /* now + a week */ unix_time_now + 604800 };
    let serialized_token = match serde_json::to_vec(&token) {
      Ok(serialized_token) => serialized_token,
      Err(_) => return Err("Internal database error"),
    };

    let token_signature = crypto.sign(&serialized_token);
    let encoded_token_signature = base64::encode(&token_signature);
    let token_key = format!("tokens:{}", encoded_token_signature);

    if self.db.put(token_key.as_bytes(), &serialized_token).is_err() {
      return Err("Internal database error");
    }

    match self.db.flush() {
      Ok(_) => Ok(encoded_token_signature),
      Err(_) => Err("Internal database error"),
    }
  }

  pub fn validate_token(&mut self, signature: &str) -> Option<Token> {
    let token_key = format!("tokens:{}", signature);
    let raw_value = self.db.get(token_key.as_bytes())?;

    let token: Token = match serde_json::from_slice(&raw_value) {
      Ok(token) => token,
      Err(_) => return None,
    };

    let time_now = match SystemTime::now().duration_since(UNIX_EPOCH) {
      Ok(t) => t.as_secs(),
      Err(_) => return None,
    };

    if time_now > token.exp_time {
      /* TODO: soft delete token */
      return None
    }

    Some(token)
  }

  pub fn get_state(&mut self, username: &str) -> Option<String> {
    let state_key = format!("users:{}:state", username);
    let raw_state = self.db.get(state_key.as_bytes())?;
    Some(base64::encode(&raw_state))
  }

  pub fn store_state(&mut self, username: &str, state: &str) -> Result<(), &'static str> {
    let state_key = format!("users:{}:state", username);
    let encoded_state = match base64::decode(state) {
      Ok(encoded_state) => encoded_state,
      Err(_) => return Err("Invalid encoding"),
    };

    if self.db.put(state_key.as_bytes(), &encoded_state).is_err() {
      return Err("Internal database error");
    }

    match self.db.flush() {
      Ok(_) => Ok(()),
      Err(_) => Err("Internal database error"),
    }
  }
}
