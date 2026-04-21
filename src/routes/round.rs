use std::sync::Arc;
use axum::{ Json, extract::State, http::StatusCode,  };
use mongodb::Database;
use serde_json::{ Value, json };
use anchor_lang::system_program;

use crate::{
    example::{ self, accounts::{ GlobalState, Round }, client::args },
    models::CreateRound,
    utils::{ PDA, instance },
};


#[axum::debug_handler]
pub async fn create_round(State(db): State<Arc<Database>>) -> Result<
    Json<Value>,
    (StatusCode, Json<Value>)
> {
    let result = tokio::task
        ::spawn_blocking(move || {
            let rt = tokio::runtime::Handle::current();

            rt.block_on(async {
                let (_connection, _provider, program) = instance().map_err(|e|
                    error_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string())
                )?;

                let (global_pda, _) = PDA::get_global_pda().await;

                let global_account: GlobalState = program
                    .account(global_pda).await
                    .map_err(|e|
                        error_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string())
                    )?;

                let (round_pda, _) = PDA::round(global_account.round_id).await;

                let tx = program
                    .request()
                    .accounts(example::client::accounts::InitializeRound {
                        signer: program.payer(),
                        global_account: global_pda,
                        round_account: round_pda,
                        system_program: system_program::ID,
                    })
                    .args(args::InitializeRound {})
                    .send().await
                    .map_err(|e| error_response(StatusCode::BAD_REQUEST, &e.to_string()))?;

                println!("TX: {}", tx);

                let round_account: Round = program
                    .account(round_pda).await
                    .map_err(|e|
                        error_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string())
                    )?;

                Ok::<_, (StatusCode, Json<Value>)>((tx, round_account, round_pda, global_account))
            })
        }).await
        .map_err(|e| error_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    let (tx, round_account, round_pda, global_account) = result?;

    // ✅ MongoDB insert — spawn_blocking ke bahar (Database Send hai)
    let collection = db.collection::<CreateRound>("rounds");

    let new_round = CreateRound {
        is_round_active: true,
        total_amount: round_account.total_amount,
        round_id: round_account.round_id,
        start_time: round_account.start_time,
        end_time: round_account.end_time,
        winner_ticket: round_account.winner_ticket,
        users: round_account.users,
        randomness_account: round_account.randomness_account,
        tx_signature: Some(tx.to_string()),
    };

    collection
        .insert_one(&new_round, None).await
        .map_err(|e| error_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    Ok(
        Json(
            json!({
        "success": true,
        "signature": tx.to_string(),
        "round_id": global_account.round_id,
        "round_pda": round_pda.to_string(),
    })
        )
    )
}

pub fn error_response(status: StatusCode, msg: &str) -> (StatusCode, Json<Value>) {
    (status, Json(json!({ "success": false, "error": msg })))
}
