use windows::{
    Wdk::{
        Foundation::OBJECT_ATTRIBUTES,
        Storage::FileSystem::{FILE_BASIC_INFORMATION, FILE_NETWORK_OPEN_INFORMATION},
        System::SystemServices::{
            DIRECTORY_NOTIFY_INFORMATION_CLASS, IO_SESSION_EVENT, IO_SESSION_STATE,
        },
    },
    Win32::{
        Foundation::{HANDLE, NTSTATUS, UNICODE_STRING},
        Security::SID,
        Storage::FileSystem::FILE_SEGMENT_ELEMENT,
        System::IO::{IO_STATUS_BLOCK, PIO_APC_ROUTINE},
    },
};

use crate::bitfield::{BitfieldUnit, UnionField};

pub const DOS_MAX_COMPONENT_LENGTH: u32 = 255;
pub const DOS_MAX_PATH_LENGTH: u32 = 260;
pub const FILE_PIPE_UNLIMITED_INSTANCES: u32 = 4294967295;
pub const MAILSLOT_SIZE_AUTO: u32 = 0;
pub const CHECKSUM_ENFORCEMENT_OFF: u32 = 1;
pub const IO_COMPLETION_QUERY_STATE: u32 = 1;
pub const IO_COMPLETION_ALL_ACCESS: u32 = 2031619;
pub const DEVICE_NAMED_PIPE: &[u8; 19] = b"\\Device\\NamedPipe\\\0";
pub const FILE_PIPE_SYMLINK_VALID_FLAGS: u32 = 3;
pub const MAILSLOT_CLASS_FIRSTCLASS: u32 = 1;
pub const MAILSLOT_CLASS_SECONDCLASS: u32 = 2;
pub const MOUNTMGR_DEVICE_NAME: &[u8; 26] = b"\\Device\\MountPointManager\0";
pub const MOUNTMGRCONTROLTYPE: u32 = 109;
pub const MOUNTDEVCONTROLTYPE: u32 = 77;
pub const IOCTL_MOUNTMGR_CREATE_POINT: u32 = 7192576;
pub const IOCTL_MOUNTMGR_DELETE_POINTS: u32 = 7192580;
pub const IOCTL_MOUNTMGR_QUERY_POINTS: u32 = 7143432;
pub const IOCTL_MOUNTMGR_DELETE_POINTS_DBONLY: u32 = 7192588;
pub const IOCTL_MOUNTMGR_NEXT_DRIVE_LETTER: u32 = 7192592;
pub const IOCTL_MOUNTMGR_AUTO_DL_ASSIGNMENTS: u32 = 7192596;
pub const IOCTL_MOUNTMGR_VOLUME_MOUNT_POINT_CREATED: u32 = 7192600;
pub const IOCTL_MOUNTMGR_VOLUME_MOUNT_POINT_DELETED: u32 = 7192604;
pub const IOCTL_MOUNTMGR_CHANGE_NOTIFY: u32 = 7159840;
pub const IOCTL_MOUNTMGR_KEEP_LINKS_WHEN_OFFLINE: u32 = 7192612;
pub const IOCTL_MOUNTMGR_CHECK_UNPROCESSED_VOLUMES: u32 = 7159848;
pub const IOCTL_MOUNTMGR_VOLUME_ARRIVAL_NOTIFICATION: u32 = 7159852;
pub const IOCTL_MOUNTMGR_QUERY_DOS_VOLUME_PATH: u32 = 109;
pub const IOCTL_MOUNTMGR_QUERY_DOS_VOLUME_PATHS: u32 = 109;
pub const IOCTL_MOUNTMGR_SCRUB_REGISTRY: u32 = 7192632;
pub const IOCTL_MOUNTMGR_QUERY_AUTO_MOUNT: u32 = 7143484;
pub const IOCTL_MOUNTMGR_SET_AUTO_MOUNT: u32 = 7192640;
pub const IOCTL_MOUNTMGR_BOOT_DL_ASSIGNMENT: u32 = 7192644;
pub const IOCTL_MOUNTMGR_TRACELOG_CACHE: u32 = 7159880;
pub const IOCTL_MOUNTMGR_PREPARE_VOLUME_DELETE: u32 = 7192652;
pub const IOCTL_MOUNTMGR_CANCEL_VOLUME_DELETE: u32 = 7192656;
pub const IOCTL_MOUNTMGR_SILO_ARRIVAL: u32 = 7192660;
pub const IOCTL_MOUNTDEV_QUERY_DEVICE_NAME: u32 = 5046280;
pub const KSEC_DEVICE_NAME: &[u8; 15] = b"\\Device\\KSecDD\0";
pub const IOCTL_KSEC_CONNECT_LSA: u32 = 3768320;
pub const IOCTL_KSEC_RNG: u32 = 3735556;
pub const IOCTL_KSEC_RNG_REKEY: u32 = 3735560;
pub const IOCTL_KSEC_ENCRYPT_MEMORY: u32 = 3735566;
pub const IOCTL_KSEC_DECRYPT_MEMORY: u32 = 3735570;
pub const IOCTL_KSEC_ENCRYPT_MEMORY_CROSS_PROC: u32 = 3735574;
pub const IOCTL_KSEC_DECRYPT_MEMORY_CROSS_PROC: u32 = 3735578;
pub const IOCTL_KSEC_ENCRYPT_MEMORY_SAME_LOGON: u32 = 3735582;
pub const IOCTL_KSEC_DECRYPT_MEMORY_SAME_LOGON: u32 = 3735586;
pub const IOCTL_KSEC_FIPS_GET_FUNCTION_TABLE: u32 = 3735588;
pub const IOCTL_KSEC_ALLOC_POOL: u32 = 3735592;
pub const IOCTL_KSEC_FREE_POOL: u32 = 3735596;
pub const IOCTL_KSEC_COPY_POOL: u32 = 3735600;
pub const IOCTL_KSEC_DUPLICATE_HANDLE: u32 = 3735604;
pub const IOCTL_KSEC_REGISTER_EXTENSION: u32 = 3735608;
pub const IOCTL_KSEC_CLIENT_CALLBACK: u32 = 3735612;
pub const IOCTL_KSEC_GET_BCRYPT_EXTENSION: u32 = 3735616;
pub const IOCTL_KSEC_GET_SSL_EXTENSION: u32 = 3735620;
pub const IOCTL_KSEC_GET_DEVICECONTROL_EXTENSION: u32 = 3735624;
pub const IOCTL_KSEC_ALLOC_VM: u32 = 3735628;
pub const IOCTL_KSEC_FREE_VM: u32 = 3735632;
pub const IOCTL_KSEC_COPY_VM: u32 = 3735636;
pub const IOCTL_KSEC_CLIENT_FREE_VM: u32 = 3735640;
pub const IOCTL_KSEC_INSERT_PROTECTED_PROCESS_ADDRESS: u32 = 3735644;
pub const IOCTL_KSEC_REMOVE_PROTECTED_PROCESS_ADDRESS: u32 = 3735648;
pub const IOCTL_KSEC_GET_BCRYPT_EXTENSION2: u32 = 3735652;
pub const IOCTL_KSEC_IPC_GET_QUEUED_FUNCTION_CALLS: u32 = 3735658;
pub const IOCTL_KSEC_IPC_SET_FUNCTION_RETURN: u32 = 3735663;
pub const FILE_INVALID_FILE_ID: u64 = 18446744073709551615;
pub const REPARSE_DATA_BUFFER_HEADER_SIZE: u32 = 8;
pub const FILE_COPY_STRUCTURED_STORAGE: u32 = 65;
pub const FILE_STRUCTURED_STORAGE: u32 = 1089;

