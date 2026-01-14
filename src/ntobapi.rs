use windows::{
    Wdk::Foundation::{OBJECT_ATTRIBUTES, OBJECT_INFORMATION_CLASS},
    Win32::{
        Foundation::{HANDLE, NTSTATUS, UNICODE_STRING},
        Security::GENERIC_MAPPING,
        System::Kernel::WAIT_TYPE,
    },
};

use crate::bitfield::{BitfieldUnit, UnionField};

pub const OBJ_PROTECT_CLOSE: u32 = 1;
pub const OBJ_AUDIT_OBJECT_CLOSE: u32 = 4;
pub const OBJECT_TYPE_ALL_ACCESS: u32 = 983041;
pub const DIRECTORY_ALL_ACCESS: u32 = 983055;
pub const SYMBOLIC_LINK_ALL_ACCESS: u32 = 983041;
pub const SYMBOLIC_LINK_ALL_ACCESS_EX: u32 = 1048575;
pub const OBJECT_BOUNDARY_DESCRIPTOR_VERSION: u32 = 1;
pub const ObjectNameInformation: u32 = 1;
pub const ObjectTypesInformation: u32 = 3;
pub const ObjectHandleFlagInformation: u32 = 4;
pub const ObjectSessionInformation: u32 = 5;
pub const ObjectSessionObjectInformation: u32 = 6;

#[repr(C)]
pub struct OBJECT_BASIC_INFORMATION {
    pub Attributes: u32,
    pub GrantedAccess: u32,
    pub HandleCount: u32,
    pub PointerCount: u32,
    pub PagedPoolCharge: u32,
    pub NonPagedPoolCharge: u32,
    pub Reserved: [u32; 3],
    pub NameInfoSize: u32,
    pub TypeInfoSize: u32,
    pub SecurityDescriptorSize: u32,
    pub CreationTime: i64,
}

impl Default for OBJECT_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for OBJECT_BASIC_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "OBJECT_BASIC_INFORMATION {{ Reserved: {:?} }}",
            self.Reserved
        )
    }
}

#[repr(C)]
pub struct OBJECT_TYPE_INFORMATION {
    pub TypeName: UNICODE_STRING,
    pub TotalNumberOfObjects: u32,
    pub TotalNumberOfHandles: u32,
    pub TotalPagedPoolUsage: u32,
    pub TotalNonPagedPoolUsage: u32,
    pub TotalNamePoolUsage: u32,
    pub TotalHandleTableUsage: u32,
    pub HighWaterNumberOfObjects: u32,
    pub HighWaterNumberOfHandles: u32,
    pub HighWaterPagedPoolUsage: u32,
    pub HighWaterNonPagedPoolUsage: u32,
    pub HighWaterNamePoolUsage: u32,
    pub HighWaterHandleTableUsage: u32,
    pub InvalidAttributes: u32,
    pub GenericMapping: GENERIC_MAPPING,
    pub ValidAccessMask: u32,
    pub SecurityRequired: bool,
    pub MaintainHandleCount: bool,
    pub TypeIndex: u8,
    pub ReservedByte: i8,
    pub PoolType: u32,
    pub DefaultPagedPoolCharge: u32,
    pub DefaultNonPagedPoolCharge: u32,
}

impl Default for OBJECT_TYPE_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for OBJECT_TYPE_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OBJECT_TYPE_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct OBJECT_TYPES_INFORMATION {
    pub NumberOfTypes: u32,
}

impl Default for OBJECT_TYPES_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for OBJECT_TYPES_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OBJECT_TYPES_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct OBJECT_HANDLE_FLAG_INFORMATION {
    pub Inherit: bool,
    pub ProtectFromClose: bool,
}

