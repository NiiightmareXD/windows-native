use windows::{
    Wdk::Foundation::OBJECT_ATTRIBUTES,
    Win32::{
        Foundation::{HANDLE, NTSTATUS, UNICODE_STRING},
        Security::{PSID, SECURITY_DESCRIPTOR, SECURITY_QUALITY_OF_SERVICE},
        System::{Threading::SRWLOCK, WindowsProgramming::CLIENT_ID},
    },
};

use crate::bitfield::{BitfieldUnit, UnionField};

pub const PORT_CONNECT: u32 = 1;
pub const PORT_ALL_ACCESS: u32 = 2031617;
pub const LPC_REQUEST: u32 = 1;
pub const LPC_REPLY: u32 = 2;
pub const LPC_DATAGRAM: u32 = 3;
pub const LPC_LOST_REPLY: u32 = 4;
pub const LPC_PORT_CLOSED: u32 = 5;
pub const LPC_CLIENT_DIED: u32 = 6;
pub const LPC_EXCEPTION: u32 = 7;
pub const LPC_DEBUG_EVENT: u32 = 8;
pub const LPC_ERROR_EVENT: u32 = 9;
pub const LPC_CONNECTION_REQUEST: u32 = 10;
pub const PORT_VALID_OBJECT_ATTRIBUTES: u32 = 64;
pub const ALPC_PORFLG_LPC_MODE: u32 = 4096;
pub const ALPC_PORFLG_ALLOW_IMPERSONATION: u32 = 65536;
pub const ALPC_PORFLG_ALLOW_LPC_REQUESTS: u32 = 131072;
pub const ALPC_PORFLG_WAITABLE_PORT: u32 = 262144;
pub const ALPC_PORFLG_ALLOW_DUP_OBJECT: u32 = 524288;
pub const ALPC_PORFLG_SYSTEM_PROCESS: u32 = 1048576;
pub const ALPC_PORFLG_WAKE_POLICY1: u32 = 2097152;
pub const ALPC_PORFLG_WAKE_POLICY2: u32 = 4194304;
pub const ALPC_PORFLG_WAKE_POLICY3: u32 = 8388608;
pub const ALPC_PORFLG_DIRECT_MESSAGE: u32 = 16777216;
pub const ALPC_PORFLG_ALLOW_MULTIHANDLE_ATTRIBUTE: u32 = 33554432;
pub const ALPC_PORFLG_OBJECT_TYPE_FILE: u32 = 1;
pub const ALPC_PORFLG_OBJECT_TYPE_INVALID: u32 = 2;
pub const ALPC_PORFLG_OBJECT_TYPE_THREAD: u32 = 4;
pub const ALPC_PORFLG_OBJECT_TYPE_SEMAPHORE: u32 = 8;
pub const ALPC_PORFLG_OBJECT_TYPE_EVENT: u32 = 16;
pub const ALPC_PORFLG_OBJECT_TYPE_PROCESS: u32 = 32;
pub const ALPC_PORFLG_OBJECT_TYPE_MUTEX: u32 = 64;
pub const ALPC_PORFLG_OBJECT_TYPE_SECTION: u32 = 128;
pub const ALPC_PORFLG_OBJECT_TYPE_REGKEY: u32 = 256;
pub const ALPC_PORFLG_OBJECT_TYPE_TOKEN: u32 = 512;
pub const ALPC_PORFLG_OBJECT_TYPE_COMPOSITION: u32 = 1024;
pub const ALPC_PORFLG_OBJECT_TYPE_JOB: u32 = 2048;
pub const ALPC_PORFLG_OBJECT_TYPE_ALL: u32 = 4093;
pub const ALPC_MESSAGE_SECURITY_ATTRIBUTE: u32 = 2147483648;
pub const ALPC_MESSAGE_VIEW_ATTRIBUTE: u32 = 1073741824;
pub const ALPC_MESSAGE_CONTEXT_ATTRIBUTE: u32 = 536870912;
pub const ALPC_MESSAGE_HANDLE_ATTRIBUTE: u32 = 268435456;
pub const ALPC_COMPLETION_LIST_BUFFER_GRANULARITY_MASK: u32 = 63;
pub const ALPC_HANDLEFLG_DUPLICATE_SAME_ACCESS: u32 = 65536;
pub const ALPC_HANDLEFLG_DUPLICATE_SAME_ATTRIBUTES: u32 = 131072;
pub const ALPC_HANDLEFLG_DUPLICATE_INHERIT: u32 = 524288;
pub const ALPC_SECFLG_CREATE_HANDLE: u32 = 131072;
pub const ALPC_SECFLG_NOSECTIONHANDLE: u32 = 262144;
pub const ALPC_VIEWFLG_NOT_SECURE: u32 = 262144;
pub const ALPC_CREATEPORTSECTIONFLG_SECURE: u32 = 262144;
pub const ALPC_MSGFLG_REPLY_MESSAGE: u32 = 1;
pub const ALPC_MSGFLG_LPC_MODE: u32 = 2;
pub const ALPC_MSGFLG_RELEASE_MESSAGE: u32 = 65536;
pub const ALPC_MSGFLG_SYNC_REQUEST: u32 = 131072;
pub const ALPC_MSGFLG_TRACK_PORT_REFERENCES: u32 = 262144;
pub const ALPC_MSGFLG_WAIT_USER_MODE: u32 = 1048576;
pub const ALPC_MSGFLG_WAIT_ALERTABLE: u32 = 2097152;
pub const ALPC_MSGFLG_WOW64_CALL: u32 = 2147483648;
pub const ALPC_CANCELFLG_TRY_CANCEL: u32 = 1;
pub const ALPC_CANCELFLG_NO_CONTEXT_CHECK: u32 = 8;
pub const ALPC_CANCELFLGP_FLUSH: u32 = 65536;
pub const ALPC_IMPERSONATEFLG_ANONYMOUS: u32 = 1;
pub const ALPC_IMPERSONATEFLG_REQUIRE_IMPERSONATE: u32 = 2;
pub const ALPC_ATTRFLG_ALLOCATEDATTR: u32 = 536870912;
pub const ALPC_ATTRFLG_VALIDATTR: u32 = 1073741824;
pub const ALPC_ATTRFLG_KEEPRUNNINGATTR: u32 = 1610612736;
pub const LPC_KERNELMODE_MESSAGE: u32 = 32768;
pub const LPC_NO_IMPERSONATE: u32 = 16384;
pub const LPC_MAX_CONNECTION_INFO_SIZE: u32 = 128;
pub const PORT_TOTAL_MAXIMUM_MESSAGE_LENGTH: u32 = 688;

