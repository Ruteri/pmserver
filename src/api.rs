use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize};
use std::sync::Mutex;

use crate::user_db::UserDB;
use crate::crypto::Crypto;

#[derive(Deserialize)]
struct LoginData {
  username: String,
  password: String,
}

#[derive(Deserialize)]
struct TokenSignaturenData {
  token: String, /* base64-encoded string */
}

#[derive(Deserialize)]
struct StoreStateData {
  token: String, /* base64-encoded string */
  state: String, /* base64-encoded string */
}

#[derive(Deserialize)]
struct RegisterUserData {
  username: String,
  password: String,
}

struct WebState {
  db: Mutex<UserDB>,
  crypto: Mutex<Crypto>,
}

fn get_token(state: web::Data<WebState>, data: web::Json<LoginData>) -> impl Responder {
  let user_provided_password = match base64::decode(&data.password) {
    Ok(pass) => pass,
    Err(_) => return HttpResponse::BadRequest().finish(),
  };

  let mut db = state.db.lock().unwrap();
  let pw_hash = match db.get_user_pw_hash(&data.username) {
    Some(pw_hash) => pw_hash,
    None => return HttpResponse::Unauthorized().finish(),
  };

  if pw_hash != Crypto::hash256(&user_provided_password) {
    return HttpResponse::Unauthorized().finish();
  }

  let crypto = state.crypto.lock().unwrap();
  match db.generate_token(&crypto, &data.username) {
    Ok(token) => HttpResponse::Ok().body(token),
    Err(_) => HttpResponse::InternalServerError().finish(),
  }
}

fn get_state(state: web::Data<WebState>, data: web::Json<TokenSignaturenData>) -> impl Responder {
  let mut db = state.db.lock().unwrap();
  let token = match db.validate_token(&data.token) {
    Some(token) => token,
    None => return HttpResponse::Unauthorized().finish(),
  };

  match db.get_state(&token.username) {
    Some(state) => HttpResponse::Ok().body(state),
    None => HttpResponse::BadRequest().finish(),
  }
}

fn store_state(state: web::Data<WebState>, data: web::Json<StoreStateData>) -> impl Responder {
  let mut db = state.db.lock().unwrap();

  let token = match db.validate_token(&data.token) {
    Some(token) => token,
    None => return HttpResponse::Unauthorized().finish(),
  };

  /* if token.exp_time > t.now() delete token and return error */

  match db.store_state(&token.username, &data.state) {
    Ok(_) => HttpResponse::Ok().finish(),
    Err(_) => HttpResponse::InternalServerError().finish(),
  }
}

fn register_user(state: web::Data<WebState>, data: web::Json<RegisterUserData>) -> impl Responder {
  let user_provided_password = match base64::decode(&data.password) {
    Ok(pass) => pass,
    Err(_) => return HttpResponse::BadRequest().body("Invalid password"),
  };

  let mut db = state.db.lock().unwrap();

  match db.register_user(&data.username, &user_provided_password) {
    Ok(_) => HttpResponse::Ok().finish(),
    Err(e) => HttpResponse::InternalServerError().body(e),
  }
}

pub fn run(crypto: Crypto, db: UserDB) {
  let db_web_state: web::Data<WebState> = web::Data::new(WebState { db: Mutex::new(db), crypto: Mutex::new(crypto) });

  HttpServer::new(move || {
      App::new()
        .register_data(db_web_state.clone())
        .route("/token", web::get().to(get_token))
        .route("/users/data", web::get().to(get_state))
        .route("/users/data", web::post().to(store_state))
        .route("/users", web::post().to(register_user))
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .unwrap();
}
