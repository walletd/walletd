use crate::keystore::{Locked, Unlocked};
use crate::Error;
use crate::HDKey;
use crate::KeyPair;
use crate::{KeyStore, LockState};
use std::fs::File;

pub trait KeyManager {}

impl KeyManager for KeyStore<Unlocked> {}
