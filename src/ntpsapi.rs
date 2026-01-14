use windows::{
    core::GUID,
    Wdk::{
        Foundation::OBJECT_ATTRIBUTES,
        System::{
            SystemServices::{KSYSTEM_TIME, PROCESS_WS_WATCH_INFORMATION},
            Threading::PROCESSINFOCLASS,
        },
    },
    Win32::{
        Foundation::{HANDLE, NTSTATUS, UNICODE_STRING},
        Security::SECURITY_QUALITY_OF_SERVICE,
        System::{
            Diagnostics::Debug::{CONTEXT, LDT_ENTRY},
            JobObjects::{
                JOBOBJECTINFOCLASS, JOBOBJECT_BASIC_ACCOUNTING_INFORMATION,
                JOBOBJECT_BASIC_LIMIT_INFORMATION, JOB_SET_ARRAY,
            },
            Kernel::{LIST_ENTRY, NT_PRODUCT_TYPE, PROCESSOR_NUMBER, SINGLE_LIST_ENTRY},
            Performance::HardwareCounterProfiling::HARDWARE_COUNTER_TYPE,
            SystemServices::{
                PROCESS_MITIGATION_ASLR_POLICY, PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY,
                PROCESS_MITIGATION_CHILD_PROCESS_POLICY,
                PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY,
                PROCESS_MITIGATION_DYNAMIC_CODE_POLICY,
                PROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY,
                PROCESS_MITIGATION_FONT_DISABLE_POLICY, PROCESS_MITIGATION_IMAGE_LOAD_POLICY,
                PROCESS_MITIGATION_PAYLOAD_RESTRICTION_POLICY,
                PROCESS_MITIGATION_REDIRECTION_TRUST_POLICY, PROCESS_MITIGATION_SEHOP_POLICY,
                PROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY,
                PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY,
                PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY,
                PROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY,
                PROCESS_MITIGATION_USER_POINTER_AUTH_POLICY,
                PROCESS_MITIGATION_USER_SHADOW_STACK_POLICY,
            },
            Threading::{IO_COUNTERS, PROCESS_MITIGATION_POLICY},
            WindowsProgramming::CLIENT_ID,
        },
    },
};

use crate::{
    bitfield::{BitfieldUnit, UnionField},
    ntexapi::{PROCESS_DISK_COUNTERS, PROCESS_ENERGY_VALUES},
    ntpebteb::TEB,
};

pub const PROCESS_SET_PORT: u32 = 2048;
pub const GDI_HANDLE_BUFFER_SIZE32: u32 = 34;
pub const GDI_HANDLE_BUFFER_SIZE64: u32 = 60;
pub const GDI_HANDLE_BUFFER_SIZE: u32 = 60;
pub const TLS_EXPANSION_SLOTS: u32 = 1024;
pub const PROCESS_PRIORITY_CLASS_UNKNOWN: u32 = 0;
pub const PROCESS_PRIORITY_CLASS_IDLE: u32 = 1;
pub const PROCESS_PRIORITY_CLASS_NORMAL: u32 = 2;
pub const PROCESS_PRIORITY_CLASS_HIGH: u32 = 3;
pub const PROCESS_PRIORITY_CLASS_REALTIME: u32 = 4;
pub const PROCESS_PRIORITY_CLASS_BELOW_NORMAL: u32 = 5;
pub const PROCESS_PRIORITY_CLASS_ABOVE_NORMAL: u32 = 6;
pub const PROCESS_HANDLE_RAISE_EXCEPTION_ON_INVALID_HANDLE_CLOSE_DISABLED: u32 = 0;
pub const PROCESS_HANDLE_RAISE_EXCEPTION_ON_INVALID_HANDLE_CLOSE_ENABLED: u32 = 1;
pub const PROCESS_HANDLE_TRACING_MAX_SLOTS: u32 = 131072;
pub const PROCESS_HANDLE_TRACE_TYPE_OPEN: u32 = 1;
pub const PROCESS_HANDLE_TRACE_TYPE_CLOSE: u32 = 2;
pub const PROCESS_HANDLE_TRACE_TYPE_BADREF: u32 = 3;
pub const PS_PROTECTED_SIGNER_MASK: u32 = 255;
pub const PS_PROTECTED_AUDIT_MASK: u32 = 8;
pub const PS_PROTECTED_TYPE_MASK: u32 = 7;
pub const WIN32K_SYSCALL_FILTER_STATE_ENABLE: u32 = 1;
pub const WIN32K_SYSCALL_FILTER_STATE_AUDIT: u32 = 2;
pub const PROCESS_READWRITEVM_LOGGING_ENABLE_READVM: u32 = 1;
pub const PROCESS_READWRITEVM_LOGGING_ENABLE_WRITEVM: u32 = 2;
pub const PROCESS_READWRITEVM_LOGGING_ENABLE_READVM_V: u32 = 1;
pub const PROCESS_READWRITEVM_LOGGING_ENABLE_WRITEVM_V: u32 = 2;
pub const PROCESS_CREATE_FLAGS_BREAKAWAY: u32 = 1;
pub const PROCESS_CREATE_FLAGS_NO_DEBUG_INHERIT: u32 = 2;
pub const PROCESS_CREATE_FLAGS_INHERIT_HANDLES: u32 = 4;
pub const PROCESS_CREATE_FLAGS_OVERRIDE_ADDRESS_SPACE: u32 = 8;
pub const PROCESS_CREATE_FLAGS_LARGE_PAGES: u32 = 16;
pub const PROCESS_CREATE_FLAGS_LARGE_PAGE_SYSTEM_DLL: u32 = 32;
pub const PROCESS_CREATE_FLAGS_PROTECTED_PROCESS: u32 = 64;
pub const PROCESS_CREATE_FLAGS_CREATE_SESSION: u32 = 128;
pub const PROCESS_CREATE_FLAGS_INHERIT_FROM_PARENT: u32 = 256;
pub const PROCESS_CREATE_FLAGS_SUSPENDED: u32 = 512;
pub const PROCESS_CREATE_FLAGS_FORCE_BREAKAWAY: u32 = 1024;
pub const PROCESS_CREATE_FLAGS_MINIMAL_PROCESS: u32 = 2048;
pub const PROCESS_CREATE_FLAGS_RELEASE_SECTION: u32 = 4096;
pub const PROCESS_CREATE_FLAGS_CLONE_MINIMAL: u32 = 8192;
pub const PROCESS_CREATE_FLAGS_CLONE_MINIMAL_REDUCED_COMMIT: u32 = 16384;
pub const PROCESS_CREATE_FLAGS_AUXILIARY_PROCESS: u32 = 32768;
pub const PROCESS_CREATE_FLAGS_CREATE_STORE: u32 = 131072;
pub const PROCESS_CREATE_FLAGS_USE_PROTECTED_ENVIRONMENT: u32 = 262144;
pub const PROCESS_GET_NEXT_FLAGS_PREVIOUS_PROCESS: u32 = 1;
pub const STATECHANGE_SET_ATTRIBUTES: u32 = 1;
pub const EXTENDED_PROCESS_CREATION_FLAG_ELEVATION_HANDLED: u32 = 1;
pub const EXTENDED_PROCESS_CREATION_FLAG_FORCELUA: u32 = 2;
pub const EXTENDED_PROCESS_CREATION_FLAG_FORCE_BREAKAWAY: u32 = 4;
pub const PS_ATTRIBUTE_NUMBER_MASK: u32 = 65535;
pub const PS_ATTRIBUTE_THREAD: u32 = 65536;
pub const PS_ATTRIBUTE_INPUT: u32 = 131072;
pub const PS_ATTRIBUTE_ADDITIVE: u32 = 262144;
pub const PS_STD_INPUT_HANDLE: u32 = 1;
pub const PS_STD_OUTPUT_HANDLE: u32 = 2;
pub const PS_STD_ERROR_HANDLE: u32 = 4;
pub const THREAD_CREATE_FLAGS_NONE: u32 = 0;
pub const THREAD_CREATE_FLAGS_CREATE_SUSPENDED: u32 = 1;
pub const THREAD_CREATE_FLAGS_SKIP_THREAD_ATTACH: u32 = 2;
pub const THREAD_CREATE_FLAGS_HIDE_FROM_DEBUGGER: u32 = 4;
pub const THREAD_CREATE_FLAGS_LOADER_WORKER: u32 = 16;
pub const THREAD_CREATE_FLAGS_SKIP_LOADER_INIT: u32 = 32;
pub const THREAD_CREATE_FLAGS_BYPASS_PROCESS_FREEZE: u32 = 64;
pub const JobObjectFreezeInformation: u32 = 18;
pub const JobObjectExtendedAccountingInformation: u32 = 19;
pub const JobObjectWakeInformation: u32 = 20;
pub const JobObjectBackgroundInformation: u32 = 21;
pub const JobObjectSchedulingRankBiasInformation: u32 = 22;
pub const JobObjectTimerVirtualizationInformation: u32 = 23;
pub const JobObjectCycleTimeNotification: u32 = 24;
pub const JobObjectClearEvent: u32 = 25;
pub const JobObjectInterferenceInformation: u32 = 26;
pub const JobObjectClearPeakJobMemoryUsed: u32 = 27;
pub const JobObjectMemoryUsageInformation: u32 = 28;
pub const JobObjectSharedCommit: u32 = 29;
pub const JobObjectContainerId: u32 = 30;
pub const JobObjectIoRateControlInformation: u32 = 31;
pub const JobObjectSiloRootDirectory: u32 = 37;
pub const JobObjectServerSiloBasicInformation: u32 = 38;
pub const JobObjectServerSiloUserSharedData: u32 = 39;
pub const JobObjectServerSiloInitialize: u32 = 40;
pub const JobObjectServerSiloRunningState: u32 = 41;
pub const JobObjectIoAttribution: u32 = 42;
pub const JobObjectMemoryPartitionInformation: u32 = 43;
pub const JobObjectContainerTelemetryId: u32 = 44;
pub const JobObjectSiloSystemRoot: u32 = 45;
pub const JobObjectEnergyTrackingState: u32 = 46;
pub const JobObjectThreadImpersonationInformation: u32 = 47;
pub const JobObjectIoPriorityLimit: u32 = 48;
pub const JobObjectPagePriorityLimit: u32 = 49;
pub const JOB_OBJECT_LIMIT_SILO_READY: u32 = 4194304;
pub const SILO_OBJECT_ROOT_DIRECTORY_SHADOW_ROOT: u32 = 1;
pub const SILO_OBJECT_ROOT_DIRECTORY_INITIALIZE: u32 = 2;
pub const SILO_OBJECT_ROOT_DIRECTORY_SHADOW_DOS_DEVICES: u32 = 4;
pub const MEMORY_BULK_INFORMATION_FLAG_BASIC: u32 = 1;
pub const JOB_OBJECT_ALL_ACCESS: u32 = 2031679;
pub const PROCESS_EXCEPTION_PORT_ALL_STATE_FLAGS: u32 = 7;
pub const CONTEXT_ARM_CONTROL: u32 = 2097153;
pub const CONTEXT_ARM_INTEGER: u32 = 2097154;
pub const CONTEXT_ARM_FLOATING_POINT: u32 = 2097156;
pub const CONTEXT_ARM_DEBUG_REGISTERS: u32 = 2097160;
pub const CONTEXT_ARM_FULL: u32 = 2097159;
pub const CONTEXT_ARM_ALL: u32 = 2097167;
pub const QUEUE_USER_APC_SPECIAL_USER_APC: HANDLE = HANDLE(1isize as *mut std::ffi::c_void);
pub const PS_ATTRIBUTE_PARENT_PROCESS: u32 = 393216;
pub const PS_ATTRIBUTE_DEBUG_OBJECT: u32 = 393217;
pub const PS_ATTRIBUTE_TOKEN: u32 = 393218;
pub const PS_ATTRIBUTE_CLIENT_ID: u32 = 65539;
pub const PS_ATTRIBUTE_TEB_ADDRESS: u32 = 65540;
pub const PS_ATTRIBUTE_IMAGE_NAME: u32 = 131077;
pub const PS_ATTRIBUTE_IMAGE_INFO: u32 = 6;
pub const PS_ATTRIBUTE_MEMORY_RESERVE: u32 = 131079;
pub const PS_ATTRIBUTE_PRIORITY_CLASS: u32 = 131080;
pub const PS_ATTRIBUTE_ERROR_MODE: u32 = 131081;
pub const PS_ATTRIBUTE_STD_HANDLE_INFO: u32 = 131082;
pub const PS_ATTRIBUTE_HANDLE_LIST: u32 = 131083;
pub const PS_ATTRIBUTE_GROUP_AFFINITY: u32 = 196620;
pub const PS_ATTRIBUTE_PREFERRED_NODE: u32 = 131085;
pub const PS_ATTRIBUTE_IDEAL_PROCESSOR: u32 = 196622;
pub const PS_ATTRIBUTE_UMS_THREAD: u32 = 196623;
pub const PS_ATTRIBUTE_MITIGATION_OPTIONS: u32 = 131088;
pub const PS_ATTRIBUTE_PROTECTION_LEVEL: u32 = 393233;
pub const PS_ATTRIBUTE_SECURE_PROCESS: u32 = 131090;
pub const PS_ATTRIBUTE_JOB_LIST: u32 = 131091;
pub const PS_ATTRIBUTE_CHILD_PROCESS_POLICY: u32 = 131092;
pub const PS_ATTRIBUTE_ALL_APPLICATION_PACKAGES_POLICY: u32 = 131093;
pub const PS_ATTRIBUTE_WIN32K_FILTER: u32 = 131094;
pub const PS_ATTRIBUTE_SAFE_OPEN_PROMPT_ORIGIN_CLAIM: u32 = 131095;
pub const PS_ATTRIBUTE_BNO_ISOLATION: u32 = 131096;
pub const PS_ATTRIBUTE_DESKTOP_APP_POLICY: u32 = 131097;
pub const PS_ATTRIBUTE_CHPE: u32 = 393242;
pub const PS_ATTRIBUTE_MITIGATION_AUDIT_OPTIONS: u32 = 131099;
pub const PS_ATTRIBUTE_MACHINE_TYPE: u32 = 393244;
pub const PS_ATTRIBUTE_COMPONENT_FILTER: u32 = 131101;
pub const PS_ATTRIBUTE_ENABLE_OPTIONAL_XSTATE_FEATURES: u32 = 196638;
pub const POWER_THROTTLING_PROCESS_VALID_FLAGS: u32 = 7;
pub const ProcThreadAttributeExtendedFlags: u32 = 1;
pub const ProcThreadAttributePackageFullName: u32 = 8;
pub const ProcThreadAttributeConsoleReference: u32 = 10;
pub const ProcThreadAttributeOsMaxVersionTested: u32 = 12;
pub const ProcThreadAttributeBnoIsolation: u32 = 19;
pub const ProcThreadAttributeIsolationManifest: u32 = 23;
pub const ProcThreadAttributeCreateStore: u32 = 28;

#[repr(C)]
pub struct PEB_LDR_DATA {
    pub Length: u32,
    pub Initialized: bool,
    pub SsHandle: HANDLE,
    pub InLoadOrderModuleList: LIST_ENTRY,
    pub InMemoryOrderModuleList: LIST_ENTRY,
    pub InInitializationOrderModuleList: LIST_ENTRY,
    pub EntryInProgress: *mut std::ffi::c_void,
    pub ShutdownInProgress: bool,
    pub ShutdownThreadId: HANDLE,
}

impl Default for PEB_LDR_DATA {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PEB_LDR_DATA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PEB_LDR_DATA {{  }}")
    }
}

#[repr(C)]
pub struct INITIAL_TEB {
    pub OldInitialTeb: INITIAL_TEB_1,
    pub StackBase: *mut std::ffi::c_void,
    pub StackLimit: *mut std::ffi::c_void,
    pub StackAllocationBase: *mut std::ffi::c_void,
}

#[repr(C)]
pub struct INITIAL_TEB_1 {
    pub OldStackBase: *mut std::ffi::c_void,
    pub OldStackLimit: *mut std::ffi::c_void,
}

impl Default for INITIAL_TEB_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for INITIAL_TEB_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "INITIAL_TEB_1 {{  }}")
    }
}

impl Default for INITIAL_TEB {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for INITIAL_TEB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "INITIAL_TEB {{ OldInitialTeb: {:?} }}",
            self.OldInitialTeb
        )
    }
}

#[repr(C)]
pub struct WOW64_PROCESS {
    pub Wow64: *mut std::ffi::c_void,
}

impl Default for WOW64_PROCESS {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for WOW64_PROCESS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WOW64_PROCESS {{  }}")
    }
}

