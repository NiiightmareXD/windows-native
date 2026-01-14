use windows::Win32::Foundation::{NTSTATUS};

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum KTHREAD_STATE {
    Initialized = 0,
    Ready = 1,
    Running = 2,
    Standby = 3,
    Terminated = 4,
    Waiting = 5,
    Transition = 6,
    DeferredReady = 7,
    GateWaitObsolete = 8,
    WaitingForProcessInSwap = 9,
    MaximumThreadState = 10,
}

impl KHETERO_CPU_POLICY {}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum KHETERO_CPU_POLICY {
    KHeteroCpuPolicyAll = 0,
    KHeteroCpuPolicyLarge = 1,
    KHeteroCpuPolicyLargeOrIdle = 2,
    KHeteroCpuPolicySmall = 3,
    KHeteroCpuPolicySmallOrIdle = 4,
    KHeteroCpuPolicyDynamic = 5,
    KHeteroCpuPolicyBiasedSmall = 6,
    KHeteroCpuPolicyBiasedLarge = 7,
    KHeteroCpuPolicyDefault = 8,
    KHeteroCpuPolicyMax = 9,
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtCallbackReturn(
        OutputBuffer: *mut std::ffi::c_void,
        OutputLength: u32,
        Status: NTSTATUS,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtFlushProcessWriteBuffers() -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtQueryDebugFilterState(ComponentId: u32, Level: u32) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtSetDebugFilterState(ComponentId: u32, Level: u32, State: bool) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtYieldExecution() -> NTSTATUS;
}