#[repr(C)]
pub struct PORT_MESSAGE {
    pub u1: PORT_MESSAGE_1,
    pub u2: PORT_MESSAGE_2,
    pub Anonymous1: PORT_MESSAGE_3,
    pub MessageId: u32,
    pub Anonymous2: PORT_MESSAGE_4,
}

#[repr(C)]
pub struct PORT_MESSAGE_1 {
    pub s1: UnionField<PORT_MESSAGE_1_1>,
    pub Length: UnionField<u32>,
    pub union_field: u32,
}

#[repr(C)]
pub struct PORT_MESSAGE_1_1 {
    pub DataLength: i16,
    pub TotalLength: i16,
}

impl Default for PORT_MESSAGE_1_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PORT_MESSAGE_1_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PORT_MESSAGE_1_1 {{  }}")
    }
}

impl Default for PORT_MESSAGE_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PORT_MESSAGE_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PORT_MESSAGE_1 {{ union }}")
    }
}

#[repr(C)]
pub struct PORT_MESSAGE_2 {
    pub s2: UnionField<PORT_MESSAGE_2_1>,
    pub ZeroInit: UnionField<u32>,
    pub union_field: u32,
}

#[repr(C)]
pub struct PORT_MESSAGE_2_1 {
    pub Type: i16,
    pub DataInfoOffset: i16,
}

impl Default for PORT_MESSAGE_2_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PORT_MESSAGE_2_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PORT_MESSAGE_2_1 {{  }}")
    }
}

impl Default for PORT_MESSAGE_2 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PORT_MESSAGE_2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PORT_MESSAGE_2 {{ union }}")
    }
}

#[repr(C)]
pub struct PORT_MESSAGE_3 {
    pub ClientId: UnionField<CLIENT_ID>,
    pub DoNotUseThisField: UnionField<f64>,
    pub union_field: [u64; 2],
}

impl Default for PORT_MESSAGE_3 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PORT_MESSAGE_3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PORT_MESSAGE_3 {{ union }}")
    }
}

#[repr(C)]
pub struct PORT_MESSAGE_4 {
    pub ClientViewSize: UnionField<usize>,
    pub CallbackId: UnionField<u32>,
    pub union_field: u64,
}

impl Default for PORT_MESSAGE_4 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PORT_MESSAGE_4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PORT_MESSAGE_4 {{ union }}")
    }
}

impl Default for PORT_MESSAGE {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PORT_MESSAGE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PORT_MESSAGE {{ u1: {:?}, u2: {:?}, Anonymous1: {:?}, Anonymous2: {:?} }}",
            self.u1, self.u2, self.Anonymous1, self.Anonymous2
        )
    }
}

#[repr(C)]
pub struct PORT_DATA_ENTRY {
    pub Base: *mut std::ffi::c_void,
    pub Size: u32,
}

impl Default for PORT_DATA_ENTRY {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PORT_DATA_ENTRY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PORT_DATA_ENTRY {{  }}")
    }
}

#[repr(C)]
pub struct PORT_DATA_INFORMATION {
    pub CountDataEntries: u32,
    pub DataEntries: [PORT_DATA_ENTRY; 1],
}

impl Default for PORT_DATA_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PORT_DATA_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PORT_DATA_INFORMATION {{ DataEntries: {:?} }}",
            self.DataEntries
        )
    }
}

