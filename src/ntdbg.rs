use windows::{
    core::GUID,
    Wdk::Foundation::OBJECT_ATTRIBUTES,
    Win32::{
        Foundation::{HANDLE, NTSTATUS},
        System::{
            Diagnostics::{
                Debug::{DEBUG_EVENT, EXCEPTION_RECORD},
                Etw::PENABLECALLBACK,
            },
            WindowsProgramming::CLIENT_ID,
        },
    },
};

use crate::{bitfield::UnionField, phnt_ntdef::PREGHANDLE};

pub const DEBUG_READ_EVENT: u32 = 1;
pub const DEBUG_PROCESS_ASSIGN: u32 = 2;
pub const DEBUG_SET_INFORMATION: u32 = 4;
pub const DEBUG_QUERY_INFORMATION: u32 = 8;
pub const DEBUG_ALL_ACCESS: u32 = 2031631;
pub const DEBUG_KILL_ON_CLOSE: u32 = 1;

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn DbgUserBreakPoint();
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn DbgBreakPoint();
}

#[repr(C)]
pub struct DBGKM_EXCEPTION {
    pub ExceptionRecord: EXCEPTION_RECORD,
    pub FirstChance: u32,
}

impl Default for DBGKM_EXCEPTION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for DBGKM_EXCEPTION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DBGKM_EXCEPTION {{  }}")
    }
}

#[repr(C)]
pub struct DBGKM_CREATE_THREAD {
    pub SubSystemKey: u32,
    pub StartAddress: *mut std::ffi::c_void,
}

impl Default for DBGKM_CREATE_THREAD {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for DBGKM_CREATE_THREAD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DBGKM_CREATE_THREAD {{  }}")
    }
}

#[repr(C)]
pub struct DBGKM_CREATE_PROCESS {
    pub SubSystemKey: u32,
    pub FileHandle: HANDLE,
    pub BaseOfImage: *mut std::ffi::c_void,
    pub DebugInfoFileOffset: u32,
    pub DebugInfoSize: u32,
    pub InitialThread: DBGKM_CREATE_THREAD,
}

impl Default for DBGKM_CREATE_PROCESS {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for DBGKM_CREATE_PROCESS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DBGKM_CREATE_PROCESS {{ InitialThread: {:?} }}",
            self.InitialThread
        )
    }
}

#[repr(C)]
pub struct DBGKM_EXIT_THREAD {
    pub ExitStatus: NTSTATUS,
}

impl Default for DBGKM_EXIT_THREAD {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for DBGKM_EXIT_THREAD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DBGKM_EXIT_THREAD {{  }}")
    }
}

#[repr(C)]
pub struct DBGKM_EXIT_PROCESS {
    pub ExitStatus: NTSTATUS,
}

impl Default for DBGKM_EXIT_PROCESS {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for DBGKM_EXIT_PROCESS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DBGKM_EXIT_PROCESS {{  }}")
    }
}

#[repr(C)]
pub struct DBGKM_LOAD_DLL {
    pub FileHandle: HANDLE,
    pub BaseOfDll: *mut std::ffi::c_void,
    pub DebugInfoFileOffset: u32,
    pub DebugInfoSize: u32,
    pub NamePointer: *mut std::ffi::c_void,
}

impl Default for DBGKM_LOAD_DLL {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for DBGKM_LOAD_DLL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DBGKM_LOAD_DLL {{  }}")
    }
}

#[repr(C)]
pub struct DBGKM_UNLOAD_DLL {
    pub BaseAddress: *mut std::ffi::c_void,
}

impl Default for DBGKM_UNLOAD_DLL {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for DBGKM_UNLOAD_DLL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DBGKM_UNLOAD_DLL {{  }}")
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum DBG_STATE {
    DbgIdle = 0,
    DbgReplyPending = 1,
    DbgCreateThreadStateChange = 2,
    DbgCreateProcessStateChange = 3,
    DbgExitThreadStateChange = 4,
    DbgExitProcessStateChange = 5,
    DbgExceptionStateChange = 6,
    DbgBreakpointStateChange = 7,
    DbgSingleStepStateChange = 8,
    DbgLoadDllStateChange = 9,
    DbgUnloadDllStateChange = 10,
}

#[repr(C)]
pub struct DBGUI_CREATE_THREAD {
    pub HandleToThread: HANDLE,
    pub NewThread: DBGKM_CREATE_THREAD,
}

impl Default for DBGUI_CREATE_THREAD {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for DBGUI_CREATE_THREAD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DBGUI_CREATE_THREAD {{ NewThread: {:?} }}",
            self.NewThread
        )
    }
}

