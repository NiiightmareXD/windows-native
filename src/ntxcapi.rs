use windows::Win32::{
    Foundation::NTSTATUS,
    System::Diagnostics::Debug::{CONTEXT, EXCEPTION_RECORD},
};

pub const KCONTINUE_FLAG_TEST_ALERT: u32 = 1;
pub const KCONTINUE_FLAG_DELIVER_APC: u32 = 2;

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlDispatchException(
        ExceptionRecord: *mut EXCEPTION_RECORD,
        ContextRecord: *mut CONTEXT,
    ) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlRaiseStatus(Status: NTSTATUS) -> !;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtContinue(ContextRecord: *mut CONTEXT, TestAlert: bool) -> NTSTATUS;
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum KCONTINUE_TYPE {
    KCONTINUE_UNWIND = 0,
    KCONTINUE_RESUME = 1,
    KCONTINUE_LONGJUMP = 2,
    KCONTINUE_SET = 3,
    KCONTINUE_LAST = 4,
}

#[repr(C)]
pub struct KCONTINUE_ARGUMENT {
    pub ContinueType: KCONTINUE_TYPE,
    pub ContinueFlags: u32,
    pub Reserved: [u64; 2],
}

impl Default for KCONTINUE_ARGUMENT {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for KCONTINUE_ARGUMENT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "KCONTINUE_ARGUMENT {{ ContinueType: {:?}, Reserved: {:?} }}",
            self.ContinueType, self.Reserved
        )
    }
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtContinueEx(
        ContextRecord: *mut CONTEXT,
        ContinueArgument: *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtRaiseException(
        ExceptionRecord: *mut EXCEPTION_RECORD,
        ContextRecord: *mut CONTEXT,
        FirstChance: bool,
    ) -> NTSTATUS;
}