#[repr(C)]
pub struct FILE_IO_COMPLETION_INFORMATION {
    pub KeyContext: *mut std::ffi::c_void,
    pub ApcContext: *mut std::ffi::c_void,
    pub IoStatusBlock: IO_STATUS_BLOCK,
}

impl Default for FILE_IO_COMPLETION_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for FILE_IO_COMPLETION_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_IO_COMPLETION_INFORMATION {{ }}")
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FILE_INFORMATION_CLASS {
    FileDirectoryInformation = 1,
    FileFullDirectoryInformation = 2,
    FileBothDirectoryInformation = 3,
    FileBasicInformation = 4,
    FileStandardInformation = 5,
    FileInternalInformation = 6,
    FileEaInformation = 7,
    FileAccessInformation = 8,
    FileNameInformation = 9,
    FileRenameInformation = 10,
    FileLinkInformation = 11,
    FileNamesInformation = 12,
    FileDispositionInformation = 13,
    FilePositionInformation = 14,
    FileFullEaInformation = 15,
    FileModeInformation = 16,
    FileAlignmentInformation = 17,
    FileAllInformation = 18,
    FileAllocationInformation = 19,
    FileEndOfFileInformation = 20,
    FileAlternateNameInformation = 21,
    FileStreamInformation = 22,
    FilePipeInformation = 23,
    FilePipeLocalInformation = 24,
    FilePipeRemoteInformation = 25,
    FileMailslotQueryInformation = 26,
    FileMailslotSetInformation = 27,
    FileCompressionInformation = 28,
    FileObjectIdInformation = 29,
    FileCompletionInformation = 30,
    FileMoveClusterInformation = 31,
    FileQuotaInformation = 32,
    FileReparsePointInformation = 33,
    FileNetworkOpenInformation = 34,
    FileAttributeTagInformation = 35,
    FileTrackingInformation = 36,
    FileIdBothDirectoryInformation = 37,
    FileIdFullDirectoryInformation = 38,
    FileValidDataLengthInformation = 39,
    FileShortNameInformation = 40,
    FileIoCompletionNotificationInformation = 41,
    FileIoStatusBlockRangeInformation = 42,
    FileIoPriorityHintInformation = 43,
    FileSfioReserveInformation = 44,
    FileSfioVolumeInformation = 45,
    FileHardLinkInformation = 46,
    FileProcessIdsUsingFileInformation = 47,
    FileNormalizedNameInformation = 48,
    FileNetworkPhysicalNameInformation = 49,
    FileIdGlobalTxDirectoryInformation = 50,
    FileIsRemoteDeviceInformation = 51,
    FileUnusedInformation = 52,
    FileNumaNodeInformation = 53,
    FileStandardLinkInformation = 54,
    FileRemoteProtocolInformation = 55,
    FileRenameInformationBypassAccessCheck = 56,
    FileLinkInformationBypassAccessCheck = 57,
    FileVolumeNameInformation = 58,
    FileIdInformation = 59,
    FileIdExtdDirectoryInformation = 60,
    FileReplaceCompletionInformation = 61,
    FileHardLinkFullIdInformation = 62,
    FileIdExtdBothDirectoryInformation = 63,
    FileDispositionInformationEx = 64,
    FileRenameInformationEx = 65,
    FileRenameInformationExBypassAccessCheck = 66,
    FileDesiredStorageClassInformation = 67,
    FileStatInformation = 68,
    FileMemoryPartitionInformation = 69,
    FileStatLxInformation = 70,
    FileCaseSensitiveInformation = 71,
    FileLinkInformationEx = 72,
    FileLinkInformationExBypassAccessCheck = 73,
    FileStorageReserveIdInformation = 74,
    FileCaseSensitiveInformationForceAccessCheck = 75,
    FileKnownFolderInformation = 76,
    FileMaximumInformation = 77,
}

