use dashmap::DashMap;
use std::sync::{Arc, atomic::AtomicBool};
use tokio::sync::broadcast;

use mongodb::Database;
use solana_client::nonblocking::rpc_client::RpcClient;
use tracing::warn;

use crate::message::SyncStatus;

#[derive(Debug)]
pub struct AddressSession {
    pub sender: broadcast::Sender<SyncStatus>,
    pub started: AtomicBool,
}

#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub rpc: Arc<RpcClient>,
    pub session: Arc<DashMap<String, Arc<AddressSession>>>,
}

impl AppState {
    pub fn new(db: Database, rpc: Arc<RpcClient>) -> Self {
        AppState {
            db,
            rpc,
            session: Arc::new(DashMap::new()),
        }
    }

    pub fn get_or_create_session(&self, address: &str) -> Arc<AddressSession> {
        warn!("DashMap session data: {:?}", self.session);
        self.session
            .entry(address.to_string())
            .or_insert_with(|| {
                let (sender, _) = broadcast::channel(10);
                Arc::new(AddressSession {
                    sender,
                    started: AtomicBool::new(false),
                })
            })
            .clone()
    }

    pub fn remove_session(&self, address: &str) -> bool {
        match self.session.remove(address) {
            Some(_) => true,
            None => false,
        }
    }
}
