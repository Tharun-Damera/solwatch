// SyncStatus enum is used in SSE API communication
#[derive(Clone, Debug)]
pub enum SyncStatus {
    Started,
    AccountData(String),
    TransactionSignatures(String),
    TransactionDetails(String),
    Completed,
    Error(String),
}