#[repr(C)]
pub struct FILE_INTERNAL_INFORMATION {
    pub Anonymous1: FILE_INTERNAL_INFORMATION_1,
}

#[repr(C)]
pub struct FILE_INTERNAL_INFORMATION_1 {
    pub IndexNumber: UnionField<i64>,
    pub Anonymous1: UnionField<FILE_INTERNAL_INFORMATION_1_1>,
    pub union_field: u64,
}

#[repr(C)]
#[repr(align(8))]
pub struct FILE_INTERNAL_INFORMATION_1_1 {
    _bitfield_align_1: [u64; 0],
    _bitfield_1: BitfieldUnit<[u8; 8]>,
}

impl Default for FILE_INTERNAL_INFORMATION_1_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for FILE_INTERNAL_INFORMATION_1_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FILE_INTERNAL_INFORMATION_1_1 {{ MftRecordIndex : {:?}, SequenceNumber : {:?} }}",
            self.MftRecordIndex(),
            self.SequenceNumber()
        )
    }
}

impl FILE_INTERNAL_INFORMATION_1_1 {
    #[inline]
    pub fn MftRecordIndex(&self) -> i64 {
        u64::cast_signed(self._bitfield_1.get(0usize, 48u8))
    }

    #[inline]
    pub fn set_MftRecordIndex(&mut self, val: i64) {
        self._bitfield_1.set(0usize, 48u8, val as u64)
    }

    #[inline]
    pub fn SequenceNumber(&self) -> i64 {
        u64::cast_signed(self._bitfield_1.get(48usize, 16u8))
    }

