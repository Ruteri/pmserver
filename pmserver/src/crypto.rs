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
    Ok(Crypto { session_key })
  }

  pub fn hash256(to_hash: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::default();
    hasher.input(to_hash);
    hasher.result().to_vec()
  }

  pub fn sign(&self, to_sign: &[u8]) -> Vec<u8> {
    let mut hash = Crypto::hash256(to_sign);

    {
      let mut hash_block_l = GenericArray::from_mut_slice(&mut hash[..16]);
      self.session_key.encrypt_block(&mut hash_block_l);
    }
    {
      let mut hash_block_h = GenericArray::from_mut_slice(&mut hash[16..]);
      self.session_key.encrypt_block(&mut hash_block_h);
    }

    hash.to_vec()
  }
}

