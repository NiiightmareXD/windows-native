use windows::{
    core::{w, GUID, PCWSTR, PSTR, PWSTR},
    Wdk::{
        Storage::FileSystem::NLSTABLEINFO,
        System::SystemServices::{KSYSTEM_TIME, RTL_BITMAP, RTL_QUERY_REGISTRY_TABLE, TIME_FIELDS},
    },
    Win32::{
        Foundation::{HANDLE, LUID, NTSTATUS, UNICODE_STRING},
        Security::{
            ACE_HEADER, ACL, ACL_INFORMATION_CLASS, CLAIM_SECURITY_ATTRIBUTES_INFORMATION,
            GENERIC_MAPPING, LUID_AND_ATTRIBUTES, PSID, SECURITY_DESCRIPTOR,
            SECURITY_DESCRIPTOR_CONTROL, SECURITY_IMPERSONATION_LEVEL, SID_AND_ATTRIBUTES,
            SID_AND_ATTRIBUTES_HASH,
        },
        System::{
            ApplicationInstallationAndServicing::{
                ACTCTX_SECTION_KEYED_DATA, ACTIVATION_CONTEXT_QUERY_INDEX,
            },
            Diagnostics::Debug::{
                CONTEXT, EXCEPTION_POINTERS, EXCEPTION_RECORD, IMAGE_NT_HEADERS64,
                IMAGE_RUNTIME_FUNCTION_ENTRY, IMAGE_SECTION_HEADER, PGET_RUNTIME_FUNCTION_CALLBACK,
                PVECTORED_EXCEPTION_HANDLER, WOW64_CONTEXT, XSAVE_AREA_HEADER,
            },
            Kernel::{LIST_ENTRY, PROCESSOR_NUMBER, RTL_BALANCED_NODE, STRING, WNF_STATE_NAME},
            Memory::HEAP_INFORMATION_CLASS,
            Performance::HardwareCounterProfiling::PERFORMANCE_DATA,
            SystemServices::ACTIVATION_CONTEXT_INFO_CLASS,
            Threading::{
                APC_CALLBACK_FUNCTION, CONDITION_VARIABLE, CRITICAL_SECTION,
                CRITICAL_SECTION_DEBUG, LPTHREAD_START_ROUTINE, PFLS_CALLBACK_FUNCTION, SRWLOCK,
                SYNCHRONIZATION_BARRIER, WORKERCALLBACKFUNC,
            },
            WindowsProgramming::CLIENT_ID,
            IO::IO_STATUS_BLOCK,
        },
        UI::WindowsAndMessaging::MESSAGE_RESOURCE_ENTRY,
    },
};

use crate::{
    bitfield::{BitfieldUnit, UnionField},
    ntexapi::{RTL_PROCESS_BACKTRACES, RTL_PROCESS_LOCKS, WNF_TYPE_ID},
    ntldr::{RTL_PROCESS_MODULES, RTL_PROCESS_MODULE_INFORMATION_EX},
    ntmmapi::SECTION_IMAGE_INFORMATION,
    ntobapi::OBJECT_BOUNDARY_DESCRIPTOR,
    ntpebteb::{PEB, TEB, TEB_ACTIVE_FRAME},
    ntpsapi::{
        INITIAL_TEB, PPS_APC_ROUTINE, PS_PROTECTION, PUSER_THREAD_START_ROUTINE,
        THREAD_STATE_CHANGE_TYPE,
    },
    ntsxs::{ACTIVATION_CONTEXT, ACTIVATION_CONTEXT_DATA, PACTIVATION_CONTEXT_NOTIFY_ROUTINE},
    phnt_ntdef::WAITORTIMERCALLBACKFUNC,
};

pub const RTL_BARRIER_FLAGS_SPIN_ONLY: u32 = 1;
pub const RTL_BARRIER_FLAGS_BLOCK_ONLY: u32 = 2;
pub const RTL_BARRIER_FLAGS_NO_DELETE: u32 = 4;
pub const RTL_FIND_CHAR_IN_UNICODE_STRING_START_AT_END: u32 = 1;
pub const RTL_FIND_CHAR_IN_UNICODE_STRING_COMPLEMENT_CHAR_SET: u32 = 2;
pub const RTL_FIND_CHAR_IN_UNICODE_STRING_CASE_INSENSITIVE: u32 = 4;
pub const RTL_USER_PROC_CURDIR_CLOSE: u32 = 2;
pub const RTL_USER_PROC_CURDIR_INHERIT: u32 = 3;
pub const RTL_MAX_DRIVE_LETTERS: u32 = 32;
pub const RTL_USER_PROC_PARAMS_NORMALIZED: u32 = 1;
pub const RTL_USER_PROC_PROFILE_USER: u32 = 2;
pub const RTL_USER_PROC_PROFILE_KERNEL: u32 = 4;
pub const RTL_USER_PROC_PROFILE_SERVER: u32 = 8;
pub const RTL_USER_PROC_RESERVE_1MB: u32 = 32;
pub const RTL_USER_PROC_RESERVE_16MB: u32 = 64;
pub const RTL_USER_PROC_CASE_SENSITIVE: u32 = 128;
pub const RTL_USER_PROC_DISABLE_HEAP_DECOMMIT: u32 = 256;
pub const RTL_USER_PROC_DLL_REDIRECTION_LOCAL: u32 = 4096;
pub const RTL_USER_PROC_APP_MANIFEST_PRESENT: u32 = 8192;
pub const RTL_USER_PROC_IMAGE_KEY_MISSING: u32 = 16384;
pub const RTL_USER_PROC_OPTIN_PROCESS: u32 = 131072;
pub const RTL_USER_PROCESS_EXTENDED_PARAMETERS_VERSION: u32 = 1;
pub const RTL_CLONE_PROCESS_FLAGS_CREATE_SUSPENDED: u32 = 1;
pub const RTL_CLONE_PROCESS_FLAGS_INHERIT_HANDLES: u32 = 2;
pub const RTL_CLONE_PROCESS_FLAGS_NO_SYNCHRONIZE: u32 = 4;
pub const RTL_PROCESS_REFLECTION_FLAGS_INHERIT_HANDLES: u32 = 2;
pub const RTL_PROCESS_REFLECTION_FLAGS_NO_SUSPEND: u32 = 4;
pub const RTL_PROCESS_REFLECTION_FLAGS_NO_SYNCHRONIZE: u32 = 8;
pub const RTL_PROCESS_REFLECTION_FLAGS_NO_CLOSE_EVENT: u32 = 16;
pub const CONTEXT_ALIGN: u32 = 16;
pub const CONTEXT_FRAME_LENGTH: u32 = 1232;
pub const CONTEXT_EX_PADDING: u32 = 16;
pub const RTL_ACTIVATE_ACTIVATION_CONTEXT_EX_FLAG_RELEASE_ON_STACK_DEALLOCATION: u32 = 1;
pub const RTL_DEACTIVATE_ACTIVATION_CONTEXT_FLAG_FORCE_EARLY_DEACTIVATION: u32 = 1;
pub const FIND_ACTIVATION_CONTEXT_SECTION_KEY_RETURN_ACTIVATION_CONTEXT: u32 = 1;
pub const FIND_ACTIVATION_CONTEXT_SECTION_KEY_RETURN_FLAGS: u32 = 2;
pub const FIND_ACTIVATION_CONTEXT_SECTION_KEY_RETURN_ASSEMBLY_METADATA: u32 = 4;
pub const RTL_QUERY_INFORMATION_ACTIVATION_CONTEXT_FLAG_USE_ACTIVE_ACTIVATION_CONTEXT: u32 = 1;
pub const RTL_QUERY_INFORMATION_ACTIVATION_CONTEXT_FLAG_ACTIVATION_CONTEXT_IS_MODULE: u32 = 2;
pub const RTL_QUERY_INFORMATION_ACTIVATION_CONTEXT_FLAG_ACTIVATION_CONTEXT_IS_ADDRESS: u32 = 4;
pub const RTL_QUERY_INFORMATION_ACTIVATION_CONTEXT_FLAG_NO_ADDREF: u32 = 2147483648;
pub const RTL_IMAGE_NT_HEADER_EX_FLAG_NO_RANGE_CHECK: u32 = 1;
pub const RTL_CREATE_ENVIRONMENT_TRANSLATE: u32 = 1;
pub const RTL_CREATE_ENVIRONMENT_TRANSLATE_FROM_OEM: u32 = 2;
pub const RTL_CREATE_ENVIRONMENT_EMPTY: u32 = 4;
pub const RtlNtdllName: &[u8; 10] = b"ntdll.dll\0";
pub const RTL_DOS_SEARCH_PATH_FLAG_APPLY_ISOLATION_REDIRECTION: u32 = 1;
pub const RTL_DOS_SEARCH_PATH_FLAG_DISALLOW_DOT_RELATIVE_PATH_SEARCH: u32 = 2;
pub const RTL_DOS_SEARCH_PATH_FLAG_APPLY_DEFAULT_EXTENSION_WHEN_NOT_RELATIVE_PATH_EVEN_IF_FILE_HAS_EXTENSION: u32 = 4;
pub const IMAGE_FILE_NATIVE_MACHINE_I386: u32 = 1;
pub const IMAGE_FILE_NATIVE_MACHINE_AMD64: u32 = 2;
pub const IMAGE_FILE_NATIVE_MACHINE_ARMNT: u32 = 4;
pub const IMAGE_FILE_NATIVE_MACHINE_ARM64: u32 = 8;
pub const RTL_HEAP_SIGNATURE: u32 = 4293853166;
pub const RTL_HEAP_SEGMENT_SIGNATURE: u32 = 3723419118;
pub const HEAP_USAGE_ALLOCATED_BLOCKS: u32 = 16;
pub const HEAP_USAGE_FREE_BUFFER: u32 = 8;
pub const HeapExtendedInformation: u32 = 2;
pub const HeapTaggingInformation: u32 = 4;
pub const HeapStackDatabase: u32 = 5;
pub const HeapMemoryLimit: u32 = 6;
pub const HeapDetailedFailureInformation: u32 = 2147483649;
pub const HeapSetDebuggingInformation: u32 = 2147483650;
pub const HeapPerformanceCountersInformationStandardHeapVersion: u32 = 1;
pub const HeapPerformanceCountersInformationSegmentHeapVersion: u32 = 2;
pub const HeapExtendedProcessHeapInformationLevel: u32 = 1;
pub const HeapExtendedHeapInformationLevel: u32 = 2;
pub const HeapExtendedHeapRegionInformationLevel: u32 = 3;
pub const HeapExtendedHeapRangeInformationLevel: u32 = 4;
pub const HeapExtendedHeapBlockInformationLevel: u32 = 5;
pub const HeapExtendedHeapHeapPerfInformationLevel: u32 = 2147483648;
pub const HEAP_STACK_QUERY_VERSION: u32 = 2;
pub const HEAP_STACK_CONTROL_VERSION: u32 = 1;
pub const HEAP_STACK_CONTROL_FLAGS_STACKTRACE_ENABLE: u32 = 1;
pub const HEAP_STACK_CONTROL_FLAGS_STACKTRACE_DISABLE: u32 = 2;
pub const RTL_QUERY_PROCESS_MODULES: u32 = 1;
pub const RTL_QUERY_PROCESS_BACKTRACES: u32 = 2;
pub const RTL_QUERY_PROCESS_HEAP_SUMMARY: u32 = 4;
pub const RTL_QUERY_PROCESS_HEAP_TAGS: u32 = 8;
pub const RTL_QUERY_PROCESS_HEAP_ENTRIES: u32 = 16;
pub const RTL_QUERY_PROCESS_LOCKS: u32 = 32;
pub const RTL_QUERY_PROCESS_MODULES32: u32 = 64;
pub const RTL_QUERY_PROCESS_VERIFIER_OPTIONS: u32 = 128;
pub const RTL_QUERY_PROCESS_MODULESEX: u32 = 256;
pub const RTL_QUERY_PROCESS_HEAP_SEGMENTS: u32 = 512;
pub const RTL_QUERY_PROCESS_CS_OWNER: u32 = 1024;
pub const RTL_QUERY_PROCESS_NONINVASIVE: u32 = 2147483648;
pub const RTL_QUERY_PROCESS_NONINVASIVE_CS_OWNER: u32 = 2147485696;
pub const RTL_ERRORMODE_FAILCRITICALERRORS: u32 = 16;
pub const RTL_ERRORMODE_NOGPFAULTERRORBOX: u32 = 32;
pub const RTL_ERRORMODE_NOOPENFILEERRORBOX: u32 = 64;
pub const RTL_IMPORT_TABLE_HASH_REVISION: u32 = 1;
pub const SecondsToStartOf1980: u64 = 11960006400;
pub const SecondsToStartOf1970: u64 = 11644473600;
pub const RTL_ATOM_TABLE_DEFAULT_NUMBER_OF_BUCKETS: u32 = 37;
pub const RTL_ATOM_MAXIMUM_NAME_LENGTH: u32 = 255;
pub const RTL_ATOM_PINNED: u32 = 1;
pub const COMPOUND_ACE_IMPERSONATION: u32 = 1;
pub const RTL_ACQUIRE_PRIVILEGE_REVERT: u32 = 1;
pub const RTL_ACQUIRE_PRIVILEGE_PROCESS: u32 = 2;
pub const BOUNDARY_DESCRIPTOR_ADD_APPCONTAINER_SID: u32 = 1;
pub const RTL_WALK_USER_MODE_STACK: u32 = 1;
pub const RTL_WALK_VALID_FLAGS: u32 = 1;
pub const RTL_UNLOAD_EVENT_TRACE_NUMBER: u32 = 64;
pub const RTL_IMAGE_MITIGATION_OPTION_STATEMASK: u32 = 3;
pub const RTL_IMAGE_MITIGATION_OPTION_FORCEMASK: u32 = 4;
pub const RTL_IMAGE_MITIGATION_OPTION_OPTIONMASK: u32 = 8;
pub const RTL_IMAGE_MITIGATION_FLAG_RESET: u32 = 1;
pub const RTL_IMAGE_MITIGATION_FLAG_REMOVE: u32 = 2;
pub const RTL_IMAGE_MITIGATION_FLAG_OSDEFAULT: u32 = 4;
pub const RTL_IMAGE_MITIGATION_FLAG_AUDIT: u32 = 8;
pub const PSM_ACTIVATION_TOKEN_PACKAGED_APPLICATION: u32 = 1;
pub const PSM_ACTIVATION_TOKEN_SHARED_ENTITY: u32 = 2;
pub const PSM_ACTIVATION_TOKEN_FULL_TRUST: u32 = 4;
pub const PSM_ACTIVATION_TOKEN_NATIVE_SERVICE: u32 = 8;
pub const PSM_ACTIVATION_TOKEN_DEVELOPMENT_APP: u32 = 16;
pub const BREAKAWAY_INHIBITED: u32 = 32;
pub const WNF_STATE_KEY: u64 = 4739561890659434612;
pub const RTL_RESOURCE_FLAG_LONG_TERM: u32 = 1;
pub const RTL_DRIVE_LETTER_VALID: u32 = 1;
pub const INVALID_ACTIVATION_CONTEXT: HANDLE = HANDLE(-1isize as *mut std::ffi::c_void);
pub const ACTCTX_PROCESS_DEFAULT: HANDLE = HANDLE(0isize as *mut std::ffi::c_void);
pub const ACTCTX_EMPTY: HANDLE = HANDLE(-3isize as *mut std::ffi::c_void);
pub const ACTCTX_SYSTEM_DEFAULT: HANDLE = HANDLE(-4isize as *mut std::ffi::c_void);
pub const RTL_HEAP_BUSY: u32 = 1;
pub const RTL_HEAP_SEGMENT: u32 = 2;
pub const RTL_HEAP_SETTABLE_VALUE: u32 = 16;
pub const RTL_HEAP_SETTABLE_FLAG1: u32 = 32;
pub const RTL_HEAP_SETTABLE_FLAG2: u32 = 64;
pub const RTL_HEAP_SETTABLE_FLAG3: u32 = 128;
pub const RTL_HEAP_SETTABLE_FLAGS: u32 = 224;
pub const RTL_HEAP_UNCOMMITTED_RANGE: u32 = 256;
pub const RTL_HEAP_PROTECTED_ENTRY: u32 = 512;
pub const RTL_SEGHEAP_MEM_SOURCE_ANY_NODE: u32 = 4294967295;
pub const RTL_HANDLE_ALLOCATED: u32 = 1;
pub const RTL_WAITER_DEREGISTER_WAIT_FOR_COMPLETION: HANDLE =
    HANDLE(-1isize as *mut std::ffi::c_void);
pub const RTL_TIMER_DELETE_WAIT_FOR_COMPLETION: HANDLE = HANDLE(-1isize as *mut std::ffi::c_void);
pub const PHCM_APPLICATION_DEFAULT: char = 0 as char;
pub const PHCM_DISGUISE_PLACEHOLDERS: char = 1 as char;
pub const PHCM_EXPOSE_PLACEHOLDERS: char = 2 as char;
pub const PHCM_MAX: char = 2 as char;
pub const PHCM_ERROR_INVALID_PARAMETER: char = -1i8 as u8 as char;
pub const PHCM_ERROR_NO_TEB: char = -2i8 as u8 as char;
pub const CONTEXT_EX_LENGTH: u32 = 32;
pub const RTL_ATOM_MAXIMUM_INTEGER_ATOM: u32 = 49152;
pub const RTL_ATOM_INVALID_ATOM: u32 = 0;
pub const PHCM_DISGUISE_FULL_PLACEHOLDERS: char = 3 as char;
pub const PHCM_ERROR_NO_PEB: char = -3i8 as u8 as char;
pub const RtlDosPathSeperatorsString: UNICODE_STRING = UNICODE_STRING {
    Length: 2,
    MaximumLength: 2,
    Buffer: PWSTR(w!("\\/").as_ptr() as *mut u16),
};
pub const RtlAlternateDosPathSeperatorString: UNICODE_STRING = UNICODE_STRING {
    Length: 1,
    MaximumLength: 1,
    Buffer: PWSTR(w!("/").as_ptr() as *mut u16),
};
pub const RtlNtPathSeperatorString: UNICODE_STRING = UNICODE_STRING {
    Length: 1,
    MaximumLength: 1,
    Buffer: PWSTR(w!("\\").as_ptr() as *mut u16),
};
pub const RtlDosDevicesPrefix: UNICODE_STRING = UNICODE_STRING {
    Length: 4,
    MaximumLength: 4,
    Buffer: PWSTR(w!("\\??\\").as_ptr() as *mut u16),
};
pub const RtlDosDevicesUncPrefix: UNICODE_STRING = UNICODE_STRING {
    Length: 8,
    MaximumLength: 8,
    Buffer: PWSTR(w!("\\??\\UNC\\").as_ptr() as *mut u16),
};
pub const RtlSlashSlashDot: UNICODE_STRING = UNICODE_STRING {
    Length: 4,
    MaximumLength: 4,
    Buffer: PWSTR(w!("\\\\.\\").as_ptr() as *mut u16),
};
pub const RtlNullString: UNICODE_STRING = UNICODE_STRING {
    Length: 0,
    MaximumLength: 0,
    Buffer: PWSTR(w!("").as_ptr() as *mut u16),
};
pub const RtlWin32NtRootSlash: UNICODE_STRING = UNICODE_STRING {
    Length: 4,
    MaximumLength: 4,
    Buffer: PWSTR(w!("\\\\?\\").as_ptr() as *mut u16),
};
pub const RtlWin32NtRoot: UNICODE_STRING = UNICODE_STRING {
    Length: 3,
    MaximumLength: 4,
    Buffer: PWSTR(w!("\\\\?").as_ptr() as *mut u16),
};
pub const RtlWin32NtUncRoot: UNICODE_STRING = UNICODE_STRING {
    Length: 7,
    MaximumLength: 7,
    Buffer: PWSTR(w!("\\\\?\\UNC").as_ptr() as *mut u16),
};
pub const RtlWin32NtUncRootSlash: UNICODE_STRING = UNICODE_STRING {
    Length: 8,
    MaximumLength: 8,
    Buffer: PWSTR(w!("\\\\?\\UNC\\").as_ptr() as *mut u16),
};
pub const RtlDefaultExtension: UNICODE_STRING = UNICODE_STRING {
    Length: 4,
    MaximumLength: 4,
    Buffer: PWSTR(w!(".DLL").as_ptr() as *mut u16),
};

#[repr(C)]
pub struct RTL_RB_TREE {
    pub Root: *mut RTL_BALANCED_NODE,
    pub Min: *mut RTL_BALANCED_NODE,
}

