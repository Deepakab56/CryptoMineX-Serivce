use mongodb::bson::oid::ObjectId;
use serde::{ Deserialize, Serialize };
use solana_sdk::pubkey::Pubkey;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Rounds {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub is_round_active: bool,
    pub total_amount: u64,
    pub round_id: u64,
    pub start_time: u64,
    pub end_time: u64,
    pub winner_ticket: u8,
    pub users: Vec<Pubkey>,
    pub randomness_account: Pubkey,
    pub tx_signature: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateRound {
    pub is_round_active: bool,
    pub total_amount: u64,
    pub round_id: u64,
    pub start_time: u64,
    pub end_time: u64,
    pub winner_ticket: u8,
    pub users: Vec<Pubkey>,
    pub randomness_account: Pubkey,
    pub tx_signature: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CloseRound {
    is_round_active: bool,
}
// models/mod.rs
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateTicket {
    pub users: Vec<Pubkey>,
    pub total_amount: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TicketDocument {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub round_id: u64,
    pub ticket_no: u8,
    pub users: Vec<String>,
    pub total_amount: u64,
}
