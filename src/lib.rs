use bitcoin::{absolute::LockTime, psbt::Psbt, Transaction};
use miniscript::psbt::finalize_mall;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn test() {
    let pre_psbt_tx = Transaction {
        version: 2,
        lock_time: LockTime::ZERO,
        input: vec![],
        output: vec![],
    };

    let mut psbt = Psbt::from_unsigned_tx(pre_psbt_tx).unwrap();

    // Construct a secp context for transaction signature verification
    let secp = bitcoin::secp256k1::Secp256k1::verification_only();
    // Assuming all partial sigs are filled in.
    // Construct a generic finalizer
    finalize_mall(&mut psbt, &secp).unwrap()
}