#[repr(C)]
pub struct LPC_CLIENT_DIED_MSG {
    pub PortMsg: PORT_MESSAGE,
    pub CreateTime: i64,
}

impl Default for LPC_CLIENT_DIED_MSG {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for LPC_CLIENT_DIED_MSG {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LPC_CLIENT_DIED_MSG {{ PortMsg: {:?} }}", self.PortMsg)
    }
}

#[repr(C)]
pub struct PORT_VIEW {
    pub Length: u32,
    pub SectionHandle: HANDLE,
    pub SectionOffset: u32,
    pub ViewSize: usize,
    pub ViewBase: *mut std::ffi::c_void,
    pub ViewRemoteBase: *mut std::ffi::c_void,
}

impl Default for PORT_VIEW {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PORT_VIEW {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PORT_VIEW {{  }}")
    }
}

#[repr(C)]
pub struct REMOTE_PORT_VIEW {
    pub Length: u32,
    pub ViewSize: usize,
    pub ViewBase: *mut std::ffi::c_void,
}

impl Default for REMOTE_PORT_VIEW {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for REMOTE_PORT_VIEW {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "REMOTE_PORT_VIEW {{  }}")
    }
}

#[repr(C)]
pub struct PORT_MESSAGE64 {
    pub u1: PORT_MESSAGE64_1,
    pub u2: PORT_MESSAGE64_2,
    pub Anonymous1: PORT_MESSAGE64_3,
    pub MessageId: u32,
    pub Anonymous2: PORT_MESSAGE64_4,
}

#[repr(C)]
pub struct PORT_MESSAGE64_1 {
    pub s1: UnionField<PORT_MESSAGE64_1_1>,
    pub Length: UnionField<u32>,
    pub union_field: u32,
}

#[repr(C)]
pub struct PORT_MESSAGE64_1_1 {
    pub DataLength: i16,
    pub TotalLength: i16,
}

impl Default for PORT_MESSAGE64_1_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PORT_MESSAGE64_1_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PORT_MESSAGE64_1_1 {{  }}")
    }
}

impl Default for PORT_MESSAGE64_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PORT_MESSAGE64_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PORT_MESSAGE64_1 {{ union }}")
    }
}

#[repr(C)]
pub struct PORT_MESSAGE64_2 {
    pub s2: UnionField<PORT_MESSAGE64_2_1>,
    pub ZeroInit: UnionField<u32>,
    pub union_field: u32,
}

#[repr(C)]
pub struct PORT_MESSAGE64_2_1 {
    pub Type: i16,
    pub DataInfoOffset: i16,
}

impl Default for PORT_MESSAGE64_2_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PORT_MESSAGE64_2_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PORT_MESSAGE64_2_1 {{  }}")
    }
}

impl Default for PORT_MESSAGE64_2 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PORT_MESSAGE64_2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PORT_MESSAGE64_2 {{ union }}")
    }
}

#[repr(C)]
pub struct PORT_MESSAGE64_3 {
    pub ClientId: UnionField<CLIENT_ID>,
    pub DoNotUseThisField: UnionField<f64>,
    pub union_field: [u64; 2],
}

impl Default for PORT_MESSAGE64_3 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PORT_MESSAGE64_3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PORT_MESSAGE64_3 {{ union }}")
    }
}

#[repr(C)]
pub struct PORT_MESSAGE64_4 {
    pub ClientViewSize: UnionField<u64>,
    pub CallbackId: UnionField<u32>,
    pub union_field: u64,
}

impl Default for PORT_MESSAGE64_4 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PORT_MESSAGE64_4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PORT_MESSAGE64_4 {{ union }}")
    }
}

impl Default for PORT_MESSAGE64 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PORT_MESSAGE64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PORT_MESSAGE64 {{ u1: {:?}, u2: {:?}, Anonymous1: {:?}, Anonymous2: {:?} }}",
            self.u1, self.u2, self.Anonymous1, self.Anonymous2
        )
    }
}

#[repr(C)]
pub struct LPC_CLIENT_DIED_MSG64 {
    pub PortMsg: PORT_MESSAGE64,
    pub CreateTime: i64,
}

impl Default for LPC_CLIENT_DIED_MSG64 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for LPC_CLIENT_DIED_MSG64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LPC_CLIENT_DIED_MSG64 {{ PortMsg: {:?} }}", self.PortMsg)
    }
}

#[repr(C)]
pub struct PORT_VIEW64 {
    pub Length: u32,
    pub SectionHandle: u64,
    pub SectionOffset: u32,
    pub ViewSize: u64,
    pub ViewBase: u64,
    pub ViewRemoteBase: u64,
}

impl Default for PORT_VIEW64 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PORT_VIEW64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PORT_VIEW64 {{  }}")
    }
}

#[repr(C)]
pub struct REMOTE_PORT_VIEW64 {
    pub Length: u32,
    pub ViewSize: u64,
    pub ViewBase: u64,
}

