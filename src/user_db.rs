use rusty_leveldb::{DB, Options};
use serde::{Serialize, Deserialize};
use std::marker::{Send, Sync};

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
  db: DB,
}

impl UserDB {
  pub fn new(config: &Config) -> UserDB {
    let mut options = Options::default();
    options.create_if_missing = true;

    let db = match DB::open(&config.db_path, options) {
        Ok(db) => { db },
        Err(e) => { panic!("failed to open database: {:?}", e) }
    };

    return UserDB { db }
  }
}

unsafe impl Send for UserDB {}
unsafe impl Sync for UserDB {}

impl UserDB {
  pub fn get_user_pw_hash(&mut self, username: &String) -> Option<String> {
    let user_key = format!("users:{}:password", username);

    let raw_value = self.db.get(user_key.as_bytes())?;
    return match std::str::from_utf8(&raw_value) {
      Ok(value) => Some(value.to_string()),
      Err(_) => None
    }
  }

  pub fn generate_token(&mut self, crypto: &Crypto, username: &String) -> Result<String, &'static str> {
    let token = Token { username: username.to_string(), exp_time: /* now + a week or so */ 0 };
    let serialized_token = match serde_json::to_vec(&token) {
      Ok(serialized_token) => serialized_token,
      Err(_) => { return Err("Internal database error"); }
    };

    let token_signature = crypto.sign(&serialized_token);
    let encoded_token_signature = base64::encode(&token_signature);
    let token_key = format!("tokens:{}", encoded_token_signature);

    if let Err(_) = self.db.put(token_key.as_bytes(), &serialized_token) {
      return Err("Internal database error");
    }

    return match self.db.flush() {
      Ok(_) => Ok(encoded_token_signature),
      Err(_) => Err("Internal database error"),
    }
  }

  pub fn get_token(&mut self, signature: &String) -> Option<Token> {
    let token_key = format!("tokens:{}", signature);
    let raw_value = self.db.get(token_key.as_bytes())?;
    return match serde_json::from_slice(&raw_value) {
      Ok(token) => Some(token),
      Err(_) => None
    }
  }

  pub fn get_state(&mut self, username: &String) -> Option<String> {
    let state_key = format!("users:{}:state", username);
    let raw_state = self.db.get(state_key.as_bytes())?;
    return Some(base64::encode(&raw_state));
  }

  pub fn store_state(&mut self, username: &String, state: &String) -> Result<(), &'static str> {
    let state_key = format!("users:{}:state", username);
    let encoded_state = match base64::decode(state) {
      Ok(encoded_state) => encoded_state,
      Err(_) => { return Err("Invalid encoding"); },
    };

    if let Err(_) = self.db.put(state_key.as_bytes(), &encoded_state) {
      return Err("Internal database error");
    }

    return match self.db.flush() {
      Ok(_) => Ok(()),
      Err(_) => Err("Internal database error"),
    }
  }
}
