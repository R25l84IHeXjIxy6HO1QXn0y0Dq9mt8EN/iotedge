use crate::constants::{AAD_BYTES, IV_BYTES};
use crate::ks;
use crate::ks::{KeyHandle, Text};
use crate::util::BoxedResult;

use base64::encode;
use ring::rand::{generate, SystemRandom};
use zeroize::Zeroize;

// NOTE: open to changing implementation so that Sync is not required
pub trait StoreBackend: Sync {
    type Error: std::error::Error;

    // NOTE: lack of &self is intentional
    fn initialize() -> Result<(), Self::Error>;

    fn write_record(&self, id: &str, record: Record) -> Result<(), Self::Error>;
    fn update_record(&self, id: &str, record: Record) -> Result<(), Self::Error>;
    fn read_record(&self, id: &str) -> Result<Record, Self::Error>;
    fn delete_record(&self, id: &str) -> Result<(), Self::Error>;
}

// NOTE: not fully public since high-level functions should be
//       invariant over backend implementation
pub(crate) struct Store<T: StoreBackend>(pub T);

impl<T: StoreBackend> Store<T> {
    // NOTE: can remove Pin<Box<...>> if async traits are added to Rust
    //       cf. https://docs.rs/crate/async-trait
    pub async fn get_secret<'a>(&'a self, id: &'a str) -> BoxedResult<'a, String> {
        let record = self.0.read_record(id)?;
        let KeyHandle(key) = ks::get_key(id).await?;
        let ptext = match ks::decrypt(&key, &record.ciphertext, &record.iv, &record.aad).await? {
            Text::Plaintext(ptext) => ptext,
            _ => panic!("ENCRYPTION API CHANGED")
        };

        Ok(ptext)
    }

    pub async fn set_secret<'a>(&'a self, id: &'a str, value: String) -> BoxedResult<'a, ()> {
        let rng = SystemRandom::new();

        let KeyHandle(key) = ks::create_key(id).await?;
        let iv: String = encode(generate::<[u8; IV_BYTES]>(&rng)?.expose());
        let aad: String = encode(generate::<[u8; AAD_BYTES]>(&rng)?.expose());

        let ctext = match ks::encrypt(&key, &value, &iv, &aad).await? {
            Text::Ciphertext(ctext) => ctext,
            _ => panic!("DECRYPTION API CHANGED")
        };

        self.0.write_record(id, Record {
            ciphertext: ctext,
            iv: iv,
            aad: aad
        })?;

        Ok(())
    }

    /*
    fn pull_secrets(&'a self, vault: ..., keys: [&str]) -> Pin<Box<dyn Future<Output = BoxedResult<()>> + 'a>>;
    */
}

#[derive(Zeroize)]
#[zeroize(drop)]
pub struct Record {
    pub ciphertext: String,
    pub iv: String,
    pub aad: String
}