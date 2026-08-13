use crate::storage::types::Escrow;
use soroban_sdk::{contractevent, String};

#[contractevent(topics = ["wardchain_init"], data_format = "vec")]
#[derive(Clone)]
pub struct InitEsc {
    pub escrow: Escrow,
}

#[contractevent(topics = ["wardchain_fund"], data_format = "vec")]
#[derive(Clone)]
pub struct FundEsc {
    pub signer: soroban_sdk::Address,
    pub amount: i128,
}

#[contractevent(topics = ["wardchain_release"], data_format = "single-value")]
#[derive(Clone)]
pub struct DisEsc {
    pub release_signer: soroban_sdk::Address,
}

#[contractevent(topics = ["wardchain_update"], data_format = "vec")]
#[derive(Clone)]
pub struct ChgEsc {
    pub platform: soroban_sdk::Address,
    pub engagement_id: String,
    pub new_escrow_properties: Escrow,
}

// Milestones
#[contractevent(topics = ["wardchain_ms_change"], data_format = "vec")]
#[derive(Clone)]
pub struct MilestoneStatusChanged {
    pub escrow: Escrow,
}

#[contractevent(topics = ["wardchain_ms_approve"], data_format = "vec")]
#[derive(Clone)]
pub struct MilestoneApproved {
    pub escrow: Escrow,
}

// Disputes
#[contractevent(topics = ["wardchain_disp_resolve"], data_format = "vec")]
#[derive(Clone)]
pub struct DisputeResolved {
    pub escrow: Escrow,
}

#[contractevent(topics = ["wardchain_dispute"], data_format = "vec")]
#[derive(Clone)]
pub struct EscrowDisputed {
    pub escrow: Escrow,
}

// Admin / TTL
#[contractevent(topics = ["wardchain_ttl_extend"], data_format = "vec")]
#[derive(Clone)]
pub struct ExtTtlEvt {
    pub platform: soroban_sdk::Address,
    pub ledgers_to_extend: u32,
}