impl Default for REMOTE_PORT_VIEW64 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for REMOTE_PORT_VIEW64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "REMOTE_PORT_VIEW64 {{  }}")
    }
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtCreatePort(
        PortHandle: *mut HANDLE,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        MaxConnectionInfoLength: u32,
        MaxMessageLength: u32,
        MaxPoolUsage: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtCreateWaitablePort(
        PortHandle: *mut HANDLE,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        MaxConnectionInfoLength: u32,
        MaxMessageLength: u32,
        MaxPoolUsage: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtConnectPort(
        PortHandle: *mut HANDLE,
        PortName: *mut UNICODE_STRING,
        SecurityQos: *mut SECURITY_QUALITY_OF_SERVICE,
        ClientView: *mut PORT_VIEW,
        ServerView: *mut REMOTE_PORT_VIEW,
        MaxMessageLength: *mut u32,
        ConnectionInformation: *mut std::ffi::c_void,
        ConnectionInformationLength: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtSecureConnectPort(
        PortHandle: *mut HANDLE,
        PortName: *mut UNICODE_STRING,
        SecurityQos: *mut SECURITY_QUALITY_OF_SERVICE,
        ClientView: *mut PORT_VIEW,
        RequiredServerSid: PSID,
        ServerView: *mut REMOTE_PORT_VIEW,
        MaxMessageLength: *mut u32,
        ConnectionInformation: *mut std::ffi::c_void,
        ConnectionInformationLength: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtListenPort(PortHandle: HANDLE, ConnectionRequest: *mut PORT_MESSAGE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtAcceptConnectPort(
        PortHandle: *mut HANDLE,
        PortContext: *mut std::ffi::c_void,
        ConnectionRequest: *mut PORT_MESSAGE,
        AcceptConnection: bool,
        ServerView: *mut PORT_VIEW,
        ClientView: *mut REMOTE_PORT_VIEW,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtCompleteConnectPort(PortHandle: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtRequestPort(PortHandle: HANDLE, RequestMessage: *mut PORT_MESSAGE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtRequestWaitReplyPort(
        PortHandle: HANDLE,
        RequestMessage: *mut PORT_MESSAGE,
        ReplyMessage: *mut PORT_MESSAGE,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtReplyPort(PortHandle: HANDLE, ReplyMessage: *mut PORT_MESSAGE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtReplyWaitReplyPort(PortHandle: HANDLE, ReplyMessage: *mut PORT_MESSAGE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtReplyWaitReceivePort(
        PortHandle: HANDLE,
        PortContext: *mut *mut std::ffi::c_void,
        ReplyMessage: *mut PORT_MESSAGE,
        ReceiveMessage: *mut PORT_MESSAGE,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtReplyWaitReceivePortEx(
        PortHandle: HANDLE,
        PortContext: *mut *mut std::ffi::c_void,
        ReplyMessage: *mut PORT_MESSAGE,
        ReceiveMessage: *mut PORT_MESSAGE,
        Timeout: *mut i64,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtImpersonateClientOfPort(PortHandle: HANDLE, Message: *mut PORT_MESSAGE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtReadRequestData(
        PortHandle: HANDLE,
        Message: *mut PORT_MESSAGE,
        DataEntryIndex: u32,
        Buffer: *mut std::ffi::c_void,
        BufferSize: usize,
        NumberOfBytesRead: *mut usize,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtWriteRequestData(
        PortHandle: HANDLE,
        Message: *mut PORT_MESSAGE,
        DataEntryIndex: u32,
        Buffer: *mut std::ffi::c_void,
        BufferSize: usize,
        NumberOfBytesWritten: *mut usize,
    ) -> NTSTATUS;
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum PORT_INFORMATION_CLASS {
    PortBasicInformation = 0,
    PortDumpInformation = 1,
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtQueryInformationPort(
        PortHandle: HANDLE,
        PortInformationClass: PORT_INFORMATION_CLASS,
        PortInformation: *mut std::ffi::c_void,
        Length: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}

#[repr(C)]
pub struct ALPC_PORT_ATTRIBUTES {
    pub Flags: u32,
    pub SecurityQos: SECURITY_QUALITY_OF_SERVICE,
    pub MaxMessageLength: usize,
    pub MemoryBandwidth: usize,
    pub MaxPoolUsage: usize,
    pub MaxSectionSize: usize,
    pub MaxViewSize: usize,
    pub MaxTotalSectionSize: usize,
    pub DupObjectTypes: u32,
    pub Reserved: u32,
}

impl Default for ALPC_PORT_ATTRIBUTES {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ALPC_PORT_ATTRIBUTES {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ALPC_PORT_ATTRIBUTES {{  }}")
    }
}

#[repr(C)]
pub struct ALPC_MESSAGE_ATTRIBUTES {
    pub AllocatedAttributes: u32,
    pub ValidAttributes: u32,
}

impl Default for ALPC_MESSAGE_ATTRIBUTES {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ALPC_MESSAGE_ATTRIBUTES {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ALPC_MESSAGE_ATTRIBUTES {{  }}")
    }
}

#[repr(C)]
pub struct ALPC_COMPLETION_LIST_STATE {
    pub u1: ALPC_COMPLETION_LIST_STATE_1,
}

#[repr(C)]
pub struct ALPC_COMPLETION_LIST_STATE_1 {
    pub s1: UnionField<ALPC_COMPLETION_LIST_STATE_1_1>,
    pub Value: UnionField<u64>,
    pub union_field: u64,
}

#[repr(C)]
#[repr(align(8))]
pub struct ALPC_COMPLETION_LIST_STATE_1_1 {
    _bitfield_align_1: [u32; 0],
    _bitfield_1: BitfieldUnit<[u8; 8]>,
}

impl Default for ALPC_COMPLETION_LIST_STATE_1_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ALPC_COMPLETION_LIST_STATE_1_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ALPC_COMPLETION_LIST_STATE_1_1 {{ Head : {:?}, Tail : {:?}, ActiveThreadCount : {:?} }}",
            self.Head(),
            self.Tail(),
            self.ActiveThreadCount()
        )
    }
}

impl ALPC_COMPLETION_LIST_STATE_1_1 {
    #[inline]
    pub fn Head(&self) -> u64 {
        self._bitfield_1.get(0usize, 24u8)
    }

    #[inline]
    pub fn set_Head(&mut self, val: u64) {
        self._bitfield_1.set(0usize, 24u8, val)
    }

    #[inline]
    pub fn Tail(&self) -> u64 {
        self._bitfield_1.get(24usize, 24u8)
    }

    #[inline]
    pub fn set_Tail(&mut self, val: u64) {
        self._bitfield_1.set(24usize, 24u8, val)
    }

    #[inline]
    pub fn ActiveThreadCount(&self) -> u64 {
        self._bitfield_1.get(48usize, 16u8)
    }

    #[inline]
    pub fn set_ActiveThreadCount(&mut self, val: u64) {
        self._bitfield_1.set(48usize, 16u8, val)
    }

    #[inline]
    pub fn new_bitfield_1(Head: u64, Tail: u64, ActiveThreadCount: u64) -> BitfieldUnit<[u8; 8]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 8]> = Default::default();

        bitfield_unit.set(0usize, 24u8, Head);

        bitfield_unit.set(24usize, 24u8, Tail);

        bitfield_unit.set(48usize, 16u8, ActiveThreadCount);

        bitfield_unit
    }
}

impl Default for ALPC_COMPLETION_LIST_STATE_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ALPC_COMPLETION_LIST_STATE_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ALPC_COMPLETION_LIST_STATE_1 {{ union }}")
    }
}

impl Default for ALPC_COMPLETION_LIST_STATE {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ALPC_COMPLETION_LIST_STATE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ALPC_COMPLETION_LIST_STATE {{ u1: {:?} }}", self.u1)
    }
}

#[repr(C)]
#[repr(align(128))]
pub struct ALPC_COMPLETION_LIST_HEADER {
    pub StartMagic: u64,
    pub TotalSize: u32,
    pub ListOffset: u32,
    pub ListSize: u32,
    pub BitmapOffset: u32,
    pub BitmapSize: u32,
    pub DataOffset: u32,
    pub DataSize: u32,
    pub AttributeFlags: u32,
    pub AttributeSize: u32,
    pub padding_0: [u64; 10],
    pub State: ALPC_COMPLETION_LIST_STATE,
    pub LastMessageId: u32,
    pub LastCallbackId: u32,
    pub padding_1: [u32; 28],
    pub PostCount: u32,
    pub padding_2: [u32; 31],
    pub ReturnCount: u32,
    pub padding_3: [u32; 31],
    pub LogSequenceNumber: u32,
    pub padding_4: [u64; 15],
    pub UserLock: SRWLOCK,
    pub EndMagic: u64,
}

impl Default for ALPC_COMPLETION_LIST_HEADER {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ALPC_COMPLETION_LIST_HEADER {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ALPC_COMPLETION_LIST_HEADER {{ State: {:?} }}",
            self.State
        )
    }
}

#[repr(C)]
pub struct ALPC_CONTEXT_ATTR {
    pub PortContext: *mut std::ffi::c_void,
    pub MessageContext: *mut std::ffi::c_void,
    pub Sequence: u32,
    pub MessageId: u32,
    pub CallbackId: u32,
}

impl Default for ALPC_CONTEXT_ATTR {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ALPC_CONTEXT_ATTR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ALPC_CONTEXT_ATTR {{  }}")
    }
}

#[repr(C)]
pub struct ALPC_HANDLE_ATTR32 {
    pub Flags: u32,
    pub Reserved0: u32,
    pub SameAccess: u32,
    pub SameAttributes: u32,
    pub Indirect: u32,
    pub Inherit: u32,
    pub Reserved1: u32,
    pub Handle: u32,
    pub ObjectType: u32,
    pub DesiredAccess: u32,
    pub GrantedAccess: u32,
}

impl Default for ALPC_HANDLE_ATTR32 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ALPC_HANDLE_ATTR32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ALPC_HANDLE_ATTR32 {{  }}")
    }
}

#[repr(C)]
pub struct ALPC_HANDLE_ATTR {
    pub Flags: u32,
    pub Reserved0: u32,
    pub SameAccess: u32,
    pub SameAttributes: u32,
    pub Indirect: u32,
    pub Inherit: u32,
    pub Reserved1: u32,
    pub Handle: HANDLE,
    pub HandleAttrArray: *mut ALPC_HANDLE_ATTR32,
    pub ObjectType: u32,
    pub HandleCount: u32,
    pub DesiredAccess: u32,
    pub GrantedAccess: u32,
}

impl Default for ALPC_HANDLE_ATTR {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ALPC_HANDLE_ATTR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ALPC_HANDLE_ATTR {{ HandleAttrArray: {:?} }}",
            self.HandleAttrArray
        )
    }
}

#[repr(C)]
pub struct ALPC_SECURITY_ATTR {
    pub Flags: u32,
    pub QoS: *mut SECURITY_QUALITY_OF_SERVICE,
    pub ContextHandle: HANDLE,
}

impl Default for ALPC_SECURITY_ATTR {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ALPC_SECURITY_ATTR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ALPC_SECURITY_ATTR {{  }}")
    }
}

#[repr(C)]
pub struct ALPC_DATA_VIEW_ATTR {
    pub Flags: u32,
    pub SectionHandle: HANDLE,
    pub ViewBase: *mut std::ffi::c_void,
    pub ViewSize: usize,
}

impl Default for ALPC_DATA_VIEW_ATTR {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ALPC_DATA_VIEW_ATTR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ALPC_DATA_VIEW_ATTR {{  }}")
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ALPC_PORT_INFORMATION_CLASS {
    AlpcBasicInformation = 0,
    AlpcPortInformation = 1,
    AlpcAssociateCompletionPortInformation = 2,
    AlpcConnectedSIDInformation = 3,
    AlpcServerInformation = 4,
    AlpcMessageZoneInformation = 5,
    AlpcRegisterCompletionListInformation = 6,
    AlpcUnregisterCompletionListInformation = 7,
    AlpcAdjustCompletionListConcurrencyCountInformation = 8,
    AlpcRegisterCallbackInformation = 9,
    AlpcCompletionListRundownInformation = 10,
    AlpcWaitForPortReferences = 11,
    AlpcServerSessionInformation = 12,
}

#[repr(C)]
pub struct ALPC_BASIC_INFORMATION {
    pub Flags: u32,
    pub SequenceNo: u32,
    pub PortContext: *mut std::ffi::c_void,
}

impl Default for ALPC_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ALPC_BASIC_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ALPC_BASIC_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct ALPC_PORT_ASSOCIATE_COMPLETION_PORT {
    pub CompletionKey: *mut std::ffi::c_void,
    pub CompletionPort: HANDLE,
}

impl Default for ALPC_PORT_ASSOCIATE_COMPLETION_PORT {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ALPC_PORT_ASSOCIATE_COMPLETION_PORT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ALPC_PORT_ASSOCIATE_COMPLETION_PORT {{  }}")
    }
}

#[repr(C)]
pub struct ALPC_SERVER_INFORMATION {
    pub Anonymous1: ALPC_SERVER_INFORMATION_1,
}

#[repr(C)]
pub struct ALPC_SERVER_INFORMATION_1 {
    pub In: UnionField<ALPC_SERVER_INFORMATION_1_1>,
    pub Out: UnionField<ALPC_SERVER_INFORMATION_1_2>,
    pub union_field: [u64; 4],
}

#[repr(C)]
pub struct ALPC_SERVER_INFORMATION_1_1 {
    pub ThreadHandle: HANDLE,
}

impl Default for ALPC_SERVER_INFORMATION_1_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ALPC_SERVER_INFORMATION_1_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ALPC_SERVER_INFORMATION_1_1 {{  }}")
    }
}

