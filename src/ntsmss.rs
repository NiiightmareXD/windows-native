use windows::Win32::Foundation::{HANDLE, NTSTATUS, UNICODE_STRING};

use crate::ntlpcapi::PORT_MESSAGE;

extern "C" {
    pub fn RtlConnectToSm(
        ApiPortName: *mut UNICODE_STRING,
        ApiPortHandle: HANDLE,
        ProcessImageType: u32,
        SmssConnection: *mut HANDLE,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn RtlSendMsgToSm(ApiPortHandle: HANDLE, MessageData: *mut PORT_MESSAGE) -> NTSTATUS;
}
