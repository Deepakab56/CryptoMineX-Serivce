use std::sync::Arc;
use axum::{ Json, extract::State, http::StatusCode };
use mongodb::{ Database, bson::doc, options::UpdateOptions };
use serde_json::{ Value, json };

use crate::{
    example::accounts::{
        GlobalState,
        Ticket as OnChainTicket, // ✅ alias — on-chain
    },
    models::TicketDocument, // ✅ renamed MongoDB model
    routes::error_response,
    utils::{ PDA, instance },
};

#[axum::debug_handler]
pub async fn get_ticket(State(db): State<Arc<Database>>) -> Result<
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

                let mut tickets = vec![];

                for i in 1u8..=25 {
                    let (ticket_pda, _) = PDA::ticket(global_account.round_id, i).await;

                    match program.account::<OnChainTicket>(ticket_pda).await {
                        Ok(ticket_data) => {
                            tickets.push(
                                json!({
                            "ticket_no": i,
                            "exists": true,
                            "total_amount": ticket_data.total_amount.to_string(),
                            "users": ticket_data.users
                                .iter()
                                .map(|u| u.to_string())
                                .collect::<Vec<_>>(),
                        })
                            );
                        }
                        Err(_) => {
                            tickets.push(
                                json!({
                            "ticket_no": i,
                            "exists": false,
                            "total_amount": "0",
                            "users": [],
                        })
                            );
                        }
                    }
                }

                Ok::<_, (StatusCode, Json<Value>)>((tickets, global_account.round_id))
            })
        }).await
        .map_err(|e| error_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    let (tickets, round_id) = result?;

    let collection = db.collection::<TicketDocument>("TicketDocument");

    for ticket in &tickets {
        if ticket["exists"] == true {
            let doc = TicketDocument {
                id: None,
                round_id,
                ticket_no: ticket["ticket_no"].as_u64().unwrap_or(0) as u8,
                users: ticket["users"]
                    .as_array()
                    .unwrap_or(&vec![])
                    .iter()
                    .map(|u| u.as_str().unwrap_or("").to_string())
                    .collect(),
                total_amount: ticket["total_amount"].as_str().unwrap_or("0").parse().unwrap_or(0),
            };

            let options = UpdateOptions::builder().upsert(true).build();

            collection
                .update_one(
                    doc! { 
            "round_id": round_id as i64, 
            "ticket_no": doc.ticket_no as i32 
        },
                    doc! { 
            "$set": {
                "users": &doc.users,
                "total_amount": doc.total_amount as i64,
            }
        },
                    options
                ).await
                .map_err(|e| error_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;
        }
    }
    Ok(
        Json(
            json!({
        "success": true,
        "round_id": round_id,
        "tickets": tickets,
    })
        )
    )
}
