use windows::{
    Wdk::Foundation::OBJECT_ATTRIBUTES,
    Win32::{
        Foundation::{HANDLE, LUID, NTSTATUS, UNICODE_STRING},
        Security::{
            GENERIC_MAPPING, OBJECT_TYPE_LIST, PRIVILEGE_SET, PSID, SECURITY_DESCRIPTOR,
            SID_AND_ATTRIBUTES, TOKEN_DEFAULT_DACL, TOKEN_GROUPS, TOKEN_MANDATORY_POLICY,
            TOKEN_OWNER, TOKEN_PRIMARY_GROUP, TOKEN_PRIVILEGES, TOKEN_SOURCE, TOKEN_TYPE,
            TOKEN_USER,
        },
    },
};

use crate::bitfield::UnionField;

pub const TOKEN_SECURITY_ATTRIBUTE_TYPE_INVALID: u32 = 0;
pub const TOKEN_SECURITY_ATTRIBUTE_TYPE_INT64: u32 = 1;
pub const TOKEN_SECURITY_ATTRIBUTE_TYPE_UINT64: u32 = 2;
pub const TOKEN_SECURITY_ATTRIBUTE_TYPE_STRING: u32 = 3;
pub const TOKEN_SECURITY_ATTRIBUTE_TYPE_FQBN: u32 = 4;
pub const TOKEN_SECURITY_ATTRIBUTE_TYPE_SID: u32 = 5;
pub const TOKEN_SECURITY_ATTRIBUTE_TYPE_BOOLEAN: u32 = 6;
pub const TOKEN_SECURITY_ATTRIBUTE_TYPE_OCTET_STRING: u32 = 16;
pub const TOKEN_SECURITY_ATTRIBUTE_NON_INHERITABLE: u32 = 1;
pub const TOKEN_SECURITY_ATTRIBUTE_VALUE_CASE_SENSITIVE: u32 = 2;
pub const TOKEN_SECURITY_ATTRIBUTE_USE_FOR_DENY_ONLY: u32 = 4;
pub const TOKEN_SECURITY_ATTRIBUTE_DISABLED_BY_DEFAULT: u32 = 8;
pub const TOKEN_SECURITY_ATTRIBUTE_DISABLED: u32 = 16;
pub const TOKEN_SECURITY_ATTRIBUTE_MANDATORY: u32 = 32;
pub const TOKEN_SECURITY_ATTRIBUTE_COMPARE_IGNORE: u32 = 64;
pub const TOKEN_SECURITY_ATTRIBUTE_VALID_FLAGS: u32 = 63;
pub const TOKEN_SECURITY_ATTRIBUTE_CUSTOM_FLAGS: u32 = 4294901760;
pub const TOKEN_SECURITY_ATTRIBUTES_INFORMATION_VERSION_V1: u32 = 1;
pub const TOKEN_SECURITY_ATTRIBUTES_INFORMATION_VERSION: u32 = 1;

#[repr(C)]
pub struct TOKEN_SECURITY_ATTRIBUTE_FQBN_VALUE {
    pub Version: u64,
    pub Name: UNICODE_STRING,
}

impl Default for TOKEN_SECURITY_ATTRIBUTE_FQBN_VALUE {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for TOKEN_SECURITY_ATTRIBUTE_FQBN_VALUE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TOKEN_SECURITY_ATTRIBUTE_FQBN_VALUE {{  }}")
    }
}

#[repr(C)]
pub struct TOKEN_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    pub pValue: *mut std::ffi::c_void,
    pub ValueLength: u32,
}

impl Default for TOKEN_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for TOKEN_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TOKEN_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {{  }}")
    }
}

#[repr(C)]
pub struct TOKEN_SECURITY_ATTRIBUTE_V1 {
    pub Name: UNICODE_STRING,
    pub ValueType: u16,
    pub Reserved: u16,
    pub Flags: u32,
    pub ValueCount: u32,
    pub Values: TOKEN_SECURITY_ATTRIBUTE_V1_1,
}

