use crate::{pow_validate, seconds_now, Block, BlockHeader, Hash, Transaction};
use ethnum::*;
use serde::{Deserialize, Serialize};

const DEFAULT_DIFFICULTY_TARGET: u32 = 0x1d00ffff;

#[derive(Debug, Serialize, Deserialize)]
pub struct Chain {
    pub items: Vec<Block>,
    pub last_update: u64,
}

impl Chain {
    pub fn get_block(&self, block_index: usize) -> Option<&Block> {
        self.items.get(block_index)
    }

    pub fn add_block(&mut self, block: Block) {
        self.items.push(block);
        self.last_update = seconds_now();
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn is_valid(&self) -> bool {
        pow_validate(&self.items)
    }

    pub fn previous_block(&self) -> Option<&Block> {
        self.items.last()
    }
}

enum TinyBlockchainError {
    IvalidChain,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TinyBlockchainParams {
    pub blocks_in_epoch: usize,
    pub init_difficulty: u32,
    pub epoch: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TinyBlockchain {
    chain: Chain,
    params: TinyBlockchainParams,
}

impl TinyBlockchain {
    pub fn new(chain: Chain, params: TinyBlockchainParams) -> Self {
        TinyBlockchain { chain, params }
    }

    fn init_genesis_block(transactions: Vec<Transaction>) -> Block {
        let block_header = BlockHeader {
            version: 1,
            prev: 0.as_u256(),
            merkle_root: 0.as_u256(),
            timestamp: seconds_now(),
            bits: DEFAULT_DIFFICULTY_TARGET,
            nonce: 1,
        };
        Block::new(0, block_header.hash(), block_header, transactions)
    }

    fn add_block(&mut self, block: Block) {
        self.chain.add_block(block);
    }

    // fn create_block(&self, block_header: BlockHeader, transactions: Vec<Transaction>) -> Block {
    //     Block::new(self.chain.len() + 1, block_header, transactions)
    // }

    fn set_chain(&mut self, new_chain: Chain) -> Result<(), TinyBlockchainError> {
        if !pow_validate(&new_chain.items) || self.chain.last_update >= new_chain.last_update {
            return Err(TinyBlockchainError::IvalidChain);
        }

        self.chain = new_chain;

        Ok(())
    }

    // fn count_blocks_in_epoch(&self) -> usize {
    //     let ago = seconds_now() - EPOCH;
    //     let mut blocks_in_two_weeks = 0;
    //
    //     for block in self.chain.items.iter().rev() {
    //         if block.header.timestamp < ago {
    //             break;
    //         }
    //
    //         blocks_in_two_weeks += 1;
    //     }
    //
    //     blocks_in_two_weeks
    // }
}

pub fn generate_mock_blocks(n: u32) -> Vec<Block> {
    let mut blocks = vec![TinyBlockchain::init_genesis_block(vec![])];

    for i in 1..n {
        let block_header = BlockHeader {
            version: 1,
            nonce: 1,
            prev: blocks[(i - 1) as usize].header_hash,
            merkle_root: 0.as_u256(),
            // bits: 0x1d00ffff,
            bits: 0x1dffffff,
            timestamp: seconds_now(),
        };
        let block = Block::new(i as usize, block_header.hash(), block_header, vec![]);
        blocks.push(block)
    }

    blocks
}
