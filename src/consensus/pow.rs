use crate::{
    get_epoch_time, hash_to_u256, pack_to_32_bits, seconds_now, unpack_to_256_bits, Block, Chain,
    Hash, TinyBlockchainParams,
};
use ethnum::*;

pub struct Proof {
    pub timestamp: u64,
    pub nonce: u32,
    pub hash: U256, // merkle_root: U256,
}

pub fn pow_validate(items: &Vec<Block>) -> bool {
    if items.is_empty() {
        return false;
    }

    // TODO: check if current index out of bounds
    for prev in 0..items.len() - 1 {
        let curr = prev + 1;
        let curr_block = items.get(curr).expect("Current block should exist");
        let prev_block = items.get(prev).expect("Previous block should exist");
        let current_hash = unpack_to_256_bits(curr_block.header.bits);

        if curr_block.header.prev != prev_block.header.hash() {
            return false;
        }

        let hash = hash_proof(curr_block.header.timestamp, curr_block.header.nonce);

        // TODO: why less or equal, not just equal
        // if hash <= current_hash {
        if hash != current_hash {
            return false;
        }
    }

    true
}

pub fn pow(bits: u32) -> Proof {
    let mut nonce: u32 = 1;
    let mut timestamp = seconds_now();
    let mut hash: U256;
    let target_hash = unpack_to_256_bits(bits);

    loop {
        hash = hash_proof(timestamp, nonce);

        if hash <= target_hash {
            break;
        }

        let (new_nonce, overflowed) = nonce.overflowing_add(1);
        if overflowed {
            timestamp = seconds_now();
        }

        nonce = new_nonce;
    }

    Proof {
        timestamp,
        nonce,
        hash,
    }
}

pub fn hash_proof(timestamp: u64, nonce: u32) -> U256 {
    let bytes = [timestamp.to_be_bytes(), (nonce as u64).to_be_bytes()].concat();

    hash_to_u256!(bytes)
}

// TODO: not sure that rest of the logic is related to POW

pub fn retarget(bits: u32, chain: &Chain, params: &TinyBlockchainParams) -> u32 {
    let epoch_takes = get_epoch_time(chain, params);
    let mut change_factor = epoch_takes as f64 / (params.blocks_in_epoch * 10 * 60) as f64;

    if change_factor > 4.0 {
        change_factor = 4.0
    } else if change_factor < 0.25 {
        change_factor = 0.25
    }

    let new_bits = calc_difficulty(change_factor, bits);

    new_bits
}

pub fn calc_difficulty(change_factor: f64, bits: u32) -> u32 {
    let mantissa = bits & 0x00ffffff;
    let exp = ((bits >> 24) & 0xff) - 3;
    let new_value = (change_factor * mantissa as f64).as_u256() * (256).as_u256().pow(exp);

    pack_to_32_bits(new_value)
}
