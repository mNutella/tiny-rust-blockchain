use crate::{hash_to_u256, Hash, Transaction, U256Def};
use ethnum::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BlockHeader {
    pub version: u32,
    pub timestamp: u64,
    #[serde(with = "U256Def")]
    pub prev: U256,
    #[serde(with = "U256Def")]
    pub merkle_root: U256,
    pub bits: u32,
    pub nonce: u32,
}

impl std::fmt::Debug for BlockHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Prev Hash: {:#064x}, Merkle Hash: {:#064x}, Timestamp: {}, Nonce: {}",
            self.prev, self.merkle_root, self.timestamp, self.nonce
        )
    }
}

impl Hash for BlockHeader {
    fn hash(&self) -> U256 {
        hash_to_u256!(format!("{:?}", self).as_bytes())
    }
}

#[derive(Serialize, Deserialize)]
pub struct Block {
    pub height: usize,
    pub header: BlockHeader,
    #[serde(with = "U256Def")]
    pub header_hash: U256, // TODO: remove?
    pub transactions_count: usize,
    pub transactions: Vec<Transaction>,
}

impl std::fmt::Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let transactions_str: String = self
            .transactions
            .iter()
            .map(|tx| {
                let mut sig = None;

                if let Some(ref signature) = tx.sig {
                    sig = Some(signature.as_ref());
                }

                // format!(
                //     // "From: {:#064x}, To: {:#064x}, Amount: {}",
                //     "From: {:?}, To: {:?}, Amount: {}, Signature: {:?}",
                //     tx.from.as_ref(),
                //     tx.to.as_ref(),
                //     tx.amount,
                //     sig
                // )
                format!(
                    "Inputs: {:?}, outputs: {:?}, Signature: {:?}",
                    tx.inputs, tx.outputs, sig
                )
            })
            .collect::<Vec<String>>()
            .join(",");
        write!(
            f,
            "Height: {}, Header: <{:?}>, Header Hash: {:#064x}, Transactions Count: {}, Transactions: {}",
            self.height,
            self.header,
            self.header_hash,
            self.transactions_count,
            transactions_str
        )
    }
}

impl Block {
    pub fn new(
        height: usize,
        header_hash: U256,
        header: BlockHeader,
        transactions: Vec<Transaction>,
    ) -> Self {
        Block {
            height,
            header,
            header_hash,
            transactions_count: transactions.len(),
            transactions,
        }
    }

    fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
    }
}
