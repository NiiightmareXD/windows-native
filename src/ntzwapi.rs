use windows::{
    core::{GUID, PWSTR},
    Wdk::{
        Foundation::{OBJECT_ATTRIBUTES, OBJECT_INFORMATION_CLASS},
        Storage::FileSystem::FILE_BASIC_INFORMATION,
        System::{
            Registry::KEY_VALUE_ENTRY,
            SystemServices::{
                DIRECTORY_NOTIFY_INFORMATION_CLASS, IO_SESSION_EVENT, IO_SESSION_STATE,
                KPROFILE_SOURCE, PARTITION_INFORMATION_CLASS,
            },
            Threading::{PROCESSINFOCLASS, THREADINFOCLASS},
        },
    },
    Win32::{
        Foundation::{HANDLE, LUID, NTSTATUS, UNICODE_STRING},
        Security::{
            AUDIT_EVENT_TYPE, GENERIC_MAPPING, OBJECT_TYPE_LIST, PRIVILEGE_SET, PSID,
            SECURITY_DESCRIPTOR, SECURITY_QUALITY_OF_SERVICE, SID_AND_ATTRIBUTES,
            TOKEN_DEFAULT_DACL, TOKEN_GROUPS, TOKEN_MANDATORY_POLICY, TOKEN_OWNER,
            TOKEN_PRIMARY_GROUP, TOKEN_PRIVILEGES, TOKEN_SOURCE, TOKEN_TYPE, TOKEN_USER,
        },
        Storage::FileSystem::FILE_SEGMENT_ELEMENT,
        System::{
            Diagnostics::Debug::{CONTEXT, EXCEPTION_RECORD},
            JobObjects::{JOBOBJECTINFOCLASS, JOB_SET_ARRAY},
            Kernel::{PROCESSOR_NUMBER, WAIT_TYPE, WNF_STATE_NAME},
            Memory::MEM_EXTENDED_PARAMETER,
            Power::{
                DEVICE_POWER_STATE, EXECUTION_STATE, LATENCY_TIME, POWER_ACTION, SYSTEM_POWER_STATE,
            },
            SystemInformation::GROUP_AFFINITY,
            WindowsProgramming::CLIENT_ID,
            IO::{IO_STATUS_BLOCK, PIO_APC_ROUTINE},
        },
    },
};

