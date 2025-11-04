use chrono::{DateTime, Utc};

#[derive(Debug)]
struct _Accounts {
    address: String,
    lamports: u64,
    owner: String,
    executable: bool,
    rent_epoch: u64,
    data_length: u64,
    indexed_at: DateTime<Utc>,
    last_updated_at: DateTime<Utc>,
}
