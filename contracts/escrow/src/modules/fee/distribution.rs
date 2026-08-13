use soroban_sdk::token::Client as TokenClient;
use soroban_sdk::{Address, Env, Map, Vec};

use crate::error::ContractError;
use super::StandardFeeResult;
use crate::modules::math::{BasicArithmetic, BasicMath};

pub fn calculate_and_distribute_fees(
    e: &Env,
    token_client: &TokenClient,
    contract_address: &Address,
    wardchain_address: &Address,
    platform_address: &Address,
    fee_result: &StandardFeeResult,
    distributions: &Map<Address, i128>,
    total: i128,
) -> Result<(), ContractError> {
    let mut actual_wardchain_fees = 0i128;
    let mut actual_platform_fees = 0i128;
    let mut net_distributions: Vec<(Address, i128)> = Vec::new(e);

    for (addr, amount) in distributions.iter() {
        if amount > 0 {
            let recipient_wardchain_fee = BasicMath::safe_div(
                BasicMath::safe_mul(amount, fee_result.wardchain_fee)?,
                total,
            )?;
            let recipient_platform_fee = BasicMath::safe_div(
                BasicMath::safe_mul(amount, fee_result.platform_fee)?,
                total,
            )?;

            let total_recipient_fee =
                BasicMath::safe_add(recipient_wardchain_fee, recipient_platform_fee)?;
            let net_amount = BasicMath::safe_sub(amount, total_recipient_fee)?;

            actual_wardchain_fees =
                BasicMath::safe_add(actual_wardchain_fees, recipient_wardchain_fee)?;
            actual_platform_fees =
                BasicMath::safe_add(actual_platform_fees, recipient_platform_fee)?;

            if net_amount > 0 {
                net_distributions.push_back((addr.clone(), net_amount));
            }
        }
    }

    if actual_wardchain_fees > 0 {
        token_client.transfer(contract_address, wardchain_address, &actual_wardchain_fees);
    }
    if actual_platform_fees > 0 {
        token_client.transfer(contract_address, platform_address, &actual_platform_fees);
    }

    for (addr, net_amount) in net_distributions.iter() {
        token_client.transfer(contract_address, &addr, &net_amount);
    }

    Ok(())
}