#[repr(C)]
pub struct PROCESS_LDT_INFORMATION {
    pub Start: u32,
    pub Length: u32,
    pub LdtEntries: [LDT_ENTRY; 1],
}

impl Default for PROCESS_LDT_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PROCESS_LDT_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PROCESS_LDT_INFORMATION {{ [LdtEntries] }}")
    }
}

#[repr(C)]
pub struct PROCESS_LDT_SIZE {
    pub Length: u32,
}

impl Default for PROCESS_LDT_SIZE {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PROCESS_LDT_SIZE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PROCESS_LDT_SIZE {{  }}")
    }
}

#[repr(C)]
pub struct PROCESS_WS_WATCH_INFORMATION_EX {
    pub BasicInfo: PROCESS_WS_WATCH_INFORMATION,
    pub FaultingThreadId: usize,
    pub Flags: usize,
}

impl Default for PROCESS_WS_WATCH_INFORMATION_EX {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PROCESS_WS_WATCH_INFORMATION_EX {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PROCESS_WS_WATCH_INFORMATION_EX {{ BasicInfo: {:?} }}",
            self.BasicInfo
        )
    }
}

#[repr(C)]
pub struct PROCESS_PRIORITY_CLASS {
    pub Foreground: bool,
    pub PriorityClass: u8,
}

impl Default for PROCESS_PRIORITY_CLASS {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PROCESS_PRIORITY_CLASS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PROCESS_PRIORITY_CLASS {{  }}")
    }
}

#[repr(C)]
pub struct PROCESS_PRIORITY_CLASS_EX {
    pub Anonymous1: PROCESS_PRIORITY_CLASS_EX_1,
    pub PriorityClass: u8,
    pub Foreground: bool,
}

#[repr(C)]
pub struct PROCESS_PRIORITY_CLASS_EX_1 {
    pub Anonymous1: UnionField<PROCESS_PRIORITY_CLASS_EX_1_1>,
    pub AllFlags: UnionField<u16>,
    pub union_field: u16,
}

#[repr(C)]
#[repr(align(2))]
pub struct PROCESS_PRIORITY_CLASS_EX_1_1 {
    _bitfield_align_1: [u8; 0],
    _bitfield_1: BitfieldUnit<[u8; 1]>,
    pub padding_0: u8,
}

impl Default for PROCESS_PRIORITY_CLASS_EX_1_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PROCESS_PRIORITY_CLASS_EX_1_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PROCESS_PRIORITY_CLASS_EX_1_1 {{ ForegroundValid : {:?}, PriorityClassValid : {:?} }}",
            self.ForegroundValid(),
            self.PriorityClassValid()
        )
    }
}

impl PROCESS_PRIORITY_CLASS_EX_1_1 {
    #[inline]
    pub fn ForegroundValid(&self) -> u16 {
        self._bitfield_1.get(0usize, 1u8) as u16
    }

    #[inline]
    pub fn set_ForegroundValid(&mut self, val: u16) {
        self._bitfield_1.set(0usize, 1u8, val as u64)
    }

    #[inline]
    pub fn PriorityClassValid(&self) -> u16 {
        self._bitfield_1.get(1usize, 1u8) as u16
    }

    #[inline]
    pub fn set_PriorityClassValid(&mut self, val: u16) {
        self._bitfield_1.set(1usize, 1u8, val as u64)
    }

    #[inline]
    pub fn new_bitfield_1(ForegroundValid: u16, PriorityClassValid: u16) -> BitfieldUnit<[u8; 1]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 1]> = Default::default();

        bitfield_unit.set(0usize, 1u8, ForegroundValid as u64);

        bitfield_unit.set(1usize, 1u8, PriorityClassValid as u64);

        bitfield_unit
    }
}

impl Default for PROCESS_PRIORITY_CLASS_EX_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PROCESS_PRIORITY_CLASS_EX_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PROCESS_PRIORITY_CLASS_EX_1 {{ union }}")
    }
}

impl Default for PROCESS_PRIORITY_CLASS_EX {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PROCESS_PRIORITY_CLASS_EX {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PROCESS_PRIORITY_CLASS_EX {{ Anonymous1: {:?} }}",
            self.Anonymous1
        )
    }
}

#[repr(C)]
pub struct PROCESS_FOREGROUND_BACKGROUND {
    pub Foreground: bool,
}

impl Default for PROCESS_FOREGROUND_BACKGROUND {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PROCESS_FOREGROUND_BACKGROUND {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PROCESS_FOREGROUND_BACKGROUND {{  }}")
    }
}

#[repr(C)]
pub struct THREAD_TLS_INFORMATION {
    pub Flags: u32,
    pub NewTlsData: *mut std::ffi::c_void,
    pub OldTlsData: *mut std::ffi::c_void,
    pub ThreadId: HANDLE,
}

impl Default for THREAD_TLS_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for THREAD_TLS_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "THREAD_TLS_INFORMATION {{  }}")
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum PROCESS_TLS_INFORMATION_TYPE {
    ProcessTlsReplaceIndex = 0,
    ProcessTlsReplaceVector = 1,
    MaxProcessTlsOperation = 2,
}

#[repr(C)]
pub struct PROCESS_TLS_INFORMATION {
    pub Flags: u32,
    pub OperationType: u32,
    pub ThreadDataCount: u32,
    pub TlsIndex: u32,
    pub PreviousCount: u32,
    pub ThreadData: [THREAD_TLS_INFORMATION; 1],
}

impl Default for PROCESS_TLS_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PROCESS_TLS_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PROCESS_TLS_INFORMATION {{ ThreadData: {:?} }}",
            self.ThreadData
        )
    }
}

#[repr(C)]
pub struct PROCESS_INSTRUMENTATION_CALLBACK_INFORMATION {
    pub Version: u32,
    pub Reserved: u32,
    pub Callback: *mut std::ffi::c_void,
}

impl Default for PROCESS_INSTRUMENTATION_CALLBACK_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PROCESS_INSTRUMENTATION_CALLBACK_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PROCESS_INSTRUMENTATION_CALLBACK_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct PROCESS_STACK_ALLOCATION_INFORMATION {
    pub ReserveSize: usize,
    pub ZeroBits: usize,
    pub StackBase: *mut std::ffi::c_void,
}

impl Default for PROCESS_STACK_ALLOCATION_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PROCESS_STACK_ALLOCATION_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PROCESS_STACK_ALLOCATION_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct PROCESS_STACK_ALLOCATION_INFORMATION_EX {
    pub PreferredNode: u32,
    pub Reserved0: u32,
    pub Reserved1: u32,
    pub Reserved2: u32,
    pub AllocInfo: PROCESS_STACK_ALLOCATION_INFORMATION,
}

impl Default for PROCESS_STACK_ALLOCATION_INFORMATION_EX {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PROCESS_STACK_ALLOCATION_INFORMATION_EX {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PROCESS_STACK_ALLOCATION_INFORMATION_EX {{ AllocInfo: {:?} }}",
            self.AllocInfo
        )
    }
}

#[repr(C)]
pub struct PROCESS_AFFINITY_UPDATE_MODE {
    pub Flags: UnionField<u32>,
    pub Anonymous1: UnionField<PROCESS_AFFINITY_UPDATE_MODE_1>,
    pub union_field: u32,
}

#[repr(C)]
#[repr(align(4))]
pub struct PROCESS_AFFINITY_UPDATE_MODE_1 {
    _bitfield_align_1: [u32; 0],
    _bitfield_1: BitfieldUnit<[u8; 4]>,
}

impl Default for PROCESS_AFFINITY_UPDATE_MODE_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PROCESS_AFFINITY_UPDATE_MODE_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PROCESS_AFFINITY_UPDATE_MODE_1 {{ EnableAutoUpdate : {:?}, Permanent : {:?}, Reserved : {:?} }}",
            self.EnableAutoUpdate(),
            self.Permanent(),
            self.Reserved()
        )
    }
}

impl PROCESS_AFFINITY_UPDATE_MODE_1 {
    #[inline]
    pub fn EnableAutoUpdate(&self) -> u32 {
        self._bitfield_1.get(0usize, 1u8) as u32
    }

    #[inline]
    pub fn set_EnableAutoUpdate(&mut self, val: u32) {
        self._bitfield_1.set(0usize, 1u8, val as u64)
    }

    #[inline]
    pub fn Permanent(&self) -> u32 {
        self._bitfield_1.get(1usize, 1u8) as u32
    }

    #[inline]
    pub fn set_Permanent(&mut self, val: u32) {
        self._bitfield_1.set(1usize, 1u8, val as u64)
    }

    #[inline]
    pub fn Reserved(&self) -> u32 {
        self._bitfield_1.get(2usize, 30u8) as u32
    }

    #[inline]
    pub fn set_Reserved(&mut self, val: u32) {
        self._bitfield_1.set(2usize, 30u8, val as u64)
    }

    #[inline]
    pub fn new_bitfield_1(
        EnableAutoUpdate: u32,
        Permanent: u32,
        Reserved: u32,
    ) -> BitfieldUnit<[u8; 4]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 4]> = Default::default();

        bitfield_unit.set(0usize, 1u8, EnableAutoUpdate as u64);

        bitfield_unit.set(1usize, 1u8, Permanent as u64);

        bitfield_unit.set(2usize, 30u8, Reserved as u64);

        bitfield_unit
    }
}

impl Default for PROCESS_AFFINITY_UPDATE_MODE {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PROCESS_AFFINITY_UPDATE_MODE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PROCESS_AFFINITY_UPDATE_MODE {{ union }}")
    }
}

#[repr(C)]
pub struct PROCESS_MEMORY_ALLOCATION_MODE {
    pub Flags: UnionField<u32>,
    pub Anonymous1: UnionField<PROCESS_MEMORY_ALLOCATION_MODE_1>,
    pub union_field: u32,
}

#[repr(C)]
#[repr(align(4))]
pub struct PROCESS_MEMORY_ALLOCATION_MODE_1 {
    _bitfield_align_1: [u32; 0],
    _bitfield_1: BitfieldUnit<[u8; 4]>,
}

impl Default for PROCESS_MEMORY_ALLOCATION_MODE_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PROCESS_MEMORY_ALLOCATION_MODE_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PROCESS_MEMORY_ALLOCATION_MODE_1 {{ TopDown : {:?}, Reserved : {:?} }}",
            self.TopDown(),
            self.Reserved()
        )
    }
}

impl PROCESS_MEMORY_ALLOCATION_MODE_1 {
    #[inline]
    pub fn TopDown(&self) -> u32 {
        self._bitfield_1.get(0usize, 1u8) as u32
    }

    #[inline]
    pub fn set_TopDown(&mut self, val: u32) {
        self._bitfield_1.set(0usize, 1u8, val as u64)
    }

    #[inline]
    pub fn Reserved(&self) -> u32 {
        self._bitfield_1.get(1usize, 31u8) as u32
    }

    #[inline]
    pub fn set_Reserved(&mut self, val: u32) {
        self._bitfield_1.set(1usize, 31u8, val as u64)
    }

    #[inline]
    pub fn new_bitfield_1(TopDown: u32, Reserved: u32) -> BitfieldUnit<[u8; 4]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 4]> = Default::default();

        bitfield_unit.set(0usize, 1u8, TopDown as u64);

        bitfield_unit.set(1usize, 31u8, Reserved as u64);

        bitfield_unit
    }
}

impl Default for PROCESS_MEMORY_ALLOCATION_MODE {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PROCESS_MEMORY_ALLOCATION_MODE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PROCESS_MEMORY_ALLOCATION_MODE {{ union }}")
    }
}

#[repr(C)]
pub struct PROCESS_HANDLE_INFORMATION {
    pub HandleCount: u32,
    pub HandleCountHighWatermark: u32,
}

impl Default for PROCESS_HANDLE_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PROCESS_HANDLE_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PROCESS_HANDLE_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct PROCESS_CYCLE_TIME_INFORMATION {
    pub AccumulatedCycles: u64,
    pub CurrentCycleCount: u64,
}

impl Default for PROCESS_CYCLE_TIME_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PROCESS_CYCLE_TIME_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PROCESS_CYCLE_TIME_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct PROCESS_WINDOW_INFORMATION {
    pub WindowFlags: u32,
    pub WindowTitleLength: u16,
    pub WindowTitle: [u16; 1],
}

impl Default for PROCESS_WINDOW_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PROCESS_WINDOW_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PROCESS_WINDOW_INFORMATION {{ WindowTitle: {:?} }}",
            self.WindowTitle
        )
    }
}

#[repr(C)]
pub struct PROCESS_HANDLE_TABLE_ENTRY_INFO {
    pub HandleValue: HANDLE,
    pub HandleCount: usize,
    pub PointerCount: usize,
    pub GrantedAccess: u32,
    pub ObjectTypeIndex: u32,
    pub HandleAttributes: u32,
    pub Reserved: u32,
}

impl Default for PROCESS_HANDLE_TABLE_ENTRY_INFO {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PROCESS_HANDLE_TABLE_ENTRY_INFO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PROCESS_HANDLE_TABLE_ENTRY_INFO {{  }}")
    }
}

#[repr(C)]
pub struct PROCESS_HANDLE_SNAPSHOT_INFORMATION {
    pub NumberOfHandles: usize,
    pub Reserved: usize,
    pub Handles: [PROCESS_HANDLE_TABLE_ENTRY_INFO; 1],
}

impl Default for PROCESS_HANDLE_SNAPSHOT_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PROCESS_HANDLE_SNAPSHOT_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PROCESS_HANDLE_SNAPSHOT_INFORMATION {{ Handles: {:?} }}",
            self.Handles
        )
    }
}

#[repr(C)]
pub struct PROCESS_MITIGATION_POLICY_INFORMATION {
    pub Policy: PROCESS_MITIGATION_POLICY,
    pub Anonymous1: PROCESS_MITIGATION_POLICY_INFORMATION_1,
}

#[repr(C)]
pub struct PROCESS_MITIGATION_POLICY_INFORMATION_1 {
    pub ASLRPolicy: UnionField<PROCESS_MITIGATION_ASLR_POLICY>,
    pub StrictHandleCheckPolicy: UnionField<PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY>,
    pub SystemCallDisablePolicy: UnionField<PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY>,
    pub ExtensionPointDisablePolicy: UnionField<PROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY>,
    pub DynamicCodePolicy: UnionField<PROCESS_MITIGATION_DYNAMIC_CODE_POLICY>,
    pub ControlFlowGuardPolicy: UnionField<PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY>,
    pub SignaturePolicy: UnionField<PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY>,
    pub FontDisablePolicy: UnionField<PROCESS_MITIGATION_FONT_DISABLE_POLICY>,
    pub ImageLoadPolicy: UnionField<PROCESS_MITIGATION_IMAGE_LOAD_POLICY>,
    pub SystemCallFilterPolicy: UnionField<PROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY>,
    pub PayloadRestrictionPolicy: UnionField<PROCESS_MITIGATION_PAYLOAD_RESTRICTION_POLICY>,
    pub ChildProcessPolicy: UnionField<PROCESS_MITIGATION_CHILD_PROCESS_POLICY>,
    pub SideChannelIsolationPolicy: UnionField<PROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY>,
    pub UserShadowStackPolicy: UnionField<PROCESS_MITIGATION_USER_SHADOW_STACK_POLICY>,
    pub RedirectionTrustPolicy: UnionField<PROCESS_MITIGATION_REDIRECTION_TRUST_POLICY>,
    pub UserPointerAuthPolicy: UnionField<PROCESS_MITIGATION_USER_POINTER_AUTH_POLICY>,
    pub SEHOPPolicy: UnionField<PROCESS_MITIGATION_SEHOP_POLICY>,
    pub union_field: u32,
}

impl Default for PROCESS_MITIGATION_POLICY_INFORMATION_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PROCESS_MITIGATION_POLICY_INFORMATION_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PROCESS_MITIGATION_POLICY_INFORMATION_1 {{ union }}")
    }
}

impl Default for PROCESS_MITIGATION_POLICY_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PROCESS_MITIGATION_POLICY_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PROCESS_MITIGATION_POLICY_INFORMATION {{ Anonymous1: {:?} }}",
            self.Anonymous1
        )
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum PROCESS_WORKING_SET_OPERATION {
    ProcessWorkingSetSwap = 0,
    ProcessWorkingSetEmpty = 1,
    ProcessWorkingSetOperationMax = 2,
}

#[repr(C)]
pub struct PROCESS_WORKING_SET_CONTROL {
    pub Version: u32,
    pub Operation: PROCESS_WORKING_SET_OPERATION,
    pub Flags: u32,
}

