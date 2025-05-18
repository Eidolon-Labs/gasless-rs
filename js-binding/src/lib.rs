use node_bindgen::derive::node_bindgen;
use alloy_primitives::{Address, hex};
use gasless::{is_address, mine_free_gas};

#[node_bindgen(name = "mineGasForTransaction")]
async fn mine_gas_for_transaction(gas_amount: u32, address: String, nonce: u32) -> Result<MiningOutput, String> {
    if !is_address(&address) {
        return Err("Invalid Address".to_string());
    }

    let address = Address::parse_checksummed(&address, None)
        .map_err(|_| "Invalid address format".to_string())?;

    let result = mine_free_gas(gas_amount, address, nonce)?;

    Ok(MiningOutput {
        duration: result.0.as_secs_f64() * 1000.0,
        gas_price: format!("0x{}", hex::encode(result.1.to_be_bytes::<32>())),
    })
}

#[node_bindgen]
struct MiningOutput {
    pub duration: f64,
    pub gas_price: String,
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_skale_pow_mining() {
        let from_address = Address::parse_checksummed("0x742d35Cc6634C0532925a3b844Bc454e4438f44e", None).unwrap();
        let nonce = 42;
        let gas = 21000;
        
        println!("Testing SKALE PoW gas mining");
        
        let start = Instant::now();
        let result = mine_free_gas(gas, from_address, nonce).unwrap();
        
        let elapsed = start.elapsed();
        println!("Mining completed successfully!");
        println!("Gas price: 0x{}", hex::encode(result.1.to_be_bytes::<32>()));
        println!("Mining took: {} seconds", result.0.as_secs_f64());
        println!("Actual elapsed time: {:.2} seconds", elapsed.as_secs_f64());
        
        assert!(result.0.as_secs_f64() > 0.0, "Duration should be positive");
    }
}