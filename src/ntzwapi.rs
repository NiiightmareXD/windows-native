use windows::{
    core::{GUID, PWSTR},
    Wdk::Foundation::OBJECT_ATTRIBUTES,
    Win32::{
        Foundation::{BOOLEAN, HANDLE, LUID, NTSTATUS, PSID, UNICODE_STRING},
        Security::{
            AUDIT_EVENT_TYPE, GENERIC_MAPPING, OBJECT_TYPE_LIST, PRIVILEGE_SET,
            SECURITY_DESCRIPTOR, SECURITY_QUALITY_OF_SERVICE, SID_AND_ATTRIBUTES,
            TOKEN_DEFAULT_DACL, TOKEN_GROUPS, TOKEN_INFORMATION_CLASS, TOKEN_MANDATORY_POLICY,
            TOKEN_OWNER, TOKEN_PRIMARY_GROUP, TOKEN_PRIVILEGES, TOKEN_SOURCE, TOKEN_TYPE,
            TOKEN_USER,
        },
        Storage::FileSystem::{FILE_SEGMENT_ELEMENT, TRANSACTION_NOTIFICATION},
        System::{
            Diagnostics::Debug::{CONTEXT, EXCEPTION_RECORD},
            JobObjects::{JOBOBJECTINFOCLASS, JOB_SET_ARRAY},
            Kernel::{EVENT_TYPE, PROCESSOR_NUMBER, TIMER_TYPE, WAIT_TYPE, WNF_STATE_NAME},
            Memory::MEM_EXTENDED_PARAMETER,
            Power::{
                DEVICE_POWER_STATE, EXECUTION_STATE, LATENCY_TIME, POWER_ACTION,
                POWER_INFORMATION_LEVEL, SYSTEM_POWER_STATE,
            },
            SystemInformation::GROUP_AFFINITY,
            SystemServices::{
                ENLISTMENT_INFORMATION_CLASS, KTMOBJECT_CURSOR, KTMOBJECT_TYPE,
                RESOURCEMANAGER_INFORMATION_CLASS, TRANSACTIONMANAGER_INFORMATION_CLASS,
                TRANSACTION_INFORMATION_CLASS,
            },
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
        PTIMER_APC_ROUTINE, SEMAPHORE_INFORMATION_CLASS, SHUTDOWN_ACTION, SYSDBG_COMMAND,
        SYSTEM_INFORMATION_CLASS, T2_SET_PARAMETERS, TIMER_INFORMATION_CLASS,
        TIMER_SET_INFORMATION_CLASS, WNF_DATA_SCOPE, WNF_DELIVERY_DESCRIPTOR,
        WNF_STATE_NAME_INFORMATION, WNF_STATE_NAME_LIFETIME, WNF_TYPE_ID, WORKERFACTORYINFOCLASS,
        WORKER_FACTORY_DEFERRED_WORK,
    },
    ntioapi::{
        DIRECTORY_NOTIFY_INFORMATION_CLASS, FILE_BASIC_INFORMATION, FILE_INFORMATION_CLASS,
        FILE_IO_COMPLETION_INFORMATION, FILE_NETWORK_OPEN_INFORMATION, FSINFOCLASS,
        IO_COMPLETION_INFORMATION_CLASS, IO_SESSION_EVENT, IO_SESSION_STATE,
    },
    ntkeapi::KPROFILE_SOURCE,
    ntlpcapi::{
        ALPC_CONTEXT_ATTR, ALPC_DATA_VIEW_ATTR, ALPC_HANDLE, ALPC_MESSAGE_ATTRIBUTES,
        ALPC_MESSAGE_INFORMATION_CLASS, ALPC_PORT_ATTRIBUTES, ALPC_PORT_INFORMATION_CLASS,
        ALPC_SECURITY_ATTR, PALPC_HANDLE, PORT_INFORMATION_CLASS, PORT_MESSAGE, PORT_VIEW,
        REMOTE_PORT_VIEW,
    },
    ntmisc::{TRACE_CONTROL_INFORMATION_CLASS, VDMSERVICECLASS},
    ntmmapi::{
        MEMORY_INFORMATION_CLASS, MEMORY_RANGE_ENTRY, PARTITION_INFORMATION_CLASS,
        SECTION_INFORMATION_CLASS, SECTION_INHERIT, VIRTUAL_MEMORY_INFORMATION_CLASS,
    },
    ntobapi::{OBJECT_BOUNDARY_DESCRIPTOR, OBJECT_INFORMATION_CLASS, SYMBOLIC_LINK_INFO_CLASS},
    ntpnpapi::{PLUGPLAY_CONTROL_CLASS, PLUGPLAY_EVENT_BLOCK},
    ntpsapi::{
        INITIAL_TEB, MEMORY_RESERVE_TYPE, PPS_APC_ROUTINE, PROCESSINFOCLASS,
        PROCESS_STATE_CHANGE_TYPE, PS_ATTRIBUTE_LIST, PS_CREATE_INFO, THREADINFOCLASS,
        THREAD_STATE_CHANGE_TYPE,
    },
    ntregapi::{
        CM_EXTENDED_PARAMETER, KEY_INFORMATION_CLASS, KEY_SET_INFORMATION_CLASS, KEY_VALUE_ENTRY,
        KEY_VALUE_INFORMATION_CLASS,
    },
    ntseapi::TOKEN_SECURITY_ATTRIBUTES_INFORMATION,
    phnt_ntdef::PENCLAVE_ROUTINE,
};

extern "C" {
    pub fn ZwAcceptConnectPort(
        PortHandle: *mut HANDLE,
        PortContext: *mut std::ffi::c_void,
        ConnectionRequest: *mut PORT_MESSAGE,
        AcceptConnection: BOOLEAN,
        ServerView: *mut PORT_VIEW,
        ClientView: *mut REMOTE_PORT_VIEW,
    ) -> NTSTATUS;
}
extern "C" {
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
extern "C" {
    pub fn ZwAccessCheckAndAuditAlarm(
        SubsystemName: *mut UNICODE_STRING,
        HandleId: *mut std::ffi::c_void,
        ObjectTypeName: *mut UNICODE_STRING,
        ObjectName: *mut UNICODE_STRING,
        SecurityDescriptor: *mut SECURITY_DESCRIPTOR,
        DesiredAccess: u32,
        GenericMapping: *mut GENERIC_MAPPING,
        ObjectCreation: BOOLEAN,
        GrantedAccess: *mut u32,
        AccessStatus: *mut NTSTATUS,
        GenerateOnClose: *mut BOOLEAN,
    ) -> NTSTATUS;
}
extern "C" {
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
extern "C" {
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
        ObjectCreation: BOOLEAN,
        GrantedAccess: *mut u32,
        AccessStatus: *mut NTSTATUS,
        GenerateOnClose: *mut BOOLEAN,
    ) -> NTSTATUS;
}
extern "C" {
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
extern "C" {
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
        ObjectCreation: BOOLEAN,
        GrantedAccess: *mut u32,
        AccessStatus: *mut NTSTATUS,
        GenerateOnClose: *mut BOOLEAN,
    ) -> NTSTATUS;
}
extern "C" {
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
        ObjectCreation: BOOLEAN,
        GrantedAccess: *mut u32,
        AccessStatus: *mut NTSTATUS,
        GenerateOnClose: *mut BOOLEAN,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwAcquireCMFViewOwnership(
        TimeStamp: *mut u64,
        tokenTaken: *mut BOOLEAN,
        replaceExisting: BOOLEAN,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwAddAtom(AtomName: PWSTR, Length: u32, Atom: *mut u16) -> NTSTATUS;
}
extern "C" {
    pub fn ZwAddAtomEx(AtomName: PWSTR, Length: u32, Atom: *mut u16, Flags: u32) -> NTSTATUS;
}
extern "C" {
    pub fn ZwAddBootEntry(BootEntry: *mut BOOT_ENTRY, Id: *mut u32) -> NTSTATUS;
}
extern "C" {
    pub fn ZwAddDriverEntry(DriverEntry: *mut EFI_DRIVER_ENTRY, Id: *mut u32) -> NTSTATUS;
}
extern "C" {
    pub fn ZwAdjustGroupsToken(
        TokenHandle: HANDLE,
        ResetToDefault: BOOLEAN,
        NewState: *mut TOKEN_GROUPS,
        BufferLength: u32,
        PreviousState: *mut TOKEN_GROUPS,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwAdjustPrivilegesToken(
        TokenHandle: HANDLE,
        DisableAllPrivileges: BOOLEAN,
        NewState: *mut TOKEN_PRIVILEGES,
        BufferLength: u32,
        PreviousState: *mut TOKEN_PRIVILEGES,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwAdjustTokenClaimsAndDeviceGroups(
        TokenHandle: HANDLE,
        UserResetToDefault: BOOLEAN,
        DeviceResetToDefault: BOOLEAN,
        DeviceGroupsResetToDefault: BOOLEAN,
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
extern "C" {
    pub fn ZwAlertResumeThread(ThreadHandle: HANDLE, PreviousSuspendCount: *mut u32) -> NTSTATUS;
}
extern "C" {
    pub fn ZwAlertThread(ThreadHandle: HANDLE) -> NTSTATUS;
}
extern "C" {
    pub fn ZwAlertThreadByThreadId(ThreadId: HANDLE) -> NTSTATUS;
}
extern "C" {
    pub fn ZwAllocateLocallyUniqueId(Luid: *mut LUID) -> NTSTATUS;
}
extern "C" {
    pub fn ZwAllocateReserveObject(
        MemoryReserveHandle: *mut HANDLE,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        Type: MEMORY_RESERVE_TYPE,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwAllocateUserPhysicalPages(
        ProcessHandle: HANDLE,
        NumberOfPages: *mut usize,
        UserPfnArray: *mut usize,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwAllocateUserPhysicalPagesEx(
        ProcessHandle: HANDLE,
        NumberOfPages: *mut usize,
        UserPfnArray: *mut usize,
        ExtendedParameters: *mut MEM_EXTENDED_PARAMETER,
        ExtendedParameterCount: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwAllocateUuids(
        Time: *mut u64,
        Range: *mut u32,
        Sequence: *mut u32,
        Seed: *mut i8,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwAllocateVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut *mut std::ffi::c_void,
        ZeroBits: usize,
        RegionSize: *mut usize,
        AllocationType: u32,
        Protect: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwAlpcAcceptConnectPort(
        PortHandle: *mut HANDLE,
        ConnectionPortHandle: HANDLE,
        Flags: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        PortAttributes: *mut ALPC_PORT_ATTRIBUTES,
        PortContext: *mut std::ffi::c_void,
        ConnectionRequest: *mut PORT_MESSAGE,
        ConnectionMessageAttributes: *mut ALPC_MESSAGE_ATTRIBUTES,
        AcceptConnection: BOOLEAN,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwAlpcCancelMessage(
        PortHandle: HANDLE,
        Flags: u32,
        MessageContext: *mut ALPC_CONTEXT_ATTR,
    ) -> NTSTATUS;
}
extern "C" {
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
extern "C" {
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
extern "C" {
    pub fn ZwAlpcCreatePort(
        PortHandle: *mut HANDLE,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        PortAttributes: *mut ALPC_PORT_ATTRIBUTES,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwAlpcCreatePortSection(
        PortHandle: HANDLE,
        Flags: u32,
        SectionHandle: HANDLE,
        SectionSize: usize,
        AlpcSectionHandle: PALPC_HANDLE,
        ActualSectionSize: *mut usize,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwAlpcCreateResourceReserve(
        PortHandle: HANDLE,
        Flags: u32,
        MessageSize: usize,
        ResourceId: PALPC_HANDLE,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwAlpcCreateSectionView(
        PortHandle: HANDLE,
        Flags: u32,
        ViewAttributes: *mut ALPC_DATA_VIEW_ATTR,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwAlpcCreateSecurityContext(
        PortHandle: HANDLE,
        Flags: u32,
        SecurityAttribute: *mut ALPC_SECURITY_ATTR,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwAlpcDeletePortSection(
        PortHandle: HANDLE,
        Flags: u32,
        SectionHandle: ALPC_HANDLE,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwAlpcDeleteResourceReserve(
        PortHandle: HANDLE,
        Flags: u32,
        ResourceId: ALPC_HANDLE,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwAlpcDeleteSectionView(
        PortHandle: HANDLE,
        Flags: u32,
        ViewBase: *mut std::ffi::c_void,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwAlpcDeleteSecurityContext(
        PortHandle: HANDLE,
        Flags: u32,
        ContextHandle: ALPC_HANDLE,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwAlpcDisconnectPort(PortHandle: HANDLE, Flags: u32) -> NTSTATUS;
}
extern "C" {
    pub fn ZwAlpcImpersonateClientContainerOfPort(
        PortHandle: HANDLE,
        Message: *mut PORT_MESSAGE,
        Flags: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwAlpcImpersonateClientOfPort(
        PortHandle: HANDLE,
        Message: *mut PORT_MESSAGE,
        Flags: *mut std::ffi::c_void,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwAlpcOpenSenderProcess(
        ProcessHandle: *mut HANDLE,
        PortHandle: HANDLE,
        PortMessage: *mut PORT_MESSAGE,
        Flags: u32,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwAlpcOpenSenderThread(
        ThreadHandle: *mut HANDLE,
        PortHandle: HANDLE,
        PortMessage: *mut PORT_MESSAGE,
        Flags: u32,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwAlpcQueryInformation(
        PortHandle: HANDLE,
        PortInformationClass: ALPC_PORT_INFORMATION_CLASS,
        PortInformation: *mut std::ffi::c_void,
        Length: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwAlpcQueryInformationMessage(
        PortHandle: HANDLE,
        PortMessage: *mut PORT_MESSAGE,
        MessageInformationClass: ALPC_MESSAGE_INFORMATION_CLASS,
        MessageInformation: *mut std::ffi::c_void,
        Length: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwAlpcRevokeSecurityContext(
        PortHandle: HANDLE,
        Flags: u32,
        ContextHandle: ALPC_HANDLE,
    ) -> NTSTATUS;
}
extern "C" {
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
extern "C" {
    pub fn ZwAlpcSetInformation(
        PortHandle: HANDLE,
        PortInformationClass: ALPC_PORT_INFORMATION_CLASS,
        PortInformation: *mut std::ffi::c_void,
        Length: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwAreMappedFilesTheSame(
        File1MappedAsAnImage: *mut std::ffi::c_void,
        File2MappedAsFile: *mut std::ffi::c_void,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwAssignProcessToJobObject(JobHandle: HANDLE, ProcessHandle: HANDLE) -> NTSTATUS;
}
extern "C" {
    pub fn ZwAssociateWaitCompletionPacket(
        WaitCompletionPacketHandle: HANDLE,
        IoCompletionHandle: HANDLE,
        TargetObjectHandle: HANDLE,
        KeyContext: *mut std::ffi::c_void,
        ApcContext: *mut std::ffi::c_void,
        IoStatus: NTSTATUS,
        IoStatusInformation: usize,
        AlreadySignaled: *mut BOOLEAN,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwCallbackReturn(
        OutputBuffer: *mut std::ffi::c_void,
        OutputLength: u32,
        Status: NTSTATUS,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwCallEnclave(
        Routine: PENCLAVE_ROUTINE,
        Parameter: *mut std::ffi::c_void,
        WaitForThread: BOOLEAN,
        ReturnValue: *mut *mut std::ffi::c_void,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwCancelIoFile(FileHandle: HANDLE, IoStatusBlock: *mut IO_STATUS_BLOCK) -> NTSTATUS;
}
extern "C" {
    pub fn ZwCancelIoFileEx(
        FileHandle: HANDLE,
        IoRequestToCancel: *mut IO_STATUS_BLOCK,
        IoStatusBlock: *mut IO_STATUS_BLOCK,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwCancelSynchronousIoFile(
        ThreadHandle: HANDLE,
        IoRequestToCancel: *mut IO_STATUS_BLOCK,
        IoStatusBlock: *mut IO_STATUS_BLOCK,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwCancelTimer(TimerHandle: HANDLE, CurrentState: *mut BOOLEAN) -> NTSTATUS;
}
extern "C" {
    pub fn ZwCancelTimer2(TimerHandle: HANDLE, Parameters: *mut std::ffi::c_void) -> NTSTATUS;
}
extern "C" {
    pub fn ZwCancelWaitCompletionPacket(
        WaitCompletionPacketHandle: HANDLE,
        RemoveSignaledPacket: BOOLEAN,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwChangeProcessState(
        ProcessStateChangeHandle: HANDLE,
        ProcessHandle: HANDLE,
        StateChangeType: PROCESS_STATE_CHANGE_TYPE,
        ExtendedInformation: *mut std::ffi::c_void,
        ExtendedInformationLength: usize,
        Reserved: u64,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwChangeThreadState(
        ThreadStateChangeHandle: HANDLE,
        ThreadHandle: HANDLE,
        StateChangeType: THREAD_STATE_CHANGE_TYPE,
        ExtendedInformation: *mut std::ffi::c_void,
        ExtendedInformationLength: usize,
        Reserved: u64,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwClearEvent(EventHandle: HANDLE) -> NTSTATUS;
}
extern "C" {
    pub fn ZwClose(Handle: HANDLE) -> NTSTATUS;
}
extern "C" {
    pub fn ZwCloseObjectAuditAlarm(
        SubsystemName: *mut UNICODE_STRING,
        HandleId: *mut std::ffi::c_void,
        GenerateOnClose: BOOLEAN,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwCommitComplete(EnlistmentHandle: HANDLE, TmVirtualClock: *mut i64) -> NTSTATUS;
}
extern "C" {
    pub fn ZwCommitEnlistment(EnlistmentHandle: HANDLE, TmVirtualClock: *mut i64) -> NTSTATUS;
}
extern "C" {
    pub fn ZwCommitTransaction(TransactionHandle: HANDLE, Wait: BOOLEAN) -> NTSTATUS;
}
extern "C" {
    pub fn ZwCompactKeys(Count: u32, KeyArray: *mut HANDLE) -> NTSTATUS;
}
extern "C" {
    pub fn ZwCompareObjects(FirstObjectHandle: HANDLE, SecondObjectHandle: HANDLE) -> NTSTATUS;
}
extern "C" {
    pub fn ZwCompareSigningLevels(FirstSigningLevel: u8, SecondSigningLevel: u8) -> NTSTATUS;
}
extern "C" {
    pub fn ZwCompareTokens(
        FirstTokenHandle: HANDLE,
        SecondTokenHandle: HANDLE,
        Equal: *mut BOOLEAN,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwCompleteConnectPort(PortHandle: HANDLE) -> NTSTATUS;
}
extern "C" {
    pub fn ZwCompressKey(Key: HANDLE) -> NTSTATUS;
}
extern "C" {
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
extern "C" {
    pub fn ZwContinue(ContextRecord: *mut CONTEXT, TestAlert: BOOLEAN) -> NTSTATUS;
}
extern "C" {
    pub fn ZwContinueEx(
        ContextRecord: *mut CONTEXT,
        ContinueArgument: *mut std::ffi::c_void,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwCreateDebugObject(
        DebugObjectHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        Flags: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwCreateDirectoryObject(
        DirectoryHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwCreateDirectoryObjectEx(
        DirectoryHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        ShadowDirectoryHandle: HANDLE,
        Flags: u32,
    ) -> NTSTATUS;
}
extern "C" {
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
extern "C" {
    pub fn ZwCreateEnlistment(
        EnlistmentHandle: *mut HANDLE,
        DesiredAccess: u32,
        ResourceManagerHandle: HANDLE,
        TransactionHandle: HANDLE,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        CreateOptions: u32,
        NotificationMask: u32,
        EnlistmentKey: *mut std::ffi::c_void,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwCreateEvent(
        EventHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        EventType: EVENT_TYPE,
        InitialState: BOOLEAN,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwCreateEventPair(
        EventPairHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwCreateFile(
        FileHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        IoStatusBlock: *mut IO_STATUS_BLOCK,
        AllocationSize: *mut i64,
        FileAttributes: u32,
        ShareAccess: u32,
        CreateDisposition: u32,
        CreateOptions: u32,
        EaBuffer: *mut std::ffi::c_void,
        EaLength: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwCreateIoCompletion(
        IoCompletionHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        Count: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwCreateIRTimer(TimerHandle: *mut HANDLE, DesiredAccess: u32) -> NTSTATUS;
}
extern "C" {
    pub fn ZwCreateJobObject(
        JobHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwCreateJobSet(NumJob: u32, UserJobSet: *mut JOB_SET_ARRAY, Flags: u32) -> NTSTATUS;
}
extern "C" {
    pub fn ZwCreateKey(
        KeyHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        TitleIndex: u32,
        Class: *mut UNICODE_STRING,
        CreateOptions: u32,
        Disposition: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwCreateKeyedEvent(
        KeyedEventHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        Flags: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwCreateKeyTransacted(
        KeyHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        TitleIndex: u32,
        Class: *mut UNICODE_STRING,
        CreateOptions: u32,
        TransactionHandle: HANDLE,
        Disposition: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
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
extern "C" {
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
extern "C" {
    pub fn ZwCreateMutant(
        MutantHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        InitialOwner: BOOLEAN,
    ) -> NTSTATUS;
}
extern "C" {
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
extern "C" {
    pub fn ZwCreatePagingFile(
        PageFileName: *mut UNICODE_STRING,
        MinimumSize: *mut i64,
        MaximumSize: *mut i64,
        Priority: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwCreatePartition(
        ParentPartitionHandle: HANDLE,
        PartitionHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        PreferredNode: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwCreatePort(
        PortHandle: *mut HANDLE,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        MaxConnectionInfoLength: u32,
        MaxMessageLength: u32,
        MaxPoolUsage: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwCreatePrivateNamespace(
        NamespaceHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        BoundaryDescriptor: *mut OBJECT_BOUNDARY_DESCRIPTOR,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwCreateProcess(
        ProcessHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        ParentProcess: HANDLE,
        InheritObjectTable: BOOLEAN,
        SectionHandle: HANDLE,
        DebugPort: HANDLE,
        TokenHandle: HANDLE,
    ) -> NTSTATUS;
}
extern "C" {
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
extern "C" {
    pub fn ZwCreateProcessStateChange(
        ProcessStateChangeHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        ProcessHandle: HANDLE,
        Reserved: u64,
    ) -> NTSTATUS;
}
extern "C" {
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
extern "C" {
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
extern "C" {
    pub fn ZwCreateResourceManager(
        ResourceManagerHandle: *mut HANDLE,
        DesiredAccess: u32,
        TmHandle: HANDLE,
        RmGuid: *mut GUID,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        CreateOptions: u32,
        Description: *mut UNICODE_STRING,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwCreateSection(
        SectionHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        MaximumSize: *mut i64,
        SectionPageProtection: u32,
        AllocationAttributes: u32,
        FileHandle: HANDLE,
    ) -> NTSTATUS;
}
extern "C" {
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
extern "C" {
    pub fn ZwCreateSemaphore(
        SemaphoreHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        InitialCount: i32,
        MaximumCount: i32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwCreateSymbolicLinkObject(
        LinkHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        LinkTarget: *mut UNICODE_STRING,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwCreateThread(
        ThreadHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        ProcessHandle: HANDLE,
        ClientId: *mut CLIENT_ID,
        ThreadContext: *mut CONTEXT,
        InitialTeb: *mut INITIAL_TEB,
        CreateSuspended: BOOLEAN,
    ) -> NTSTATUS;
}
extern "C" {
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
extern "C" {
    pub fn ZwCreateThreadStateChange(
        ThreadStateChangeHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        ThreadHandle: HANDLE,
        Reserved: u64,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwCreateTimer(
        TimerHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        TimerType: TIMER_TYPE,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwCreateTimer2(
        TimerHandle: *mut HANDLE,
        Reserved1: *mut std::ffi::c_void,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        Attributes: u32,
        DesiredAccess: u32,
    ) -> NTSTATUS;
}
extern "C" {
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
extern "C" {
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
extern "C" {
    pub fn ZwCreateTransaction(
        TransactionHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        Uow: *mut GUID,
        TmHandle: HANDLE,
        CreateOptions: u32,
        IsolationLevel: u32,
        IsolationFlags: u32,
        Timeout: *mut i64,
        Description: *mut UNICODE_STRING,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwCreateTransactionManager(
        TmHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        LogFileName: *mut UNICODE_STRING,
        CreateOptions: u32,
        CommitStrength: u32,
    ) -> NTSTATUS;
}
extern "C" {
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
extern "C" {
    pub fn ZwCreateWaitablePort(
        PortHandle: *mut HANDLE,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        MaxConnectionInfoLength: u32,
        MaxMessageLength: u32,
        MaxPoolUsage: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwCreateWaitCompletionPacket(
        WaitCompletionPacketHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwCreateWnfStateName(
        StateName: *mut WNF_STATE_NAME,
        NameLifetime: WNF_STATE_NAME_LIFETIME,
        DataScope: WNF_DATA_SCOPE,
        PersistData: BOOLEAN,
        TypeId: *const WNF_TYPE_ID,
        MaximumStateSize: u32,
        SecurityDescriptor: *mut SECURITY_DESCRIPTOR,
    ) -> NTSTATUS;
}
extern "C" {
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
extern "C" {
    pub fn ZwDebugActiveProcess(ProcessHandle: HANDLE, DebugObjectHandle: HANDLE) -> NTSTATUS;
}
extern "C" {
    pub fn ZwDebugContinue(
        DebugObjectHandle: HANDLE,
        ClientId: *mut CLIENT_ID,
        ContinueStatus: NTSTATUS,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwDelayExecution(Alertable: BOOLEAN, DelayInterval: *mut i64) -> NTSTATUS;
}
extern "C" {
    pub fn ZwDeleteAtom(Atom: u16) -> NTSTATUS;
}
extern "C" {
    pub fn ZwDeleteBootEntry(Id: u32) -> NTSTATUS;
}
extern "C" {
    pub fn ZwDeleteDriverEntry(Id: u32) -> NTSTATUS;
}
extern "C" {
    pub fn ZwDeleteFile(ObjectAttributes: *mut OBJECT_ATTRIBUTES) -> NTSTATUS;
}
extern "C" {
    pub fn ZwDeleteKey(KeyHandle: HANDLE) -> NTSTATUS;
}
extern "C" {
    pub fn ZwDeleteObjectAuditAlarm(
        SubsystemName: *mut UNICODE_STRING,
        HandleId: *mut std::ffi::c_void,
        GenerateOnClose: BOOLEAN,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwDeletePrivateNamespace(NamespaceHandle: HANDLE) -> NTSTATUS;
}
extern "C" {
    pub fn ZwDeleteValueKey(KeyHandle: HANDLE, ValueName: *mut UNICODE_STRING) -> NTSTATUS;
}
extern "C" {
    pub fn ZwDeleteWnfStateData(
        StateName: *const WNF_STATE_NAME,
        ExplicitScope: *const std::os::raw::c_void,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwDeleteWnfStateName(StateName: *const WNF_STATE_NAME) -> NTSTATUS;
}
extern "C" {
    pub fn ZwDeviceIoControlFile(
        FileHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: *mut std::ffi::c_void,
        IoStatusBlock: *mut IO_STATUS_BLOCK,
        IoControlCode: u32,
        InputBuffer: *mut std::ffi::c_void,
        InputBufferLength: u32,
        OutputBuffer: *mut std::ffi::c_void,
        OutputBufferLength: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwDisableLastKnownGood() -> NTSTATUS;
}
extern "C" {
    pub fn ZwDisplayString(String: *mut UNICODE_STRING) -> NTSTATUS;
}
extern "C" {
    pub fn ZwDrawText(Text: *mut UNICODE_STRING) -> NTSTATUS;
}
extern "C" {
    pub fn ZwDuplicateObject(
        SourceProcessHandle: HANDLE,
        SourceHandle: HANDLE,
        TargetProcessHandle: HANDLE,
        TargetHandle: *mut HANDLE,
        DesiredAccess: u32,
        HandleAttributes: u32,
        Options: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwDuplicateToken(
        ExistingTokenHandle: HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        EffectiveOnly: BOOLEAN,
        Type: TOKEN_TYPE,
        NewTokenHandle: *mut HANDLE,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwEnableLastKnownGood() -> NTSTATUS;
}
extern "C" {
    pub fn ZwEnumerateBootEntries(
        Buffer: *mut std::ffi::c_void,
        BufferLength: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwEnumerateDriverEntries(
        Buffer: *mut std::ffi::c_void,
        BufferLength: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwEnumerateKey(
        KeyHandle: HANDLE,
        Index: u32,
        KeyInformationClass: KEY_INFORMATION_CLASS,
        KeyInformation: *mut std::ffi::c_void,
        Length: u32,
        ResultLength: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwEnumerateSystemEnvironmentValuesEx(
        InformationClass: u32,
        Buffer: *mut std::ffi::c_void,
        BufferLength: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwEnumerateTransactionObject(
        RootObjectHandle: HANDLE,
        QueryType: KTMOBJECT_TYPE,
        ObjectCursor: *mut KTMOBJECT_CURSOR,
        ObjectCursorLength: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwEnumerateValueKey(
        KeyHandle: HANDLE,
        Index: u32,
        KeyValueInformationClass: KEY_VALUE_INFORMATION_CLASS,
        KeyValueInformation: *mut std::ffi::c_void,
        Length: u32,
        ResultLength: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwExtendSection(SectionHandle: HANDLE, NewSectionSize: *mut i64) -> NTSTATUS;
}
extern "C" {
    pub fn ZwFilterBootOption(
        FilterOperation: FILTER_BOOT_OPTION_OPERATION,
        ObjectType: u32,
        ElementType: u32,
        Data: *mut std::ffi::c_void,
        DataSize: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwFilterToken(
        ExistingTokenHandle: HANDLE,
        Flags: u32,
        SidsToDisable: *mut TOKEN_GROUPS,
        PrivilegesToDelete: *mut TOKEN_PRIVILEGES,
        RestrictedSids: *mut TOKEN_GROUPS,
        NewTokenHandle: *mut HANDLE,
    ) -> NTSTATUS;
}
extern "C" {
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
extern "C" {
    pub fn ZwFindAtom(AtomName: PWSTR, Length: u32, Atom: *mut u16) -> NTSTATUS;
}
extern "C" {
    pub fn ZwFlushBuffersFile(FileHandle: HANDLE, IoStatusBlock: *mut IO_STATUS_BLOCK) -> NTSTATUS;
}
extern "C" {
    pub fn ZwFlushBuffersFileEx(
        FileHandle: HANDLE,
        Flags: u32,
        Parameters: *mut std::ffi::c_void,
        ParametersSize: u32,
        IoStatusBlock: *mut IO_STATUS_BLOCK,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwFlushInstallUILanguage(InstallUILanguage: u16, SetComittedFlag: u32) -> NTSTATUS;
}
extern "C" {
    pub fn ZwFlushInstructionCache(
        ProcessHandle: HANDLE,
        BaseAddress: *mut std::ffi::c_void,
        Length: usize,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwFlushKey(KeyHandle: HANDLE) -> NTSTATUS;
}
extern "C" {
    pub fn ZwFlushProcessWriteBuffers() -> NTSTATUS;
}
extern "C" {
    pub fn ZwFlushVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut *mut std::ffi::c_void,
        RegionSize: *mut usize,
        IoStatus: *mut IO_STATUS_BLOCK,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwFlushWriteBuffer() -> NTSTATUS;
}
extern "C" {
    pub fn ZwFreeUserPhysicalPages(
        ProcessHandle: HANDLE,
        NumberOfPages: *mut usize,
        UserPfnArray: *mut usize,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwFreeVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut *mut std::ffi::c_void,
        RegionSize: *mut usize,
        FreeType: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwFreezeRegistry(TimeOutInSeconds: u32) -> NTSTATUS;
}
extern "C" {
    pub fn ZwFreezeTransactions(FreezeTimeout: *mut i64, ThawTimeout: *mut i64) -> NTSTATUS;
}
extern "C" {
    pub fn ZwFsControlFile(
        FileHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: *mut std::ffi::c_void,
        IoStatusBlock: *mut IO_STATUS_BLOCK,
        FsControlCode: u32,
        InputBuffer: *mut std::ffi::c_void,
        InputBufferLength: u32,
        OutputBuffer: *mut std::ffi::c_void,
        OutputBufferLength: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwGetCachedSigningLevel(
        File: HANDLE,
        Flags: *mut u32,
        SigningLevel: *mut u8,
        Thumbprint: *mut u8,
        ThumbprintSize: *mut u32,
        ThumbprintAlgorithm: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwGetCompleteWnfStateSubscription(
        OldDescriptorStateName: *mut WNF_STATE_NAME,
        OldSubscriptionId: *mut u64,
        OldDescriptorEventMask: u32,
        OldDescriptorStatus: u32,
        NewDeliveryDescriptor: *mut WNF_DELIVERY_DESCRIPTOR,
        DescriptorSize: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwGetContextThread(ThreadHandle: HANDLE, ThreadContext: *mut CONTEXT) -> NTSTATUS;
}
extern "C" {
    pub fn ZwGetCurrentProcessorNumber() -> u32;
}
extern "C" {
    pub fn ZwGetCurrentProcessorNumberEx(ProcessorNumber: *mut PROCESSOR_NUMBER) -> u32;
}
extern "C" {
    pub fn ZwGetDevicePowerState(Device: HANDLE, State: *mut DEVICE_POWER_STATE) -> NTSTATUS;
}
extern "C" {
    pub fn ZwGetMUIRegistryInfo(
        Flags: u32,
        DataSize: *mut u32,
        Data: *mut std::ffi::c_void,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwGetNextProcess(
        ProcessHandle: HANDLE,
        DesiredAccess: u32,
        HandleAttributes: u32,
        Flags: u32,
        NewProcessHandle: *mut HANDLE,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwGetNextThread(
        ProcessHandle: HANDLE,
        ThreadHandle: HANDLE,
        DesiredAccess: u32,
        HandleAttributes: u32,
        Flags: u32,
        NewThreadHandle: *mut HANDLE,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwGetNlsSectionPtr(
        SectionType: u32,
        SectionData: u32,
        ContextData: *mut std::ffi::c_void,
        SectionPointer: *mut *mut std::ffi::c_void,
        SectionSize: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwGetNotificationResourceManager(
        ResourceManagerHandle: HANDLE,
        TransactionNotification: *mut TRANSACTION_NOTIFICATION,
        NotificationLength: u32,
        Timeout: *mut i64,
        ReturnLength: *mut u32,
        Asynchronous: u32,
        AsynchronousContext: usize,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwGetPlugPlayEvent(
        EventHandle: HANDLE,
        Context: *mut std::ffi::c_void,
        EventBlock: *mut PLUGPLAY_EVENT_BLOCK,
        EventBufferSize: u32,
    ) -> NTSTATUS;
}
extern "C" {
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
extern "C" {
    pub fn ZwImpersonateAnonymousToken(ThreadHandle: HANDLE) -> NTSTATUS;
}
extern "C" {
    pub fn ZwImpersonateClientOfPort(PortHandle: HANDLE, Message: *mut PORT_MESSAGE) -> NTSTATUS;
}
extern "C" {
    pub fn ZwImpersonateThread(
        ServerThreadHandle: HANDLE,
        ClientThreadHandle: HANDLE,
        SecurityQos: *mut SECURITY_QUALITY_OF_SERVICE,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwInitializeEnclave(
        ProcessHandle: HANDLE,
        BaseAddress: *mut std::ffi::c_void,
        EnclaveInformation: *mut std::ffi::c_void,
        EnclaveInformationLength: u32,
        EnclaveError: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwInitializeNlsFiles(
        BaseAddress: *mut *mut std::ffi::c_void,
        DefaultLocaleId: *mut u32,
        DefaultCasingTableSize: *mut i64,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwInitializeRegistry(BootCondition: u16) -> NTSTATUS;
}
extern "C" {
    pub fn ZwInitiatePowerAction(
        SystemAction: POWER_ACTION,
        LightestSystemState: SYSTEM_POWER_STATE,
        Flags: u32,
        Asynchronous: BOOLEAN,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwIsProcessInJob(ProcessHandle: HANDLE, JobHandle: HANDLE) -> NTSTATUS;
}
extern "C" {
    pub fn ZwIsSystemResumeAutomatic() -> BOOLEAN;
}
extern "C" {
    pub fn ZwIsUILanguageComitted() -> NTSTATUS;
}
extern "C" {
    pub fn ZwListenPort(PortHandle: HANDLE, ConnectionRequest: *mut PORT_MESSAGE) -> NTSTATUS;
}
extern "C" {
    pub fn ZwLoadDriver(DriverServiceName: *mut UNICODE_STRING) -> NTSTATUS;
}
extern "C" {
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
extern "C" {
    pub fn ZwLoadKey(
        TargetKey: *mut OBJECT_ATTRIBUTES,
        SourceFile: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwLoadKey2(
        TargetKey: *mut OBJECT_ATTRIBUTES,
        SourceFile: *mut OBJECT_ATTRIBUTES,
        Flags: u32,
    ) -> NTSTATUS;
}
extern "C" {
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
extern "C" {
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
extern "C" {
    pub fn ZwLockFile(
        FileHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: *mut std::ffi::c_void,
        IoStatusBlock: *mut IO_STATUS_BLOCK,
        ByteOffset: *mut i64,
        Length: *mut i64,
        Key: u32,
        FailImmediately: BOOLEAN,
        ExclusiveLock: BOOLEAN,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwLockProductActivationKeys(pPrivateVer: *mut u32, pSafeMode: *mut u32) -> NTSTATUS;
}
extern "C" {
    pub fn ZwLockRegistryKey(KeyHandle: HANDLE) -> NTSTATUS;
}
extern "C" {
    pub fn ZwLockVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut *mut std::ffi::c_void,
        RegionSize: *mut usize,
        MapType: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwMakePermanentObject(Handle: HANDLE) -> NTSTATUS;
}
extern "C" {
    pub fn ZwMakeTemporaryObject(Handle: HANDLE) -> NTSTATUS;
}
extern "C" {
    pub fn ZwManagePartition(
        TargetHandle: HANDLE,
        SourceHandle: HANDLE,
        PartitionInformationClass: PARTITION_INFORMATION_CLASS,
        PartitionInformation: *mut std::ffi::c_void,
        PartitionInformationLength: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwMapCMFModule(
        What: u32,
        Index: u32,
        CacheIndexOut: *mut u32,
        CacheFlagsOut: *mut u32,
        ViewSizeOut: *mut u32,
        BaseAddress: *mut *mut std::ffi::c_void,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwMapUserPhysicalPages(
        VirtualAddress: *mut std::ffi::c_void,
        NumberOfPages: usize,
        UserPfnArray: *mut usize,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwMapUserPhysicalPagesScatter(
        VirtualAddresses: *mut *mut std::ffi::c_void,
        NumberOfPages: usize,
        UserPfnArray: *mut usize,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwMapViewOfSection(
        SectionHandle: HANDLE,
        ProcessHandle: HANDLE,
        BaseAddress: *mut *mut std::ffi::c_void,
        ZeroBits: usize,
        CommitSize: usize,
        SectionOffset: *mut i64,
        ViewSize: *mut usize,
        InheritDisposition: SECTION_INHERIT,
        AllocationType: u32,
        Win32Protect: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwModifyBootEntry(BootEntry: *mut BOOT_ENTRY) -> NTSTATUS;
}
extern "C" {
    pub fn ZwModifyDriverEntry(DriverEntry: *mut EFI_DRIVER_ENTRY) -> NTSTATUS;
}
extern "C" {
    pub fn ZwNotifyChangeDirectoryFile(
        FileHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: *mut std::ffi::c_void,
        IoStatusBlock: *mut IO_STATUS_BLOCK,
        Buffer: *mut std::ffi::c_void,
        Length: u32,
        CompletionFilter: u32,
        WatchTree: BOOLEAN,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwNotifyChangeDirectoryFileEx(
        FileHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: *mut std::ffi::c_void,
        IoStatusBlock: *mut IO_STATUS_BLOCK,
        Buffer: *mut std::ffi::c_void,
        Length: u32,
        CompletionFilter: u32,
        WatchTree: BOOLEAN,
        DirectoryNotifyInformationClass: DIRECTORY_NOTIFY_INFORMATION_CLASS,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwNotifyChangeKey(
        KeyHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: *mut std::ffi::c_void,
        IoStatusBlock: *mut IO_STATUS_BLOCK,
        CompletionFilter: u32,
        WatchTree: BOOLEAN,
        Buffer: *mut std::ffi::c_void,
        BufferSize: u32,
        Asynchronous: BOOLEAN,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwNotifyChangeMultipleKeys(
        MasterKeyHandle: HANDLE,
        Count: u32,
        SubordinateObjects: *mut OBJECT_ATTRIBUTES,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: *mut std::ffi::c_void,
        IoStatusBlock: *mut IO_STATUS_BLOCK,
        CompletionFilter: u32,
        WatchTree: BOOLEAN,
        Buffer: *mut std::ffi::c_void,
        BufferSize: u32,
        Asynchronous: BOOLEAN,
    ) -> NTSTATUS;
}
extern "C" {
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
extern "C" {
    pub fn ZwOpenDirectoryObject(
        DirectoryHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwOpenEnlistment(
        EnlistmentHandle: *mut HANDLE,
        DesiredAccess: u32,
        ResourceManagerHandle: HANDLE,
        EnlistmentGuid: *mut GUID,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwOpenEvent(
        EventHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwOpenEventPair(
        EventPairHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwOpenFile(
        FileHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        IoStatusBlock: *mut IO_STATUS_BLOCK,
        ShareAccess: u32,
        OpenOptions: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwOpenIoCompletion(
        IoCompletionHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwOpenJobObject(
        JobHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwOpenKey(
        KeyHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwOpenKeyedEvent(
        KeyedEventHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwOpenKeyEx(
        KeyHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        OpenOptions: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwOpenKeyTransacted(
        KeyHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        TransactionHandle: HANDLE,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwOpenKeyTransactedEx(
        KeyHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        OpenOptions: u32,
        TransactionHandle: HANDLE,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwOpenMutant(
        MutantHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}
extern "C" {
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
        ObjectCreation: BOOLEAN,
        AccessGranted: BOOLEAN,
        GenerateOnClose: *mut BOOLEAN,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwOpenPartition(
        PartitionHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwOpenPrivateNamespace(
        NamespaceHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        BoundaryDescriptor: *mut OBJECT_BOUNDARY_DESCRIPTOR,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwOpenProcess(
        ProcessHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        ClientId: *mut CLIENT_ID,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwOpenProcessToken(
        ProcessHandle: HANDLE,
        DesiredAccess: u32,
        TokenHandle: *mut HANDLE,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwOpenProcessTokenEx(
        ProcessHandle: HANDLE,
        DesiredAccess: u32,
        HandleAttributes: u32,
        TokenHandle: *mut HANDLE,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwOpenResourceManager(
        ResourceManagerHandle: *mut HANDLE,
        DesiredAccess: u32,
        TmHandle: HANDLE,
        ResourceManagerGuid: *mut GUID,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwOpenSection(
        SectionHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwOpenSemaphore(
        SemaphoreHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwOpenSession(
        SessionHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwOpenSymbolicLinkObject(
        LinkHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwOpenThread(
        ThreadHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        ClientId: *mut CLIENT_ID,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwOpenThreadToken(
        ThreadHandle: HANDLE,
        DesiredAccess: u32,
        OpenAsSelf: BOOLEAN,
        TokenHandle: *mut HANDLE,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwOpenThreadTokenEx(
        ThreadHandle: HANDLE,
        DesiredAccess: u32,
        OpenAsSelf: BOOLEAN,
        HandleAttributes: u32,
        TokenHandle: *mut HANDLE,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwOpenTimer(
        TimerHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwOpenTransaction(
        TransactionHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        Uow: *mut GUID,
        TmHandle: HANDLE,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwOpenTransactionManager(
        TmHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        LogFileName: *mut UNICODE_STRING,
        TmIdentity: *mut GUID,
        OpenOptions: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwPlugPlayControl(
        PnPControlClass: PLUGPLAY_CONTROL_CLASS,
        PnPControlData: *mut std::ffi::c_void,
        PnPControlDataLength: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwPowerInformation(
        InformationLevel: POWER_INFORMATION_LEVEL,
        InputBuffer: *mut std::ffi::c_void,
        InputBufferLength: u32,
        OutputBuffer: *mut std::ffi::c_void,
        OutputBufferLength: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwPrepareComplete(EnlistmentHandle: HANDLE, TmVirtualClock: *mut i64) -> NTSTATUS;
}
extern "C" {
    pub fn ZwPrepareEnlistment(EnlistmentHandle: HANDLE, TmVirtualClock: *mut i64) -> NTSTATUS;
}
extern "C" {
    pub fn ZwPrePrepareComplete(EnlistmentHandle: HANDLE, TmVirtualClock: *mut i64) -> NTSTATUS;
}
extern "C" {
    pub fn ZwPrePrepareEnlistment(EnlistmentHandle: HANDLE, TmVirtualClock: *mut i64) -> NTSTATUS;
}
extern "C" {
    pub fn ZwPrivilegeCheck(
        ClientToken: HANDLE,
        RequiredPrivileges: *mut PRIVILEGE_SET,
        Result: *mut BOOLEAN,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwPrivilegedServiceAuditAlarm(
        SubsystemName: *mut UNICODE_STRING,
        ServiceName: *mut UNICODE_STRING,
        ClientToken: HANDLE,
        Privileges: *mut PRIVILEGE_SET,
        AccessGranted: BOOLEAN,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwPrivilegeObjectAuditAlarm(
        SubsystemName: *mut UNICODE_STRING,
        HandleId: *mut std::ffi::c_void,
        ClientToken: HANDLE,
        DesiredAccess: u32,
        Privileges: *mut PRIVILEGE_SET,
        AccessGranted: BOOLEAN,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwPropagationComplete(
        ResourceManagerHandle: HANDLE,
        RequestCookie: u32,
        BufferLength: u32,
        Buffer: *mut std::ffi::c_void,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwPropagationFailed(
        ResourceManagerHandle: HANDLE,
        RequestCookie: u32,
        PropStatus: NTSTATUS,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwProtectVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut *mut std::ffi::c_void,
        RegionSize: *mut usize,
        NewProtect: u32,
        OldProtect: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwPulseEvent(EventHandle: HANDLE, PreviousState: *mut i32) -> NTSTATUS;
}
extern "C" {
    pub fn ZwQueryAttributesFile(
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        FileInformation: *mut FILE_BASIC_INFORMATION,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwQueryBootEntryOrder(Ids: *mut u32, Count: *mut u32) -> NTSTATUS;
}
extern "C" {
    pub fn ZwQueryBootOptions(
        BootOptions: *mut BOOT_OPTIONS,
        BootOptionsLength: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwQueryDebugFilterState(ComponentId: u32, Level: u32) -> NTSTATUS;
}
extern "C" {
    pub fn ZwQueryDefaultLocale(UserProfile: BOOLEAN, DefaultLocaleId: *mut u32) -> NTSTATUS;
}
extern "C" {
    pub fn ZwQueryDefaultUILanguage(DefaultUILanguageId: *mut u16) -> NTSTATUS;
}
extern "C" {
    pub fn ZwQueryDirectoryFile(
        FileHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: *mut std::ffi::c_void,
        IoStatusBlock: *mut IO_STATUS_BLOCK,
        FileInformation: *mut std::ffi::c_void,
        Length: u32,
        FileInformationClass: FILE_INFORMATION_CLASS,
        ReturnSingleEntry: BOOLEAN,
        FileName: *mut UNICODE_STRING,
        RestartScan: BOOLEAN,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwQueryDirectoryFileEx(
        FileHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: *mut std::ffi::c_void,
        IoStatusBlock: *mut IO_STATUS_BLOCK,
        FileInformation: *mut std::ffi::c_void,
        Length: u32,
        FileInformationClass: FILE_INFORMATION_CLASS,
        QueryFlags: u32,
        FileName: *mut UNICODE_STRING,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwQueryDirectoryObject(
        DirectoryHandle: HANDLE,
        Buffer: *mut std::ffi::c_void,
        Length: u32,
        ReturnSingleEntry: BOOLEAN,
        RestartScan: BOOLEAN,
        Context: *mut u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwQueryDriverEntryOrder(Ids: *mut u32, Count: *mut u32) -> NTSTATUS;
}
extern "C" {
    pub fn ZwQueryEaFile(
        FileHandle: HANDLE,
        IoStatusBlock: *mut IO_STATUS_BLOCK,
        Buffer: *mut std::ffi::c_void,
        Length: u32,
        ReturnSingleEntry: BOOLEAN,
        EaList: *mut std::ffi::c_void,
        EaListLength: u32,
        EaIndex: *mut u32,
        RestartScan: BOOLEAN,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwQueryEvent(
        EventHandle: HANDLE,
        EventInformationClass: EVENT_INFORMATION_CLASS,
        EventInformation: *mut std::ffi::c_void,
        EventInformationLength: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwQueryFullAttributesFile(
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        FileInformation: *mut FILE_NETWORK_OPEN_INFORMATION,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwQueryInformationAtom(
        Atom: u16,
        AtomInformationClass: ATOM_INFORMATION_CLASS,
        AtomInformation: *mut std::ffi::c_void,
        AtomInformationLength: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwQueryInformationByName(
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        IoStatusBlock: *mut IO_STATUS_BLOCK,
        FileInformation: *mut std::ffi::c_void,
        Length: u32,
        FileInformationClass: FILE_INFORMATION_CLASS,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwQueryInformationEnlistment(
        EnlistmentHandle: HANDLE,
        EnlistmentInformationClass: ENLISTMENT_INFORMATION_CLASS,
        EnlistmentInformation: *mut std::ffi::c_void,
        EnlistmentInformationLength: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwQueryInformationFile(
        FileHandle: HANDLE,
        IoStatusBlock: *mut IO_STATUS_BLOCK,
        FileInformation: *mut std::ffi::c_void,
        Length: u32,
        FileInformationClass: FILE_INFORMATION_CLASS,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwQueryInformationJobObject(
        JobHandle: HANDLE,
        JobObjectInformationClass: JOBOBJECTINFOCLASS,
        JobObjectInformation: *mut std::ffi::c_void,
        JobObjectInformationLength: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwQueryInformationPort(
        PortHandle: HANDLE,
        PortInformationClass: PORT_INFORMATION_CLASS,
        PortInformation: *mut std::ffi::c_void,
        Length: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwQueryInformationProcess(
        ProcessHandle: HANDLE,
        ProcessInformationClass: PROCESSINFOCLASS,
        ProcessInformation: *mut std::ffi::c_void,
        ProcessInformationLength: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwQueryInformationResourceManager(
        ResourceManagerHandle: HANDLE,
        ResourceManagerInformationClass: RESOURCEMANAGER_INFORMATION_CLASS,
        ResourceManagerInformation: *mut std::ffi::c_void,
        ResourceManagerInformationLength: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwQueryInformationThread(
        ThreadHandle: HANDLE,
        ThreadInformationClass: THREADINFOCLASS,
        ThreadInformation: *mut std::ffi::c_void,
        ThreadInformationLength: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwQueryInformationToken(
        TokenHandle: HANDLE,
        TokenInformationClass: TOKEN_INFORMATION_CLASS,
        TokenInformation: *mut std::ffi::c_void,
        TokenInformationLength: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwQueryInformationTransaction(
        TransactionHandle: HANDLE,
        TransactionInformationClass: TRANSACTION_INFORMATION_CLASS,
        TransactionInformation: *mut std::ffi::c_void,
        TransactionInformationLength: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwQueryInformationTransactionManager(
        TransactionManagerHandle: HANDLE,
        TransactionManagerInformationClass: TRANSACTIONMANAGER_INFORMATION_CLASS,
        TransactionManagerInformation: *mut std::ffi::c_void,
        TransactionManagerInformationLength: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwQueryInformationWorkerFactory(
        WorkerFactoryHandle: HANDLE,
        WorkerFactoryInformationClass: WORKERFACTORYINFOCLASS,
        WorkerFactoryInformation: *mut std::ffi::c_void,
        WorkerFactoryInformationLength: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwQueryInstallUILanguage(InstallUILanguageId: *mut u16) -> NTSTATUS;
}
extern "C" {
    pub fn ZwQueryIntervalProfile(ProfileSource: KPROFILE_SOURCE, Interval: *mut u32) -> NTSTATUS;
}
extern "C" {
    pub fn ZwQueryIoCompletion(
        IoCompletionHandle: HANDLE,
        IoCompletionInformationClass: IO_COMPLETION_INFORMATION_CLASS,
        IoCompletionInformation: *mut std::ffi::c_void,
        IoCompletionInformationLength: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwQueryKey(
        KeyHandle: HANDLE,
        KeyInformationClass: KEY_INFORMATION_CLASS,
        KeyInformation: *mut std::ffi::c_void,
        Length: u32,
        ResultLength: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwQueryLicenseValue(
        ValueName: *mut UNICODE_STRING,
        Type: *mut u32,
        Data: *mut std::ffi::c_void,
        DataSize: u32,
        ResultDataSize: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwQueryMultipleValueKey(
        KeyHandle: HANDLE,
        ValueEntries: *mut KEY_VALUE_ENTRY,
        EntryCount: u32,
        ValueBuffer: *mut std::ffi::c_void,
        BufferLength: *mut u32,
        RequiredBufferLength: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwQueryMutant(
        MutantHandle: HANDLE,
        MutantInformationClass: MUTANT_INFORMATION_CLASS,
        MutantInformation: *mut std::ffi::c_void,
        MutantInformationLength: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwQueryObject(
        Handle: HANDLE,
        ObjectInformationClass: OBJECT_INFORMATION_CLASS,
        ObjectInformation: *mut std::ffi::c_void,
        ObjectInformationLength: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwQueryOpenSubKeys(TargetKey: *mut OBJECT_ATTRIBUTES, HandleCount: *mut u32)
    -> NTSTATUS;
}
extern "C" {
    pub fn ZwQueryOpenSubKeysEx(
        TargetKey: *mut OBJECT_ATTRIBUTES,
        BufferLength: u32,
        Buffer: *mut std::ffi::c_void,
        RequiredSize: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwQueryPerformanceCounter(
        PerformanceCounter: *mut i64,
        PerformanceFrequency: *mut i64,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwQueryQuotaInformationFile(
        FileHandle: HANDLE,
        IoStatusBlock: *mut IO_STATUS_BLOCK,
        Buffer: *mut std::ffi::c_void,
        Length: u32,
        ReturnSingleEntry: BOOLEAN,
        SidList: *mut std::ffi::c_void,
        SidListLength: u32,
        StartSid: PSID,
        RestartScan: BOOLEAN,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwQuerySection(
        SectionHandle: HANDLE,
        SectionInformationClass: SECTION_INFORMATION_CLASS,
        SectionInformation: *mut std::ffi::c_void,
        SectionInformationLength: usize,
        ReturnLength: *mut usize,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwQuerySecurityAttributesToken(
        TokenHandle: HANDLE,
        Attributes: *mut UNICODE_STRING,
        NumberOfAttributes: u32,
        Buffer: *mut std::ffi::c_void,
        Length: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwQuerySecurityObject(
        Handle: HANDLE,
        SecurityInformation: u32,
        SecurityDescriptor: *mut SECURITY_DESCRIPTOR,
        Length: u32,
        LengthNeeded: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwQuerySemaphore(
        SemaphoreHandle: HANDLE,
        SemaphoreInformationClass: SEMAPHORE_INFORMATION_CLASS,
        SemaphoreInformation: *mut std::ffi::c_void,
        SemaphoreInformationLength: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwQuerySymbolicLinkObject(
        LinkHandle: HANDLE,
        LinkTarget: *mut UNICODE_STRING,
        ReturnedLength: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwQuerySystemEnvironmentValue(
        VariableName: *mut UNICODE_STRING,
        VariableValue: PWSTR,
        ValueLength: u16,
        ReturnLength: *mut u16,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwQuerySystemEnvironmentValueEx(
        VariableName: *mut UNICODE_STRING,
        VendorGuid: *mut GUID,
        Value: *mut std::ffi::c_void,
        ValueLength: *mut u32,
        Attributes: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwQuerySystemInformation(
        SystemInformationClass: SYSTEM_INFORMATION_CLASS,
        SystemInformation: *mut std::ffi::c_void,
        SystemInformationLength: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwQuerySystemInformationEx(
        SystemInformationClass: SYSTEM_INFORMATION_CLASS,
        InputBuffer: *mut std::ffi::c_void,
        InputBufferLength: u32,
        SystemInformation: *mut std::ffi::c_void,
        SystemInformationLength: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwQuerySystemTime(SystemTime: *mut i64) -> NTSTATUS;
}
extern "C" {
    pub fn ZwQueryTimer(
        TimerHandle: HANDLE,
        TimerInformationClass: TIMER_INFORMATION_CLASS,
        TimerInformation: *mut std::ffi::c_void,
        TimerInformationLength: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwQueryTimerResolution(
        MaximumTime: *mut u32,
        MinimumTime: *mut u32,
        CurrentTime: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwQueryValueKey(
        KeyHandle: HANDLE,
        ValueName: *mut UNICODE_STRING,
        KeyValueInformationClass: KEY_VALUE_INFORMATION_CLASS,
        KeyValueInformation: *mut std::ffi::c_void,
        Length: u32,
        ResultLength: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwQueryVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut std::ffi::c_void,
        MemoryInformationClass: MEMORY_INFORMATION_CLASS,
        MemoryInformation: *mut std::ffi::c_void,
        MemoryInformationLength: usize,
        ReturnLength: *mut usize,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwQueryVolumeInformationFile(
        FileHandle: HANDLE,
        IoStatusBlock: *mut IO_STATUS_BLOCK,
        FsInformation: *mut std::ffi::c_void,
        Length: u32,
        FsInformationClass: FSINFOCLASS,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwQueryWnfStateData(
        StateName: *const WNF_STATE_NAME,
        TypeId: *const WNF_TYPE_ID,
        ExplicitScope: *const std::os::raw::c_void,
        ChangeStamp: *mut u32,
        Buffer: *mut std::ffi::c_void,
        BufferSize: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwQueryWnfStateNameInformation(
        StateName: *const WNF_STATE_NAME,
        NameInfoClass: WNF_STATE_NAME_INFORMATION,
        ExplicitScope: *const std::os::raw::c_void,
        InfoBuffer: *mut std::ffi::c_void,
        InfoBufferSize: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwQueueApcThread(
        ThreadHandle: HANDLE,
        ApcRoutine: PPS_APC_ROUTINE,
        ApcArgument1: *mut std::ffi::c_void,
        ApcArgument2: *mut std::ffi::c_void,
        ApcArgument3: *mut std::ffi::c_void,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwQueueApcThreadEx(
        ThreadHandle: HANDLE,
        ReserveHandle: HANDLE,
        ApcRoutine: PPS_APC_ROUTINE,
        ApcArgument1: *mut std::ffi::c_void,
        ApcArgument2: *mut std::ffi::c_void,
        ApcArgument3: *mut std::ffi::c_void,
    ) -> NTSTATUS;
}
extern "C" {
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
extern "C" {
    pub fn ZwRaiseException(
        ExceptionRecord: *mut EXCEPTION_RECORD,
        ContextRecord: *mut CONTEXT,
        FirstChance: BOOLEAN,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwRaiseHardError(
        ErrorStatus: NTSTATUS,
        NumberOfParameters: u32,
        UnicodeStringParameterMask: u32,
        Parameters: *mut usize,
        ValidResponseOptions: u32,
        Response: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwReadFile(
        FileHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: *mut std::ffi::c_void,
        IoStatusBlock: *mut IO_STATUS_BLOCK,
        Buffer: *mut std::ffi::c_void,
        Length: u32,
        ByteOffset: *mut i64,
        Key: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
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
extern "C" {
    pub fn ZwReadOnlyEnlistment(EnlistmentHandle: HANDLE, TmVirtualClock: *mut i64) -> NTSTATUS;
}
extern "C" {
    pub fn ZwReadRequestData(
        PortHandle: HANDLE,
        Message: *mut PORT_MESSAGE,
        DataEntryIndex: u32,
        Buffer: *mut std::ffi::c_void,
        BufferSize: usize,
        NumberOfBytesRead: *mut usize,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwReadVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut std::ffi::c_void,
        Buffer: *mut std::ffi::c_void,
        BufferSize: usize,
        NumberOfBytesRead: *mut usize,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwRecoverEnlistment(
        EnlistmentHandle: HANDLE,
        EnlistmentKey: *mut std::ffi::c_void,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwRecoverResourceManager(ResourceManagerHandle: HANDLE) -> NTSTATUS;
}
extern "C" {
    pub fn ZwRecoverTransactionManager(TransactionManagerHandle: HANDLE) -> NTSTATUS;
}
extern "C" {
    pub fn ZwRegisterProtocolAddressInformation(
        ResourceManager: HANDLE,
        ProtocolId: *mut GUID,
        ProtocolInformationSize: u32,
        ProtocolInformation: *mut std::ffi::c_void,
        CreateOptions: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwRegisterThreadTerminatePort(PortHandle: HANDLE) -> NTSTATUS;
}
extern "C" {
    pub fn ZwReleaseCMFViewOwnership() -> NTSTATUS;
}
extern "C" {
    pub fn ZwReleaseKeyedEvent(
        KeyedEventHandle: HANDLE,
        KeyValue: *mut std::ffi::c_void,
        Alertable: BOOLEAN,
        Timeout: *mut i64,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwReleaseMutant(MutantHandle: HANDLE, PreviousCount: *mut i32) -> NTSTATUS;
}
extern "C" {
    pub fn ZwReleaseSemaphore(
        SemaphoreHandle: HANDLE,
        ReleaseCount: i32,
        PreviousCount: *mut i32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwReleaseWorkerFactoryWorker(WorkerFactoryHandle: HANDLE) -> NTSTATUS;
}
extern "C" {
    pub fn ZwRemoveIoCompletion(
        IoCompletionHandle: HANDLE,
        KeyContext: *mut *mut std::ffi::c_void,
        ApcContext: *mut *mut std::ffi::c_void,
        IoStatusBlock: *mut IO_STATUS_BLOCK,
        Timeout: *mut i64,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwRemoveIoCompletionEx(
        IoCompletionHandle: HANDLE,
        IoCompletionInformation: *mut FILE_IO_COMPLETION_INFORMATION,
        Count: u32,
        NumEntriesRemoved: *mut u32,
        Timeout: *mut i64,
        Alertable: BOOLEAN,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwRemoveProcessDebug(ProcessHandle: HANDLE, DebugObjectHandle: HANDLE) -> NTSTATUS;
}
extern "C" {
    pub fn ZwRenameKey(KeyHandle: HANDLE, NewName: *mut UNICODE_STRING) -> NTSTATUS;
}
extern "C" {
    pub fn ZwRenameTransactionManager(
        LogFileName: *mut UNICODE_STRING,
        ExistingTransactionManagerGuid: *mut GUID,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwReplaceKey(
        NewFile: *mut OBJECT_ATTRIBUTES,
        TargetHandle: HANDLE,
        OldFile: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwReplacePartitionUnit(
        TargetInstancePath: *mut UNICODE_STRING,
        SpareInstancePath: *mut UNICODE_STRING,
        Flags: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwReplyPort(PortHandle: HANDLE, ReplyMessage: *mut PORT_MESSAGE) -> NTSTATUS;
}
extern "C" {
    pub fn ZwReplyWaitReceivePort(
        PortHandle: HANDLE,
        PortContext: *mut *mut std::ffi::c_void,
        ReplyMessage: *mut PORT_MESSAGE,
        ReceiveMessage: *mut PORT_MESSAGE,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwReplyWaitReceivePortEx(
        PortHandle: HANDLE,
        PortContext: *mut *mut std::ffi::c_void,
        ReplyMessage: *mut PORT_MESSAGE,
        ReceiveMessage: *mut PORT_MESSAGE,
        Timeout: *mut i64,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwReplyWaitReplyPort(PortHandle: HANDLE, ReplyMessage: *mut PORT_MESSAGE) -> NTSTATUS;
}
extern "C" {
    pub fn ZwRequestPort(PortHandle: HANDLE, RequestMessage: *mut PORT_MESSAGE) -> NTSTATUS;
}
extern "C" {
    pub fn ZwRequestWaitReplyPort(
        PortHandle: HANDLE,
        RequestMessage: *mut PORT_MESSAGE,
        ReplyMessage: *mut PORT_MESSAGE,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwRequestWakeupLatency(latency: LATENCY_TIME) -> NTSTATUS;
}
extern "C" {
    pub fn ZwResetEvent(EventHandle: HANDLE, PreviousState: *mut i32) -> NTSTATUS;
}
extern "C" {
    pub fn ZwResetWriteWatch(
        ProcessHandle: HANDLE,
        BaseAddress: *mut std::ffi::c_void,
        RegionSize: usize,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwRestoreKey(KeyHandle: HANDLE, FileHandle: HANDLE, Flags: u32) -> NTSTATUS;
}
extern "C" {
    pub fn ZwResumeProcess(ProcessHandle: HANDLE) -> NTSTATUS;
}
extern "C" {
    pub fn ZwResumeThread(ThreadHandle: HANDLE, PreviousSuspendCount: *mut u32) -> NTSTATUS;
}
extern "C" {
    pub fn ZwRevertContainerImpersonation() -> NTSTATUS;
}
extern "C" {
    pub fn ZwRollbackComplete(EnlistmentHandle: HANDLE, TmVirtualClock: *mut i64) -> NTSTATUS;
}
extern "C" {
    pub fn ZwRollbackEnlistment(EnlistmentHandle: HANDLE, TmVirtualClock: *mut i64) -> NTSTATUS;
}
extern "C" {
    pub fn ZwRollbackTransaction(TransactionHandle: HANDLE, Wait: BOOLEAN) -> NTSTATUS;
}
extern "C" {
    pub fn ZwRollforwardTransactionManager(
        TransactionManagerHandle: HANDLE,
        TmVirtualClock: *mut i64,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSaveKey(KeyHandle: HANDLE, FileHandle: HANDLE) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSaveKeyEx(KeyHandle: HANDLE, FileHandle: HANDLE, Format: u32) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSaveMergedKeys(
        HighPrecedenceKeyHandle: HANDLE,
        LowPrecedenceKeyHandle: HANDLE,
        FileHandle: HANDLE,
    ) -> NTSTATUS;
}
extern "C" {
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
extern "C" {
    pub fn ZwSerializeBoot() -> NTSTATUS;
}
extern "C" {
    pub fn ZwSetBootEntryOrder(Ids: *mut u32, Count: u32) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSetBootOptions(BootOptions: *mut BOOT_OPTIONS, FieldsToChange: u32) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSetCachedSigningLevel(
        Flags: u32,
        InputSigningLevel: u8,
        SourceFiles: *mut HANDLE,
        SourceFileCount: u32,
        TargetFile: HANDLE,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSetContextThread(ThreadHandle: HANDLE, ThreadContext: *mut CONTEXT) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSetDebugFilterState(ComponentId: u32, Level: u32, State: BOOLEAN) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSetDefaultHardErrorPort(DefaultHardErrorPort: HANDLE) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSetDefaultLocale(UserProfile: BOOLEAN, DefaultLocaleId: u32) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSetDefaultUILanguage(DefaultUILanguageId: u16) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSetDriverEntryOrder(Ids: *mut u32, Count: u32) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSetEaFile(
        FileHandle: HANDLE,
        IoStatusBlock: *mut IO_STATUS_BLOCK,
        Buffer: *mut std::ffi::c_void,
        Length: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSetEvent(EventHandle: HANDLE, PreviousState: *mut i32) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSetEventBoostPriority(EventHandle: HANDLE) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSetHighEventPair(EventPairHandle: HANDLE) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSetHighWaitLowEventPair(EventPairHandle: HANDLE) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSetInformationDebugObject(
        DebugObjectHandle: HANDLE,
        DebugObjectInformationClass: DEBUGOBJECTINFOCLASS,
        DebugInformation: *mut std::ffi::c_void,
        DebugInformationLength: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSetInformationEnlistment(
        EnlistmentHandle: HANDLE,
        EnlistmentInformationClass: ENLISTMENT_INFORMATION_CLASS,
        EnlistmentInformation: *mut std::ffi::c_void,
        EnlistmentInformationLength: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSetInformationFile(
        FileHandle: HANDLE,
        IoStatusBlock: *mut IO_STATUS_BLOCK,
        FileInformation: *mut std::ffi::c_void,
        Length: u32,
        FileInformationClass: FILE_INFORMATION_CLASS,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSetInformationJobObject(
        JobHandle: HANDLE,
        JobObjectInformationClass: JOBOBJECTINFOCLASS,
        JobObjectInformation: *mut std::ffi::c_void,
        JobObjectInformationLength: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSetInformationKey(
        KeyHandle: HANDLE,
        KeySetInformationClass: KEY_SET_INFORMATION_CLASS,
        KeySetInformation: *mut std::ffi::c_void,
        KeySetInformationLength: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSetInformationObject(
        Handle: HANDLE,
        ObjectInformationClass: OBJECT_INFORMATION_CLASS,
        ObjectInformation: *mut std::ffi::c_void,
        ObjectInformationLength: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSetInformationProcess(
        ProcessHandle: HANDLE,
        ProcessInformationClass: PROCESSINFOCLASS,
        ProcessInformation: *mut std::ffi::c_void,
        ProcessInformationLength: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSetInformationResourceManager(
        ResourceManagerHandle: HANDLE,
        ResourceManagerInformationClass: RESOURCEMANAGER_INFORMATION_CLASS,
        ResourceManagerInformation: *mut std::ffi::c_void,
        ResourceManagerInformationLength: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSetInformationSymbolicLink(
        LinkHandle: HANDLE,
        SymbolicLinkInformationClass: SYMBOLIC_LINK_INFO_CLASS,
        SymbolicLinkInformation: *mut std::ffi::c_void,
        SymbolicLinkInformationLength: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSetInformationThread(
        ThreadHandle: HANDLE,
        ThreadInformationClass: THREADINFOCLASS,
        ThreadInformation: *mut std::ffi::c_void,
        ThreadInformationLength: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSetInformationToken(
        TokenHandle: HANDLE,
        TokenInformationClass: TOKEN_INFORMATION_CLASS,
        TokenInformation: *mut std::ffi::c_void,
        TokenInformationLength: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSetInformationTransaction(
        TransactionHandle: HANDLE,
        TransactionInformationClass: TRANSACTION_INFORMATION_CLASS,
        TransactionInformation: *mut std::ffi::c_void,
        TransactionInformationLength: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSetInformationTransactionManager(
        TmHandle: HANDLE,
        TransactionManagerInformationClass: TRANSACTIONMANAGER_INFORMATION_CLASS,
        TransactionManagerInformation: *mut std::ffi::c_void,
        TransactionManagerInformationLength: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSetInformationVirtualMemory(
        ProcessHandle: HANDLE,
        VmInformationClass: VIRTUAL_MEMORY_INFORMATION_CLASS,
        NumberOfEntries: usize,
        VirtualAddresses: *mut MEMORY_RANGE_ENTRY,
        VmInformation: *mut std::ffi::c_void,
        VmInformationLength: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSetInformationWorkerFactory(
        WorkerFactoryHandle: HANDLE,
        WorkerFactoryInformationClass: WORKERFACTORYINFOCLASS,
        WorkerFactoryInformation: *mut std::ffi::c_void,
        WorkerFactoryInformationLength: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSetIntervalProfile(Interval: u32, Source: KPROFILE_SOURCE) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSetIoCompletion(
        IoCompletionHandle: HANDLE,
        KeyContext: *mut std::ffi::c_void,
        ApcContext: *mut std::ffi::c_void,
        IoStatus: NTSTATUS,
        IoStatusInformation: usize,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSetIoCompletionEx(
        IoCompletionHandle: HANDLE,
        IoCompletionPacketHandle: HANDLE,
        KeyContext: *mut std::ffi::c_void,
        ApcContext: *mut std::ffi::c_void,
        IoStatus: NTSTATUS,
        IoStatusInformation: usize,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSetIRTimer(TimerHandle: HANDLE, DueTime: *mut i64) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSetLdtEntries(
        Selector0: u32,
        Entry0Low: u32,
        Entry0Hi: u32,
        Selector1: u32,
        Entry1Low: u32,
        Entry1Hi: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSetLowEventPair(EventPairHandle: HANDLE) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSetLowWaitHighEventPair(EventPairHandle: HANDLE) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSetQuotaInformationFile(
        FileHandle: HANDLE,
        IoStatusBlock: *mut IO_STATUS_BLOCK,
        Buffer: *mut std::ffi::c_void,
        Length: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSetSecurityObject(
        Handle: HANDLE,
        SecurityInformation: u32,
        SecurityDescriptor: *mut SECURITY_DESCRIPTOR,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSetSystemEnvironmentValue(
        VariableName: *mut UNICODE_STRING,
        VariableValue: *mut UNICODE_STRING,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSetSystemEnvironmentValueEx(
        VariableName: *mut UNICODE_STRING,
        VendorGuid: *mut GUID,
        Value: *mut std::ffi::c_void,
        ValueLength: u32,
        Attributes: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSetSystemInformation(
        SystemInformationClass: SYSTEM_INFORMATION_CLASS,
        SystemInformation: *mut std::ffi::c_void,
        SystemInformationLength: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSetSystemPowerState(
        SystemAction: POWER_ACTION,
        LightestSystemState: SYSTEM_POWER_STATE,
        Flags: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSetSystemTime(SystemTime: *mut i64, PreviousTime: *mut i64) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSetThreadExecutionState(
        NewFlags: EXECUTION_STATE,
        PreviousFlags: *mut EXECUTION_STATE,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSetTimer(
        TimerHandle: HANDLE,
        DueTime: *mut i64,
        TimerApcRoutine: PTIMER_APC_ROUTINE,
        TimerContext: *mut std::ffi::c_void,
        ResumeTimer: BOOLEAN,
        Period: i32,
        PreviousState: *mut BOOLEAN,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSetTimer2(
        TimerHandle: HANDLE,
        DueTime: *mut i64,
        Period: *mut i64,
        Parameters: *mut T2_SET_PARAMETERS,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSetTimerEx(
        TimerHandle: HANDLE,
        TimerSetInformationClass: TIMER_SET_INFORMATION_CLASS,
        TimerSetInformation: *mut std::ffi::c_void,
        TimerSetInformationLength: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSetTimerResolution(
        DesiredTime: u32,
        SetResolution: BOOLEAN,
        ActualTime: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSetUuidSeed(Seed: *mut i8) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSetValueKey(
        KeyHandle: HANDLE,
        ValueName: *mut UNICODE_STRING,
        TitleIndex: u32,
        Type: u32,
        Data: *mut std::ffi::c_void,
        DataSize: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSetVolumeInformationFile(
        FileHandle: HANDLE,
        IoStatusBlock: *mut IO_STATUS_BLOCK,
        FsInformation: *mut std::ffi::c_void,
        Length: u32,
        FsInformationClass: FSINFOCLASS,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSetWnfProcessNotificationEvent(NotificationEvent: HANDLE) -> NTSTATUS;
}
extern "C" {
    pub fn ZwShutdownSystem(Action: SHUTDOWN_ACTION) -> NTSTATUS;
}
extern "C" {
    pub fn ZwShutdownWorkerFactory(
        WorkerFactoryHandle: HANDLE,
        PendingWorkerCount: *mut i32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSignalAndWaitForSingleObject(
        SignalHandle: HANDLE,
        WaitHandle: HANDLE,
        Alertable: BOOLEAN,
        Timeout: *mut i64,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSinglePhaseReject(EnlistmentHandle: HANDLE, TmVirtualClock: *mut i64) -> NTSTATUS;
}
extern "C" {
    pub fn ZwStartProfile(ProfileHandle: HANDLE) -> NTSTATUS;
}
extern "C" {
    pub fn ZwStopProfile(ProfileHandle: HANDLE) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSubscribeWnfStateChange(
        StateName: *const WNF_STATE_NAME,
        ChangeStamp: u32,
        EventMask: u32,
        SubscriptionId: *mut u64,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSuspendProcess(ProcessHandle: HANDLE) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSuspendThread(ThreadHandle: HANDLE, PreviousSuspendCount: *mut u32) -> NTSTATUS;
}
extern "C" {
    pub fn ZwSystemDebugControl(
        Command: SYSDBG_COMMAND,
        InputBuffer: *mut std::ffi::c_void,
        InputBufferLength: u32,
        OutputBuffer: *mut std::ffi::c_void,
        OutputBufferLength: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwTerminateEnclave(
        BaseAddress: *mut std::ffi::c_void,
        WaitForThread: BOOLEAN,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwTerminateJobObject(JobHandle: HANDLE, ExitStatus: NTSTATUS) -> NTSTATUS;
}
extern "C" {
    pub fn ZwTerminateProcess(ProcessHandle: HANDLE, ExitStatus: NTSTATUS) -> NTSTATUS;
}
extern "C" {
    pub fn ZwTerminateThread(ThreadHandle: HANDLE, ExitStatus: NTSTATUS) -> NTSTATUS;
}
extern "C" {
    pub fn ZwTestAlert() -> NTSTATUS;
}
extern "C" {
    pub fn ZwThawRegistry() -> NTSTATUS;
}
extern "C" {
    pub fn ZwThawTransactions() -> NTSTATUS;
}
extern "C" {
    pub fn ZwTraceControl(
        TraceInformationClass: TRACE_CONTROL_INFORMATION_CLASS,
        InputBuffer: *mut std::ffi::c_void,
        InputBufferLength: u32,
        TraceInformation: *mut std::ffi::c_void,
        TraceInformationLength: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwTraceEvent(
        TraceHandle: HANDLE,
        Flags: u32,
        FieldSize: u32,
        Fields: *mut std::ffi::c_void,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwTranslateFilePath(
        InputFilePath: *mut FILE_PATH,
        OutputType: u32,
        OutputFilePath: *mut FILE_PATH,
        OutputFilePathLength: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwUmsThreadYield(SchedulerParam: *mut std::ffi::c_void) -> NTSTATUS;
}
extern "C" {
    pub fn ZwUnloadDriver(DriverServiceName: *mut UNICODE_STRING) -> NTSTATUS;
}
extern "C" {
    pub fn ZwUnloadKey(TargetKey: *mut OBJECT_ATTRIBUTES) -> NTSTATUS;
}
extern "C" {
    pub fn ZwUnloadKey2(TargetKey: *mut OBJECT_ATTRIBUTES, Flags: u32) -> NTSTATUS;
}
extern "C" {
    pub fn ZwUnloadKeyEx(TargetKey: *mut OBJECT_ATTRIBUTES, Event: HANDLE) -> NTSTATUS;
}
extern "C" {
    pub fn ZwUnlockFile(
        FileHandle: HANDLE,
        IoStatusBlock: *mut IO_STATUS_BLOCK,
        ByteOffset: *mut i64,
        Length: *mut i64,
        Key: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwUnlockVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut *mut std::ffi::c_void,
        RegionSize: *mut usize,
        MapType: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwUnmapViewOfSection(
        ProcessHandle: HANDLE,
        BaseAddress: *mut std::ffi::c_void,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwUnmapViewOfSectionEx(
        ProcessHandle: HANDLE,
        BaseAddress: *mut std::ffi::c_void,
        Flags: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwUnsubscribeWnfStateChange(StateName: *const WNF_STATE_NAME) -> NTSTATUS;
}
extern "C" {
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
extern "C" {
    pub fn ZwVdmControl(Service: VDMSERVICECLASS, ServiceData: *mut std::ffi::c_void) -> NTSTATUS;
}
extern "C" {
    pub fn ZwWaitForAlertByThreadId(Address: *mut std::ffi::c_void, Timeout: *mut i64) -> NTSTATUS;
}
extern "C" {
    pub fn ZwWaitForDebugEvent(
        DebugObjectHandle: HANDLE,
        Alertable: BOOLEAN,
        Timeout: *mut i64,
        WaitStateChange: *mut DBGUI_WAIT_STATE_CHANGE,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwWaitForKeyedEvent(
        KeyedEventHandle: HANDLE,
        KeyValue: *mut std::ffi::c_void,
        Alertable: BOOLEAN,
        Timeout: *mut i64,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwWaitForMultipleObjects(
        Count: u32,
        Handles: *mut HANDLE,
        WaitType: WAIT_TYPE,
        Alertable: BOOLEAN,
        Timeout: *mut i64,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwWaitForMultipleObjects32(
        Count: u32,
        Handles: *mut i32,
        WaitType: WAIT_TYPE,
        Alertable: BOOLEAN,
        Timeout: *mut i64,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwWaitForSingleObject(Handle: HANDLE, Alertable: BOOLEAN, Timeout: *mut i64)
    -> NTSTATUS;
}
extern "C" {
    pub fn ZwWaitForWorkViaWorkerFactory(
        WorkerFactoryHandle: HANDLE,
        MiniPackets: *mut FILE_IO_COMPLETION_INFORMATION,
        Count: u32,
        PacketsReturned: *mut u32,
        DeferredWork: *mut WORKER_FACTORY_DEFERRED_WORK,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwWaitHighEventPair(EventPairHandle: HANDLE) -> NTSTATUS;
}
extern "C" {
    pub fn ZwWaitLowEventPair(EventPairHandle: HANDLE) -> NTSTATUS;
}
extern "C" {
    pub fn ZwWorkerFactoryWorkerReady(WorkerFactoryHandle: HANDLE) -> NTSTATUS;
}
extern "C" {
    pub fn ZwWriteFile(
        FileHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: *mut std::ffi::c_void,
        IoStatusBlock: *mut IO_STATUS_BLOCK,
        Buffer: *mut std::ffi::c_void,
        Length: u32,
        ByteOffset: *mut i64,
        Key: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
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
extern "C" {
    pub fn ZwWriteRequestData(
        PortHandle: HANDLE,
        Message: *mut PORT_MESSAGE,
        DataEntryIndex: u32,
        Buffer: *mut std::ffi::c_void,
        BufferSize: usize,
        NumberOfBytesWritten: *mut usize,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwWriteVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut std::ffi::c_void,
        Buffer: *mut std::ffi::c_void,
        BufferSize: usize,
        NumberOfBytesWritten: *mut usize,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn ZwYieldExecution() -> NTSTATUS;
}
