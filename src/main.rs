mod config;
mod crypto;
mod api;
mod user_db;

use config::Config;
use crypto::Crypto;
use user_db::UserDB;
use api::run;

fn main() {
  let config = Config::new().unwrap();
  let crypto = Crypto::new(&config).unwrap();
  let db = UserDB::new(&config);

  run(crypto, db);
}
