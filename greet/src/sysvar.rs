pub use solana_sdk_ids::sysvar::greet::{check_id, id, ID};
use {crate::Greet, solana_sysvar_id::impl_sysvar_id};

impl_sysvar_id!(Greet);
