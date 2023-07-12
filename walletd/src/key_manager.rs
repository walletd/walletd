use crate::HDKey;
use crate::KeyPair;
use crate::{KeyStore, LockState};
use crate::keystore::{Unlocked, Locked};
use crate::Error;
use std::fs::File;
use chacha20poly1305;

pub trait KeyManager {}

impl KeyManager for KeyStore<Unlocked> {
}



