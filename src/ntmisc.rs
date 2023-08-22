use windows::Win32::Foundation::{HANDLE, NTSTATUS};

pub const FLT_PORT_ALL_ACCESS: u32 = 2031617;

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum VDMSERVICECLASS {
    VdmStartExecution = 0,
    VdmQueueInterrupt = 1,
    VdmDelayInterrupt = 2,
    VdmInitialize = 3,
    VdmFeatures = 4,
    VdmSetInt21Handler = 5,
    VdmQueryDir = 6,
    VdmPrinterDirectIoOpen = 7,
    VdmPrinterDirectIoClose = 8,
    VdmPrinterInitialize = 9,
    VdmSetLdtEntries = 10,
    VdmSetProcessLdtInfo = 11,
    VdmAdlibEmulation = 12,
    VdmPMCliControl = 13,
    VdmQueryVdmProcess = 14,
    VdmPreInitialize = 15,
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtVdmControl(Service: VDMSERVICECLASS, ServiceData: *mut std::ffi::c_void) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtTraceEvent(
        TraceHandle: HANDLE,
        Flags: u32,
        FieldSize: u32,
        Fields: *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum TRACE_CONTROL_INFORMATION_CLASS {
    TraceControlStartLogger = 1,
    TraceControlStopLogger = 2,
    TraceControlQueryLogger = 3,
    TraceControlUpdateLogger = 4,
    TraceControlFlushLogger = 5,
    TraceControlIncrementLoggerFile = 6,
    TraceControlUnknown = 7,
    TraceControlRealtimeConnect = 11,
    TraceControlActivityIdCreate = 12,
    TraceControlWdiDispatchControl = 13,
    TraceControlRealtimeDisconnectConsumerByHandle = 14,
    TraceControlRegisterGuidsCode = 15,
    TraceControlReceiveNotification = 16,
    TraceControlSendDataBlock = 17,
    TraceControlSendReplyDataBlock = 18,
    TraceControlReceiveReplyDataBlock = 19,
    TraceControlWdiUpdateSem = 20,
    TraceControlEnumTraceGuidList = 21,
    TraceControlGetTraceGuidInfo = 22,
    TraceControlEnumerateTraceGuids = 23,
    TraceControlRegisterSecurityProv = 24,
    TraceControlQueryReferenceTime = 25,
    TraceControlTrackProviderBinary = 26,
    TraceControlAddNotificationEvent = 27,
    TraceControlUpdateDisallowList = 28,
    TraceControlSetEnableAllKeywordsCode = 29,
    TraceControlSetProviderTraitsCode = 30,
    TraceControlUseDescriptorTypeCode = 31,
    TraceControlEnumTraceGroupList = 32,
    TraceControlGetTraceGroupInfo = 33,
    TraceControlGetDisallowList = 34,
    TraceControlSetCompressionSettings = 35,
    TraceControlGetCompressionSettings = 36,
    TraceControlUpdatePeriodicCaptureState = 37,
    TraceControlGetPrivateSessionTraceHandle = 38,
    TraceControlRegisterPrivateSession = 39,
    TraceControlQuerySessionDemuxObject = 40,
    TraceControlSetProviderBinaryTracking = 41,
    TraceControlMaxLoggers = 42,
    TraceControlMaxPmcCounter = 43,
    TraceControlQueryUsedProcessorCount = 44,
    TraceControlGetPmcOwnership = 45,
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtTraceControl(
        TraceInformationClass: TRACE_CONTROL_INFORMATION_CLASS,
        InputBuffer: *mut std::ffi::c_void,
        InputBufferLength: u32,
        TraceInformation: *mut std::ffi::c_void,
        TraceInformationLength: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}
