use windows::Win32::Foundation::NTSTATUS;

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtFreezeTransactions(FreezeTimeout: *mut i64, ThawTimeout: *mut i64) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtThawTransactions() -> NTSTATUS;
}
