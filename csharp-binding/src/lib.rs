use gasless::{mine_free_gas};
use alloy_primitives::{Address, U256, hex};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[unsafe(no_mangle)]
pub extern "C" fn mine_gas_c(gas_amount: u32, address: *const c_char, nonce: u32) -> *mut c_char {
    if address.is_null() {
        return std::ptr::null_mut();
    }

    let c_str = unsafe { CStr::from_ptr(address) };
    let addr_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let addr = match Address::parse_checksummed(addr_str, None) {
        Ok(a) => a,
        Err(_) => return std::ptr::null_mut(),
    };

    let result = mine_free_gas(gas_amount, addr, nonce);

    match result {
        Ok((duration, gas)) => {
            let response = format!("duration_ms:{};gas_price:0x{}", duration.as_secs_f64() * 1000.0, hex::encode(gas.to_be_bytes::<32>()));
            CString::new(response).unwrap().into_raw()
        }
        Err(_) => std::ptr::null_mut(),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn free_cstring(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            CString::from_raw(s);
        }
    }
}
