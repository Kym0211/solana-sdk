//! The greet sysvar provides greeting ie "GM GM" 
//!
//! [`Greet`] implements [`Sysvar::get`] and can be loaded efficiently without
//! passing the sysvar account ID to the program.
//!
//! See also the Solana [SIMD proposal][simd].
//!
//! [simd]: https://github.com/solana-foundation/solana-improvement-documents/blob/main/proposals/0047-syscall-and-sysvar-for-last-restart-slot.md
//!
//! # Examples
//!
//! Accessing via on-chain program directly:
//!
//! ```no_run
//! # use solana_account_info::AccountInfo;
//! # use solana_msg::msg;
//! # use solana_sysvar::Sysvar;
//! # use solana_program_error::ProgramResult;
//! # use solana_pubkey::Pubkey;
//! # use solana_greet::Greet;
//!
//! fn process_instruction(
//!     program_id: &Pubkey,
//!     accounts: &[AccountInfo],
//!     instruction_data: &[u8],
//! ) -> ProgramResult {
//!
//!     let greet = Greet::get();
//!     msg!("last restart slot: {:?}", greet.greeting);
//!
//!     Ok(())
//! }
//! ```
//!
#[cfg(feature = "bincode")]
use crate::SysvarSerialize;
use crate::{impl_sysvar_get, Sysvar};
pub use {
    solana_greet::Greet,
    solana_sdk_ids::sysvar::greet::{check_id, id, ID},
};

impl Sysvar for Greet {
    impl_sysvar_get!(sol_get_greet_sysvar);
}

// #[cfg(feature = "bincode")]
// impl SysvarSerialize for Greet {}