impl Default for PROCESS_WORKING_SET_CONTROL {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PROCESS_WORKING_SET_CONTROL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PROCESS_WORKING_SET_CONTROL {{ Operation: {:?} }}",
            self.Operation
        )
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum PS_PROTECTED_TYPE {
    PsProtectedTypeNone = 0,
    PsProtectedTypeProtectedLight = 1,
    PsProtectedTypeProtected = 2,
    PsProtectedTypeMax = 3,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum PS_PROTECTED_SIGNER {
    PsProtectedSignerNone = 0,
    PsProtectedSignerAuthenticode = 1,
    PsProtectedSignerCodeGen = 2,
    PsProtectedSignerAntimalware = 3,
    PsProtectedSignerLsa = 4,
    PsProtectedSignerWindows = 5,
    PsProtectedSignerWinTcb = 6,
    PsProtectedSignerWinSystem = 7,
    PsProtectedSignerApp = 8,
    PsProtectedSignerMax = 9,
}

#[repr(C)]
pub struct PS_PROTECTION {
    pub Anonymous1: PS_PROTECTION_1,
}

#[repr(C)]
pub struct PS_PROTECTION_1 {
    pub Level: UnionField<u8>,
    pub Anonymous1: UnionField<PS_PROTECTION_1_1>,
    pub union_field: u8,
}

#[repr(C, packed)]
pub struct PS_PROTECTION_1_1 {
    _bitfield_align_1: [u8; 0],
    _bitfield_1: BitfieldUnit<[u8; 1]>,
}

impl Default for PS_PROTECTION_1_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PS_PROTECTION_1_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PS_PROTECTION_1_1 {{ Type : {:?}, Audit : {:?}, Signer : {:?} }}",
            self.Type(),
            self.Audit(),
            self.Signer()
        )
    }
}

impl PS_PROTECTION_1_1 {
    #[inline]
    pub fn Type(&self) -> u8 {
        self._bitfield_1.get(0usize, 3u8) as u8
    }

    #[inline]
    pub fn set_Type(&mut self, val: u8) {
        self._bitfield_1.set(0usize, 3u8, val as u64)
    }

    #[inline]
    pub fn Audit(&self) -> u8 {
        self._bitfield_1.get(3usize, 1u8) as u8
    }

    #[inline]
    pub fn set_Audit(&mut self, val: u8) {
        self._bitfield_1.set(3usize, 1u8, val as u64)
    }

    #[inline]
    pub fn Signer(&self) -> u8 {
        self._bitfield_1.get(4usize, 4u8) as u8
    }

    #[inline]
    pub fn set_Signer(&mut self, val: u8) {
        self._bitfield_1.set(4usize, 4u8, val as u64)
    }

    #[inline]
    pub fn new_bitfield_1(Type: u8, Audit: u8, Signer: u8) -> BitfieldUnit<[u8; 1]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 1]> = Default::default();

        bitfield_unit.set(0usize, 3u8, Type as u64);

        bitfield_unit.set(3usize, 1u8, Audit as u64);

        bitfield_unit.set(4usize, 4u8, Signer as u64);

        bitfield_unit
    }
}

impl Default for PS_PROTECTION_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PS_PROTECTION_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PS_PROTECTION_1 {{ union }}")
    }
}

impl Default for PS_PROTECTION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PS_PROTECTION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PS_PROTECTION {{ Anonymous1: {:?} }}", self.Anonymous1)
    }
}

#[repr(C)]
pub struct PROCESS_FAULT_INFORMATION {
    pub FaultFlags: u32,
    pub AdditionalInfo: u32,
}

impl Default for PROCESS_FAULT_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PROCESS_FAULT_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PROCESS_FAULT_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct PROCESS_TELEMETRY_ID_INFORMATION {
    pub HeaderSize: u32,
    pub ProcessId: u32,
    pub ProcessStartKey: u64,
    pub CreateTime: u64,
    pub CreateInterruptTime: u64,
    pub CreateUnbiasedInterruptTime: u64,
    pub ProcessSequenceNumber: u64,
    pub SessionCreateTime: u64,
    pub SessionId: u32,
    pub BootId: u32,
    pub ImageChecksum: u32,
    pub ImageTimeDateStamp: u32,
    pub UserSidOffset: u32,
    pub ImagePathOffset: u32,
    pub PackageNameOffset: u32,
    pub RelativeAppNameOffset: u32,
    pub CommandLineOffset: u32,
}

impl Default for PROCESS_TELEMETRY_ID_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PROCESS_TELEMETRY_ID_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PROCESS_TELEMETRY_ID_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct PROCESS_COMMIT_RELEASE_INFORMATION {
    pub Version: u32,
    pub Anonymous1: PROCESS_COMMIT_RELEASE_INFORMATION_1,
    pub CommitDebt: usize,
    pub CommittedMemResetSize: usize,
    pub RepurposedMemResetSize: usize,
}

#[repr(C)]
#[repr(align(4))]
pub struct PROCESS_COMMIT_RELEASE_INFORMATION_1 {
    _bitfield_align_1: [u32; 0],
    _bitfield_1: BitfieldUnit<[u8; 4]>,
}

impl Default for PROCESS_COMMIT_RELEASE_INFORMATION_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PROCESS_COMMIT_RELEASE_INFORMATION_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PROCESS_COMMIT_RELEASE_INFORMATION_1 {{ Eligible : {:?}, ReleaseRepurposedMemResetCommit : {:?}, ForceReleaseMemResetCommit : {:?}, Spare : {:?} }}",
            self.Eligible(),
            self.ReleaseRepurposedMemResetCommit(),
            self.ForceReleaseMemResetCommit(),
            self.Spare()
        )
    }
}

impl PROCESS_COMMIT_RELEASE_INFORMATION_1 {
    #[inline]
    pub fn Eligible(&self) -> u32 {
        self._bitfield_1.get(0usize, 1u8) as u32
    }

    #[inline]
    pub fn set_Eligible(&mut self, val: u32) {
        self._bitfield_1.set(0usize, 1u8, val as u64)
    }

    #[inline]
    pub fn ReleaseRepurposedMemResetCommit(&self) -> u32 {
        self._bitfield_1.get(1usize, 1u8) as u32
    }

    #[inline]
    pub fn set_ReleaseRepurposedMemResetCommit(&mut self, val: u32) {
        self._bitfield_1.set(1usize, 1u8, val as u64)
    }

    #[inline]
    pub fn ForceReleaseMemResetCommit(&self) -> u32 {
        self._bitfield_1.get(2usize, 1u8) as u32
    }

    #[inline]
    pub fn set_ForceReleaseMemResetCommit(&mut self, val: u32) {
        self._bitfield_1.set(2usize, 1u8, val as u64)
    }

    #[inline]
    pub fn Spare(&self) -> u32 {
        self._bitfield_1.get(3usize, 29u8) as u32
    }

    #[inline]
    pub fn set_Spare(&mut self, val: u32) {
        self._bitfield_1.set(3usize, 29u8, val as u64)
    }

    #[inline]
    pub fn new_bitfield_1(
        Eligible: u32,
        ReleaseRepurposedMemResetCommit: u32,
        ForceReleaseMemResetCommit: u32,
        Spare: u32,
    ) -> BitfieldUnit<[u8; 4]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 4]> = Default::default();

        bitfield_unit.set(0usize, 1u8, Eligible as u64);

        bitfield_unit.set(1usize, 1u8, ReleaseRepurposedMemResetCommit as u64);

        bitfield_unit.set(2usize, 1u8, ForceReleaseMemResetCommit as u64);

        bitfield_unit.set(3usize, 29u8, Spare as u64);

        bitfield_unit
    }
}

impl Default for PROCESS_COMMIT_RELEASE_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PROCESS_COMMIT_RELEASE_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PROCESS_COMMIT_RELEASE_INFORMATION {{ Anonymous1: {:?} }}",
            self.Anonymous1
        )
    }
}

#[repr(C)]
pub struct PROCESS_JOB_MEMORY_INFO {
    pub SharedCommitUsage: u64,
    pub PrivateCommitUsage: u64,
    pub PeakPrivateCommitUsage: u64,
    pub PrivateCommitLimit: u64,
    pub TotalCommitLimit: u64,
}

impl Default for PROCESS_JOB_MEMORY_INFO {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PROCESS_JOB_MEMORY_INFO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PROCESS_JOB_MEMORY_INFO {{  }}")
    }
}

#[repr(C)]
pub struct PROCESS_CHILD_PROCESS_INFORMATION {
    pub ProhibitChildProcesses: bool,
    pub AlwaysAllowSecureChildProcess: bool,
    pub AuditProhibitChildProcesses: bool,
}

impl Default for PROCESS_CHILD_PROCESS_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PROCESS_CHILD_PROCESS_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PROCESS_CHILD_PROCESS_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct WIN32K_SYSCALL_FILTER {
    pub FilterState: u32,
    pub FilterSet: u32,
}

impl Default for WIN32K_SYSCALL_FILTER {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for WIN32K_SYSCALL_FILTER {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WIN32K_SYSCALL_FILTER {{  }}")
    }
}

#[repr(C)]
pub struct PROCESS_WAKE_INFORMATION {
    pub NotificationChannel: u64,
    pub WakeCounters: [u32; 7],
    pub WakeFilter: *mut JOBOBJECT_WAKE_FILTER,
}

impl Default for PROCESS_WAKE_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PROCESS_WAKE_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PROCESS_WAKE_INFORMATION {{ WakeCounters: {:?}, WakeFilter: {:?} }}",
            self.WakeCounters, self.WakeFilter
        )
    }
}

#[repr(C)]
pub struct PROCESS_ENERGY_TRACKING_STATE {
    pub StateUpdateMask: u32,
    pub StateDesiredValue: u32,
    pub StateSequence: u32,
    _bitfield_align_1: [u8; 0],
    _bitfield_1: BitfieldUnit<[u8; 1]>,
    pub padding_0: u16,
    pub Tag: [u16; 64],
}

impl Default for PROCESS_ENERGY_TRACKING_STATE {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PROCESS_ENERGY_TRACKING_STATE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PROCESS_ENERGY_TRACKING_STATE {{ UpdateTag : {:?}, Tag: {:?} }}",
            self.UpdateTag(),
            self.Tag
        )
    }
}

impl PROCESS_ENERGY_TRACKING_STATE {
    #[inline]
    pub fn UpdateTag(&self) -> u32 {
        self._bitfield_1.get(0usize, 1u8) as u32
    }

    #[inline]
    pub fn set_UpdateTag(&mut self, val: u32) {
        self._bitfield_1.set(0usize, 1u8, val as u64)
    }

    #[inline]
    pub fn new_bitfield_1(UpdateTag: u32) -> BitfieldUnit<[u8; 1]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 1]> = Default::default();

        bitfield_unit.set(0usize, 1u8, UpdateTag as u64);

        bitfield_unit
    }
}

#[repr(C)]
pub struct MANAGE_WRITES_TO_EXECUTABLE_MEMORY {
    _bitfield_align_1: [u32; 0],
    _bitfield_1: BitfieldUnit<[u8; 4]>,
    pub KernelWriteToExecutableSignal: *mut std::ffi::c_void,
}

impl Default for MANAGE_WRITES_TO_EXECUTABLE_MEMORY {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for MANAGE_WRITES_TO_EXECUTABLE_MEMORY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MANAGE_WRITES_TO_EXECUTABLE_MEMORY {{ Version : {:?}, ProcessEnableWriteExceptions : {:?}, ThreadAllowWrites : {:?}, Spare : {:?} }}",
            self.Version(),
            self.ProcessEnableWriteExceptions(),
            self.ThreadAllowWrites(),
            self.Spare()
        )
    }
}

impl MANAGE_WRITES_TO_EXECUTABLE_MEMORY {
    #[inline]
    pub fn Version(&self) -> u32 {
        self._bitfield_1.get(0usize, 8u8) as u32
    }

    #[inline]
    pub fn set_Version(&mut self, val: u32) {
        self._bitfield_1.set(0usize, 8u8, val as u64)
    }

    #[inline]
    pub fn ProcessEnableWriteExceptions(&self) -> u32 {
        self._bitfield_1.get(8usize, 1u8) as u32
    }

    #[inline]
    pub fn set_ProcessEnableWriteExceptions(&mut self, val: u32) {
        self._bitfield_1.set(8usize, 1u8, val as u64)
    }

    #[inline]
    pub fn ThreadAllowWrites(&self) -> u32 {
        self._bitfield_1.get(9usize, 1u8) as u32
    }

    #[inline]
    pub fn set_ThreadAllowWrites(&mut self, val: u32) {
        self._bitfield_1.set(9usize, 1u8, val as u64)
    }

    #[inline]
    pub fn Spare(&self) -> u32 {
        self._bitfield_1.get(10usize, 22u8) as u32
    }

    #[inline]
    pub fn set_Spare(&mut self, val: u32) {
        self._bitfield_1.set(10usize, 22u8, val as u64)
    }

    #[inline]
    pub fn new_bitfield_1(
        Version: u32,
        ProcessEnableWriteExceptions: u32,
        ThreadAllowWrites: u32,
        Spare: u32,
    ) -> BitfieldUnit<[u8; 4]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 4]> = Default::default();

        bitfield_unit.set(0usize, 8u8, Version as u64);

        bitfield_unit.set(8usize, 1u8, ProcessEnableWriteExceptions as u64);

        bitfield_unit.set(9usize, 1u8, ThreadAllowWrites as u64);

        bitfield_unit.set(10usize, 22u8, Spare as u64);

        bitfield_unit
    }
}

#[repr(C)]
pub struct PROCESS_READWRITEVM_LOGGING_INFORMATION {
    pub Flags: UnionField<u8>,
    pub Anonymous1: UnionField<PROCESS_READWRITEVM_LOGGING_INFORMATION_1>,
    pub union_field: u8,
}

#[repr(C, packed)]
pub struct PROCESS_READWRITEVM_LOGGING_INFORMATION_1 {
    _bitfield_align_1: [u8; 0],
    _bitfield_1: BitfieldUnit<[u8; 1]>,
}

impl Default for PROCESS_READWRITEVM_LOGGING_INFORMATION_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PROCESS_READWRITEVM_LOGGING_INFORMATION_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PROCESS_READWRITEVM_LOGGING_INFORMATION_1 {{ EnableReadVmLogging : {:?}, EnableWriteVmLogging : {:?}, Unused : {:?} }}",
            self.EnableReadVmLogging(),
            self.EnableWriteVmLogging(),
            self.Unused()
        )
    }
}

impl PROCESS_READWRITEVM_LOGGING_INFORMATION_1 {
    #[inline]
    pub fn EnableReadVmLogging(&self) -> u8 {
        self._bitfield_1.get(0usize, 1u8) as u8
    }

    #[inline]
    pub fn set_EnableReadVmLogging(&mut self, val: u8) {
        self._bitfield_1.set(0usize, 1u8, val as u64)
    }

    #[inline]
    pub fn EnableWriteVmLogging(&self) -> u8 {
        self._bitfield_1.get(1usize, 1u8) as u8
    }

    #[inline]
    pub fn set_EnableWriteVmLogging(&mut self, val: u8) {
        self._bitfield_1.set(1usize, 1u8, val as u64)
    }

    #[inline]
    pub fn Unused(&self) -> u8 {
        self._bitfield_1.get(2usize, 6u8) as u8
    }

    #[inline]
    pub fn set_Unused(&mut self, val: u8) {
        self._bitfield_1.set(2usize, 6u8, val as u64)
    }

    #[inline]
    pub fn new_bitfield_1(
        EnableReadVmLogging: u8,
        EnableWriteVmLogging: u8,
        Unused: u8,
    ) -> BitfieldUnit<[u8; 1]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 1]> = Default::default();

        bitfield_unit.set(0usize, 1u8, EnableReadVmLogging as u64);

        bitfield_unit.set(1usize, 1u8, EnableWriteVmLogging as u64);

        bitfield_unit.set(2usize, 6u8, Unused as u64);

        bitfield_unit
    }
}

impl Default for PROCESS_READWRITEVM_LOGGING_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PROCESS_READWRITEVM_LOGGING_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PROCESS_READWRITEVM_LOGGING_INFORMATION {{ union }}")
    }
}

#[repr(C)]
pub struct PROCESS_UPTIME_INFORMATION {
    pub QueryInterruptTime: u64,
    pub QueryUnbiasedTime: u64,
    pub EndInterruptTime: u64,
    pub TimeSinceCreation: u64,
    pub Uptime: u64,
    pub SuspendedTime: u64,
    pub Anonymous1: PROCESS_UPTIME_INFORMATION_1,
}

