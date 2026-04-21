use solana_sdk::pubkey::Pubkey;

use crate::{ example::{ self } };

pub struct PDA;

impl PDA {
    pub async fn get_global_pda() -> (Pubkey, u8) {
        let (global_pda, _bump) = Pubkey::find_program_address(&[b"globals"], &example::ID);

        (global_pda, _bump)
    }

    pub async fn round(round_id: u64) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[b"round", &round_id.to_le_bytes()], &example::ID)
    }

    pub async  fn ticket(round_id: u64, ticket_no: u8) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[b"ticket", &round_id.to_le_bytes(), &[ticket_no]],
            &example::ID
        )
    }
}
