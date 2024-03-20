use core::slice::SlicePattern;
use ed25519_dalek::{ed25519::SignatureBytes, Signature};
use ethnum::*;
use ring::digest;
use serde::{de::DeserializeOwned, Deserialize, Deserializer, Serialize, Serializer};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{Chain, TinyBlockchainParams};

// TODO: refactor naming
pub fn get_zeroes_in_u256(byte_array: &[u8; 32]) -> (u8, u8) {
    let mut bit_position: u8 = 0;
    let mut additional_bits: u8 = 0;

    for b in byte_array {
        if *b == 0 {
            bit_position += 8;
        } else {
            for i in (0..8).rev() {
                if b & (1 << i) != 0 {
                    break;
                }
                additional_bits += 1;
            }
            break;
        }
    }

    (bit_position, additional_bits)
}

pub fn pack_to_32_bits(value: U256) -> u32 {
    let bytes_array = value.to_be_bytes();
    eprintln!("Bytes: {:?}", bytes_array);

    // determine exponent
    let (bit_position, additional_bits) = get_zeroes_in_u256(&bytes_array);
    let bit_position = bit_position as usize;
    let additional_bits = additional_bits as usize;

    let mut exponent = 32 - ((bit_position) / 8);
    let byte_position = bit_position / 8;

    if byte_position >= 0x20 - 3 {
        // TODO: don't cast like this, the exponent should be saved. Just return max value in this case
        return value.as_u32();
    }

    // increase exponent if half byte is 0
    if additional_bits == 0 && bit_position != 0 {
        exponent += 1
    }

    let half_zero_byte = if byte_position != 0 {
        bytes_array[byte_position]
    } else {
        bytes_array[0]
    };

    let mantissa = u32::from_be_bytes([
        half_zero_byte,
        bytes_array[byte_position + 1],
        bytes_array[byte_position + 2],
        bytes_array[byte_position + 3],
    ]);

    // eprintln!("Bit position: {}", bit_position);
    // eprintln!("Exponent: {:x} = {}", half_zero_byte, half_zero_byte);
    // eprintln!("Mantissa: {:x}", mantissa);

    // combine exponent and mantissa into 32-bit integer
    (exponent as u32) << 24 | (mantissa >> 8)
}

pub fn unpack_to_256_bits(value: u32) -> U256 {
    ((value & 0x00ffffff).as_u256() * (256.as_u256()).pow(((value >> 24) & 0xff) - 3)).as_u256()
}

pub fn str_to_hex(hex: &str) -> U256 {
    U256::from_str_radix(hex, 16).unwrap()
}

pub fn seconds_now() -> u64 {
    let since_the_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_secs()
}

pub fn is_epoch(block_height: usize, chain: &Chain, params: &TinyBlockchainParams) -> bool {
    chain.len() >= params.blocks_in_epoch as usize
        && block_height % params.blocks_in_epoch as usize == 0
}

pub fn get_epoch_time(chain: &Chain, params: &TinyBlockchainParams) -> u64 {
    if chain.len() < params.blocks_in_epoch {
        // TODO: panic doesn't make sense here, it should continue to work
        panic!("Epoch is less than 2016 blocks")
    }

    let now = seconds_now();
    let block_index = chain.len() - params.blocks_in_epoch;
    if let Some(epoch_started) = chain.get_block(block_index) {
        println!("Now: {}, Started: {}", now, epoch_started.header.timestamp);
        return now - epoch_started.header.timestamp;
    }

    // TODO: look weird, think about it
    panic!("The block doesn't exists")
}

#[derive(Serialize, Deserialize, PartialEq, Eq)]
#[serde(remote = "U256")]
pub struct U256Def([u128; 2]);

impl From<U256Def> for U256 {
    fn from(def: U256Def) -> Self {
        // Convert from U256Def to ethnum::u256
        // This might involve calling a constructor or other conversion logic
        Self::from_words(def.0[0], def.0[1])
    }
}

impl From<U256> for U256Def {
    fn from(u: ethnum::u256) -> Self {
        // Convert from ethnum::u256 to U256Def
        // This might simply involve copying values or more complex logic
        let words = u.into_words();
        Self([words.0, words.1])
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DigestWrapper(
    #[serde(
        serialize_with = "serialize_digest",
        deserialize_with = "deserialize_digest"
    )]
    digest::Digest,
);

fn serialize_digest<S>(digest: &digest::Digest, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // Serialize the digest as a byte array
    serializer.serialize_bytes(digest.as_ref())
}

fn deserialize_digest<'de, D>(deserializer: D) -> Result<digest::Digest, D::Error>
where
    D: Deserializer<'de>,
{
    // Deserialize the byte array into a digest
    let bytes = Vec::deserialize(deserializer)?.as_slice();
    let digest = digest::digest(&digest::SHA256, bytes);
    Ok(digest)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SignatureWrapper(
    #[serde(
        serialize_with = "serialize_signature",
        deserialize_with = "deserialize_signature"
    )]
    Option<Signature>,
);

fn serialize_signature<S>(sig: &Option<Signature>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // Serialize the signature as a byte array
    match sig {
        Some(inner) => serializer.serialize_bytes(&inner.to_bytes()),
        None => String::from("null").serialize(serializer),
    }
}

fn deserialize_signature<'de, D, T>(deserializer: D) -> Result<Option<Signature>, D::Error>
where
    D: Deserializer<'de>,
    T: DeserializeOwned + PartialEq + From<String>,
{
    // Deserialize the byte array into a signature
    let value = T::deserialize(deserializer)?;
    if value == T::from(String::from("null")) {
        Ok(None)
    } else {
        let bytes = Vec::deserialize(deserializer)?.as_slice();
        let sig = Signature::from_slice(bytes).unwrap();
        Ok(Some(sig))
    }
}