#[repr(C)]
#[repr(align(1))]

pub union PROCESS_UPTIME_INFORMATION_1 {
    _bitfield_align_1: [u8; 0],
    _bitfield_1: BitfieldUnit<[u8; 2]>,
}

impl Default for PROCESS_UPTIME_INFORMATION_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PROCESS_UPTIME_INFORMATION_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PROCESS_UPTIME_INFORMATION_1 {{ union }}")
    }
}

impl PROCESS_UPTIME_INFORMATION_1 {
    #[inline]
    pub fn HangCount(&self) -> u32 {
        unsafe { self._bitfield_1.get(0usize, 4u8) as u32 }
    }

    #[inline]
    pub fn set_HangCount(&mut self, val: u32) {
        unsafe { self._bitfield_1.set(0usize, 4u8, val as u64) }
    }

    #[inline]
    pub fn GhostCount(&self) -> u32 {
        unsafe { self._bitfield_1.get(4usize, 4u8) as u32 }
    }

    #[inline]
    pub fn set_GhostCount(&mut self, val: u32) {
        unsafe { self._bitfield_1.set(4usize, 4u8, val as u64) }
    }

    #[inline]
    pub fn Crashed(&self) -> u32 {
        unsafe { self._bitfield_1.get(8usize, 1u8) as u32 }
    }

    #[inline]
    pub fn set_Crashed(&mut self, val: u32) {
        unsafe { self._bitfield_1.set(8usize, 1u8, val as u64) }
    }

    #[inline]
    pub fn Terminated(&self) -> u32 {
        unsafe { self._bitfield_1.get(9usize, 1u8) as u32 }
    }

    #[inline]
    pub fn set_Terminated(&mut self, val: u32) {
        unsafe { self._bitfield_1.set(9usize, 1u8, val as u64) }
    }

    #[inline]
    pub fn new_bitfield_1(
        HangCount: u32,
        GhostCount: u32,
        Crashed: u32,
        Terminated: u32,
    ) -> BitfieldUnit<[u8; 2]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 2]> = Default::default();

        bitfield_unit.set(0usize, 4u8, HangCount as u64);

        bitfield_unit.set(4usize, 4u8, GhostCount as u64);

        bitfield_unit.set(8usize, 1u8, Crashed as u64);

        bitfield_unit.set(9usize, 1u8, Terminated as u64);

        bitfield_unit
    }
}

impl Default for PROCESS_UPTIME_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PROCESS_UPTIME_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PROCESS_UPTIME_INFORMATION {{ Anonymous1: {:?} }}",
            self.Anonymous1
        )
    }
}

#[repr(C)]
pub struct PROCESS_SYSTEM_RESOURCE_MANAGEMENT {
    pub Flags: UnionField<u32>,
    pub Anonymous1: UnionField<PROCESS_SYSTEM_RESOURCE_MANAGEMENT_1>,
    pub union_field: u32,
}

#[repr(C)]
#[repr(align(4))]
pub struct PROCESS_SYSTEM_RESOURCE_MANAGEMENT_1 {
    _bitfield_align_1: [u32; 0],
    _bitfield_1: BitfieldUnit<[u8; 4]>,
}

impl Default for PROCESS_SYSTEM_RESOURCE_MANAGEMENT_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PROCESS_SYSTEM_RESOURCE_MANAGEMENT_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PROCESS_SYSTEM_RESOURCE_MANAGEMENT_1 {{ Foreground : {:?}, Reserved : {:?} }}",
            self.Foreground(),
            self.Reserved()
        )
    }
}

impl PROCESS_SYSTEM_RESOURCE_MANAGEMENT_1 {
    #[inline]
    pub fn Foreground(&self) -> u32 {
        self._bitfield_1.get(0usize, 1u8) as u32
    }

    #[inline]
    pub fn set_Foreground(&mut self, val: u32) {
        self._bitfield_1.set(0usize, 1u8, val as u64)
    }

    #[inline]
    pub fn Reserved(&self) -> u32 {
        self._bitfield_1.get(1usize, 31u8) as u32
    }

    #[inline]
    pub fn set_Reserved(&mut self, val: u32) {
        self._bitfield_1.set(1usize, 31u8, val as u64)
    }

    #[inline]
    pub fn new_bitfield_1(Foreground: u32, Reserved: u32) -> BitfieldUnit<[u8; 4]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 4]> = Default::default();

        bitfield_unit.set(0usize, 1u8, Foreground as u64);

        bitfield_unit.set(1usize, 31u8, Reserved as u64);

        bitfield_unit
    }
}

impl Default for PROCESS_SYSTEM_RESOURCE_MANAGEMENT {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PROCESS_SYSTEM_RESOURCE_MANAGEMENT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PROCESS_SYSTEM_RESOURCE_MANAGEMENT {{ union }}")
    }
}

#[repr(C)]
pub struct PROCESS_SECURITY_DOMAIN_INFORMATION {
    pub SecurityDomain: u64,
}

impl Default for PROCESS_SECURITY_DOMAIN_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PROCESS_SECURITY_DOMAIN_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PROCESS_SECURITY_DOMAIN_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct PROCESS_COMBINE_SECURITY_DOMAINS_INFORMATION {
    pub ProcessHandle: HANDLE,
}

impl Default for PROCESS_COMBINE_SECURITY_DOMAINS_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PROCESS_COMBINE_SECURITY_DOMAINS_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PROCESS_COMBINE_SECURITY_DOMAINS_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct PROCESS_LOGGING_INFORMATION {
    pub Flags: UnionField<u32>,
    pub Anonymous1: UnionField<PROCESS_LOGGING_INFORMATION_1>,
    pub union_field: u32,
}

#[repr(C)]
#[repr(align(4))]
pub struct PROCESS_LOGGING_INFORMATION_1 {
    _bitfield_align_1: [u32; 0],
    _bitfield_1: BitfieldUnit<[u8; 4]>,
}

impl Default for PROCESS_LOGGING_INFORMATION_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PROCESS_LOGGING_INFORMATION_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PROCESS_LOGGING_INFORMATION_1 {{ EnableReadVmLogging : {:?}, EnableWriteVmLogging : {:?}, EnableProcessSuspendResumeLogging : {:?}, EnableThreadSuspendResumeLogging : {:?}, EnableLocalExecProtectVmLogging : {:?}, EnableRemoteExecProtectVmLogging : {:?}, Reserved : {:?} }}",
            self.EnableReadVmLogging(),
            self.EnableWriteVmLogging(),
            self.EnableProcessSuspendResumeLogging(),
            self.EnableThreadSuspendResumeLogging(),
            self.EnableLocalExecProtectVmLogging(),
            self.EnableRemoteExecProtectVmLogging(),
            self.Reserved()
        )
    }
}

impl PROCESS_LOGGING_INFORMATION_1 {
    #[inline]
    pub fn EnableReadVmLogging(&self) -> u32 {
        self._bitfield_1.get(0usize, 1u8) as u32
    }

    #[inline]
    pub fn set_EnableReadVmLogging(&mut self, val: u32) {
        self._bitfield_1.set(0usize, 1u8, val as u64)
    }

    #[inline]
    pub fn EnableWriteVmLogging(&self) -> u32 {
        self._bitfield_1.get(1usize, 1u8) as u32
    }

    #[inline]
    pub fn set_EnableWriteVmLogging(&mut self, val: u32) {
        self._bitfield_1.set(1usize, 1u8, val as u64)
    }

    #[inline]
    pub fn EnableProcessSuspendResumeLogging(&self) -> u32 {
        self._bitfield_1.get(2usize, 1u8) as u32
    }

    #[inline]
    pub fn set_EnableProcessSuspendResumeLogging(&mut self, val: u32) {
        self._bitfield_1.set(2usize, 1u8, val as u64)
    }

    #[inline]
    pub fn EnableThreadSuspendResumeLogging(&self) -> u32 {
        self._bitfield_1.get(3usize, 1u8) as u32
    }

    #[inline]
    pub fn set_EnableThreadSuspendResumeLogging(&mut self, val: u32) {
        self._bitfield_1.set(3usize, 1u8, val as u64)
    }

    #[inline]
    pub fn EnableLocalExecProtectVmLogging(&self) -> u32 {
        self._bitfield_1.get(4usize, 1u8) as u32
    }

    #[inline]
    pub fn set_EnableLocalExecProtectVmLogging(&mut self, val: u32) {
        self._bitfield_1.set(4usize, 1u8, val as u64)
    }

    #[inline]
    pub fn EnableRemoteExecProtectVmLogging(&self) -> u32 {
        self._bitfield_1.get(5usize, 1u8) as u32
    }

    #[inline]
    pub fn set_EnableRemoteExecProtectVmLogging(&mut self, val: u32) {
        self._bitfield_1.set(5usize, 1u8, val as u64)
    }

    #[inline]
    pub fn Reserved(&self) -> u32 {
        self._bitfield_1.get(6usize, 26u8) as u32
    }

    #[inline]
    pub fn set_Reserved(&mut self, val: u32) {
        self._bitfield_1.set(6usize, 26u8, val as u64)
    }

    #[inline]
    pub fn new_bitfield_1(
        EnableReadVmLogging: u32,
        EnableWriteVmLogging: u32,
        EnableProcessSuspendResumeLogging: u32,
        EnableThreadSuspendResumeLogging: u32,
        EnableLocalExecProtectVmLogging: u32,
        EnableRemoteExecProtectVmLogging: u32,
        Reserved: u32,
    ) -> BitfieldUnit<[u8; 4]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 4]> = Default::default();

        bitfield_unit.set(0usize, 1u8, EnableReadVmLogging as u64);

        bitfield_unit.set(1usize, 1u8, EnableWriteVmLogging as u64);

        bitfield_unit.set(2usize, 1u8, EnableProcessSuspendResumeLogging as u64);

        bitfield_unit.set(3usize, 1u8, EnableThreadSuspendResumeLogging as u64);

        bitfield_unit.set(4usize, 1u8, EnableLocalExecProtectVmLogging as u64);

        bitfield_unit.set(5usize, 1u8, EnableRemoteExecProtectVmLogging as u64);

        bitfield_unit.set(6usize, 26u8, Reserved as u64);

        bitfield_unit
    }
}

impl Default for PROCESS_LOGGING_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PROCESS_LOGGING_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PROCESS_LOGGING_INFORMATION {{ union }}")
    }
}

#[repr(C)]
pub struct PROCESS_LEAP_SECOND_INFORMATION {
    pub Flags: u32,
    pub Reserved: u32,
}

impl Default for PROCESS_LEAP_SECOND_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PROCESS_LEAP_SECOND_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PROCESS_LEAP_SECOND_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct PROCESS_FIBER_SHADOW_STACK_ALLOCATION_INFORMATION {
    pub ReserveSize: u64,
    pub CommitSize: u64,
    pub PreferredNode: u32,
    pub Reserved: u32,
    pub Ssp: *mut std::ffi::c_void,
}

impl Default for PROCESS_FIBER_SHADOW_STACK_ALLOCATION_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PROCESS_FIBER_SHADOW_STACK_ALLOCATION_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PROCESS_FIBER_SHADOW_STACK_ALLOCATION_INFORMATION {{  }}"
        )
    }
}

#[repr(C)]
pub struct PROCESS_FREE_FIBER_SHADOW_STACK_ALLOCATION_INFORMATION {
    pub Ssp: *mut std::ffi::c_void,
}

impl Default for PROCESS_FREE_FIBER_SHADOW_STACK_ALLOCATION_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PROCESS_FREE_FIBER_SHADOW_STACK_ALLOCATION_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PROCESS_FREE_FIBER_SHADOW_STACK_ALLOCATION_INFORMATION {{  }}"
        )
    }
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtQueryPortInformationProcess() -> NTSTATUS;
}

#[repr(C)]
pub struct THREAD_BASIC_INFORMATION {
    pub ExitStatus: NTSTATUS,
    pub TebBaseAddress: *mut TEB,
    pub ClientId: CLIENT_ID,
    pub AffinityMask: usize,
    pub Priority: i32,
    pub BasePriority: i32,
}

impl Default for THREAD_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for THREAD_BASIC_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "THREAD_BASIC_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct THREAD_LAST_SYSCALL_INFORMATION {
    pub FirstArgument: *mut std::ffi::c_void,
    pub SystemCallNumber: u16,
    pub Pad: [u16; 1],
    pub WaitTime: u64,
}

impl Default for THREAD_LAST_SYSCALL_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for THREAD_LAST_SYSCALL_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "THREAD_LAST_SYSCALL_INFORMATION {{ Pad: {:?} }}",
            self.Pad
        )
    }
}

#[repr(C)]
pub struct THREAD_CYCLE_TIME_INFORMATION {
    pub AccumulatedCycles: u64,
    pub CurrentCycleCount: u64,
}

impl Default for THREAD_CYCLE_TIME_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for THREAD_CYCLE_TIME_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "THREAD_CYCLE_TIME_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct THREAD_TEB_INFORMATION {
    pub TebInformation: *mut std::ffi::c_void,
    pub TebOffset: u32,
    pub BytesToRead: u32,
}

impl Default for THREAD_TEB_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for THREAD_TEB_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "THREAD_TEB_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct COUNTER_READING {
    pub Type: HARDWARE_COUNTER_TYPE,
    pub Index: u32,
    pub Start: u64,
    pub Total: u64,
}

impl Default for COUNTER_READING {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for COUNTER_READING {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "COUNTER_READING {{  }}")
    }
}

#[repr(C)]
pub struct THREAD_PERFORMANCE_DATA {
    pub Size: u16,
    pub Version: u16,
    pub ProcessorNumber: PROCESSOR_NUMBER,
    pub ContextSwitches: u32,
    pub HwCountersCount: u32,
    pub UpdateCount: u64,
    pub WaitReasonBitMap: u64,
    pub HardwareCounters: u64,
    pub CycleTime: COUNTER_READING,
    pub HwCounters: [COUNTER_READING; 16],
}

impl Default for THREAD_PERFORMANCE_DATA {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for THREAD_PERFORMANCE_DATA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "THREAD_PERFORMANCE_DATA {{ CycleTime: {:?}, HwCounters: {:?} }}",
            self.CycleTime, self.HwCounters
        )
    }
}

#[repr(C)]
pub struct THREAD_PROFILING_INFORMATION {
    pub HardwareCounters: u64,
    pub Flags: u32,
    pub Enable: u32,
    pub PerformanceData: *mut THREAD_PERFORMANCE_DATA,
}

impl Default for THREAD_PROFILING_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for THREAD_PROFILING_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "THREAD_PROFILING_INFORMATION {{ PerformanceData: {:?} }}",
            self.PerformanceData
        )
    }
}

#[repr(C)]
#[repr(align(16))]
pub struct RTL_UMS_CONTEXT {
    pub Link: SINGLE_LIST_ENTRY,
    pub padding_0: u64,
    pub Context: CONTEXT,
    pub Teb: *mut std::ffi::c_void,
    pub UserContext: *mut std::ffi::c_void,
    _bitfield_align_1: [u8; 0],
    _bitfield_1: BitfieldUnit<[u8; 1]>,
    pub Flags: i32,
    _bitfield_align_2: [u64; 0],
    _bitfield_2: BitfieldUnit<[u8; 8]>,
    pub ContextLock: u64,
    pub PrimaryUmsContext: *mut RTL_UMS_CONTEXT,
    pub SwitchCount: u32,
    pub KernelYieldCount: u32,
    pub MixedYieldCount: u32,
    pub YieldCount: u32,
}

impl Default for RTL_UMS_CONTEXT {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_UMS_CONTEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RTL_UMS_CONTEXT {{ ScheduledThread : {:?}, Suspended : {:?}, VolatileContext : {:?}, Terminated : {:?}, DebugActive : {:?}, RunningOnSelfThread : {:?}, DenyRunningOnSelfThread : {:?}, KernelUpdateLock : {:?}, PrimaryClientID : {:?}, PrimaryUmsContext: {:?} }}",
            self.ScheduledThread(),
            self.Suspended(),
            self.VolatileContext(),
            self.Terminated(),
            self.DebugActive(),
            self.RunningOnSelfThread(),
            self.DenyRunningOnSelfThread(),
            self.KernelUpdateLock(),
            self.PrimaryClientID(),
            self.PrimaryUmsContext
        )
    }
}

impl RTL_UMS_CONTEXT {
    #[inline]
    pub fn ScheduledThread(&self) -> u32 {
        self._bitfield_1.get(0usize, 1u8) as u32
    }

    #[inline]
    pub fn set_ScheduledThread(&mut self, val: u32) {
        self._bitfield_1.set(0usize, 1u8, val as u64)
    }