use crate::{
    ntdbg::{DBGUI_WAIT_STATE_CHANGE, DEBUGOBJECTINFOCLASS},
    ntexapi::{
        ATOM_INFORMATION_CLASS, BOOT_ENTRY, BOOT_OPTIONS, EFI_DRIVER_ENTRY,
        EVENT_INFORMATION_CLASS, FILE_PATH, FILTER_BOOT_OPTION_OPERATION, MUTANT_INFORMATION_CLASS,
        SEMAPHORE_INFORMATION_CLASS, SHUTDOWN_ACTION, SYSDBG_COMMAND, SYSTEM_INFORMATION_CLASS,
        T2_SET_PARAMETERS, TIMER_INFORMATION_CLASS, WNF_DATA_SCOPE, WNF_DELIVERY_DESCRIPTOR,
        WNF_STATE_NAME_INFORMATION, WNF_STATE_NAME_LIFETIME, WNF_TYPE_ID, WORKERFACTORYINFOCLASS,
        WORKER_FACTORY_DEFERRED_WORK,
    },
    ntioapi::{FILE_IO_COMPLETION_INFORMATION, IO_COMPLETION_INFORMATION_CLASS},
    ntlpcapi::{
        ALPC_CONTEXT_ATTR, ALPC_DATA_VIEW_ATTR, ALPC_MESSAGE_ATTRIBUTES,
        ALPC_MESSAGE_INFORMATION_CLASS, ALPC_PORT_ATTRIBUTES, ALPC_PORT_INFORMATION_CLASS,
        ALPC_SECURITY_ATTR, PORT_INFORMATION_CLASS, PORT_MESSAGE, PORT_VIEW, REMOTE_PORT_VIEW,
    },
    ntmisc::{TRACE_CONTROL_INFORMATION_CLASS, VDMSERVICECLASS},
    ntmmapi::SECTION_INFORMATION_CLASS,
    ntobapi::{OBJECT_BOUNDARY_DESCRIPTOR, SYMBOLIC_LINK_INFO_CLASS},
    ntpnpapi::{PLUGPLAY_CONTROL_CLASS, PLUGPLAY_EVENT_BLOCK},
    ntpsapi::{
        INITIAL_TEB, MEMORY_RESERVE_TYPE, PPS_APC_ROUTINE, PROCESS_STATE_CHANGE_TYPE,
        PS_ATTRIBUTE_LIST, PS_CREATE_INFO, THREAD_STATE_CHANGE_TYPE,
    },
    ntregapi::CM_EXTENDED_PARAMETER,
    ntseapi::TOKEN_SECURITY_ATTRIBUTES_INFORMATION,
    phnt_ntdef::PENCLAVE_ROUTINE,
};

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwAcceptConnectPort(
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
    pub fn ZwAccessCheck(
        SecurityDescriptor: *mut SECURITY_DESCRIPTOR,
        ClientToken: HANDLE,
        DesiredAccess: u32,
        GenericMapping: *mut GENERIC_MAPPING,
        PrivilegeSet: *mut PRIVILEGE_SET,
        PrivilegeSetLength: *mut u32,
        GrantedAccess: *mut u32,
        AccessStatus: *mut NTSTATUS,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwAccessCheckAndAuditAlarm(
        SubsystemName: *mut UNICODE_STRING,
        HandleId: *mut std::ffi::c_void,
        ObjectTypeName: *mut UNICODE_STRING,
        ObjectName: *mut UNICODE_STRING,
        SecurityDescriptor: *mut SECURITY_DESCRIPTOR,
        DesiredAccess: u32,
        GenericMapping: *mut GENERIC_MAPPING,
        ObjectCreation: bool,
        GrantedAccess: *mut u32,
        AccessStatus: *mut NTSTATUS,
        GenerateOnClose: *mut bool,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwAccessCheckByType(
        SecurityDescriptor: *mut SECURITY_DESCRIPTOR,
        PrincipalSelfSid: PSID,
        ClientToken: HANDLE,
        DesiredAccess: u32,
        ObjectTypeList: *mut OBJECT_TYPE_LIST,
        ObjectTypeListLength: u32,
        GenericMapping: *mut GENERIC_MAPPING,
        PrivilegeSet: *mut PRIVILEGE_SET,
        PrivilegeSetLength: *mut u32,
        GrantedAccess: *mut u32,
        AccessStatus: *mut NTSTATUS,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwAccessCheckByTypeAndAuditAlarm(
        SubsystemName: *mut UNICODE_STRING,
        HandleId: *mut std::ffi::c_void,
        ObjectTypeName: *mut UNICODE_STRING,
        ObjectName: *mut UNICODE_STRING,
        SecurityDescriptor: *mut SECURITY_DESCRIPTOR,
        PrincipalSelfSid: PSID,
        DesiredAccess: u32,
        AuditType: AUDIT_EVENT_TYPE,
        Flags: u32,
        ObjectTypeList: *mut OBJECT_TYPE_LIST,
        ObjectTypeListLength: u32,
        GenericMapping: *mut GENERIC_MAPPING,
        ObjectCreation: bool,
        GrantedAccess: *mut u32,
        AccessStatus: *mut NTSTATUS,
        GenerateOnClose: *mut bool,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwAccessCheckByTypeResultList(
        SecurityDescriptor: *mut SECURITY_DESCRIPTOR,
        PrincipalSelfSid: PSID,
        ClientToken: HANDLE,
        DesiredAccess: u32,
        ObjectTypeList: *mut OBJECT_TYPE_LIST,
        ObjectTypeListLength: u32,
        GenericMapping: *mut GENERIC_MAPPING,
        PrivilegeSet: *mut PRIVILEGE_SET,
        PrivilegeSetLength: *mut u32,
        GrantedAccess: *mut u32,
        AccessStatus: *mut NTSTATUS,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwAccessCheckByTypeResultListAndAuditAlarm(
        SubsystemName: *mut UNICODE_STRING,
        HandleId: *mut std::ffi::c_void,
        ObjectTypeName: *mut UNICODE_STRING,
        ObjectName: *mut UNICODE_STRING,
        SecurityDescriptor: *mut SECURITY_DESCRIPTOR,
        PrincipalSelfSid: PSID,
        DesiredAccess: u32,
        AuditType: AUDIT_EVENT_TYPE,
        Flags: u32,
        ObjectTypeList: *mut OBJECT_TYPE_LIST,
        ObjectTypeListLength: u32,
        GenericMapping: *mut GENERIC_MAPPING,
        ObjectCreation: bool,
        GrantedAccess: *mut u32,
        AccessStatus: *mut NTSTATUS,
        GenerateOnClose: *mut bool,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwAccessCheckByTypeResultListAndAuditAlarmByHandle(
        SubsystemName: *mut UNICODE_STRING,
        HandleId: *mut std::ffi::c_void,
        ClientToken: HANDLE,
        ObjectTypeName: *mut UNICODE_STRING,
        ObjectName: *mut UNICODE_STRING,
        SecurityDescriptor: *mut SECURITY_DESCRIPTOR,
        PrincipalSelfSid: PSID,
        DesiredAccess: u32,
        AuditType: AUDIT_EVENT_TYPE,
        Flags: u32,
        ObjectTypeList: *mut OBJECT_TYPE_LIST,
        ObjectTypeListLength: u32,
        GenericMapping: *mut GENERIC_MAPPING,
        ObjectCreation: bool,
        GrantedAccess: *mut u32,
        AccessStatus: *mut NTSTATUS,
        GenerateOnClose: *mut bool,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwAcquireCMFViewOwnership(
        TimeStamp: *mut u64,
        tokenTaken: *mut bool,
        replaceExisting: bool,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwAddAtom(AtomName: PWSTR, Length: u32, Atom: *mut u16) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwAddAtomEx(AtomName: PWSTR, Length: u32, Atom: *mut u16, Flags: u32) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwAddBootEntry(BootEntry: *mut BOOT_ENTRY, Id: *mut u32) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwAddDriverEntry(DriverEntry: *mut EFI_DRIVER_ENTRY, Id: *mut u32) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwAdjustGroupsToken(
        TokenHandle: HANDLE,
        ResetToDefault: bool,
        NewState: *mut TOKEN_GROUPS,
        BufferLength: u32,
        PreviousState: *mut TOKEN_GROUPS,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwAdjustPrivilegesToken(
        TokenHandle: HANDLE,
        DisableAllPrivileges: bool,
        NewState: *mut TOKEN_PRIVILEGES,
        BufferLength: u32,
        PreviousState: *mut TOKEN_PRIVILEGES,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwAdjustTokenClaimsAndDeviceGroups(
        TokenHandle: HANDLE,
        UserResetToDefault: bool,
        DeviceResetToDefault: bool,
        DeviceGroupsResetToDefault: bool,
        NewUserState: *mut TOKEN_SECURITY_ATTRIBUTES_INFORMATION,
        NewDeviceState: *mut TOKEN_SECURITY_ATTRIBUTES_INFORMATION,
        NewDeviceGroupsState: *mut TOKEN_GROUPS,
        UserBufferLength: u32,
        PreviousUserState: *mut TOKEN_SECURITY_ATTRIBUTES_INFORMATION,
        DeviceBufferLength: u32,
        PreviousDeviceState: *mut TOKEN_SECURITY_ATTRIBUTES_INFORMATION,
        DeviceGroupsBufferLength: u32,
        PreviousDeviceGroups: *mut TOKEN_GROUPS,
        UserReturnLength: *mut u32,
        DeviceReturnLength: *mut u32,
        DeviceGroupsReturnBufferLength: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwAlertResumeThread(ThreadHandle: HANDLE, PreviousSuspendCount: *mut u32) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwAlertThread(ThreadHandle: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwAlertThreadByThreadId(ThreadId: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwAllocateReserveObject(
        MemoryReserveHandle: *mut HANDLE,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        Type: MEMORY_RESERVE_TYPE,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwAllocateUserPhysicalPages(
        ProcessHandle: HANDLE,
        NumberOfPages: *mut usize,
        UserPfnArray: *mut usize,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwAllocateUserPhysicalPagesEx(
        ProcessHandle: HANDLE,
        NumberOfPages: *mut usize,
        UserPfnArray: *mut usize,
        ExtendedParameters: *mut MEM_EXTENDED_PARAMETER,
        ExtendedParameterCount: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwAllocateUuids(
        Time: *mut u64,
        Range: *mut u32,
        Sequence: *mut u32,
        Seed: *mut i8,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwAlpcAcceptConnectPort(
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
    pub fn ZwAlpcCancelMessage(
        PortHandle: HANDLE,
        Flags: u32,
        MessageContext: *mut ALPC_CONTEXT_ATTR,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwAlpcConnectPort(
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
    pub fn ZwAlpcConnectPortEx(
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
    pub fn ZwAlpcCreatePort(
        PortHandle: *mut HANDLE,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        PortAttributes: *mut ALPC_PORT_ATTRIBUTES,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwAlpcCreatePortSection(
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
    pub fn ZwAlpcCreateResourceReserve(
        PortHandle: HANDLE,
        Flags: u32,
        MessageSize: usize,
        ResourceId: *mut HANDLE,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwAlpcCreateSectionView(
        PortHandle: HANDLE,
        Flags: u32,
        ViewAttributes: *mut ALPC_DATA_VIEW_ATTR,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwAlpcCreateSecurityContext(
        PortHandle: HANDLE,
        Flags: u32,
        SecurityAttribute: *mut ALPC_SECURITY_ATTR,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwAlpcDeletePortSection(
        PortHandle: HANDLE,
        Flags: u32,
        SectionHandle: HANDLE,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwAlpcDeleteResourceReserve(
        PortHandle: HANDLE,
        Flags: u32,
        ResourceId: HANDLE,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwAlpcDeleteSectionView(
        PortHandle: HANDLE,
        Flags: u32,
        ViewBase: *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwAlpcDeleteSecurityContext(
        PortHandle: HANDLE,
        Flags: u32,
        ContextHandle: HANDLE,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwAlpcDisconnectPort(PortHandle: HANDLE, Flags: u32) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwAlpcImpersonateClientContainerOfPort(
        PortHandle: HANDLE,
        Message: *mut PORT_MESSAGE,
        Flags: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwAlpcImpersonateClientOfPort(
        PortHandle: HANDLE,
        Message: *mut PORT_MESSAGE,
        Flags: *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwAlpcOpenSenderProcess(
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
    pub fn ZwAlpcOpenSenderThread(
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
    pub fn ZwAlpcQueryInformation(
        PortHandle: HANDLE,
        PortInformationClass: ALPC_PORT_INFORMATION_CLASS,
        PortInformation: *mut std::ffi::c_void,
        Length: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwAlpcQueryInformationMessage(
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
    pub fn ZwAlpcRevokeSecurityContext(
        PortHandle: HANDLE,
        Flags: u32,
        ContextHandle: HANDLE,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwAlpcSendWaitReceivePort(
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
    pub fn ZwAlpcSetInformation(
        PortHandle: HANDLE,
        PortInformationClass: ALPC_PORT_INFORMATION_CLASS,
        PortInformation: *mut std::ffi::c_void,
        Length: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwAreMappedFilesTheSame(
        File1MappedAsAnImage: *mut std::ffi::c_void,
        File2MappedAsFile: *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwAssignProcessToJobObject(JobHandle: HANDLE, ProcessHandle: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwAssociateWaitCompletionPacket(
        WaitCompletionPacketHandle: HANDLE,
        IoCompletionHandle: HANDLE,
        TargetObjectHandle: HANDLE,
        KeyContext: *mut std::ffi::c_void,
        ApcContext: *mut std::ffi::c_void,
        IoStatus: NTSTATUS,
        IoStatusInformation: usize,
        AlreadySignaled: *mut bool,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwCallbackReturn(
        OutputBuffer: *mut std::ffi::c_void,
        OutputLength: u32,
        Status: NTSTATUS,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwCallEnclave(
        Routine: PENCLAVE_ROUTINE,
        Parameter: *mut std::ffi::c_void,
        WaitForThread: bool,
        ReturnValue: *mut *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwCancelIoFile(FileHandle: HANDLE, IoStatusBlock: *mut IO_STATUS_BLOCK) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwCancelIoFileEx(
        FileHandle: HANDLE,
        IoRequestToCancel: *mut IO_STATUS_BLOCK,
        IoStatusBlock: *mut IO_STATUS_BLOCK,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwCancelSynchronousIoFile(
        ThreadHandle: HANDLE,
        IoRequestToCancel: *mut IO_STATUS_BLOCK,
        IoStatusBlock: *mut IO_STATUS_BLOCK,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwCancelTimer2(TimerHandle: HANDLE, Parameters: *mut std::ffi::c_void) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwCancelWaitCompletionPacket(
        WaitCompletionPacketHandle: HANDLE,
        RemoveSignaledPacket: bool,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwChangeProcessState(
        ProcessStateChangeHandle: HANDLE,
        ProcessHandle: HANDLE,
        StateChangeType: PROCESS_STATE_CHANGE_TYPE,
        ExtendedInformation: *mut std::ffi::c_void,
        ExtendedInformationLength: usize,
        Reserved: u64,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwChangeThreadState(
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
    pub fn ZwClearEvent(EventHandle: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwCloseObjectAuditAlarm(
        SubsystemName: *mut UNICODE_STRING,
        HandleId: *mut std::ffi::c_void,
        GenerateOnClose: bool,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwCompactKeys(Count: u32, KeyArray: *mut HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwCompareObjects(FirstObjectHandle: HANDLE, SecondObjectHandle: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwCompareSigningLevels(FirstSigningLevel: u8, SecondSigningLevel: u8) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwCompareTokens(
        FirstTokenHandle: HANDLE,
        SecondTokenHandle: HANDLE,
        Equal: *mut bool,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwCompleteConnectPort(PortHandle: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwCompressKey(Key: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwConnectPort(
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
    pub fn ZwContinue(ContextRecord: *mut CONTEXT, TestAlert: bool) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwContinueEx(
        ContextRecord: *mut CONTEXT,
        ContinueArgument: *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwCreateDebugObject(
        DebugObjectHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        Flags: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwCreateDirectoryObjectEx(
        DirectoryHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        ShadowDirectoryHandle: HANDLE,
        Flags: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwCreateEnclave(
        ProcessHandle: HANDLE,
        BaseAddress: *mut *mut std::ffi::c_void,
        ZeroBits: usize,
        Size: usize,
        InitialCommitment: usize,
        EnclaveType: u32,
        EnclaveInformation: *mut std::ffi::c_void,
        EnclaveInformationLength: u32,
        EnclaveError: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwCreateEventPair(
        EventPairHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwCreateIoCompletion(
        IoCompletionHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        Count: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwCreateIRTimer(TimerHandle: *mut HANDLE, DesiredAccess: u32) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwCreateJobObject(
        JobHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwCreateJobSet(NumJob: u32, UserJobSet: *mut JOB_SET_ARRAY, Flags: u32) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwCreateKeyedEvent(
        KeyedEventHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        Flags: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwCreateLowBoxToken(
        TokenHandle: *mut HANDLE,
        ExistingTokenHandle: HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        PackageSid: PSID,
        CapabilityCount: u32,
        Capabilities: *mut SID_AND_ATTRIBUTES,
        HandleCount: u32,
        Handles: *mut HANDLE,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwCreateMailslotFile(
        FileHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        IoStatusBlock: *mut IO_STATUS_BLOCK,
        CreateOptions: u32,
        MailslotQuota: u32,
        MaximumMessageSize: u32,
        ReadTimeout: *mut i64,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwCreateMutant(
        MutantHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        InitialOwner: bool,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwCreateNamedPipeFile(
        FileHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        IoStatusBlock: *mut IO_STATUS_BLOCK,
        ShareAccess: u32,
        CreateDisposition: u32,
        CreateOptions: u32,
        NamedPipeType: u32,
        ReadMode: u32,
        CompletionMode: u32,
        MaximumInstances: u32,
        InboundQuota: u32,
        OutboundQuota: u32,
        DefaultTimeout: *mut i64,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwCreatePagingFile(
        PageFileName: *mut UNICODE_STRING,
        MinimumSize: *mut i64,
        MaximumSize: *mut i64,
        Priority: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwCreatePartition(
        ParentPartitionHandle: HANDLE,
        PartitionHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        PreferredNode: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwCreatePort(
        PortHandle: *mut HANDLE,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        MaxConnectionInfoLength: u32,
        MaxMessageLength: u32,
        MaxPoolUsage: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwCreatePrivateNamespace(
        NamespaceHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        BoundaryDescriptor: *mut OBJECT_BOUNDARY_DESCRIPTOR,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwCreateProcess(
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
    pub fn ZwCreateProcessEx(
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
    pub fn ZwCreateProcessStateChange(
        ProcessStateChangeHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        ProcessHandle: HANDLE,
        Reserved: u64,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwCreateProfile(
        ProfileHandle: *mut HANDLE,
        Process: HANDLE,
        ProfileBase: *mut std::ffi::c_void,
        ProfileSize: usize,
        BucketSize: u32,
        Buffer: *mut u32,
        BufferSize: u32,
        ProfileSource: KPROFILE_SOURCE,
        Affinity: usize,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwCreateProfileEx(
        ProfileHandle: *mut HANDLE,
        Process: HANDLE,
        ProfileBase: *mut std::ffi::c_void,
        ProfileSize: usize,
        BucketSize: u32,
        Buffer: *mut u32,
        BufferSize: u32,
        ProfileSource: KPROFILE_SOURCE,
        GroupCount: u16,
        GroupAffinity: *mut GROUP_AFFINITY,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwCreateSectionEx(
        SectionHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        MaximumSize: *mut i64,
        SectionPageProtection: u32,
        AllocationAttributes: u32,
        FileHandle: HANDLE,
        ExtendedParameters: *mut MEM_EXTENDED_PARAMETER,
        ExtendedParameterCount: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwCreateSemaphore(
        SemaphoreHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        InitialCount: i32,
        MaximumCount: i32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwCreateSymbolicLinkObject(
        LinkHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        LinkTarget: *mut UNICODE_STRING,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwCreateThread(
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
    pub fn ZwCreateThreadEx(
        ThreadHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        ProcessHandle: HANDLE,
        StartRoutine: *mut std::ffi::c_void,
        Argument: *mut std::ffi::c_void,
        CreateFlags: u32,
        ZeroBits: usize,
        StackSize: usize,
        MaximumStackSize: usize,
        AttributeList: *mut PS_ATTRIBUTE_LIST,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwCreateThreadStateChange(
        ThreadStateChangeHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        ThreadHandle: HANDLE,
        Reserved: u64,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwCreateTimer2(
        TimerHandle: *mut HANDLE,
        Reserved1: *mut std::ffi::c_void,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        Attributes: u32,
        DesiredAccess: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwCreateToken(
        TokenHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        Type: TOKEN_TYPE,
        AuthenticationId: *mut LUID,
        ExpirationTime: *mut i64,
        User: *mut TOKEN_USER,
        Groups: *mut TOKEN_GROUPS,
        Privileges: *mut TOKEN_PRIVILEGES,
        Owner: *mut TOKEN_OWNER,
        PrimaryGroup: *mut TOKEN_PRIMARY_GROUP,
        DefaultDacl: *mut TOKEN_DEFAULT_DACL,
        Source: *mut TOKEN_SOURCE,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwCreateTokenEx(
        TokenHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        Type: TOKEN_TYPE,
        AuthenticationId: *mut LUID,
        ExpirationTime: *mut i64,
        User: *mut TOKEN_USER,
        Groups: *mut TOKEN_GROUPS,
        Privileges: *mut TOKEN_PRIVILEGES,
        UserAttributes: *mut TOKEN_SECURITY_ATTRIBUTES_INFORMATION,
        DeviceAttributes: *mut TOKEN_SECURITY_ATTRIBUTES_INFORMATION,
        DeviceGroups: *mut TOKEN_GROUPS,
        MandatoryPolicy: *mut TOKEN_MANDATORY_POLICY,
        Owner: *mut TOKEN_OWNER,
        PrimaryGroup: *mut TOKEN_PRIMARY_GROUP,
        DefaultDacl: *mut TOKEN_DEFAULT_DACL,
        Source: *mut TOKEN_SOURCE,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwCreateUserProcess(
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

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwCreateWaitablePort(
        PortHandle: *mut HANDLE,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        MaxConnectionInfoLength: u32,
        MaxMessageLength: u32,
        MaxPoolUsage: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwCreateWaitCompletionPacket(
        WaitCompletionPacketHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwCreateWnfStateName(
        StateName: *mut WNF_STATE_NAME,
        NameLifetime: WNF_STATE_NAME_LIFETIME,
        DataScope: WNF_DATA_SCOPE,
        PersistData: bool,
        TypeId: *const WNF_TYPE_ID,
        MaximumStateSize: u32,
        SecurityDescriptor: *mut SECURITY_DESCRIPTOR,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwCreateWorkerFactory(
        WorkerFactoryHandleReturn: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        CompletionPortHandle: HANDLE,
        WorkerProcessHandle: HANDLE,
        StartRoutine: *mut std::ffi::c_void,
        StartParameter: *mut std::ffi::c_void,
        MaxThreadCount: u32,
        StackReserve: usize,
        StackCommit: usize,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwDebugActiveProcess(ProcessHandle: HANDLE, DebugObjectHandle: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwDebugContinue(
        DebugObjectHandle: HANDLE,
        ClientId: *mut CLIENT_ID,
        ContinueStatus: NTSTATUS,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwDelayExecution(Alertable: bool, DelayInterval: *mut i64) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwDeleteAtom(Atom: u16) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwDeleteBootEntry(Id: u32) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwDeleteDriverEntry(Id: u32) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwDeleteObjectAuditAlarm(
        SubsystemName: *mut UNICODE_STRING,
        HandleId: *mut std::ffi::c_void,
        GenerateOnClose: bool,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwDeletePrivateNamespace(NamespaceHandle: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwDeleteWnfStateData(
        StateName: *const WNF_STATE_NAME,
        ExplicitScope: *const std::os::raw::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwDeleteWnfStateName(StateName: *const WNF_STATE_NAME) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwDisableLastKnownGood() -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwEnableLastKnownGood() -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwEnumerateBootEntries(
        Buffer: *mut std::ffi::c_void,
        BufferLength: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwEnumerateDriverEntries(
        Buffer: *mut std::ffi::c_void,
        BufferLength: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwEnumerateSystemEnvironmentValuesEx(
        InformationClass: u32,
        Buffer: *mut std::ffi::c_void,
        BufferLength: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwExtendSection(SectionHandle: HANDLE, NewSectionSize: *mut i64) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwFilterBootOption(
        FilterOperation: FILTER_BOOT_OPTION_OPERATION,
        ObjectType: u32,
        ElementType: u32,
        Data: *mut std::ffi::c_void,
        DataSize: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwFilterToken(
        ExistingTokenHandle: HANDLE,
        Flags: u32,
        SidsToDisable: *mut TOKEN_GROUPS,
        PrivilegesToDelete: *mut TOKEN_PRIVILEGES,
        RestrictedSids: *mut TOKEN_GROUPS,
        NewTokenHandle: *mut HANDLE,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwFilterTokenEx(
        ExistingTokenHandle: HANDLE,
        Flags: u32,
        SidsToDisable: *mut TOKEN_GROUPS,
        PrivilegesToDelete: *mut TOKEN_PRIVILEGES,
        RestrictedSids: *mut TOKEN_GROUPS,
        DisableUserClaimsCount: u32,
        UserClaimsToDisable: *mut UNICODE_STRING,
        DisableDeviceClaimsCount: u32,
        DeviceClaimsToDisable: *mut UNICODE_STRING,
        DeviceGroupsToDisable: *mut TOKEN_GROUPS,
        RestrictedUserAttributes: *mut TOKEN_SECURITY_ATTRIBUTES_INFORMATION,
        RestrictedDeviceAttributes: *mut TOKEN_SECURITY_ATTRIBUTES_INFORMATION,
        RestrictedDeviceGroups: *mut TOKEN_GROUPS,
        NewTokenHandle: *mut HANDLE,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwFindAtom(AtomName: PWSTR, Length: u32, Atom: *mut u16) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwFlushInstallUILanguage(InstallUILanguage: u16, SetComittedFlag: u32) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwFlushInstructionCache(
        ProcessHandle: HANDLE,
        BaseAddress: *mut std::ffi::c_void,
        Length: usize,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwFlushProcessWriteBuffers() -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwFlushWriteBuffer() -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwFreeUserPhysicalPages(
        ProcessHandle: HANDLE,
        NumberOfPages: *mut usize,
        UserPfnArray: *mut usize,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwFreezeRegistry(TimeOutInSeconds: u32) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwFreezeTransactions(FreezeTimeout: *mut i64, ThawTimeout: *mut i64) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwGetCachedSigningLevel(
        File: HANDLE,
        Flags: *mut u32,
        SigningLevel: *mut u8,
        Thumbprint: *mut u8,
        ThumbprintSize: *mut u32,
        ThumbprintAlgorithm: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwGetCompleteWnfStateSubscription(
        OldDescriptorStateName: *mut WNF_STATE_NAME,
        OldSubscriptionId: *mut u64,
        OldDescriptorEventMask: u32,
        OldDescriptorStatus: u32,
        NewDeliveryDescriptor: *mut WNF_DELIVERY_DESCRIPTOR,
        DescriptorSize: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwGetContextThread(ThreadHandle: HANDLE, ThreadContext: *mut CONTEXT) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwGetCurrentProcessorNumber() -> u32;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwGetCurrentProcessorNumberEx(ProcessorNumber: *mut PROCESSOR_NUMBER) -> u32;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwGetDevicePowerState(Device: HANDLE, State: *mut DEVICE_POWER_STATE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwGetMUIRegistryInfo(
        Flags: u32,
        DataSize: *mut u32,
        Data: *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwGetNextProcess(
        ProcessHandle: HANDLE,
        DesiredAccess: u32,
        HandleAttributes: u32,
        Flags: u32,
        NewProcessHandle: *mut HANDLE,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwGetNextThread(
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
    pub fn ZwGetNlsSectionPtr(
        SectionType: u32,
        SectionData: u32,
        ContextData: *mut std::ffi::c_void,
        SectionPointer: *mut *mut std::ffi::c_void,
        SectionSize: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwGetPlugPlayEvent(
        EventHandle: HANDLE,
        Context: *mut std::ffi::c_void,
        EventBlock: *mut PLUGPLAY_EVENT_BLOCK,
        EventBufferSize: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwGetWriteWatch(
        ProcessHandle: HANDLE,
        Flags: u32,
        BaseAddress: *mut std::ffi::c_void,
        RegionSize: usize,
        UserAddressArray: *mut *mut std::ffi::c_void,
        EntriesInUserAddressArray: *mut usize,
        Granularity: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwImpersonateAnonymousToken(ThreadHandle: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwImpersonateClientOfPort(PortHandle: HANDLE, Message: *mut PORT_MESSAGE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwImpersonateThread(
        ServerThreadHandle: HANDLE,
        ClientThreadHandle: HANDLE,
        SecurityQos: *mut SECURITY_QUALITY_OF_SERVICE,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwInitializeEnclave(
        ProcessHandle: HANDLE,
        BaseAddress: *mut std::ffi::c_void,
        EnclaveInformation: *mut std::ffi::c_void,
        EnclaveInformationLength: u32,
        EnclaveError: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwInitializeNlsFiles(
        BaseAddress: *mut *mut std::ffi::c_void,
        DefaultLocaleId: *mut u32,
        DefaultCasingTableSize: *mut i64,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwInitializeRegistry(BootCondition: u16) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwInitiatePowerAction(
        SystemAction: POWER_ACTION,
        LightestSystemState: SYSTEM_POWER_STATE,
        Flags: u32,
        Asynchronous: bool,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwIsProcessInJob(ProcessHandle: HANDLE, JobHandle: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwIsSystemResumeAutomatic() -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwIsUILanguageComitted() -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwListenPort(PortHandle: HANDLE, ConnectionRequest: *mut PORT_MESSAGE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwLoadEnclaveData(
        ProcessHandle: HANDLE,
        BaseAddress: *mut std::ffi::c_void,
        Buffer: *mut std::ffi::c_void,
        BufferSize: usize,
        Protect: u32,
        PageInformation: *mut std::ffi::c_void,
        PageInformationLength: u32,
        NumberOfBytesWritten: *mut usize,
        EnclaveError: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwLoadKey(
        TargetKey: *mut OBJECT_ATTRIBUTES,
        SourceFile: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwLoadKey2(
        TargetKey: *mut OBJECT_ATTRIBUTES,
        SourceFile: *mut OBJECT_ATTRIBUTES,
        Flags: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwLoadKey3(
        TargetKey: *mut OBJECT_ATTRIBUTES,
        SourceFile: *mut OBJECT_ATTRIBUTES,
        Flags: u32,
        ExtendedParameters: *mut CM_EXTENDED_PARAMETER,
        ExtendedParameterCount: u32,
        DesiredAccess: u32,
        RootHandle: *mut HANDLE,
        Reserved: *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwLoadKeyEx(
        TargetKey: *mut OBJECT_ATTRIBUTES,
        SourceFile: *mut OBJECT_ATTRIBUTES,
        Flags: u32,
        TrustClassKey: HANDLE,
        Event: HANDLE,
        DesiredAccess: u32,
        RootHandle: *mut HANDLE,
        Reserved: *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwLockProductActivationKeys(pPrivateVer: *mut u32, pSafeMode: *mut u32) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwLockRegistryKey(KeyHandle: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwLockVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut *mut std::ffi::c_void,
        RegionSize: *mut usize,
        MapType: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwMakePermanentObject(Handle: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwManagePartition(
        TargetHandle: HANDLE,
        SourceHandle: HANDLE,
        PartitionInformationClass: PARTITION_INFORMATION_CLASS,
        PartitionInformation: *mut std::ffi::c_void,
        PartitionInformationLength: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwMapCMFModule(
        What: u32,
        Index: u32,
        CacheIndexOut: *mut u32,
        CacheFlagsOut: *mut u32,
        ViewSizeOut: *mut u32,
        BaseAddress: *mut *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwMapUserPhysicalPages(
        VirtualAddress: *mut std::ffi::c_void,
        NumberOfPages: usize,
        UserPfnArray: *mut usize,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwMapUserPhysicalPagesScatter(
        VirtualAddresses: *mut *mut std::ffi::c_void,
        NumberOfPages: usize,
        UserPfnArray: *mut usize,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwModifyBootEntry(BootEntry: *mut BOOT_ENTRY) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwModifyDriverEntry(DriverEntry: *mut EFI_DRIVER_ENTRY) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwNotifyChangeDirectoryFile(
        FileHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: *mut std::ffi::c_void,
        IoStatusBlock: *mut IO_STATUS_BLOCK,
        Buffer: *mut std::ffi::c_void,
        Length: u32,
        CompletionFilter: u32,
        WatchTree: bool,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwNotifyChangeDirectoryFileEx(
        FileHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: *mut std::ffi::c_void,
        IoStatusBlock: *mut IO_STATUS_BLOCK,
        Buffer: *mut std::ffi::c_void,
        Length: u32,
        CompletionFilter: u32,
        WatchTree: bool,
        DirectoryNotifyInformationClass: DIRECTORY_NOTIFY_INFORMATION_CLASS,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwNotifyChangeMultipleKeys(
        MasterKeyHandle: HANDLE,
        Count: u32,
        SubordinateObjects: *mut OBJECT_ATTRIBUTES,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: *mut std::ffi::c_void,
        IoStatusBlock: *mut IO_STATUS_BLOCK,
        CompletionFilter: u32,
        WatchTree: bool,
        Buffer: *mut std::ffi::c_void,
        BufferSize: u32,
        Asynchronous: bool,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwNotifyChangeSession(
        SessionHandle: HANDLE,
        ChangeSequenceNumber: u32,
        ChangeTimeStamp: *mut i64,
        Event: IO_SESSION_EVENT,
        NewState: IO_SESSION_STATE,
        PreviousState: IO_SESSION_STATE,
        Payload: *mut std::ffi::c_void,
        PayloadSize: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwOpenEventPair(
        EventPairHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwOpenIoCompletion(
        IoCompletionHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwOpenJobObject(
        JobHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwOpenKeyedEvent(
        KeyedEventHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwOpenMutant(
        MutantHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwOpenObjectAuditAlarm(
        SubsystemName: *mut UNICODE_STRING,
        HandleId: *mut std::ffi::c_void,
        ObjectTypeName: *mut UNICODE_STRING,
        ObjectName: *mut UNICODE_STRING,
        SecurityDescriptor: *mut SECURITY_DESCRIPTOR,
        ClientToken: HANDLE,
        DesiredAccess: u32,
        GrantedAccess: u32,
        Privileges: *mut PRIVILEGE_SET,
        ObjectCreation: bool,
        AccessGranted: bool,
        GenerateOnClose: *mut bool,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwOpenPartition(
        PartitionHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwOpenPrivateNamespace(
        NamespaceHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        BoundaryDescriptor: *mut OBJECT_BOUNDARY_DESCRIPTOR,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwOpenSemaphore(
        SemaphoreHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwOpenSession(
        SessionHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwOpenThread(
        ThreadHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        ClientId: *mut CLIENT_ID,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwOpenThreadToken(
        ThreadHandle: HANDLE,
        DesiredAccess: u32,
        OpenAsSelf: bool,
        TokenHandle: *mut HANDLE,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwPlugPlayControl(
        PnPControlClass: PLUGPLAY_CONTROL_CLASS,
        PnPControlData: *mut std::ffi::c_void,
        PnPControlDataLength: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwPrivilegeCheck(
        ClientToken: HANDLE,
        RequiredPrivileges: *mut PRIVILEGE_SET,
        Result: *mut bool,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwPrivilegedServiceAuditAlarm(
        SubsystemName: *mut UNICODE_STRING,
        ServiceName: *mut UNICODE_STRING,
        ClientToken: HANDLE,
        Privileges: *mut PRIVILEGE_SET,
        AccessGranted: bool,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwPrivilegeObjectAuditAlarm(
        SubsystemName: *mut UNICODE_STRING,
        HandleId: *mut std::ffi::c_void,
        ClientToken: HANDLE,
        DesiredAccess: u32,
        Privileges: *mut PRIVILEGE_SET,
        AccessGranted: bool,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwPropagationComplete(
        ResourceManagerHandle: HANDLE,
        RequestCookie: u32,
        BufferLength: u32,
        Buffer: *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwPropagationFailed(
        ResourceManagerHandle: HANDLE,
        RequestCookie: u32,
        PropStatus: NTSTATUS,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwProtectVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut *mut std::ffi::c_void,
        RegionSize: *mut usize,
        NewProtect: u32,
        OldProtect: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwPulseEvent(EventHandle: HANDLE, PreviousState: *mut i32) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwQueryAttributesFile(
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        FileInformation: *mut FILE_BASIC_INFORMATION,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwQueryBootEntryOrder(Ids: *mut u32, Count: *mut u32) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwQueryBootOptions(
        BootOptions: *mut BOOT_OPTIONS,
        BootOptionsLength: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwQueryDebugFilterState(ComponentId: u32, Level: u32) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwQueryDefaultLocale(UserProfile: bool, DefaultLocaleId: *mut u32) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwQueryDefaultUILanguage(DefaultUILanguageId: *mut u16) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwQueryDirectoryObject(
        DirectoryHandle: HANDLE,
        Buffer: *mut std::ffi::c_void,
        Length: u32,
        ReturnSingleEntry: bool,
        RestartScan: bool,
        Context: *mut u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwQueryDriverEntryOrder(Ids: *mut u32, Count: *mut u32) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwQueryEvent(
        EventHandle: HANDLE,
        EventInformationClass: EVENT_INFORMATION_CLASS,
        EventInformation: *mut std::ffi::c_void,
        EventInformationLength: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwQueryInformationAtom(
        Atom: u16,
        AtomInformationClass: ATOM_INFORMATION_CLASS,
        AtomInformation: *mut std::ffi::c_void,
        AtomInformationLength: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwQueryInformationJobObject(
        JobHandle: HANDLE,
        JobObjectInformationClass: JOBOBJECTINFOCLASS,
        JobObjectInformation: *mut std::ffi::c_void,
        JobObjectInformationLength: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwQueryInformationPort(
        PortHandle: HANDLE,
        PortInformationClass: PORT_INFORMATION_CLASS,
        PortInformation: *mut std::ffi::c_void,
        Length: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwQueryInformationProcess(
        ProcessHandle: HANDLE,
        ProcessInformationClass: PROCESSINFOCLASS,
        ProcessInformation: *mut std::ffi::c_void,
        ProcessInformationLength: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwQueryInformationThread(
        ThreadHandle: HANDLE,
        ThreadInformationClass: THREADINFOCLASS,
        ThreadInformation: *mut std::ffi::c_void,
        ThreadInformationLength: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwQueryInformationWorkerFactory(
        WorkerFactoryHandle: HANDLE,
        WorkerFactoryInformationClass: WORKERFACTORYINFOCLASS,
        WorkerFactoryInformation: *mut std::ffi::c_void,
        WorkerFactoryInformationLength: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwQueryInstallUILanguage(InstallUILanguageId: *mut u16) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwQueryIntervalProfile(ProfileSource: KPROFILE_SOURCE, Interval: *mut u32) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwQueryIoCompletion(
        IoCompletionHandle: HANDLE,
        IoCompletionInformationClass: IO_COMPLETION_INFORMATION_CLASS,
        IoCompletionInformation: *mut std::ffi::c_void,
        IoCompletionInformationLength: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwQueryLicenseValue(
        ValueName: *mut UNICODE_STRING,
        Type: *mut u32,
        Data: *mut std::ffi::c_void,
        DataSize: u32,
        ResultDataSize: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwQueryMultipleValueKey(
        KeyHandle: HANDLE,
        ValueEntries: *mut KEY_VALUE_ENTRY,
        EntryCount: u32,
        ValueBuffer: *mut std::ffi::c_void,
        BufferLength: *mut u32,
        RequiredBufferLength: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwQueryMutant(
        MutantHandle: HANDLE,
        MutantInformationClass: MUTANT_INFORMATION_CLASS,
        MutantInformation: *mut std::ffi::c_void,
        MutantInformationLength: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwQueryOpenSubKeys(TargetKey: *mut OBJECT_ATTRIBUTES, HandleCount: *mut u32)
    -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwQueryOpenSubKeysEx(
        TargetKey: *mut OBJECT_ATTRIBUTES,
        BufferLength: u32,
        Buffer: *mut std::ffi::c_void,
        RequiredSize: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwQueryPerformanceCounter(
        PerformanceCounter: *mut i64,
        PerformanceFrequency: *mut i64,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwQuerySection(
        SectionHandle: HANDLE,
        SectionInformationClass: SECTION_INFORMATION_CLASS,
        SectionInformation: *mut std::ffi::c_void,
        SectionInformationLength: usize,
        ReturnLength: *mut usize,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwQuerySecurityAttributesToken(
        TokenHandle: HANDLE,
        Attributes: *mut UNICODE_STRING,
        NumberOfAttributes: u32,
        Buffer: *mut std::ffi::c_void,
        Length: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwQuerySemaphore(
        SemaphoreHandle: HANDLE,
        SemaphoreInformationClass: SEMAPHORE_INFORMATION_CLASS,
        SemaphoreInformation: *mut std::ffi::c_void,
        SemaphoreInformationLength: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwQuerySystemEnvironmentValue(
        VariableName: *mut UNICODE_STRING,
        VariableValue: PWSTR,
        ValueLength: u16,
        ReturnLength: *mut u16,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwQuerySystemEnvironmentValueEx(
        VariableName: *mut UNICODE_STRING,
        VendorGuid: *mut GUID,
        Value: *mut std::ffi::c_void,
        ValueLength: *mut u32,
        Attributes: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwQuerySystemInformation(
        SystemInformationClass: SYSTEM_INFORMATION_CLASS,
        SystemInformation: *mut std::ffi::c_void,
        SystemInformationLength: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwQuerySystemInformationEx(
        SystemInformationClass: SYSTEM_INFORMATION_CLASS,
        InputBuffer: *mut std::ffi::c_void,
        InputBufferLength: u32,
        SystemInformation: *mut std::ffi::c_void,
        SystemInformationLength: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwQuerySystemTime(SystemTime: *mut i64) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwQueryTimer(
        TimerHandle: HANDLE,
        TimerInformationClass: TIMER_INFORMATION_CLASS,
        TimerInformation: *mut std::ffi::c_void,
        TimerInformationLength: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwQueryTimerResolution(
        MaximumTime: *mut u32,
        MinimumTime: *mut u32,
        CurrentTime: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwQueryWnfStateData(
        StateName: *const WNF_STATE_NAME,
        TypeId: *const WNF_TYPE_ID,
        ExplicitScope: *const std::os::raw::c_void,
        ChangeStamp: *mut u32,
        Buffer: *mut std::ffi::c_void,
        BufferSize: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwQueryWnfStateNameInformation(
        StateName: *const WNF_STATE_NAME,
        NameInfoClass: WNF_STATE_NAME_INFORMATION,
        ExplicitScope: *const std::os::raw::c_void,
        InfoBuffer: *mut std::ffi::c_void,
        InfoBufferSize: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwQueueApcThread(
        ThreadHandle: HANDLE,
        ApcRoutine: PPS_APC_ROUTINE,
        ApcArgument1: *mut std::ffi::c_void,
        ApcArgument2: *mut std::ffi::c_void,
        ApcArgument3: *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwQueueApcThreadEx(
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
    pub fn ZwQueueApcThreadEx2(
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
    pub fn ZwRaiseException(
        ExceptionRecord: *mut EXCEPTION_RECORD,
        ContextRecord: *mut CONTEXT,
        FirstChance: bool,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwRaiseHardError(
        ErrorStatus: NTSTATUS,
        NumberOfParameters: u32,
        UnicodeStringParameterMask: u32,
        Parameters: *mut usize,
        ValidResponseOptions: u32,
        Response: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwReadFileScatter(
        FileHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: *mut std::ffi::c_void,
        IoStatusBlock: *mut IO_STATUS_BLOCK,
        SegmentArray: *mut FILE_SEGMENT_ELEMENT,
        Length: u32,
        ByteOffset: *mut i64,
        Key: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwReadRequestData(
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
    pub fn ZwReadVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut std::ffi::c_void,
        Buffer: *mut std::ffi::c_void,
        BufferSize: usize,
        NumberOfBytesRead: *mut usize,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwRegisterProtocolAddressInformation(
        ResourceManager: HANDLE,
        ProtocolId: *mut GUID,
        ProtocolInformationSize: u32,
        ProtocolInformation: *mut std::ffi::c_void,
        CreateOptions: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwRegisterThreadTerminatePort(PortHandle: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwReleaseCMFViewOwnership() -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwReleaseKeyedEvent(
        KeyedEventHandle: HANDLE,
        KeyValue: *mut std::ffi::c_void,
        Alertable: bool,
        Timeout: *mut i64,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwReleaseMutant(MutantHandle: HANDLE, PreviousCount: *mut i32) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwReleaseSemaphore(
        SemaphoreHandle: HANDLE,
        ReleaseCount: i32,
        PreviousCount: *mut i32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwReleaseWorkerFactoryWorker(WorkerFactoryHandle: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwRemoveIoCompletion(
        IoCompletionHandle: HANDLE,
        KeyContext: *mut *mut std::ffi::c_void,
        ApcContext: *mut *mut std::ffi::c_void,
        IoStatusBlock: *mut IO_STATUS_BLOCK,
        Timeout: *mut i64,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwRemoveIoCompletionEx(
        IoCompletionHandle: HANDLE,
        IoCompletionInformation: *mut FILE_IO_COMPLETION_INFORMATION,
        Count: u32,
        NumEntriesRemoved: *mut u32,
        Timeout: *mut i64,
        Alertable: bool,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwRemoveProcessDebug(ProcessHandle: HANDLE, DebugObjectHandle: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwRenameTransactionManager(
        LogFileName: *mut UNICODE_STRING,
        ExistingTransactionManagerGuid: *mut GUID,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwReplaceKey(
        NewFile: *mut OBJECT_ATTRIBUTES,
        TargetHandle: HANDLE,
        OldFile: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwReplacePartitionUnit(
        TargetInstancePath: *mut UNICODE_STRING,
        SpareInstancePath: *mut UNICODE_STRING,
        Flags: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwReplyPort(PortHandle: HANDLE, ReplyMessage: *mut PORT_MESSAGE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwReplyWaitReceivePort(
        PortHandle: HANDLE,
        PortContext: *mut *mut std::ffi::c_void,
        ReplyMessage: *mut PORT_MESSAGE,
        ReceiveMessage: *mut PORT_MESSAGE,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwReplyWaitReceivePortEx(
        PortHandle: HANDLE,
        PortContext: *mut *mut std::ffi::c_void,
        ReplyMessage: *mut PORT_MESSAGE,
        ReceiveMessage: *mut PORT_MESSAGE,
        Timeout: *mut i64,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwReplyWaitReplyPort(PortHandle: HANDLE, ReplyMessage: *mut PORT_MESSAGE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwRequestPort(PortHandle: HANDLE, RequestMessage: *mut PORT_MESSAGE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwRequestWaitReplyPort(
        PortHandle: HANDLE,
        RequestMessage: *mut PORT_MESSAGE,
        ReplyMessage: *mut PORT_MESSAGE,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwRequestWakeupLatency(latency: LATENCY_TIME) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwResetEvent(EventHandle: HANDLE, PreviousState: *mut i32) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwResetWriteWatch(
        ProcessHandle: HANDLE,
        BaseAddress: *mut std::ffi::c_void,
        RegionSize: usize,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwResumeProcess(ProcessHandle: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwResumeThread(ThreadHandle: HANDLE, PreviousSuspendCount: *mut u32) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwRevertContainerImpersonation() -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwSaveMergedKeys(
        HighPrecedenceKeyHandle: HANDLE,
        LowPrecedenceKeyHandle: HANDLE,
        FileHandle: HANDLE,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwSecureConnectPort(
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
    pub fn ZwSerializeBoot() -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwSetBootEntryOrder(Ids: *mut u32, Count: u32) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwSetBootOptions(BootOptions: *mut BOOT_OPTIONS, FieldsToChange: u32) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwSetCachedSigningLevel(
        Flags: u32,
        InputSigningLevel: u8,
        SourceFiles: *mut HANDLE,
        SourceFileCount: u32,
        TargetFile: HANDLE,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwSetContextThread(ThreadHandle: HANDLE, ThreadContext: *mut CONTEXT) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwSetDebugFilterState(ComponentId: u32, Level: u32, State: bool) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwSetDefaultHardErrorPort(DefaultHardErrorPort: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwSetDefaultLocale(UserProfile: bool, DefaultLocaleId: u32) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwSetDefaultUILanguage(DefaultUILanguageId: u16) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwSetDriverEntryOrder(Ids: *mut u32, Count: u32) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwSetEventBoostPriority(EventHandle: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwSetHighEventPair(EventPairHandle: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwSetHighWaitLowEventPair(EventPairHandle: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwSetInformationDebugObject(
        DebugObjectHandle: HANDLE,
        DebugObjectInformationClass: DEBUGOBJECTINFOCLASS,
        DebugInformation: *mut std::ffi::c_void,
        DebugInformationLength: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwSetInformationJobObject(
        JobHandle: HANDLE,
        JobObjectInformationClass: JOBOBJECTINFOCLASS,
        JobObjectInformation: *mut std::ffi::c_void,
        JobObjectInformationLength: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwSetInformationObject(
        Handle: HANDLE,
        ObjectInformationClass: OBJECT_INFORMATION_CLASS,
        ObjectInformation: *mut std::ffi::c_void,
        ObjectInformationLength: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwSetInformationProcess(
        ProcessHandle: HANDLE,
        ProcessInformationClass: PROCESSINFOCLASS,
        ProcessInformation: *mut std::ffi::c_void,
        ProcessInformationLength: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwSetInformationSymbolicLink(
        LinkHandle: HANDLE,
        SymbolicLinkInformationClass: SYMBOLIC_LINK_INFO_CLASS,
        SymbolicLinkInformation: *mut std::ffi::c_void,
        SymbolicLinkInformationLength: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwSetInformationWorkerFactory(
        WorkerFactoryHandle: HANDLE,
        WorkerFactoryInformationClass: WORKERFACTORYINFOCLASS,
        WorkerFactoryInformation: *mut std::ffi::c_void,
        WorkerFactoryInformationLength: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwSetIntervalProfile(Interval: u32, Source: KPROFILE_SOURCE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwSetIoCompletion(
        IoCompletionHandle: HANDLE,
        KeyContext: *mut std::ffi::c_void,
        ApcContext: *mut std::ffi::c_void,
        IoStatus: NTSTATUS,
        IoStatusInformation: usize,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwSetIoCompletionEx(
        IoCompletionHandle: HANDLE,
        IoCompletionPacketHandle: HANDLE,
        KeyContext: *mut std::ffi::c_void,
        ApcContext: *mut std::ffi::c_void,
        IoStatus: NTSTATUS,
        IoStatusInformation: usize,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwSetIRTimer(TimerHandle: HANDLE, DueTime: *mut i64) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwSetLdtEntries(
        Selector0: u32,
        Entry0Low: u32,
        Entry0Hi: u32,
        Selector1: u32,
        Entry1Low: u32,
        Entry1Hi: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwSetLowEventPair(EventPairHandle: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwSetLowWaitHighEventPair(EventPairHandle: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwSetSystemEnvironmentValue(
        VariableName: *mut UNICODE_STRING,
        VariableValue: *mut UNICODE_STRING,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwSetSystemEnvironmentValueEx(
        VariableName: *mut UNICODE_STRING,
        VendorGuid: *mut GUID,
        Value: *mut std::ffi::c_void,
        ValueLength: u32,
        Attributes: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwSetSystemInformation(
        SystemInformationClass: SYSTEM_INFORMATION_CLASS,
        SystemInformation: *mut std::ffi::c_void,
        SystemInformationLength: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwSetSystemPowerState(
        SystemAction: POWER_ACTION,
        LightestSystemState: SYSTEM_POWER_STATE,
        Flags: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwSetSystemTime(SystemTime: *mut i64, PreviousTime: *mut i64) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwSetThreadExecutionState(
        NewFlags: EXECUTION_STATE,
        PreviousFlags: *mut EXECUTION_STATE,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwSetTimer2(
        TimerHandle: HANDLE,
        DueTime: *mut i64,
        Period: *mut i64,
        Parameters: *mut T2_SET_PARAMETERS,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwSetTimerResolution(
        DesiredTime: u32,
        SetResolution: bool,
        ActualTime: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwSetUuidSeed(Seed: *mut i8) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwSetWnfProcessNotificationEvent(NotificationEvent: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwShutdownSystem(Action: SHUTDOWN_ACTION) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwShutdownWorkerFactory(
        WorkerFactoryHandle: HANDLE,
        PendingWorkerCount: *mut i32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwSignalAndWaitForSingleObject(
        SignalHandle: HANDLE,
        WaitHandle: HANDLE,
        Alertable: bool,
        Timeout: *mut i64,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwStartProfile(ProfileHandle: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwStopProfile(ProfileHandle: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwSubscribeWnfStateChange(
        StateName: *const WNF_STATE_NAME,
        ChangeStamp: u32,
        EventMask: u32,
        SubscriptionId: *mut u64,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwSuspendProcess(ProcessHandle: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwSuspendThread(ThreadHandle: HANDLE, PreviousSuspendCount: *mut u32) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwSystemDebugControl(
        Command: SYSDBG_COMMAND,
        InputBuffer: *mut std::ffi::c_void,
        InputBufferLength: u32,
        OutputBuffer: *mut std::ffi::c_void,
        OutputBufferLength: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwTerminateEnclave(
        BaseAddress: *mut std::ffi::c_void,
        WaitForThread: bool,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwTerminateJobObject(JobHandle: HANDLE, ExitStatus: NTSTATUS) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwTerminateThread(ThreadHandle: HANDLE, ExitStatus: NTSTATUS) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwTestAlert() -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwThawRegistry() -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwThawTransactions() -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwTraceControl(
        TraceInformationClass: TRACE_CONTROL_INFORMATION_CLASS,
        InputBuffer: *mut std::ffi::c_void,
        InputBufferLength: u32,
        TraceInformation: *mut std::ffi::c_void,
        TraceInformationLength: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwTraceEvent(
        TraceHandle: HANDLE,
        Flags: u32,
        FieldSize: u32,
        Fields: *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwTranslateFilePath(
        InputFilePath: *mut FILE_PATH,
        OutputType: u32,
        OutputFilePath: *mut FILE_PATH,
        OutputFilePathLength: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwUmsThreadYield(SchedulerParam: *mut std::ffi::c_void) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwUnloadKey(TargetKey: *mut OBJECT_ATTRIBUTES) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwUnloadKey2(TargetKey: *mut OBJECT_ATTRIBUTES, Flags: u32) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwUnloadKeyEx(TargetKey: *mut OBJECT_ATTRIBUTES, Event: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwUnlockVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut *mut std::ffi::c_void,
        RegionSize: *mut usize,
        MapType: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwUnmapViewOfSectionEx(
        ProcessHandle: HANDLE,
        BaseAddress: *mut std::ffi::c_void,
        Flags: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwUnsubscribeWnfStateChange(StateName: *const WNF_STATE_NAME) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwUpdateWnfStateData(
        StateName: *const WNF_STATE_NAME,
        Buffer: *const std::os::raw::c_void,
        Length: u32,
        TypeId: *const WNF_TYPE_ID,
        ExplicitScope: *const std::os::raw::c_void,
        MatchingChangeStamp: u32,
        CheckStamp: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwVdmControl(Service: VDMSERVICECLASS, ServiceData: *mut std::ffi::c_void) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwWaitForAlertByThreadId(Address: *mut std::ffi::c_void, Timeout: *mut i64) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwWaitForDebugEvent(
        DebugObjectHandle: HANDLE,
        Alertable: bool,
        Timeout: *mut i64,
        WaitStateChange: *mut DBGUI_WAIT_STATE_CHANGE,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwWaitForKeyedEvent(
        KeyedEventHandle: HANDLE,
        KeyValue: *mut std::ffi::c_void,
        Alertable: bool,
        Timeout: *mut i64,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwWaitForMultipleObjects(
        Count: u32,
        Handles: *mut HANDLE,
        WaitType: WAIT_TYPE,
        Alertable: bool,
        Timeout: *mut i64,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwWaitForMultipleObjects32(
        Count: u32,
        Handles: *mut i32,
        WaitType: WAIT_TYPE,
        Alertable: bool,
        Timeout: *mut i64,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwWaitForWorkViaWorkerFactory(
        WorkerFactoryHandle: HANDLE,
        MiniPackets: *mut FILE_IO_COMPLETION_INFORMATION,
        Count: u32,
        PacketsReturned: *mut u32,
        DeferredWork: *mut WORKER_FACTORY_DEFERRED_WORK,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwWaitHighEventPair(EventPairHandle: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwWaitLowEventPair(EventPairHandle: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwWorkerFactoryWorkerReady(WorkerFactoryHandle: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwWriteFileGather(
        FileHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: *mut std::ffi::c_void,
        IoStatusBlock: *mut IO_STATUS_BLOCK,
        SegmentArray: *mut FILE_SEGMENT_ELEMENT,
        Length: u32,
        ByteOffset: *mut i64,
        Key: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwWriteRequestData(
        PortHandle: HANDLE,
        Message: *mut PORT_MESSAGE,
        DataEntryIndex: u32,
        Buffer: *mut std::ffi::c_void,
        BufferSize: usize,
        NumberOfBytesWritten: *mut usize,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwWriteVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut std::ffi::c_void,
        Buffer: *mut std::ffi::c_void,
        BufferSize: usize,
        NumberOfBytesWritten: *mut usize,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn ZwYieldExecution() -> NTSTATUS;
}
