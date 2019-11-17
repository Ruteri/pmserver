use aes::block_cipher_trait::generic_array::GenericArray;
use aes::block_cipher_trait::BlockCipher;
use aes::Aes256;
use sha2::{Sha256, Digest};

use crate::config::Config;

pub struct Crypto {
  session_key: Aes256, /* TODO: should be stored in secret-service or similar */
}

impl Crypto {
  pub fn new(config: &Config) -> Result<Crypto, &'static str> {
    let mut hasher = Sha256::default();
    hasher.input("5922d97fddb970f2c29847bb970c28b2c1aa0d5f73babeaff4fcf970db0d2bec");
    hasher.input(&config.raw_session_key_seed);
    let seed = hasher.result();

    let key_arr = GenericArray::from_slice(&seed);
    let session_key = Aes256::new(key_arr);
    return Ok(Crypto { session_key });
  }

  pub fn sign(&self, to_sign: &Vec<u8>) -> Vec<u8> {
    let mut hasher = Sha256::default();
    hasher.input(to_sign);
    let hash = hasher.result();
    // let mut hash_block = GenericArray::clone_from_slice(hash.as_slice());
    // self.session_key.encrypt_block(hash.as_mut_slice());
    return hash.to_vec();
  }
}