    #[inline]
    pub fn Suspended(&self) -> u32 {
        self._bitfield_1.get(1usize, 1u8) as u32
    }

    #[inline]
    pub fn set_Suspended(&mut self, val: u32) {
        self._bitfield_1.set(1usize, 1u8, val as u64)
    }

    #[inline]
    pub fn VolatileContext(&self) -> u32 {
        self._bitfield_1.get(2usize, 1u8) as u32
    }

    #[inline]
    pub fn set_VolatileContext(&mut self, val: u32) {
        self._bitfield_1.set(2usize, 1u8, val as u64)
    }

    #[inline]
    pub fn Terminated(&self) -> u32 {
        self._bitfield_1.get(3usize, 1u8) as u32
    }

    #[inline]
    pub fn set_Terminated(&mut self, val: u32) {
        self._bitfield_1.set(3usize, 1u8, val as u64)
    }

    #[inline]
    pub fn DebugActive(&self) -> u32 {
        self._bitfield_1.get(4usize, 1u8) as u32
    }

    #[inline]
    pub fn set_DebugActive(&mut self, val: u32) {
        self._bitfield_1.set(4usize, 1u8, val as u64)
    }

    #[inline]
    pub fn RunningOnSelfThread(&self) -> u32 {
        self._bitfield_1.get(5usize, 1u8) as u32
    }

    #[inline]
    pub fn set_RunningOnSelfThread(&mut self, val: u32) {
        self._bitfield_1.set(5usize, 1u8, val as u64)
    }

    #[inline]
    pub fn DenyRunningOnSelfThread(&self) -> u32 {
        self._bitfield_1.get(6usize, 1u8) as u32
    }

    #[inline]
    pub fn set_DenyRunningOnSelfThread(&mut self, val: u32) {
        self._bitfield_1.set(6usize, 1u8, val as u64)
    }

    #[inline]
    pub fn new_bitfield_1(
        ScheduledThread: u32,
        Suspended: u32,
        VolatileContext: u32,
        Terminated: u32,
        DebugActive: u32,
        RunningOnSelfThread: u32,
        DenyRunningOnSelfThread: u32,
    ) -> BitfieldUnit<[u8; 1]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 1]> = Default::default();

        bitfield_unit.set(0usize, 1u8, ScheduledThread as u64);

        bitfield_unit.set(1usize, 1u8, Suspended as u64);

        bitfield_unit.set(2usize, 1u8, VolatileContext as u64);

        bitfield_unit.set(3usize, 1u8, Terminated as u64);

        bitfield_unit.set(4usize, 1u8, DebugActive as u64);

        bitfield_unit.set(5usize, 1u8, RunningOnSelfThread as u64);

        bitfield_unit.set(6usize, 1u8, DenyRunningOnSelfThread as u64);

        bitfield_unit
    }

    #[inline]
    pub fn KernelUpdateLock(&self) -> u64 {
        self._bitfield_2.get(0usize, 2u8)
    }

    #[inline]
    pub fn set_KernelUpdateLock(&mut self, val: u64) {
        self._bitfield_2.set(0usize, 2u8, val)
    }

    #[inline]
    pub fn PrimaryClientID(&self) -> u64 {
        self._bitfield_2.get(2usize, 62u8)
    }

    #[inline]
    pub fn set_PrimaryClientID(&mut self, val: u64) {
        self._bitfield_2.set(2usize, 62u8, val)
    }

    #[inline]
    pub fn new_bitfield_2(KernelUpdateLock: u64, PrimaryClientID: u64) -> BitfieldUnit<[u8; 8]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 8]> = Default::default();

        bitfield_unit.set(0usize, 2u8, KernelUpdateLock);

        bitfield_unit.set(2usize, 62u8, PrimaryClientID);

        bitfield_unit
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum THREAD_UMS_INFORMATION_COMMAND {
    UmsInformationCommandInvalid = 0,
    UmsInformationCommandAttach = 1,
    UmsInformationCommandDetach = 2,
    UmsInformationCommandQuery = 3,
}

#[repr(C)]
pub struct RTL_UMS_COMPLETION_LIST {
    pub ThreadListHead: *mut SINGLE_LIST_ENTRY,
    pub CompletionEvent: *mut std::ffi::c_void,
    pub CompletionFlags: u32,
    pub InternalListHead: SINGLE_LIST_ENTRY,
}

impl Default for RTL_UMS_COMPLETION_LIST {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_UMS_COMPLETION_LIST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RTL_UMS_COMPLETION_LIST {{  }}")
    }
}

#[repr(C)]
pub struct THREAD_UMS_INFORMATION {
    pub Command: THREAD_UMS_INFORMATION_COMMAND,
    pub CompletionList: *mut RTL_UMS_COMPLETION_LIST,
    pub UmsContext: *mut RTL_UMS_CONTEXT,
    pub Anonymous1: THREAD_UMS_INFORMATION_1,
}

#[repr(C)]
pub struct THREAD_UMS_INFORMATION_1 {
    pub Flags: UnionField<u32>,
    pub Anonymous1: UnionField<THREAD_UMS_INFORMATION_1_1>,
    pub union_field: u32,
}

#[repr(C)]
#[repr(align(4))]
pub struct THREAD_UMS_INFORMATION_1_1 {
    _bitfield_align_1: [u32; 0],
    _bitfield_1: BitfieldUnit<[u8; 4]>,
}

impl Default for THREAD_UMS_INFORMATION_1_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for THREAD_UMS_INFORMATION_1_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "THREAD_UMS_INFORMATION_1_1 {{ IsUmsSchedulerThread : {:?}, IsUmsWorkerThread : {:?}, SpareBits : {:?} }}",
            self.IsUmsSchedulerThread(),
            self.IsUmsWorkerThread(),
            self.SpareBits()
        )
    }
}

impl THREAD_UMS_INFORMATION_1_1 {
    #[inline]
    pub fn IsUmsSchedulerThread(&self) -> u32 {
        self._bitfield_1.get(0usize, 1u8) as u32
    }

    #[inline]
    pub fn set_IsUmsSchedulerThread(&mut self, val: u32) {
        self._bitfield_1.set(0usize, 1u8, val as u64)
    }

    #[inline]
    pub fn IsUmsWorkerThread(&self) -> u32 {
        self._bitfield_1.get(1usize, 1u8) as u32
    }

    #[inline]
    pub fn set_IsUmsWorkerThread(&mut self, val: u32) {
        self._bitfield_1.set(1usize, 1u8, val as u64)
    }

    #[inline]
    pub fn SpareBits(&self) -> u32 {
        self._bitfield_1.get(2usize, 30u8) as u32
    }

    #[inline]
    pub fn set_SpareBits(&mut self, val: u32) {
        self._bitfield_1.set(2usize, 30u8, val as u64)
    }

    #[inline]
    pub fn new_bitfield_1(
        IsUmsSchedulerThread: u32,
        IsUmsWorkerThread: u32,
        SpareBits: u32,
    ) -> BitfieldUnit<[u8; 4]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 4]> = Default::default();

        bitfield_unit.set(0usize, 1u8, IsUmsSchedulerThread as u64);

        bitfield_unit.set(1usize, 1u8, IsUmsWorkerThread as u64);

        bitfield_unit.set(2usize, 30u8, SpareBits as u64);

        bitfield_unit
    }
}

impl Default for THREAD_UMS_INFORMATION_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for THREAD_UMS_INFORMATION_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "THREAD_UMS_INFORMATION_1 {{ union }}")
    }
}

impl Default for THREAD_UMS_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for THREAD_UMS_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "THREAD_UMS_INFORMATION {{ Command: {:?}, CompletionList: {:?}, UmsContext: {:?}, Anonymous1: {:?} }}",
            self.Command, self.CompletionList, self.UmsContext, self.Anonymous1
        )
    }
}

#[repr(C)]
pub struct ALPC_WORK_ON_BEHALF_TICKET {
    pub ThreadId: u32,
    pub ThreadCreationTimeLow: u32,
}

impl Default for ALPC_WORK_ON_BEHALF_TICKET {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ALPC_WORK_ON_BEHALF_TICKET {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ALPC_WORK_ON_BEHALF_TICKET {{  }}")
    }
}

#[repr(C)]
pub struct RTL_WORK_ON_BEHALF_TICKET_EX {
    pub Ticket: ALPC_WORK_ON_BEHALF_TICKET,
    pub Anonymous1: RTL_WORK_ON_BEHALF_TICKET_EX_1,
    pub Reserved2: u32,
}

#[repr(C)]
pub struct RTL_WORK_ON_BEHALF_TICKET_EX_1 {
    pub Flags: UnionField<u32>,
    pub Anonymous1: UnionField<RTL_WORK_ON_BEHALF_TICKET_EX_1_1>,
    pub union_field: u32,
}

#[repr(C)]
#[repr(align(4))]
pub struct RTL_WORK_ON_BEHALF_TICKET_EX_1_1 {
    _bitfield_align_1: [u32; 0],
    _bitfield_1: BitfieldUnit<[u8; 4]>,
}

impl Default for RTL_WORK_ON_BEHALF_TICKET_EX_1_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_WORK_ON_BEHALF_TICKET_EX_1_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RTL_WORK_ON_BEHALF_TICKET_EX_1_1 {{ CurrentThread : {:?}, Reserved1 : {:?} }}",
            self.CurrentThread(),
            self.Reserved1()
        )
    }
}

impl RTL_WORK_ON_BEHALF_TICKET_EX_1_1 {
    #[inline]
    pub fn CurrentThread(&self) -> u32 {
        self._bitfield_1.get(0usize, 1u8) as u32
    }

    #[inline]
    pub fn set_CurrentThread(&mut self, val: u32) {
        self._bitfield_1.set(0usize, 1u8, val as u64)
    }

    #[inline]
    pub fn Reserved1(&self) -> u32 {
        self._bitfield_1.get(1usize, 31u8) as u32
    }

    #[inline]
    pub fn set_Reserved1(&mut self, val: u32) {
        self._bitfield_1.set(1usize, 31u8, val as u64)
    }

    #[inline]
    pub fn new_bitfield_1(CurrentThread: u32, Reserved1: u32) -> BitfieldUnit<[u8; 4]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 4]> = Default::default();

        bitfield_unit.set(0usize, 1u8, CurrentThread as u64);

        bitfield_unit.set(1usize, 31u8, Reserved1 as u64);

        bitfield_unit
    }
}

impl Default for RTL_WORK_ON_BEHALF_TICKET_EX_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_WORK_ON_BEHALF_TICKET_EX_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RTL_WORK_ON_BEHALF_TICKET_EX_1 {{ union }}")
    }
}

