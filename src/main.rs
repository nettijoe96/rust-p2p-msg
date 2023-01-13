use k256::{SecretKey};
use k256::ecdsa::{recoverable, SigningKey, VerifyingKey};
use k256::ecdsa::signature::{Signer, Verifier};
use std::fs;

const KEY: &'static str = "-----BEGIN EC PRIVATE KEY-----
MHQCAQEEIAMBqidYvIzYZAEs48Z4pPIc16nFREmEz5RkXIcaNVIooAcGBSuBBAAK
oUQDQgAEXgsMy/GcsIS7PltglEVmLGM31BlpBhp0lySj/QhdAfRiqWDbvmSwAqeN
pWp2LOzT8Q2sRbPeVK/dbw598aDNBg==
-----END EC PRIVATE KEY-----";


fn main() {
    let path = "../keys/sk0.pem";
    //let pem = fs::read_to_string(path).expect("Couldn't find or load that file.");

    let sk = SecretKey::from_sec1_pem(KEY).unwrap(); // replace unwrap with '?'
    // if sk.is_err() {
    //     let e = sk.err();
    //     println!("err3 {:?}", e);
    // }

    let signing_key: SigningKey = sk.into();
    let message = b"ECDSA proves knowledge of a secret number in the context of a single message";
    let signature: recoverable::Signature = signing_key.sign(message);
    signature.s().to_bytes();

    println!("{:?}", signature);
    
    let verifying_key: VerifyingKey = signing_key.verifying_key();
    let ok = verifying_key.verify(message, &signature).is_ok();
    println!("{ok}");
    
}