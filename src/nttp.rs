use windows::Win32::{
    Foundation::{HANDLE, NTSTATUS},
    System::{
        Threading::{
            CRITICAL_SECTION, PTP_CALLBACK_INSTANCE, PTP_CLEANUP_GROUP, PTP_IO, PTP_POOL,
            PTP_SIMPLE_CALLBACK, PTP_TIMER, PTP_TIMER_CALLBACK, PTP_WAIT, PTP_WAIT_CALLBACK,
            PTP_WORK, PTP_WORK_CALLBACK, TP_CALLBACK_ENVIRON_V3, TP_POOL_STACK_INFORMATION,
        },
        IO::IO_STATUS_BLOCK,
    },
};

pub type PTP_ALPC_CALLBACK = std::option::Option<
    unsafe extern "system" fn(
        Instance: PTP_CALLBACK_INSTANCE,
        Context: *mut std::ffi::c_void,
        Alpc: *mut std::ffi::c_void,
    ),
>;

pub type PTP_ALPC_CALLBACK_EX = std::option::Option<
    unsafe extern "system" fn(
        Instance: PTP_CALLBACK_INSTANCE,
        Context: *mut std::ffi::c_void,
        Alpc: *mut std::ffi::c_void,
        ApcContext: *mut std::ffi::c_void,
    ),