impl Default for RTL_WORK_ON_BEHALF_TICKET_EX {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_WORK_ON_BEHALF_TICKET_EX {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RTL_WORK_ON_BEHALF_TICKET_EX {{ Ticket: {:?}, Anonymous1: {:?} }}",
            self.Ticket, self.Anonymous1
        )
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum THREAD_WORKLOAD_CLASS {
    ThreadWorkloadClassDefault = 0,
    ThreadWorkloadClassGraphics = 1,
    MaxThreadWorkloadClass = 2,
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtCreateProcess(
        ProcessHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        ParentProcess: HANDLE,
        InheritObjectTable: bool,
        SectionHandle: HANDLE,
        DebugPort: HANDLE,
        TokenHandle: HANDLE,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtCreateProcessEx(
        ProcessHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        ParentProcess: HANDLE,
        Flags: u32,
        SectionHandle: HANDLE,
        DebugPort: HANDLE,
        TokenHandle: HANDLE,
        Reserved: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtTerminateProcess(ProcessHandle: HANDLE, ExitStatus: NTSTATUS) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtSuspendProcess(ProcessHandle: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtResumeProcess(ProcessHandle: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtGetNextProcess(
        ProcessHandle: HANDLE,
        DesiredAccess: u32,
        HandleAttributes: u32,
        Flags: u32,
        NewProcessHandle: *mut HANDLE,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtGetNextThread(
        ProcessHandle: HANDLE,
        ThreadHandle: HANDLE,
        DesiredAccess: u32,
        HandleAttributes: u32,
        Flags: u32,
        NewThreadHandle: *mut HANDLE,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtSetInformationProcess(
        ProcessHandle: HANDLE,
        ProcessInformationClass: PROCESSINFOCLASS,
        ProcessInformation: *mut std::ffi::c_void,
        ProcessInformationLength: u32,
    ) -> NTSTATUS;
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum PROCESS_STATE_CHANGE_TYPE {
    ProcessStateChangeSuspend = 0,
    ProcessStateChangeResume = 1,
    ProcessStateChangeMax = 2,
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtCreateProcessStateChange(
        ProcessStateChangeHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        ProcessHandle: HANDLE,
        Reserved: u64,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtChangeProcessState(
        ProcessStateChangeHandle: HANDLE,
        ProcessHandle: HANDLE,
        StateChangeType: PROCESS_STATE_CHANGE_TYPE,
        ExtendedInformation: *mut std::ffi::c_void,
        ExtendedInformationLength: usize,
        Reserved: u64,
    ) -> NTSTATUS;
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum THREAD_STATE_CHANGE_TYPE {
    ThreadStateChangeSuspend = 0,
    ThreadStateChangeResume = 1,
    ThreadStateChangeMax = 2,
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtCreateThreadStateChange(
        ThreadStateChangeHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        ThreadHandle: HANDLE,
        Reserved: u64,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtChangeThreadState(
        ThreadStateChangeHandle: HANDLE,
        ThreadHandle: HANDLE,
        StateChangeType: THREAD_STATE_CHANGE_TYPE,
        ExtendedInformation: *mut std::ffi::c_void,
        ExtendedInformationLength: usize,
        Reserved: u64,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtCreateThread(
        ThreadHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        ProcessHandle: HANDLE,
        ClientId: *mut CLIENT_ID,
        ThreadContext: *mut CONTEXT,
        InitialTeb: *mut INITIAL_TEB,
        CreateSuspended: bool,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtOpenThread(
        ThreadHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        ClientId: *mut CLIENT_ID,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtTerminateThread(ThreadHandle: HANDLE, ExitStatus: NTSTATUS) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtSuspendThread(ThreadHandle: HANDLE, PreviousSuspendCount: *mut u32) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtResumeThread(ThreadHandle: HANDLE, PreviousSuspendCount: *mut u32) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtGetCurrentProcessorNumber() -> u32;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtGetCurrentProcessorNumberEx(ProcessorNumber: *mut PROCESSOR_NUMBER) -> u32;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtGetContextThread(ThreadHandle: HANDLE, ThreadContext: *mut CONTEXT) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtSetContextThread(ThreadHandle: HANDLE, ThreadContext: *mut CONTEXT) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtAlertThread(ThreadHandle: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtAlertResumeThread(ThreadHandle: HANDLE, PreviousSuspendCount: *mut u32) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtTestAlert() -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtImpersonateThread(
        ServerThreadHandle: HANDLE,
        ClientThreadHandle: HANDLE,
        SecurityQos: *mut SECURITY_QUALITY_OF_SERVICE,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtRegisterThreadTerminatePort(PortHandle: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtSetLdtEntries(
        Selector0: u32,
        Entry0Low: u32,
        Entry0Hi: u32,
        Selector1: u32,
        Entry1Low: u32,
        Entry1Hi: u32,
    ) -> NTSTATUS;
}

pub type PPS_APC_ROUTINE = std::option::Option<
    unsafe extern "system" fn(
        ApcArgument1: *mut std::ffi::c_void,
        ApcArgument2: *mut std::ffi::c_void,
        ApcArgument3: *mut std::ffi::c_void,
    ),
>;

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtQueueApcThread(
        ThreadHandle: HANDLE,
        ApcRoutine: PPS_APC_ROUTINE,
        ApcArgument1: *mut std::ffi::c_void,
        ApcArgument2: *mut std::ffi::c_void,
        ApcArgument3: *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtQueueApcThreadEx(
        ThreadHandle: HANDLE,
        ReserveHandle: HANDLE,
        ApcRoutine: PPS_APC_ROUTINE,
        ApcArgument1: *mut std::ffi::c_void,
        ApcArgument2: *mut std::ffi::c_void,
        ApcArgument3: *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtQueueApcThreadEx2(
        ThreadHandle: HANDLE,
        ReserveHandle: HANDLE,
        ApcFlags: u32,
        ApcRoutine: PPS_APC_ROUTINE,
        ApcArgument1: *mut std::ffi::c_void,
        ApcArgument2: *mut std::ffi::c_void,
        ApcArgument3: *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtAlertThreadByThreadId(ThreadId: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtWaitForAlertByThreadId(Address: *mut std::ffi::c_void, Timeout: *mut i64) -> NTSTATUS;
}

#[repr(C)]
pub struct PROC_THREAD_ATTRIBUTE {
    pub Attribute: usize,
    pub Size: usize,
    pub Value: usize,
}

impl Default for PROC_THREAD_ATTRIBUTE {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PROC_THREAD_ATTRIBUTE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PROC_THREAD_ATTRIBUTE {{  }}")
    }
}

#[repr(C)]
pub struct PROC_THREAD_ATTRIBUTE_LIST {
    pub PresentFlags: u32,
    pub AttributeCount: u32,
    pub LastAttribute: u32,
    pub SpareUlong0: u32,
    pub ExtendedFlagsAttribute: *mut PROC_THREAD_ATTRIBUTE,
    pub Attributes: [PROC_THREAD_ATTRIBUTE; 1],
}

impl Default for PROC_THREAD_ATTRIBUTE_LIST {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PROC_THREAD_ATTRIBUTE_LIST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PROC_THREAD_ATTRIBUTE_LIST {{ ExtendedFlagsAttribute: {:?}, Attributes: {:?} }}",
            self.ExtendedFlagsAttribute, self.Attributes
        )
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SE_SAFE_OPEN_PROMPT_EXPERIENCE_RESULTS {
    SeSafeOpenExperienceNone = 0,
    SeSafeOpenExperienceCalled = 1,
    SeSafeOpenExperienceAppRepCalled = 2,
    SeSafeOpenExperiencePromptDisplayed = 4,
    SeSafeOpenExperienceUAC = 8,
    SeSafeOpenExperienceUninstaller = 16,
    SeSafeOpenExperienceIgnoreUnknownOrBad = 32,
    SeSafeOpenExperienceDefenderTrustedInstaller = 64,
    SeSafeOpenExperienceMOTWPresent = 128,
    SeSafeOpenExperienceElevatedNoPropagation = 256,
}

#[repr(C)]
pub struct SE_SAFE_OPEN_PROMPT_RESULTS {
    pub Results: SE_SAFE_OPEN_PROMPT_EXPERIENCE_RESULTS,
    pub Path: [u16; 260],
}

impl Default for SE_SAFE_OPEN_PROMPT_RESULTS {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for SE_SAFE_OPEN_PROMPT_RESULTS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SE_SAFE_OPEN_PROMPT_RESULTS {{ Results: {:?}, Path: {:?} }}",
            self.Results, self.Path
        )
    }
}

#[repr(C)]
pub struct PROC_THREAD_BNOISOLATION_ATTRIBUTE {
    pub IsolationEnabled: bool,
    pub IsolationPrefix: [u16; 136],
}

impl Default for PROC_THREAD_BNOISOLATION_ATTRIBUTE {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PROC_THREAD_BNOISOLATION_ATTRIBUTE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PROC_THREAD_BNOISOLATION_ATTRIBUTE {{ IsolationPrefix: {:?} }}",
            self.IsolationPrefix
        )
    }
}

#[repr(C)]
pub struct ISOLATION_MANIFEST_PROPERTIES {
    pub InstancePath: UNICODE_STRING,
    pub FriendlyName: UNICODE_STRING,
    pub Description: UNICODE_STRING,
    pub Level: usize,
}

impl Default for ISOLATION_MANIFEST_PROPERTIES {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ISOLATION_MANIFEST_PROPERTIES {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ISOLATION_MANIFEST_PROPERTIES {{  }}")
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum PS_ATTRIBUTE_NUM {
    PsAttributeParentProcess = 0,
    PsAttributeDebugObject = 1,
    PsAttributeToken = 2,
    PsAttributeClientId = 3,
    PsAttributeTebAddress = 4,
    PsAttributeImageName = 5,
    PsAttributeImageInfo = 6,
    PsAttributeMemoryReserve = 7,
    PsAttributePriorityClass = 8,
    PsAttributeErrorMode = 9,
    PsAttributeStdHandleInfo = 10,
    PsAttributeHandleList = 11,
    PsAttributeGroupAffinity = 12,
    PsAttributePreferredNode = 13,
    PsAttributeIdealProcessor = 14,
    PsAttributeUmsThread = 15,
    PsAttributeMitigationOptions = 16,
    PsAttributeProtectionLevel = 17,
    PsAttributeSecureProcess = 18,
    PsAttributeJobList = 19,
    PsAttributeChildProcessPolicy = 20,
    PsAttributeAllApplicationPackagesPolicy = 21,
    PsAttributeWin32kFilter = 22,
    PsAttributeSafeOpenPromptOriginClaim = 23,
    PsAttributeBnoIsolation = 24,
    PsAttributeDesktopAppPolicy = 25,
    PsAttributeChpe = 26,
    PsAttributeMitigationAuditOptions = 27,
    PsAttributeMachineType = 28,
    PsAttributeComponentFilter = 29,
    PsAttributeEnableOptionalXStateFeatures = 30,
    PsAttributeMax = 31,
}

#[repr(C)]
pub struct PS_ATTRIBUTE {
    pub Attribute: usize,
    pub Size: usize,
    pub Anonymous1: PS_ATTRIBUTE_1,
    pub ReturnLength: *mut usize,
}

#[repr(C)]
pub struct PS_ATTRIBUTE_1 {
    pub Value: UnionField<usize>,
    pub ValuePtr: UnionField<*mut std::ffi::c_void>,
    pub union_field: u64,
}

impl Default for PS_ATTRIBUTE_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PS_ATTRIBUTE_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PS_ATTRIBUTE_1 {{ union }}")
    }
}

impl Default for PS_ATTRIBUTE {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PS_ATTRIBUTE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PS_ATTRIBUTE {{ Anonymous1: {:?} }}", self.Anonymous1)
    }
}

#[repr(C)]
pub struct PS_ATTRIBUTE_LIST {
    pub TotalLength: usize,
    pub Attributes: [PS_ATTRIBUTE; 1],
}

impl Default for PS_ATTRIBUTE_LIST {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PS_ATTRIBUTE_LIST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PS_ATTRIBUTE_LIST {{ Attributes: {:?} }}",
            self.Attributes
        )
    }
}

#[repr(C)]
pub struct PS_MEMORY_RESERVE {
    pub ReserveAddress: *mut std::ffi::c_void,
    pub ReserveSize: usize,
}

impl Default for PS_MEMORY_RESERVE {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PS_MEMORY_RESERVE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PS_MEMORY_RESERVE {{  }}")
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum PS_STD_HANDLE_STATE {
    PsNeverDuplicate = 0,
    PsRequestDuplicate = 1,
    PsAlwaysDuplicate = 2,
    PsMaxStdHandleStates = 3,
}

#[repr(C)]
pub struct PS_STD_HANDLE_INFO {
    pub Anonymous1: PS_STD_HANDLE_INFO_1,
    pub StdHandleSubsystemType: u32,
}

#[repr(C)]
pub struct PS_STD_HANDLE_INFO_1 {
    pub Flags: UnionField<u32>,
    pub Anonymous1: UnionField<PS_STD_HANDLE_INFO_1_1>,
    pub union_field: u32,
}

#[repr(C)]
#[repr(align(4))]
pub struct PS_STD_HANDLE_INFO_1_1 {
    _bitfield_align_1: [u8; 0],
    _bitfield_1: BitfieldUnit<[u8; 1]>,
    pub padding_0: [u8; 3],
}

impl Default for PS_STD_HANDLE_INFO_1_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PS_STD_HANDLE_INFO_1_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PS_STD_HANDLE_INFO_1_1 {{ StdHandleState : {:?}, PseudoHandleMask : {:?} }}",
            self.StdHandleState(),
            self.PseudoHandleMask()
        )
    }
}

impl PS_STD_HANDLE_INFO_1_1 {
    #[inline]
    pub fn StdHandleState(&self) -> u32 {
        self._bitfield_1.get(0usize, 2u8) as u32
    }

    #[inline]
    pub fn set_StdHandleState(&mut self, val: u32) {
        self._bitfield_1.set(0usize, 2u8, val as u64)
    }

    #[inline]
    pub fn PseudoHandleMask(&self) -> u32 {
        self._bitfield_1.get(2usize, 3u8) as u32
    }

    #[inline]
    pub fn set_PseudoHandleMask(&mut self, val: u32) {
        self._bitfield_1.set(2usize, 3u8, val as u64)
    }

    #[inline]
    pub fn new_bitfield_1(StdHandleState: u32, PseudoHandleMask: u32) -> BitfieldUnit<[u8; 1]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 1]> = Default::default();

        bitfield_unit.set(0usize, 2u8, StdHandleState as u64);

        bitfield_unit.set(2usize, 3u8, PseudoHandleMask as u64);

        bitfield_unit
    }
}

impl Default for PS_STD_HANDLE_INFO_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PS_STD_HANDLE_INFO_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PS_STD_HANDLE_INFO_1 {{ union }}")
    }
}

impl Default for PS_STD_HANDLE_INFO {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PS_STD_HANDLE_INFO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PS_STD_HANDLE_INFO {{ Anonymous1: {:?} }}",
            self.Anonymous1
        )
    }
}

#[repr(C)]
pub struct PS_TRUSTLET_ATTRIBUTE_ACCESSRIGHTS {
    _bitfield_align_1: [u8; 0],
    _bitfield_1: UnionField<BitfieldUnit<[u8; 1]>>,
    pub AccessRights: UnionField<u8>,
    pub union_field: u8,
}

impl Default for PS_TRUSTLET_ATTRIBUTE_ACCESSRIGHTS {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PS_TRUSTLET_ATTRIBUTE_ACCESSRIGHTS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PS_TRUSTLET_ATTRIBUTE_ACCESSRIGHTS {{ union }}")
    }
}

impl PS_TRUSTLET_ATTRIBUTE_ACCESSRIGHTS {
    #[inline]
    pub fn Trustlet(&self) -> u8 {
        unsafe { self._bitfield_1.as_ref().get(0usize, 1u8) as u8 }
    }

    #[inline]
    pub fn set_Trustlet(&mut self, val: u8) {
        unsafe { self._bitfield_1.as_mut().set(0usize, 1u8, val as u64) }
    }

    #[inline]
    pub fn Ntos(&self) -> u8 {
        unsafe { self._bitfield_1.as_ref().get(1usize, 1u8) as u8 }
    }

    #[inline]
    pub fn set_Ntos(&mut self, val: u8) {
        unsafe { self._bitfield_1.as_mut().set(1usize, 1u8, val as u64) }
    }

    #[inline]
    pub fn WriteHandle(&self) -> u8 {
        unsafe { self._bitfield_1.as_ref().get(2usize, 1u8) as u8 }
    }

    #[inline]
    pub fn set_WriteHandle(&mut self, val: u8) {
        unsafe { self._bitfield_1.as_mut().set(2usize, 1u8, val as u64) }
    }

    #[inline]
    pub fn ReadHandle(&self) -> u8 {
        unsafe { self._bitfield_1.as_ref().get(3usize, 1u8) as u8 }
    }

    #[inline]
    pub fn set_ReadHandle(&mut self, val: u8) {
        unsafe { self._bitfield_1.as_mut().set(3usize, 1u8, val as u64) }
    }

    #[inline]
    pub fn Reserved(&self) -> u8 {
        unsafe { self._bitfield_1.as_ref().get(4usize, 4u8) as u8 }
    }

    #[inline]
    pub fn set_Reserved(&mut self, val: u8) {
        unsafe { self._bitfield_1.as_mut().set(4usize, 4u8, val as u64) }
    }

    #[inline]
    pub fn new_bitfield_1(
        Trustlet: u8,
        Ntos: u8,
        WriteHandle: u8,
        ReadHandle: u8,
        Reserved: u8,
    ) -> BitfieldUnit<[u8; 1]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 1]> = Default::default();

        bitfield_unit.set(0usize, 1u8, Trustlet as u64);

        bitfield_unit.set(1usize, 1u8, Ntos as u64);

        bitfield_unit.set(2usize, 1u8, WriteHandle as u64);

        bitfield_unit.set(3usize, 1u8, ReadHandle as u64);

        bitfield_unit.set(4usize, 4u8, Reserved as u64);

        bitfield_unit
    }
}

#[repr(C)]
pub struct PS_TRUSTLET_ATTRIBUTE_TYPE {
    pub Anonymous1: PS_TRUSTLET_ATTRIBUTE_TYPE_1,
}

#[repr(C)]
pub struct PS_TRUSTLET_ATTRIBUTE_TYPE_1 {
    pub Anonymous1: UnionField<PS_TRUSTLET_ATTRIBUTE_TYPE_1_1>,
    pub AttributeType: UnionField<u32>,
    pub union_field: u32,
}

#[repr(C)]
pub struct PS_TRUSTLET_ATTRIBUTE_TYPE_1_1 {
    pub Version: u8,
    pub DataCount: u8,
    pub SemanticType: u8,
    pub AccessRights: PS_TRUSTLET_ATTRIBUTE_ACCESSRIGHTS,
}

impl Default for PS_TRUSTLET_ATTRIBUTE_TYPE_1_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PS_TRUSTLET_ATTRIBUTE_TYPE_1_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PS_TRUSTLET_ATTRIBUTE_TYPE_1_1 {{ AccessRights: {:?} }}",
            self.AccessRights
        )
    }
}

impl Default for PS_TRUSTLET_ATTRIBUTE_TYPE_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PS_TRUSTLET_ATTRIBUTE_TYPE_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PS_TRUSTLET_ATTRIBUTE_TYPE_1 {{ union }}")
    }
}

impl Default for PS_TRUSTLET_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PS_TRUSTLET_ATTRIBUTE_TYPE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PS_TRUSTLET_ATTRIBUTE_TYPE {{ Anonymous1: {:?} }}",
            self.Anonymous1
        )
    }
}

#[repr(C)]
pub struct PS_TRUSTLET_ATTRIBUTE_HEADER {
    pub AttributeType: PS_TRUSTLET_ATTRIBUTE_TYPE,
    _bitfield_align_1: [u32; 0],
    _bitfield_1: BitfieldUnit<[u8; 4]>,
}

impl Default for PS_TRUSTLET_ATTRIBUTE_HEADER {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PS_TRUSTLET_ATTRIBUTE_HEADER {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PS_TRUSTLET_ATTRIBUTE_HEADER {{ AttributeType: {:?}, InstanceNumber : {:?}, Reserved : {:?} }}",
            self.AttributeType,
            self.InstanceNumber(),
            self.Reserved()
        )
    }
}

impl PS_TRUSTLET_ATTRIBUTE_HEADER {
    #[inline]
    pub fn InstanceNumber(&self) -> u32 {
        self._bitfield_1.get(0usize, 8u8) as u32
    }

    #[inline]
    pub fn set_InstanceNumber(&mut self, val: u32) {
        self._bitfield_1.set(0usize, 8u8, val as u64)
    }

    #[inline]
    pub fn Reserved(&self) -> u32 {
        self._bitfield_1.get(8usize, 24u8) as u32
    }

    #[inline]
    pub fn set_Reserved(&mut self, val: u32) {
        self._bitfield_1.set(8usize, 24u8, val as u64)
    }

    #[inline]
    pub fn new_bitfield_1(InstanceNumber: u32, Reserved: u32) -> BitfieldUnit<[u8; 4]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 4]> = Default::default();

        bitfield_unit.set(0usize, 8u8, InstanceNumber as u64);

        bitfield_unit.set(8usize, 24u8, Reserved as u64);

        bitfield_unit
    }
}

#[repr(C)]
pub struct PS_TRUSTLET_ATTRIBUTE_DATA {
    pub Header: PS_TRUSTLET_ATTRIBUTE_HEADER,
    pub Data: [u64; 1],
}

impl Default for PS_TRUSTLET_ATTRIBUTE_DATA {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PS_TRUSTLET_ATTRIBUTE_DATA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PS_TRUSTLET_ATTRIBUTE_DATA {{ Header: {:?}, Data: {:?} }}",
            self.Header, self.Data
        )
    }
}

#[repr(C)]
pub struct PS_TRUSTLET_CREATE_ATTRIBUTES {
    pub TrustletIdentity: u64,
    pub Attributes: [PS_TRUSTLET_ATTRIBUTE_DATA; 1],
}

impl Default for PS_TRUSTLET_CREATE_ATTRIBUTES {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PS_TRUSTLET_CREATE_ATTRIBUTES {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PS_TRUSTLET_CREATE_ATTRIBUTES {{ Attributes: {:?} }}",
            self.Attributes
        )
    }
}

