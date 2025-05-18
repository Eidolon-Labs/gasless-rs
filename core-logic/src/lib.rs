use alloy_primitives::{Address, B256, U256};
use rand::Rng;
use std::time::{Instant, Duration};

pub fn is_address(value: &str) -> bool {
    if !value.starts_with("0x") || value.len() != 42 {
        return false;
    }
    value[2..].chars().all(|c| c.is_ascii_hexdigit())
}

pub fn mine_free_gas(gas_amount: u32, address: Address, nonce: u32) -> Result<(Duration, U256), String> {
    let nonce_bytes = U256::from(nonce).to_be_bytes::<32>();
    let nonce_hash = U256::from_be_bytes(B256::from(keccak256(&nonce_bytes)).0);
    let address_hash = U256::from_be_bytes(B256::from(keccak256(address.as_slice())).0);
    let nonce_address_xor = nonce_hash ^ address_hash;
    let div_constant = U256::MAX;

    let start = Instant::now();
    let mut rng = rand::thread_rng();
    let mut iterations = 0;

    loop {
        let mut candidate_bytes = [0u8; 32];
        rng.fill(&mut candidate_bytes);
        let candidate = U256::from_be_bytes(candidate_bytes);
        let candidate_hash = U256::from_be_bytes(B256::from(keccak256(&candidate.to_be_bytes::<32>())).0);
        let result_hash = nonce_address_xor ^ candidate_hash;

        if result_hash == U256::ZERO {
            continue;
        }

        let external_gas = div_constant / result_hash;

        if external_gas >= U256::from(gas_amount) {
            let duration = start.elapsed();
            return Ok((duration, candidate));
        }

        iterations += 1;
        if iterations % 5000 == 0 {
            std::thread::yield_now();
        }
    }
}

fn keccak256(data: impl AsRef<[u8]>) -> [u8; 32] {
    use sha3::{Digest, Keccak256};
    let mut hasher = Keccak256::new();
    hasher.update(data);
    hasher.finalize().into()
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_address() {
        assert!(is_address("0x1234567890123456789012345678901234567890"));
        assert!(!is_address("0x12345"));
        assert!(!is_address("not_an_address"));
    }
    
    #[test]
    fn test_basic_mining() {
        let address = Address::parse_checksummed("0x1234567890123456789012345678901234567890", None).unwrap();
        let result = mine_free_gas(21000, address, 1);
        assert!(result.is_ok());
    }
}