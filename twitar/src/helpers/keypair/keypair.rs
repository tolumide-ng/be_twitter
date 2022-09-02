#[cfg(test)]
#[path = "./keypair.test.rs"]
mod keypair_test;


#[derive(Debug, Clone)]
pub struct KeyPair {
    pub key: String,
    pub secret: String,
}

impl KeyPair {
    /// Creates KeyPair with the given key and secret.
    pub fn new(key: String, secret: String) -> KeyPair {
        KeyPair { key: key.into(), secret: secret.into() }
    }

    // Create an empty KeyPair
    pub fn empty() -> KeyPair {
        KeyPair { key: "".into(), secret: "".into() }
    }
}

