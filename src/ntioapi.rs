use windows::{
    core::GUID,
    Wdk::{
        Foundation::OBJECT_ATTRIBUTES,
        Storage::FileSystem::{FILE_BASIC_INFORMATION, FILE_STANDARD_INFORMATION},
    },
    Win32::{
        Foundation::{BOOLEAN, HANDLE, NTSTATUS, PSID, UNICODE_STRING},
        Security::SID,
        Storage::FileSystem::{FILE_ID_128, FILE_SEGMENT_ELEMENT},
        System::{
            Ioctl::STORAGE_RESERVE_ID,
            IO::{IO_STATUS_BLOCK, PIO_APC_ROUTINE},
        },
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
pub const DEVICE_NAMED_PIPE: &[u8; 19usize] = b"\\Device\\NamedPipe\\\0";
pub const FILE_PIPE_SYMLINK_VALID_FLAGS: u32 = 3;
pub const MAILSLOT_CLASS_FIRSTCLASS: u32 = 1;
pub const MAILSLOT_CLASS_SECONDCLASS: u32 = 2;
pub const MOUNTMGR_DEVICE_NAME: &[u8; 26usize] = b"\\Device\\MountPointManager\0";
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
pub const KSEC_DEVICE_NAME: &[u8; 15usize] = b"\\Device\\KSecDD\0";
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
pub const FILE_INVALID_FILE_ID: i64 = -1;
pub const REPARSE_DATA_BUFFER_HEADER_SIZE: u32 = 8;
#[repr(C)]
pub struct EXTENDED_CREATE_INFORMATION {
    pub ExtendedCreateFlags: i64,
    pub EaBuffer: *mut std::ffi::c_void,
    pub EaLength: u32,
}
impl Default for EXTENDED_CREATE_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for EXTENDED_CREATE_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EXTENDED_CREATE_INFORMATION {{  }}")
    }
}
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
    _bitfield_1: BitfieldUnit<[u8; 8usize]>,
}
impl Default for FILE_INTERNAL_INFORMATION_1_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_INTERNAL_INFORMATION_1_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_INTERNAL_INFORMATION_1_1 {{ MftRecordIndex : {:?}, SequenceNumber : {:?} }}", self.MftRecordIndex(), self.SequenceNumber())
    }
}
impl FILE_INTERNAL_INFORMATION_1_1 {
    #[inline]
    pub fn MftRecordIndex(&self) -> i64 {
        unsafe { std::mem::transmute(self._bitfield_1.get(0usize, 48u8)) }
    }
    #[inline]
    pub fn set_MftRecordIndex(&mut self, val: i64) {
        unsafe {
            let val: u64 = std::mem::transmute(val);
            self._bitfield_1.set(0usize, 48u8, val as u64)
        }
    }
    #[inline]
    pub fn SequenceNumber(&self) -> i64 {
        unsafe { std::mem::transmute(self._bitfield_1.get(48usize, 16u8)) }
    }
    #[inline]
    pub fn set_SequenceNumber(&mut self, val: i64) {
        unsafe {
            let val: u64 = std::mem::transmute(val);
            self._bitfield_1.set(48usize, 16u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(MftRecordIndex: i64, SequenceNumber: i64) -> BitfieldUnit<[u8; 8usize]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 8usize]> = Default::default();
        bitfield_unit.set(0usize, 48u8, {
            let MftRecordIndex: u64 = unsafe { std::mem::transmute(MftRecordIndex) };
            MftRecordIndex as u64
        });
        bitfield_unit.set(48usize, 16u8, {
            let SequenceNumber: u64 = unsafe { std::mem::transmute(SequenceNumber) };
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
        write!(f, "FILE_INTERNAL_INFORMATION {{ Anonymous1: {:?} }}", self.Anonymous1)
    }
}
#[repr(C)]
pub struct FILE_EA_INFORMATION {
    pub EaSize: u32,
}
impl Default for FILE_EA_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_EA_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_EA_INFORMATION {{  }}")
    }
}
#[repr(C)]
pub struct FILE_ACCESS_INFORMATION {
    pub AccessFlags: u32,
}
impl Default for FILE_ACCESS_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_ACCESS_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_ACCESS_INFORMATION {{  }}")
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
pub struct FILE_MODE_INFORMATION {
    pub Mode: u32,
}
impl Default for FILE_MODE_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_MODE_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_MODE_INFORMATION {{  }}")
    }
}
#[repr(C)]
pub struct FILE_ALIGNMENT_INFORMATION {
    pub AlignmentRequirement: u32,
}
impl Default for FILE_ALIGNMENT_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_ALIGNMENT_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_ALIGNMENT_INFORMATION {{  }}")
    }
}
#[repr(C)]
pub struct FILE_NAME_INFORMATION {
    pub FileNameLength: u32,
    pub FileName: [u16; 1usize],
}
impl Default for FILE_NAME_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_NAME_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_NAME_INFORMATION {{ FileName: {:?} }}", self.FileName)
    }
}
#[repr(C)]
pub struct FILE_ALL_INFORMATION {
    pub BasicInformation: FILE_BASIC_INFORMATION,
    pub StandardInformation: FILE_STANDARD_INFORMATION,
    pub InternalInformation: FILE_INTERNAL_INFORMATION,
    pub EaInformation: FILE_EA_INFORMATION,
    pub AccessInformation: FILE_ACCESS_INFORMATION,
    pub PositionInformation: FILE_POSITION_INFORMATION,
    pub ModeInformation: FILE_MODE_INFORMATION,
    pub AlignmentInformation: FILE_ALIGNMENT_INFORMATION,
    pub NameInformation: FILE_NAME_INFORMATION,
}
impl Default for FILE_ALL_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_ALL_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_ALL_INFORMATION {{ BasicInformation: {:?}, StandardInformation: {:?}, InternalInformation: {:?}, EaInformation: {:?}, AccessInformation: {:?}, PositionInformation: {:?}, ModeInformation: {:?}, AlignmentInformation: {:?}, NameInformation: {:?} }}", self.BasicInformation, self.StandardInformation, self.InternalInformation, self.EaInformation, self.AccessInformation, self.PositionInformation, self.ModeInformation, self.AlignmentInformation, self.NameInformation)
    }
}
#[repr(C)]
pub struct FILE_NETWORK_OPEN_INFORMATION {
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub AllocationSize: i64,
    pub EndOfFile: i64,
    pub FileAttributes: u32,
}
impl Default for FILE_NETWORK_OPEN_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_NETWORK_OPEN_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_NETWORK_OPEN_INFORMATION {{  }}")
    }
}
#[repr(C)]
pub struct FILE_ATTRIBUTE_TAG_INFORMATION {
    pub FileAttributes: u32,
    pub ReparseTag: u32,
}
impl Default for FILE_ATTRIBUTE_TAG_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_ATTRIBUTE_TAG_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_ATTRIBUTE_TAG_INFORMATION {{  }}")
    }
}
#[repr(C)]
pub struct FILE_ALLOCATION_INFORMATION {
    pub AllocationSize: i64,
}
impl Default for FILE_ALLOCATION_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_ALLOCATION_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_ALLOCATION_INFORMATION {{  }}")
    }
}
#[repr(C)]
pub struct FILE_COMPRESSION_INFORMATION {
    pub CompressedFileSize: i64,
    pub CompressionFormat: u16,
    pub CompressionUnitShift: u8,
    pub ChunkShift: u8,
    pub ClusterShift: u8,
    pub Reserved: [u8; 3usize],
}
impl Default for FILE_COMPRESSION_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_COMPRESSION_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_COMPRESSION_INFORMATION {{ Reserved: {:?} }}", self.Reserved)
    }
}
#[repr(C)]
pub struct FILE_DISPOSITION_INFORMATION {
    pub DeleteFileA: BOOLEAN,
}
impl Default for FILE_DISPOSITION_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_DISPOSITION_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_DISPOSITION_INFORMATION {{  }}")
    }
}
#[repr(C)]
pub struct FILE_END_OF_FILE_INFORMATION {
    pub EndOfFile: i64,
}
impl Default for FILE_END_OF_FILE_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_END_OF_FILE_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_END_OF_FILE_INFORMATION {{  }}")
    }
}
#[repr(C)]
pub struct FILE_END_OF_FILE_INFORMATION_EX {
    pub EndOfFile: i64,
    pub PagingFileSizeInMM: i64,
    pub PagingFileMaxSize: i64,
    pub Flags: u32,
}
impl Default for FILE_END_OF_FILE_INFORMATION_EX {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_END_OF_FILE_INFORMATION_EX {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_END_OF_FILE_INFORMATION_EX {{  }}")
    }
}
#[repr(C)]
pub struct FILE_VALID_DATA_LENGTH_INFORMATION {
    pub ValidDataLength: i64,
}
impl Default for FILE_VALID_DATA_LENGTH_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_VALID_DATA_LENGTH_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_VALID_DATA_LENGTH_INFORMATION {{  }}")
    }
}
#[repr(C)]
pub struct FILE_LINK_INFORMATION {
    pub ReplaceIfExists: BOOLEAN,
    pub RootDirectory: HANDLE,
    pub FileNameLength: u32,
    pub FileName: [u16; 1usize],
}
impl Default for FILE_LINK_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_LINK_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_LINK_INFORMATION {{ FileName: {:?} }}", self.FileName)
    }
}
#[repr(C)]
pub struct FILE_LINK_INFORMATION_EX {
    pub Flags: u32,
    pub RootDirectory: HANDLE,
    pub FileNameLength: u32,
    pub FileName: [u16; 1usize],
}
impl Default for FILE_LINK_INFORMATION_EX {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_LINK_INFORMATION_EX {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_LINK_INFORMATION_EX {{ FileName: {:?} }}", self.FileName)
    }
}
#[repr(C)]
pub struct FILE_MOVE_CLUSTER_INFORMATION {
    pub ClusterCount: u32,
    pub RootDirectory: HANDLE,
    pub FileNameLength: u32,
    pub FileName: [u16; 1usize],
}
impl Default for FILE_MOVE_CLUSTER_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_MOVE_CLUSTER_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_MOVE_CLUSTER_INFORMATION {{ FileName: {:?} }}", self.FileName)
    }
}
#[repr(C)]
pub struct FILE_RENAME_INFORMATION {
    pub ReplaceIfExists: BOOLEAN,
    pub RootDirectory: HANDLE,
    pub FileNameLength: u32,
    pub FileName: [u16; 1usize],
}
impl Default for FILE_RENAME_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_RENAME_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_RENAME_INFORMATION {{ FileName: {:?} }}", self.FileName)
    }
}
#[repr(C)]
pub struct FILE_RENAME_INFORMATION_EX {
    pub Flags: u32,
    pub RootDirectory: HANDLE,
    pub FileNameLength: u32,
    pub FileName: [u16; 1usize],
}
impl Default for FILE_RENAME_INFORMATION_EX {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_RENAME_INFORMATION_EX {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_RENAME_INFORMATION_EX {{ FileName: {:?} }}", self.FileName)
    }
}
#[repr(C)]
pub struct FILE_STREAM_INFORMATION {
    pub NextEntryOffset: u32,
    pub StreamNameLength: u32,
    pub StreamSize: i64,
    pub StreamAllocationSize: i64,
    pub StreamName: [u16; 1usize],
}
impl Default for FILE_STREAM_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_STREAM_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_STREAM_INFORMATION {{ StreamName: {:?} }}", self.StreamName)
    }
}
#[repr(C)]
pub struct FILE_TRACKING_INFORMATION {
    pub DestinationFile: HANDLE,
    pub ObjectInformationLength: u32,
    pub ObjectInformation: [i8; 1usize],
}
impl Default for FILE_TRACKING_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_TRACKING_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_TRACKING_INFORMATION {{ ObjectInformation: {:?} }}", self.ObjectInformation)
    }
}
#[repr(C)]
pub struct FILE_COMPLETION_INFORMATION {
    pub Port: HANDLE,
    pub Key: *mut std::ffi::c_void,
}
impl Default for FILE_COMPLETION_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_COMPLETION_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_COMPLETION_INFORMATION {{  }}")
    }
}
#[repr(C)]
pub struct FILE_PIPE_INFORMATION {
    pub ReadMode: u32,
    pub CompletionMode: u32,
}
impl Default for FILE_PIPE_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_PIPE_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_PIPE_INFORMATION {{  }}")
    }
}
#[repr(C)]
pub struct FILE_PIPE_LOCAL_INFORMATION {
    pub NamedPipeType: u32,
    pub NamedPipeConfiguration: u32,
    pub MaximumInstances: u32,
    pub CurrentInstances: u32,
    pub InboundQuota: u32,
    pub ReadDataAvailable: u32,
    pub OutboundQuota: u32,
    pub WriteQuotaAvailable: u32,
    pub NamedPipeState: u32,
    pub NamedPipeEnd: u32,
}
impl Default for FILE_PIPE_LOCAL_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_PIPE_LOCAL_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_PIPE_LOCAL_INFORMATION {{  }}")
    }
}
#[repr(C)]
pub struct FILE_PIPE_REMOTE_INFORMATION {
    pub CollectDataTime: i64,
    pub MaximumCollectionCount: u32,
}
impl Default for FILE_PIPE_REMOTE_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_PIPE_REMOTE_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_PIPE_REMOTE_INFORMATION {{  }}")
    }
}
#[repr(C)]
pub struct FILE_MAILSLOT_QUERY_INFORMATION {
    pub MaximumMessageSize: u32,
    pub MailslotQuota: u32,
    pub NextMessageSize: u32,
    pub MessagesAvailable: u32,
    pub ReadTimeout: i64,
}
impl Default for FILE_MAILSLOT_QUERY_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_MAILSLOT_QUERY_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_MAILSLOT_QUERY_INFORMATION {{  }}")
    }
}
#[repr(C)]
pub struct FILE_MAILSLOT_SET_INFORMATION {
    pub ReadTimeout: *mut i64,
}
impl Default for FILE_MAILSLOT_SET_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_MAILSLOT_SET_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_MAILSLOT_SET_INFORMATION {{  }}")
    }
}
#[repr(C)]
pub struct FILE_REPARSE_POINT_INFORMATION {
    pub FileReference: i64,
    pub Tag: u32,
}
impl Default for FILE_REPARSE_POINT_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_REPARSE_POINT_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_REPARSE_POINT_INFORMATION {{  }}")
    }
}
#[repr(C)]
pub struct FILE_LINK_ENTRY_INFORMATION {
    pub NextEntryOffset: u32,
    pub ParentFileId: i64,
    pub FileNameLength: u32,
    pub FileName: [u16; 1usize],
}
impl Default for FILE_LINK_ENTRY_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_LINK_ENTRY_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_LINK_ENTRY_INFORMATION {{ FileName: {:?} }}", self.FileName)
    }
}
#[repr(C)]
pub struct FILE_LINKS_INFORMATION {
    pub BytesNeeded: u32,
    pub EntriesReturned: u32,
    pub Entry: FILE_LINK_ENTRY_INFORMATION,
}
impl Default for FILE_LINKS_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_LINKS_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_LINKS_INFORMATION {{ Entry: {:?} }}", self.Entry)
    }
}
#[repr(C)]
pub struct FILE_NETWORK_PHYSICAL_NAME_INFORMATION {
    pub FileNameLength: u32,
    pub FileName: [u16; 1usize],
}
impl Default for FILE_NETWORK_PHYSICAL_NAME_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_NETWORK_PHYSICAL_NAME_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_NETWORK_PHYSICAL_NAME_INFORMATION {{ FileName: {:?} }}", self.FileName)
    }
}
#[repr(C)]
pub struct FILE_STANDARD_LINK_INFORMATION {
    pub NumberOfAccessibleLinks: u32,
    pub TotalNumberOfLinks: u32,
    pub DeletePending: BOOLEAN,
    pub Directory: BOOLEAN,
}
impl Default for FILE_STANDARD_LINK_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_STANDARD_LINK_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_STANDARD_LINK_INFORMATION {{  }}")
    }
}
#[repr(C)]
pub struct FILE_SFIO_RESERVE_INFORMATION {
    pub RequestsPerPeriod: u32,
    pub Period: u32,
    pub RetryFailures: BOOLEAN,
    pub Discardable: BOOLEAN,
    pub RequestSize: u32,
    pub NumOutstandingRequests: u32,
}
impl Default for FILE_SFIO_RESERVE_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_SFIO_RESERVE_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_SFIO_RESERVE_INFORMATION {{  }}")
    }
}
#[repr(C)]
pub struct FILE_SFIO_VOLUME_INFORMATION {
    pub MaximumRequestsPerPeriod: u32,
    pub MinimumPeriod: u32,
    pub MinimumTransferSize: u32,
}
impl Default for FILE_SFIO_VOLUME_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_SFIO_VOLUME_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_SFIO_VOLUME_INFORMATION {{  }}")
    }
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum IO_PRIORITY_HINT {
    IoPriorityVeryLow = 0,
    IoPriorityLow = 1,
    IoPriorityNormal = 2,
    IoPriorityHigh = 3,
    IoPriorityCritical = 4,
    MaxIoPriorityTypes = 5,
}
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct FILE_IO_PRIORITY_HINT_INFORMATION {
    pub PriorityHint: IO_PRIORITY_HINT,
}
impl Default for FILE_IO_PRIORITY_HINT_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
pub struct FILE_IO_PRIORITY_HINT_INFORMATION_EX {
    pub PriorityHint: IO_PRIORITY_HINT,
    pub BoostOutstanding: BOOLEAN,
}
impl Default for FILE_IO_PRIORITY_HINT_INFORMATION_EX {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_IO_PRIORITY_HINT_INFORMATION_EX {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_IO_PRIORITY_HINT_INFORMATION_EX {{ PriorityHint: {:?} }}", self.PriorityHint)
    }
}
#[repr(C)]
pub struct FILE_IO_COMPLETION_NOTIFICATION_INFORMATION {
    pub Flags: u32,
}
impl Default for FILE_IO_COMPLETION_NOTIFICATION_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_IO_COMPLETION_NOTIFICATION_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_IO_COMPLETION_NOTIFICATION_INFORMATION {{  }}")
    }
}
#[repr(C)]
pub struct FILE_PROCESS_IDS_USING_FILE_INFORMATION {
    pub NumberOfProcessIdsInList: u32,
    pub ProcessIdList: [usize; 1usize],
}
impl Default for FILE_PROCESS_IDS_USING_FILE_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_PROCESS_IDS_USING_FILE_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_PROCESS_IDS_USING_FILE_INFORMATION {{ ProcessIdList: {:?} }}", self.ProcessIdList)
    }
}
#[repr(C)]
pub struct FILE_IS_REMOTE_DEVICE_INFORMATION {
    pub IsRemote: BOOLEAN,
}
impl Default for FILE_IS_REMOTE_DEVICE_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_IS_REMOTE_DEVICE_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_IS_REMOTE_DEVICE_INFORMATION {{  }}")
    }
}
#[repr(C)]
pub struct FILE_NUMA_NODE_INFORMATION {
    pub NodeNumber: u16,
}
impl Default for FILE_NUMA_NODE_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_NUMA_NODE_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_NUMA_NODE_INFORMATION {{  }}")
    }
}
#[repr(C)]
pub struct FILE_IOSTATUSBLOCK_RANGE_INFORMATION {
    pub IoStatusBlockRange: *mut u8,
    pub Length: u32,
}
impl Default for FILE_IOSTATUSBLOCK_RANGE_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_IOSTATUSBLOCK_RANGE_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_IOSTATUSBLOCK_RANGE_INFORMATION {{  }}")
    }
}
#[repr(C)]
pub struct FILE_REMOTE_PROTOCOL_INFORMATION {
    pub StructureVersion: u16,
    pub StructureSize: u16,
    pub Protocol: u32,
    pub ProtocolMajorVersion: u16,
    pub ProtocolMinorVersion: u16,
    pub ProtocolRevision: u16,
    pub Reserved: u16,
    pub Flags: u32,
    pub GenericReserved: FILE_REMOTE_PROTOCOL_INFORMATION_1,
    pub ProtocolSpecific: FILE_REMOTE_PROTOCOL_INFORMATION_2,
}
#[repr(C)]
pub struct FILE_REMOTE_PROTOCOL_INFORMATION_1 {
    pub Reserved: [u32; 8usize],
}
impl Default for FILE_REMOTE_PROTOCOL_INFORMATION_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_REMOTE_PROTOCOL_INFORMATION_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_REMOTE_PROTOCOL_INFORMATION_1 {{ Reserved: {:?} }}", self.Reserved)
    }
}
#[repr(C)]
pub struct FILE_REMOTE_PROTOCOL_INFORMATION_2 {
    pub Smb2: UnionField<FILE_REMOTE_PROTOCOL_INFORMATION_2_1>,
    pub Reserved: UnionField<[u32; 16usize]>,
    pub union_field: [u32; 16usize],
}
#[repr(C)]
pub struct FILE_REMOTE_PROTOCOL_INFORMATION_2_1 {
    pub Server: FILE_REMOTE_PROTOCOL_INFORMATION_2_1_1,
    pub Share: FILE_REMOTE_PROTOCOL_INFORMATION_2_1_2,
}
#[repr(C)]
pub struct FILE_REMOTE_PROTOCOL_INFORMATION_2_1_1 {
    pub Capabilities: u32,
}
impl Default for FILE_REMOTE_PROTOCOL_INFORMATION_2_1_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_REMOTE_PROTOCOL_INFORMATION_2_1_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_REMOTE_PROTOCOL_INFORMATION_2_1_1 {{  }}")
    }
}
#[repr(C)]
pub struct FILE_REMOTE_PROTOCOL_INFORMATION_2_1_2 {
    pub Capabilities: u32,
    pub ShareFlags: u32,
    pub ShareType: u8,
    pub Reserved0: [u8; 3usize],
    pub Reserved1: u32,
}
impl Default for FILE_REMOTE_PROTOCOL_INFORMATION_2_1_2 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_REMOTE_PROTOCOL_INFORMATION_2_1_2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_REMOTE_PROTOCOL_INFORMATION_2_1_2 {{ Reserved0: {:?} }}", self.Reserved0)
    }
}
impl Default for FILE_REMOTE_PROTOCOL_INFORMATION_2_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_REMOTE_PROTOCOL_INFORMATION_2_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_REMOTE_PROTOCOL_INFORMATION_2_1 {{ Server: {:?}, Share: {:?} }}", self.Server, self.Share)
    }
}
impl Default for FILE_REMOTE_PROTOCOL_INFORMATION_2 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_REMOTE_PROTOCOL_INFORMATION_2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_REMOTE_PROTOCOL_INFORMATION_2 {{ union }}")
    }
}
impl Default for FILE_REMOTE_PROTOCOL_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_REMOTE_PROTOCOL_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_REMOTE_PROTOCOL_INFORMATION {{ GenericReserved: {:?}, ProtocolSpecific: {:?} }}", self.GenericReserved, self.ProtocolSpecific)
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
pub struct FILE_VOLUME_NAME_INFORMATION {
    pub DeviceNameLength: u32,
    pub DeviceName: [u16; 1usize],
}
impl Default for FILE_VOLUME_NAME_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_VOLUME_NAME_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_VOLUME_NAME_INFORMATION {{ DeviceName: {:?} }}", self.DeviceName)
    }
}
#[repr(C)]
pub struct FILE_ID_INFORMATION {
    pub VolumeSerialNumber: u64,
    pub Anonymous1: FILE_ID_INFORMATION_1,
}
#[repr(C)]
pub struct FILE_ID_INFORMATION_1 {
    pub FileId: UnionField<FILE_ID_128>,
    pub Anonymous1: UnionField<FILE_ID_INFORMATION_1_1>,
    pub union_field: [u64; 2usize],
}
#[repr(C)]
#[repr(align(8))]
pub struct FILE_ID_INFORMATION_1_1 {
    _bitfield_align_1: [u64; 0],
    _bitfield_1: BitfieldUnit<[u8; 16usize]>,
}
impl Default for FILE_ID_INFORMATION_1_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_ID_INFORMATION_1_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_ID_INFORMATION_1_1 {{ FileIdLowPart : {:?}, FileIdHighPart : {:?} }}", self.FileIdLowPart(), self.FileIdHighPart())
    }
}
impl FILE_ID_INFORMATION_1_1 {
    #[inline]
    pub fn FileIdLowPart(&self) -> i64 {
        unsafe { std::mem::transmute(self._bitfield_1.get(0usize, 64u8)) }
    }
    #[inline]
    pub fn set_FileIdLowPart(&mut self, val: i64) {
        unsafe {
            let val: u64 = std::mem::transmute(val);
            self._bitfield_1.set(0usize, 64u8, val as u64)
        }
    }
    #[inline]
    pub fn FileIdHighPart(&self) -> i64 {
        unsafe { std::mem::transmute(self._bitfield_1.get(64usize, 64u8)) }
    }
    #[inline]
    pub fn set_FileIdHighPart(&mut self, val: i64) {
        unsafe {
            let val: u64 = std::mem::transmute(val);
            self._bitfield_1.set(64usize, 64u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(FileIdLowPart: i64, FileIdHighPart: i64) -> BitfieldUnit<[u8; 16usize]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 16usize]> = Default::default();
        bitfield_unit.set(0usize, 64u8, {
            let FileIdLowPart: u64 = unsafe { std::mem::transmute(FileIdLowPart) };
            FileIdLowPart as u64
        });
        bitfield_unit.set(64usize, 64u8, {
            let FileIdHighPart: u64 = unsafe { std::mem::transmute(FileIdHighPart) };
            FileIdHighPart as u64
        });
        bitfield_unit
    }
}
impl Default for FILE_ID_INFORMATION_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_ID_INFORMATION_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_ID_INFORMATION_1 {{ union }}")
    }
}
impl Default for FILE_ID_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_ID_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_ID_INFORMATION {{ Anonymous1: {:?} }}", self.Anonymous1)
    }
}
#[repr(C)]
pub struct FILE_ID_EXTD_DIR_INFORMATION {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub EndOfFile: i64,
    pub AllocationSize: i64,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub EaSize: u32,
    pub ReparsePointTag: u32,
    pub FileId: FILE_ID_128,
    pub FileName: [u16; 1usize],
}
impl Default for FILE_ID_EXTD_DIR_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_ID_EXTD_DIR_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_ID_EXTD_DIR_INFORMATION {{ FileName: {:?} }}", self.FileName)
    }
}
#[repr(C)]
pub struct FILE_LINK_ENTRY_FULL_ID_INFORMATION {
    pub NextEntryOffset: u32,
    pub ParentFileId: FILE_ID_128,
    pub FileNameLength: u32,
    pub FileName: [u16; 1usize],
}
impl Default for FILE_LINK_ENTRY_FULL_ID_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_LINK_ENTRY_FULL_ID_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_LINK_ENTRY_FULL_ID_INFORMATION {{ FileName: {:?} }}", self.FileName)
    }
}
#[repr(C)]
pub struct FILE_LINKS_FULL_ID_INFORMATION {
    pub BytesNeeded: u32,
    pub EntriesReturned: u32,
    pub Entry: FILE_LINK_ENTRY_FULL_ID_INFORMATION,
}
impl Default for FILE_LINKS_FULL_ID_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_LINKS_FULL_ID_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_LINKS_FULL_ID_INFORMATION {{ Entry: {:?} }}", self.Entry)
    }
}
#[repr(C)]
pub struct FILE_ID_EXTD_BOTH_DIR_INFORMATION {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub EndOfFile: i64,
    pub AllocationSize: i64,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub EaSize: u32,
    pub ReparsePointTag: u32,
    pub FileId: FILE_ID_128,
    pub ShortNameLength: i8,
    pub ShortName: [u16; 12usize],
    pub FileName: [u16; 1usize],
}
impl Default for FILE_ID_EXTD_BOTH_DIR_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_ID_EXTD_BOTH_DIR_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_ID_EXTD_BOTH_DIR_INFORMATION {{ ShortName: {:?}, FileName: {:?} }}", self.ShortName, self.FileName)
    }
}
#[repr(C)]
pub struct FILE_STAT_INFORMATION {
    pub FileId: i64,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub AllocationSize: i64,
    pub EndOfFile: i64,
    pub FileAttributes: u32,
    pub ReparseTag: u32,
    pub NumberOfLinks: u32,
    pub EffectiveAccess: u32,
}
impl Default for FILE_STAT_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_STAT_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_STAT_INFORMATION {{  }}")
    }
}
#[repr(C)]
pub struct FILE_MEMORY_PARTITION_INFORMATION {
    pub OwnerPartitionHandle: HANDLE,
    pub Flags: FILE_MEMORY_PARTITION_INFORMATION_1,
}
#[repr(C)]
pub struct FILE_MEMORY_PARTITION_INFORMATION_1 {
    pub Anonymous1: UnionField<FILE_MEMORY_PARTITION_INFORMATION_1_1>,
    pub AllFlags: UnionField<u32>,
    pub union_field: u32,
}
#[repr(C)]
pub struct FILE_MEMORY_PARTITION_INFORMATION_1_1 {
    pub NoCrossPartitionAccess: u8,
    pub Spare: [u8; 3usize],
}
impl Default for FILE_MEMORY_PARTITION_INFORMATION_1_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_MEMORY_PARTITION_INFORMATION_1_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_MEMORY_PARTITION_INFORMATION_1_1 {{ Spare: {:?} }}", self.Spare)
    }
}
impl Default for FILE_MEMORY_PARTITION_INFORMATION_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_MEMORY_PARTITION_INFORMATION_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_MEMORY_PARTITION_INFORMATION_1 {{ union }}")
    }
}
impl Default for FILE_MEMORY_PARTITION_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_MEMORY_PARTITION_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_MEMORY_PARTITION_INFORMATION {{ Flags: {:?} }}", self.Flags)
    }
}
#[repr(C)]
pub struct FILE_STAT_LX_INFORMATION {
    pub FileId: i64,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub AllocationSize: i64,
    pub EndOfFile: i64,
    pub FileAttributes: u32,
    pub ReparseTag: u32,
    pub NumberOfLinks: u32,
    pub EffectiveAccess: u32,
    pub LxFlags: u32,
    pub LxUid: u32,
    pub LxGid: u32,
    pub LxMode: u32,
    pub LxDeviceIdMajor: u32,
    pub LxDeviceIdMinor: u32,
}
impl Default for FILE_STAT_LX_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_STAT_LX_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_STAT_LX_INFORMATION {{  }}")
    }
}
#[repr(C)]
pub struct FILE_STORAGE_RESERVE_ID_INFORMATION {
    pub StorageReserveId: STORAGE_RESERVE_ID,
}
impl Default for FILE_STORAGE_RESERVE_ID_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_STORAGE_RESERVE_ID_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_STORAGE_RESERVE_ID_INFORMATION {{  }}")
    }
}
#[repr(C)]
pub struct FILE_CASE_SENSITIVE_INFORMATION {
    pub Flags: u32,
}
impl Default for FILE_CASE_SENSITIVE_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_CASE_SENSITIVE_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_CASE_SENSITIVE_INFORMATION {{  }}")
    }
}
impl FILE_KNOWN_FOLDER_TYPE {}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FILE_KNOWN_FOLDER_TYPE {
    KnownFolderNone = 0,
    KnownFolderDesktop = 1,
    KnownFolderDocuments = 2,
    KnownFolderDownloads = 3,
    KnownFolderMusic = 4,
    KnownFolderPictures = 5,
    KnownFolderVideos = 6,
    KnownFolderOther = 7,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct FILE_KNOWN_FOLDER_INFORMATION {
    pub Type: FILE_KNOWN_FOLDER_TYPE,
}
impl Default for FILE_KNOWN_FOLDER_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
pub struct FILE_DIRECTORY_INFORMATION {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub EndOfFile: i64,
    pub AllocationSize: i64,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub FileName: [u16; 1usize],
}
impl Default for FILE_DIRECTORY_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_DIRECTORY_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_DIRECTORY_INFORMATION {{ FileName: {:?} }}", self.FileName)
    }
}
#[repr(C)]
pub struct FILE_FULL_DIR_INFORMATION {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub EndOfFile: i64,
    pub AllocationSize: i64,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub EaSize: u32,
    pub FileName: [u16; 1usize],
}
impl Default for FILE_FULL_DIR_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_FULL_DIR_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_FULL_DIR_INFORMATION {{ FileName: {:?} }}", self.FileName)
    }
}
#[repr(C)]
pub struct FILE_ID_FULL_DIR_INFORMATION {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub EndOfFile: i64,
    pub AllocationSize: i64,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub EaSize: u32,
    pub FileId: i64,
    pub FileName: [u16; 1usize],
}
impl Default for FILE_ID_FULL_DIR_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_ID_FULL_DIR_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_ID_FULL_DIR_INFORMATION {{ FileName: {:?} }}", self.FileName)
    }
}
#[repr(C)]
pub struct FILE_BOTH_DIR_INFORMATION {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub EndOfFile: i64,
    pub AllocationSize: i64,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub EaSize: u32,
    pub ShortNameLength: i8,
    pub ShortName: [u16; 12usize],
    pub FileName: [u16; 1usize],
}
impl Default for FILE_BOTH_DIR_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_BOTH_DIR_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_BOTH_DIR_INFORMATION {{ ShortName: {:?}, FileName: {:?} }}", self.ShortName, self.FileName)
    }
}
#[repr(C)]
pub struct FILE_ID_BOTH_DIR_INFORMATION {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub EndOfFile: i64,
    pub AllocationSize: i64,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub EaSize: u32,
    pub ShortNameLength: i8,
    pub ShortName: [u16; 12usize],
    pub FileId: i64,
    pub FileName: [u16; 1usize],
}
impl Default for FILE_ID_BOTH_DIR_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_ID_BOTH_DIR_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_ID_BOTH_DIR_INFORMATION {{ ShortName: {:?}, FileName: {:?} }}", self.ShortName, self.FileName)
    }
}
#[repr(C)]
pub struct FILE_NAMES_INFORMATION {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub FileNameLength: u32,
    pub FileName: [u16; 1usize],
}
impl Default for FILE_NAMES_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_NAMES_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_NAMES_INFORMATION {{ FileName: {:?} }}", self.FileName)
    }
}
#[repr(C)]
pub struct FILE_ID_GLOBAL_TX_DIR_INFORMATION {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub EndOfFile: i64,
    pub AllocationSize: i64,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub FileId: i64,
    pub LockingTransactionId: GUID,
    pub TxInfoFlags: u32,
    pub FileName: [u16; 1usize],
}
impl Default for FILE_ID_GLOBAL_TX_DIR_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_ID_GLOBAL_TX_DIR_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_ID_GLOBAL_TX_DIR_INFORMATION {{ FileName: {:?} }}", self.FileName)
    }
}
#[repr(C)]
pub struct FILE_OBJECTID_INFORMATION {
    pub FileReference: i64,
    pub ObjectId: [u8; 16usize],
    pub Anonymous1: FILE_OBJECTID_INFORMATION_1,
}
#[repr(C)]
pub struct FILE_OBJECTID_INFORMATION_1 {
    pub Anonymous1: UnionField<FILE_OBJECTID_INFORMATION_1_1>,
    pub ExtendedInfo: UnionField<[u8; 48usize]>,
    pub union_field: [u8; 48usize],
}
#[repr(C)]
pub struct FILE_OBJECTID_INFORMATION_1_1 {
    pub BirthVolumeId: [u8; 16usize],
    pub BirthObjectId: [u8; 16usize],
    pub DomainId: [u8; 16usize],
}
impl Default for FILE_OBJECTID_INFORMATION_1_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_OBJECTID_INFORMATION_1_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_OBJECTID_INFORMATION_1_1 {{ BirthVolumeId: {:?}, BirthObjectId: {:?}, DomainId: {:?} }}", self.BirthVolumeId, self.BirthObjectId, self.DomainId)
    }
}
impl Default for FILE_OBJECTID_INFORMATION_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_OBJECTID_INFORMATION_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_OBJECTID_INFORMATION_1 {{ union }}")
    }
}
impl Default for FILE_OBJECTID_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_OBJECTID_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_OBJECTID_INFORMATION {{ ObjectId: {:?}, Anonymous1: {:?} }}", self.ObjectId, self.Anonymous1)
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
pub struct FILE_FULL_EA_INFORMATION {
    pub NextEntryOffset: u32,
    pub Flags: u8,
    pub EaNameLength: u8,
    pub EaValueLength: u16,
    pub EaName: [i8; 1usize],
}
impl Default for FILE_FULL_EA_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_FULL_EA_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_FULL_EA_INFORMATION {{ EaName: {:?} }}", self.EaName)
    }
}
#[repr(C)]
pub struct FILE_GET_EA_INFORMATION {
    pub NextEntryOffset: u32,
    pub EaNameLength: u8,
    pub EaName: [i8; 1usize],
}
impl Default for FILE_GET_EA_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_GET_EA_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_GET_EA_INFORMATION {{ EaName: {:?} }}", self.EaName)
    }
}
#[repr(C)]
pub struct FILE_GET_QUOTA_INFORMATION {
    pub NextEntryOffset: u32,
    pub SidLength: u32,
    pub Sid: SID,
}
impl Default for FILE_GET_QUOTA_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_GET_QUOTA_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_GET_QUOTA_INFORMATION {{  }}")
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
#[repr(C)]
pub struct FILE_FS_VOLUME_INFORMATION {
    pub VolumeCreationTime: i64,
    pub VolumeSerialNumber: u32,
    pub VolumeLabelLength: u32,
    pub SupportsObjects: BOOLEAN,
    pub VolumeLabel: [u16; 1usize],
}
impl Default for FILE_FS_VOLUME_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_FS_VOLUME_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_FS_VOLUME_INFORMATION {{ VolumeLabel: {:?} }}", self.VolumeLabel)
    }
}
#[repr(C)]
pub struct FILE_FS_LABEL_INFORMATION {
    pub VolumeLabelLength: u32,
    pub VolumeLabel: [u16; 1usize],
}
impl Default for FILE_FS_LABEL_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_FS_LABEL_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_FS_LABEL_INFORMATION {{ VolumeLabel: {:?} }}", self.VolumeLabel)
    }
}
#[repr(C)]
pub struct FILE_FS_SIZE_INFORMATION {
    pub TotalAllocationUnits: i64,
    pub AvailableAllocationUnits: i64,
    pub SectorsPerAllocationUnit: u32,
    pub BytesPerSector: u32,
}
impl Default for FILE_FS_SIZE_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_FS_SIZE_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_FS_SIZE_INFORMATION {{  }}")
    }
}
#[repr(C)]
pub struct FILE_FS_CONTROL_INFORMATION {
    pub FreeSpaceStartFiltering: i64,
    pub FreeSpaceThreshold: i64,
    pub FreeSpaceStopFiltering: i64,
    pub DefaultQuotaThreshold: i64,
    pub DefaultQuotaLimit: i64,
    pub FileSystemControlFlags: u32,
}
impl Default for FILE_FS_CONTROL_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_FS_CONTROL_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_FS_CONTROL_INFORMATION {{  }}")
    }
}
#[repr(C)]
pub struct FILE_FS_FULL_SIZE_INFORMATION {
    pub TotalAllocationUnits: i64,
    pub CallerAvailableAllocationUnits: i64,
    pub ActualAvailableAllocationUnits: i64,
    pub SectorsPerAllocationUnit: u32,
    pub BytesPerSector: u32,
}
impl Default for FILE_FS_FULL_SIZE_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_FS_FULL_SIZE_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_FS_FULL_SIZE_INFORMATION {{  }}")
    }
}
#[repr(C)]
pub struct FILE_FS_OBJECTID_INFORMATION {
    pub ObjectId: [u8; 16usize],
    pub Anonymous1: FILE_FS_OBJECTID_INFORMATION_1,
}
#[repr(C)]
pub struct FILE_FS_OBJECTID_INFORMATION_1 {
    pub Anonymous1: UnionField<FILE_FS_OBJECTID_INFORMATION_1_1>,
    pub ExtendedInfo: UnionField<[u8; 48usize]>,
    pub union_field: [u8; 48usize],
}
#[repr(C)]
pub struct FILE_FS_OBJECTID_INFORMATION_1_1 {
    pub BirthVolumeId: [u8; 16usize],
    pub BirthObjectId: [u8; 16usize],
    pub DomainId: [u8; 16usize],
}
impl Default for FILE_FS_OBJECTID_INFORMATION_1_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_FS_OBJECTID_INFORMATION_1_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_FS_OBJECTID_INFORMATION_1_1 {{ BirthVolumeId: {:?}, BirthObjectId: {:?}, DomainId: {:?} }}", self.BirthVolumeId, self.BirthObjectId, self.DomainId)
    }
}
impl Default for FILE_FS_OBJECTID_INFORMATION_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_FS_OBJECTID_INFORMATION_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_FS_OBJECTID_INFORMATION_1 {{ union }}")
    }
}
impl Default for FILE_FS_OBJECTID_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_FS_OBJECTID_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_FS_OBJECTID_INFORMATION {{ ObjectId: {:?}, Anonymous1: {:?} }}", self.ObjectId, self.Anonymous1)
    }
}
#[repr(C)]
pub struct FILE_FS_DEVICE_INFORMATION {
    pub DeviceType: u32,
    pub Characteristics: u32,
}
impl Default for FILE_FS_DEVICE_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_FS_DEVICE_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_FS_DEVICE_INFORMATION {{  }}")
    }
}
#[repr(C)]
pub struct FILE_FS_ATTRIBUTE_INFORMATION {
    pub FileSystemAttributes: u32,
    pub MaximumComponentNameLength: i32,
    pub FileSystemNameLength: u32,
    pub FileSystemName: [u16; 1usize],
}
impl Default for FILE_FS_ATTRIBUTE_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_FS_ATTRIBUTE_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_FS_ATTRIBUTE_INFORMATION {{ FileSystemName: {:?} }}", self.FileSystemName)
    }
}
#[repr(C)]
pub struct FILE_FS_DRIVER_PATH_INFORMATION {
    pub DriverInPath: BOOLEAN,
    pub DriverNameLength: u32,
    pub DriverName: [u16; 1usize],
}
impl Default for FILE_FS_DRIVER_PATH_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_FS_DRIVER_PATH_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_FS_DRIVER_PATH_INFORMATION {{ DriverName: {:?} }}", self.DriverName)
    }
}
#[repr(C)]
pub struct FILE_FS_VOLUME_FLAGS_INFORMATION {
    pub Flags: u32,
}
impl Default for FILE_FS_VOLUME_FLAGS_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_FS_VOLUME_FLAGS_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_FS_VOLUME_FLAGS_INFORMATION {{  }}")
    }
}
#[repr(C)]
pub struct FILE_FS_SECTOR_SIZE_INFORMATION {
    pub LogicalBytesPerSector: u32,
    pub PhysicalBytesPerSectorForAtomicity: u32,
    pub PhysicalBytesPerSectorForPerformance: u32,
    pub FileSystemEffectivePhysicalBytesPerSectorForAtomicity: u32,
    pub Flags: u32,
    pub ByteOffsetForSectorAlignment: u32,
    pub ByteOffsetForPartitionAlignment: u32,
}
impl Default for FILE_FS_SECTOR_SIZE_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_FS_SECTOR_SIZE_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_FS_SECTOR_SIZE_INFORMATION {{  }}")
    }
}
#[repr(C)]
pub struct FILE_FS_DATA_COPY_INFORMATION {
    pub NumberOfCopies: u32,
}
impl Default for FILE_FS_DATA_COPY_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_FS_DATA_COPY_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_FS_DATA_COPY_INFORMATION {{  }}")
    }
}
#[repr(C)]
pub struct FILE_FS_METADATA_SIZE_INFORMATION {
    pub TotalMetadataAllocationUnits: i64,
    pub SectorsPerAllocationUnit: u32,
    pub BytesPerSector: u32,
}
impl Default for FILE_FS_METADATA_SIZE_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_FS_METADATA_SIZE_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_FS_METADATA_SIZE_INFORMATION {{  }}")
    }
}
#[repr(C)]
pub struct FILE_FS_FULL_SIZE_INFORMATION_EX {
    pub ActualTotalAllocationUnits: u64,
    pub ActualAvailableAllocationUnits: u64,
    pub ActualPoolUnavailableAllocationUnits: u64,
    pub CallerTotalAllocationUnits: u64,
    pub CallerAvailableAllocationUnits: u64,
    pub CallerPoolUnavailableAllocationUnits: u64,
    pub UsedAllocationUnits: u64,
    pub TotalReservedAllocationUnits: u64,
    pub VolumeStorageReserveAllocationUnits: u64,
    pub AvailableCommittedAllocationUnits: u64,
    pub PoolAvailableAllocationUnits: u64,
    pub SectorsPerAllocationUnit: u32,
    pub BytesPerSector: u32,
}
impl Default for FILE_FS_FULL_SIZE_INFORMATION_EX {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_FS_FULL_SIZE_INFORMATION_EX {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_FS_FULL_SIZE_INFORMATION_EX {{  }}")
    }
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtCreateFile(FileHandle: *mut HANDLE, DesiredAccess: u32, ObjectAttributes: *mut OBJECT_ATTRIBUTES, IoStatusBlock: *mut IO_STATUS_BLOCK, AllocationSize: *mut i64, FileAttributes: u32, ShareAccess: u32, CreateDisposition: u32, CreateOptions: u32, EaBuffer: *mut std::ffi::c_void, EaLength: u32) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtCreateNamedPipeFile(FileHandle: *mut HANDLE, DesiredAccess: u32, ObjectAttributes: *mut OBJECT_ATTRIBUTES, IoStatusBlock: *mut IO_STATUS_BLOCK, ShareAccess: u32, CreateDisposition: u32, CreateOptions: u32, NamedPipeType: u32, ReadMode: u32, CompletionMode: u32, MaximumInstances: u32, InboundQuota: u32, OutboundQuota: u32, DefaultTimeout: *mut i64) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtCreateMailslotFile(FileHandle: *mut HANDLE, DesiredAccess: u32, ObjectAttributes: *mut OBJECT_ATTRIBUTES, IoStatusBlock: *mut IO_STATUS_BLOCK, CreateOptions: u32, MailslotQuota: u32, MaximumMessageSize: u32, ReadTimeout: *mut i64) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtOpenFile(FileHandle: *mut HANDLE, DesiredAccess: u32, ObjectAttributes: *mut OBJECT_ATTRIBUTES, IoStatusBlock: *mut IO_STATUS_BLOCK, ShareAccess: u32, OpenOptions: u32) -> NTSTATUS;
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
    pub fn NtFlushBuffersFileEx(FileHandle: HANDLE, Flags: u32, Parameters: *mut std::ffi::c_void, ParametersSize: u32, IoStatusBlock: *mut IO_STATUS_BLOCK) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtQueryInformationFile(FileHandle: HANDLE, IoStatusBlock: *mut IO_STATUS_BLOCK, FileInformation: *mut std::ffi::c_void, Length: u32, FileInformationClass: FILE_INFORMATION_CLASS) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtQueryInformationByName(ObjectAttributes: *mut OBJECT_ATTRIBUTES, IoStatusBlock: *mut IO_STATUS_BLOCK, FileInformation: *mut std::ffi::c_void, Length: u32, FileInformationClass: FILE_INFORMATION_CLASS) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtSetInformationFile(FileHandle: HANDLE, IoStatusBlock: *mut IO_STATUS_BLOCK, FileInformation: *mut std::ffi::c_void, Length: u32, FileInformationClass: FILE_INFORMATION_CLASS) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtQueryDirectoryFile(FileHandle: HANDLE, Event: HANDLE, ApcRoutine: PIO_APC_ROUTINE, ApcContext: *mut std::ffi::c_void, IoStatusBlock: *mut IO_STATUS_BLOCK, FileInformation: *mut std::ffi::c_void, Length: u32, FileInformationClass: FILE_INFORMATION_CLASS, ReturnSingleEntry: BOOLEAN, FileName: *mut UNICODE_STRING, RestartScan: BOOLEAN) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtQueryDirectoryFileEx(FileHandle: HANDLE, Event: HANDLE, ApcRoutine: PIO_APC_ROUTINE, ApcContext: *mut std::ffi::c_void, IoStatusBlock: *mut IO_STATUS_BLOCK, FileInformation: *mut std::ffi::c_void, Length: u32, FileInformationClass: FILE_INFORMATION_CLASS, QueryFlags: u32, FileName: *mut UNICODE_STRING) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtQueryEaFile(FileHandle: HANDLE, IoStatusBlock: *mut IO_STATUS_BLOCK, Buffer: *mut std::ffi::c_void, Length: u32, ReturnSingleEntry: BOOLEAN, EaList: *mut std::ffi::c_void, EaListLength: u32, EaIndex: *mut u32, RestartScan: BOOLEAN) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtSetEaFile(FileHandle: HANDLE, IoStatusBlock: *mut IO_STATUS_BLOCK, Buffer: *mut std::ffi::c_void, Length: u32) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtQueryQuotaInformationFile(FileHandle: HANDLE, IoStatusBlock: *mut IO_STATUS_BLOCK, Buffer: *mut std::ffi::c_void, Length: u32, ReturnSingleEntry: BOOLEAN, SidList: *mut std::ffi::c_void, SidListLength: u32, StartSid: PSID, RestartScan: BOOLEAN) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtSetQuotaInformationFile(FileHandle: HANDLE, IoStatusBlock: *mut IO_STATUS_BLOCK, Buffer: *mut std::ffi::c_void, Length: u32) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtQueryVolumeInformationFile(FileHandle: HANDLE, IoStatusBlock: *mut IO_STATUS_BLOCK, FsInformation: *mut std::ffi::c_void, Length: u32, FsInformationClass: FSINFOCLASS) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtSetVolumeInformationFile(FileHandle: HANDLE, IoStatusBlock: *mut IO_STATUS_BLOCK, FsInformation: *mut std::ffi::c_void, Length: u32, FsInformationClass: FSINFOCLASS) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtCancelIoFile(FileHandle: HANDLE, IoStatusBlock: *mut IO_STATUS_BLOCK) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtCancelIoFileEx(FileHandle: HANDLE, IoRequestToCancel: *mut IO_STATUS_BLOCK, IoStatusBlock: *mut IO_STATUS_BLOCK) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtCancelSynchronousIoFile(ThreadHandle: HANDLE, IoRequestToCancel: *mut IO_STATUS_BLOCK, IoStatusBlock: *mut IO_STATUS_BLOCK) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtDeviceIoControlFile(FileHandle: HANDLE, Event: HANDLE, ApcRoutine: PIO_APC_ROUTINE, ApcContext: *mut std::ffi::c_void, IoStatusBlock: *mut IO_STATUS_BLOCK, IoControlCode: u32, InputBuffer: *mut std::ffi::c_void, InputBufferLength: u32, OutputBuffer: *mut std::ffi::c_void, OutputBufferLength: u32) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtFsControlFile(FileHandle: HANDLE, Event: HANDLE, ApcRoutine: PIO_APC_ROUTINE, ApcContext: *mut std::ffi::c_void, IoStatusBlock: *mut IO_STATUS_BLOCK, FsControlCode: u32, InputBuffer: *mut std::ffi::c_void, InputBufferLength: u32, OutputBuffer: *mut std::ffi::c_void, OutputBufferLength: u32) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtReadFile(FileHandle: HANDLE, Event: HANDLE, ApcRoutine: PIO_APC_ROUTINE, ApcContext: *mut std::ffi::c_void, IoStatusBlock: *mut IO_STATUS_BLOCK, Buffer: *mut std::ffi::c_void, Length: u32, ByteOffset: *mut i64, Key: *mut u32) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtWriteFile(FileHandle: HANDLE, Event: HANDLE, ApcRoutine: PIO_APC_ROUTINE, ApcContext: *mut std::ffi::c_void, IoStatusBlock: *mut IO_STATUS_BLOCK, Buffer: *mut std::ffi::c_void, Length: u32, ByteOffset: *mut i64, Key: *mut u32) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtReadFileScatter(FileHandle: HANDLE, Event: HANDLE, ApcRoutine: PIO_APC_ROUTINE, ApcContext: *mut std::ffi::c_void, IoStatusBlock: *mut IO_STATUS_BLOCK, SegmentArray: *mut FILE_SEGMENT_ELEMENT, Length: u32, ByteOffset: *mut i64, Key: *mut u32) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtWriteFileGather(FileHandle: HANDLE, Event: HANDLE, ApcRoutine: PIO_APC_ROUTINE, ApcContext: *mut std::ffi::c_void, IoStatusBlock: *mut IO_STATUS_BLOCK, SegmentArray: *mut FILE_SEGMENT_ELEMENT, Length: u32, ByteOffset: *mut i64, Key: *mut u32) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtLockFile(FileHandle: HANDLE, Event: HANDLE, ApcRoutine: PIO_APC_ROUTINE, ApcContext: *mut std::ffi::c_void, IoStatusBlock: *mut IO_STATUS_BLOCK, ByteOffset: *mut i64, Length: *mut i64, Key: u32, FailImmediately: BOOLEAN, ExclusiveLock: BOOLEAN) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtUnlockFile(FileHandle: HANDLE, IoStatusBlock: *mut IO_STATUS_BLOCK, ByteOffset: *mut i64, Length: *mut i64, Key: u32) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtQueryAttributesFile(ObjectAttributes: *mut OBJECT_ATTRIBUTES, FileInformation: *mut FILE_BASIC_INFORMATION) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtQueryFullAttributesFile(ObjectAttributes: *mut OBJECT_ATTRIBUTES, FileInformation: *mut FILE_NETWORK_OPEN_INFORMATION) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtNotifyChangeDirectoryFile(FileHandle: HANDLE, Event: HANDLE, ApcRoutine: PIO_APC_ROUTINE, ApcContext: *mut std::ffi::c_void, IoStatusBlock: *mut IO_STATUS_BLOCK, Buffer: *mut std::ffi::c_void, Length: u32, CompletionFilter: u32, WatchTree: BOOLEAN) -> NTSTATUS;
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum DIRECTORY_NOTIFY_INFORMATION_CLASS {
    DirectoryNotifyInformation = 1,
    DirectoryNotifyExtendedInformation = 2,
    DirectoryNotifyFullInformation = 3,
    DirectoryNotifyMaximumInformation = 4,
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtNotifyChangeDirectoryFileEx(FileHandle: HANDLE, Event: HANDLE, ApcRoutine: PIO_APC_ROUTINE, ApcContext: *mut std::ffi::c_void, IoStatusBlock: *mut IO_STATUS_BLOCK, Buffer: *mut std::ffi::c_void, Length: u32, CompletionFilter: u32, WatchTree: BOOLEAN, DirectoryNotifyInformationClass: DIRECTORY_NOTIFY_INFORMATION_CLASS) -> NTSTATUS;
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
    pub fn NtCreateIoCompletion(IoCompletionHandle: *mut HANDLE, DesiredAccess: u32, ObjectAttributes: *mut OBJECT_ATTRIBUTES, Count: u32) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtOpenIoCompletion(IoCompletionHandle: *mut HANDLE, DesiredAccess: u32, ObjectAttributes: *mut OBJECT_ATTRIBUTES) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtQueryIoCompletion(IoCompletionHandle: HANDLE, IoCompletionInformationClass: IO_COMPLETION_INFORMATION_CLASS, IoCompletionInformation: *mut std::ffi::c_void, IoCompletionInformationLength: u32, ReturnLength: *mut u32) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtSetIoCompletion(IoCompletionHandle: HANDLE, KeyContext: *mut std::ffi::c_void, ApcContext: *mut std::ffi::c_void, IoStatus: NTSTATUS, IoStatusInformation: usize) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtSetIoCompletionEx(IoCompletionHandle: HANDLE, IoCompletionPacketHandle: HANDLE, KeyContext: *mut std::ffi::c_void, ApcContext: *mut std::ffi::c_void, IoStatus: NTSTATUS, IoStatusInformation: usize) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtRemoveIoCompletion(IoCompletionHandle: HANDLE, KeyContext: *mut *mut std::ffi::c_void, ApcContext: *mut *mut std::ffi::c_void, IoStatusBlock: *mut IO_STATUS_BLOCK, Timeout: *mut i64) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtRemoveIoCompletionEx(IoCompletionHandle: HANDLE, IoCompletionInformation: *mut FILE_IO_COMPLETION_INFORMATION, Count: u32, NumEntriesRemoved: *mut u32, Timeout: *mut i64, Alertable: BOOLEAN) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtCreateWaitCompletionPacket(WaitCompletionPacketHandle: *mut HANDLE, DesiredAccess: u32, ObjectAttributes: *mut OBJECT_ATTRIBUTES) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtAssociateWaitCompletionPacket(WaitCompletionPacketHandle: HANDLE, IoCompletionHandle: HANDLE, TargetObjectHandle: HANDLE, KeyContext: *mut std::ffi::c_void, ApcContext: *mut std::ffi::c_void, IoStatus: NTSTATUS, IoStatusInformation: usize, AlreadySignaled: *mut BOOLEAN) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtCancelWaitCompletionPacket(WaitCompletionPacketHandle: HANDLE, RemoveSignaledPacket: BOOLEAN) -> NTSTATUS;
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum IO_SESSION_EVENT {
    IoSessionEventIgnore = 0,
    IoSessionEventCreated = 1,
    IoSessionEventTerminated = 2,
    IoSessionEventConnected = 3,
    IoSessionEventDisconnected = 4,
    IoSessionEventLogon = 5,
    IoSessionEventLogoff = 6,
    IoSessionEventMax = 7,
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum IO_SESSION_STATE {
    IoSessionStateCreated = 1,
    IoSessionStateInitialized = 2,
    IoSessionStateConnected = 3,
    IoSessionStateDisconnected = 4,
    IoSessionStateDisconnectedLoggedOn = 5,
    IoSessionStateLoggedOn = 6,
    IoSessionStateLoggedOff = 7,
    IoSessionStateTerminated = 8,
    IoSessionStateMax = 9,
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtOpenSession(SessionHandle: *mut HANDLE, DesiredAccess: u32, ObjectAttributes: *mut OBJECT_ATTRIBUTES) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtNotifyChangeSession(SessionHandle: HANDLE, ChangeSequenceNumber: u32, ChangeTimeStamp: *mut i64, Event: IO_SESSION_EVENT, NewState: IO_SESSION_STATE, PreviousState: IO_SESSION_STATE, Payload: *mut std::ffi::c_void, PayloadSize: u32) -> NTSTATUS;
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum INTERFACE_TYPE {
    InterfaceTypeUndefined = -1,
    Internal = 0,
    Isa = 1,
    Eisa = 2,
    MicroChannel = 3,
    TurboChannel = 4,
    PCIBus = 5,
    VMEBus = 6,
    NuBus = 7,
    PCMCIABus = 8,
    CBus = 9,
    MPIBus = 10,
    MPSABus = 11,
    ProcessorInternal = 12,
    InternalPowerBus = 13,
    PNPISABus = 14,
    PNPBus = 15,
    Vmcs = 16,
    ACPIBus = 17,
    MaximumInterfaceType = 18,
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum DMA_WIDTH {
    Width8Bits = 0,
    Width16Bits = 1,
    Width32Bits = 2,
    Width64Bits = 3,
    WidthNoWrap = 4,
    MaximumDmaWidth = 5,
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum DMA_SPEED {
    Compatible = 0,
    TypeA = 1,
    TypeB = 2,
    TypeC = 3,
    TypeF = 4,
    MaximumDmaSpeed = 5,
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum BUS_DATA_TYPE {
    ConfigurationSpaceUndefined = -1,
    Cmos = 0,
    EisaConfiguration = 1,
    Pos = 2,
    CbusConfiguration = 3,
    PCIConfiguration = 4,
    VMEConfiguration = 5,
    NuBusConfiguration = 6,
    PCMCIAConfiguration = 7,
    MPIConfiguration = 8,
    MPSAConfiguration = 9,
    PNPISAConfiguration = 10,
    SgiInternalConfiguration = 11,
    MaximumBusDataType = 12,
}
#[repr(C)]
pub struct REPARSE_DATA_BUFFER {
    pub ReparseTag: u32,
    pub ReparseDataLength: u16,
    pub Reserved: u16,
    pub Anonymous1: REPARSE_DATA_BUFFER_1,
}
#[repr(C)]
pub struct REPARSE_DATA_BUFFER_1 {
    pub SymbolicLinkReparseBuffer: UnionField<REPARSE_DATA_BUFFER_1_1>,
    pub MountPointReparseBuffer: UnionField<REPARSE_DATA_BUFFER_1_2>,
    pub GenericReparseBuffer: UnionField<REPARSE_DATA_BUFFER_1_3>,
    pub union_field: [u32; 4usize],
}
#[repr(C)]
pub struct REPARSE_DATA_BUFFER_1_1 {
    pub SubstituteNameOffset: u16,
    pub SubstituteNameLength: u16,
    pub PrintNameOffset: u16,
    pub PrintNameLength: u16,
    pub Flags: u32,
    pub PathBuffer: [u16; 1usize],
}
impl Default for REPARSE_DATA_BUFFER_1_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for REPARSE_DATA_BUFFER_1_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "REPARSE_DATA_BUFFER_1_1 {{ PathBuffer: {:?} }}", self.PathBuffer)
    }
}
#[repr(C)]
pub struct REPARSE_DATA_BUFFER_1_2 {
    pub SubstituteNameOffset: u16,
    pub SubstituteNameLength: u16,
    pub PrintNameOffset: u16,
    pub PrintNameLength: u16,
    pub PathBuffer: [u16; 1usize],
}
impl Default for REPARSE_DATA_BUFFER_1_2 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for REPARSE_DATA_BUFFER_1_2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "REPARSE_DATA_BUFFER_1_2 {{ PathBuffer: {:?} }}", self.PathBuffer)
    }
}
#[repr(C)]
pub struct REPARSE_DATA_BUFFER_1_3 {
    pub DataBuffer: [u8; 1usize],
}
impl Default for REPARSE_DATA_BUFFER_1_3 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for REPARSE_DATA_BUFFER_1_3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "REPARSE_DATA_BUFFER_1_3 {{ DataBuffer: {:?} }}", self.DataBuffer)
    }
}
impl Default for REPARSE_DATA_BUFFER_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for REPARSE_DATA_BUFFER_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "REPARSE_DATA_BUFFER_1 {{ union }}")
    }
}
impl Default for REPARSE_DATA_BUFFER {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for REPARSE_DATA_BUFFER {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "REPARSE_DATA_BUFFER {{ Anonymous1: {:?} }}", self.Anonymous1)
    }
}
#[repr(C)]
pub struct FILE_PIPE_ASSIGN_EVENT_BUFFER {
    pub EventHandle: HANDLE,
    pub KeyValue: u32,
}
impl Default for FILE_PIPE_ASSIGN_EVENT_BUFFER {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_PIPE_ASSIGN_EVENT_BUFFER {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_PIPE_ASSIGN_EVENT_BUFFER {{  }}")
    }
}
#[repr(C)]
pub struct FILE_PIPE_PEEK_BUFFER {
    pub NamedPipeState: u32,
    pub ReadDataAvailable: u32,
    pub NumberOfMessages: u32,
    pub MessageLength: u32,
    pub Data: [i8; 1usize],
}
impl Default for FILE_PIPE_PEEK_BUFFER {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_PIPE_PEEK_BUFFER {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_PIPE_PEEK_BUFFER {{ Data: {:?} }}", self.Data)
    }
}
#[repr(C)]
pub struct FILE_PIPE_EVENT_BUFFER {
    pub NamedPipeState: u32,
    pub EntryType: u32,
    pub ByteCount: u32,
    pub KeyValue: u32,
    pub NumberRequests: u32,
}
impl Default for FILE_PIPE_EVENT_BUFFER {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_PIPE_EVENT_BUFFER {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_PIPE_EVENT_BUFFER {{  }}")
    }
}
#[repr(C)]
pub struct FILE_PIPE_WAIT_FOR_BUFFER {
    pub Timeout: i64,
    pub NameLength: u32,
    pub TimeoutSpecified: BOOLEAN,
    pub Name: [u16; 1usize],
}
impl Default for FILE_PIPE_WAIT_FOR_BUFFER {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_PIPE_WAIT_FOR_BUFFER {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_PIPE_WAIT_FOR_BUFFER {{ Name: {:?} }}", self.Name)
    }
}
#[repr(C)]
pub struct FILE_PIPE_CLIENT_PROCESS_BUFFER {
    pub ClientSession: *mut std::ffi::c_void,
    pub ClientProcess: *mut std::ffi::c_void,
}
impl Default for FILE_PIPE_CLIENT_PROCESS_BUFFER {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_PIPE_CLIENT_PROCESS_BUFFER {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_PIPE_CLIENT_PROCESS_BUFFER {{  }}")
    }
}
#[repr(C)]
pub struct FILE_PIPE_CLIENT_PROCESS_BUFFER_V2 {
    pub ClientSession: u64,
    pub ClientProcess: *mut std::ffi::c_void,
}
impl Default for FILE_PIPE_CLIENT_PROCESS_BUFFER_V2 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_PIPE_CLIENT_PROCESS_BUFFER_V2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_PIPE_CLIENT_PROCESS_BUFFER_V2 {{  }}")
    }
}
#[repr(C)]
pub struct FILE_PIPE_CLIENT_PROCESS_BUFFER_EX {
    pub ClientSession: *mut std::ffi::c_void,
    pub ClientProcess: *mut std::ffi::c_void,
    pub ClientComputerNameLength: u16,
    pub ClientComputerBuffer: [u16; 16usize],
}
impl Default for FILE_PIPE_CLIENT_PROCESS_BUFFER_EX {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_PIPE_CLIENT_PROCESS_BUFFER_EX {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_PIPE_CLIENT_PROCESS_BUFFER_EX {{ ClientComputerBuffer: {:?} }}", self.ClientComputerBuffer)
    }
}
#[repr(C)]
pub struct FILE_PIPE_SILO_ARRIVAL_INPUT {
    pub JobHandle: HANDLE,
}
impl Default for FILE_PIPE_SILO_ARRIVAL_INPUT {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_PIPE_SILO_ARRIVAL_INPUT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_PIPE_SILO_ARRIVAL_INPUT {{  }}")
    }
}
#[repr(C)]
pub struct FILE_PIPE_CREATE_SYMLINK_INPUT {
    pub NameOffset: u16,
    pub NameLength: u16,
    pub SubstituteNameOffset: u16,
    pub SubstituteNameLength: u16,
    pub Flags: u32,
}
impl Default for FILE_PIPE_CREATE_SYMLINK_INPUT {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_PIPE_CREATE_SYMLINK_INPUT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_PIPE_CREATE_SYMLINK_INPUT {{  }}")
    }
}
#[repr(C)]
pub struct FILE_PIPE_DELETE_SYMLINK_INPUT {
    pub NameOffset: u16,
    pub NameLength: u16,
}
impl Default for FILE_PIPE_DELETE_SYMLINK_INPUT {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for FILE_PIPE_DELETE_SYMLINK_INPUT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILE_PIPE_DELETE_SYMLINK_INPUT {{  }}")
    }
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
    pub MountPoints: [MOUNTMGR_MOUNT_POINT; 1usize],
}
impl Default for MOUNTMGR_MOUNT_POINTS {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for MOUNTMGR_MOUNT_POINTS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MOUNTMGR_MOUNT_POINTS {{ MountPoints: {:?} }}", self.MountPoints)
    }
}
#[repr(C)]
pub struct MOUNTMGR_DRIVE_LETTER_TARGET {
    pub DeviceNameLength: u16,
    pub DeviceName: [u16; 1usize],
}
impl Default for MOUNTMGR_DRIVE_LETTER_TARGET {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for MOUNTMGR_DRIVE_LETTER_TARGET {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MOUNTMGR_DRIVE_LETTER_TARGET {{ DeviceName: {:?} }}", self.DeviceName)
    }
}
#[repr(C)]
pub struct MOUNTMGR_DRIVE_LETTER_INFORMATION {
    pub DriveLetterWasAssigned: BOOLEAN,
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
    pub DeviceName: [u16; 1usize],
}
impl Default for MOUNTMGR_TARGET_NAME {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for MOUNTMGR_TARGET_NAME {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MOUNTMGR_TARGET_NAME {{ DeviceName: {:?} }}", self.DeviceName)
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
    pub Name: [u16; 1usize],
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
    pub MultiSz: [u16; 1usize],
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
