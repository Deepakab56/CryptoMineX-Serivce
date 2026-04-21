use std::sync::Arc;
use axum::{ Json, extract::State, http::StatusCode };
use mongodb::{ Database, bson::{ doc } };
use serde_json::{ Value, json };

use crate::{
    example::{ self, accounts::{ GlobalState, Round } },
    models::Rounds,
    routes::error_response,
    utils::{ PDA, instance },
};

#[axum::debug_handler]
pub async fn close_round(State(db): State<Arc<Database>>) -> Result<
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
                    .accounts(example::client::accounts::CloseAccount {
                        signer: program.payer(),
                        global_account: global_pda,
                        round_account: round_pda,
                    })
                    .args(example::client::args::CloseAccount {})
                    .send().await
                    .map_err(|err|
                        error_response(StatusCode::INTERNAL_SERVER_ERROR, &err.to_string())
                    )?;

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

    let collection = db.collection::<Rounds>("Rounds");

    let mut doc = doc! {};
    doc.insert("is_round_active", false);

    let round_id = round_account.round_id.to_string();

    let _result = collection
        .update_one(doc! { "is_round_active":round_id }, doc! { "$set":doc }, None).await
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
