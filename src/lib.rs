mod blockchain;
mod consensus;
mod hash;
mod merkle_tree;
mod network;
mod primitives;
mod utils;

pub use blockchain::*;
pub use consensus::*;
pub use hash::*;
pub use merkle_tree::*;
pub use network::*;
pub use primitives::*;
pub use utils::*;

#[macro_export]
macro_rules! hash_to_u256 {
    ($bytes:expr) => {{
        use ethnum::U256;
        use sha2::{Digest, Sha256};

        let mut hasher = Sha256::new();
        hasher.update($bytes);
        let result = format!("0x{:x}", hasher.finalize());
        U256::from_str_hex(&result).unwrap()
    }};
}