#[repr(C)]
pub struct ALPC_SERVER_INFORMATION_1_2 {
    pub ThreadBlocked: bool,
    pub ConnectedProcessId: HANDLE,
    pub ConnectionPortName: UNICODE_STRING,
}

impl Default for ALPC_SERVER_INFORMATION_1_2 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ALPC_SERVER_INFORMATION_1_2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ALPC_SERVER_INFORMATION_1_2 {{  }}")
    }
}

impl Default for ALPC_SERVER_INFORMATION_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ALPC_SERVER_INFORMATION_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ALPC_SERVER_INFORMATION_1 {{ union }}")
    }
}

impl Default for ALPC_SERVER_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ALPC_SERVER_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ALPC_SERVER_INFORMATION {{ Anonymous1: {:?} }}",
            self.Anonymous1
        )
    }
}

#[repr(C)]
pub struct ALPC_PORT_MESSAGE_ZONE_INFORMATION {
    pub Buffer: *mut std::ffi::c_void,
    pub Size: u32,
}

impl Default for ALPC_PORT_MESSAGE_ZONE_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ALPC_PORT_MESSAGE_ZONE_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ALPC_PORT_MESSAGE_ZONE_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct ALPC_PORT_COMPLETION_LIST_INFORMATION {
    pub Buffer: *mut std::ffi::c_void,
    pub Size: u32,
    pub ConcurrencyCount: u32,
    pub AttributeFlags: u32,
}

