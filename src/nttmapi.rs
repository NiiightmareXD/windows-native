use windows::{
    core::GUID,
    Wdk::Foundation::OBJECT_ATTRIBUTES,
    Win32::{
        Foundation::{BOOLEAN, HANDLE, NTSTATUS, UNICODE_STRING},
        Storage::FileSystem::TRANSACTION_NOTIFICATION,
        System::SystemServices::{
            ENLISTMENT_INFORMATION_CLASS, KTMOBJECT_CURSOR, KTMOBJECT_TYPE,
            RESOURCEMANAGER_INFORMATION_CLASS, TRANSACTIONMANAGER_INFORMATION_CLASS,
            TRANSACTION_INFORMATION_CLASS,
        },
    },
};

extern "C" {
    pub fn NtCreateTransactionManager(
        TmHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        LogFileName: *mut UNICODE_STRING,
        CreateOptions: u32,
        CommitStrength: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn NtOpenTransactionManager(
        TmHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        LogFileName: *mut UNICODE_STRING,
        TmIdentity: *mut GUID,
        OpenOptions: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn NtRenameTransactionManager(
        LogFileName: *mut UNICODE_STRING,
        ExistingTransactionManagerGuid: *mut GUID,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn NtRollforwardTransactionManager(
        TransactionManagerHandle: HANDLE,
        TmVirtualClock: *mut i64,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn NtRecoverTransactionManager(TransactionManagerHandle: HANDLE) -> NTSTATUS;
}
extern "C" {
    pub fn NtQueryInformationTransactionManager(
        TransactionManagerHandle: HANDLE,
        TransactionManagerInformationClass: TRANSACTIONMANAGER_INFORMATION_CLASS,
        TransactionManagerInformation: *mut std::ffi::c_void,
        TransactionManagerInformationLength: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn NtSetInformationTransactionManager(
        TmHandle: HANDLE,
        TransactionManagerInformationClass: TRANSACTIONMANAGER_INFORMATION_CLASS,
        TransactionManagerInformation: *mut std::ffi::c_void,
        TransactionManagerInformationLength: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn NtEnumerateTransactionObject(
        RootObjectHandle: HANDLE,
        QueryType: KTMOBJECT_TYPE,
        ObjectCursor: *mut KTMOBJECT_CURSOR,
        ObjectCursorLength: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn NtCreateTransaction(
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
    pub fn NtOpenTransaction(
        TransactionHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        Uow: *mut GUID,
        TmHandle: HANDLE,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn NtQueryInformationTransaction(
        TransactionHandle: HANDLE,
        TransactionInformationClass: TRANSACTION_INFORMATION_CLASS,
        TransactionInformation: *mut std::ffi::c_void,
        TransactionInformationLength: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn NtSetInformationTransaction(
        TransactionHandle: HANDLE,
        TransactionInformationClass: TRANSACTION_INFORMATION_CLASS,
        TransactionInformation: *mut std::ffi::c_void,
        TransactionInformationLength: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn NtCommitTransaction(TransactionHandle: HANDLE, Wait: BOOLEAN) -> NTSTATUS;
}
extern "C" {
    pub fn NtRollbackTransaction(TransactionHandle: HANDLE, Wait: BOOLEAN) -> NTSTATUS;
}
extern "C" {
    pub fn NtCreateEnlistment(
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
    pub fn NtOpenEnlistment(
        EnlistmentHandle: *mut HANDLE,
        DesiredAccess: u32,
        ResourceManagerHandle: HANDLE,
        EnlistmentGuid: *mut GUID,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn NtQueryInformationEnlistment(
        EnlistmentHandle: HANDLE,
        EnlistmentInformationClass: ENLISTMENT_INFORMATION_CLASS,
        EnlistmentInformation: *mut std::ffi::c_void,
        EnlistmentInformationLength: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn NtSetInformationEnlistment(
        EnlistmentHandle: HANDLE,
        EnlistmentInformationClass: ENLISTMENT_INFORMATION_CLASS,
        EnlistmentInformation: *mut std::ffi::c_void,
        EnlistmentInformationLength: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn NtRecoverEnlistment(
        EnlistmentHandle: HANDLE,
        EnlistmentKey: *mut std::ffi::c_void,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn NtPrePrepareEnlistment(EnlistmentHandle: HANDLE, TmVirtualClock: *mut i64) -> NTSTATUS;
}
extern "C" {
    pub fn NtPrepareEnlistment(EnlistmentHandle: HANDLE, TmVirtualClock: *mut i64) -> NTSTATUS;
}
extern "C" {
    pub fn NtCommitEnlistment(EnlistmentHandle: HANDLE, TmVirtualClock: *mut i64) -> NTSTATUS;
}
extern "C" {
    pub fn NtRollbackEnlistment(EnlistmentHandle: HANDLE, TmVirtualClock: *mut i64) -> NTSTATUS;
}
extern "C" {
    pub fn NtPrePrepareComplete(EnlistmentHandle: HANDLE, TmVirtualClock: *mut i64) -> NTSTATUS;
}
extern "C" {
    pub fn NtPrepareComplete(EnlistmentHandle: HANDLE, TmVirtualClock: *mut i64) -> NTSTATUS;
}
extern "C" {
    pub fn NtCommitComplete(EnlistmentHandle: HANDLE, TmVirtualClock: *mut i64) -> NTSTATUS;
}
extern "C" {
    pub fn NtReadOnlyEnlistment(EnlistmentHandle: HANDLE, TmVirtualClock: *mut i64) -> NTSTATUS;
}
extern "C" {
    pub fn NtRollbackComplete(EnlistmentHandle: HANDLE, TmVirtualClock: *mut i64) -> NTSTATUS;
}
extern "C" {
    pub fn NtSinglePhaseReject(EnlistmentHandle: HANDLE, TmVirtualClock: *mut i64) -> NTSTATUS;
}
extern "C" {
    pub fn NtCreateResourceManager(
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
    pub fn NtOpenResourceManager(
        ResourceManagerHandle: *mut HANDLE,
        DesiredAccess: u32,
        TmHandle: HANDLE,
        ResourceManagerGuid: *mut GUID,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn NtRecoverResourceManager(ResourceManagerHandle: HANDLE) -> NTSTATUS;
}
extern "C" {
    pub fn NtGetNotificationResourceManager(
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
    pub fn NtQueryInformationResourceManager(
        ResourceManagerHandle: HANDLE,
        ResourceManagerInformationClass: RESOURCEMANAGER_INFORMATION_CLASS,
        ResourceManagerInformation: *mut std::ffi::c_void,
        ResourceManagerInformationLength: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn NtSetInformationResourceManager(
        ResourceManagerHandle: HANDLE,
        ResourceManagerInformationClass: RESOURCEMANAGER_INFORMATION_CLASS,
        ResourceManagerInformation: *mut std::ffi::c_void,
        ResourceManagerInformationLength: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn NtRegisterProtocolAddressInformation(
        ResourceManager: HANDLE,
        ProtocolId: *mut GUID,
        ProtocolInformationSize: u32,
        ProtocolInformation: *mut std::ffi::c_void,
        CreateOptions: u32,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn NtPropagationComplete(
        ResourceManagerHandle: HANDLE,
        RequestCookie: u32,
        BufferLength: u32,
        Buffer: *mut std::ffi::c_void,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn NtPropagationFailed(
        ResourceManagerHandle: HANDLE,
        RequestCookie: u32,
        PropStatus: NTSTATUS,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn NtFreezeTransactions(FreezeTimeout: *mut i64, ThawTimeout: *mut i64) -> NTSTATUS;
}
extern "C" {
    pub fn NtThawTransactions() -> NTSTATUS;
}