#[repr(C)]
pub struct DBGUI_CREATE_PROCESS {
    pub HandleToProcess: HANDLE,
    pub HandleToThread: HANDLE,
    pub NewProcess: DBGKM_CREATE_PROCESS,
}

impl Default for DBGUI_CREATE_PROCESS {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for DBGUI_CREATE_PROCESS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DBGUI_CREATE_PROCESS {{ NewProcess: {:?} }}",
            self.NewProcess
        )
    }
}

#[repr(C)]
pub struct DBGUI_WAIT_STATE_CHANGE {
    pub NewState: DBG_STATE,
    pub AppClientId: CLIENT_ID,
    pub StateInfo: DBGUI_WAIT_STATE_CHANGE_1,
}

#[repr(C)]
pub struct DBGUI_WAIT_STATE_CHANGE_1 {
    pub Exception: UnionField<DBGKM_EXCEPTION>,
    pub CreateThread: UnionField<DBGUI_CREATE_THREAD>,
    pub CreateProcessInfo: UnionField<DBGUI_CREATE_PROCESS>,
    pub ExitThread: UnionField<DBGKM_EXIT_THREAD>,
    pub ExitProcess: UnionField<DBGKM_EXIT_PROCESS>,
    pub LoadDll: UnionField<DBGKM_LOAD_DLL>,
    pub UnloadDll: UnionField<DBGKM_UNLOAD_DLL>,
    pub union_field: [u64; 20],
}

impl Default for DBGUI_WAIT_STATE_CHANGE_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for DBGUI_WAIT_STATE_CHANGE_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DBGUI_WAIT_STATE_CHANGE_1 {{ union }}")
    }
}

impl Default for DBGUI_WAIT_STATE_CHANGE {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for DBGUI_WAIT_STATE_CHANGE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DBGUI_WAIT_STATE_CHANGE {{ NewState: {:?}, StateInfo: {:?} }}",
            self.NewState, self.StateInfo
        )
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum DEBUGOBJECTINFOCLASS {
    DebugObjectUnusedInformation = 0,
    DebugObjectKillProcessOnExitInformation = 1,
    MaxDebugObjectInfoClass = 2,
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtCreateDebugObject(
        DebugObjectHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        Flags: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtDebugActiveProcess(ProcessHandle: HANDLE, DebugObjectHandle: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtDebugContinue(
        DebugObjectHandle: HANDLE,
        ClientId: *mut CLIENT_ID,
        ContinueStatus: NTSTATUS,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtRemoveProcessDebug(ProcessHandle: HANDLE, DebugObjectHandle: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtSetInformationDebugObject(
        DebugObjectHandle: HANDLE,
        DebugObjectInformationClass: DEBUGOBJECTINFOCLASS,
        DebugInformation: *mut std::ffi::c_void,
        DebugInformationLength: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtWaitForDebugEvent(
        DebugObjectHandle: HANDLE,
        Alertable: bool,
        Timeout: *mut i64,
        WaitStateChange: *mut DBGUI_WAIT_STATE_CHANGE,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn DbgUiConnectToDbg() -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn DbgUiGetThreadDebugObject() -> HANDLE;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn DbgUiSetThreadDebugObject(DebugObject: HANDLE);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn DbgUiWaitStateChange(
        StateChange: *mut DBGUI_WAIT_STATE_CHANGE,
        Timeout: *mut i64,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn DbgUiContinue(AppClientId: *mut CLIENT_ID, ContinueStatus: NTSTATUS) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn DbgUiStopDebugging(Process: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn DbgUiDebugActiveProcess(Process: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn DbgUiRemoteBreakin(Context: *mut std::ffi::c_void);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn DbgUiIssueRemoteBreakin(Process: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn DbgUiConvertStateChangeStructure(
        StateChange: *mut DBGUI_WAIT_STATE_CHANGE,
        DebugEvent: *mut DEBUG_EVENT,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn DbgUiConvertStateChangeStructureEx(
        StateChange: *mut DBGUI_WAIT_STATE_CHANGE,
        DebugEvent: *mut DEBUG_EVENT,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn EtwEventRegister(
        ProviderId: *const GUID,
        EnableCallback: PENABLECALLBACK,
        CallbackContext: *mut std::ffi::c_void,
        RegHandle: PREGHANDLE,
    ) -> NTSTATUS;
}
