use axum::{ Router, http::Method };
use tower_http::cors::{ Any, CorsLayer };
use solana_sdk::pubkey::Pubkey;
use std::env;
use dotenv::dotenv;
use anchor_lang::declare_program;

declare_program!(example);
use crate::example::accounts::GlobalState;

mod db;
pub mod utils;
pub mod models;
pub mod routes;

use utils::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let db = db::connect().await.expect("Failed to connect to database");
    println!("Database connected");

    let (_connection, _provider, instances) = instance()?;
    println!("Program ID: {}", example::ID);

    let (global_pda, bump) = Pubkey::find_program_address(&[b"globals"], &example::ID);
    println!("Global PDA: {}", global_pda);
    println!("Bump: {}", bump);

    let global_account: GlobalState = instances.account(global_pda).await?;
    println!("Round ID: {:?}", global_account.round_id);

    let port = env::var("PORT").unwrap_or_else(|_| "5200".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any)
        .allow_origin(Any);

    let app = Router::new().nest("/", routes::routes(db)).layer(cors);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("🚀 Server running on http://{}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}

pub async fn hello() -> &'static str {
    "Hello from Rust 🚀"
}