    #[inline]
    pub fn set_SequenceNumber(&mut self, val: i64) {
        self._bitfield_1.set(48usize, 16u8, val as u64)
    }

    #[inline]
    pub fn new_bitfield_1(MftRecordIndex: i64, SequenceNumber: i64) -> BitfieldUnit<[u8; 8]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 8]> = Default::default();

        bitfield_unit.set(0usize, 48u8, {
            MftRecordIndex as u64
        });

        bitfield_unit.set(48usize, 16u8, {
            SequenceNumber as u64
        });

        bitfield_unit
    }
}

impl Default for FILE_INTERNAL_INFORMATION_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for FILE_INTERNAL_INFORMATION_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_INTERNAL_INFORMATION_1 {{ union }}")
    }
}

impl Default for FILE_INTERNAL_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for FILE_INTERNAL_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FILE_INTERNAL_INFORMATION {{ Anonymous1: {:?} }}",
            self.Anonymous1
        )
    }
}

#[repr(C)]
pub struct FILE_POSITION_INFORMATION {
    pub CurrentByteOffset: i64,
}

impl Default for FILE_POSITION_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for FILE_POSITION_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_POSITION_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct FILE_LINK_INFORMATION_EX {
    pub Flags: u32,
    pub RootDirectory: HANDLE,
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}

impl Default for FILE_LINK_INFORMATION_EX {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for FILE_LINK_INFORMATION_EX {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FILE_LINK_INFORMATION_EX {{ FileName: {:?} }}",
            self.FileName
        )
    }
}

#[repr(C)]
pub struct FILE_RENAME_INFORMATION_EX {
    pub Flags: u32,
    pub RootDirectory: HANDLE,
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}

impl Default for FILE_RENAME_INFORMATION_EX {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for FILE_RENAME_INFORMATION_EX {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FILE_RENAME_INFORMATION_EX {{ FileName: {:?} }}",
            self.FileName
        )
    }
}

#[repr(C)]
pub struct FILE_INTEGRITY_STREAM_INFORMATION {
    pub ChecksumAlgorithm: u16,
    pub ChecksumChunkShift: u8,
    pub ClusterShift: u8,
    pub Flags: u32,
}

impl Default for FILE_INTEGRITY_STREAM_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for FILE_INTEGRITY_STREAM_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_INTEGRITY_STREAM_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct FILE_DIRECTORY_NEXT_INFORMATION {
    pub NextEntryOffset: u32,
}

impl Default for FILE_DIRECTORY_NEXT_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for FILE_DIRECTORY_NEXT_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_DIRECTORY_NEXT_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct FILE_QUOTA_INFORMATION {
    pub NextEntryOffset: u32,
    pub SidLength: u32,
    pub ChangeTime: i64,
    pub QuotaUsed: i64,
    pub QuotaThreshold: i64,
    pub QuotaLimit: i64,
    pub Sid: SID,
}

