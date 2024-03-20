use ethnum::U256;

pub trait Hash {
    fn hash(&self) -> U256;
}
