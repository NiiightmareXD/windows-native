use windows::Win32::Foundation::{BOOLEAN, NTSTATUS};

pub const LOW_PRIORITY: u32 = 0;
pub const LOW_REALTIME_PRIORITY: u32 = 16;
pub const HIGH_PRIORITY: u32 = 31;
pub const MAXIMUM_PRIORITY: u32 = 32;
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
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum KWAIT_REASON {
    Executive = 0,
    FreePage = 1,
    PageIn = 2,
    PoolAllocation = 3,
    DelayExecution = 4,
    Suspended = 5,
    UserRequest = 6,
    WrExecutive = 7,
    WrFreePage = 8,
    WrPageIn = 9,
    WrPoolAllocation = 10,
    WrDelayExecution = 11,
    WrSuspended = 12,
    WrUserRequest = 13,
    WrEventPair = 14,
    WrQueue = 15,
    WrLpcReceive = 16,
    WrLpcReply = 17,
    WrVirtualMemory = 18,
    WrPageOut = 19,
    WrRendezvous = 20,
    WrKeyedEvent = 21,
    WrTerminated = 22,
    WrProcessInSwap = 23,
    WrCpuRateControl = 24,
    WrCalloutStack = 25,
    WrKernel = 26,
    WrResource = 27,
    WrPushLock = 28,
    WrMutex = 29,
    WrQuantumEnd = 30,
    WrDispatchInt = 31,
    WrPreempted = 32,
    WrYieldExecution = 33,
    WrFastMutex = 34,
    WrGuardedMutex = 35,
    WrRundown = 36,
    WrAlertByThreadId = 37,
    WrDeferredPreempt = 38,
    WrPhysicalFault = 39,
    WrIoRing = 40,
    WrMdlCache = 41,
    MaximumWaitReason = 42,
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum KPROFILE_SOURCE {
    ProfileTime = 0,
    ProfileAlignmentFixup = 1,
    ProfileTotalIssues = 2,
    ProfilePipelineDry = 3,
    ProfileLoadInstructions = 4,
    ProfilePipelineFrozen = 5,
    ProfileBranchInstructions = 6,
    ProfileTotalNonissues = 7,
    ProfileDcacheMisses = 8,
    ProfileIcacheMisses = 9,
    ProfileCacheMisses = 10,
    ProfileBranchMispredictions = 11,
    ProfileStoreInstructions = 12,
    ProfileFpInstructions = 13,
    ProfileIntegerInstructions = 14,
    Profile2Issue = 15,
    Profile3Issue = 16,
    Profile4Issue = 17,
    ProfileSpecialInstructions = 18,
    ProfileTotalCycles = 19,
    ProfileIcacheIssues = 20,
    ProfileDcacheAccesses = 21,
    ProfileMemoryBarrierCycles = 22,
    ProfileLoadLinkedIssues = 23,
    ProfileMaximum = 24,
}
extern "C" {
    pub fn NtCallbackReturn(
        OutputBuffer: *mut std::ffi::c_void,
        OutputLength: u32,
        Status: NTSTATUS,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn NtFlushProcessWriteBuffers() -> NTSTATUS;
}
extern "C" {
    pub fn NtQueryDebugFilterState(ComponentId: u32, Level: u32) -> NTSTATUS;
}
extern "C" {
    pub fn NtSetDebugFilterState(ComponentId: u32, Level: u32, State: BOOLEAN) -> NTSTATUS;
}
extern "C" {
    pub fn NtYieldExecution() -> NTSTATUS;
}
