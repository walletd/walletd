//! This module contains functions to handle fee estimation for Monero
//! transactions

use crate::MoneroAmount;

pub const DEFAULT_DUST_THRESHOLD: u64 = 2000000000; // 2 * pow(10, 9)
const APPROXIMATE_INPUT_BYTES: usize = 80;

pub fn estimate_rct_tx_size(
    n_inputs: usize,
    mixin: usize,
    n_outputs: usize,
    extra_size: usize,
    bulletproof: bool,
    clsag: bool,
    bulletproof_plus: bool,
    use_view_tags: bool,
) -> u64 {
    let mut size = 0;
    // tx prefix and first few bytes
    size += 1 + 6;
    // vin
    size += n_inputs * (1 + 6 + (mixin + 1) * 2 + 32);
    // vout
    size += n_outputs * (6 + 32);
    // extra
    size += extra_size;
    // rct signatures
    // type
    size += 1;

    // rangeSigs
    if bulletproof || bulletproof_plus {
        let mut log_padded_outputs = 0;
        while (1 << log_padded_outputs) < n_outputs {
            log_padded_outputs += 1;
            size +=
                (2 * (6 + log_padded_outputs) + if bulletproof_plus { 6 } else { 4 + 5 }) * 32 + 3;
        }
    } else {
        size += (2 * 64 * 32 + 32 + 64 * 32) * n_outputs;
    }

    // MGs/CLSAGs
    if clsag {
        size += n_inputs * (32 * (mixin + 1) + 64);
    } else {
        size += n_inputs * (64 * (mixin + 1) + 32);
    }

    // sizeof(crypto::view_tag);
    if use_view_tags {
        size += n_outputs * crate::transaction::ViewTag::size_of();
    }

    // pseudoOuts
    size += 32 * n_inputs;
    // ecdhInfo
    size += 8 * n_outputs;
    // outPk - only commitment is saved
    size += 32 * n_outputs;
    // txnFee
    size += 4;

    size as u64
}

pub fn estimate_tx_size(
    use_rct: bool,
    n_inputs: usize,
    mixin: usize,
    n_outputs: usize,
    extra_size: usize,
    bulletproof: bool,
    clsag: bool,
    bulletproof_plus: bool,
    use_view_tags: bool,
) -> u64 {
    if use_rct {
        return estimate_rct_tx_size(
            n_inputs,
            mixin,
            n_outputs,
            extra_size,
            bulletproof,
            clsag,
            bulletproof_plus,
            use_view_tags,
        );
    } else {
        return (n_inputs * (mixin + 1) * APPROXIMATE_INPUT_BYTES
            + extra_size
            + (if use_view_tags { todo!() } else { 0 })) as u64;
    }
}

pub fn estimate_tx_weight(
    use_rct: bool,
    n_inputs: usize,
    mixin: usize,
    n_outputs: usize,
    extra_size: usize,
    bulletproof: bool,
    clsag: bool,
    bulletproof_plus: bool,
    use_view_tags: bool,
) -> u64 {
    let mut size = estimate_tx_size(
        use_rct,
        n_inputs,
        mixin,
        n_outputs,
        extra_size,
        bulletproof,
        clsag,
        bulletproof_plus,
        use_view_tags,
    );
    if use_rct && (bulletproof || bulletproof_plus) && n_outputs > 2 {
        // notional size of a 2 output proof, normalized to 1 proof (ie, divided by 2)
        let bp_base = (32 * ((if bulletproof_plus { 6 } else { 9 }) + 7 * 2)) / 2;
        let mut log_padded_outputs = 2;
        while (1 << log_padded_outputs) < n_outputs {
            log_padded_outputs += 1;
        }
        let nlr = 2 * (6 + log_padded_outputs);
        let bp_size = 32 * ((if bulletproof_plus { 6 } else { 9 }) + nlr);
        let bp_clawback = (bp_base * (1 << log_padded_outputs) - bp_size) * 4 / 5;
        size += bp_clawback;
    }
    size
}

pub fn calculate_fee_from_weight(
    base_fee: u64,
    weight: u64,
    fee_quantization_mask: u64,
) -> MoneroAmount {
    let mut fee = weight * base_fee;
    fee = (fee + fee_quantization_mask - 1) / fee_quantization_mask * fee_quantization_mask;
    MoneroAmount::from_piconero(fee)
}

pub fn calculate_fee(fee_per_kb: u64, bytes: u64) -> MoneroAmount {
    let kB = (bytes + 1023) / 1024;
    MoneroAmount::from_piconero(kB * fee_per_kb)
}

pub fn estimate_fee(
    use_per_byte_fee: bool,
    use_rct: bool,
    n_inputs: usize,
    mixin: usize,
    n_outputs: usize,
    extra_size: usize,
    bulletproof: bool,
    clsag: bool,
    bulletproof_plus: bool,
    use_view_tags: bool,
    base_fee: u64,
    fee_quantization_mask: u64,
) -> MoneroAmount {
    if use_per_byte_fee {
        let estimated_tx_weight = estimate_tx_weight(
            use_rct,
            n_inputs,
            mixin,
            n_outputs,
            extra_size,
            bulletproof,
            clsag,
            bulletproof_plus,
            use_view_tags,
        );
        return calculate_fee_from_weight(base_fee, estimated_tx_weight, fee_quantization_mask);
    } else {
        let estimated_tx_size = estimate_tx_size(
            use_rct,
            n_inputs,
            mixin,
            n_outputs,
            extra_size,
            bulletproof,
            clsag,
            bulletproof_plus,
            use_view_tags,
        );
        return calculate_fee(base_fee, estimated_tx_size);
    }
}