impl Default for FILE_QUOTA_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for FILE_QUOTA_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_QUOTA_INFORMATION {{  }}")
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FSINFOCLASS {
    FileFsVolumeInformation = 1,
    FileFsLabelInformation = 2,
    FileFsSizeInformation = 3,
    FileFsDeviceInformation = 4,
    FileFsAttributeInformation = 5,
    FileFsControlInformation = 6,
    FileFsFullSizeInformation = 7,
    FileFsObjectIdInformation = 8,
    FileFsDriverPathInformation = 9,
    FileFsVolumeFlagsInformation = 10,
    FileFsSectorSizeInformation = 11,
    FileFsDataCopyInformation = 12,
    FileFsMetadataSizeInformation = 13,
    FileFsFullSizeInformationEx = 14,
    FileFsMaximumInformation = 15,
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtCreateNamedPipeFile(
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
    pub fn NtCreateMailslotFile(
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
    pub fn NtDeleteFile(ObjectAttributes: *mut OBJECT_ATTRIBUTES) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtFlushBuffersFile(FileHandle: HANDLE, IoStatusBlock: *mut IO_STATUS_BLOCK) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtQueryEaFile(
        FileHandle: HANDLE,
        IoStatusBlock: *mut IO_STATUS_BLOCK,
        Buffer: *mut std::ffi::c_void,
        Length: u32,
        ReturnSingleEntry: bool,
        EaList: *mut std::ffi::c_void,
        EaListLength: u32,
        EaIndex: *mut u32,
        RestartScan: bool,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtSetEaFile(
        FileHandle: HANDLE,
        IoStatusBlock: *mut IO_STATUS_BLOCK,
        Buffer: *mut std::ffi::c_void,
        Length: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtCancelIoFile(FileHandle: HANDLE, IoStatusBlock: *mut IO_STATUS_BLOCK) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtCancelSynchronousIoFile(
        ThreadHandle: HANDLE,
        IoRequestToCancel: *mut IO_STATUS_BLOCK,
        IoStatusBlock: *mut IO_STATUS_BLOCK,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtReadFileScatter(
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
    pub fn NtWriteFileGather(
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
    pub fn NtQueryAttributesFile(
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        FileInformation: *mut FILE_BASIC_INFORMATION,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtQueryFullAttributesFile(
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        FileInformation: *mut FILE_NETWORK_OPEN_INFORMATION,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtNotifyChangeDirectoryFile(
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
    pub fn NtNotifyChangeDirectoryFileEx(
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
    pub fn NtLoadDriver(DriverServiceName: *mut UNICODE_STRING) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtUnloadDriver(DriverServiceName: *mut UNICODE_STRING) -> NTSTATUS;
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum IO_COMPLETION_INFORMATION_CLASS {
    IoCompletionBasicInformation = 0,
}

#[repr(C)]
pub struct IO_COMPLETION_BASIC_INFORMATION {
    pub Depth: i32,
}

impl Default for IO_COMPLETION_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for IO_COMPLETION_BASIC_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IO_COMPLETION_BASIC_INFORMATION {{  }}")
    }
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtCreateIoCompletion(
        IoCompletionHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        Count: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtOpenIoCompletion(
        IoCompletionHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtQueryIoCompletion(
        IoCompletionHandle: HANDLE,
        IoCompletionInformationClass: IO_COMPLETION_INFORMATION_CLASS,
        IoCompletionInformation: *mut std::ffi::c_void,
        IoCompletionInformationLength: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtSetIoCompletion(
        IoCompletionHandle: HANDLE,
        KeyContext: *mut std::ffi::c_void,
        ApcContext: *mut std::ffi::c_void,
        IoStatus: NTSTATUS,
        IoStatusInformation: usize,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtSetIoCompletionEx(
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
    pub fn NtRemoveIoCompletion(
        IoCompletionHandle: HANDLE,
        KeyContext: *mut *mut std::ffi::c_void,
        ApcContext: *mut *mut std::ffi::c_void,
        IoStatusBlock: *mut IO_STATUS_BLOCK,
        Timeout: *mut i64,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtRemoveIoCompletionEx(
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
    pub fn NtCreateWaitCompletionPacket(
        WaitCompletionPacketHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtAssociateWaitCompletionPacket(
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
    pub fn NtCancelWaitCompletionPacket(
        WaitCompletionPacketHandle: HANDLE,
        RemoveSignaledPacket: bool,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtOpenSession(
        SessionHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtNotifyChangeSession(
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

#[repr(C)]
pub struct FILE_MAILSLOT_PEEK_BUFFER {
    pub ReadDataAvailable: u32,
    pub NumberOfMessages: u32,
    pub MessageLength: u32,
}

impl Default for FILE_MAILSLOT_PEEK_BUFFER {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for FILE_MAILSLOT_PEEK_BUFFER {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_MAILSLOT_PEEK_BUFFER {{  }}")
    }
}

#[repr(C)]
pub struct MOUNTMGR_CREATE_POINT_INPUT {
    pub SymbolicLinkNameOffset: u16,
    pub SymbolicLinkNameLength: u16,
    pub DeviceNameOffset: u16,
    pub DeviceNameLength: u16,
}

impl Default for MOUNTMGR_CREATE_POINT_INPUT {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for MOUNTMGR_CREATE_POINT_INPUT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MOUNTMGR_CREATE_POINT_INPUT {{  }}")
    }
}

#[repr(C)]
pub struct MOUNTMGR_MOUNT_POINT {
    pub SymbolicLinkNameOffset: u32,
    pub SymbolicLinkNameLength: u16,
    pub Reserved1: u16,
    pub UniqueIdOffset: u32,
    pub UniqueIdLength: u16,
    pub Reserved2: u16,
    pub DeviceNameOffset: u32,
    pub DeviceNameLength: u16,
    pub Reserved3: u16,
}

impl Default for MOUNTMGR_MOUNT_POINT {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for MOUNTMGR_MOUNT_POINT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MOUNTMGR_MOUNT_POINT {{  }}")
    }
}

#[repr(C)]
pub struct MOUNTMGR_MOUNT_POINTS {
    pub Size: u32,
    pub NumberOfMountPoints: u32,
    pub MountPoints: [MOUNTMGR_MOUNT_POINT; 1],
}

impl Default for MOUNTMGR_MOUNT_POINTS {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for MOUNTMGR_MOUNT_POINTS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MOUNTMGR_MOUNT_POINTS {{ MountPoints: {:?} }}",
            self.MountPoints
        )
    }
}

#[repr(C)]
pub struct MOUNTMGR_DRIVE_LETTER_TARGET {
    pub DeviceNameLength: u16,
    pub DeviceName: [u16; 1],
}

impl Default for MOUNTMGR_DRIVE_LETTER_TARGET {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for MOUNTMGR_DRIVE_LETTER_TARGET {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MOUNTMGR_DRIVE_LETTER_TARGET {{ DeviceName: {:?} }}",
            self.DeviceName
        )
    }
}

#[repr(C)]
pub struct MOUNTMGR_DRIVE_LETTER_INFORMATION {
    pub DriveLetterWasAssigned: bool,
    pub CurrentDriveLetter: u8,
}

impl Default for MOUNTMGR_DRIVE_LETTER_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for MOUNTMGR_DRIVE_LETTER_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MOUNTMGR_DRIVE_LETTER_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct MOUNTMGR_VOLUME_MOUNT_POINT {
    pub SourceVolumeNameOffset: u16,
    pub SourceVolumeNameLength: u16,
    pub TargetVolumeNameOffset: u16,
    pub TargetVolumeNameLength: u16,
}

impl Default for MOUNTMGR_VOLUME_MOUNT_POINT {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for MOUNTMGR_VOLUME_MOUNT_POINT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MOUNTMGR_VOLUME_MOUNT_POINT {{  }}")
    }
}

#[repr(C)]
pub struct MOUNTMGR_CHANGE_NOTIFY_INFO {
    pub EpicNumber: u32,
}

impl Default for MOUNTMGR_CHANGE_NOTIFY_INFO {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for MOUNTMGR_CHANGE_NOTIFY_INFO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MOUNTMGR_CHANGE_NOTIFY_INFO {{  }}")
    }
}

#[repr(C)]
pub struct MOUNTMGR_TARGET_NAME {
    pub DeviceNameLength: u16,
    pub DeviceName: [u16; 1],
}

impl Default for MOUNTMGR_TARGET_NAME {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for MOUNTMGR_TARGET_NAME {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MOUNTMGR_TARGET_NAME {{ DeviceName: {:?} }}",
            self.DeviceName
        )
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MOUNTMGR_AUTO_MOUNT_STATE {
    Disabled = 0,
    Enabled = 1,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MOUNTMGR_QUERY_AUTO_MOUNT {
    pub CurrentState: MOUNTMGR_AUTO_MOUNT_STATE,
}

impl Default for MOUNTMGR_QUERY_AUTO_MOUNT {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MOUNTMGR_SET_AUTO_MOUNT {
    pub NewState: MOUNTMGR_AUTO_MOUNT_STATE,
}

impl Default for MOUNTMGR_SET_AUTO_MOUNT {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

#[repr(C)]
pub struct MOUNTMGR_SILO_ARRIVAL_INPUT {
    pub JobHandle: HANDLE,
}

impl Default for MOUNTMGR_SILO_ARRIVAL_INPUT {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for MOUNTMGR_SILO_ARRIVAL_INPUT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MOUNTMGR_SILO_ARRIVAL_INPUT {{  }}")
    }
}

#[repr(C)]
pub struct MOUNTDEV_NAME {
    pub NameLength: u16,
    pub Name: [u16; 1],
}

impl Default for MOUNTDEV_NAME {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for MOUNTDEV_NAME {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MOUNTDEV_NAME {{ Name: {:?} }}", self.Name)
    }
}

#[repr(C)]
pub struct MOUNTMGR_VOLUME_PATHS {
    pub MultiSzLength: u32,
    pub MultiSz: [u16; 1],
}

impl Default for MOUNTMGR_VOLUME_PATHS {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for MOUNTMGR_VOLUME_PATHS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MOUNTMGR_VOLUME_PATHS {{ MultiSz: {:?} }}", self.MultiSz)
    }
}