impl Default for OBJECT_HANDLE_FLAG_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for OBJECT_HANDLE_FLAG_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OBJECT_HANDLE_FLAG_INFORMATION {{  }}")
    }
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtSetInformationObject(
        Handle: HANDLE,
        ObjectInformationClass: OBJECT_INFORMATION_CLASS,
        ObjectInformation: *mut std::ffi::c_void,
        ObjectInformationLength: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtDuplicateObject(
        SourceProcessHandle: HANDLE,
        SourceHandle: HANDLE,
        TargetProcessHandle: HANDLE,
        TargetHandle: *mut HANDLE,
        DesiredAccess: u32,
        HandleAttributes: u32,
        Options: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtMakeTemporaryObject(Handle: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtMakePermanentObject(Handle: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtSignalAndWaitForSingleObject(
        SignalHandle: HANDLE,
        WaitHandle: HANDLE,
        Alertable: bool,
        Timeout: *mut i64,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtWaitForMultipleObjects(
        Count: u32,
        Handles: *mut HANDLE,
        WaitType: WAIT_TYPE,
        Alertable: bool,
        Timeout: *mut i64,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtWaitForMultipleObjects32(
        Count: u32,
        Handles: *mut i32,
        WaitType: WAIT_TYPE,
        Alertable: bool,
        Timeout: *mut i64,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtCompareObjects(FirstObjectHandle: HANDLE, SecondObjectHandle: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtCreateDirectoryObject(
        DirectoryHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtCreateDirectoryObjectEx(
        DirectoryHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        ShadowDirectoryHandle: HANDLE,
        Flags: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtOpenDirectoryObject(
        DirectoryHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}

#[repr(C)]
pub struct OBJECT_DIRECTORY_INFORMATION {
    pub Name: UNICODE_STRING,
    pub TypeName: UNICODE_STRING,
}

impl Default for OBJECT_DIRECTORY_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for OBJECT_DIRECTORY_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OBJECT_DIRECTORY_INFORMATION {{  }}")
    }
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtQueryDirectoryObject(
        DirectoryHandle: HANDLE,
        Buffer: *mut std::ffi::c_void,
        Length: u32,
        ReturnSingleEntry: bool,
        RestartScan: bool,
        Context: *mut u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum BOUNDARY_ENTRY_TYPE {
    OBNS_Invalid = 0,
    OBNS_Name = 1,
    OBNS_SID = 2,
    OBNS_IL = 3,
}

#[repr(C)]
pub struct OBJECT_BOUNDARY_ENTRY {
    pub EntryType: BOUNDARY_ENTRY_TYPE,
    pub EntrySize: u32,
}

impl Default for OBJECT_BOUNDARY_ENTRY {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for OBJECT_BOUNDARY_ENTRY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "OBJECT_BOUNDARY_ENTRY {{ EntryType: {:?} }}",
            self.EntryType
        )
    }
}

#[repr(C)]
pub struct OBJECT_BOUNDARY_DESCRIPTOR {
    pub Version: u32,
    pub Items: u32,
    pub TotalSize: u32,
    pub Anonymous1: OBJECT_BOUNDARY_DESCRIPTOR_1,
}

#[repr(C)]
pub struct OBJECT_BOUNDARY_DESCRIPTOR_1 {
    pub Flags: UnionField<u32>,
    pub Anonymous1: UnionField<OBJECT_BOUNDARY_DESCRIPTOR_1_1>,
    pub union_field: u32,
}

#[repr(C)]
#[repr(align(4))]
pub struct OBJECT_BOUNDARY_DESCRIPTOR_1_1 {
    _bitfield_align_1: [u32; 0],
    _bitfield_1: BitfieldUnit<[u8; 4]>,
}

impl Default for OBJECT_BOUNDARY_DESCRIPTOR_1_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for OBJECT_BOUNDARY_DESCRIPTOR_1_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "OBJECT_BOUNDARY_DESCRIPTOR_1_1 {{ AddAppContainerSid : {:?}, Reserved : {:?} }}",
            self.AddAppContainerSid(),
            self.Reserved()
        )
    }
}

impl OBJECT_BOUNDARY_DESCRIPTOR_1_1 {
    #[inline]
    pub fn AddAppContainerSid(&self) -> u32 {
        self._bitfield_1.get(0usize, 1u8) as u32
    }

    #[inline]
    pub fn set_AddAppContainerSid(&mut self, val: u32) {
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
    pub fn new_bitfield_1(AddAppContainerSid: u32, Reserved: u32) -> BitfieldUnit<[u8; 4]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 4]> = Default::default();

        bitfield_unit.set(0usize, 1u8, AddAppContainerSid as u64);

        bitfield_unit.set(1usize, 31u8, Reserved as u64);

        bitfield_unit
    }
}

impl Default for OBJECT_BOUNDARY_DESCRIPTOR_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for OBJECT_BOUNDARY_DESCRIPTOR_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OBJECT_BOUNDARY_DESCRIPTOR_1 {{ union }}")
    }
}

impl Default for OBJECT_BOUNDARY_DESCRIPTOR {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for OBJECT_BOUNDARY_DESCRIPTOR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "OBJECT_BOUNDARY_DESCRIPTOR {{ Anonymous1: {:?} }}",
            self.Anonymous1
        )
    }
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtCreatePrivateNamespace(
        NamespaceHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        BoundaryDescriptor: *mut OBJECT_BOUNDARY_DESCRIPTOR,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtOpenPrivateNamespace(
        NamespaceHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        BoundaryDescriptor: *mut OBJECT_BOUNDARY_DESCRIPTOR,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtDeletePrivateNamespace(NamespaceHandle: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtCreateSymbolicLinkObject(
        LinkHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        LinkTarget: *mut UNICODE_STRING,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtOpenSymbolicLinkObject(
        LinkHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtQuerySymbolicLinkObject(
        LinkHandle: HANDLE,
        LinkTarget: *mut UNICODE_STRING,
        ReturnedLength: *mut u32,
    ) -> NTSTATUS;
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SYMBOLIC_LINK_INFO_CLASS {
    SymbolicLinkGlobalInformation = 1,
    SymbolicLinkAccessMask = 2,
    MaxnSymbolicLinkInfoClass = 3,
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtSetInformationSymbolicLink(
        LinkHandle: HANDLE,
        SymbolicLinkInformationClass: SYMBOLIC_LINK_INFO_CLASS,
        SymbolicLinkInformation: *mut std::ffi::c_void,
        SymbolicLinkInformationLength: u32,
    ) -> NTSTATUS;
}