>;

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn TpAllocPool(PoolReturn: *mut PTP_POOL, Reserved: *mut std::ffi::c_void) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn TpReleasePool(Pool: PTP_POOL);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn TpSetPoolMaxThreads(Pool: PTP_POOL, MaxThreads: u32);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn TpSetPoolMinThreads(Pool: PTP_POOL, MinThreads: u32) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn TpQueryPoolStackInformation(
        Pool: PTP_POOL,
        PoolStackInformation: *mut TP_POOL_STACK_INFORMATION,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn TpSetPoolStackInformation(
        Pool: PTP_POOL,
        PoolStackInformation: *mut TP_POOL_STACK_INFORMATION,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn TpSetPoolThreadBasePriority(Pool: PTP_POOL, BasePriority: u32) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn TpAllocCleanupGroup(CleanupGroupReturn: *mut PTP_CLEANUP_GROUP) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn TpReleaseCleanupGroup(CleanupGroup: PTP_CLEANUP_GROUP);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn TpReleaseCleanupGroupMembers(
        CleanupGroup: PTP_CLEANUP_GROUP,
        CancelPendingCallbacks: u32,
        CleanupParameter: *mut std::ffi::c_void,
    );
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn TpCallbackSetEventOnCompletion(Instance: PTP_CALLBACK_INSTANCE, Event: HANDLE);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn TpCallbackReleaseSemaphoreOnCompletion(
        Instance: PTP_CALLBACK_INSTANCE,
        Semaphore: HANDLE,
        ReleaseCount: u32,
    );
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn TpCallbackReleaseMutexOnCompletion(Instance: PTP_CALLBACK_INSTANCE, Mutex: HANDLE);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn TpCallbackLeaveCriticalSectionOnCompletion(
        Instance: PTP_CALLBACK_INSTANCE,
        CriticalSection: *mut CRITICAL_SECTION,
    );
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn TpCallbackUnloadDllOnCompletion(
        Instance: PTP_CALLBACK_INSTANCE,
        DllHandle: *mut std::ffi::c_void,
    );
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn TpCallbackMayRunLong(Instance: PTP_CALLBACK_INSTANCE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn TpDisassociateCallback(Instance: PTP_CALLBACK_INSTANCE);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn TpSimpleTryPost(
        Callback: PTP_SIMPLE_CALLBACK,
        Context: *mut std::ffi::c_void,
        CallbackEnviron: *mut TP_CALLBACK_ENVIRON_V3,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn TpAllocWork(
        WorkReturn: *mut PTP_WORK,
        Callback: PTP_WORK_CALLBACK,
        Context: *mut std::ffi::c_void,
        CallbackEnviron: *mut TP_CALLBACK_ENVIRON_V3,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn TpReleaseWork(Work: PTP_WORK);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn TpPostWork(Work: PTP_WORK);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn TpWaitForWork(Work: PTP_WORK, CancelPendingCallbacks: u32);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn TpAllocTimer(
        Timer: *mut PTP_TIMER,
        Callback: PTP_TIMER_CALLBACK,
        Context: *mut std::ffi::c_void,
        CallbackEnviron: *mut TP_CALLBACK_ENVIRON_V3,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn TpReleaseTimer(Timer: PTP_TIMER);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn TpSetTimer(Timer: PTP_TIMER, DueTime: *mut i64, Period: u32, WindowLength: u32);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn TpSetTimerEx(
        Timer: PTP_TIMER,
        DueTime: *mut i64,
        Period: u32,
        WindowLength: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn TpIsTimerSet(Timer: PTP_TIMER) -> u32;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn TpWaitForTimer(Timer: PTP_TIMER, CancelPendingCallbacks: u32);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn TpAllocWait(
        WaitReturn: *mut PTP_WAIT,
        Callback: PTP_WAIT_CALLBACK,
        Context: *mut std::ffi::c_void,
        CallbackEnviron: *mut TP_CALLBACK_ENVIRON_V3,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn TpReleaseWait(Wait: PTP_WAIT);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn TpSetWait(Wait: PTP_WAIT, Handle: HANDLE, Timeout: *mut i64);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn TpSetWaitEx(
        Wait: PTP_WAIT,
        Handle: HANDLE,
        Timeout: *mut i64,
        Reserved: *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn TpWaitForWait(Wait: PTP_WAIT, CancelPendingCallbacks: u32);
}

pub type PTP_IO_CALLBACK = std::option::Option<
    unsafe extern "system" fn(
        Instance: PTP_CALLBACK_INSTANCE,
        Context: *mut std::ffi::c_void,
        ApcContext: *mut std::ffi::c_void,
        IoSB: *mut IO_STATUS_BLOCK,
        Io: PTP_IO,
    ),
>;

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn TpAllocIoCompletion(
        IoReturn: *mut PTP_IO,
        File: HANDLE,
        Callback: PTP_IO_CALLBACK,
        Context: *mut std::ffi::c_void,
        CallbackEnviron: *mut TP_CALLBACK_ENVIRON_V3,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn TpReleaseIoCompletion(Io: PTP_IO);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn TpStartAsyncIoOperation(Io: PTP_IO);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn TpCancelAsyncIoOperation(Io: PTP_IO);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn TpWaitForIoCompletion(Io: PTP_IO, CancelPendingCallbacks: u32);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn TpAllocAlpcCompletion(
        AlpcReturn: *mut *mut std::ffi::c_void,
        AlpcPort: HANDLE,
        Callback: PTP_ALPC_CALLBACK,
        Context: *mut std::ffi::c_void,
        CallbackEnviron: *mut TP_CALLBACK_ENVIRON_V3,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn TpAllocAlpcCompletionEx(
        AlpcReturn: *mut *mut std::ffi::c_void,
        AlpcPort: HANDLE,
        Callback: PTP_ALPC_CALLBACK_EX,
        Context: *mut std::ffi::c_void,
        CallbackEnviron: *mut TP_CALLBACK_ENVIRON_V3,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn TpReleaseAlpcCompletion(Alpc: *mut std::ffi::c_void);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn TpWaitForAlpcCompletion(Alpc: *mut std::ffi::c_void);
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum TP_TRACE_TYPE {
    TpTraceThreadPriority = 1,
    TpTraceThreadAffinity = 2,
    MaxTpTraceType = 3,
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn TpCaptureCaller(Type: TP_TRACE_TYPE);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn TpCheckTerminateWorker(Thread: HANDLE);
}
