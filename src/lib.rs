use std::fmt;

use base58::ToBase58;
use hex::ToHex;
use libsecp256k1::Error as Secp256k1Error;
use libsecp256k1::{PublicKey, SecretKey};
use sha2::Sha256;
use sha3::Digest;
use sha3::Keccak256;

/// Key Errors.
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// Public key format error.
    InvalidPublic,
    /// Digest data format error.
    InvalidMessage,
    /// Signature data format error.
    InvalidSignature,
    /// Invalid checksum of base58check.
    InvalidChecksum,
    /// Private key format error.
    InvalidPrivate,
    /// Invalid address format.
    InvalidAddress,
    /// Unable to generate a key pair.
    FailedKeyGeneration,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::InvalidPublic => "Invalid Public",
            Error::InvalidMessage => "Invalid Message",
            Error::InvalidSignature => "Invalid Signature",
            Error::InvalidChecksum => "Invalid Checksum",
            Error::InvalidPrivate => "Invalid Private",
            Error::InvalidAddress => "Invalid Address",
            Error::FailedKeyGeneration => "Key generation failed",
        };

        msg.fmt(f)
    }
}

impl std::error::Error for Error {}

impl From<Secp256k1Error> for Error {
    fn from(e: Secp256k1Error) -> Self {
        match e {
            Secp256k1Error::InvalidPublicKey => Error::InvalidPublic,
            Secp256k1Error::InvalidSecretKey => Error::InvalidPrivate,
            Secp256k1Error::InvalidMessage => Error::InvalidMessage,
            _ => Error::InvalidSignature,
        }
    }
}

/// The mainnet uses 0x41('A') as address type prefix.
const ADDRESS_TYPE_PREFIX: u8 = 0x41;

/// Private key of Secp256k1.
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Private([u8; 32]);

impl Private {
    /// As raw bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0[..]
    }
}

impl fmt::Display for Private {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.encode_hex::<String>().fmt(f)
    }
}

/// Public key of Secp256k1.
#[derive(Clone)]
pub struct Public([u8; 64]);

impl AsRef<[u8]> for Public {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl fmt::Display for Public {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (&self.0[..]).encode_hex::<String>().fmt(f)
    }
}

impl Public {
    /// Public key from private key.
    pub fn from_private(private: &Private) -> Result<Public, Error> {
        let secret_key = SecretKey::parse_slice(private.as_bytes())?;
        let pub_key = PublicKey::from_secret_key(&secret_key);

        let mut key = [0u8; 64];
        key[..].copy_from_slice(&pub_key.serialize()[1..]);

        Ok(Public(key))
    }
}

/// Address of Tron, saved in 21-byte format.
#[derive(PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
pub struct Address([u8; 21]);

impl Address {
    /// Address of a private key.
    pub fn from_private(private: &Private) -> Address {
        Address::from_public(&Public::from_private(private).expect("public from private; qed"))
    }

    /// Address of a public key.
    pub fn from_public(public: &Public) -> Address {
        let mut hasher = Keccak256::new();
        hasher.update(public);
        let digest = hasher.finalize();

        let mut raw = [ADDRESS_TYPE_PREFIX; 21];
        raw[1..21].copy_from_slice(&digest[digest.len() - 20..]);

        Address(raw)
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        b58encode_check(&self.0).fmt(f)
    }
}

/// Base58check encode.
pub fn b58encode_check<T: AsRef<[u8]>>(raw: T) -> String {
    let mut hasher = Sha256::new();
    hasher.update(raw.as_ref());
    let digest1 = hasher.finalize();

    let mut hasher = Sha256::new();
    hasher.update(&digest1);
    let digest = hasher.finalize();

    let mut raw = raw.as_ref().to_owned();
    raw.extend(&digest[..4]);
    raw.to_base58()
}

pub struct KeyPair {
    private: Private,
    public: Public,
}

impl KeyPair {
    pub fn from(sk: SecretKey, pk: PublicKey) -> KeyPair {
        let mut pk_data = [0u8; 64];
        pk_data[..].copy_from_slice(&pk.serialize()[1..]);

        KeyPair {
            private: Private(sk.serialize()),
            public: Public(pk_data),
        }
    }

    pub fn simple_print(&self) {
        println!(
            "{addr}:{priv}",
            addr = &Address::from_public(&self.public),
            priv = self.private
        )
    }
}

impl std::fmt::Display for KeyPair {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        writeln!(f, "private: {:}", &self.private)?;
        writeln!(f, "public:  {:}", &self.public)?;
        write!(f, "address: {:}", &Address::from_public(&self.public))
    }
}