impl Default for ALPC_PORT_COMPLETION_LIST_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ALPC_PORT_COMPLETION_LIST_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ALPC_PORT_COMPLETION_LIST_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct ALPC_REGISTER_CALLBACK {
    pub CallbackObject: *mut std::ffi::c_void,
    pub CallbackContext: *mut std::ffi::c_void,
}

impl Default for ALPC_REGISTER_CALLBACK {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ALPC_REGISTER_CALLBACK {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ALPC_REGISTER_CALLBACK {{  }}")
    }
}

#[repr(C)]
pub struct ALPC_SERVER_SESSION_INFORMATION {
    pub SessionId: u32,
    pub ProcessId: u32,
}

impl Default for ALPC_SERVER_SESSION_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ALPC_SERVER_SESSION_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ALPC_SERVER_SESSION_INFORMATION {{  }}")
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ALPC_MESSAGE_INFORMATION_CLASS {
    AlpcMessageSidInformation = 0,
    AlpcMessageTokenModifiedIdInformation = 1,
    AlpcMessageDirectStatusInformation = 2,
    AlpcMessageHandleInformation = 3,
    MaxAlpcMessageInfoClass = 4,
}

#[repr(C)]
pub struct ALPC_MESSAGE_HANDLE_INFORMATION {
    pub Index: u32,
    pub Flags: u32,
    pub Handle: u32,
    pub ObjectType: u32,
    pub GrantedAccess: u32,
}

