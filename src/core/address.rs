use diesel::TextExpressionMethods;
use k256::elliptic_curve::{PublicKey, ScalarCore};
use k256::elliptic_curve::sec1::{EncodedPoint, ToEncodedPoint};
use k256::{Secp256k1, SecretKey};
use rand_core::OsRng;
use sha3::{Sha3_256};
use sha2::{Sha256, Digest};
use bs58;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    pub public_key: Vec<u8>,
    pub private_key: Option<Vec<u8>>,
    pub address: String,
}

impl Address {
    pub fn create() -> Address {
        let secret = SecretKey::new(ScalarCore::random(&mut OsRng));
        let pub_key = secret.public_key().to_encoded_point(false);
        Address::from(&pub_key.to_bytes(),
                      Option::from(secret.to_be_bytes().to_vec()))
    }
    pub fn from_public(public_key: &[u8]) -> Address {
        Address::from(public_key, None)
    }
    pub fn from_private(private_key: &[u8]) -> Address {
        let secret = SecretKey::from_be_bytes(private_key)
            .expect("Couldn't retrieve secret from be bytesz");
        let pub_key = secret.public_key().to_encoded_point(false);
        Address::from(&pub_key.to_bytes(),
                      Option::from(Vec::from(private_key)))
    }
    fn from(public_key: &[u8], be_bytes: Option<Vec<u8>>) -> Address {
        let mut hasher = Sha3_256::new();
        let pub_bytes = &public_key[1..];
        hasher.update(pub_bytes);
        let hash3 = hasher.finalize();
        let first_byte: &[u8] = &[0x26];
        let last_bytes: &[u8] = &hash3[(hash3.len() - 20)..];
        let address_bytes: &[u8] = &[first_byte, last_bytes].concat();
        let copy_address_bytes: &[u8] = address_bytes.clone();
        let mut hasher256 = Sha256::new();
        hasher256.update(address_bytes);
        let h1 = hasher256.finalize();
        hasher256 = Sha256::new();
        hasher256.update(h1);
        let h2 = hasher256.finalize();
        let check_sum = &h2[0..4];
        let final_bytes: &[u8] = &[copy_address_bytes, check_sum].concat();
        let encoded = bs58::encode(final_bytes).into_string();
        Address {
            address: encoded,
            public_key: Vec::from(public_key),
            private_key: be_bytes,
        }
    }
    pub fn is_valid(&self) -> bool {
        let address = &self.address;
        let decoded = bs58::decode(address).into_vec().unwrap();
        let address_bytes: &[u8] = &decoded[0..decoded.len() - 4];
        let check_sum: &[u8] = &decoded[decoded.len() - 4..];
        let mut hasher256 = Sha256::new();
        hasher256.update(address_bytes);
        let h1 = hasher256.finalize();
        hasher256 = Sha256::new();
        hasher256.update(h1);
        let h2 = hasher256.finalize();
        let result = &h2[0..4];

        result == check_sum
    }
}