#[repr(C)]
pub struct PS_BNO_ISOLATION_PARAMETERS {
    pub IsolationPrefix: UNICODE_STRING,
    pub HandleCount: u32,
    pub Handles: *mut *mut std::ffi::c_void,
    pub IsolationEnabled: bool,
}

impl Default for PS_BNO_ISOLATION_PARAMETERS {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PS_BNO_ISOLATION_PARAMETERS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PS_BNO_ISOLATION_PARAMETERS {{ Handles: {:?} }}",
            self.Handles
        )
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum PS_MITIGATION_OPTION {
    PS_MITIGATION_OPTION_NX = 0,
    PS_MITIGATION_OPTION_SEHOP = 1,
    PS_MITIGATION_OPTION_FORCE_RELOCATE_IMAGES = 2,
    PS_MITIGATION_OPTION_HEAP_TERMINATE = 3,
    PS_MITIGATION_OPTION_BOTTOM_UP_ASLR = 4,
    PS_MITIGATION_OPTION_HIGH_ENTROPY_ASLR = 5,
    PS_MITIGATION_OPTION_STRICT_HANDLE_CHECKS = 6,
    PS_MITIGATION_OPTION_WIN32K_SYSTEM_CALL_DISABLE = 7,
    PS_MITIGATION_OPTION_EXTENSION_POINT_DISABLE = 8,
    PS_MITIGATION_OPTION_PROHIBIT_DYNAMIC_CODE = 9,
    PS_MITIGATION_OPTION_CONTROL_FLOW_GUARD = 10,
    PS_MITIGATION_OPTION_BLOCK_NON_MICROSOFT_BINARIES = 11,
    PS_MITIGATION_OPTION_FONT_DISABLE = 12,
    PS_MITIGATION_OPTION_IMAGE_LOAD_NO_REMOTE = 13,
    PS_MITIGATION_OPTION_IMAGE_LOAD_NO_LOW_LABEL = 14,
    PS_MITIGATION_OPTION_IMAGE_LOAD_PREFER_SYSTEM32 = 15,
    PS_MITIGATION_OPTION_RETURN_FLOW_GUARD = 16,
    PS_MITIGATION_OPTION_LOADER_INTEGRITY_CONTINUITY = 17,
    PS_MITIGATION_OPTION_STRICT_CONTROL_FLOW_GUARD = 18,
    PS_MITIGATION_OPTION_RESTRICT_SET_THREAD_CONTEXT = 19,
    PS_MITIGATION_OPTION_ROP_STACKPIVOT = 20,
    PS_MITIGATION_OPTION_ROP_CALLER_CHECK = 21,
    PS_MITIGATION_OPTION_ROP_SIMEXEC = 22,
    PS_MITIGATION_OPTION_EXPORT_ADDRESS_FILTER = 23,
    PS_MITIGATION_OPTION_EXPORT_ADDRESS_FILTER_PLUS = 24,
    PS_MITIGATION_OPTION_RESTRICT_CHILD_PROCESS_CREATION = 25,
    PS_MITIGATION_OPTION_IMPORT_ADDRESS_FILTER = 26,
    PS_MITIGATION_OPTION_MODULE_TAMPERING_PROTECTION = 27,
    PS_MITIGATION_OPTION_RESTRICT_INDIRECT_BRANCH_PREDICTION = 28,
    PS_MITIGATION_OPTION_SPECULATIVE_STORE_BYPASS_DISABLE = 29,
    PS_MITIGATION_OPTION_ALLOW_DOWNGRADE_DYNAMIC_CODE_POLICY = 30,
    PS_MITIGATION_OPTION_CET_USER_SHADOW_STACKS = 31,
    PS_MITIGATION_OPTION_USER_CET_SET_CONTEXT_IP_VALIDATION = 32,
    PS_MITIGATION_OPTION_BLOCK_NON_CET_BINARIES = 33,
    PS_MITIGATION_OPTION_CET_DYNAMIC_APIS_OUT_OF_PROC_ONLY = 34,
    PS_MITIGATION_OPTION_REDIRECTION_TRUST = 35,
    PS_MITIGATION_OPTION_RESTRICT_CORE_SHARING = 36,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum PS_CREATE_STATE {
    PsCreateInitialState = 0,
    PsCreateFailOnFileOpen = 1,
    PsCreateFailOnSectionCreate = 2,
    PsCreateFailExeFormat = 3,
    PsCreateFailMachineMismatch = 4,
    PsCreateFailExeName = 5,
    PsCreateSuccess = 6,
    PsCreateMaximumStates = 7,
}

#[repr(C)]
pub struct PS_CREATE_INFO {
    pub Size: usize,
    pub State: PS_CREATE_STATE,
    pub Anonymous1: PS_CREATE_INFO_1,
}

#[repr(C)]
pub struct PS_CREATE_INFO_1 {
    pub InitState: UnionField<PS_CREATE_INFO_1_1>,
    pub FailSection: UnionField<PS_CREATE_INFO_1_2>,
    pub ExeFormat: UnionField<PS_CREATE_INFO_1_3>,
    pub ExeName: UnionField<PS_CREATE_INFO_1_4>,
    pub SuccessState: UnionField<PS_CREATE_INFO_1_5>,
    pub union_field: [u64; 9],
}

#[repr(C)]
pub struct PS_CREATE_INFO_1_1 {
    pub Anonymous1: PS_CREATE_INFO_1_1_1,
    pub AdditionalFileAccess: u32,
}

#[repr(C)]
pub struct PS_CREATE_INFO_1_1_1 {
    pub InitFlags: UnionField<u32>,
    pub Anonymous1: UnionField<PS_CREATE_INFO_1_1_1_1>,
    pub union_field: u32,
}

#[repr(C)]
#[repr(align(2))]
pub struct PS_CREATE_INFO_1_1_1_1 {
    _bitfield_align_1: [u16; 0],
    _bitfield_1: BitfieldUnit<[u8; 4]>,
}

impl Default for PS_CREATE_INFO_1_1_1_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PS_CREATE_INFO_1_1_1_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PS_CREATE_INFO_1_1_1_1 {{ WriteOutputOnExit : {:?}, DetectManifest : {:?}, IFEOSkipDebugger : {:?}, IFEODoNotPropagateKeyState : {:?}, SpareBits1 : {:?}, SpareBits2 : {:?}, ProhibitedImageCharacteristics : {:?} }}",
            self.WriteOutputOnExit(),
            self.DetectManifest(),
            self.IFEOSkipDebugger(),
            self.IFEODoNotPropagateKeyState(),
            self.SpareBits1(),
            self.SpareBits2(),
            self.ProhibitedImageCharacteristics()
        )
    }
}

impl PS_CREATE_INFO_1_1_1_1 {
    #[inline]
    pub fn WriteOutputOnExit(&self) -> u8 {
        self._bitfield_1.get(0usize, 1u8) as u8
    }

    #[inline]
    pub fn set_WriteOutputOnExit(&mut self, val: u8) {
        self._bitfield_1.set(0usize, 1u8, val as u64)
    }

    #[inline]
    pub fn DetectManifest(&self) -> u8 {
        self._bitfield_1.get(1usize, 1u8) as u8
    }

    #[inline]
    pub fn set_DetectManifest(&mut self, val: u8) {
        self._bitfield_1.set(1usize, 1u8, val as u64)
    }

    #[inline]
    pub fn IFEOSkipDebugger(&self) -> u8 {
        self._bitfield_1.get(2usize, 1u8) as u8
    }

    #[inline]
    pub fn set_IFEOSkipDebugger(&mut self, val: u8) {
        self._bitfield_1.set(2usize, 1u8, val as u64)
    }

    #[inline]
    pub fn IFEODoNotPropagateKeyState(&self) -> u8 {
        self._bitfield_1.get(3usize, 1u8) as u8
    }

    #[inline]
    pub fn set_IFEODoNotPropagateKeyState(&mut self, val: u8) {
        self._bitfield_1.set(3usize, 1u8, val as u64)
    }

    #[inline]
    pub fn SpareBits1(&self) -> u8 {
        self._bitfield_1.get(4usize, 4u8) as u8
    }

    #[inline]
    pub fn set_SpareBits1(&mut self, val: u8) {
        self._bitfield_1.set(4usize, 4u8, val as u64)
    }

    #[inline]
    pub fn SpareBits2(&self) -> u8 {
        self._bitfield_1.get(8usize, 8u8) as u8
    }

    #[inline]
    pub fn set_SpareBits2(&mut self, val: u8) {
        self._bitfield_1.set(8usize, 8u8, val as u64)
    }

    #[inline]
    pub fn ProhibitedImageCharacteristics(&self) -> u16 {
        self._bitfield_1.get(16usize, 16u8) as u16
    }

    #[inline]
    pub fn set_ProhibitedImageCharacteristics(&mut self, val: u16) {
        self._bitfield_1.set(16usize, 16u8, val as u64)
    }

    #[inline]
    pub fn new_bitfield_1(
        WriteOutputOnExit: u8,
        DetectManifest: u8,
        IFEOSkipDebugger: u8,
        IFEODoNotPropagateKeyState: u8,
        SpareBits1: u8,
        SpareBits2: u8,
        ProhibitedImageCharacteristics: u16,
    ) -> BitfieldUnit<[u8; 4]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 4]> = Default::default();

        bitfield_unit.set(0usize, 1u8, WriteOutputOnExit as u64);

        bitfield_unit.set(1usize, 1u8, DetectManifest as u64);

        bitfield_unit.set(2usize, 1u8, IFEOSkipDebugger as u64);

        bitfield_unit.set(3usize, 1u8, IFEODoNotPropagateKeyState as u64);

        bitfield_unit.set(4usize, 4u8, SpareBits1 as u64);

        bitfield_unit.set(8usize, 8u8, SpareBits2 as u64);

        bitfield_unit.set(16usize, 16u8, ProhibitedImageCharacteristics as u64);

        bitfield_unit
    }
}

impl Default for PS_CREATE_INFO_1_1_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PS_CREATE_INFO_1_1_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PS_CREATE_INFO_1_1_1 {{ union }}")
    }
}

impl Default for PS_CREATE_INFO_1_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PS_CREATE_INFO_1_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PS_CREATE_INFO_1_1 {{ Anonymous1: {:?} }}",
            self.Anonymous1
        )
    }
}

#[repr(C)]
pub struct PS_CREATE_INFO_1_2 {
    pub FileHandle: HANDLE,
}

impl Default for PS_CREATE_INFO_1_2 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PS_CREATE_INFO_1_2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PS_CREATE_INFO_1_2 {{  }}")
    }
}

#[repr(C)]
pub struct PS_CREATE_INFO_1_3 {
    pub DllCharacteristics: u16,
}

impl Default for PS_CREATE_INFO_1_3 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PS_CREATE_INFO_1_3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PS_CREATE_INFO_1_3 {{  }}")
    }
}

#[repr(C)]
pub struct PS_CREATE_INFO_1_4 {
    pub IFEOKey: HANDLE,
}

impl Default for PS_CREATE_INFO_1_4 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PS_CREATE_INFO_1_4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PS_CREATE_INFO_1_4 {{  }}")
    }
}

#[repr(C)]
pub struct PS_CREATE_INFO_1_5 {
    pub Anonymous1: PS_CREATE_INFO_1_5_1,
    pub FileHandle: HANDLE,
    pub SectionHandle: HANDLE,
    pub UserProcessParametersNative: u64,
    pub UserProcessParametersWow64: u32,
    pub CurrentParameterFlags: u32,
    pub PebAddressNative: u64,
    pub PebAddressWow64: u32,
    pub ManifestAddress: u64,
    pub ManifestSize: u32,
}

#[repr(C)]
pub struct PS_CREATE_INFO_1_5_1 {
    pub OutputFlags: UnionField<u32>,
    pub Anonymous1: UnionField<PS_CREATE_INFO_1_5_1_1>,
    pub union_field: u32,
}

#[repr(C)]
#[repr(align(2))]
pub struct PS_CREATE_INFO_1_5_1_1 {
    _bitfield_align_1: [u16; 0],
    _bitfield_1: BitfieldUnit<[u8; 4]>,
}

impl Default for PS_CREATE_INFO_1_5_1_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PS_CREATE_INFO_1_5_1_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PS_CREATE_INFO_1_5_1_1 {{ ProtectedProcess : {:?}, AddressSpaceOverride : {:?}, DevOverrideEnabled : {:?}, ManifestDetected : {:?}, ProtectedProcessLight : {:?}, SpareBits1 : {:?}, SpareBits2 : {:?}, SpareBits3 : {:?} }}",
            self.ProtectedProcess(),
            self.AddressSpaceOverride(),
            self.DevOverrideEnabled(),
            self.ManifestDetected(),
            self.ProtectedProcessLight(),
            self.SpareBits1(),
            self.SpareBits2(),
            self.SpareBits3()
        )
    }
}

impl PS_CREATE_INFO_1_5_1_1 {
    #[inline]
    pub fn ProtectedProcess(&self) -> u8 {
        self._bitfield_1.get(0usize, 1u8) as u8
    }

    #[inline]
    pub fn set_ProtectedProcess(&mut self, val: u8) {
        self._bitfield_1.set(0usize, 1u8, val as u64)
    }

    #[inline]
    pub fn AddressSpaceOverride(&self) -> u8 {
        self._bitfield_1.get(1usize, 1u8) as u8
    }

    #[inline]
    pub fn set_AddressSpaceOverride(&mut self, val: u8) {
        self._bitfield_1.set(1usize, 1u8, val as u64)
    }

    #[inline]
    pub fn DevOverrideEnabled(&self) -> u8 {
        self._bitfield_1.get(2usize, 1u8) as u8
    }

    #[inline]
    pub fn set_DevOverrideEnabled(&mut self, val: u8) {
        self._bitfield_1.set(2usize, 1u8, val as u64)
    }

    #[inline]
    pub fn ManifestDetected(&self) -> u8 {
        self._bitfield_1.get(3usize, 1u8) as u8
    }

    #[inline]
    pub fn set_ManifestDetected(&mut self, val: u8) {
        self._bitfield_1.set(3usize, 1u8, val as u64)
    }

    #[inline]
    pub fn ProtectedProcessLight(&self) -> u8 {
        self._bitfield_1.get(4usize, 1u8) as u8
    }

    #[inline]
    pub fn set_ProtectedProcessLight(&mut self, val: u8) {
        self._bitfield_1.set(4usize, 1u8, val as u64)
    }

    #[inline]
    pub fn SpareBits1(&self) -> u8 {
        self._bitfield_1.get(5usize, 3u8) as u8
    }

    #[inline]
    pub fn set_SpareBits1(&mut self, val: u8) {
        self._bitfield_1.set(5usize, 3u8, val as u64)
    }

    #[inline]
    pub fn SpareBits2(&self) -> u8 {
        self._bitfield_1.get(8usize, 8u8) as u8
    }

    #[inline]
    pub fn set_SpareBits2(&mut self, val: u8) {
        self._bitfield_1.set(8usize, 8u8, val as u64)
    }

    #[inline]
    pub fn SpareBits3(&self) -> u16 {
        self._bitfield_1.get(16usize, 16u8) as u16
    }

    #[inline]
    pub fn set_SpareBits3(&mut self, val: u16) {
        self._bitfield_1.set(16usize, 16u8, val as u64)
    }

    #[inline]
    pub fn new_bitfield_1(
        ProtectedProcess: u8,
        AddressSpaceOverride: u8,
        DevOverrideEnabled: u8,
        ManifestDetected: u8,
        ProtectedProcessLight: u8,
        SpareBits1: u8,
        SpareBits2: u8,
        SpareBits3: u16,
    ) -> BitfieldUnit<[u8; 4]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 4]> = Default::default();

        bitfield_unit.set(0usize, 1u8, ProtectedProcess as u64);

        bitfield_unit.set(1usize, 1u8, AddressSpaceOverride as u64);

        bitfield_unit.set(2usize, 1u8, DevOverrideEnabled as u64);

        bitfield_unit.set(3usize, 1u8, ManifestDetected as u64);

        bitfield_unit.set(4usize, 1u8, ProtectedProcessLight as u64);

        bitfield_unit.set(5usize, 3u8, SpareBits1 as u64);

        bitfield_unit.set(8usize, 8u8, SpareBits2 as u64);

        bitfield_unit.set(16usize, 16u8, SpareBits3 as u64);

        bitfield_unit
    }
}

impl Default for PS_CREATE_INFO_1_5_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PS_CREATE_INFO_1_5_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PS_CREATE_INFO_1_5_1 {{ union }}")
    }
}

impl Default for PS_CREATE_INFO_1_5 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PS_CREATE_INFO_1_5 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PS_CREATE_INFO_1_5 {{ Anonymous1: {:?} }}",
            self.Anonymous1
        )
    }
}