#[repr(C)]
pub struct TOKEN_SECURITY_ATTRIBUTE_V1_1 {
    pub pInt64: UnionField<*mut i64>,
    pub pUint64: UnionField<*mut u64>,
    pub pString: UnionField<*mut UNICODE_STRING>,
    pub pFqbn: UnionField<*mut TOKEN_SECURITY_ATTRIBUTE_FQBN_VALUE>,
    pub pOctetString: UnionField<*mut TOKEN_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE>,
    pub union_field: u64,
}

impl Default for TOKEN_SECURITY_ATTRIBUTE_V1_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for TOKEN_SECURITY_ATTRIBUTE_V1_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TOKEN_SECURITY_ATTRIBUTE_V1_1 {{ union }}")
    }
}

impl Default for TOKEN_SECURITY_ATTRIBUTE_V1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for TOKEN_SECURITY_ATTRIBUTE_V1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TOKEN_SECURITY_ATTRIBUTE_V1 {{ Values: {:?} }}",
            self.Values
        )
    }
}

#[repr(C)]
pub struct TOKEN_SECURITY_ATTRIBUTES_INFORMATION {
    pub Version: u16,
    pub Reserved: u16,
    pub AttributeCount: u32,
    pub Attribute: TOKEN_SECURITY_ATTRIBUTES_INFORMATION_1,
}

#[repr(C)]
#[derive(Copy, Clone)]

pub union TOKEN_SECURITY_ATTRIBUTES_INFORMATION_1 {
    pub pAttributeV1: *mut TOKEN_SECURITY_ATTRIBUTE_V1,
}

impl Default for TOKEN_SECURITY_ATTRIBUTES_INFORMATION_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for TOKEN_SECURITY_ATTRIBUTES_INFORMATION_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TOKEN_SECURITY_ATTRIBUTES_INFORMATION_1 {{ union }}")
    }
}

impl Default for TOKEN_SECURITY_ATTRIBUTES_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for TOKEN_SECURITY_ATTRIBUTES_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TOKEN_SECURITY_ATTRIBUTES_INFORMATION {{ Attribute: {:?} }}",
            self.Attribute
        )
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum TOKEN_SECURITY_ATTRIBUTE_OPERATION {
    TOKEN_SECURITY_ATTRIBUTE_OPERATION_NONE = 0,
    TOKEN_SECURITY_ATTRIBUTE_OPERATION_REPLACE_ALL = 1,
    TOKEN_SECURITY_ATTRIBUTE_OPERATION_ADD = 2,
    TOKEN_SECURITY_ATTRIBUTE_OPERATION_DELETE = 3,
    TOKEN_SECURITY_ATTRIBUTE_OPERATION_REPLACE = 4,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TOKEN_SECURITY_ATTRIBUTES_AND_OPERATION_INFORMATION {
    pub Attributes: *mut TOKEN_SECURITY_ATTRIBUTES_INFORMATION,
    pub Operations: *mut TOKEN_SECURITY_ATTRIBUTE_OPERATION,
}

impl Default for TOKEN_SECURITY_ATTRIBUTES_AND_OPERATION_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

#[repr(C)]
pub struct TOKEN_PROCESS_TRUST_LEVEL {
    pub TrustLevelSid: PSID,
}

impl Default for TOKEN_PROCESS_TRUST_LEVEL {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for TOKEN_PROCESS_TRUST_LEVEL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TOKEN_PROCESS_TRUST_LEVEL {{  }}")
    }
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtCreateToken(
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
    pub fn NtCreateLowBoxToken(
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
    pub fn NtCreateTokenEx(
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
    pub fn NtAdjustTokenClaimsAndDeviceGroups(
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
    pub fn NtFilterTokenEx(
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
    pub fn NtCompareTokens(
        FirstTokenHandle: HANDLE,
        SecondTokenHandle: HANDLE,
        Equal: *mut bool,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtQuerySecurityAttributesToken(
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
    pub fn NtAccessCheck(
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
    pub fn NtAccessCheckByType(
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
    pub fn NtAccessCheckByTypeResultList(
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
    pub fn NtSetCachedSigningLevel(
        Flags: u32,
        InputSigningLevel: u8,
        SourceFiles: *mut HANDLE,
        SourceFileCount: u32,
        TargetFile: HANDLE,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtGetCachedSigningLevel(
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
    pub fn NtCompareSigningLevels(FirstSigningLevel: u8, SecondSigningLevel: u8) -> NTSTATUS;
}
