//extern crate ecdsa;

use k256::{SecretKey};
use k256::ecdsa::{SigningKey, VerifyingKey};
use k256::ecdsa::Signature;
use k256::ecdsa::signature::{Signer, Verifier};
use std::fs;

const KEY: &'static str = "-----BEGIN EC PRIVATE KEY-----
MHQCAQEEIAMBqidYvIzYZAEs48Z4pPIc16nFREmEz5RkXIcaNVIooAcGBSuBBAAK
oUQDQgAEXgsMy/GcsIS7PltglEVmLGM31BlpBhp0lySj/QhdAfRiqWDbvmSwAqeN
pWp2LOzT8Q2sRbPeVK/dbw598aDNBg==
-----END EC PRIVATE KEY-----";

//Result<Signature, Box<dyn std::error::Error>>
// pub fn sign(keyfile: String, msg: String) -> Vec<u8> {
//     let path = format!("../keys/{keyfile}");
//     let pem = fs::read_to_string(path).expect("Couldn't find or load that file.");
//     let sk = SecretKey::from_sec1_pem(&pem).unwrap(); // replace unwrap with '?'

//     let signing_key: SigningKey = sk.into();
//     let msgb = msg.as_bytes();
//     let sig: Signature = signing_key.sign(msgb);
//     println!("{:?}", sig);

//     return sigb;
// }

// fn verify(pubkey: &[i8], sig: &[i8], msg: String) -> Result<bool, Box<dyn std::error::Error>> {

//     // let verifying_key: VerifyingKey = signing_key.verifying_key();
//     // let ok = verifying_key.verify(message, &signature).is_ok();
//     // println!("{ok}");

//     Ok();
// }

