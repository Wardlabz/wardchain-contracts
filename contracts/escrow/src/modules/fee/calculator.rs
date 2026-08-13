use crate::{
    error::ContractError,
    modules::{
        math::{BasicArithmetic, BasicMath},
        math::{SafeArithmetic, SafeMath},
    },
};

const WARDCHAIN_FEE_BPS: u32 = 30;
const BASIS_POINTS_DENOMINATOR: i128 = 10000;

#[derive(Debug, Clone)]
pub struct StandardFeeResult {
    pub wardchain_fee: i128,
    pub platform_fee: i128,
    pub receiver_amount: i128,
}

pub trait FeeCalculatorTrait {
    fn calculate_standard_fees(
        total_amount: i128,
        platform_fee_bps: u32,
    ) -> Result<StandardFeeResult, ContractError>;
}

#[derive(Clone)]
pub struct FeeCalculator;

impl FeeCalculatorTrait for FeeCalculator {
    fn calculate_standard_fees(
        total_amount: i128,
        platform_fee_bps: u32,
    ) -> Result<StandardFeeResult, ContractError> {
        let wardchain_fee = SafeMath::safe_mul_div(
            total_amount,
            WARDCHAIN_FEE_BPS,
            BASIS_POINTS_DENOMINATOR,
        )?;
        let platform_fee =
            SafeMath::safe_mul_div(total_amount, platform_fee_bps, BASIS_POINTS_DENOMINATOR)?;

        let after_wardchain = BasicMath::safe_sub(total_amount, wardchain_fee)?;
        let receiver_amount = BasicMath::safe_sub(after_wardchain, platform_fee)?;

        Ok(StandardFeeResult {
            wardchain_fee,
            platform_fee,
            receiver_amount,
        })
    }
}