impl Default for RTL_RB_TREE {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_RB_TREE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RTL_RB_TREE {{  }}")
    }
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlRbInsertNodeEx(
        Tree: *mut RTL_RB_TREE,
        Parent: *mut RTL_BALANCED_NODE,
        Right: bool,
        Node: *mut RTL_BALANCED_NODE,
    ) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlRbRemoveNode(Tree: *mut RTL_RB_TREE, Node: *mut RTL_BALANCED_NODE) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlInitializeCriticalSection(CriticalSection: *mut CRITICAL_SECTION) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlInitializeCriticalSectionAndSpinCount(
        CriticalSection: *mut CRITICAL_SECTION,
        SpinCount: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlInitializeCriticalSectionEx(
        CriticalSection: *mut CRITICAL_SECTION,
        SpinCount: u32,
        Flags: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlDeleteCriticalSection(CriticalSection: *mut CRITICAL_SECTION) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlEnterCriticalSection(CriticalSection: *mut CRITICAL_SECTION) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlLeaveCriticalSection(CriticalSection: *mut CRITICAL_SECTION) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlTryEnterCriticalSection(CriticalSection: *mut CRITICAL_SECTION) -> u32;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlIsCriticalSectionLocked(CriticalSection: *mut CRITICAL_SECTION) -> u32;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlIsCriticalSectionLockedByThread(CriticalSection: *mut CRITICAL_SECTION) -> u32;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlGetCriticalSectionRecursionCount(CriticalSection: *mut CRITICAL_SECTION) -> u32;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlSetCriticalSectionSpinCount(
        CriticalSection: *mut CRITICAL_SECTION,
        SpinCount: u32,
    ) -> u32;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlQueryCriticalSectionOwner(EventHandle: HANDLE) -> HANDLE;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlCheckForOrphanedCriticalSections(ThreadHandle: HANDLE);
}

#[repr(C)]
pub struct RTL_RESOURCE {
    pub CriticalSection: CRITICAL_SECTION,
    pub SharedSemaphore: HANDLE,
    pub NumberOfWaitingShared: u32,
    pub ExclusiveSemaphore: HANDLE,
    pub NumberOfWaitingExclusive: u32,
    pub NumberOfActive: i32,
    pub ExclusiveOwnerThread: HANDLE,
    pub Flags: u32,
    pub DebugInfo: *mut CRITICAL_SECTION_DEBUG,
}

impl Default for RTL_RESOURCE {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_RESOURCE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RTL_RESOURCE {{  }}")
    }
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlInitializeResource(Resource: *mut RTL_RESOURCE);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlDeleteResource(Resource: *mut RTL_RESOURCE);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlAcquireResourceShared(Resource: *mut RTL_RESOURCE, Wait: bool) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlAcquireResourceExclusive(Resource: *mut RTL_RESOURCE, Wait: bool) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlReleaseResource(Resource: *mut RTL_RESOURCE);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlConvertSharedToExclusive(Resource: *mut RTL_RESOURCE);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlConvertExclusiveToShared(Resource: *mut RTL_RESOURCE);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlInitializeSRWLock(SRWLock: *mut SRWLOCK);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlAcquireSRWLockExclusive(SRWLock: *mut SRWLOCK);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlAcquireSRWLockShared(SRWLock: *mut SRWLOCK);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlReleaseSRWLockExclusive(SRWLock: *mut SRWLOCK);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlReleaseSRWLockShared(SRWLock: *mut SRWLOCK);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlTryAcquireSRWLockExclusive(SRWLock: *mut SRWLOCK) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlTryAcquireSRWLockShared(SRWLock: *mut SRWLOCK) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlAcquireReleaseSRWLockExclusive(SRWLock: *mut SRWLOCK);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlInitializeConditionVariable(ConditionVariable: *mut CONDITION_VARIABLE);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlSleepConditionVariableCS(
        ConditionVariable: *mut CONDITION_VARIABLE,
        CriticalSection: *mut CRITICAL_SECTION,
        Timeout: *mut i64,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlSleepConditionVariableSRW(
        ConditionVariable: *mut CONDITION_VARIABLE,
        SRWLock: *mut SRWLOCK,
        Timeout: *mut i64,
        Flags: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlWakeConditionVariable(ConditionVariable: *mut CONDITION_VARIABLE);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlWakeAllConditionVariable(ConditionVariable: *mut CONDITION_VARIABLE);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlInitBarrier(
        Barrier: *mut SYNCHRONIZATION_BARRIER,
        TotalThreads: u32,
        SpinCount: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlDeleteBarrier(Barrier: *mut SYNCHRONIZATION_BARRIER) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlBarrier(Barrier: *mut SYNCHRONIZATION_BARRIER, Flags: u32) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlBarrierForDelete(Barrier: *mut SYNCHRONIZATION_BARRIER, Flags: u32) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlWaitOnAddress(
        Address: *mut std::os::raw::c_void,
        CompareAddress: *mut std::ffi::c_void,
        AddressSize: usize,
        Timeout: *mut i64,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlWakeAddressAll(Address: *mut std::ffi::c_void);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlWakeAddressSingle(Address: *mut std::ffi::c_void);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlAppendAsciizToString(Destination: *mut STRING, Source: *mut i8) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlCreateUnicodeStringFromAsciiz(
        DestinationString: *mut UNICODE_STRING,
        SourceString: *mut i8,
    ) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlFindUnicodeSubstring(
        FullString: *mut UNICODE_STRING,
        SearchString: *mut UNICODE_STRING,
        CaseInSensitive: bool,
    ) -> PWSTR;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlFindCharInUnicodeString(
        Flags: u32,
        StringToSearch: *mut UNICODE_STRING,
        CharSet: *mut UNICODE_STRING,
        NonInclusivePrefixLength: *mut u16,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlEraseUnicodeString(String: *mut UNICODE_STRING);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlAnsiCharToUnicodeChar(SourceCharacter: *mut *mut u8) -> u16;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlUpcaseUnicodeStringToAnsiString(
        DestinationString: *mut STRING,
        SourceString: *mut UNICODE_STRING,
        AllocateDestinationString: bool,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlConsoleMultiByteToUnicodeN(
        UnicodeString: PWSTR,
        MaxBytesInUnicodeString: u32,
        BytesInUnicodeString: *mut u32,
        MultiByteString: *const i8,
        BytesInMultiByteString: u32,
        pdwSpecialChar: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlInitNlsTables(
        AnsiNlsBase: *mut u16,
        OemNlsBase: *mut u16,
        LanguageNlsBase: *mut u16,
        TableInfo: *mut NLSTABLEINFO,
    );
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlResetRtlTranslations(TableInfo: *mut NLSTABLEINFO);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlIsTextUnicode(Buffer: *mut std::ffi::c_void, Size: u32, Result: *mut u32) -> bool;
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum RTL_NORM_FORM {
    NormOther = 0,
    NormC = 1,
    NormD = 2,
    NormKC = 5,
    NormKD = 6,
    NormIdna = 13,
    DisallowUnassigned = 256,
    NormCDisallowUnassigned = 257,
    NormDDisallowUnassigned = 258,
    NormKCDisallowUnassigned = 261,
    NormKDDisallowUnassigned = 262,
    NormIdnaDisallowUnassigned = 269,
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlIsNameInExpression(
        Expression: *mut UNICODE_STRING,
        Name: *mut UNICODE_STRING,
        IgnoreCase: bool,
        UpcaseTable: PWSTR,
    ) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlIsNameInUnUpcasedExpression(
        Expression: *mut UNICODE_STRING,
        Name: *mut UNICODE_STRING,
        IgnoreCase: bool,
        UpcaseTable: PWSTR,
    ) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlDoesNameContainWildCards(Expression: *mut UNICODE_STRING) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlEqualDomainName(
        String1: *mut UNICODE_STRING,
        String2: *mut UNICODE_STRING,
    ) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlEqualComputerName(
        String1: *mut UNICODE_STRING,
        String2: *mut UNICODE_STRING,
    ) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlDnsHostNameToComputerName(
        ComputerNameString: *mut UNICODE_STRING,
        DnsHostNameString: *mut UNICODE_STRING,
        AllocateComputerNameString: bool,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlStringFromGUIDEx(
        Guid: *mut GUID,
        GuidString: *mut UNICODE_STRING,
        AllocateGuidString: bool,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlConvertLCIDToString(
        LcidValue: u32,
        Base: u32,
        Padding: u32,
        pResultBuf: PWSTR,
        Size: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlIsValidLocaleName(LocaleName: PCWSTR, Flags: u32) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlGetParentLocaleName(
        LocaleName: PCWSTR,
        ParentLocaleName: *mut UNICODE_STRING,
        Flags: u32,
        AllocateDestinationString: bool,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlLcidToLocaleName(
        lcid: u32,
        LocaleName: *mut UNICODE_STRING,
        Flags: u32,
        AllocateDestinationString: bool,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlLocaleNameToLcid(LocaleName: PCWSTR, lcid: *mut u32, Flags: u32) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlLCIDToCultureName(Lcid: u32, String: *mut UNICODE_STRING) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlCultureNameToLCID(String: *mut UNICODE_STRING, Lcid: *mut u32) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlCleanUpTEBLangLists();
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlGetLocaleFileMappingAddress(
        BaseAddress: *mut *mut std::ffi::c_void,
        DefaultLocaleId: *mut u32,
        DefaultCasingTableSize: *mut i64,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlGetCurrentPeb() -> *mut PEB;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlAcquirePebLock();
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlReleasePebLock();
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlTryAcquirePebLock() -> u32;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlAllocateFromPeb(Size: u32, Block: *mut *mut std::ffi::c_void) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlFreeToPeb(Block: *mut std::ffi::c_void, Size: u32) -> NTSTATUS;
}

#[repr(C)]
pub struct CURDIR {
    pub DosPath: UNICODE_STRING,
    pub Handle: HANDLE,
}

impl Default for CURDIR {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for CURDIR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CURDIR {{  }}")
    }
}

#[repr(C)]
pub struct RTL_DRIVE_LETTER_CURDIR {
    pub Flags: u16,
    pub Length: u16,
    pub TimeStamp: u32,
    pub DosPath: STRING,
}

impl Default for RTL_DRIVE_LETTER_CURDIR {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_DRIVE_LETTER_CURDIR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RTL_DRIVE_LETTER_CURDIR {{  }}")
    }
}

#[repr(C)]
pub struct RTL_USER_PROCESS_PARAMETERS {
    pub MaximumLength: u32,
    pub Length: u32,
    pub Flags: u32,
    pub DebugFlags: u32,
    pub ConsoleHandle: HANDLE,
    pub ConsoleFlags: u32,
    pub StandardInput: HANDLE,
    pub StandardOutput: HANDLE,
    pub StandardError: HANDLE,
    pub CurrentDirectory: CURDIR,
    pub DllPath: UNICODE_STRING,
    pub ImagePathName: UNICODE_STRING,
    pub CommandLine: UNICODE_STRING,
    pub Environment: *mut std::ffi::c_void,
    pub StartingX: u32,
    pub StartingY: u32,
    pub CountX: u32,
    pub CountY: u32,
    pub CountCharsX: u32,
    pub CountCharsY: u32,
    pub FillAttribute: u32,
    pub WindowFlags: u32,
    pub ShowWindowFlags: u32,
    pub WindowTitle: UNICODE_STRING,
    pub DesktopInfo: UNICODE_STRING,
    pub ShellInfo: UNICODE_STRING,
    pub RuntimeData: UNICODE_STRING,
    pub CurrentDirectories: [RTL_DRIVE_LETTER_CURDIR; 32],
    pub EnvironmentSize: usize,
    pub EnvironmentVersion: usize,
    pub PackageDependencyData: *mut std::ffi::c_void,
    pub ProcessGroupId: u32,
    pub LoaderThreads: u32,
    pub RedirectionDllName: UNICODE_STRING,
    pub HeapPartitionName: UNICODE_STRING,
    pub DefaultThreadpoolCpuSetMasks: usize,
    pub DefaultThreadpoolCpuSetMaskCount: u32,
    pub DefaultThreadpoolThreadMaximum: u32,
    pub HeapMemoryTypeMask: u32,
}

impl Default for RTL_USER_PROCESS_PARAMETERS {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_USER_PROCESS_PARAMETERS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RTL_USER_PROCESS_PARAMETERS {{ CurrentDirectory: {:?}, CurrentDirectories: {:?} }}",
            self.CurrentDirectory, self.CurrentDirectories
        )
    }
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlCreateProcessParameters(
        pProcessParameters: *mut *mut RTL_USER_PROCESS_PARAMETERS,
        ImagePathName: *mut UNICODE_STRING,
        DllPath: *mut UNICODE_STRING,
        CurrentDirectory: *mut UNICODE_STRING,
        CommandLine: *mut UNICODE_STRING,
        Environment: *mut std::ffi::c_void,
        WindowTitle: *mut UNICODE_STRING,
        DesktopInfo: *mut UNICODE_STRING,
        ShellInfo: *mut UNICODE_STRING,
        RuntimeData: *mut UNICODE_STRING,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlCreateProcessParametersEx(
        pProcessParameters: *mut *mut RTL_USER_PROCESS_PARAMETERS,
        ImagePathName: *mut UNICODE_STRING,
        DllPath: *mut UNICODE_STRING,
        CurrentDirectory: *mut UNICODE_STRING,
        CommandLine: *mut UNICODE_STRING,
        Environment: *mut std::ffi::c_void,
        WindowTitle: *mut UNICODE_STRING,
        DesktopInfo: *mut UNICODE_STRING,
        ShellInfo: *mut UNICODE_STRING,
        RuntimeData: *mut UNICODE_STRING,
        Flags: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlDestroyProcessParameters(
        ProcessParameters: *mut RTL_USER_PROCESS_PARAMETERS,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlNormalizeProcessParams(
        ProcessParameters: *mut RTL_USER_PROCESS_PARAMETERS,
    ) -> *mut RTL_USER_PROCESS_PARAMETERS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlDeNormalizeProcessParams(
        ProcessParameters: *mut RTL_USER_PROCESS_PARAMETERS,
    ) -> *mut RTL_USER_PROCESS_PARAMETERS;
}

#[repr(C)]
pub struct RTL_USER_PROCESS_INFORMATION {
    pub Length: u32,
    pub ProcessHandle: HANDLE,
    pub ThreadHandle: HANDLE,
    pub ClientId: CLIENT_ID,
    pub ImageInformation: SECTION_IMAGE_INFORMATION,
}

impl Default for RTL_USER_PROCESS_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_USER_PROCESS_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RTL_USER_PROCESS_INFORMATION {{  }}")
    }
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlCreateUserProcess(
        NtImagePathName: *mut UNICODE_STRING,
        AttributesDeprecated: u32,
        ProcessParameters: *mut RTL_USER_PROCESS_PARAMETERS,
        ProcessSecurityDescriptor: *mut SECURITY_DESCRIPTOR,
        ThreadSecurityDescriptor: *mut SECURITY_DESCRIPTOR,
        ParentProcess: HANDLE,
        InheritHandles: bool,
        DebugPort: HANDLE,
        TokenHandle: HANDLE,
        ProcessInformation: *mut RTL_USER_PROCESS_INFORMATION,
    ) -> NTSTATUS;
}

#[repr(C)]
pub struct RTL_USER_PROCESS_EXTENDED_PARAMETERS {
    pub Version: u16,
    pub NodeNumber: u16,
    pub ProcessSecurityDescriptor: *mut SECURITY_DESCRIPTOR,
    pub ThreadSecurityDescriptor: *mut SECURITY_DESCRIPTOR,
    pub ParentProcess: HANDLE,
    pub DebugPort: HANDLE,
    pub TokenHandle: HANDLE,
    pub JobHandle: HANDLE,
}

impl Default for RTL_USER_PROCESS_EXTENDED_PARAMETERS {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_USER_PROCESS_EXTENDED_PARAMETERS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RTL_USER_PROCESS_EXTENDED_PARAMETERS {{  }}")
    }
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlCreateUserProcessEx(
        NtImagePathName: *mut UNICODE_STRING,
        ProcessParameters: *mut RTL_USER_PROCESS_PARAMETERS,
        InheritHandles: bool,
        ProcessExtendedParameters: *mut RTL_USER_PROCESS_EXTENDED_PARAMETERS,
        ProcessInformation: *mut RTL_USER_PROCESS_INFORMATION,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlExitUserProcess(ExitStatus: NTSTATUS) -> !;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlCloneUserProcess(
        ProcessFlags: u32,
        ProcessSecurityDescriptor: *mut SECURITY_DESCRIPTOR,
        ThreadSecurityDescriptor: *mut SECURITY_DESCRIPTOR,
        DebugPort: HANDLE,
        ProcessInformation: *mut RTL_USER_PROCESS_INFORMATION,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlUpdateClonedCriticalSection(CriticalSection: *mut CRITICAL_SECTION);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlUpdateClonedSRWLock(SRWLock: *mut SRWLOCK, Shared: u32);
}

#[repr(C)]
pub struct RTLP_PROCESS_REFLECTION_REFLECTION_INFORMATION {
    pub ReflectionProcessHandle: HANDLE,
    pub ReflectionThreadHandle: HANDLE,
    pub ReflectionClientId: CLIENT_ID,
}

impl Default for RTLP_PROCESS_REFLECTION_REFLECTION_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTLP_PROCESS_REFLECTION_REFLECTION_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RTLP_PROCESS_REFLECTION_REFLECTION_INFORMATION {{  }}")
    }
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlCreateProcessReflection(
        ProcessHandle: HANDLE,
        Flags: u32,
        StartRoutine: *mut std::ffi::c_void,
        StartContext: *mut std::ffi::c_void,
        EventHandle: HANDLE,
        ReflectionInformation: *mut RTLP_PROCESS_REFLECTION_REFLECTION_INFORMATION,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlSetProcessIsCritical(
        NewValue: bool,
        OldValue: *mut bool,
        CheckFlag: bool,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlSetThreadIsCritical(
        NewValue: bool,
        OldValue: *mut bool,
        CheckFlag: bool,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlValidProcessProtection(ProcessProtection: PS_PROTECTION) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlTestProtectedAccess(Source: PS_PROTECTION, Target: PS_PROTECTION) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlIsCurrentProcess(ProcessHandle: HANDLE) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlIsCurrentThread(ThreadHandle: HANDLE) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlCreateUserThread(
        ProcessHandle: HANDLE,
        ThreadSecurityDescriptor: *mut SECURITY_DESCRIPTOR,
        CreateSuspended: bool,
        ZeroBits: u32,
        MaximumStackSize: usize,
        CommittedStackSize: usize,
        StartAddress: PUSER_THREAD_START_ROUTINE,
        Parameter: *mut std::ffi::c_void,
        ThreadHandle: *mut HANDLE,
        ClientId: *mut CLIENT_ID,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlExitUserThread(ExitStatus: NTSTATUS) -> !;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlIsCurrentThreadAttachExempt() -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlCreateUserStack(
        CommittedStackSize: usize,
        MaximumStackSize: usize,
        ZeroBits: usize,
        PageSize: usize,
        ReserveAlignment: usize,
        InitialTeb: *mut INITIAL_TEB,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlFreeUserStack(AllocationBase: *mut std::ffi::c_void) -> NTSTATUS;
}

#[repr(C)]
pub struct CONTEXT_CHUNK {
    pub Offset: i32,
    pub Length: u32,
}

impl Default for CONTEXT_CHUNK {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for CONTEXT_CHUNK {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CONTEXT_CHUNK {{  }}")
    }
}

#[repr(C)]
pub struct CONTEXT_EX {
    pub All: CONTEXT_CHUNK,
    pub Legacy: CONTEXT_CHUNK,
    pub XState: CONTEXT_CHUNK,
    pub KernelCet: CONTEXT_CHUNK,
}

impl Default for CONTEXT_EX {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for CONTEXT_EX {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CONTEXT_EX {{ All: {:?}, Legacy: {:?}, XState: {:?}, KernelCet: {:?} }}",
            self.All, self.Legacy, self.XState, self.KernelCet
        )
    }
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlInitializeContext(
        Reserved: HANDLE,
        Context: *mut CONTEXT,
        Parameter: *mut std::ffi::c_void,
        InitialPc: *mut std::ffi::c_void,
        InitialSp: *mut std::ffi::c_void,
    ) -> u64;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlInitializeExtendedContext(
        Context: *mut CONTEXT,
        ContextFlags: u32,
        ContextEx: *mut *mut CONTEXT_EX,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlInitializeExtendedContext2(
        Context: *mut CONTEXT,
        ContextFlags: u32,
        ContextEx: *mut *mut CONTEXT_EX,
        EnabledExtendedFeatures: u64,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlCopyContext(
        Context: *mut CONTEXT,
        ContextFlags: u32,
        Source: *mut CONTEXT,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlCopyExtendedContext(
        Destination: *mut CONTEXT_EX,
        ContextFlags: u32,
        Source: *mut CONTEXT_EX,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlGetExtendedContextLength(ContextFlags: u32, ContextLength: *mut u32) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlGetExtendedContextLength2(
        ContextFlags: u32,
        ContextLength: *mut u32,
        EnabledExtendedFeatures: u64,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlGetExtendedFeaturesMask(ContextEx: *mut CONTEXT_EX) -> u64;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlLocateExtendedFeature(
        ContextEx: *mut CONTEXT_EX,
        FeatureId: u32,
        Length: *mut u32,
    ) -> *mut std::ffi::c_void;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlLocateLegacyContext(ContextEx: *mut CONTEXT_EX, Length: *mut u32) -> *mut CONTEXT;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlSetExtendedFeaturesMask(ContextEx: *mut CONTEXT_EX, FeatureMask: u64);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlWow64GetThreadContext(
        ThreadHandle: HANDLE,
        ThreadContext: *mut WOW64_CONTEXT,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlWow64SetThreadContext(
        ThreadHandle: HANDLE,
        ThreadContext: *mut WOW64_CONTEXT,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlRemoteCall(
        ProcessHandle: HANDLE,
        ThreadHandle: HANDLE,
        CallSite: *mut std::ffi::c_void,
        ArgumentCount: u32,
        Arguments: *mut usize,
        PassContext: bool,
        AlreadySuspended: bool,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlAddVectoredExceptionHandler(
        First: u32,
        Handler: PVECTORED_EXCEPTION_HANDLER,
    ) -> *mut std::ffi::c_void;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlRemoveVectoredExceptionHandler(Handle: *mut std::ffi::c_void) -> u32;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlAddVectoredContinueHandler(
        First: u32,
        Handler: PVECTORED_EXCEPTION_HANDLER,
    ) -> *mut std::ffi::c_void;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlRemoveVectoredContinueHandler(Handle: *mut std::ffi::c_void) -> u32;
}

pub type PRTLP_UNHANDLED_EXCEPTION_FILTER =
    Option<unsafe extern "system" fn(ExceptionInfo: *mut EXCEPTION_POINTERS) -> u32>;

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlSetUnhandledExceptionFilter(
        UnhandledExceptionFilter: PRTLP_UNHANDLED_EXCEPTION_FILTER,
    );
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlUnhandledExceptionFilter(ExceptionPointers: *mut EXCEPTION_POINTERS) -> i32;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlUnhandledExceptionFilter2(
        ExceptionPointers: *mut EXCEPTION_POINTERS,
        Flags: u32,
    ) -> i32;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlKnownExceptionFilter(ExceptionPointers: *mut EXCEPTION_POINTERS) -> i32;
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FUNCTION_TABLE_TYPE {
    RF_SORTED = 0,
    RF_UNSORTED = 1,
    RF_CALLBACK = 2,
    RF_KERNEL_DYNAMIC = 3,
}

#[repr(C)]
pub struct DYNAMIC_FUNCTION_TABLE {
    pub ListEntry: LIST_ENTRY,
    pub FunctionTable: *mut IMAGE_RUNTIME_FUNCTION_ENTRY,
    pub TimeStamp: i64,
    pub MinimumAddress: u64,
    pub MaximumAddress: u64,
    pub BaseAddress: u64,
    pub Callback: PGET_RUNTIME_FUNCTION_CALLBACK,
    pub Context: *mut std::ffi::c_void,
    pub OutOfProcessCallbackDll: PWSTR,
    pub Type: FUNCTION_TABLE_TYPE,
    pub EntryCount: u32,
    pub TreeNodeMin: RTL_BALANCED_NODE,
    pub TreeNodeMax: RTL_BALANCED_NODE,
}

impl Default for DYNAMIC_FUNCTION_TABLE {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for DYNAMIC_FUNCTION_TABLE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DYNAMIC_FUNCTION_TABLE {{ Type: {:?} }}", self.Type)
    }
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlGetFunctionTableListHead() -> *mut LIST_ENTRY;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlGetActiveActivationContext(ActivationContext: *mut ACTIVATION_CONTEXT) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlAddRefActivationContext(ActivationContext: *mut ACTIVATION_CONTEXT);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlReleaseActivationContext(ActivationContext: *mut ACTIVATION_CONTEXT);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlZombifyActivationContext(ActivationContext: *mut ACTIVATION_CONTEXT) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlIsActivationContextActive(ActivationContext: *mut ACTIVATION_CONTEXT) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlActivateActivationContext(
        Flags: u32,
        ActivationContext: *mut ACTIVATION_CONTEXT,
        Cookie: *mut usize,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlActivateActivationContextEx(
        Flags: u32,
        Teb: *mut TEB,
        ActivationContext: *mut ACTIVATION_CONTEXT,
        Cookie: *mut usize,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlDeactivateActivationContext(Flags: u32, Cookie: usize);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlCreateActivationContext(
        Flags: u32,
        ActivationContextData: *mut ACTIVATION_CONTEXT_DATA,
        ExtraBytes: u32,
        NotificationRoutine: PACTIVATION_CONTEXT_NOTIFY_ROUTINE,
        NotificationContext: *mut std::ffi::c_void,
        ActivationContext: *mut *mut ACTIVATION_CONTEXT,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlFindActivationContextSectionString(
        Flags: u32,
        ExtensionGuid: *mut GUID,
        SectionId: u32,
        StringToFind: *mut UNICODE_STRING,
        ReturnedData: *mut ACTCTX_SECTION_KEYED_DATA,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlFindActivationContextSectionGuid(
        Flags: u32,
        ExtensionGuid: *mut GUID,
        SectionId: u32,
        GuidToFind: *mut GUID,
        ReturnedData: *mut ACTCTX_SECTION_KEYED_DATA,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlQueryActivationContextApplicationSettings(
        Flags: u32,
        ActivationContext: *mut ACTIVATION_CONTEXT,
        SettingsNameSpace: PWSTR,
        SettingName: PWSTR,
        Buffer: PWSTR,
        BufferLength: usize,
        RequiredLength: *mut usize,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlQueryInformationActivationContext(
        Flags: u32,
        ActivationContext: *mut ACTIVATION_CONTEXT,
        SubInstanceIndex: *mut ACTIVATION_CONTEXT_QUERY_INDEX,
        ActivationContextInformationClass: ACTIVATION_CONTEXT_INFO_CLASS,
        ActivationContextInformation: *mut std::ffi::c_void,
        ActivationContextInformationLength: usize,
        ReturnLength: *mut usize,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlQueryInformationActiveActivationContext(
        ActivationContextInformationClass: ACTIVATION_CONTEXT_INFO_CLASS,
        ActivationContextInformation: *mut std::ffi::c_void,
        ActivationContextInformationLength: usize,
        ReturnLength: *mut usize,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlImageNtHeader(BaseOfImage: *mut std::ffi::c_void) -> *mut IMAGE_NT_HEADERS64;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlImageNtHeaderEx(
        Flags: u32,
        BaseOfImage: *mut std::ffi::c_void,
        Size: u64,
        OutHeaders: *mut *mut IMAGE_NT_HEADERS64,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlAddressInSectionTable(
        NtHeaders: *mut IMAGE_NT_HEADERS64,
        BaseOfImage: *mut std::ffi::c_void,
        VirtualAddress: u32,
    ) -> *mut std::ffi::c_void;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlSectionTableFromVirtualAddress(
        NtHeaders: *mut IMAGE_NT_HEADERS64,
        BaseOfImage: *mut std::ffi::c_void,
        VirtualAddress: u32,
    ) -> *mut IMAGE_SECTION_HEADER;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlImageDirectoryEntryToData(
        BaseOfImage: *mut std::ffi::c_void,
        MappedAsImage: bool,
        DirectoryEntry: u16,
        Size: *mut u32,
    ) -> *mut std::ffi::c_void;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlImageRvaToSection(
        NtHeaders: *mut IMAGE_NT_HEADERS64,
        BaseOfImage: *mut std::ffi::c_void,
        Rva: u32,
    ) -> *mut IMAGE_SECTION_HEADER;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlImageRvaToVa(
        NtHeaders: *mut IMAGE_NT_HEADERS64,
        BaseOfImage: *mut std::ffi::c_void,
        Rva: u32,
        LastRvaSection: *mut *mut IMAGE_SECTION_HEADER,
    ) -> *mut std::ffi::c_void;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlFindExportedRoutineByName(
        BaseOfImage: *mut std::ffi::c_void,
        RoutineName: *mut i8,
    ) -> *mut std::ffi::c_void;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlGuardCheckLongJumpTarget(
        PcValue: *mut std::ffi::c_void,
        IsFastFail: bool,
        IsLongJumpTarget: *mut bool,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlCreateEnvironment(
        CloneCurrentEnvironment: bool,
        Environment: *mut *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlCreateEnvironmentEx(
        SourceEnv: *mut std::ffi::c_void,
        Environment: *mut *mut std::ffi::c_void,
        Flags: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlDestroyEnvironment(Environment: *mut std::ffi::c_void) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlSetCurrentEnvironment(
        Environment: *mut std::ffi::c_void,
        PreviousEnvironment: *mut *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlSetEnvironmentVar(
        Environment: *mut *mut std::ffi::c_void,
        Name: PCWSTR,
        NameLength: usize,
        Value: PCWSTR,
        ValueLength: usize,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlSetEnvironmentVariable(
        Environment: *mut *mut std::ffi::c_void,
        Name: *mut UNICODE_STRING,
        Value: *mut UNICODE_STRING,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlQueryEnvironmentVariable(
        Environment: *mut std::ffi::c_void,
        Name: PCWSTR,
        NameLength: usize,
        Value: PWSTR,
        ValueLength: usize,
        ReturnLength: *mut usize,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlQueryEnvironmentVariable_U(
        Environment: *mut std::ffi::c_void,
        Name: *mut UNICODE_STRING,
        Value: *mut UNICODE_STRING,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlExpandEnvironmentStrings(
        Environment: *mut std::ffi::c_void,
        Source: PCWSTR,
        SourceLength: usize,
        Destination: PWSTR,
        DestinationLength: usize,
        ReturnLength: *mut usize,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlExpandEnvironmentStrings_U(
        Environment: *mut std::ffi::c_void,
        Source: *mut UNICODE_STRING,
        Destination: *mut UNICODE_STRING,
        ReturnedLength: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlSetEnvironmentStrings(NewEnvironment: PCWSTR, NewEnvironmentSize: usize) -> NTSTATUS;
}

#[repr(C)]
pub struct RTLP_CURDIR_REF {
    pub ReferenceCount: i32,
    pub DirectoryHandle: HANDLE,
}

impl Default for RTLP_CURDIR_REF {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTLP_CURDIR_REF {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RTLP_CURDIR_REF {{  }}")
    }
}

#[repr(C)]
pub struct RTL_RELATIVE_NAME_U {
    pub RelativeName: UNICODE_STRING,
    pub ContainingDirectory: HANDLE,
    pub CurDirRef: *mut RTLP_CURDIR_REF,
}

impl Default for RTL_RELATIVE_NAME_U {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_RELATIVE_NAME_U {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RTL_RELATIVE_NAME_U {{ CurDirRef: {:?} }}",
            self.CurDirRef
        )
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum RTL_PATH_TYPE {
    RtlPathTypeUnknown = 0,
    RtlPathTypeUncAbsolute = 1,
    RtlPathTypeDriveAbsolute = 2,
    RtlPathTypeDriveRelative = 3,
    RtlPathTypeRooted = 4,
    RtlPathTypeRelative = 5,
    RtlPathTypeLocalDevice = 6,
    RtlPathTypeRootLocalDevice = 7,
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlDetermineDosPathNameType_U(DosFileName: PCWSTR) -> RTL_PATH_TYPE;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlIsDosDeviceName_U(DosFileName: PCWSTR) -> u32;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlGetFullPathName_U(
        FileName: PCWSTR,
        BufferLength: u32,
        Buffer: PWSTR,
        FilePart: *mut PWSTR,
    ) -> u32;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlGetFullPathName_UEx(
        FileName: PCWSTR,
        BufferLength: u32,
        Buffer: PWSTR,
        FilePart: *mut PWSTR,
        BytesRequired: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlGetFullPathName_UstrEx(
        FileName: *mut UNICODE_STRING,
        StaticString: *mut UNICODE_STRING,
        DynamicString: *mut UNICODE_STRING,
        StringUsed: *mut *mut UNICODE_STRING,
        FilePartPrefixCch: *mut usize,
        NameInvalid: *mut bool,
        InputPathType: *mut RTL_PATH_TYPE,
        BytesRequired: *mut usize,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlGetCurrentDirectory_U(BufferLength: u32, Buffer: PWSTR) -> u32;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlSetCurrentDirectory_U(PathName: *mut UNICODE_STRING) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlGetLongestNtPathLength() -> u32;
}

#[repr(C)]
pub struct RTL_BUFFER {
    pub Buffer: *mut u8,
    pub StaticBuffer: *mut u8,
    pub Size: usize,
    pub StaticSize: usize,
}

impl Default for RTL_BUFFER {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_BUFFER {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RTL_BUFFER {{  }}")
    }
}

#[repr(C)]
pub struct RTL_UNICODE_STRING_BUFFER {
    pub String: UNICODE_STRING,
    pub ByteBuffer: RTL_BUFFER,
    pub MinimumStaticBufferForTerminalNul: [u8; 2],
}

impl Default for RTL_UNICODE_STRING_BUFFER {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_UNICODE_STRING_BUFFER {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RTL_UNICODE_STRING_BUFFER {{ ByteBuffer: {:?}, MinimumStaticBufferForTerminalNul: {:?} }}",
            self.ByteBuffer, self.MinimumStaticBufferForTerminalNul
        )
    }
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlNtPathNameToDosPathName(
        Flags: u32,
        Path: *mut RTL_UNICODE_STRING_BUFFER,
        Disposition: *mut u32,
        FilePart: *mut PWSTR,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlDosPathNameToNtPathName_U(
        DosFileName: PCWSTR,
        NtFileName: *mut UNICODE_STRING,
        FilePart: *mut PWSTR,
        RelativeName: *mut RTL_RELATIVE_NAME_U,
    ) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlDosPathNameToNtPathName_U_WithStatus(
        DosFileName: PCWSTR,
        NtFileName: *mut UNICODE_STRING,
        FilePart: *mut PWSTR,
        RelativeName: *mut RTL_RELATIVE_NAME_U,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlDosLongPathNameToNtPathName_U_WithStatus(
        DosFileName: PCWSTR,
        NtFileName: *mut UNICODE_STRING,
        FilePart: *mut PWSTR,
        RelativeName: *mut RTL_RELATIVE_NAME_U,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlDosPathNameToRelativeNtPathName_U(
        DosFileName: PCWSTR,
        NtFileName: *mut UNICODE_STRING,
        FilePart: *mut PWSTR,
        RelativeName: *mut RTL_RELATIVE_NAME_U,
    ) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlDosPathNameToRelativeNtPathName_U_WithStatus(
        DosFileName: PCWSTR,
        NtFileName: *mut UNICODE_STRING,
        FilePart: *mut PWSTR,
        RelativeName: *mut RTL_RELATIVE_NAME_U,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlDosLongPathNameToRelativeNtPathName_U_WithStatus(
        DosFileName: PCWSTR,
        NtFileName: *mut UNICODE_STRING,
        FilePart: *mut PWSTR,
        RelativeName: *mut RTL_RELATIVE_NAME_U,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlReleaseRelativeName(RelativeName: *mut RTL_RELATIVE_NAME_U);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlDosSearchPath_U(
        Path: PCWSTR,
        FileName: PCWSTR,
        Extension: PCWSTR,
        BufferLength: u32,
        Buffer: PWSTR,
        FilePart: *mut PWSTR,
    ) -> u32;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlDosSearchPath_Ustr(
        Flags: u32,
        Path: *mut UNICODE_STRING,
        FileName: *mut UNICODE_STRING,
        DefaultExtension: *mut UNICODE_STRING,
        StaticString: *mut UNICODE_STRING,
        DynamicString: *mut UNICODE_STRING,
        FullFileNameOut: *mut *const UNICODE_STRING,
        FilePartPrefixCch: *mut usize,
        BytesRequired: *mut usize,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlDoesFileExists_U(FileName: PCWSTR) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlDosApplyFileIsolationRedirection_Ustr(
        Flags: u32,
        OriginalName: *mut UNICODE_STRING,
        Extension: *mut UNICODE_STRING,
        StaticString: *mut UNICODE_STRING,
        DynamicString: *mut UNICODE_STRING,
        NewName: *mut *mut UNICODE_STRING,
        NewFlags: *mut u32,
        FileNameSize: *mut usize,
        RequiredLength: *mut usize,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlGetLengthWithoutLastFullDosOrNtPathElement(
        Flags: u32,
        PathString: *mut UNICODE_STRING,
        Length: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlGetLengthWithoutTrailingPathSeperators(
        Flags: u32,
        PathString: *mut UNICODE_STRING,
        Length: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlComputePrivatizedDllName_U(
        DllName: *mut UNICODE_STRING,
        RealName: *mut UNICODE_STRING,
        LocalName: *mut UNICODE_STRING,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlGetSearchPath(SearchPathA: *mut PWSTR) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlSetSearchPathMode(Flags: u32) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlGetExePath(DosPathName: PCWSTR, SearchPathA: *mut PWSTR) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlReleasePath(Path: PWSTR);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlReplaceSystemDirectoryInPath(
        Destination: *mut UNICODE_STRING,
        Machine: u16,
        TargetMachine: u16,
        IncludePathSeperator: bool,
    ) -> u32;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlWow64GetProcessMachines(
        ProcessHandle: HANDLE,
        ProcessMachine: *mut u16,
        NativeMachine: *mut u16,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlGetImageFileMachines(FileName: PCWSTR, FileMachines: *mut u16) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlAreLongPathsEnabled() -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlIsThreadWithinLoaderCallout() -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlDllShutdownInProgress() -> bool;
}

#[repr(C)]
pub struct RTL_HEAP_ENTRY {
    pub Size: usize,
    pub Flags: u16,
    pub AllocatorBackTraceIndex: u16,
    pub u: RTL_HEAP_ENTRY_1,
}

#[repr(C)]
pub struct RTL_HEAP_ENTRY_1 {
    pub s1: UnionField<RTL_HEAP_ENTRY_1_1>,
    pub s2: UnionField<RTL_HEAP_ENTRY_1_2>,
    pub union_field: [u64; 2],
}

#[repr(C)]
pub struct RTL_HEAP_ENTRY_1_1 {
    pub Settable: usize,
    pub Tag: u32,
}

impl Default for RTL_HEAP_ENTRY_1_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_HEAP_ENTRY_1_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RTL_HEAP_ENTRY_1_1 {{  }}")
    }
}

#[repr(C)]
pub struct RTL_HEAP_ENTRY_1_2 {
    pub CommittedSize: usize,
    pub FirstBlock: *mut std::ffi::c_void,
}

impl Default for RTL_HEAP_ENTRY_1_2 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_HEAP_ENTRY_1_2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RTL_HEAP_ENTRY_1_2 {{  }}")
    }
}

impl Default for RTL_HEAP_ENTRY_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_HEAP_ENTRY_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RTL_HEAP_ENTRY_1 {{ union }}")
    }
}

impl Default for RTL_HEAP_ENTRY {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_HEAP_ENTRY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RTL_HEAP_ENTRY {{ u: {:?} }}", self.u)
    }
}

#[repr(C)]
pub struct RTL_HEAP_TAG {
    pub NumberOfAllocations: u32,
    pub NumberOfFrees: u32,
    pub BytesAllocated: usize,
    pub TagIndex: u16,
    pub CreatorBackTraceIndex: u16,
    pub TagName: [u16; 24],
}

impl Default for RTL_HEAP_TAG {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_HEAP_TAG {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RTL_HEAP_TAG {{ TagName: {:?} }}", self.TagName)
    }
}

#[repr(C)]
pub struct RTL_HEAP_INFORMATION_V1 {
    pub BaseAddress: *mut std::ffi::c_void,
    pub Flags: u32,
    pub EntryOverhead: u16,
    pub CreatorBackTraceIndex: u16,
    pub BytesAllocated: usize,
    pub BytesCommitted: usize,
    pub NumberOfTags: u32,
    pub NumberOfEntries: u32,
    pub NumberOfPseudoTags: u32,
    pub PseudoTagGranularity: u32,
    pub Reserved: [u32; 5],
    pub Tags: *mut RTL_HEAP_TAG,
    pub Entries: *mut RTL_HEAP_ENTRY,
}

impl Default for RTL_HEAP_INFORMATION_V1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_HEAP_INFORMATION_V1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RTL_HEAP_INFORMATION_V1 {{ Reserved: {:?}, Tags: {:?}, Entries: {:?} }}",
            self.Reserved, self.Tags, self.Entries
        )
    }
}

#[repr(C)]
pub struct RTL_HEAP_INFORMATION_V2 {
    pub BaseAddress: *mut std::ffi::c_void,
    pub Flags: u32,
    pub EntryOverhead: u16,
    pub CreatorBackTraceIndex: u16,
    pub BytesAllocated: usize,
    pub BytesCommitted: usize,
    pub NumberOfTags: u32,
    pub NumberOfEntries: u32,
    pub NumberOfPseudoTags: u32,
    pub PseudoTagGranularity: u32,
    pub Reserved: [u32; 5],
    pub Tags: *mut RTL_HEAP_TAG,
    pub Entries: *mut RTL_HEAP_ENTRY,
    pub HeapTag: u64,
}

impl Default for RTL_HEAP_INFORMATION_V2 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_HEAP_INFORMATION_V2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RTL_HEAP_INFORMATION_V2 {{ Reserved: {:?}, Tags: {:?}, Entries: {:?} }}",
            self.Reserved, self.Tags, self.Entries
        )
    }
}

#[repr(C)]
pub struct RTL_PROCESS_HEAPS_V1 {
    pub NumberOfHeaps: u32,
    pub Heaps: [RTL_HEAP_INFORMATION_V1; 1],
}

impl Default for RTL_PROCESS_HEAPS_V1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_PROCESS_HEAPS_V1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RTL_PROCESS_HEAPS_V1 {{ Heaps: {:?} }}", self.Heaps)
    }
}

#[repr(C)]
pub struct RTL_PROCESS_HEAPS_V2 {
    pub NumberOfHeaps: u32,
    pub Heaps: [RTL_HEAP_INFORMATION_V2; 1],
}

impl Default for RTL_PROCESS_HEAPS_V2 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_PROCESS_HEAPS_V2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RTL_PROCESS_HEAPS_V2 {{ Heaps: {:?} }}", self.Heaps)
    }
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlSizeHeap(
        HeapHandle: *mut std::ffi::c_void,
        Flags: u32,
        BaseAddress: *mut std::ffi::c_void,
    ) -> usize;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlZeroHeap(HeapHandle: *mut std::ffi::c_void, Flags: u32) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlProtectHeap(HeapHandle: *mut std::ffi::c_void, MakeReadOnly: bool);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlLockHeap(HeapHandle: *mut std::ffi::c_void) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlUnlockHeap(HeapHandle: *mut std::ffi::c_void) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlReAllocateHeap(
        HeapHandle: *mut std::ffi::c_void,
        Flags: u32,
        BaseAddress: *mut std::ffi::c_void,
        Size: usize,
    ) -> *mut std::ffi::c_void;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlGetUserInfoHeap(
        HeapHandle: *mut std::ffi::c_void,
        Flags: u32,
        BaseAddress: *mut std::ffi::c_void,
        UserValue: *mut *mut std::ffi::c_void,
        UserFlags: *mut u32,
    ) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlSetUserValueHeap(
        HeapHandle: *mut std::ffi::c_void,
        Flags: u32,
        BaseAddress: *mut std::ffi::c_void,
        UserValue: *mut std::ffi::c_void,
    ) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlSetUserFlagsHeap(
        HeapHandle: *mut std::ffi::c_void,
        Flags: u32,
        BaseAddress: *mut std::ffi::c_void,
        UserFlagsReset: u32,
        UserFlagsSet: u32,
    ) -> bool;
}

#[repr(C)]
pub struct RTL_HEAP_TAG_INFO {
    pub NumberOfAllocations: u32,
    pub NumberOfFrees: u32,
    pub BytesAllocated: usize,
}

impl Default for RTL_HEAP_TAG_INFO {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_HEAP_TAG_INFO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RTL_HEAP_TAG_INFO {{  }}")
    }
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlCreateTagHeap(
        HeapHandle: *mut std::ffi::c_void,
        Flags: u32,
        TagPrefix: PWSTR,
        TagNames: PWSTR,
    ) -> u32;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlQueryTagHeap(
        HeapHandle: *mut std::ffi::c_void,
        Flags: u32,
        TagIndex: u16,
        ResetCounters: bool,
        TagInfo: *mut RTL_HEAP_TAG_INFO,
    ) -> PWSTR;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlExtendHeap(
        HeapHandle: *mut std::ffi::c_void,
        Flags: u32,
        Base: *mut std::ffi::c_void,
        Size: usize,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlCompactHeap(HeapHandle: *mut std::ffi::c_void, Flags: u32) -> usize;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlValidateHeap(
        HeapHandle: *mut std::ffi::c_void,
        Flags: u32,
        BaseAddress: *mut std::ffi::c_void,
    ) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlValidateProcessHeaps() -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlGetProcessHeaps(NumberOfHeaps: u32, ProcessHeaps: *mut *mut std::ffi::c_void) -> u32;
}

pub type PRTL_ENUM_HEAPS_ROUTINE = Option<
    unsafe extern "system" fn(arg1: *mut std::ffi::c_void, arg2: *mut std::ffi::c_void) -> NTSTATUS,
>;

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlEnumProcessHeaps(
        EnumRoutine: PRTL_ENUM_HEAPS_ROUTINE,
        Parameter: *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[repr(C)]
pub struct RTL_HEAP_USAGE_ENTRY {
    pub Next: *mut RTL_HEAP_USAGE_ENTRY,
    pub Address: *mut std::ffi::c_void,
    pub Size: usize,
    pub AllocatorBackTraceIndex: u16,
    pub TagIndex: u16,
}

impl Default for RTL_HEAP_USAGE_ENTRY {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_HEAP_USAGE_ENTRY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RTL_HEAP_USAGE_ENTRY {{ Next: {:?} }}", self.Next)
    }
}

#[repr(C)]
pub struct RTL_HEAP_USAGE {
    pub Length: u32,
    pub BytesAllocated: usize,
    pub BytesCommitted: usize,
    pub BytesReserved: usize,
    pub BytesReservedMaximum: usize,
    pub Entries: *mut RTL_HEAP_USAGE_ENTRY,
    pub AddedEntries: *mut RTL_HEAP_USAGE_ENTRY,
    pub RemovedEntries: *mut RTL_HEAP_USAGE_ENTRY,
    pub Reserved: [usize; 8],
}

impl Default for RTL_HEAP_USAGE {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_HEAP_USAGE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RTL_HEAP_USAGE {{ Entries: {:?}, AddedEntries: {:?}, RemovedEntries: {:?}, Reserved: {:?} }}",
            self.Entries, self.AddedEntries, self.RemovedEntries, self.Reserved
        )
    }
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlUsageHeap(
        HeapHandle: *mut std::ffi::c_void,
        Flags: u32,
        Usage: *mut RTL_HEAP_USAGE,
    ) -> NTSTATUS;
}

#[repr(C)]
pub struct RTL_HEAP_WALK_ENTRY {
    pub DataAddress: *mut std::ffi::c_void,
    pub DataSize: usize,
    pub OverheadBytes: u8,
    pub SegmentIndex: u8,
    pub Flags: u16,
    pub Anonymous1: RTL_HEAP_WALK_ENTRY_1,
}

#[repr(C)]
pub struct RTL_HEAP_WALK_ENTRY_1 {
    pub Block: UnionField<RTL_HEAP_WALK_ENTRY_1_1>,
    pub Segment: UnionField<RTL_HEAP_WALK_ENTRY_1_2>,
    pub union_field: [u64; 3],
}

#[repr(C)]
pub struct RTL_HEAP_WALK_ENTRY_1_1 {
    pub Settable: usize,
    pub TagIndex: u16,
    pub AllocatorBackTraceIndex: u16,
    pub Reserved: [u32; 2],
}

impl Default for RTL_HEAP_WALK_ENTRY_1_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_HEAP_WALK_ENTRY_1_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RTL_HEAP_WALK_ENTRY_1_1 {{ Reserved: {:?} }}",
            self.Reserved
        )
    }
}

#[repr(C)]
pub struct RTL_HEAP_WALK_ENTRY_1_2 {
    pub CommittedSize: u32,
    pub UnCommittedSize: u32,
    pub FirstEntry: *mut std::ffi::c_void,
    pub LastEntry: *mut std::ffi::c_void,
}

impl Default for RTL_HEAP_WALK_ENTRY_1_2 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_HEAP_WALK_ENTRY_1_2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RTL_HEAP_WALK_ENTRY_1_2 {{  }}")
    }
}

impl Default for RTL_HEAP_WALK_ENTRY_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_HEAP_WALK_ENTRY_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RTL_HEAP_WALK_ENTRY_1 {{ union }}")
    }
}

impl Default for RTL_HEAP_WALK_ENTRY {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_HEAP_WALK_ENTRY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RTL_HEAP_WALK_ENTRY {{ Anonymous1: {:?} }}",
            self.Anonymous1
        )
    }
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlWalkHeap(
        HeapHandle: *mut std::ffi::c_void,
        Entry: *mut RTL_HEAP_WALK_ENTRY,
    ) -> NTSTATUS;
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum HEAP_COMPATIBILITY_MODE {
    HEAP_COMPATIBILITY_STANDARD = 0,
    HEAP_COMPATIBILITY_LAL = 1,
    HEAP_COMPATIBILITY_LFH = 2,
}

#[repr(C)]
pub struct RTLP_TAG_INFO {
    pub Id: GUID,
    pub CurrentAllocatedBytes: usize,
}

impl Default for RTLP_TAG_INFO {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTLP_TAG_INFO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RTLP_TAG_INFO {{  }}")
    }
}

#[repr(C)]
pub struct RTLP_HEAP_TAGGING_INFO {
    pub Version: u16,
    pub Flags: u16,
    pub ProcessHandle: *mut std::ffi::c_void,
    pub EntriesCount: usize,
    pub Entries: [RTLP_TAG_INFO; 1],
}

impl Default for RTLP_HEAP_TAGGING_INFO {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTLP_HEAP_TAGGING_INFO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RTLP_HEAP_TAGGING_INFO {{ Entries: {:?} }}",
            self.Entries
        )
    }
}

#[repr(C)]
pub struct PROCESS_HEAP_INFORMATION {
    pub ReserveSize: usize,
    pub CommitSize: usize,
    pub NumberOfHeaps: u32,
    pub FirstHeapInformationOffset: usize,
}

impl Default for PROCESS_HEAP_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PROCESS_HEAP_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PROCESS_HEAP_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct HEAP_REGION_INFORMATION {
    pub Address: *mut std::ffi::c_void,
    pub ReserveSize: usize,
    pub CommitSize: usize,
    pub FirstRangeInformationOffset: usize,
    pub NextRegionInformationOffset: usize,
}

impl Default for HEAP_REGION_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for HEAP_REGION_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HEAP_REGION_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct HEAP_RANGE_INFORMATION {
    pub Address: *mut std::ffi::c_void,
    pub Size: usize,
    pub Type: u32,
    pub Protection: u32,
    pub FirstBlockInformationOffset: usize,
    pub NextRangeInformationOffset: usize,
}

impl Default for HEAP_RANGE_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for HEAP_RANGE_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HEAP_RANGE_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct HEAP_BLOCK_INFORMATION {
    pub Address: *mut std::ffi::c_void,
    pub Flags: u32,
    pub DataSize: usize,
    pub OverheadSize: usize,
    pub NextBlockInformationOffset: usize,
}

impl Default for HEAP_BLOCK_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for HEAP_BLOCK_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HEAP_BLOCK_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct HEAP_INFORMATION {
    pub Address: *mut std::ffi::c_void,
    pub Mode: u32,
    pub ReserveSize: usize,
    pub CommitSize: usize,
    pub FirstRegionInformationOffset: usize,
    pub NextHeapInformationOffset: usize,
}

impl Default for HEAP_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for HEAP_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HEAP_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct SEGMENT_HEAP_PERFORMANCE_COUNTER_INFORMATION {
    pub SegmentReserveSize: usize,
    pub SegmentCommitSize: usize,
    pub SegmentCount: usize,
    pub AllocatedSize: usize,
    pub LargeAllocReserveSize: usize,
    pub LargeAllocCommitSize: usize,
}

impl Default for SEGMENT_HEAP_PERFORMANCE_COUNTER_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for SEGMENT_HEAP_PERFORMANCE_COUNTER_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SEGMENT_HEAP_PERFORMANCE_COUNTER_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct HEAP_PERFORMANCE_COUNTERS_INFORMATION {
    pub Size: u32,
    pub Version: u32,
    pub HeapIndex: u32,
    pub LastHeapIndex: u32,
    pub BaseAddress: *mut std::ffi::c_void,
    pub ReserveSize: usize,
    pub CommitSize: usize,
    pub SegmentCount: u32,
    pub LargeUCRMemory: usize,
    pub UCRLength: u32,
    pub AllocatedSpace: usize,
    pub FreeSpace: usize,
    pub FreeListLength: u32,
    pub Contention: u32,
    pub VirtualBlocks: u32,
    pub CommitRate: u32,
    pub DecommitRate: u32,
    pub SegmentHeapPerfInformation: SEGMENT_HEAP_PERFORMANCE_COUNTER_INFORMATION,
}

impl Default for HEAP_PERFORMANCE_COUNTERS_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for HEAP_PERFORMANCE_COUNTERS_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "HEAP_PERFORMANCE_COUNTERS_INFORMATION {{ SegmentHeapPerfInformation: {:?} }}",
            self.SegmentHeapPerfInformation
        )
    }
}

#[repr(C)]
pub struct HEAP_INFORMATION_ITEM {
    pub Level: u32,
    pub Size: usize,
    pub Anonymous1: HEAP_INFORMATION_ITEM_1,
}

#[repr(C)]
pub struct HEAP_INFORMATION_ITEM_1 {
    pub ProcessHeapInformation: UnionField<PROCESS_HEAP_INFORMATION>,
    pub HeapInformation: UnionField<HEAP_INFORMATION>,
    pub HeapRegionInformation: UnionField<HEAP_REGION_INFORMATION>,
    pub HeapRangeInformation: UnionField<HEAP_RANGE_INFORMATION>,
    pub HeapBlockInformation: UnionField<HEAP_BLOCK_INFORMATION>,
    pub HeapPerfInformation: UnionField<HEAP_PERFORMANCE_COUNTERS_INFORMATION>,
    pub DynamicStart: UnionField<usize>,
    pub union_field: [u64; 19],
}

impl Default for HEAP_INFORMATION_ITEM_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for HEAP_INFORMATION_ITEM_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HEAP_INFORMATION_ITEM_1 {{ union }}")
    }
}

impl Default for HEAP_INFORMATION_ITEM {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for HEAP_INFORMATION_ITEM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "HEAP_INFORMATION_ITEM {{ Anonymous1: {:?} }}",
            self.Anonymous1
        )
    }
}

pub type PRTL_HEAP_EXTENDED_ENUMERATION_ROUTINE = Option<
    unsafe extern "system" fn(
        arg1: *mut HEAP_INFORMATION_ITEM,
        arg2: *mut std::ffi::c_void,
    ) -> NTSTATUS,
>;

#[repr(C)]
pub struct HEAP_EXTENDED_INFORMATION {
    pub ProcessHandle: HANDLE,
    pub HeapHandle: *mut std::ffi::c_void,
    pub Level: u32,
    pub CallbackRoutine: PRTL_HEAP_EXTENDED_ENUMERATION_ROUTINE,
    pub CallbackContext: *mut std::ffi::c_void,
    pub Anonymous1: HEAP_EXTENDED_INFORMATION_1,
}

#[repr(C)]
pub struct HEAP_EXTENDED_INFORMATION_1 {
    pub ProcessHeapInformation: UnionField<PROCESS_HEAP_INFORMATION>,
    pub HeapInformation: UnionField<HEAP_INFORMATION>,
    pub union_field: [u64; 6],
}

impl Default for HEAP_EXTENDED_INFORMATION_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for HEAP_EXTENDED_INFORMATION_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HEAP_EXTENDED_INFORMATION_1 {{ union }}")
    }
}

impl Default for HEAP_EXTENDED_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for HEAP_EXTENDED_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "HEAP_EXTENDED_INFORMATION {{ CallbackRoutine: {:?}, Anonymous1: {:?} }}",
            self.CallbackRoutine, self.Anonymous1
        )
    }
}

pub type RTL_HEAP_STACK_WRITE_ROUTINE = Option<
    unsafe extern "system" fn(
        Information: *mut std::ffi::c_void,
        Size: u32,
        Context: *mut std::ffi::c_void,
    ) -> NTSTATUS,
>;

#[repr(C)]
pub struct RTLP_HEAP_STACK_TRACE_SERIALIZATION_INIT {
    pub Count: u32,
    pub Total: u32,
    pub Flags: u32,
}

impl Default for RTLP_HEAP_STACK_TRACE_SERIALIZATION_INIT {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTLP_HEAP_STACK_TRACE_SERIALIZATION_INIT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RTLP_HEAP_STACK_TRACE_SERIALIZATION_INIT {{  }}")
    }
}

#[repr(C)]
pub struct RTLP_HEAP_STACK_TRACE_SERIALIZATION_HEADER {
    pub Version: u16,
    pub PointerSize: u16,
    pub Heap: *mut std::ffi::c_void,
    pub TotalCommit: usize,
    pub TotalReserve: usize,
}

impl Default for RTLP_HEAP_STACK_TRACE_SERIALIZATION_HEADER {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTLP_HEAP_STACK_TRACE_SERIALIZATION_HEADER {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RTLP_HEAP_STACK_TRACE_SERIALIZATION_HEADER {{  }}")
    }
}

#[repr(C)]
pub struct RTLP_HEAP_STACK_TRACE_SERIALIZATION_ALLOCATION {
    pub Address: *mut std::ffi::c_void,
    pub Flags: u32,
    pub DataSize: usize,
}

impl Default for RTLP_HEAP_STACK_TRACE_SERIALIZATION_ALLOCATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTLP_HEAP_STACK_TRACE_SERIALIZATION_ALLOCATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RTLP_HEAP_STACK_TRACE_SERIALIZATION_ALLOCATION {{  }}")
    }
}

#[repr(C)]
pub struct RTLP_HEAP_STACK_TRACE_SERIALIZATION_STACKFRAME {
    pub StackFrame: [*mut std::ffi::c_void; 8],
}

impl Default for RTLP_HEAP_STACK_TRACE_SERIALIZATION_STACKFRAME {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTLP_HEAP_STACK_TRACE_SERIALIZATION_STACKFRAME {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RTLP_HEAP_STACK_TRACE_SERIALIZATION_STACKFRAME {{ StackFrame: {:?} }}",
            self.StackFrame
        )
    }
}

#[repr(C)]
pub struct RTL_HEAP_STACK_QUERY {
    pub Version: u32,
    pub ProcessHandle: HANDLE,
    pub WriteRoutine: RTL_HEAP_STACK_WRITE_ROUTINE,
    pub SerializationContext: *mut std::ffi::c_void,
    pub QueryLevel: u8,
    pub Flags: u8,
}

impl Default for RTL_HEAP_STACK_QUERY {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_HEAP_STACK_QUERY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RTL_HEAP_STACK_QUERY {{ WriteRoutine: {:?} }}",
            self.WriteRoutine
        )
    }
}

#[repr(C)]
pub struct RTL_HEAP_STACK_CONTROL {
    pub Version: u16,
    pub Flags: u16,
    pub ProcessHandle: HANDLE,
}

impl Default for RTL_HEAP_STACK_CONTROL {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_HEAP_STACK_CONTROL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RTL_HEAP_STACK_CONTROL {{  }}")
    }
}

pub type PRTL_HEAP_DEBUGGING_INTERCEPTOR_ROUTINE = Option<
    unsafe extern "system" fn(
        HeapHandle: *mut std::ffi::c_void,
        Action: u32,
        StackFramesToCapture: u32,
        StackTrace: *mut *mut std::ffi::c_void,
    ) -> NTSTATUS,
>;

pub type PRTL_HEAP_LEAK_ENUMERATION_ROUTINE = Option<
    unsafe extern "system" fn(
        Reserved: i32,
        HeapHandle: *mut std::ffi::c_void,
        BaseAddress: *mut std::ffi::c_void,
        BlockSize: usize,
        StackTraceDepth: u32,
        StackTrace: *mut *mut std::ffi::c_void,
    ) -> NTSTATUS,
>;

#[repr(C)]
pub struct HEAP_DEBUGGING_INFORMATION {
    pub InterceptorFunction: PRTL_HEAP_DEBUGGING_INTERCEPTOR_ROUTINE,
    pub InterceptorValue: u16,
    pub ExtendedOptions: u32,
    pub StackTraceDepth: u32,
    pub MinTotalBlockSize: usize,
    pub MaxTotalBlockSize: usize,
    pub HeapLeakEnumerationRoutine: PRTL_HEAP_LEAK_ENUMERATION_ROUTINE,
}

impl Default for HEAP_DEBUGGING_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for HEAP_DEBUGGING_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "HEAP_DEBUGGING_INFORMATION {{ InterceptorFunction: {:?}, HeapLeakEnumerationRoutine: {:?} }}",
            self.InterceptorFunction, self.HeapLeakEnumerationRoutine
        )
    }
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlQueryHeapInformation(
        HeapHandle: *mut std::ffi::c_void,
        HeapInformationClass: HEAP_INFORMATION_CLASS,
        HeapInformation: *mut std::ffi::c_void,
        HeapInformationLength: usize,
        ReturnLength: *mut usize,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlSetHeapInformation(
        HeapHandle: *mut std::ffi::c_void,
        HeapInformationClass: HEAP_INFORMATION_CLASS,
        HeapInformation: *mut std::ffi::c_void,
        HeapInformationLength: usize,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlMultipleAllocateHeap(
        HeapHandle: *mut std::ffi::c_void,
        Flags: u32,
        Size: usize,
        Count: u32,
        Array: *mut *mut std::ffi::c_void,
    ) -> u32;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlMultipleFreeHeap(
        HeapHandle: *mut std::ffi::c_void,
        Flags: u32,
        Count: u32,
        Array: *mut *mut std::ffi::c_void,
    ) -> u32;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlDetectHeapLeaks();
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlFlushHeaps();
}

#[repr(C)]
pub struct RTL_MEMORY_ZONE_SEGMENT {
    pub NextSegment: *mut RTL_MEMORY_ZONE_SEGMENT,
    pub Size: usize,
    pub Next: *mut std::ffi::c_void,
    pub Limit: *mut std::ffi::c_void,
}

impl Default for RTL_MEMORY_ZONE_SEGMENT {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_MEMORY_ZONE_SEGMENT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RTL_MEMORY_ZONE_SEGMENT {{ NextSegment: {:?} }}",
            self.NextSegment
        )
    }
}

#[repr(C)]
pub struct RTL_MEMORY_ZONE {
    pub Segment: RTL_MEMORY_ZONE_SEGMENT,
    pub Lock: SRWLOCK,
    pub LockCount: u32,
    pub FirstSegment: *mut RTL_MEMORY_ZONE_SEGMENT,
}

impl Default for RTL_MEMORY_ZONE {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_MEMORY_ZONE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RTL_MEMORY_ZONE {{ Segment: {:?}, FirstSegment: {:?} }}",
            self.Segment, self.FirstSegment
        )
    }
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlCreateMemoryZone(
        MemoryZone: *mut *mut std::ffi::c_void,
        InitialSize: usize,
        Flags: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlDestroyMemoryZone(MemoryZone: *mut std::ffi::c_void) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlAllocateMemoryZone(
        MemoryZone: *mut std::ffi::c_void,
        BlockSize: usize,
        Block: *mut *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlResetMemoryZone(MemoryZone: *mut std::ffi::c_void) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlLockMemoryZone(MemoryZone: *mut std::ffi::c_void) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlUnlockMemoryZone(MemoryZone: *mut std::ffi::c_void) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlCreateMemoryBlockLookaside(
        MemoryBlockLookaside: *mut *mut std::ffi::c_void,
        Flags: u32,
        InitialSize: u32,
        MinimumBlockSize: u32,
        MaximumBlockSize: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlDestroyMemoryBlockLookaside(MemoryBlockLookaside: *mut std::ffi::c_void) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlAllocateMemoryBlockLookaside(
        MemoryBlockLookaside: *mut std::ffi::c_void,
        BlockSize: u32,
        Block: *mut *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlFreeMemoryBlockLookaside(
        MemoryBlockLookaside: *mut std::ffi::c_void,
        Block: *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlExtendMemoryBlockLookaside(
        MemoryBlockLookaside: *mut std::ffi::c_void,
        Increment: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlResetMemoryBlockLookaside(MemoryBlockLookaside: *mut std::ffi::c_void) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlLockMemoryBlockLookaside(MemoryBlockLookaside: *mut std::ffi::c_void) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlUnlockMemoryBlockLookaside(MemoryBlockLookaside: *mut std::ffi::c_void) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlGetCurrentTransaction() -> HANDLE;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlSetCurrentTransaction(TransactionHandle: HANDLE) -> u32;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlCopyLuidAndAttributesArray(
        Count: u32,
        Src: *mut LUID_AND_ATTRIBUTES,
        Dest: *mut LUID_AND_ATTRIBUTES,
    );
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlUshortByteSwap(Source: u16);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlUlongByteSwap(Source: u32);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlUlonglongByteSwap(Source: u64);
}

#[repr(C)]
pub struct RTL_PROCESS_VERIFIER_OPTIONS {
    pub SizeStruct: u32,
    pub Option: u32,
    pub OptionData: [u8; 1],
}

impl Default for RTL_PROCESS_VERIFIER_OPTIONS {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_PROCESS_VERIFIER_OPTIONS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RTL_PROCESS_VERIFIER_OPTIONS {{ OptionData: {:?} }}",
            self.OptionData
        )
    }
}

#[repr(C)]
pub struct RTL_DEBUG_INFORMATION {
    pub SectionHandleClient: HANDLE,
    pub ViewBaseClient: *mut std::ffi::c_void,
    pub ViewBaseTarget: *mut std::ffi::c_void,
    pub ViewBaseDelta: usize,
    pub EventPairClient: HANDLE,
    pub EventPairTarget: HANDLE,
    pub TargetProcessId: HANDLE,
    pub TargetThreadHandle: HANDLE,
    pub Flags: u32,
    pub OffsetFree: usize,
    pub CommitSize: usize,
    pub ViewSize: usize,
    pub Anonymous1: RTL_DEBUG_INFORMATION_1,
    pub BackTraces: *mut RTL_PROCESS_BACKTRACES,
    pub Heaps: *mut std::ffi::c_void,
    pub Locks: *mut RTL_PROCESS_LOCKS,
    pub SpecificHeap: *mut std::ffi::c_void,
    pub TargetProcessHandle: HANDLE,
    pub VerifierOptions: *mut RTL_PROCESS_VERIFIER_OPTIONS,
    pub ProcessHeap: *mut std::ffi::c_void,
    pub CriticalSectionHandle: HANDLE,
    pub CriticalSectionOwnerThread: HANDLE,
    pub Reserved: [*mut std::ffi::c_void; 4],
}

#[repr(C)]
pub struct RTL_DEBUG_INFORMATION_1 {
    pub Modules: UnionField<*mut RTL_PROCESS_MODULES>,
    pub ModulesEx: UnionField<*mut RTL_PROCESS_MODULE_INFORMATION_EX>,
    pub union_field: u64,
}

impl Default for RTL_DEBUG_INFORMATION_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_DEBUG_INFORMATION_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RTL_DEBUG_INFORMATION_1 {{ union }}")
    }
}

impl Default for RTL_DEBUG_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_DEBUG_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RTL_DEBUG_INFORMATION {{ Anonymous1: {:?}, VerifierOptions: {:?}, Reserved: {:?} }}",
            self.Anonymous1, self.VerifierOptions, self.Reserved
        )
    }
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlCreateQueryDebugBuffer(
        MaximumCommit: u32,
        UseEventPair: bool,
    ) -> *mut RTL_DEBUG_INFORMATION;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlDestroyQueryDebugBuffer(Buffer: *mut RTL_DEBUG_INFORMATION) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlCommitDebugInfo(
        Buffer: *mut RTL_DEBUG_INFORMATION,
        Size: usize,
    ) -> *mut std::ffi::c_void;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlDeCommitDebugInfo(
        Buffer: *mut RTL_DEBUG_INFORMATION,
        p: *mut std::ffi::c_void,
        Size: usize,
    );
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlQueryProcessDebugInformation(
        UniqueProcessId: HANDLE,
        Flags: u32,
        Buffer: *mut RTL_DEBUG_INFORMATION,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlSetProcessDebugInformation(
        UniqueProcessId: HANDLE,
        Flags: u32,
        Buffer: *mut RTL_DEBUG_INFORMATION,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlFindMessage(
        DllHandle: *mut std::ffi::c_void,
        MessageTableId: u32,
        MessageLanguageId: u32,
        MessageId: u32,
        MessageEntry: *mut *mut MESSAGE_RESOURCE_ENTRY,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlFormatMessage(
        MessageFormat: PWSTR,
        MaximumWidth: u32,
        IgnoreInserts: bool,
        ArgumentsAreAnsi: bool,
        ArgumentsAreAnArray: bool,
        Arguments: *mut *mut std::ffi::c_void,
        Buffer: PWSTR,
        Length: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}

#[repr(C)]
pub struct PARSE_MESSAGE_CONTEXT {
    pub fFlags: u32,
    pub cwSavColumn: u32,
    pub iwSrc: usize,
    pub iwDst: usize,
    pub iwDstSpace: usize,
    pub lpvArgStart: *mut std::ffi::c_void,
}

impl Default for PARSE_MESSAGE_CONTEXT {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PARSE_MESSAGE_CONTEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "PARSE_MESSAGE_CONTEXT {{  }}")
    }
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlFormatMessageEx(
        MessageFormat: PWSTR,
        MaximumWidth: u32,
        IgnoreInserts: bool,
        ArgumentsAreAnsi: bool,
        ArgumentsAreAnArray: bool,
        Arguments: *mut *mut std::ffi::c_void,
        Buffer: PWSTR,
        Length: u32,
        ReturnLength: *mut u32,
        ParseContext: *mut PARSE_MESSAGE_CONTEXT,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlGetFileMUIPath(
        Flags: u32,
        FilePath: PCWSTR,
        Language: PWSTR,
        LanguageLength: *mut u32,
        FileMUIPath: PWSTR,
        FileMUIPathLength: *mut u32,
        Enumerator: *mut u64,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlLoadString(
        DllHandle: *mut std::ffi::c_void,
        StringId: u32,
        StringLanguage: PCWSTR,
        Flags: u32,
        ReturnString: *mut PCWSTR,
        ReturnStringLen: *mut u16,
        ReturnLanguageName: PWSTR,
        ReturnLanguageLen: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlGetLastNtStatus() -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlGetLastWin32Error() -> i32;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlSetLastWin32ErrorAndNtStatusFromNtStatus(Status: NTSTATUS);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlSetLastWin32Error(Win32Error: i32);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlRestoreLastWin32Error(Win32Error: i32);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlGetThreadErrorMode() -> u32;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlSetThreadErrorMode(NewMode: u32, OldMode: *mut u32) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlReportException(
        ExceptionRecord: *mut EXCEPTION_RECORD,
        ContextRecord: *mut CONTEXT,
        Flags: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlReportExceptionEx(
        ExceptionRecord: *mut EXCEPTION_RECORD,
        ContextRecord: *mut CONTEXT,
        Flags: u32,
        Timeout: *mut i64,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlWerpReportException(
        ProcessId: u32,
        CrashReportSharedMem: HANDLE,
        Flags: u32,
        CrashVerticalProcessHandle: *mut HANDLE,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlReportSilentProcessExit(ProcessHandle: HANDLE, ExitStatus: NTSTATUS) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlComputeImportTableHash(
        FileHandle: HANDLE,
        Hash: *mut i8,
        ImportTableHashRevision: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlIntegerToChar(Value: u32, Base: u32, OutputLength: i32, String: PSTR) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlLargeIntegerToChar(
        Value: *mut i64,
        Base: u32,
        OutputLength: i32,
        String: PSTR,
    ) -> NTSTATUS;
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct in_addr {
    pub s_addr: u32,
}

pub type IN_ADDR = in_addr;

pub type PIN_ADDR = *mut in_addr;

#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Copy, Clone)]
pub struct in6_addr {
    pub s6_addr: [u8; 16],
}

pub type IN6_ADDR = in6_addr;

pub type PCIN_ADDR = *const IN_ADDR;

pub type PCIN6_ADDR = *const IN6_ADDR;

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlCutoverTimeToSystemTime(
        CutoverTime: *mut TIME_FIELDS,
        SystemTime: *mut i64,
        CurrentSystemTime: *mut i64,
        ThisYear: bool,
    ) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlSystemTimeToLocalTime(SystemTime: *mut i64, LocalTime: *mut i64) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlTimeToElapsedTimeFields(Time: *mut i64, TimeFields: *mut TIME_FIELDS);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlGetSystemTimePrecise() -> i64;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlGetSystemTimeAndBias(
        TimeZoneBias: KSYSTEM_TIME,
        TimeZoneBiasEffectiveStart: *mut i64,
        TimeZoneBiasEffectiveEnd: *mut i64,
    ) -> KSYSTEM_TIME;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlGetInterruptTimePrecise(PerformanceCounter: *mut i64) -> i64;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlQueryUnbiasedInterruptTime(InterruptTime: *mut i64) -> bool;
}

#[repr(C)]
pub struct RTL_TIME_ZONE_INFORMATION {
    pub Bias: i32,
    pub StandardName: [u16; 32],
    pub StandardStart: TIME_FIELDS,
    pub StandardBias: i32,
    pub DaylightName: [u16; 32],
    pub DaylightStart: TIME_FIELDS,
    pub DaylightBias: i32,
}

impl Default for RTL_TIME_ZONE_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_TIME_ZONE_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RTL_TIME_ZONE_INFORMATION {{ StandardName: {:?}, StandardStart: {:?}, DaylightName: {:?}, DaylightStart: {:?} }}",
            self.StandardName, self.StandardStart, self.DaylightName, self.DaylightStart
        )
    }
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlQueryTimeZoneInformation(
        TimeZoneInformation: *mut RTL_TIME_ZONE_INFORMATION,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlSetTimeZoneInformation(
        TimeZoneInformation: *mut RTL_TIME_ZONE_INFORMATION,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlInterlockedClearBitRun(
        BitMapHeader: *mut RTL_BITMAP,
        StartingIndex: u32,
        NumberToClear: u32,
    );
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlInterlockedSetBitRun(
        BitMapHeader: *mut RTL_BITMAP,
        StartingIndex: u32,
        NumberToSet: u32,
    );
}

#[repr(C)]
pub struct RTL_BITMAP_EX {
    pub SizeOfBitMap: u64,
    pub Buffer: *mut u64,
}

impl Default for RTL_BITMAP_EX {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_BITMAP_EX {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RTL_BITMAP_EX {{  }}")
    }
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlInitializeBitMapEx(
        BitMapHeader: *mut RTL_BITMAP_EX,
        BitMapBuffer: *mut u64,
        SizeOfBitMap: u64,
    );
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlTestBitEx(BitMapHeader: *mut RTL_BITMAP_EX, BitNumber: u64) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlClearAllBitsEx(BitMapHeader: *mut RTL_BITMAP_EX);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlClearBitEx(BitMapHeader: *mut RTL_BITMAP_EX, BitNumber: u64);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlSetBitEx(BitMapHeader: *mut RTL_BITMAP_EX, BitNumber: u64);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlFindSetBitsEx(
        BitMapHeader: *mut RTL_BITMAP_EX,
        NumberToFind: u64,
        HintIndex: u64,
    ) -> u64;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlFindSetBitsAndClearEx(
        BitMapHeader: *mut RTL_BITMAP_EX,
        NumberToFind: u64,
        HintIndex: u64,
    ) -> u64;
}

#[repr(C)]
pub struct RTL_HANDLE_TABLE_ENTRY {
    pub Anonymous1: RTL_HANDLE_TABLE_ENTRY_1,
}

#[repr(C)]
pub struct RTL_HANDLE_TABLE_ENTRY_1 {
    pub Flags: UnionField<u32>,
    pub NextFree: UnionField<*mut RTL_HANDLE_TABLE_ENTRY>,
    pub union_field: u64,
}

impl Default for RTL_HANDLE_TABLE_ENTRY_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_HANDLE_TABLE_ENTRY_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RTL_HANDLE_TABLE_ENTRY_1 {{ union }}")
    }
}

impl Default for RTL_HANDLE_TABLE_ENTRY {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_HANDLE_TABLE_ENTRY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RTL_HANDLE_TABLE_ENTRY {{ Anonymous1: {:?} }}",
            self.Anonymous1
        )
    }
}

#[repr(C)]
pub struct RTL_HANDLE_TABLE {
    pub MaximumNumberOfHandles: u32,
    pub SizeOfHandleTableEntry: u32,
    pub Reserved: [u32; 2],
    pub FreeHandles: *mut RTL_HANDLE_TABLE_ENTRY,
    pub CommittedHandles: *mut RTL_HANDLE_TABLE_ENTRY,
    pub UnCommittedHandles: *mut RTL_HANDLE_TABLE_ENTRY,
    pub MaxReservedHandles: *mut RTL_HANDLE_TABLE_ENTRY,
}

impl Default for RTL_HANDLE_TABLE {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_HANDLE_TABLE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RTL_HANDLE_TABLE {{ Reserved: {:?}, FreeHandles: {:?}, CommittedHandles: {:?}, UnCommittedHandles: {:?}, MaxReservedHandles: {:?} }}",
            self.Reserved,
            self.FreeHandles,
            self.CommittedHandles,
            self.UnCommittedHandles,
            self.MaxReservedHandles
        )
    }
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlInitializeHandleTable(
        MaximumNumberOfHandles: u32,
        SizeOfHandleTableEntry: u32,
        HandleTable: *mut RTL_HANDLE_TABLE,
    );
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlDestroyHandleTable(HandleTable: *mut RTL_HANDLE_TABLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlAllocateHandle(
        HandleTable: *mut RTL_HANDLE_TABLE,
        HandleIndex: *mut u32,
    ) -> *mut RTL_HANDLE_TABLE_ENTRY;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlFreeHandle(
        HandleTable: *mut RTL_HANDLE_TABLE,
        Handle: *mut RTL_HANDLE_TABLE_ENTRY,
    ) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlIsValidHandle(
        HandleTable: *mut RTL_HANDLE_TABLE,
        Handle: *mut RTL_HANDLE_TABLE_ENTRY,
    ) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlIsValidIndexHandle(
        HandleTable: *mut RTL_HANDLE_TABLE,
        HandleIndex: u32,
        Handle: *mut *mut RTL_HANDLE_TABLE_ENTRY,
    ) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlCreateAtomTable(
        NumberOfBuckets: u32,
        AtomTableHandle: *mut *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlDestroyAtomTable(AtomTableHandle: *mut std::ffi::c_void) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlEmptyAtomTable(
        AtomTableHandle: *mut std::ffi::c_void,
        IncludePinnedAtoms: bool,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlAddAtomToAtomTable(
        AtomTableHandle: *mut std::ffi::c_void,
        AtomName: PWSTR,
        Atom: *mut u16,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlLookupAtomInAtomTable(
        AtomTableHandle: *mut std::ffi::c_void,
        AtomName: PWSTR,
        Atom: *mut u16,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlDeleteAtomFromAtomTable(
        AtomTableHandle: *mut std::ffi::c_void,
        Atom: u16,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlPinAtomInAtomTable(AtomTableHandle: *mut std::ffi::c_void, Atom: u16) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlQueryAtomInAtomTable(
        AtomTableHandle: *mut std::ffi::c_void,
        Atom: u16,
        AtomUsage: *mut u32,
        AtomFlags: *mut u32,
        AtomName: PWSTR,
        AtomNameLength: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlGetIntegerAtom(AtomName: PWSTR, IntegerAtom: *mut u16) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlCopySidAndAttributesArray(
        Count: u32,
        Src: *mut SID_AND_ATTRIBUTES,
        SidAreaSize: u32,
        Dest: *mut SID_AND_ATTRIBUTES,
        SidArea: PSID,
        RemainingSidArea: *mut PSID,
        RemainingSidAreaSize: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlSidDominates(Sid1: PSID, Sid2: PSID, Dominates: *mut bool) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlSidDominatesForTrust(
        Sid1: PSID,
        Sid2: PSID,
        DominatesTrust: *mut bool,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlSidEqualLevel(Sid1: PSID, Sid2: PSID, EqualLevel: *mut bool) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlSidIsHigherLevel(Sid1: PSID, Sid2: PSID, HigherLevel: *mut bool) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlLengthSidAsUnicodeString(Sid: PSID, StringLength: *mut u32) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlSidHashInitialize(
        SidAttr: *mut SID_AND_ATTRIBUTES,
        SidCount: u32,
        SidAttrHash: *mut SID_AND_ATTRIBUTES_HASH,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlSidHashLookup(
        SidAttrHash: *mut SID_AND_ATTRIBUTES_HASH,
        Sid: PSID,
    ) -> *mut SID_AND_ATTRIBUTES;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlIsElevatedRid(SidAttr: *mut SID_AND_ATTRIBUTES) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlDeriveCapabilitySidsFromName(
        UnicodeString: *mut UNICODE_STRING,
        CapabilityGroupSid: PSID,
        CapabilitySid: PSID,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlGetControlSecurityDescriptor(
        SecurityDescriptor: *mut SECURITY_DESCRIPTOR,
        Control: *mut SECURITY_DESCRIPTOR_CONTROL,
        Revision: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlSetControlSecurityDescriptor(
        SecurityDescriptor: *mut SECURITY_DESCRIPTOR,
        ControlBitsOfInterest: SECURITY_DESCRIPTOR_CONTROL,
        ControlBitsToSet: SECURITY_DESCRIPTOR_CONTROL,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlSetAttributesSecurityDescriptor(
        SecurityDescriptor: *mut SECURITY_DESCRIPTOR,
        Control: SECURITY_DESCRIPTOR_CONTROL,
        Revision: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlGetSecurityDescriptorRMControl(
        SecurityDescriptor: *mut SECURITY_DESCRIPTOR,
        RMControl: *mut u8,
    ) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlSetSecurityDescriptorRMControl(
        SecurityDescriptor: *mut SECURITY_DESCRIPTOR,
        RMControl: *mut u8,
    );
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlMakeSelfRelativeSD(
        AbsoluteSecurityDescriptor: *mut SECURITY_DESCRIPTOR,
        SelfRelativeSecurityDescriptor: *mut SECURITY_DESCRIPTOR,
        BufferLength: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlSelfRelativeToAbsoluteSD2(
        SelfRelativeSecurityDescriptor: *mut SECURITY_DESCRIPTOR,
        BufferSize: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlValidAcl(Acl: *mut ACL) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlQueryInformationAcl(
        Acl: *mut ACL,
        AclInformation: *mut std::ffi::c_void,
        AclInformationLength: u32,
        AclInformationClass: ACL_INFORMATION_CLASS,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlSetInformationAcl(
        Acl: *mut ACL,
        AclInformation: *mut std::ffi::c_void,
        AclInformationLength: u32,
        AclInformationClass: ACL_INFORMATION_CLASS,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlFirstFreeAce(Acl: *mut ACL, FirstFree: *mut *mut std::ffi::c_void) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlFindAceByType(Acl: *mut ACL, AceType: u8, Index: *mut u32) -> *mut std::ffi::c_void;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlOwnerAcesPresent(pAcl: *mut ACL) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlAddAccessDeniedAce(
        Acl: *mut ACL,
        AceRevision: u32,
        AccessMask: u32,
        Sid: PSID,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlAddAccessDeniedAceEx(
        Acl: *mut ACL,
        AceRevision: u32,
        AceFlags: u32,
        AccessMask: u32,
        Sid: PSID,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlAddAuditAccessAce(
        Acl: *mut ACL,
        AceRevision: u32,
        AccessMask: u32,
        Sid: PSID,
        AuditSuccess: bool,
        AuditFailure: bool,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlAddAuditAccessAceEx(
        Acl: *mut ACL,
        AceRevision: u32,
        AceFlags: u32,
        AccessMask: u32,
        Sid: PSID,
        AuditSuccess: bool,
        AuditFailure: bool,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlAddAccessAllowedObjectAce(
        Acl: *mut ACL,
        AceRevision: u32,
        AceFlags: u32,
        AccessMask: u32,
        ObjectTypeGuid: *mut GUID,
        InheritedObjectTypeGuid: *mut GUID,
        Sid: PSID,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlAddAccessDeniedObjectAce(
        Acl: *mut ACL,
        AceRevision: u32,
        AceFlags: u32,
        AccessMask: u32,
        ObjectTypeGuid: *mut GUID,
        InheritedObjectTypeGuid: *mut GUID,
        Sid: PSID,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlAddAuditAccessObjectAce(
        Acl: *mut ACL,
        AceRevision: u32,
        AceFlags: u32,
        AccessMask: u32,
        ObjectTypeGuid: *mut GUID,
        InheritedObjectTypeGuid: *mut GUID,
        Sid: PSID,
        AuditSuccess: bool,
        AuditFailure: bool,
    ) -> NTSTATUS;
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct COMPOUND_ACCESS_ALLOWED_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub CompoundAceType: u16,
    pub Reserved: u16,
    pub SidStart: u32,
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlAddCompoundAce(
        Acl: *mut ACL,
        AceRevision: u32,
        AceType: u8,
        AccessMask: u32,
        ServerSid: PSID,
        ClientSid: PSID,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlAddMandatoryAce(
        Acl: *mut ACL,
        AceRevision: u32,
        AceFlags: u32,
        Sid: PSID,
        AceType: u8,
        AccessMask: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlAddResourceAttributeAce(
        Acl: *mut ACL,
        AceRevision: u32,
        AceFlags: u32,
        AccessMask: u32,
        Sid: PSID,
        AttributeInfo: *mut CLAIM_SECURITY_ATTRIBUTES_INFORMATION,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlAddScopedPolicyIDAce(
        Acl: *mut ACL,
        AceRevision: u32,
        AceFlags: u32,
        AccessMask: u32,
        Sid: PSID,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlDefaultNpAcl(Acl: *mut *mut ACL) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlNewSecurityObject(
        ParentDescriptor: *mut SECURITY_DESCRIPTOR,
        CreatorDescriptor: *mut SECURITY_DESCRIPTOR,
        NewDescriptor: *mut *mut SECURITY_DESCRIPTOR,
        IsDirectoryObject: bool,
        Token: HANDLE,
        GenericMapping: *mut GENERIC_MAPPING,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlNewSecurityObjectEx(
        ParentDescriptor: *mut SECURITY_DESCRIPTOR,
        CreatorDescriptor: *mut SECURITY_DESCRIPTOR,
        NewDescriptor: *mut *mut SECURITY_DESCRIPTOR,
        ObjectType: *mut GUID,
        IsDirectoryObject: bool,
        AutoInheritFlags: u32,
        Token: HANDLE,
        GenericMapping: *mut GENERIC_MAPPING,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlNewSecurityObjectWithMultipleInheritance(
        ParentDescriptor: *mut SECURITY_DESCRIPTOR,
        CreatorDescriptor: *mut SECURITY_DESCRIPTOR,
        NewDescriptor: *mut *mut SECURITY_DESCRIPTOR,
        ObjectType: *mut *mut GUID,
        GuidCount: u32,
        IsDirectoryObject: bool,
        AutoInheritFlags: u32,
        Token: HANDLE,
        GenericMapping: *mut GENERIC_MAPPING,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlDeleteSecurityObject(ObjectDescriptor: *mut *mut SECURITY_DESCRIPTOR) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlQuerySecurityObject(
        ObjectDescriptor: *mut SECURITY_DESCRIPTOR,
        SecurityInformation: u32,
        ResultantDescriptor: *mut SECURITY_DESCRIPTOR,
        DescriptorLength: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlSetSecurityObject(
        SecurityInformation: u32,
        ModificationDescriptor: *mut SECURITY_DESCRIPTOR,
        ObjectsSecurityDescriptor: *mut *mut SECURITY_DESCRIPTOR,
        GenericMapping: *mut GENERIC_MAPPING,
        TokenHandle: HANDLE,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlSetSecurityObjectEx(
        SecurityInformation: u32,
        ModificationDescriptor: *mut SECURITY_DESCRIPTOR,
        ObjectsSecurityDescriptor: *mut *mut SECURITY_DESCRIPTOR,
        AutoInheritFlags: u32,
        GenericMapping: *mut GENERIC_MAPPING,
        TokenHandle: HANDLE,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlConvertToAutoInheritSecurityObject(
        ParentDescriptor: *mut SECURITY_DESCRIPTOR,
        CurrentSecurityDescriptor: *mut SECURITY_DESCRIPTOR,
        NewSecurityDescriptor: *mut *mut SECURITY_DESCRIPTOR,
        ObjectType: *mut GUID,
        IsDirectoryObject: bool,
        GenericMapping: *mut GENERIC_MAPPING,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlNewInstanceSecurityObject(
        ParentDescriptorChanged: bool,
        CreatorDescriptorChanged: bool,
        OldClientTokenModifiedId: *mut LUID,
        NewClientTokenModifiedId: *mut LUID,
        ParentDescriptor: *mut SECURITY_DESCRIPTOR,
        CreatorDescriptor: *mut SECURITY_DESCRIPTOR,
        NewDescriptor: *mut *mut SECURITY_DESCRIPTOR,
        IsDirectoryObject: bool,
        TokenHandle: HANDLE,
        GenericMapping: *mut GENERIC_MAPPING,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlCopySecurityDescriptor(
        InputSecurityDescriptor: *mut SECURITY_DESCRIPTOR,
        OutputSecurityDescriptor: *mut *mut SECURITY_DESCRIPTOR,
    ) -> NTSTATUS;
}

#[repr(C)]
pub struct RTL_ACE_DATA {
    pub AceType: u8,
    pub InheritFlags: u8,
    pub AceFlags: u8,
    pub AccessMask: u32,
    pub Sid: *mut PSID,
}

impl Default for RTL_ACE_DATA {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_ACE_DATA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RTL_ACE_DATA {{ Sid: {:?} }}", self.Sid)
    }
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlCreateUserSecurityObject(
        AceData: *mut RTL_ACE_DATA,
        AceCount: u32,
        OwnerSid: PSID,
        GroupSid: PSID,
        IsDirectoryObject: bool,
        GenericMapping: *mut GENERIC_MAPPING,
        NewSecurityDescriptor: *mut *mut SECURITY_DESCRIPTOR,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlCreateAndSetSD(
        AceData: *mut RTL_ACE_DATA,
        AceCount: u32,
        OwnerSid: PSID,
        GroupSid: PSID,
        NewSecurityDescriptor: *mut *mut SECURITY_DESCRIPTOR,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlRunEncodeUnicodeString(Seed: *mut u8, String: *mut UNICODE_STRING);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlRunDecodeUnicodeString(Seed: u8, String: *mut UNICODE_STRING);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlImpersonateSelf(ImpersonationLevel: SECURITY_IMPERSONATION_LEVEL) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlImpersonateSelfEx(
        ImpersonationLevel: SECURITY_IMPERSONATION_LEVEL,
        AdditionalAccess: u32,
        ThreadToken: *mut HANDLE,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlAdjustPrivilege(
        Privilege: u32,
        Enable: bool,
        Client: bool,
        WasEnabled: *mut bool,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlAcquirePrivilege(
        Privilege: *mut u32,
        NumPriv: u32,
        Flags: u32,
        ReturnedState: *mut *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlReleasePrivilege(StatePointer: *mut std::ffi::c_void);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlRemovePrivileges(
        TokenHandle: HANDLE,
        PrivilegesToKeep: *mut u32,
        PrivilegeCount: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlCreateBoundaryDescriptor(
        Name: *mut UNICODE_STRING,
        Flags: u32,
    ) -> *mut OBJECT_BOUNDARY_DESCRIPTOR;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlDeleteBoundaryDescriptor(BoundaryDescriptor: *mut OBJECT_BOUNDARY_DESCRIPTOR);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlAddSIDToBoundaryDescriptor(
        BoundaryDescriptor: *mut *mut OBJECT_BOUNDARY_DESCRIPTOR,
        RequiredSid: PSID,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlAddIntegrityLabelToBoundaryDescriptor(
        BoundaryDescriptor: *mut *mut OBJECT_BOUNDARY_DESCRIPTOR,
        IntegrityLabel: PSID,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlGetNtVersionNumbers(
        NtMajorVersion: *mut u32,
        NtMinorVersion: *mut u32,
        NtBuildNumber: *mut u32,
    );
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlGetNtGlobalFlags() -> u32;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlRegisterWait(
        WaitHandle: *mut HANDLE,
        Handle: HANDLE,
        Function: WAITORTIMERCALLBACKFUNC,
        Context: *mut std::ffi::c_void,
        Milliseconds: u32,
        Flags: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlDeregisterWait(WaitHandle: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlDeregisterWaitEx(WaitHandle: HANDLE, CompletionEvent: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlQueueWorkItem(
        Function: WORKERCALLBACKFUNC,
        Context: *mut std::ffi::c_void,
        Flags: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlSetIoCompletionCallback(
        FileHandle: HANDLE,
        CompletionProc: APC_CALLBACK_FUNCTION,
        Flags: u32,
    ) -> NTSTATUS;
}

pub type PRTL_START_POOL_THREAD = Option<
    unsafe extern "system" fn(
        arg1: LPTHREAD_START_ROUTINE,
        arg2: *mut std::ffi::c_void,
        arg3: *mut HANDLE,
    ) -> NTSTATUS,
>;

pub type PRTL_EXIT_POOL_THREAD =
    Option<unsafe extern "system" fn(arg1: NTSTATUS) -> NTSTATUS>;

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlSetThreadPoolStartFunc(
        StartPoolThread: PRTL_START_POOL_THREAD,
        ExitPoolThread: PRTL_EXIT_POOL_THREAD,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlUserThreadStart(Function: LPTHREAD_START_ROUTINE, Parameter: *mut std::ffi::c_void);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrInitializeThunk(ContextRecord: *mut CONTEXT, Parameter: *mut std::ffi::c_void);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlDelayExecution(Alertable: bool, DelayInterval: *mut i64) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlCreateTimerQueue(TimerQueueHandle: *mut HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlCreateTimer(
        TimerQueueHandle: HANDLE,
        Handle: *mut HANDLE,
        Function: WAITORTIMERCALLBACKFUNC,
        Context: *mut std::ffi::c_void,
        DueTime: u32,
        Period: u32,
        Flags: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlUpdateTimer(
        TimerQueueHandle: HANDLE,
        TimerHandle: HANDLE,
        DueTime: u32,
        Period: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlDeleteTimer(
        TimerQueueHandle: HANDLE,
        TimerToCancel: HANDLE,
        Event: HANDLE,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlDeleteTimerQueue(TimerQueueHandle: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlDeleteTimerQueueEx(TimerQueueHandle: HANDLE, Event: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlFormatCurrentUserKeyPath(CurrentUserKeyPath: *mut UNICODE_STRING) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlOpenCurrentUser(DesiredAccess: u32, CurrentUserKey: *mut HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlQueryRegistryValuesEx(
        RelativeTo: u32,
        Path: PCWSTR,
        QueryTable: *mut RTL_QUERY_REGISTRY_TABLE,
        Context: *mut std::ffi::c_void,
        Environment: *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlEnableThreadProfiling(
        ThreadHandle: HANDLE,
        Flags: u32,
        HardwareCounters: u64,
        PerformanceDataHandle: *mut *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlDisableThreadProfiling(PerformanceDataHandle: *mut std::ffi::c_void) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlQueryThreadProfiling(ThreadHandle: HANDLE, Enabled: *mut bool) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlReadThreadProfilingData(
        PerformanceDataHandle: HANDLE,
        Flags: u32,
        PerformanceData: *mut PERFORMANCE_DATA,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlGetNativeSystemInformation(
        SystemInformationClass: u32,
        NativeSystemInformation: *mut std::ffi::c_void,
        InformationLength: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlQueueApcWow64Thread(
        ThreadHandle: HANDLE,
        ApcRoutine: PPS_APC_ROUTINE,
        ApcArgument1: *mut std::ffi::c_void,
        ApcArgument2: *mut std::ffi::c_void,
        ApcArgument3: *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlWow64EnableFsRedirection(Wow64FsEnableRedirection: bool) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlWow64EnableFsRedirectionEx(
        Wow64FsEnableRedirection: *mut std::ffi::c_void,
        OldFsRedirectionLevel: *mut *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlComputeCrc32(PartialCrc: u32, Buffer: *mut std::ffi::c_void, Length: u32) -> u32;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlEncodePointer(Ptr: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlDecodePointer(Ptr: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlEncodeSystemPointer(Ptr: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlDecodeSystemPointer(Ptr: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlEncodeRemotePointer(
        ProcessHandle: HANDLE,
        Pointer: *mut std::ffi::c_void,
        EncodedPointer: *mut *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlDecodeRemotePointer(
        ProcessHandle: HANDLE,
        Pointer: *mut std::ffi::c_void,
        DecodedPointer: *mut *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlIsProcessorFeaturePresent(ProcessorFeature: u32) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlGetCurrentProcessorNumber() -> u32;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlGetCurrentProcessorNumberEx(ProcessorNumber: *mut PROCESSOR_NUMBER);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlPushFrame(Frame: *mut TEB_ACTIVE_FRAME);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlPopFrame(Frame: *mut TEB_ACTIVE_FRAME);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlGetFrame() -> *mut TEB_ACTIVE_FRAME;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlGetEnabledExtendedAndSupervisorFeatures(FeatureMask: u64) -> u64;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlLocateSupervisorFeature(
        XStateHeader: *mut XSAVE_AREA_HEADER,
        FeatureId: u32,
        Length: *mut u32,
    ) -> *mut std::ffi::c_void;
}

#[repr(C)]
pub struct RTL_ELEVATION_FLAGS {
    pub Flags: UnionField<u32>,
    pub Anonymous1: UnionField<RTL_ELEVATION_FLAGS_1>,
    pub union_field: u32,
}

#[repr(C)]
#[repr(align(4))]
pub struct RTL_ELEVATION_FLAGS_1 {
    _bitfield_align_1: [u32; 0],
    _bitfield_1: BitfieldUnit<[u8; 4]>,
}

impl Default for RTL_ELEVATION_FLAGS_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_ELEVATION_FLAGS_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RTL_ELEVATION_FLAGS_1 {{ ElevationEnabled : {:?}, VirtualizationEnabled : {:?}, InstallerDetectEnabled : {:?}, ReservedBits : {:?} }}",
            self.ElevationEnabled(),
            self.VirtualizationEnabled(),
            self.InstallerDetectEnabled(),
            self.ReservedBits()
        )
    }
}

impl RTL_ELEVATION_FLAGS_1 {
    #[inline]
    pub fn ElevationEnabled(&self) -> u32 {
        self._bitfield_1.get(0usize, 1u8) as u32
    }

    #[inline]
    pub fn set_ElevationEnabled(&mut self, val: u32) {
        self._bitfield_1.set(0usize, 1u8, val as u64)
    }

    #[inline]
    pub fn VirtualizationEnabled(&self) -> u32 {
        self._bitfield_1.get(1usize, 1u8) as u32
    }

    #[inline]
    pub fn set_VirtualizationEnabled(&mut self, val: u32) {
        self._bitfield_1.set(1usize, 1u8, val as u64)
    }

    #[inline]
    pub fn InstallerDetectEnabled(&self) -> u32 {
        self._bitfield_1.get(2usize, 1u8) as u32
    }

    #[inline]
    pub fn set_InstallerDetectEnabled(&mut self, val: u32) {
        self._bitfield_1.set(2usize, 1u8, val as u64)
    }

    #[inline]
    pub fn ReservedBits(&self) -> u32 {
        self._bitfield_1.get(3usize, 29u8) as u32
    }

    #[inline]
    pub fn set_ReservedBits(&mut self, val: u32) {
        self._bitfield_1.set(3usize, 29u8, val as u64)
    }

    #[inline]
    pub fn new_bitfield_1(
        ElevationEnabled: u32,
        VirtualizationEnabled: u32,
        InstallerDetectEnabled: u32,
        ReservedBits: u32,
    ) -> BitfieldUnit<[u8; 4]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 4]> = Default::default();

        bitfield_unit.set(0usize, 1u8, ElevationEnabled as u64);

        bitfield_unit.set(1usize, 1u8, VirtualizationEnabled as u64);

        bitfield_unit.set(2usize, 1u8, InstallerDetectEnabled as u64);

        bitfield_unit.set(3usize, 29u8, ReservedBits as u64);

        bitfield_unit
    }
}

impl Default for RTL_ELEVATION_FLAGS {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_ELEVATION_FLAGS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RTL_ELEVATION_FLAGS {{ union }}")
    }
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlQueryElevationFlags(Flags: *mut RTL_ELEVATION_FLAGS) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlRegisterThreadWithCsrss() -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlLockCurrentThread() -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlUnlockCurrentThread() -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlLockModuleSection(Address: *mut std::ffi::c_void) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlUnlockModuleSection(Address: *mut std::ffi::c_void) -> NTSTATUS;
}

#[repr(C)]
pub struct RTL_UNLOAD_EVENT_TRACE {
    pub BaseAddress: *mut std::ffi::c_void,
    pub SizeOfImage: usize,
    pub Sequence: u32,
    pub TimeDateStamp: u32,
    pub CheckSum: u32,
    pub ImageName: [u16; 32],
    pub Version: [u32; 2],
}

impl Default for RTL_UNLOAD_EVENT_TRACE {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_UNLOAD_EVENT_TRACE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RTL_UNLOAD_EVENT_TRACE {{ ImageName: {:?}, Version: {:?} }}",
            self.ImageName, self.Version
        )
    }
}

#[repr(C)]
pub struct RTL_UNLOAD_EVENT_TRACE32 {
    pub BaseAddress: u32,
    pub SizeOfImage: u32,
    pub Sequence: u32,
    pub TimeDateStamp: u32,
    pub CheckSum: u32,
    pub ImageName: [u16; 32],
    pub Version: [u32; 2],
}

impl Default for RTL_UNLOAD_EVENT_TRACE32 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_UNLOAD_EVENT_TRACE32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RTL_UNLOAD_EVENT_TRACE32 {{ ImageName: {:?}, Version: {:?} }}",
            self.ImageName, self.Version
        )
    }
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlGetUnloadEventTrace() -> *mut RTL_UNLOAD_EVENT_TRACE;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlGetUnloadEventTraceEx(
        ElementSize: *mut *mut u32,
        ElementCount: *mut *mut u32,
        EventTrace: *mut *mut std::ffi::c_void,
    );
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlQueryPerformanceCounter(PerformanceCounter: *mut i64) -> u32;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlQueryPerformanceFrequency(PerformanceFrequency: *mut i64) -> u32;
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum IMAGE_MITIGATION_POLICY {
    ImageDepPolicy = 0,
    ImageAslrPolicy = 1,
    ImageDynamicCodePolicy = 2,
    ImageStrictHandleCheckPolicy = 3,
    ImageSystemCallDisablePolicy = 4,
    ImageMitigationOptionsMask = 5,
    ImageExtensionPointDisablePolicy = 6,
    ImageControlFlowGuardPolicy = 7,
    ImageSignaturePolicy = 8,
    ImageFontDisablePolicy = 9,
    ImageImageLoadPolicy = 10,
    ImagePayloadRestrictionPolicy = 11,
    ImageChildProcessPolicy = 12,
    ImageSehopPolicy = 13,
    ImageHeapPolicy = 14,
    ImageUserShadowStackPolicy = 15,
    ImageRedirectionTrustPolicy = 16,
    ImageUserPointerAuthPolicy = 17,
    MaxImageMitigationPolicy = 18,
}

#[repr(C)]
pub struct RTL_IMAGE_MITIGATION_POLICY {
    pub Anonymous1: UnionField<RTL_IMAGE_MITIGATION_POLICY_1>,
    pub Anonymous2: UnionField<RTL_IMAGE_MITIGATION_POLICY_2>,
    pub union_field: u64,
}

#[repr(C)]
#[repr(align(8))]
pub struct RTL_IMAGE_MITIGATION_POLICY_1 {
    _bitfield_align_1: [u64; 0],
    _bitfield_1: BitfieldUnit<[u8; 8]>,
}

impl Default for RTL_IMAGE_MITIGATION_POLICY_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_IMAGE_MITIGATION_POLICY_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RTL_IMAGE_MITIGATION_POLICY_1 {{ AuditState : {:?}, AuditFlag : {:?}, EnableAdditionalAuditingOption : {:?}, Reserved : {:?} }}",
            self.AuditState(),
            self.AuditFlag(),
            self.EnableAdditionalAuditingOption(),
            self.Reserved()
        )
    }
}

impl RTL_IMAGE_MITIGATION_POLICY_1 {
    #[inline]
    pub fn AuditState(&self) -> u64 {
        self._bitfield_1.get(0usize, 2u8)
    }

    #[inline]
    pub fn set_AuditState(&mut self, val: u64) {
        self._bitfield_1.set(0usize, 2u8, val)
    }

    #[inline]
    pub fn AuditFlag(&self) -> u64 {
        self._bitfield_1.get(2usize, 1u8)
    }

    #[inline]
    pub fn set_AuditFlag(&mut self, val: u64) {
        self._bitfield_1.set(2usize, 1u8, val)
    }

    #[inline]
    pub fn EnableAdditionalAuditingOption(&self) -> u64 {
        self._bitfield_1.get(3usize, 1u8)
    }

    #[inline]
    pub fn set_EnableAdditionalAuditingOption(&mut self, val: u64) {
        self._bitfield_1.set(3usize, 1u8, val)
    }

    #[inline]
    pub fn Reserved(&self) -> u64 {
        self._bitfield_1.get(4usize, 60u8)
    }

    #[inline]
    pub fn set_Reserved(&mut self, val: u64) {
        self._bitfield_1.set(4usize, 60u8, val)
    }

    #[inline]
    pub fn new_bitfield_1(
        AuditState: u64,
        AuditFlag: u64,
        EnableAdditionalAuditingOption: u64,
        Reserved: u64,
    ) -> BitfieldUnit<[u8; 8]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 8]> = Default::default();

        bitfield_unit.set(0usize, 2u8, AuditState);

        bitfield_unit.set(2usize, 1u8, AuditFlag);

        bitfield_unit.set(3usize, 1u8, EnableAdditionalAuditingOption);

        bitfield_unit.set(4usize, 60u8, Reserved);

        bitfield_unit
    }
}

#[repr(C)]
#[repr(align(8))]
pub struct RTL_IMAGE_MITIGATION_POLICY_2 {
    _bitfield_align_1: [u64; 0],
    _bitfield_1: BitfieldUnit<[u8; 8]>,
}

impl Default for RTL_IMAGE_MITIGATION_POLICY_2 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_IMAGE_MITIGATION_POLICY_2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RTL_IMAGE_MITIGATION_POLICY_2 {{ PolicyState : {:?}, AlwaysInherit : {:?}, EnableAdditionalPolicyOption : {:?}, AuditReserved : {:?} }}",
            self.PolicyState(),
            self.AlwaysInherit(),
            self.EnableAdditionalPolicyOption(),
            self.AuditReserved()
        )
    }
}

impl RTL_IMAGE_MITIGATION_POLICY_2 {
    #[inline]
    pub fn PolicyState(&self) -> u64 {
        self._bitfield_1.get(0usize, 2u8)
    }

    #[inline]
    pub fn set_PolicyState(&mut self, val: u64) {
        self._bitfield_1.set(0usize, 2u8, val)
    }

    #[inline]
    pub fn AlwaysInherit(&self) -> u64 {
        self._bitfield_1.get(2usize, 1u8)
    }

    #[inline]
    pub fn set_AlwaysInherit(&mut self, val: u64) {
        self._bitfield_1.set(2usize, 1u8, val)
    }

    #[inline]
    pub fn EnableAdditionalPolicyOption(&self) -> u64 {
        self._bitfield_1.get(3usize, 1u8)
    }

    #[inline]
    pub fn set_EnableAdditionalPolicyOption(&mut self, val: u64) {
        self._bitfield_1.set(3usize, 1u8, val)
    }

    #[inline]
    pub fn AuditReserved(&self) -> u64 {
        self._bitfield_1.get(4usize, 60u8)
    }

    #[inline]
    pub fn set_AuditReserved(&mut self, val: u64) {
        self._bitfield_1.set(4usize, 60u8, val)
    }

    #[inline]
    pub fn new_bitfield_1(
        PolicyState: u64,
        AlwaysInherit: u64,
        EnableAdditionalPolicyOption: u64,
        AuditReserved: u64,
    ) -> BitfieldUnit<[u8; 8]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 8]> = Default::default();

        bitfield_unit.set(0usize, 2u8, PolicyState);

        bitfield_unit.set(2usize, 1u8, AlwaysInherit);

        bitfield_unit.set(3usize, 1u8, EnableAdditionalPolicyOption);

        bitfield_unit.set(4usize, 60u8, AuditReserved);

        bitfield_unit
    }
}

impl Default for RTL_IMAGE_MITIGATION_POLICY {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_IMAGE_MITIGATION_POLICY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RTL_IMAGE_MITIGATION_POLICY {{ union }}")
    }
}

#[repr(C)]
pub struct RTL_IMAGE_MITIGATION_DEP_POLICY {
    pub Dep: RTL_IMAGE_MITIGATION_POLICY,
}

impl Default for RTL_IMAGE_MITIGATION_DEP_POLICY {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_IMAGE_MITIGATION_DEP_POLICY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RTL_IMAGE_MITIGATION_DEP_POLICY {{ Dep: {:?} }}",
            self.Dep
        )
    }
}

#[repr(C)]
pub struct RTL_IMAGE_MITIGATION_ASLR_POLICY {
    pub ForceRelocateImages: RTL_IMAGE_MITIGATION_POLICY,
    pub BottomUpRandomization: RTL_IMAGE_MITIGATION_POLICY,
    pub HighEntropyRandomization: RTL_IMAGE_MITIGATION_POLICY,
}

impl Default for RTL_IMAGE_MITIGATION_ASLR_POLICY {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_IMAGE_MITIGATION_ASLR_POLICY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RTL_IMAGE_MITIGATION_ASLR_POLICY {{ ForceRelocateImages: {:?}, BottomUpRandomization: {:?}, HighEntropyRandomization: {:?} }}",
            self.ForceRelocateImages, self.BottomUpRandomization, self.HighEntropyRandomization
        )
    }
}

#[repr(C)]
pub struct RTL_IMAGE_MITIGATION_DYNAMIC_CODE_POLICY {
    pub BlockDynamicCode: RTL_IMAGE_MITIGATION_POLICY,
}

impl Default for RTL_IMAGE_MITIGATION_DYNAMIC_CODE_POLICY {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_IMAGE_MITIGATION_DYNAMIC_CODE_POLICY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RTL_IMAGE_MITIGATION_DYNAMIC_CODE_POLICY {{ BlockDynamicCode: {:?} }}",
            self.BlockDynamicCode
        )
    }
}

#[repr(C)]
pub struct RTL_IMAGE_MITIGATION_STRICT_HANDLE_CHECK_POLICY {
    pub StrictHandleChecks: RTL_IMAGE_MITIGATION_POLICY,
}

impl Default for RTL_IMAGE_MITIGATION_STRICT_HANDLE_CHECK_POLICY {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_IMAGE_MITIGATION_STRICT_HANDLE_CHECK_POLICY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RTL_IMAGE_MITIGATION_STRICT_HANDLE_CHECK_POLICY {{ StrictHandleChecks: {:?} }}",
            self.StrictHandleChecks
        )
    }
}

#[repr(C)]
pub struct RTL_IMAGE_MITIGATION_SYSTEM_CALL_DISABLE_POLICY {
    pub BlockWin32kSystemCalls: RTL_IMAGE_MITIGATION_POLICY,
}

impl Default for RTL_IMAGE_MITIGATION_SYSTEM_CALL_DISABLE_POLICY {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_IMAGE_MITIGATION_SYSTEM_CALL_DISABLE_POLICY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RTL_IMAGE_MITIGATION_SYSTEM_CALL_DISABLE_POLICY {{ BlockWin32kSystemCalls: {:?} }}",
            self.BlockWin32kSystemCalls
        )
    }
}

#[repr(C)]
pub struct RTL_IMAGE_MITIGATION_EXTENSION_POINT_DISABLE_POLICY {
    pub DisableExtensionPoints: RTL_IMAGE_MITIGATION_POLICY,
}

impl Default for RTL_IMAGE_MITIGATION_EXTENSION_POINT_DISABLE_POLICY {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_IMAGE_MITIGATION_EXTENSION_POINT_DISABLE_POLICY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RTL_IMAGE_MITIGATION_EXTENSION_POINT_DISABLE_POLICY {{ DisableExtensionPoints: {:?} }}",
            self.DisableExtensionPoints
        )
    }
}

#[repr(C)]
pub struct RTL_IMAGE_MITIGATION_CONTROL_FLOW_GUARD_POLICY {
    pub ControlFlowGuard: RTL_IMAGE_MITIGATION_POLICY,
    pub StrictControlFlowGuard: RTL_IMAGE_MITIGATION_POLICY,
}

impl Default for RTL_IMAGE_MITIGATION_CONTROL_FLOW_GUARD_POLICY {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_IMAGE_MITIGATION_CONTROL_FLOW_GUARD_POLICY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RTL_IMAGE_MITIGATION_CONTROL_FLOW_GUARD_POLICY {{ ControlFlowGuard: {:?}, StrictControlFlowGuard: {:?} }}",
            self.ControlFlowGuard, self.StrictControlFlowGuard
        )
    }
}

#[repr(C)]
pub struct RTL_IMAGE_MITIGATION_BINARY_SIGNATURE_POLICY {
    pub BlockNonMicrosoftSignedBinaries: RTL_IMAGE_MITIGATION_POLICY,
    pub EnforceSigningOnModuleDependencies: RTL_IMAGE_MITIGATION_POLICY,
}

impl Default for RTL_IMAGE_MITIGATION_BINARY_SIGNATURE_POLICY {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_IMAGE_MITIGATION_BINARY_SIGNATURE_POLICY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RTL_IMAGE_MITIGATION_BINARY_SIGNATURE_POLICY {{ BlockNonMicrosoftSignedBinaries: {:?}, EnforceSigningOnModuleDependencies: {:?} }}",
            self.BlockNonMicrosoftSignedBinaries, self.EnforceSigningOnModuleDependencies
        )
    }
}

#[repr(C)]
pub struct RTL_IMAGE_MITIGATION_FONT_DISABLE_POLICY {
    pub DisableNonSystemFonts: RTL_IMAGE_MITIGATION_POLICY,
}

impl Default for RTL_IMAGE_MITIGATION_FONT_DISABLE_POLICY {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_IMAGE_MITIGATION_FONT_DISABLE_POLICY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RTL_IMAGE_MITIGATION_FONT_DISABLE_POLICY {{ DisableNonSystemFonts: {:?} }}",
            self.DisableNonSystemFonts
        )
    }
}

#[repr(C)]
pub struct RTL_IMAGE_MITIGATION_IMAGE_LOAD_POLICY {
    pub BlockRemoteImageLoads: RTL_IMAGE_MITIGATION_POLICY,
    pub BlockLowLabelImageLoads: RTL_IMAGE_MITIGATION_POLICY,
    pub PreferSystem32: RTL_IMAGE_MITIGATION_POLICY,
}

impl Default for RTL_IMAGE_MITIGATION_IMAGE_LOAD_POLICY {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_IMAGE_MITIGATION_IMAGE_LOAD_POLICY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RTL_IMAGE_MITIGATION_IMAGE_LOAD_POLICY {{ BlockRemoteImageLoads: {:?}, BlockLowLabelImageLoads: {:?}, PreferSystem32: {:?} }}",
            self.BlockRemoteImageLoads, self.BlockLowLabelImageLoads, self.PreferSystem32
        )
    }
}

#[repr(C)]
pub struct RTL_IMAGE_MITIGATION_PAYLOAD_RESTRICTION_POLICY {
    pub EnableExportAddressFilter: RTL_IMAGE_MITIGATION_POLICY,
    pub EnableExportAddressFilterPlus: RTL_IMAGE_MITIGATION_POLICY,
    pub EnableImportAddressFilter: RTL_IMAGE_MITIGATION_POLICY,
    pub EnableRopStackPivot: RTL_IMAGE_MITIGATION_POLICY,
    pub EnableRopCallerCheck: RTL_IMAGE_MITIGATION_POLICY,
    pub EnableRopSimExec: RTL_IMAGE_MITIGATION_POLICY,
    pub EafPlusModuleList: [u16; 512],
}

impl Default for RTL_IMAGE_MITIGATION_PAYLOAD_RESTRICTION_POLICY {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_IMAGE_MITIGATION_PAYLOAD_RESTRICTION_POLICY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RTL_IMAGE_MITIGATION_PAYLOAD_RESTRICTION_POLICY {{ EnableExportAddressFilter: {:?}, EnableExportAddressFilterPlus: {:?}, EnableImportAddressFilter: {:?}, EnableRopStackPivot: {:?}, EnableRopCallerCheck: {:?}, EnableRopSimExec: {:?}, EafPlusModuleList: {:?} }}",
            self.EnableExportAddressFilter,
            self.EnableExportAddressFilterPlus,
            self.EnableImportAddressFilter,
            self.EnableRopStackPivot,
            self.EnableRopCallerCheck,
            self.EnableRopSimExec,
            self.EafPlusModuleList
        )
    }
}

#[repr(C)]
pub struct RTL_IMAGE_MITIGATION_CHILD_PROCESS_POLICY {
    pub DisallowChildProcessCreation: RTL_IMAGE_MITIGATION_POLICY,
}

impl Default for RTL_IMAGE_MITIGATION_CHILD_PROCESS_POLICY {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_IMAGE_MITIGATION_CHILD_PROCESS_POLICY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RTL_IMAGE_MITIGATION_CHILD_PROCESS_POLICY {{ DisallowChildProcessCreation: {:?} }}",
            self.DisallowChildProcessCreation
        )
    }
}

#[repr(C)]
pub struct RTL_IMAGE_MITIGATION_SEHOP_POLICY {
    pub Sehop: RTL_IMAGE_MITIGATION_POLICY,
}

impl Default for RTL_IMAGE_MITIGATION_SEHOP_POLICY {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_IMAGE_MITIGATION_SEHOP_POLICY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RTL_IMAGE_MITIGATION_SEHOP_POLICY {{ Sehop: {:?} }}",
            self.Sehop
        )
    }
}

#[repr(C)]
pub struct RTL_IMAGE_MITIGATION_HEAP_POLICY {
    pub TerminateOnHeapErrors: RTL_IMAGE_MITIGATION_POLICY,
}

impl Default for RTL_IMAGE_MITIGATION_HEAP_POLICY {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_IMAGE_MITIGATION_HEAP_POLICY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RTL_IMAGE_MITIGATION_HEAP_POLICY {{ TerminateOnHeapErrors: {:?} }}",
            self.TerminateOnHeapErrors
        )
    }
}

#[repr(C)]
pub struct RTL_IMAGE_MITIGATION_USER_SHADOW_STACK_POLICY {
    pub UserShadowStack: RTL_IMAGE_MITIGATION_POLICY,
    pub SetContextIpValidation: RTL_IMAGE_MITIGATION_POLICY,
    pub BlockNonCetBinaries: RTL_IMAGE_MITIGATION_POLICY,
}

impl Default for RTL_IMAGE_MITIGATION_USER_SHADOW_STACK_POLICY {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_IMAGE_MITIGATION_USER_SHADOW_STACK_POLICY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RTL_IMAGE_MITIGATION_USER_SHADOW_STACK_POLICY {{ UserShadowStack: {:?}, SetContextIpValidation: {:?}, BlockNonCetBinaries: {:?} }}",
            self.UserShadowStack, self.SetContextIpValidation, self.BlockNonCetBinaries
        )
    }
}

#[repr(C)]
pub struct RTL_IMAGE_MITIGATION_REDIRECTION_TRUST_POLICY {
    pub BlockUntrustedRedirections: RTL_IMAGE_MITIGATION_POLICY,
}

impl Default for RTL_IMAGE_MITIGATION_REDIRECTION_TRUST_POLICY {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_IMAGE_MITIGATION_REDIRECTION_TRUST_POLICY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RTL_IMAGE_MITIGATION_REDIRECTION_TRUST_POLICY {{ BlockUntrustedRedirections: {:?} }}",
            self.BlockUntrustedRedirections
        )
    }
}

#[repr(C)]
pub struct RTL_IMAGE_MITIGATION_USER_POINTER_AUTH_POLICY {
    pub PointerAuthUserIp: RTL_IMAGE_MITIGATION_POLICY,
}

impl Default for RTL_IMAGE_MITIGATION_USER_POINTER_AUTH_POLICY {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_IMAGE_MITIGATION_USER_POINTER_AUTH_POLICY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RTL_IMAGE_MITIGATION_USER_POINTER_AUTH_POLICY {{ PointerAuthUserIp: {:?} }}",
            self.PointerAuthUserIp
        )
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum RTL_IMAGE_MITIGATION_OPTION_STATE {
    RtlMitigationOptionStateNotConfigured = 0,
    RtlMitigationOptionStateOn = 1,
    RtlMitigationOptionStateOff = 2,
    RtlMitigationOptionStateForce = 3,
    RtlMitigationOptionStateOption = 4,
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlQueryImageMitigationPolicy(
        ImagePath: PWSTR,
        Policy: IMAGE_MITIGATION_POLICY,
        Flags: u32,
        Buffer: *mut std::ffi::c_void,
        BufferSize: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlSetImageMitigationPolicy(
        ImagePath: PWSTR,
        Policy: IMAGE_MITIGATION_POLICY,
        Flags: u32,
        Buffer: *mut std::ffi::c_void,
        BufferSize: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlGetCurrentServiceSessionId() -> u32;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlGetTokenNamedObjectPath(
        TokenHandle: HANDLE,
        Sid: PSID,
        ObjectPath: *mut UNICODE_STRING,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlGetAppContainerNamedObjectPath(
        TokenHandle: HANDLE,
        AppContainerSid: PSID,
        RelativePath: bool,
        ObjectPath: *mut UNICODE_STRING,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlGetAppContainerParent(
        AppContainerSid: PSID,
        AppContainerSidParent: *mut PSID,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlCheckSandboxedToken(TokenHandle: HANDLE, IsSandboxed: *mut bool) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlCheckTokenCapability(
        TokenHandle: HANDLE,
        CapabilitySidToCheck: PSID,
        HasCapability: *mut bool,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlCapabilityCheck(
        TokenHandle: HANDLE,
        CapabilityName: *mut UNICODE_STRING,
        HasCapability: *mut bool,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlCheckTokenMembership(
        TokenHandle: HANDLE,
        SidToCheck: PSID,
        IsMember: *mut bool,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlCheckTokenMembershipEx(
        TokenHandle: HANDLE,
        SidToCheck: PSID,
        Flags: u32,
        IsMember: *mut bool,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlQueryTokenHostIdAsUlong64(TokenHandle: HANDLE, HostId: *mut u64) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlIsParentOfChildAppContainer(
        ParentAppContainerSid: PSID,
        ChildAppContainerSid: PSID,
    ) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlIsCapabilitySid(Sid: PSID) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlIsPackageSid(Sid: PSID) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlIsValidProcessTrustLabelSid(Sid: PSID) -> bool;
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum APPCONTAINER_SID_TYPE {
    NotAppContainerSidType = 0,
    ChildAppContainerSidType = 1,
    ParentAppContainerSidType = 2,
    InvalidAppContainerSidType = 3,
    MaxAppContainerSidType = 4,
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlGetAppContainerSidType(
        AppContainerSid: PSID,
        AppContainerSidType: *mut APPCONTAINER_SID_TYPE,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlFlsAlloc(Callback: PFLS_CALLBACK_FUNCTION, FlsIndex: *mut u32) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlFlsFree(FlsIndex: u32) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlFlsGetValue(FlsIndex: u32, FlsData: *mut *mut std::ffi::c_void) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlFlsSetValue(FlsIndex: u32, FlsData: *mut std::ffi::c_void) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlAppxIsFileOwnedByTrustedInstaller(
        FileHandle: HANDLE,
        IsFileOwnedByTrustedInstaller: *mut bool,
    ) -> NTSTATUS;
}

#[repr(C)]
pub struct PS_PKG_CLAIM {
    pub Flags: u32,
    pub Origin: u32,
}

impl Default for PS_PKG_CLAIM {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PS_PKG_CLAIM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PS_PKG_CLAIM {{  }}")
    }
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlQueryPackageClaims(
        TokenHandle: HANDLE,
        PackageFullName: PWSTR,
        PackageSize: *mut usize,
        AppId: PWSTR,
        AppIdSize: *mut usize,
        DynamicId: *mut GUID,
        PkgClaim: *mut PS_PKG_CLAIM,
        AttributesPresent: *mut u64,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlQueryProtectedPolicy(PolicyGuid: *mut GUID, PolicyValue: *mut usize) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlSetProtectedPolicy(
        PolicyGuid: *mut GUID,
        PolicyValue: usize,
        OldPolicyValue: *mut usize,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlIsEnclaveFeaturePresent(FeatureMask: u32) -> bool;
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum RTL_BSD_ITEM_TYPE {
    RtlBsdItemVersionNumber = 0,
    RtlBsdItemProductType = 1,
    RtlBsdItemAabEnabled = 2,
    RtlBsdItemAabTimeout = 3,
    RtlBsdItemBootGood = 4,
    RtlBsdItemBootShutdown = 5,
    RtlBsdSleepInProgress = 6,
    RtlBsdPowerTransition = 7,
    RtlBsdItemBootAttemptCount = 8,
    RtlBsdItemBootCheckpoint = 9,
    RtlBsdItemBootId = 10,
    RtlBsdItemShutdownBootId = 11,
    RtlBsdItemReportedAbnormalShutdownBootId = 12,
    RtlBsdItemErrorInfo = 13,
    RtlBsdItemPowerButtonPressInfo = 14,
    RtlBsdItemChecksum = 15,
    RtlBsdPowerTransitionExtension = 16,
    RtlBsdItemFeatureConfigurationState = 17,
    RtlBsdItemMax = 18,
}

#[repr(C)]
pub struct RTL_BSD_DATA_POWER_TRANSITION {
    pub PowerButtonTimestamp: i64,
    pub Flags: RTL_BSD_DATA_POWER_TRANSITION_1,
    pub ConnectedStandbyScenarioInstanceId: u8,
    pub ConnectedStandbyEntryReason: u8,
    pub ConnectedStandbyExitReason: u8,
    pub SystemSleepTransitionCount: u16,
    pub LastReferenceTime: i64,
    pub LastReferenceTimeChecksum: u32,
    pub LastUpdateBootId: u32,
}

#[repr(C, packed)]
pub struct RTL_BSD_DATA_POWER_TRANSITION_1 {
    _bitfield_align_1: [u8; 0],
    _bitfield_1: BitfieldUnit<[u8; 1]>,
}

impl Default for RTL_BSD_DATA_POWER_TRANSITION_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_BSD_DATA_POWER_TRANSITION_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RTL_BSD_DATA_POWER_TRANSITION_1 {{ SystemRunning : {:?}, ConnectedStandbyInProgress : {:?}, UserShutdownInProgress : {:?}, SystemShutdownInProgress : {:?}, SleepInProgress : {:?} }}",
            self.SystemRunning(),
            self.ConnectedStandbyInProgress(),
            self.UserShutdownInProgress(),
            self.SystemShutdownInProgress(),
            self.SleepInProgress()
        )
    }
}

impl RTL_BSD_DATA_POWER_TRANSITION_1 {
    #[inline]
    pub fn SystemRunning(&self) -> bool {
        unsafe { std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u8) }
    }

    #[inline]
    pub fn set_SystemRunning(&mut self, val: bool) {
        self._bitfield_1.set(0usize, 1u8, val as u64)
    }

    #[inline]
    pub fn ConnectedStandbyInProgress(&self) -> bool {
        unsafe { std::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u8) }
    }

    #[inline]
    pub fn set_ConnectedStandbyInProgress(&mut self, val: bool) {
        self._bitfield_1.set(1usize, 1u8, val as u64)
    }

    #[inline]
    pub fn UserShutdownInProgress(&self) -> bool {
        unsafe { std::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u8) }
    }

    #[inline]
    pub fn set_UserShutdownInProgress(&mut self, val: bool) {
        self._bitfield_1.set(2usize, 1u8, val as u64)
    }

    #[inline]
    pub fn SystemShutdownInProgress(&self) -> bool {
        unsafe { std::mem::transmute(self._bitfield_1.get(3usize, 1u8) as u8) }
    }

    #[inline]
    pub fn set_SystemShutdownInProgress(&mut self, val: bool) {
        self._bitfield_1.set(3usize, 1u8, val as u64)
    }

    #[inline]
    pub fn SleepInProgress(&self) -> bool {
        unsafe { std::mem::transmute(self._bitfield_1.get(4usize, 4u8) as u8) }
    }

    #[inline]
    pub fn set_SleepInProgress(&mut self, val: bool) {
        self._bitfield_1.set(4usize, 4u8, val as u64)
    }

    #[inline]
    pub fn new_bitfield_1(
        SystemRunning: bool,
        ConnectedStandbyInProgress: bool,
        UserShutdownInProgress: bool,
        SystemShutdownInProgress: bool,
        SleepInProgress: bool,
    ) -> BitfieldUnit<[u8; 1]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 1]> = Default::default();

        bitfield_unit.set(0usize, 1u8, {
            SystemRunning as u64
        });

        bitfield_unit.set(1usize, 1u8, {
            ConnectedStandbyInProgress as u64
        });

        bitfield_unit.set(2usize, 1u8, {
            UserShutdownInProgress as u64
        });

        bitfield_unit.set(3usize, 1u8, {
            SystemShutdownInProgress as u64
        });

        bitfield_unit.set(4usize, 4u8, {
            SleepInProgress as u64
        });

        bitfield_unit
    }
}

impl Default for RTL_BSD_DATA_POWER_TRANSITION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_BSD_DATA_POWER_TRANSITION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RTL_BSD_DATA_POWER_TRANSITION {{ Flags: {:?} }}",
            self.Flags
        )
    }
}

#[repr(C)]
pub struct RTL_BSD_DATA_ERROR_INFO {
    pub BootId: u32,
    pub RepeatCount: u32,
    pub OtherErrorCount: u32,
    pub Code: u32,
    pub OtherErrorCount2: u32,
}

impl Default for RTL_BSD_DATA_ERROR_INFO {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_BSD_DATA_ERROR_INFO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RTL_BSD_DATA_ERROR_INFO {{  }}")
    }
}

#[repr(C)]
pub struct RTL_BSD_POWER_BUTTON_PRESS_INFO {
    pub LastPressTime: i64,
    pub CumulativePressCount: u32,
    pub LastPressBootId: u16,
    pub LastPowerWatchdogStage: u8,
    pub Flags: RTL_BSD_POWER_BUTTON_PRESS_INFO_1,
    pub LastReleaseTime: i64,
    pub CumulativeReleaseCount: u32,
    pub LastReleaseBootId: u16,
    pub ErrorCount: u16,
    pub CurrentConnectedStandbyPhase: u8,
    pub TransitionLatestCheckpointId: u32,
    pub TransitionLatestCheckpointType: u32,
    pub TransitionLatestCheckpointSequenceNumber: u32,
}

#[repr(C, packed)]
pub struct RTL_BSD_POWER_BUTTON_PRESS_INFO_1 {
    _bitfield_align_1: [u8; 0],
    _bitfield_1: BitfieldUnit<[u8; 1]>,
}

impl Default for RTL_BSD_POWER_BUTTON_PRESS_INFO_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_BSD_POWER_BUTTON_PRESS_INFO_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RTL_BSD_POWER_BUTTON_PRESS_INFO_1 {{ WatchdogArmed : {:?}, ShutdownInProgress : {:?} }}",
            self.WatchdogArmed(),
            self.ShutdownInProgress()
        )
    }
}

impl RTL_BSD_POWER_BUTTON_PRESS_INFO_1 {
    #[inline]
    pub fn WatchdogArmed(&self) -> u8 {
        self._bitfield_1.get(0usize, 1u8) as u8
    }

    #[inline]
    pub fn set_WatchdogArmed(&mut self, val: u8) {
        self._bitfield_1.set(0usize, 1u8, val as u64)
    }

    #[inline]
    pub fn ShutdownInProgress(&self) -> u8 {
        self._bitfield_1.get(1usize, 1u8) as u8
    }

    #[inline]
    pub fn set_ShutdownInProgress(&mut self, val: u8) {
        self._bitfield_1.set(1usize, 1u8, val as u64)
    }

    #[inline]
    pub fn new_bitfield_1(WatchdogArmed: u8, ShutdownInProgress: u8) -> BitfieldUnit<[u8; 1]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 1]> = Default::default();

        bitfield_unit.set(0usize, 1u8, WatchdogArmed as u64);

        bitfield_unit.set(1usize, 1u8, ShutdownInProgress as u64);

        bitfield_unit
    }
}

impl Default for RTL_BSD_POWER_BUTTON_PRESS_INFO {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_BSD_POWER_BUTTON_PRESS_INFO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RTL_BSD_POWER_BUTTON_PRESS_INFO {{ Flags: {:?} }}",
            self.Flags
        )
    }
}

#[repr(C)]
pub struct RTL_BSD_ITEM {
    pub Type: RTL_BSD_ITEM_TYPE,
    pub DataBuffer: *mut std::ffi::c_void,
    pub DataLength: u32,
}

impl Default for RTL_BSD_ITEM {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_BSD_ITEM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RTL_BSD_ITEM {{ Type: {:?} }}", self.Type)
    }
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlCreateBootStatusDataFile() -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlLockBootStatusData(FileHandle: *mut HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlUnlockBootStatusData(FileHandle: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlGetSetBootStatusData(
        FileHandle: HANDLE,
        Read: bool,
        DataClass: RTL_BSD_ITEM_TYPE,
        Buffer: *mut std::ffi::c_void,
        BufferSize: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlCheckBootStatusIntegrity(FileHandle: HANDLE, Verified: *mut bool) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlRestoreBootStatusDefaults(FileHandle: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlRestoreSystemBootStatusDefaults() -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlGetSystemBootStatus(
        BootStatusInformationClass: RTL_BSD_ITEM_TYPE,
        DataBuffer: *mut std::ffi::c_void,
        DataLength: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlSetSystemBootStatus(
        BootStatusInformationClass: RTL_BSD_ITEM_TYPE,
        DataBuffer: *mut std::ffi::c_void,
        DataLength: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlCheckPortableOperatingSystem(IsPortable: *mut bool) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlSetPortableOperatingSystem(IsPortable: bool) -> NTSTATUS;
}

pub type PRTL_SECURE_MEMORY_CACHE_CALLBACK = Option<
    unsafe extern "system" fn(arg1: *mut std::ffi::c_void, arg2: usize) -> NTSTATUS,
>;

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlRegisterSecureMemoryCacheCallback(
        Callback: PRTL_SECURE_MEMORY_CACHE_CALLBACK,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlDeregisterSecureMemoryCacheCallback(
        Callback: PRTL_SECURE_MEMORY_CACHE_CALLBACK,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlFlushSecureMemoryCache(
        MemoryCache: *mut std::ffi::c_void,
        MemoryLength: usize,
    ) -> bool;
}

#[repr(C)]
pub struct RTL_FEATURE_USAGE_REPORT {
    pub FeatureId: u32,
    pub ReportingKind: u16,
    pub ReportingOptions: u16,
}

impl Default for RTL_FEATURE_USAGE_REPORT {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_FEATURE_USAGE_REPORT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RTL_FEATURE_USAGE_REPORT {{  }}")
    }
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlNotifyFeatureUsage(FeatureUsageReport: *mut RTL_FEATURE_USAGE_REPORT) -> NTSTATUS;
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum RTL_FEATURE_CONFIGURATION_TYPE {
    RtlFeatureConfigurationBoot = 0,
    RtlFeatureConfigurationRuntime = 1,
    RtlFeatureConfigurationCount = 2,
}

#[repr(C)]
pub struct RTL_FEATURE_CONFIGURATION {
    pub FeatureId: u32,
    pub Anonymous1: RTL_FEATURE_CONFIGURATION_1,
    pub VariantPayload: u32,
}

#[repr(C)]
pub struct RTL_FEATURE_CONFIGURATION_1 {
    pub Flags: UnionField<u32>,
    pub Anonymous1: UnionField<RTL_FEATURE_CONFIGURATION_1_1>,
    pub union_field: u32,
}

#[repr(C)]
#[repr(align(4))]
pub struct RTL_FEATURE_CONFIGURATION_1_1 {
    _bitfield_align_1: [u16; 0],
    _bitfield_1: BitfieldUnit<[u8; 4]>,
}

impl Default for RTL_FEATURE_CONFIGURATION_1_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_FEATURE_CONFIGURATION_1_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RTL_FEATURE_CONFIGURATION_1_1 {{ Priority : {:?}, EnabledState : {:?}, IsWexpConfiguration : {:?}, HasSubscriptions : {:?}, Variant : {:?}, VariantPayloadKind : {:?}, Reserved : {:?} }}",
            self.Priority(),
            self.EnabledState(),
            self.IsWexpConfiguration(),
            self.HasSubscriptions(),
            self.Variant(),
            self.VariantPayloadKind(),
            self.Reserved()
        )
    }
}

impl RTL_FEATURE_CONFIGURATION_1_1 {
    #[inline]
    pub fn Priority(&self) -> u32 {
        self._bitfield_1.get(0usize, 4u8) as u32
    }

    #[inline]
    pub fn set_Priority(&mut self, val: u32) {
        self._bitfield_1.set(0usize, 4u8, val as u64)
    }

    #[inline]
    pub fn EnabledState(&self) -> u32 {
        self._bitfield_1.get(4usize, 2u8) as u32
    }

    #[inline]
    pub fn set_EnabledState(&mut self, val: u32) {
        self._bitfield_1.set(4usize, 2u8, val as u64)
    }

    #[inline]
    pub fn IsWexpConfiguration(&self) -> u32 {
        self._bitfield_1.get(6usize, 1u8) as u32
    }

    #[inline]
    pub fn set_IsWexpConfiguration(&mut self, val: u32) {
        self._bitfield_1.set(6usize, 1u8, val as u64)
    }

    #[inline]
    pub fn HasSubscriptions(&self) -> u32 {
        self._bitfield_1.get(7usize, 1u8) as u32
    }

    #[inline]
    pub fn set_HasSubscriptions(&mut self, val: u32) {
        self._bitfield_1.set(7usize, 1u8, val as u64)
    }

    #[inline]
    pub fn Variant(&self) -> u32 {
        self._bitfield_1.get(8usize, 6u8) as u32
    }

    #[inline]
    pub fn set_Variant(&mut self, val: u32) {
        self._bitfield_1.set(8usize, 6u8, val as u64)
    }

    #[inline]
    pub fn VariantPayloadKind(&self) -> u32 {
        self._bitfield_1.get(14usize, 2u8) as u32
    }

    #[inline]
    pub fn set_VariantPayloadKind(&mut self, val: u32) {
        self._bitfield_1.set(14usize, 2u8, val as u64)
    }

    #[inline]
    pub fn Reserved(&self) -> u32 {
        self._bitfield_1.get(16usize, 16u8) as u32
    }

    #[inline]
    pub fn set_Reserved(&mut self, val: u32) {
        self._bitfield_1.set(16usize, 16u8, val as u64)
    }

    #[inline]
    pub fn new_bitfield_1(
        Priority: u32,
        EnabledState: u32,
        IsWexpConfiguration: u32,
        HasSubscriptions: u32,
        Variant: u32,
        VariantPayloadKind: u32,
        Reserved: u32,
    ) -> BitfieldUnit<[u8; 4]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 4]> = Default::default();

        bitfield_unit.set(0usize, 4u8, Priority as u64);

        bitfield_unit.set(4usize, 2u8, EnabledState as u64);

        bitfield_unit.set(6usize, 1u8, IsWexpConfiguration as u64);

        bitfield_unit.set(7usize, 1u8, HasSubscriptions as u64);

        bitfield_unit.set(8usize, 6u8, Variant as u64);

        bitfield_unit.set(14usize, 2u8, VariantPayloadKind as u64);

        bitfield_unit.set(16usize, 16u8, Reserved as u64);

        bitfield_unit
    }
}

impl Default for RTL_FEATURE_CONFIGURATION_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_FEATURE_CONFIGURATION_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RTL_FEATURE_CONFIGURATION_1 {{ union }}")
    }
}

impl Default for RTL_FEATURE_CONFIGURATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_FEATURE_CONFIGURATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RTL_FEATURE_CONFIGURATION {{ Anonymous1: {:?} }}",
            self.Anonymous1
        )
    }
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlQueryFeatureConfiguration(
        FeatureId: u32,
        FeatureType: RTL_FEATURE_CONFIGURATION_TYPE,
        ChangeStamp: *mut u64,
        FeatureConfiguration: *mut RTL_FEATURE_CONFIGURATION,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlSetFeatureConfigurations(
        ChangeStamp: *mut u64,
        FeatureType: RTL_FEATURE_CONFIGURATION_TYPE,
        FeatureConfiguration: *mut RTL_FEATURE_CONFIGURATION,
        FeatureConfigurationCount: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlQueryAllFeatureConfigurations(
        FeatureType: RTL_FEATURE_CONFIGURATION_TYPE,
        ChangeStamp: *mut u64,
        FeatureConfigurations: *mut RTL_FEATURE_CONFIGURATION,
        FeatureConfigurationCount: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlQueryFeatureConfigurationChangeStamp() -> u64;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlQueryFeatureUsageNotificationSubscriptions(
        FeatureConfiguration: *mut RTL_FEATURE_CONFIGURATION,
        FeatureConfigurationCount: *mut u32,
    ) -> NTSTATUS;
}

pub type PRTL_FEATURE_CONFIGURATION_CHANGE_NOTIFICATION =
    Option<unsafe extern "system" fn(arg1: *mut std::ffi::c_void)>;

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlRegisterFeatureConfigurationChangeNotification(
        Callback: *mut PRTL_FEATURE_CONFIGURATION_CHANGE_NOTIFICATION,
        Context: *mut std::ffi::c_void,
        ChangeStamp: *mut u64,
        NotificationHandle: *mut HANDLE,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlUnregisterFeatureConfigurationChangeNotification(
        NotificationHandle: HANDLE,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlSubscribeForFeatureUsageNotification(
        FeatureConfiguration: *mut RTL_FEATURE_CONFIGURATION,
        FeatureConfigurationCount: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlUnsubscribeFromFeatureUsageNotifications(
        FeatureConfiguration: *mut RTL_FEATURE_CONFIGURATION,
        FeatureConfigurationCount: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlEqualWnfChangeStamps(ChangeStamp1: u32, ChangeStamp2: u32) -> bool;
}

pub type PWNF_USER_CALLBACK = Option<
    unsafe extern "system" fn(
        arg1: WNF_STATE_NAME,
        arg2: u32,
        arg3: *mut WNF_TYPE_ID,
        arg4: *mut std::ffi::c_void,
        arg5: *const std::os::raw::c_void,
        arg6: u32,
    ) -> NTSTATUS,
>;

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlQueryWnfStateData(
        ChangeStamp: *mut u32,
        StateName: WNF_STATE_NAME,
        Callback: PWNF_USER_CALLBACK,
        CallbackContext: *mut std::ffi::c_void,
        TypeId: *mut WNF_TYPE_ID,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlPublishWnfStateData(
        StateName: WNF_STATE_NAME,
        TypeId: *const WNF_TYPE_ID,
        Buffer: *const std::os::raw::c_void,
        Length: u32,
        ExplicitScope: *const std::os::raw::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlSubscribeWnfStateChangeNotification(
        SubscriptionHandle: *mut *mut std::ffi::c_void,
        StateName: WNF_STATE_NAME,
        ChangeStamp: u32,
        Callback: PWNF_USER_CALLBACK,
        CallbackContext: *mut std::ffi::c_void,
        TypeId: *const WNF_TYPE_ID,
        SerializationGroup: u32,
        Flags: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlUnsubscribeWnfStateChangeNotification(Callback: PWNF_USER_CALLBACK) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtCopyFileChunk(
        SourceHandle: HANDLE,
        DestinationHandle: HANDLE,
        EventHandle: HANDLE,
        IoStatusBlock: *mut IO_STATUS_BLOCK,
        Length: u32,
        SourceOffset: *mut i64,
        DestOffset: *mut i64,
        SourceKey: *mut GUID,
        DestKey: *mut GUID,
        Flags: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlQueryPropertyStore(Key: usize, Context: *mut usize) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlRemovePropertyStore(Key: usize, Context: *mut usize) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlCompareExchangePropertyStore(
        Key: usize,
        Comperand: *mut usize,
        Exchange: *mut usize,
        Context: *mut usize,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlWow64ChangeThreadState(
        ThreadStateChangeHandle: HANDLE,
        ThreadHandle: HANDLE,
        StateChangeType: THREAD_STATE_CHANGE_TYPE,
        ExtendedInformation: *mut std::ffi::c_void,
        ExtendedInformationLength: usize,
        Reserved: u64,
    ) -> NTSTATUS;
}
