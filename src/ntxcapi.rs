use windows::{
    core::PSTR,
    Win32::{
        Foundation::{BOOLEAN, NTSTATUS},
        System::Diagnostics::Debug::{CONTEXT, EXCEPTION_RECORD},
    },
};

pub const KCONTINUE_FLAG_TEST_ALERT: u32 = 1;
pub const KCONTINUE_FLAG_DELIVER_APC: u32 = 2;
extern "C" {
    pub fn RtlDispatchException(
        ExceptionRecord: *mut EXCEPTION_RECORD,
        ContextRecord: *mut CONTEXT,
    ) -> BOOLEAN;
}
extern "C" {
    pub fn RtlRaiseStatus(Status: NTSTATUS) -> !;
}
extern "C" {
    pub fn RtlRaiseException(ExceptionRecord: *mut EXCEPTION_RECORD);
}
extern "C" {
    pub fn NtContinue(ContextRecord: *mut CONTEXT, TestAlert: BOOLEAN) -> NTSTATUS;
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
    pub Reserved: [u64; 2usize],
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
extern "C" {
    pub fn NtContinueEx(
        ContextRecord: *mut CONTEXT,
        ContinueArgument: *mut std::ffi::c_void,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn NtRaiseException(
        ExceptionRecord: *mut EXCEPTION_RECORD,
        ContextRecord: *mut CONTEXT,
        FirstChance: BOOLEAN,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn RtlAssert(
        VoidFailedAssertion: *mut std::ffi::c_void,
        VoidFileName: *mut std::ffi::c_void,
        LineNumber: u32,
        MutableMessage: PSTR,
    ) -> !;
}
