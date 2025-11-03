# Solwatch 🛰️

A selective Solana indexer and dashboard that tracks any wallet's on-chain activity in real-time.
Built with Rust (Axum, Tokio, SQLx) + React + PostgreSQL.

## Features
- Real-time transaction feed via WebSocket.
- Background indexer fetching Solana account data.
- Postgres-backed caching for fast queries.
- Reactive UI for transaction and balance updates.

## Tech Stack
**Backend:** Rust (Axum, SQLx, Tokio)  
**Frontend:** React + JavaScript  
**Database:** PostgreSQL  
**Infra:** Docker Compose  