impl Default for PS_CREATE_INFO_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PS_CREATE_INFO_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PS_CREATE_INFO_1 {{ union }}")
    }
}

impl Default for PS_CREATE_INFO {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PS_CREATE_INFO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PS_CREATE_INFO {{ State: {:?}, Anonymous1: {:?} }}",
            self.State, self.Anonymous1
        )
    }
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtCreateUserProcess(
        ProcessHandle: *mut HANDLE,
        ThreadHandle: *mut HANDLE,
        ProcessDesiredAccess: u32,
        ThreadDesiredAccess: u32,
        ProcessObjectAttributes: *mut OBJECT_ATTRIBUTES,
        ThreadObjectAttributes: *mut OBJECT_ATTRIBUTES,
        ProcessFlags: u32,
        ThreadFlags: u32,
        ProcessParameters: *mut std::ffi::c_void,
        CreateInfo: *mut PS_CREATE_INFO,
        AttributeList: *mut PS_ATTRIBUTE_LIST,
    ) -> NTSTATUS;
}

pub type PUSER_THREAD_START_ROUTINE = std::option::Option<
    unsafe extern "system" fn(ThreadParameter: *mut std::ffi::c_void) -> NTSTATUS,
>;

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtCreateThreadEx(
        ThreadHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        ProcessHandle: HANDLE,
        StartRoutine: PUSER_THREAD_START_ROUTINE,
        Argument: *mut std::ffi::c_void,
        CreateFlags: u32,
        ZeroBits: usize,
        StackSize: usize,
        MaximumStackSize: usize,
        AttributeList: *mut PS_ATTRIBUTE_LIST,
    ) -> NTSTATUS;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct JOBOBJECT_EXTENDED_LIMIT_INFORMATION_V2 {
    pub BasicLimitInformation: JOBOBJECT_BASIC_LIMIT_INFORMATION,
    pub IoInfo: IO_COUNTERS,
    pub ProcessMemoryLimit: usize,
    pub JobMemoryLimit: usize,
    pub PeakProcessMemoryUsed: usize,
    pub PeakJobMemoryUsed: usize,
    pub JobTotalMemoryLimit: usize,
}

impl Default for JOBOBJECT_EXTENDED_LIMIT_INFORMATION_V2 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for JOBOBJECT_EXTENDED_LIMIT_INFORMATION_V2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "JOBOBJECT_EXTENDED_LIMIT_INFORMATION_V2 {{ BasicLimitInformation: {:?}, IoInfo: {:?}, ProcessMemoryLimit: {:?}, JobMemoryLimit: {:?}, PeakProcessMemoryUsed: {:?}, PeakJobMemoryUsed: {:?}, JobTotalMemoryLimit: {:?} }}",
            self.BasicLimitInformation,
            self.IoInfo,
            self.ProcessMemoryLimit,
            self.JobMemoryLimit,
            self.PeakProcessMemoryUsed,
            self.PeakJobMemoryUsed,
            self.JobTotalMemoryLimit
        )
    }
}

#[repr(C)]
pub struct JOBOBJECT_EXTENDED_ACCOUNTING_INFORMATION {
    pub BasicInfo: JOBOBJECT_BASIC_ACCOUNTING_INFORMATION,
    pub IoInfo: IO_COUNTERS,
    pub DiskIoInfo: PROCESS_DISK_COUNTERS,
    pub ContextSwitches: u64,
    pub TotalCycleTime: i64,
    pub ReadyTime: u64,
    pub EnergyValues: PROCESS_ENERGY_VALUES,
}

impl Default for JOBOBJECT_EXTENDED_ACCOUNTING_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for JOBOBJECT_EXTENDED_ACCOUNTING_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "JOBOBJECT_EXTENDED_ACCOUNTING_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct JOBOBJECT_WAKE_INFORMATION {
    pub NotificationChannel: HANDLE,
    pub WakeCounters: [u64; 7],
}

impl Default for JOBOBJECT_WAKE_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for JOBOBJECT_WAKE_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "JOBOBJECT_WAKE_INFORMATION {{ WakeCounters: {:?} }}",
            self.WakeCounters
        )
    }
}

#[repr(C)]
pub struct JOBOBJECT_WAKE_INFORMATION_V1 {
    pub NotificationChannel: HANDLE,
    pub WakeCounters: [u64; 4],
}

impl Default for JOBOBJECT_WAKE_INFORMATION_V1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for JOBOBJECT_WAKE_INFORMATION_V1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "JOBOBJECT_WAKE_INFORMATION_V1 {{ WakeCounters: {:?} }}",
            self.WakeCounters
        )
    }
}

#[repr(C)]
pub struct JOBOBJECT_INTERFERENCE_INFORMATION {
    pub Count: u64,
}

impl Default for JOBOBJECT_INTERFERENCE_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for JOBOBJECT_INTERFERENCE_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "JOBOBJECT_INTERFERENCE_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct JOBOBJECT_WAKE_FILTER {
    pub HighEdgeFilter: u32,
    pub LowEdgeFilter: u32,
}

impl Default for JOBOBJECT_WAKE_FILTER {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for JOBOBJECT_WAKE_FILTER {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "JOBOBJECT_WAKE_FILTER {{  }}")
    }
}

#[repr(C)]
pub struct JOBOBJECT_FREEZE_INFORMATION {
    pub Anonymous1: JOBOBJECT_FREEZE_INFORMATION_1,
    pub Freeze: bool,
    pub Swap: bool,
    pub Reserved0: [u8; 2],
    pub WakeFilter: JOBOBJECT_WAKE_FILTER,
}

#[repr(C)]
pub struct JOBOBJECT_FREEZE_INFORMATION_1 {
    pub Flags: UnionField<u32>,
    pub Anonymous1: UnionField<JOBOBJECT_FREEZE_INFORMATION_1_1>,
    pub union_field: u32,
}

#[repr(C)]
#[repr(align(4))]
pub struct JOBOBJECT_FREEZE_INFORMATION_1_1 {
    _bitfield_align_1: [u32; 0],
    _bitfield_1: BitfieldUnit<[u8; 4]>,
}

impl Default for JOBOBJECT_FREEZE_INFORMATION_1_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for JOBOBJECT_FREEZE_INFORMATION_1_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "JOBOBJECT_FREEZE_INFORMATION_1_1 {{ FreezeOperation : {:?}, FilterOperation : {:?}, SwapOperation : {:?}, Reserved : {:?} }}",
            self.FreezeOperation(),
            self.FilterOperation(),
            self.SwapOperation(),
            self.Reserved()
        )
    }
}

impl JOBOBJECT_FREEZE_INFORMATION_1_1 {
    #[inline]
    pub fn FreezeOperation(&self) -> u32 {
        self._bitfield_1.get(0usize, 1u8) as u32
    }

    #[inline]
    pub fn set_FreezeOperation(&mut self, val: u32) {
        self._bitfield_1.set(0usize, 1u8, val as u64)
    }

    #[inline]
    pub fn FilterOperation(&self) -> u32 {
        self._bitfield_1.get(1usize, 1u8) as u32
    }

    #[inline]
    pub fn set_FilterOperation(&mut self, val: u32) {
        self._bitfield_1.set(1usize, 1u8, val as u64)
    }

    #[inline]
    pub fn SwapOperation(&self) -> u32 {
        self._bitfield_1.get(2usize, 1u8) as u32
    }

    #[inline]
    pub fn set_SwapOperation(&mut self, val: u32) {
        self._bitfield_1.set(2usize, 1u8, val as u64)
    }

    #[inline]
    pub fn Reserved(&self) -> u32 {
        self._bitfield_1.get(3usize, 29u8) as u32
    }

    #[inline]
    pub fn set_Reserved(&mut self, val: u32) {
        self._bitfield_1.set(3usize, 29u8, val as u64)
    }

    #[inline]
    pub fn new_bitfield_1(
        FreezeOperation: u32,
        FilterOperation: u32,
        SwapOperation: u32,
        Reserved: u32,
    ) -> BitfieldUnit<[u8; 4]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 4]> = Default::default();

        bitfield_unit.set(0usize, 1u8, FreezeOperation as u64);

        bitfield_unit.set(1usize, 1u8, FilterOperation as u64);

        bitfield_unit.set(2usize, 1u8, SwapOperation as u64);

        bitfield_unit.set(3usize, 29u8, Reserved as u64);

        bitfield_unit
    }
}

impl Default for JOBOBJECT_FREEZE_INFORMATION_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for JOBOBJECT_FREEZE_INFORMATION_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "JOBOBJECT_FREEZE_INFORMATION_1 {{ union }}")
    }
}

impl Default for JOBOBJECT_FREEZE_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for JOBOBJECT_FREEZE_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "JOBOBJECT_FREEZE_INFORMATION {{ Anonymous1: {:?}, Reserved0: {:?}, WakeFilter: {:?} }}",
            self.Anonymous1, self.Reserved0, self.WakeFilter
        )
    }
}

#[repr(C)]
pub struct JOBOBJECT_CONTAINER_IDENTIFIER_V2 {
    pub ContainerId: GUID,
    pub ContainerTelemetryId: GUID,
    pub JobId: u32,
}

impl Default for JOBOBJECT_CONTAINER_IDENTIFIER_V2 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for JOBOBJECT_CONTAINER_IDENTIFIER_V2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "JOBOBJECT_CONTAINER_IDENTIFIER_V2 {{  }}")
    }
}

#[repr(C)]
pub struct JOBOBJECT_MEMORY_USAGE_INFORMATION {
    pub JobMemory: u64,
    pub PeakJobMemoryUsed: u64,
}

impl Default for JOBOBJECT_MEMORY_USAGE_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for JOBOBJECT_MEMORY_USAGE_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "JOBOBJECT_MEMORY_USAGE_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct JOBOBJECT_MEMORY_USAGE_INFORMATION_V2 {
    pub BasicInfo: JOBOBJECT_MEMORY_USAGE_INFORMATION,
    pub JobSharedMemory: u64,
    pub Reserved: [u64; 2],
}

impl Default for JOBOBJECT_MEMORY_USAGE_INFORMATION_V2 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for JOBOBJECT_MEMORY_USAGE_INFORMATION_V2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "JOBOBJECT_MEMORY_USAGE_INFORMATION_V2 {{ BasicInfo: {:?}, Reserved: {:?} }}",
            self.BasicInfo, self.Reserved
        )
    }
}

#[repr(C)]
pub struct SILO_USER_SHARED_DATA {
    pub ServiceSessionId: u32,
    pub ActiveConsoleId: u32,
    pub ConsoleSessionForegroundProcessId: i64,
    pub NtProductType: NT_PRODUCT_TYPE,
    pub SuiteMask: u32,
    pub SharedUserSessionId: u32,
    pub IsMultiSessionSku: bool,
    pub NtSystemRoot: [u16; 260],
    pub UserModeGlobalLogger: [u16; 16],
    pub TimeZoneId: u32,
    pub TimeZoneBiasStamp: i32,
    pub TimeZoneBias: KSYSTEM_TIME,
    pub TimeZoneBiasEffectiveStart: i64,
    pub TimeZoneBiasEffectiveEnd: i64,
}

impl Default for SILO_USER_SHARED_DATA {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for SILO_USER_SHARED_DATA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SILO_USER_SHARED_DATA {{ NtSystemRoot: {:?}, UserModeGlobalLogger: {:?} }}",
            self.NtSystemRoot, self.UserModeGlobalLogger
        )
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SILOOBJECT_ROOT_DIRECTORY {
    pub Anonymous1: SILOOBJECT_ROOT_DIRECTORY_1,
}

#[repr(C)]
#[derive(Copy, Clone)]

pub union SILOOBJECT_ROOT_DIRECTORY_1 {
    pub ControlFlags: u32,
    pub Path: UNICODE_STRING,
}

impl Default for SILOOBJECT_ROOT_DIRECTORY_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for SILOOBJECT_ROOT_DIRECTORY_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SILOOBJECT_ROOT_DIRECTORY_1 {{ union }}")
    }
}

impl Default for SILOOBJECT_ROOT_DIRECTORY {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for SILOOBJECT_ROOT_DIRECTORY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SILOOBJECT_ROOT_DIRECTORY {{ Anonymous1: {:?} }}",
            self.Anonymous1
        )
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SERVERSILO_INIT_INFORMATION {
    pub DeleteEvent: HANDLE,
    pub IsDownlevelContainer: bool,
}

impl Default for SERVERSILO_INIT_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

#[repr(C)]
pub struct JOBOBJECT_ENERGY_TRACKING_STATE {
    pub Value: u64,
    pub UpdateMask: u32,
    pub DesiredState: u32,
}

impl Default for JOBOBJECT_ENERGY_TRACKING_STATE {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for JOBOBJECT_ENERGY_TRACKING_STATE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "JOBOBJECT_ENERGY_TRACKING_STATE {{  }}")
    }
}

impl JOBOBJECT_IO_PRIORITY_LIMIT_FLAGS {}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum JOBOBJECT_IO_PRIORITY_LIMIT_FLAGS {
    JOBOBJECT_IO_PRIORITY_LIMIT_ENABLE = 1,
}

#[repr(C)]
pub struct JOBOBJECT_IO_PRIORITY_LIMIT {
    pub Flags: JOBOBJECT_IO_PRIORITY_LIMIT_FLAGS,
    pub Priority: u32,
}

impl std::fmt::Debug for JOBOBJECT_IO_PRIORITY_LIMIT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "JOBOBJECT_IO_PRIORITY_LIMIT {{ Flags: {:?} }}",
            self.Flags
        )
    }
}

impl JOBOBJECT_PAGE_PRIORITY_LIMIT_FLAGS {}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum JOBOBJECT_PAGE_PRIORITY_LIMIT_FLAGS {
    JOBOBJECT_PAGE_PRIORITY_LIMIT_ENABLE = 1,
}

#[repr(C)]
pub struct JOBOBJECT_PAGE_PRIORITY_LIMIT {
    pub Flags: JOBOBJECT_PAGE_PRIORITY_LIMIT_FLAGS,
    pub Priority: u32,
}

impl std::fmt::Debug for JOBOBJECT_PAGE_PRIORITY_LIMIT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "JOBOBJECT_PAGE_PRIORITY_LIMIT {{ Flags: {:?} }}",
            self.Flags
        )
    }
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtCreateJobObject(
        JobHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtOpenJobObject(
        JobHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtAssignProcessToJobObject(JobHandle: HANDLE, ProcessHandle: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtTerminateJobObject(JobHandle: HANDLE, ExitStatus: NTSTATUS) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtIsProcessInJob(ProcessHandle: HANDLE, JobHandle: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtQueryInformationJobObject(
        JobHandle: HANDLE,
        JobObjectInformationClass: JOBOBJECTINFOCLASS,
        JobObjectInformation: *mut std::ffi::c_void,
        JobObjectInformationLength: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtSetInformationJobObject(
        JobHandle: HANDLE,
        JobObjectInformationClass: JOBOBJECTINFOCLASS,
        JobObjectInformation: *mut std::ffi::c_void,
        JobObjectInformationLength: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtCreateJobSet(NumJob: u32, UserJobSet: *mut JOB_SET_ARRAY, Flags: u32) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtRevertContainerImpersonation() -> NTSTATUS;
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MEMORY_RESERVE_TYPE {
    MemoryReserveUserApc = 0,
    MemoryReserveIoCompletion = 1,
    MemoryReserveTypeMax = 2,
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtAllocateReserveObject(
        MemoryReserveHandle: *mut HANDLE,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        Type: MEMORY_RESERVE_TYPE,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn PssNtCaptureSnapshot(
        SnapshotHandle: *mut HANDLE,
        ProcessHandle: HANDLE,
        CaptureFlags: u32,
        ThreadContextFlags: u32,
    ) -> NTSTATUS;
}

#[repr(C)]
pub struct NTPSS_MEMORY_BULK_INFORMATION {
    pub QueryFlags: u32,
    pub NumberOfEntries: u32,
    pub NextValidAddress: *mut std::ffi::c_void,
}

impl Default for NTPSS_MEMORY_BULK_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for NTPSS_MEMORY_BULK_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NTPSS_MEMORY_BULK_INFORMATION {{  }}")
    }
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtPssCaptureVaSpaceBulk(
        ProcessHandle: HANDLE,
        BaseAddress: *mut std::ffi::c_void,
        BulkInformation: *mut NTPSS_MEMORY_BULK_INFORMATION,
        BulkInformationLength: usize,
        ReturnLength: *mut usize,
    ) -> NTSTATUS;
}
