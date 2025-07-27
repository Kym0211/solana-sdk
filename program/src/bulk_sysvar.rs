#[inline]
pub fn bulk_sysvar_fetch() -> u64 {
    #[cfg(target_os = "solana")]
    unsafe {
        crate::syscalls::bulk_sysvar_fetch()
    }

    #[cfg(not(target_os = "solana"))] 
    {
        crate::program_stubs::bulk_sysvar_fetch()
    }
}