impl Default for ALPC_MESSAGE_HANDLE_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ALPC_MESSAGE_HANDLE_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ALPC_MESSAGE_HANDLE_INFORMATION {{  }}")
    }
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtAlpcCreatePort(
        PortHandle: *mut HANDLE,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        PortAttributes: *mut ALPC_PORT_ATTRIBUTES,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtAlpcDisconnectPort(PortHandle: HANDLE, Flags: u32) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtAlpcQueryInformation(
        PortHandle: HANDLE,
        PortInformationClass: ALPC_PORT_INFORMATION_CLASS,
        PortInformation: *mut std::ffi::c_void,
        Length: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtAlpcSetInformation(
        PortHandle: HANDLE,
        PortInformationClass: ALPC_PORT_INFORMATION_CLASS,
        PortInformation: *mut std::ffi::c_void,
        Length: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtAlpcCreatePortSection(
        PortHandle: HANDLE,
        Flags: u32,
        SectionHandle: HANDLE,
        SectionSize: usize,
        AlpcSectionHandle: *mut HANDLE,
        ActualSectionSize: *mut usize,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtAlpcDeletePortSection(
        PortHandle: HANDLE,
        Flags: u32,
        SectionHandle: HANDLE,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtAlpcCreateResourceReserve(
        PortHandle: HANDLE,
        Flags: u32,
        MessageSize: usize,
        ResourceId: *mut HANDLE,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtAlpcDeleteResourceReserve(
        PortHandle: HANDLE,
        Flags: u32,
        ResourceId: HANDLE,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtAlpcCreateSectionView(
        PortHandle: HANDLE,
        Flags: u32,
        ViewAttributes: *mut ALPC_DATA_VIEW_ATTR,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtAlpcDeleteSectionView(
        PortHandle: HANDLE,
        Flags: u32,
        ViewBase: *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtAlpcCreateSecurityContext(
        PortHandle: HANDLE,
        Flags: u32,
        SecurityAttribute: *mut ALPC_SECURITY_ATTR,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtAlpcDeleteSecurityContext(
        PortHandle: HANDLE,
        Flags: u32,
        ContextHandle: HANDLE,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtAlpcRevokeSecurityContext(
        PortHandle: HANDLE,
        Flags: u32,
        ContextHandle: HANDLE,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtAlpcQueryInformationMessage(
        PortHandle: HANDLE,
        PortMessage: *mut PORT_MESSAGE,
        MessageInformationClass: ALPC_MESSAGE_INFORMATION_CLASS,
        MessageInformation: *mut std::ffi::c_void,
        Length: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtAlpcConnectPort(
        PortHandle: *mut HANDLE,
        PortName: *mut UNICODE_STRING,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        PortAttributes: *mut ALPC_PORT_ATTRIBUTES,
        Flags: u32,
        RequiredServerSid: PSID,
        ConnectionMessage: *mut PORT_MESSAGE,
        BufferLength: *mut u32,
        OutMessageAttributes: *mut ALPC_MESSAGE_ATTRIBUTES,
        InMessageAttributes: *mut ALPC_MESSAGE_ATTRIBUTES,
        Timeout: *mut i64,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtAlpcConnectPortEx(
        PortHandle: *mut HANDLE,
        ConnectionPortObjectAttributes: *mut OBJECT_ATTRIBUTES,
        ClientPortObjectAttributes: *mut OBJECT_ATTRIBUTES,
        PortAttributes: *mut ALPC_PORT_ATTRIBUTES,
        Flags: u32,
        ServerSecurityRequirements: *mut SECURITY_DESCRIPTOR,
        ConnectionMessage: *mut PORT_MESSAGE,
        BufferLength: *mut usize,
        OutMessageAttributes: *mut ALPC_MESSAGE_ATTRIBUTES,
        InMessageAttributes: *mut ALPC_MESSAGE_ATTRIBUTES,
        Timeout: *mut i64,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtAlpcAcceptConnectPort(
        PortHandle: *mut HANDLE,
        ConnectionPortHandle: HANDLE,
        Flags: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        PortAttributes: *mut ALPC_PORT_ATTRIBUTES,
        PortContext: *mut std::ffi::c_void,
        ConnectionRequest: *mut PORT_MESSAGE,
        ConnectionMessageAttributes: *mut ALPC_MESSAGE_ATTRIBUTES,
        AcceptConnection: bool,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtAlpcSendWaitReceivePort(
        PortHandle: HANDLE,
        Flags: u32,
        SendMessageA: *mut PORT_MESSAGE,
        SendMessageAttributes: *mut ALPC_MESSAGE_ATTRIBUTES,
        ReceiveMessage: *mut PORT_MESSAGE,
        BufferLength: *mut usize,
        ReceiveMessageAttributes: *mut ALPC_MESSAGE_ATTRIBUTES,
        Timeout: *mut i64,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtAlpcCancelMessage(
        PortHandle: HANDLE,
        Flags: u32,
        MessageContext: *mut ALPC_CONTEXT_ATTR,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtAlpcImpersonateClientOfPort(
        PortHandle: HANDLE,
        Message: *mut PORT_MESSAGE,
        Flags: *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtAlpcImpersonateClientContainerOfPort(
        PortHandle: HANDLE,
        Message: *mut PORT_MESSAGE,
        Flags: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtAlpcOpenSenderProcess(
        ProcessHandle: *mut HANDLE,
        PortHandle: HANDLE,
        PortMessage: *mut PORT_MESSAGE,
        Flags: u32,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtAlpcOpenSenderThread(
        ThreadHandle: *mut HANDLE,
        PortHandle: HANDLE,
        PortMessage: *mut PORT_MESSAGE,
        Flags: u32,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn AlpcMaxAllowedMessageLength() -> u32;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn AlpcGetHeaderSize(Flags: u32) -> u32;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn AlpcInitializeMessageAttribute(
        AttributeFlags: u32,
        Buffer: *mut ALPC_MESSAGE_ATTRIBUTES,
        BufferSize: u32,
        RequiredBufferSize: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn AlpcGetMessageAttribute(
        Buffer: *mut ALPC_MESSAGE_ATTRIBUTES,
        AttributeFlag: u32,
    ) -> *mut std::ffi::c_void;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn AlpcRegisterCompletionList(
        PortHandle: HANDLE,
        Buffer: *mut ALPC_COMPLETION_LIST_HEADER,
        Size: u32,
        ConcurrencyCount: u32,
        AttributeFlags: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn AlpcUnregisterCompletionList(PortHandle: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn AlpcRundownCompletionList(PortHandle: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn AlpcAdjustCompletionListConcurrencyCount(
        PortHandle: HANDLE,
        ConcurrencyCount: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn AlpcRegisterCompletionListWorkerThread(CompletionList: *mut std::ffi::c_void)
    -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn AlpcUnregisterCompletionListWorkerThread(
        CompletionList: *mut std::ffi::c_void,
    ) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn AlpcGetCompletionListLastMessageInformation(
        CompletionList: *mut std::ffi::c_void,
        LastMessageId: *mut u32,
        LastCallbackId: *mut u32,
    );
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn AlpcGetOutstandingCompletionListMessageCount(
        CompletionList: *mut std::ffi::c_void,
    ) -> u32;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn AlpcGetMessageFromCompletionList(
        CompletionList: *mut std::ffi::c_void,
        MessageAttributes: *mut *mut ALPC_MESSAGE_ATTRIBUTES,
    ) -> *mut PORT_MESSAGE;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn AlpcFreeCompletionListMessage(
        CompletionList: *mut std::ffi::c_void,
        Message: *mut PORT_MESSAGE,
    );
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn AlpcGetCompletionListMessageAttributes(
        CompletionList: *mut std::ffi::c_void,
        Message: *mut PORT_MESSAGE,
    ) -> *mut ALPC_MESSAGE_ATTRIBUTES;
}
