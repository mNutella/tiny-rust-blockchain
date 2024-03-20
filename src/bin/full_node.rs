use ring::{
    digest, rand,
    signature::{Ed25519KeyPair, KeyPair, UnparsedPublicKey, ED25519},
};
use tiny_blockchain::{hash_to_u256, Transaction};

const LEAF_PREFIX: &[u8] = &[0];
macro_rules! hash_leaf {
    {$d:expr} => {
        hash_to_u256!(&[LEAF_PREFIX, $d].concat())
    }
}

fn generate_pk() -> Ed25519KeyPair {
    // Generate a key pair
    let rng = rand::SystemRandom::new();
    let pkcs8_bytes = Ed25519KeyPair::generate_pkcs8(&rng).unwrap();

    // Create a key pair from the PKCS#8 document
    let key_pair = Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref()).unwrap();

    key_pair
}

fn main() {
    // let to_keypair = generate_pk();
    // let to_pk_bytes = to_keypair.public_key().as_ref();
    // let to_pk_digest = digest::digest(&digest::SHA256, to_pk_bytes);

    // let from_keypair = generate_pk();
    // let from_pk_bytes = from_keypair.public_key().as_ref();
    // let from_pk = UnparsedPublicKey::new(&ED25519, from_pk_bytes);

    // let tx = Transaction::new(&from_keypair, to_pk_digest, 10);

    // println!("Tx: {:?}", tx);
    // println!("Verification: {:?}", tx.verify(from_pk, &from_keypair));
    // println!("Tx hash: {:?}", hash_to_u256!(tx.as_bytes()));
}
