use crate::{hash_to_u256, DigestWrapper, Hash, U256Def};
use ethnum::{AsU256, U256};
use ring::{
    digest,
    signature::{Ed25519KeyPair, KeyPair, Signature, UnparsedPublicKey},
};
use serde::{Deserialize, Serialize};

/** An input of a transaction. It contains the location of the previous
 * transaction's output that it claims and a signature that matches the
 * output's public key.
 */
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UtxoInput {
    prev_output: Option<usize>,
    sig: DigestWrapper,
}

/** An output of a transaction. It contains the public key that the next input
 * must be able to sign with to claim it.
 */
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UtxoOutput {
    pk: DigestWrapper,
    value: u32,
}

/** The basic transaction that is broadcasted on the network and contained in
 * blocks. A transaction can contain multiple inputs and outputs.
 */
#[derive(Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub version: u32,
    pub inputs: Vec<UtxoInput>,
    pub outputs: Vec<UtxoOutput>,
    #[serde(with = "U256Def")]
    pub hash: U256,
    pub sig: Option<Signature>,
}

// pub struct Transaction {
//     pub version: u32,
//     pub sig: Option<Signature>,
//     pub amount: u32,
//     pub from: digest::Digest,
//     pub to: digest::Digest,
// }

impl Transaction {
    // pub fn new(from_keypair: &Ed25519KeyPair, to_pubkey: digest::Digest, amount: u32) -> Self {
    //     // Get the public key
    //     let pubkey_bytes = from_keypair.public_key().as_ref();

    //     // Represent the public key as a digest
    //     let pubkey_digest = digest::digest(&digest::SHA256, pubkey_bytes);

    //     let mut tx = Transaction {
    //         version: 1,
    //         from: pubkey_digest,
    //         to: to_pubkey,
    //         amount,
    //         sig: None,
    //     };
    //     tx.sign(from_keypair);

    //     tx
    // }

    pub fn new(
        from_keypair: &Ed25519KeyPair,
        inputs: Vec<UtxoInput>,
        outputs: Vec<UtxoOutput>,
    ) -> Self {
        // Get the public key
        let pubkey_bytes = from_keypair.public_key().as_ref();

        // Represent the public key as a digest
        let pubkey_digest = digest::digest(&digest::SHA256, pubkey_bytes);

        let mut tx = Transaction {
            version: 1,
            inputs,
            outputs,
            hash: 0.as_u256(),
            sig: None,
        };
        tx.hash = tx.hash();
        tx.sign(from_keypair);

        tx
    }

    pub fn is_coinbase(&self) -> bool {
        self.inputs.len() == 1 && self.inputs[0].prev_output.is_none()
    }

    // TODO: move validation logic outside
    // pub fn is_valid(&self) -> bool {
    //     self.sig.is_some() && self.amount > 0
    // }

    pub fn sign(&mut self, keypair: &Ed25519KeyPair) {
        self.sig = Some(keypair.sign(&self.as_bytes()));
    }

    pub fn verify(&self, pub_key: UnparsedPublicKey<&[u8]>, temp_keypair: &Ed25519KeyPair) -> bool {
        if let Some(signature) = self.sig {
            // TODO: think about cloning, maybe there is a way to avoid the cloning (to make sig None)
            let mut temp_sig = self.clone();
            temp_sig.sig = None;

            match pub_key.verify(&temp_sig.as_bytes(), signature.as_ref()) {
                Ok(_) => return true,
                Err(err) => {
                    eprintln!("Error: {:?}", err);
                    return false;
                }
            }
        }

        false
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        format!("{:?}", self).into_bytes()
    }
}

impl std::fmt::Debug for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut sig = String::with_capacity(0);

        if let Some(ref signature) = self.sig {
            sig = signature
                .as_ref()
                .iter()
                .map(|c| format!("{:x}", c))
                .collect::<String>();
        }

        // write!(
        //     f,
        //     "\nFrom: {:?}\nTo: {:?}\nAmount: {}\nSignature: {:?}\n",
        //     self.from, self.to, self.amount, sig
        // )
        write!(
            f,
            "Inputs: {:?}\nOtputs: {:?}\nSignature: {:?}\n",
            self.inputs, self.outputs, sig
        )
    }
}

impl Hash for Transaction {
    fn hash(&self) -> U256 {
        hash_to_u256!(format!("{:?}", self).as_bytes())
    }
}
