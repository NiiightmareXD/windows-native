use windows::{
    core::PWSTR,
    Wdk::Foundation::OBJECT_ATTRIBUTES,
    Win32::{
        Foundation::{HANDLE, NTSTATUS, UNICODE_STRING},
        Security::{
            Authentication::Identity::{
                DOMAIN_PASSWORD_INFORMATION, LOGON_HOURS, USER_ALL_INFORMATION,
            },
            PSID, SECURITY_DESCRIPTOR, SID_NAME_USE,
        },
        System::{
            Kernel::STRING,
            PasswordManagement::{CYPHER_BLOCK, ENCRYPTED_LM_OWF_PASSWORD},
        },
    },
};

use crate::bitfield::UnionField;

pub const SAM_MAXIMUM_LOOKUP_COUNT: u32 = 1000;
pub const SAM_MAXIMUM_LOOKUP_LENGTH: u32 = 32000;
pub const SAM_MAX_PASSWORD_LENGTH: u32 = 256;
pub const SAM_PASSWORD_ENCRYPTION_SALT_LEN: u32 = 16;
pub const SAM_SERVER_CONNECT: u32 = 1;
pub const SAM_SERVER_SHUTDOWN: u32 = 2;
pub const SAM_SERVER_INITIALIZE: u32 = 4;
pub const SAM_SERVER_CREATE_DOMAIN: u32 = 8;
pub const SAM_SERVER_ENUMERATE_DOMAINS: u32 = 16;
pub const SAM_SERVER_LOOKUP_DOMAIN: u32 = 32;
pub const SAM_SERVER_ALL_ACCESS: u32 = 983103;
pub const SAM_SERVER_READ: u32 = 131088;
pub const SAM_SERVER_WRITE: u32 = 131086;
pub const SAM_SERVER_EXECUTE: u32 = 131105;
pub const DOMAIN_READ_PASSWORD_PARAMETERS: u32 = 1;
pub const DOMAIN_WRITE_PASSWORD_PARAMS: u32 = 2;
pub const DOMAIN_READ_OTHER_PARAMETERS: u32 = 4;
pub const DOMAIN_WRITE_OTHER_PARAMETERS: u32 = 8;
pub const DOMAIN_CREATE_USER: u32 = 16;
pub const DOMAIN_CREATE_GROUP: u32 = 32;
pub const DOMAIN_CREATE_ALIAS: u32 = 64;
pub const DOMAIN_GET_ALIAS_MEMBERSHIP: u32 = 128;
pub const DOMAIN_LIST_ACCOUNTS: u32 = 256;
pub const DOMAIN_LOOKUP: u32 = 512;
pub const DOMAIN_ADMINISTER_SERVER: u32 = 1024;
pub const DOMAIN_ALL_ACCESS: u32 = 985087;
pub const DOMAIN_READ: u32 = 131204;
pub const DOMAIN_WRITE: u32 = 132218;
pub const DOMAIN_EXECUTE: u32 = 131841;
pub const GROUP_READ_INFORMATION: u32 = 1;
pub const GROUP_WRITE_ACCOUNT: u32 = 2;
pub const GROUP_ADD_MEMBER: u32 = 4;
pub const GROUP_REMOVE_MEMBER: u32 = 8;
pub const GROUP_LIST_MEMBERS: u32 = 16;
pub const GROUP_ALL_ACCESS: u32 = 983071;
pub const GROUP_READ: u32 = 131088;
pub const GROUP_WRITE: u32 = 131086;
pub const GROUP_EXECUTE: u32 = 131073;
pub const ALIAS_ADD_MEMBER: u32 = 1;
pub const ALIAS_REMOVE_MEMBER: u32 = 2;
pub const ALIAS_LIST_MEMBERS: u32 = 4;
pub const ALIAS_READ_INFORMATION: u32 = 8;
pub const ALIAS_WRITE_ACCOUNT: u32 = 16;
pub const ALIAS_ALL_ACCESS: u32 = 983071;
pub const ALIAS_READ: u32 = 131076;
pub const ALIAS_WRITE: u32 = 131091;
pub const ALIAS_EXECUTE: u32 = 131080;
pub const ALIAS_ALL_NAME: u32 = 1;
pub const ALIAS_ALL_MEMBER_COUNT: u32 = 2;
pub const ALIAS_ALL_ADMIN_COMMENT: u32 = 4;
pub const ALIAS_ALL_SHELL_ADMIN_OBJECT_PROPERTIES: u32 = 8;
pub const GROUP_TYPE_BUILTIN_LOCAL_GROUP: u32 = 1;
pub const GROUP_TYPE_ACCOUNT_GROUP: u32 = 2;
pub const GROUP_TYPE_RESOURCE_GROUP: u32 = 4;
pub const GROUP_TYPE_UNIVERSAL_GROUP: u32 = 8;
pub const GROUP_TYPE_APP_BASIC_GROUP: u32 = 16;
pub const GROUP_TYPE_APP_QUERY_GROUP: u32 = 32;
pub const GROUP_TYPE_SECURITY_ENABLED: u32 = 2147483648;
pub const GROUP_TYPE_RESOURCE_BEHAVOIR: u32 = 52;
pub const USER_READ_GENERAL: u32 = 1;
pub const USER_READ_PREFERENCES: u32 = 2;
pub const USER_WRITE_PREFERENCES: u32 = 4;
pub const USER_READ_LOGON: u32 = 8;
pub const USER_READ_ACCOUNT: u32 = 16;
pub const USER_WRITE_ACCOUNT: u32 = 32;
pub const USER_CHANGE_PASSWORD: u32 = 64;
pub const USER_FORCE_PASSWORD_CHANGE: u32 = 128;
pub const USER_LIST_GROUPS: u32 = 256;
pub const USER_READ_GROUP_INFORMATION: u32 = 512;
pub const USER_WRITE_GROUP_INFORMATION: u32 = 1024;
pub const USER_ALL_ACCESS: u32 = 985087;
pub const USER_READ: u32 = 131866;
pub const USER_WRITE: u32 = 131140;
pub const USER_EXECUTE: u32 = 131137;
pub const NEXT_FREE_ACCOUNT_CONTROL_BIT: u32 = 4194304;
pub const USER_MACHINE_ACCOUNT_MASK: u32 = 448;
pub const USER_ACCOUNT_TYPE_MASK: u32 = 472;
pub const USER_COMPUTED_ACCOUNT_CONTROL_BITS: u32 = 132096;
pub const SAM_HOURS_PER_WEEK: u32 = 168;
pub const SAM_MINUTES_PER_WEEK: u32 = 10080;
pub const USER_ALL_USERNAME: u32 = 1;
pub const USER_ALL_FULLNAME: u32 = 2;
pub const USER_ALL_USERID: u32 = 4;
pub const USER_ALL_PRIMARYGROUPID: u32 = 8;
pub const USER_ALL_ADMINCOMMENT: u32 = 16;
pub const USER_ALL_USERCOMMENT: u32 = 32;
pub const USER_ALL_HOMEDIRECTORY: u32 = 64;
pub const USER_ALL_HOMEDIRECTORYDRIVE: u32 = 128;
pub const USER_ALL_SCRIPTPATH: u32 = 256;
pub const USER_ALL_PROFILEPATH: u32 = 512;
pub const USER_ALL_WORKSTATIONS: u32 = 1024;
pub const USER_ALL_LASTLOGON: u32 = 2048;
pub const USER_ALL_LASTLOGOFF: u32 = 4096;
pub const USER_ALL_LOGONHOURS: u32 = 8192;
pub const USER_ALL_BADPASSWORDCOUNT: u32 = 16384;
pub const USER_ALL_LOGONCOUNT: u32 = 32768;
pub const USER_ALL_PASSWORDCANCHANGE: u32 = 65536;
pub const USER_ALL_PASSWORDMUSTCHANGE: u32 = 131072;
pub const USER_ALL_PASSWORDLASTSET: u32 = 262144;
pub const USER_ALL_ACCOUNTEXPIRES: u32 = 524288;
pub const USER_ALL_USERACCOUNTCONTROL: u32 = 1048576;
pub const USER_ALL_COUNTRYCODE: u32 = 4194304;
pub const USER_ALL_CODEPAGE: u32 = 8388608;
pub const USER_ALL_NTPASSWORDPRESENT: u32 = 16777216;
pub const USER_ALL_LMPASSWORDPRESENT: u32 = 33554432;
pub const USER_ALL_PRIVATEDATA: u32 = 67108864;
pub const USER_ALL_PASSWORDEXPIRED: u32 = 134217728;
pub const USER_ALL_SECURITYDESCRIPTOR: u32 = 268435456;
pub const USER_ALL_OWFPASSWORD: u32 = 536870912;
pub const USER_ALL_UNDEFINED_MASK: u32 = 3221225472;
pub const USER_ALL_READ_GENERAL_MASK: u32 = 63;
pub const USER_ALL_READ_LOGON_MASK: u32 = 262080;
pub const USER_ALL_READ_ACCOUNT_MASK: u32 = 3932160;
pub const USER_ALL_READ_PREFERENCES_MASK: u32 = 12582912;
pub const USER_ALL_READ_TRUSTED_MASK: u32 = 520093696;
pub const USER_ALL_READ_CANT_MASK: u32 = 3221225472;
pub const USER_ALL_WRITE_ACCOUNT_MASK: u32 = 3680219;
pub const USER_ALL_WRITE_PREFERENCES_MASK: u32 = 12582944;
pub const USER_ALL_WRITE_FORCE_PASSWORD_CHANGE_MASK: u32 = 184549376;
pub const USER_ALL_WRITE_TRUSTED_MASK: u32 = 335861760;
pub const USER_ALL_WRITE_CANT_MASK: u32 = 3221422084;
pub const USER_EXTENDED_FIELD_UPN: u32 = 1;
pub const USER_EXTENDED_FIELD_A2D2: u32 = 2;
pub const USER_EXTENDED_FIELD_USER_TILE: u32 = 4096;
pub const USER_EXTENDED_FIELD_PASSWORD_HINT: u32 = 8192;
pub const USER_EXTENDED_FIELD_DONT_SHOW_IN_LOGON_UI: u32 = 16384;
pub const USER_EXTENDED_FIELD_SHELL_ADMIN_OBJECT_PROPERTIES: u32 = 32768;
pub const SAM_PWD_CHANGE_NO_ERROR: u32 = 0;
pub const SAM_PWD_CHANGE_PASSWORD_TOO_SHORT: u32 = 1;
pub const SAM_PWD_CHANGE_PWD_IN_HISTORY: u32 = 2;
pub const SAM_PWD_CHANGE_USERNAME_IN_PASSWORD: u32 = 3;
pub const SAM_PWD_CHANGE_FULLNAME_IN_PASSWORD: u32 = 4;
pub const SAM_PWD_CHANGE_NOT_COMPLEX: u32 = 5;
pub const SAM_PWD_CHANGE_MACHINE_PASSWORD_NOT_DEFAULT: u32 = 6;
pub const SAM_PWD_CHANGE_FAILED_BY_FILTER: u32 = 7;
pub const SAM_PWD_CHANGE_PASSWORD_TOO_LONG: u32 = 8;
pub const SAM_PWD_CHANGE_FAILURE_REASON_MAX: u32 = 8;
pub const SAM_USER_ACCOUNT: u32 = 1;
pub const SAM_GLOBAL_GROUP_ACCOUNT: u32 = 2;
pub const SAM_LOCAL_GROUP_ACCOUNT: u32 = 4;
pub const SAM_DELTA_NOTIFY_ROUTINE: &[u8; 12] = b"DeltaNotify\0";
pub const SAM_SID_COMPATIBILITY_ALL: u32 = 0;
pub const SAM_SID_COMPATIBILITY_LAX: u32 = 1;
pub const SAM_SID_COMPATIBILITY_STRICT: u32 = 2;
pub const SAM_VALIDATE_PASSWORD_LAST_SET: u32 = 1;
pub const SAM_VALIDATE_BAD_PASSWORD_TIME: u32 = 2;
pub const SAM_VALIDATE_LOCKOUT_TIME: u32 = 4;
pub const SAM_VALIDATE_BAD_PASSWORD_COUNT: u32 = 8;
pub const SAM_VALIDATE_PASSWORD_HISTORY_LENGTH: u32 = 16;
pub const SAM_VALIDATE_PASSWORD_HISTORY: u32 = 32;
pub const DOMAIN_PROMOTION_INCREMENT: (u32, u32) = (0, 16);
pub const DOMAIN_PROMOTION_MASK: (u32, u32) = (0, 4294967280);

#[repr(C)]
pub struct SAM_RID_ENUMERATION {
    pub RelativeId: u32,
    pub Name: UNICODE_STRING,
}

impl Default for SAM_RID_ENUMERATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for SAM_RID_ENUMERATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SAM_RID_ENUMERATION {{  }}")
    }
}

#[repr(C)]
pub struct SAM_SID_ENUMERATION {
    pub Sid: PSID,
    pub Name: UNICODE_STRING,
}

impl Default for SAM_SID_ENUMERATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for SAM_SID_ENUMERATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SAM_SID_ENUMERATION {{  }}")
    }
}

#[repr(C)]
pub struct SAM_BYTE_ARRAY {
    pub Size: u32,
    pub Data: *mut u8,
}

impl Default for SAM_BYTE_ARRAY {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for SAM_BYTE_ARRAY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SAM_BYTE_ARRAY {{  }}")
    }
}

#[repr(C)]
pub struct SAM_BYTE_ARRAY_32K {
    pub Size: u32,
    pub Data: *mut u8,
}

impl Default for SAM_BYTE_ARRAY_32K {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for SAM_BYTE_ARRAY_32K {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SAM_BYTE_ARRAY_32K {{  }}")
    }
}

pub type SAM_SHELL_OBJECT_PROPERTIES = SAM_BYTE_ARRAY_32K;

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamFreeMemory(Buffer: *mut std::ffi::c_void) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamCloseHandle(SamHandle: *mut std::ffi::c_void) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamSetSecurityObject(
        ObjectHandle: *mut std::ffi::c_void,
        SecurityInformation: u32,
        SecurityDescriptor: *mut SECURITY_DESCRIPTOR,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamQuerySecurityObject(
        ObjectHandle: *mut std::ffi::c_void,
        SecurityInformation: u32,
        SecurityDescriptor: *mut *mut SECURITY_DESCRIPTOR,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamRidToSid(ObjectHandle: *mut std::ffi::c_void, Rid: u32, Sid: *mut PSID) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamConnect(
        ServerName: *mut UNICODE_STRING,
        ServerHandle: *mut *mut std::ffi::c_void,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamConnectWithCreds(
        ServerName: *mut UNICODE_STRING,
        ServerHandle: *mut *mut std::ffi::c_void,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        Creds: *mut std::ffi::c_void,
        Spn: PWSTR,
        pfDstIsW2K: *mut bool,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamShutdownSamServer(ServerHandle: *mut std::ffi::c_void) -> NTSTATUS;
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum DOMAIN_INFORMATION_CLASS {
    DomainPasswordInformation = 1,
    DomainGeneralInformation = 2,
    DomainLogoffInformation = 3,
    DomainOemInformation = 4,
    DomainNameInformation = 5,
    DomainReplicationInformation = 6,
    DomainServerRoleInformation = 7,
    DomainModifiedInformation = 8,
    DomainStateInformation = 9,
    DomainUasInformation = 10,
    DomainGeneralInformation2 = 11,
    DomainLockoutInformation = 12,
    DomainModifiedInformation2 = 13,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum DOMAIN_SERVER_ENABLE_STATE {
    DomainServerEnabled = 1,
    DomainServerDisabled = 2,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum DOMAIN_SERVER_ROLE {
    DomainServerRoleBackup = 2,
    DomainServerRolePrimary = 3,
}

#[repr(C, packed(4))]
pub struct DOMAIN_GENERAL_INFORMATION {
    pub ForceLogoff: i64,
    pub OemInformation: UNICODE_STRING,
    pub DomainName: UNICODE_STRING,
    pub ReplicaSourceNodeName: UNICODE_STRING,
    pub DomainModifiedCount: i64,
    pub DomainServerState: DOMAIN_SERVER_ENABLE_STATE,
    pub DomainServerRole: DOMAIN_SERVER_ROLE,
    pub UasCompatibilityRequired: bool,
    pub UserCount: u32,
    pub GroupCount: u32,
    pub AliasCount: u32,
}

impl Default for DOMAIN_GENERAL_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for DOMAIN_GENERAL_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DOMAIN_GENERAL_INFORMATION {{ DomainServerState: {:?}, DomainServerRole: {:?} }}",
            self.DomainServerState, self.DomainServerRole
        )
    }
}

#[repr(C, packed(4))]
pub struct DOMAIN_GENERAL_INFORMATION2 {
    pub I1: DOMAIN_GENERAL_INFORMATION,
    pub LockoutDuration: i64,
    pub LockoutObservationWindow: i64,
    pub LockoutThreshold: u16,
}

impl Default for DOMAIN_GENERAL_INFORMATION2 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for DOMAIN_GENERAL_INFORMATION2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DOMAIN_GENERAL_INFORMATION2 {{ I1: {:?} }}", self.I1)
    }
}

#[repr(C)]
pub struct DOMAIN_UAS_INFORMATION {
    pub UasCompatibilityRequired: bool,
}

impl Default for DOMAIN_UAS_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for DOMAIN_UAS_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DOMAIN_UAS_INFORMATION {{  }}")
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum DOMAIN_PASSWORD_CONSTRUCTION {
    DomainPasswordSimple = 1,
    DomainPasswordComplex = 2,
}

#[repr(C)]
pub struct DOMAIN_LOGOFF_INFORMATION {
    pub ForceLogoff: i64,
}

impl Default for DOMAIN_LOGOFF_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for DOMAIN_LOGOFF_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DOMAIN_LOGOFF_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct DOMAIN_OEM_INFORMATION {
    pub OemInformation: UNICODE_STRING,
}

impl Default for DOMAIN_OEM_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for DOMAIN_OEM_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DOMAIN_OEM_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct DOMAIN_NAME_INFORMATION {
    pub DomainName: UNICODE_STRING,
}

impl Default for DOMAIN_NAME_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for DOMAIN_NAME_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DOMAIN_NAME_INFORMATION {{  }}")
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DOMAIN_SERVER_ROLE_INFORMATION {
    pub DomainServerRole: DOMAIN_SERVER_ROLE,
}

impl Default for DOMAIN_SERVER_ROLE_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

#[repr(C)]
pub struct DOMAIN_REPLICATION_INFORMATION {
    pub ReplicaSourceNodeName: UNICODE_STRING,
}

impl Default for DOMAIN_REPLICATION_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for DOMAIN_REPLICATION_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DOMAIN_REPLICATION_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct DOMAIN_MODIFIED_INFORMATION {
    pub DomainModifiedCount: i64,
    pub CreationTime: i64,
}

impl Default for DOMAIN_MODIFIED_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for DOMAIN_MODIFIED_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DOMAIN_MODIFIED_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct DOMAIN_MODIFIED_INFORMATION2 {
    pub DomainModifiedCount: i64,
    pub CreationTime: i64,
    pub ModifiedCountAtLastPromotion: i64,
}

impl Default for DOMAIN_MODIFIED_INFORMATION2 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for DOMAIN_MODIFIED_INFORMATION2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DOMAIN_MODIFIED_INFORMATION2 {{  }}")
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DOMAIN_STATE_INFORMATION {
    pub DomainServerState: DOMAIN_SERVER_ENABLE_STATE,
}

impl Default for DOMAIN_STATE_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

#[repr(C)]
pub struct DOMAIN_LOCKOUT_INFORMATION {
    pub LockoutDuration: i64,
    pub LockoutObservationWindow: i64,
    pub LockoutThreshold: u16,
}

impl Default for DOMAIN_LOCKOUT_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for DOMAIN_LOCKOUT_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DOMAIN_LOCKOUT_INFORMATION {{  }}")
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum DOMAIN_DISPLAY_INFORMATION {
    DomainDisplayUser = 1,
    DomainDisplayMachine = 2,
    DomainDisplayGroup = 3,
    DomainDisplayOemUser = 4,
    DomainDisplayOemGroup = 5,
    DomainDisplayServer = 6,
}

#[repr(C)]
pub struct DOMAIN_DISPLAY_USER {
    pub Index: u32,
    pub Rid: u32,
    pub AccountControl: u32,
    pub LogonName: UNICODE_STRING,
    pub AdminComment: UNICODE_STRING,
    pub FullName: UNICODE_STRING,
}

impl Default for DOMAIN_DISPLAY_USER {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for DOMAIN_DISPLAY_USER {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DOMAIN_DISPLAY_USER {{  }}")
    }
}

#[repr(C)]
pub struct DOMAIN_DISPLAY_MACHINE {
    pub Index: u32,
    pub Rid: u32,
    pub AccountControl: u32,
    pub Machine: UNICODE_STRING,
    pub Comment: UNICODE_STRING,
}

impl Default for DOMAIN_DISPLAY_MACHINE {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for DOMAIN_DISPLAY_MACHINE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DOMAIN_DISPLAY_MACHINE {{  }}")
    }
}

#[repr(C)]
pub struct DOMAIN_DISPLAY_GROUP {
    pub Index: u32,
    pub Rid: u32,
    pub Attributes: u32,
    pub Group: UNICODE_STRING,
    pub Comment: UNICODE_STRING,
}

impl Default for DOMAIN_DISPLAY_GROUP {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for DOMAIN_DISPLAY_GROUP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DOMAIN_DISPLAY_GROUP {{  }}")
    }
}

#[repr(C)]
pub struct DOMAIN_DISPLAY_OEM_USER {
    pub Index: u32,
    pub User: STRING,
}

impl Default for DOMAIN_DISPLAY_OEM_USER {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for DOMAIN_DISPLAY_OEM_USER {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DOMAIN_DISPLAY_OEM_USER {{  }}")
    }
}

#[repr(C)]
pub struct DOMAIN_DISPLAY_OEM_GROUP {
    pub Index: u32,
    pub Group: STRING,
}

impl Default for DOMAIN_DISPLAY_OEM_GROUP {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for DOMAIN_DISPLAY_OEM_GROUP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DOMAIN_DISPLAY_OEM_GROUP {{  }}")
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum DOMAIN_LOCALIZABLE_ACCOUNTS_INFORMATION {
    DomainLocalizableAccountsBasic = 1,
}

#[repr(C)]
pub struct DOMAIN_LOCALIZABLE_ACCOUNTS_ENTRY {
    pub Rid: u32,
    pub Use: SID_NAME_USE,
    pub Name: UNICODE_STRING,
    pub AdminComment: UNICODE_STRING,
}

impl Default for DOMAIN_LOCALIZABLE_ACCOUNTS_ENTRY {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for DOMAIN_LOCALIZABLE_ACCOUNTS_ENTRY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DOMAIN_LOCALIZABLE_ACCOUNTS_ENTRY {{  }}")
    }
}

#[repr(C)]
pub struct DOMAIN_LOCALIZABLE_ACCOUNTS {
    pub Count: u32,
    pub Entries: *mut DOMAIN_LOCALIZABLE_ACCOUNTS_ENTRY,
}

impl Default for DOMAIN_LOCALIZABLE_ACCOUNTS {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for DOMAIN_LOCALIZABLE_ACCOUNTS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DOMAIN_LOCALIZABLE_ACCOUNTS {{ Entries: {:?} }}",
            self.Entries
        )
    }
}

#[repr(C)]
pub struct DOMAIN_LOCALIZABLE_INFO_BUFFER {
    pub Basic: UnionField<DOMAIN_LOCALIZABLE_ACCOUNTS>,
    pub union_field: [u64; 2],
}

impl Default for DOMAIN_LOCALIZABLE_INFO_BUFFER {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for DOMAIN_LOCALIZABLE_INFO_BUFFER {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DOMAIN_LOCALIZABLE_INFO_BUFFER {{ union }}")
    }
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamLookupDomainInSamServer(
        ServerHandle: *mut std::ffi::c_void,
        Name: *mut UNICODE_STRING,
        DomainId: *mut PSID,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamEnumerateDomainsInSamServer(
        ServerHandle: *mut std::ffi::c_void,
        EnumerationContext: *mut u32,
        Buffer: *mut *mut std::ffi::c_void,
        PreferedMaximumLength: u32,
        CountReturned: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamOpenDomain(
        ServerHandle: *mut std::ffi::c_void,
        DesiredAccess: u32,
        DomainId: PSID,
        DomainHandle: *mut *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamQueryInformationDomain(
        DomainHandle: *mut std::ffi::c_void,
        DomainInformationClass: DOMAIN_INFORMATION_CLASS,
        Buffer: *mut *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamSetInformationDomain(
        DomainHandle: *mut std::ffi::c_void,
        DomainInformationClass: DOMAIN_INFORMATION_CLASS,
        DomainInformation: *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamLookupNamesInDomain(
        DomainHandle: *mut std::ffi::c_void,
        Count: u32,
        Names: *mut UNICODE_STRING,
        RelativeIds: *mut *mut u32,
        Use: *mut *mut SID_NAME_USE,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamLookupNamesInDomain2(
        DomainHandle: *mut std::ffi::c_void,
        Count: u32,
        Names: *mut UNICODE_STRING,
        Sids: *mut PSID,
        Use: *mut *mut SID_NAME_USE,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamLookupIdsInDomain(
        DomainHandle: *mut std::ffi::c_void,
        Count: u32,
        RelativeIds: *mut u32,
        Names: *mut *mut UNICODE_STRING,
        Use: *mut *mut SID_NAME_USE,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamRemoveMemberFromForeignDomain(
        DomainHandle: *mut std::ffi::c_void,
        MemberId: PSID,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamQueryLocalizableAccountsInDomain(
        Domain: *mut std::ffi::c_void,
        Flags: u32,
        LanguageId: u32,
        Class: DOMAIN_LOCALIZABLE_ACCOUNTS_INFORMATION,
        Buffer: *mut *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[repr(C)]
pub struct GROUP_MEMBERSHIP {
    pub RelativeId: u32,
    pub Attributes: u32,
}

impl Default for GROUP_MEMBERSHIP {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for GROUP_MEMBERSHIP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GROUP_MEMBERSHIP {{  }}")
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GROUP_INFORMATION_CLASS {
    GroupGeneralInformation = 1,
    GroupNameInformation = 2,
    GroupAttributeInformation = 3,
    GroupAdminCommentInformation = 4,
    GroupReplicationInformation = 5,
}

#[repr(C)]
pub struct GROUP_GENERAL_INFORMATION {
    pub Name: UNICODE_STRING,
    pub Attributes: u32,
    pub MemberCount: u32,
    pub AdminComment: UNICODE_STRING,
}

impl Default for GROUP_GENERAL_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for GROUP_GENERAL_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GROUP_GENERAL_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct GROUP_NAME_INFORMATION {
    pub Name: UNICODE_STRING,
}

impl Default for GROUP_NAME_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for GROUP_NAME_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GROUP_NAME_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct GROUP_ATTRIBUTE_INFORMATION {
    pub Attributes: u32,
}

impl Default for GROUP_ATTRIBUTE_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for GROUP_ATTRIBUTE_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GROUP_ATTRIBUTE_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct GROUP_ADM_COMMENT_INFORMATION {
    pub AdminComment: UNICODE_STRING,
}

impl Default for GROUP_ADM_COMMENT_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for GROUP_ADM_COMMENT_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GROUP_ADM_COMMENT_INFORMATION {{  }}")
    }
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamEnumerateGroupsInDomain(
        DomainHandle: *mut std::ffi::c_void,
        EnumerationContext: *mut u32,
        Buffer: *mut *mut std::ffi::c_void,
        PreferedMaximumLength: u32,
        CountReturned: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamCreateGroupInDomain(
        DomainHandle: *mut std::ffi::c_void,
        AccountName: *mut UNICODE_STRING,
        DesiredAccess: u32,
        GroupHandle: *mut *mut std::ffi::c_void,
        RelativeId: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamOpenGroup(
        DomainHandle: *mut std::ffi::c_void,
        DesiredAccess: u32,
        GroupId: u32,
        GroupHandle: *mut *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamDeleteGroup(GroupHandle: *mut std::ffi::c_void) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamQueryInformationGroup(
        GroupHandle: *mut std::ffi::c_void,
        GroupInformationClass: GROUP_INFORMATION_CLASS,
        Buffer: *mut *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamSetInformationGroup(
        GroupHandle: *mut std::ffi::c_void,
        GroupInformationClass: GROUP_INFORMATION_CLASS,
        Buffer: *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamAddMemberToGroup(
        GroupHandle: *mut std::ffi::c_void,
        MemberId: u32,
        Attributes: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamRemoveMemberFromGroup(GroupHandle: *mut std::ffi::c_void, MemberId: u32) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamGetMembersInGroup(
        GroupHandle: *mut std::ffi::c_void,
        MemberIds: *mut *mut u32,
        Attributes: *mut *mut u32,
        MemberCount: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamSetMemberAttributesOfGroup(
        GroupHandle: *mut std::ffi::c_void,
        MemberId: u32,
        Attributes: u32,
    ) -> NTSTATUS;
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ALIAS_INFORMATION_CLASS {
    AliasGeneralInformation = 1,
    AliasNameInformation = 2,
    AliasAdminCommentInformation = 3,
    AliasReplicationInformation = 4,
    AliasExtendedInformation = 5,
}

#[repr(C)]
pub struct ALIAS_GENERAL_INFORMATION {
    pub Name: UNICODE_STRING,
    pub MemberCount: u32,
    pub AdminComment: UNICODE_STRING,
}

impl Default for ALIAS_GENERAL_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ALIAS_GENERAL_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ALIAS_GENERAL_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct ALIAS_NAME_INFORMATION {
    pub Name: UNICODE_STRING,
}

impl Default for ALIAS_NAME_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ALIAS_NAME_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ALIAS_NAME_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct ALIAS_ADM_COMMENT_INFORMATION {
    pub AdminComment: UNICODE_STRING,
}

impl Default for ALIAS_ADM_COMMENT_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ALIAS_ADM_COMMENT_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ALIAS_ADM_COMMENT_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct ALIAS_EXTENDED_INFORMATION {
    pub WhichFields: u32,
    pub ShellAdminObjectProperties: SAM_SHELL_OBJECT_PROPERTIES,
}

impl Default for ALIAS_EXTENDED_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ALIAS_EXTENDED_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ALIAS_EXTENDED_INFORMATION {{ ShellAdminObjectProperties: {:?} }}",
            self.ShellAdminObjectProperties
        )
    }
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamEnumerateAliasesInDomain(
        DomainHandle: *mut std::ffi::c_void,
        EnumerationContext: *mut u32,
        Buffer: *mut *mut std::ffi::c_void,
        PreferedMaximumLength: u32,
        CountReturned: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamCreateAliasInDomain(
        DomainHandle: *mut std::ffi::c_void,
        AccountName: *mut UNICODE_STRING,
        DesiredAccess: u32,
        AliasHandle: *mut *mut std::ffi::c_void,
        RelativeId: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamOpenAlias(
        DomainHandle: *mut std::ffi::c_void,
        DesiredAccess: u32,
        AliasId: u32,
        AliasHandle: *mut *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamDeleteAlias(AliasHandle: *mut std::ffi::c_void) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamQueryInformationAlias(
        AliasHandle: *mut std::ffi::c_void,
        AliasInformationClass: ALIAS_INFORMATION_CLASS,
        Buffer: *mut *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamSetInformationAlias(
        AliasHandle: *mut std::ffi::c_void,
        AliasInformationClass: ALIAS_INFORMATION_CLASS,
        Buffer: *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamAddMemberToAlias(AliasHandle: *mut std::ffi::c_void, MemberId: PSID) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamAddMultipleMembersToAlias(
        AliasHandle: *mut std::ffi::c_void,
        MemberIds: *mut PSID,
        MemberCount: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamRemoveMemberFromAlias(AliasHandle: *mut std::ffi::c_void, MemberId: PSID)
    -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamRemoveMultipleMembersFromAlias(
        AliasHandle: *mut std::ffi::c_void,
        MemberIds: *mut PSID,
        MemberCount: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamGetMembersInAlias(
        AliasHandle: *mut std::ffi::c_void,
        MemberIds: *mut *mut PSID,
        MemberCount: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamGetAliasMembership(
        DomainHandle: *mut std::ffi::c_void,
        PassedCount: u32,
        Sids: *mut PSID,
        MembershipCount: *mut u32,
        Aliases: *mut *mut u32,
    ) -> NTSTATUS;
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum USER_INFORMATION_CLASS {
    UserGeneralInformation = 1,
    UserPreferencesInformation = 2,
    UserLogonInformation = 3,
    UserLogonHoursInformation = 4,
    UserAccountInformation = 5,
    UserNameInformation = 6,
    UserAccountNameInformation = 7,
    UserFullNameInformation = 8,
    UserPrimaryGroupInformation = 9,
    UserHomeInformation = 10,
    UserScriptInformation = 11,
    UserProfileInformation = 12,
    UserAdminCommentInformation = 13,
    UserWorkStationsInformation = 14,
    UserSetPasswordInformation = 15,
    UserControlInformation = 16,
    UserExpiresInformation = 17,
    UserInternal1Information = 18,
    UserInternal2Information = 19,
    UserParametersInformation = 20,
    UserAllInformation = 21,
    UserInternal3Information = 22,
    UserInternal4Information = 23,
    UserInternal5Information = 24,
    UserInternal4InformationNew = 25,
    UserInternal5InformationNew = 26,
    UserInternal6Information = 27,
    UserExtendedInformation = 28,
    UserLogonUIInformation = 29,
    UserUnknownTodoInformation = 30,
    UserInternal7Information = 31,
    UserInternal8Information = 32,
}

#[repr(C)]
pub struct USER_GENERAL_INFORMATION {
    pub UserName: UNICODE_STRING,
    pub FullName: UNICODE_STRING,
    pub PrimaryGroupId: u32,
    pub AdminComment: UNICODE_STRING,
    pub UserComment: UNICODE_STRING,
}

impl Default for USER_GENERAL_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for USER_GENERAL_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "USER_GENERAL_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct USER_PREFERENCES_INFORMATION {
    pub UserComment: UNICODE_STRING,
    pub Reserved1: UNICODE_STRING,
    pub CountryCode: u16,
    pub CodePage: u16,
}

impl Default for USER_PREFERENCES_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for USER_PREFERENCES_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "USER_PREFERENCES_INFORMATION {{  }}")
    }
}

#[repr(C, packed(4))]
pub struct USER_LOGON_INFORMATION {
    pub UserName: UNICODE_STRING,
    pub FullName: UNICODE_STRING,
    pub UserId: u32,
    pub PrimaryGroupId: u32,
    pub HomeDirectory: UNICODE_STRING,
    pub HomeDirectoryDrive: UNICODE_STRING,
    pub ScriptPath: UNICODE_STRING,
    pub ProfilePath: UNICODE_STRING,
    pub WorkStations: UNICODE_STRING,
    pub LastLogon: i64,
    pub LastLogoff: i64,
    pub PasswordLastSet: i64,
    pub PasswordCanChange: i64,
    pub PasswordMustChange: i64,
    pub LogonHours: LOGON_HOURS,
    pub BadPasswordCount: u16,
    pub LogonCount: u16,
    pub UserAccountControl: u32,
}

impl Default for USER_LOGON_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for USER_LOGON_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "USER_LOGON_INFORMATION {{ LogonHours }}")
    }
}

#[repr(C)]
pub struct USER_LOGON_HOURS_INFORMATION {
    pub LogonHours: LOGON_HOURS,
}

impl Default for USER_LOGON_HOURS_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for USER_LOGON_HOURS_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "USER_LOGON_HOURS_INFORMATION {{ LogonHours: {:?} }}",
            self.LogonHours
        )
    }
}

#[repr(C, packed(4))]
pub struct USER_ACCOUNT_INFORMATION {
    pub UserName: UNICODE_STRING,
    pub FullName: UNICODE_STRING,
    pub UserId: u32,
    pub PrimaryGroupId: u32,
    pub HomeDirectory: UNICODE_STRING,
    pub HomeDirectoryDrive: UNICODE_STRING,
    pub ScriptPath: UNICODE_STRING,
    pub ProfilePath: UNICODE_STRING,
    pub AdminComment: UNICODE_STRING,
    pub WorkStations: UNICODE_STRING,
    pub LastLogon: i64,
    pub LastLogoff: i64,
    pub LogonHours: LOGON_HOURS,
    pub BadPasswordCount: u16,
    pub LogonCount: u16,
    pub PasswordLastSet: i64,
    pub AccountExpires: i64,
    pub UserAccountControl: u32,
}

impl Default for USER_ACCOUNT_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for USER_ACCOUNT_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "USER_ACCOUNT_INFORMATION {{ }}")
    }
}

#[repr(C)]
pub struct USER_NAME_INFORMATION {
    pub UserName: UNICODE_STRING,
    pub FullName: UNICODE_STRING,
}

impl Default for USER_NAME_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for USER_NAME_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "USER_NAME_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct USER_ACCOUNT_NAME_INFORMATION {
    pub UserName: UNICODE_STRING,
}

impl Default for USER_ACCOUNT_NAME_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for USER_ACCOUNT_NAME_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "USER_ACCOUNT_NAME_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct USER_FULL_NAME_INFORMATION {
    pub FullName: UNICODE_STRING,
}

impl Default for USER_FULL_NAME_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for USER_FULL_NAME_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "USER_FULL_NAME_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct USER_PRIMARY_GROUP_INFORMATION {
    pub PrimaryGroupId: u32,
}

impl Default for USER_PRIMARY_GROUP_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for USER_PRIMARY_GROUP_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "USER_PRIMARY_GROUP_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct USER_HOME_INFORMATION {
    pub HomeDirectory: UNICODE_STRING,
    pub HomeDirectoryDrive: UNICODE_STRING,
}

impl Default for USER_HOME_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for USER_HOME_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "USER_HOME_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct USER_SCRIPT_INFORMATION {
    pub ScriptPath: UNICODE_STRING,
}

impl Default for USER_SCRIPT_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for USER_SCRIPT_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "USER_SCRIPT_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct USER_PROFILE_INFORMATION {
    pub ProfilePath: UNICODE_STRING,
}

impl Default for USER_PROFILE_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for USER_PROFILE_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "USER_PROFILE_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct USER_ADMIN_COMMENT_INFORMATION {
    pub AdminComment: UNICODE_STRING,
}

impl Default for USER_ADMIN_COMMENT_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for USER_ADMIN_COMMENT_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "USER_ADMIN_COMMENT_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct USER_WORKSTATIONS_INFORMATION {
    pub WorkStations: UNICODE_STRING,
}

impl Default for USER_WORKSTATIONS_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for USER_WORKSTATIONS_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "USER_WORKSTATIONS_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct USER_SET_PASSWORD_INFORMATION {
    pub Password: UNICODE_STRING,
    pub PasswordExpired: bool,
}

impl Default for USER_SET_PASSWORD_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for USER_SET_PASSWORD_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "USER_SET_PASSWORD_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct USER_CONTROL_INFORMATION {
    pub UserAccountControl: u32,
}

impl Default for USER_CONTROL_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for USER_CONTROL_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "USER_CONTROL_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct USER_EXPIRES_INFORMATION {
    pub AccountExpires: i64,
}

impl Default for USER_EXPIRES_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for USER_EXPIRES_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "USER_EXPIRES_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct ENCRYPTED_NT_OWF_PASSWORD {
    pub data: [CYPHER_BLOCK; 2],
}

impl Default for ENCRYPTED_NT_OWF_PASSWORD {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ENCRYPTED_NT_OWF_PASSWORD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ENCRYPTED_NT_OWF_PASSWORD {{ data: {:?} }}", self.data)
    }
}

#[repr(C)]
pub struct USER_INTERNAL1_INFORMATION {
    pub EncryptedNtOwfPassword: ENCRYPTED_NT_OWF_PASSWORD,
    pub EncryptedLmOwfPassword: ENCRYPTED_LM_OWF_PASSWORD,
    pub NtPasswordPresent: bool,
    pub LmPasswordPresent: bool,
    pub PasswordExpired: bool,
}

impl Default for USER_INTERNAL1_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for USER_INTERNAL1_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "USER_INTERNAL1_INFORMATION {{ EncryptedNtOwfPassword: {:?}, EncryptedLmOwfPassword: {:?} }}",
            self.EncryptedNtOwfPassword, self.EncryptedLmOwfPassword
        )
    }
}

#[repr(C)]
pub struct USER_INTERNAL2_INFORMATION {
    pub StatisticsToApply: u32,
    pub LastLogon: i64,
    pub LastLogoff: i64,
    pub BadPasswordCount: u16,
    pub LogonCount: u16,
}

impl Default for USER_INTERNAL2_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for USER_INTERNAL2_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "USER_INTERNAL2_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct USER_PARAMETERS_INFORMATION {
    pub Parameters: UNICODE_STRING,
}

impl Default for USER_PARAMETERS_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for USER_PARAMETERS_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "USER_PARAMETERS_INFORMATION {{  }}")
    }
}

#[repr(C, packed(4))]
pub struct USER_INTERNAL3_INFORMATION {
    pub I1: USER_ALL_INFORMATION,
    pub LastBadPasswordTime: i64,
}

impl Default for USER_INTERNAL3_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for USER_INTERNAL3_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "USER_INTERNAL3_INFORMATION {{ }}")
    }
}

#[repr(C)]
pub struct ENCRYPTED_USER_PASSWORD {
    pub Buffer: [u8; 516],
}

impl Default for ENCRYPTED_USER_PASSWORD {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ENCRYPTED_USER_PASSWORD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ENCRYPTED_USER_PASSWORD {{ Buffer: {:?} }}", self.Buffer)
    }
}

#[repr(C)]
pub struct USER_INTERNAL4_INFORMATION {
    pub I1: USER_ALL_INFORMATION,
    pub UserPassword: ENCRYPTED_USER_PASSWORD,
}

impl Default for USER_INTERNAL4_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for USER_INTERNAL4_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "USER_INTERNAL4_INFORMATION {{ I1: UserPassword: {:?} }}",
            self.UserPassword
        )
    }
}

#[repr(C)]
pub struct USER_INTERNAL5_INFORMATION {
    pub UserPassword: ENCRYPTED_USER_PASSWORD,
    pub PasswordExpired: bool,
}

impl Default for USER_INTERNAL5_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for USER_INTERNAL5_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "USER_INTERNAL5_INFORMATION {{ UserPassword: {:?} }}",
            self.UserPassword
        )
    }
}

#[repr(C)]
pub struct ENCRYPTED_USER_PASSWORD_NEW {
    pub Buffer: [u8; 532],
}

impl Default for ENCRYPTED_USER_PASSWORD_NEW {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ENCRYPTED_USER_PASSWORD_NEW {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ENCRYPTED_USER_PASSWORD_NEW {{ Buffer: {:?} }}",
            self.Buffer
        )
    }
}

#[repr(C)]
pub struct USER_INTERNAL4_INFORMATION_NEW {
    pub I1: USER_ALL_INFORMATION,
    pub UserPassword: ENCRYPTED_USER_PASSWORD_NEW,
}

impl Default for USER_INTERNAL4_INFORMATION_NEW {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for USER_INTERNAL4_INFORMATION_NEW {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "USER_INTERNAL4_INFORMATION_NEW {{ UserPassword: {:?} }}",
            self.UserPassword
        )
    }
}

#[repr(C)]
pub struct USER_INTERNAL5_INFORMATION_NEW {
    pub UserPassword: ENCRYPTED_USER_PASSWORD_NEW,
    pub PasswordExpired: bool,
}

impl Default for USER_INTERNAL5_INFORMATION_NEW {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for USER_INTERNAL5_INFORMATION_NEW {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "USER_INTERNAL5_INFORMATION_NEW {{ UserPassword: {:?} }}",
            self.UserPassword
        )
    }
}

#[repr(C)]
pub struct USER_ALLOWED_TO_DELEGATE_TO_LIST {
    pub Size: u32,
    pub NumSPNs: u32,
    pub SPNList: [UNICODE_STRING; 1],
}

impl Default for USER_ALLOWED_TO_DELEGATE_TO_LIST {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for USER_ALLOWED_TO_DELEGATE_TO_LIST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "USER_ALLOWED_TO_DELEGATE_TO_LIST {{ SPNList: {:?} }}",
            self.SPNList
        )
    }
}

#[repr(C)]
pub struct USER_INTERNAL6_INFORMATION {
    pub I1: USER_ALL_INFORMATION,
    pub LastBadPasswordTime: i64,
    pub ExtendedFields: u32,
    pub UPNDefaulted: bool,
    pub UPN: UNICODE_STRING,
    pub A2D2List: *mut USER_ALLOWED_TO_DELEGATE_TO_LIST,
}

impl Default for USER_INTERNAL6_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for USER_INTERNAL6_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "USER_INTERNAL6_INFORMATION {{ A2D2List: {:?} }}",
            self.A2D2List
        )
    }
}

pub type SAM_USER_TILE = SAM_BYTE_ARRAY_32K;

pub type PSAM_USER_TILE = *mut SAM_BYTE_ARRAY_32K;

#[repr(C)]
pub struct USER_EXTENDED_INFORMATION {
    pub ExtendedWhichFields: u32,
    pub UserTile: SAM_USER_TILE,
    pub PasswordHint: UNICODE_STRING,
    pub DontShowInLogonUI: bool,
    pub ShellAdminObjectProperties: SAM_SHELL_OBJECT_PROPERTIES,
}

impl Default for USER_EXTENDED_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for USER_EXTENDED_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "USER_EXTENDED_INFORMATION {{ UserTile: {:?}, ShellAdminObjectProperties: {:?} }}",
            self.UserTile, self.ShellAdminObjectProperties
        )
    }
}

#[repr(C)]
pub struct USER_LOGON_UI_INFORMATION {
    pub PasswordIsBlank: bool,
    pub AccountIsDisabled: bool,
}

impl Default for USER_LOGON_UI_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for USER_LOGON_UI_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "USER_LOGON_UI_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct ENCRYPTED_PASSWORD_AES {
    pub AuthData: [u8; 64],
    pub Salt: [u8; 16],
    pub cbCipher: u32,
    pub Cipher: *mut u8,
    pub PBKDF2Iterations: u64,
}

impl Default for ENCRYPTED_PASSWORD_AES {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ENCRYPTED_PASSWORD_AES {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ENCRYPTED_PASSWORD_AES {{ AuthData: {:?}, Salt: {:?} }}",
            self.AuthData, self.Salt
        )
    }
}

#[repr(C)]
pub struct USER_INTERNAL7_INFORMATION {
    pub UserPassword: ENCRYPTED_PASSWORD_AES,
    pub PasswordExpired: bool,
}

impl Default for USER_INTERNAL7_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for USER_INTERNAL7_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "USER_INTERNAL7_INFORMATION {{ UserPassword: {:?} }}",
            self.UserPassword
        )
    }
}

#[repr(C)]
pub struct USER_INTERNAL8_INFORMATION {
    pub I1: USER_ALL_INFORMATION,
    pub UserPassword: ENCRYPTED_PASSWORD_AES,
}

impl Default for USER_INTERNAL8_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for USER_INTERNAL8_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "USER_INTERNAL8_INFORMATION {{ UserPassword: {:?} }}",
            self.UserPassword
        )
    }
}

#[repr(C)]
pub struct USER_PWD_CHANGE_FAILURE_INFORMATION {
    pub ExtendedFailureReason: u32,
    pub FilterModuleName: UNICODE_STRING,
}

impl Default for USER_PWD_CHANGE_FAILURE_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for USER_PWD_CHANGE_FAILURE_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "USER_PWD_CHANGE_FAILURE_INFORMATION {{  }}")
    }
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamEnumerateUsersInDomain(
        DomainHandle: *mut std::ffi::c_void,
        EnumerationContext: *mut u32,
        UserAccountControl: u32,
        Buffer: *mut *mut std::ffi::c_void,
        PreferedMaximumLength: u32,
        CountReturned: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamCreateUserInDomain(
        DomainHandle: *mut std::ffi::c_void,
        AccountName: *mut UNICODE_STRING,
        DesiredAccess: u32,
        UserHandle: *mut *mut std::ffi::c_void,
        RelativeId: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamCreateUser2InDomain(
        DomainHandle: *mut std::ffi::c_void,
        AccountName: *mut UNICODE_STRING,
        AccountType: u32,
        DesiredAccess: u32,
        UserHandle: *mut *mut std::ffi::c_void,
        GrantedAccess: *mut u32,
        RelativeId: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamOpenUser(
        DomainHandle: *mut std::ffi::c_void,
        DesiredAccess: u32,
        UserId: u32,
        UserHandle: *mut *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamDeleteUser(UserHandle: *mut std::ffi::c_void) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamQueryInformationUser(
        UserHandle: *mut std::ffi::c_void,
        UserInformationClass: USER_INFORMATION_CLASS,
        Buffer: *mut *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamSetInformationUser(
        UserHandle: *mut std::ffi::c_void,
        UserInformationClass: USER_INFORMATION_CLASS,
        Buffer: *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamGetGroupsForUser(
        UserHandle: *mut std::ffi::c_void,
        Groups: *mut *mut GROUP_MEMBERSHIP,
        MembershipCount: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamChangePasswordUser(
        UserHandle: *mut std::ffi::c_void,
        OldPassword: *mut UNICODE_STRING,
        NewPassword: *mut UNICODE_STRING,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamChangePasswordUser2(
        ServerName: *mut UNICODE_STRING,
        UserName: *mut UNICODE_STRING,
        OldPassword: *mut UNICODE_STRING,
        NewPassword: *mut UNICODE_STRING,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamChangePasswordUser3(
        ServerName: *mut UNICODE_STRING,
        UserName: *mut UNICODE_STRING,
        OldPassword: *mut UNICODE_STRING,
        NewPassword: *mut UNICODE_STRING,
        EffectivePasswordPolicy: *mut *mut DOMAIN_PASSWORD_INFORMATION,
        PasswordChangeFailureInfo: *mut *mut USER_PWD_CHANGE_FAILURE_INFORMATION,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamQueryDisplayInformation(
        DomainHandle: *mut std::ffi::c_void,
        DisplayInformation: DOMAIN_DISPLAY_INFORMATION,
        Index: u32,
        EntryCount: u32,
        PreferredMaximumLength: u32,
        TotalAvailable: *mut u32,
        TotalReturned: *mut u32,
        ReturnedEntryCount: *mut u32,
        SortedBuffer: *mut *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamGetDisplayEnumerationIndex(
        DomainHandle: *mut std::ffi::c_void,
        DisplayInformation: DOMAIN_DISPLAY_INFORMATION,
        Prefix: *mut UNICODE_STRING,
        Index: *mut u32,
    ) -> NTSTATUS;
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SECURITY_DB_DELTA_TYPE {
    SecurityDbNew = 1,
    SecurityDbRename = 2,
    SecurityDbDelete = 3,
    SecurityDbChangeMemberAdd = 4,
    SecurityDbChangeMemberSet = 5,
    SecurityDbChangeMemberDel = 6,
    SecurityDbChange = 7,
    SecurityDbChangePassword = 8,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SECURITY_DB_OBJECT_TYPE {
    SecurityDbObjectSamDomain = 1,
    SecurityDbObjectSamUser = 2,
    SecurityDbObjectSamGroup = 3,
    SecurityDbObjectSamAlias = 4,
    SecurityDbObjectLsaPolicy = 5,
    SecurityDbObjectLsaTDomain = 6,
    SecurityDbObjectLsaAccount = 7,
    SecurityDbObjectLsaSecret = 8,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SAM_ACCOUNT_TYPE {
    SamObjectUser = 1,
    SamObjectGroup = 2,
    SamObjectAlias = 3,
}

#[repr(C)]
pub struct SAM_GROUP_MEMBER_ID {
    pub MemberRid: u32,
}

impl Default for SAM_GROUP_MEMBER_ID {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for SAM_GROUP_MEMBER_ID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SAM_GROUP_MEMBER_ID {{  }}")
    }
}

#[repr(C)]
pub struct SAM_ALIAS_MEMBER_ID {
    pub MemberSid: PSID,
}

impl Default for SAM_ALIAS_MEMBER_ID {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for SAM_ALIAS_MEMBER_ID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SAM_ALIAS_MEMBER_ID {{  }}")
    }
}

#[repr(C)]
pub struct SAM_DELTA_DATA {
    pub GroupMemberId: UnionField<SAM_GROUP_MEMBER_ID>,
    pub AliasMemberId: UnionField<SAM_ALIAS_MEMBER_ID>,
    pub AccountControl: UnionField<u32>,
    pub union_field: u64,
}

impl Default for SAM_DELTA_DATA {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for SAM_DELTA_DATA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SAM_DELTA_DATA {{ union }}")
    }
}

pub type PSAM_DELTA_NOTIFICATION_ROUTINE = std::option::Option<
    unsafe extern "system" fn(
        DomainSid: PSID,
        DeltaType: SECURITY_DB_DELTA_TYPE,
        ObjectType: SECURITY_DB_OBJECT_TYPE,
        ObjectRid: u32,
        ObjectName: *mut UNICODE_STRING,
        ModifiedCount: *mut i64,
        DeltaData: *mut SAM_DELTA_DATA,
    ) -> NTSTATUS,
>;

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamRegisterObjectChangeNotification(
        ObjectType: SECURITY_DB_OBJECT_TYPE,
        NotificationEventHandle: HANDLE,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamUnregisterObjectChangeNotification(
        ObjectType: SECURITY_DB_OBJECT_TYPE,
        NotificationEventHandle: HANDLE,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamGetCompatibilityMode(ObjectHandle: *mut std::ffi::c_void, Mode: *mut u32)
    -> NTSTATUS;
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum PASSWORD_POLICY_VALIDATION_TYPE {
    SamValidateAuthentication = 1,
    SamValidatePasswordChange = 2,
    SamValidatePasswordReset = 3,
}

#[repr(C)]
pub struct SAM_VALIDATE_PASSWORD_HASH {
    pub Length: u32,
    pub Hash: *mut u8,
}

impl Default for SAM_VALIDATE_PASSWORD_HASH {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for SAM_VALIDATE_PASSWORD_HASH {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SAM_VALIDATE_PASSWORD_HASH {{  }}")
    }
}

#[repr(C)]
pub struct SAM_VALIDATE_PERSISTED_FIELDS {
    pub PresentFields: u32,
    pub PasswordLastSet: i64,
    pub BadPasswordTime: i64,
    pub LockoutTime: i64,
    pub BadPasswordCount: u32,
    pub PasswordHistoryLength: u32,
    pub PasswordHistory: *mut SAM_VALIDATE_PASSWORD_HASH,
}

impl Default for SAM_VALIDATE_PERSISTED_FIELDS {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for SAM_VALIDATE_PERSISTED_FIELDS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SAM_VALIDATE_PERSISTED_FIELDS {{ PasswordHistory: {:?} }}",
            self.PasswordHistory
        )
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SAM_VALIDATE_VALIDATION_STATUS {
    SamValidateSuccess = 0,
    SamValidatePasswordMustChange = 1,
    SamValidateAccountLockedOut = 2,
    SamValidatePasswordExpired = 3,
    SamValidatePasswordIncorrect = 4,
    SamValidatePasswordIsInHistory = 5,
    SamValidatePasswordTooShort = 6,
    SamValidatePasswordTooLong = 7,
    SamValidatePasswordNotComplexEnough = 8,
    SamValidatePasswordTooRecent = 9,
    SamValidatePasswordFilterError = 10,
}

#[repr(C)]
pub struct SAM_VALIDATE_STANDARD_OUTPUT_ARG {
    pub ChangedPersistedFields: SAM_VALIDATE_PERSISTED_FIELDS,
    pub ValidationStatus: SAM_VALIDATE_VALIDATION_STATUS,
}

impl Default for SAM_VALIDATE_STANDARD_OUTPUT_ARG {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for SAM_VALIDATE_STANDARD_OUTPUT_ARG {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SAM_VALIDATE_STANDARD_OUTPUT_ARG {{ ChangedPersistedFields: {:?}, ValidationStatus: {:?} }}",
            self.ChangedPersistedFields, self.ValidationStatus
        )
    }
}

#[repr(C)]
pub struct SAM_VALIDATE_AUTHENTICATION_INPUT_ARG {
    pub InputPersistedFields: SAM_VALIDATE_PERSISTED_FIELDS,
    pub PasswordMatched: bool,
}

impl Default for SAM_VALIDATE_AUTHENTICATION_INPUT_ARG {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for SAM_VALIDATE_AUTHENTICATION_INPUT_ARG {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SAM_VALIDATE_AUTHENTICATION_INPUT_ARG {{ InputPersistedFields: {:?} }}",
            self.InputPersistedFields
        )
    }
}

#[repr(C)]
pub struct SAM_VALIDATE_PASSWORD_CHANGE_INPUT_ARG {
    pub InputPersistedFields: SAM_VALIDATE_PERSISTED_FIELDS,
    pub ClearPassword: UNICODE_STRING,
    pub UserAccountName: UNICODE_STRING,
    pub HashedPassword: SAM_VALIDATE_PASSWORD_HASH,
    pub PasswordMatch: bool,
}

impl Default for SAM_VALIDATE_PASSWORD_CHANGE_INPUT_ARG {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for SAM_VALIDATE_PASSWORD_CHANGE_INPUT_ARG {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SAM_VALIDATE_PASSWORD_CHANGE_INPUT_ARG {{ InputPersistedFields: {:?}, HashedPassword: {:?} }}",
            self.InputPersistedFields, self.HashedPassword
        )
    }
}

#[repr(C)]
pub struct SAM_VALIDATE_PASSWORD_RESET_INPUT_ARG {
    pub InputPersistedFields: SAM_VALIDATE_PERSISTED_FIELDS,
    pub ClearPassword: UNICODE_STRING,
    pub UserAccountName: UNICODE_STRING,
    pub HashedPassword: SAM_VALIDATE_PASSWORD_HASH,
    pub PasswordMustChangeAtNextLogon: bool,
    pub ClearLockout: bool,
}

impl Default for SAM_VALIDATE_PASSWORD_RESET_INPUT_ARG {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for SAM_VALIDATE_PASSWORD_RESET_INPUT_ARG {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SAM_VALIDATE_PASSWORD_RESET_INPUT_ARG {{ InputPersistedFields: {:?}, HashedPassword: {:?} }}",
            self.InputPersistedFields, self.HashedPassword
        )
    }
}

#[repr(C)]
pub struct SAM_VALIDATE_INPUT_ARG {
    pub ValidateAuthenticationInput: UnionField<SAM_VALIDATE_AUTHENTICATION_INPUT_ARG>,
    pub ValidatePasswordChangeInput: UnionField<SAM_VALIDATE_PASSWORD_CHANGE_INPUT_ARG>,
    pub ValidatePasswordResetInput: UnionField<SAM_VALIDATE_PASSWORD_RESET_INPUT_ARG>,
    pub union_field: [u64; 13],
}

impl Default for SAM_VALIDATE_INPUT_ARG {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for SAM_VALIDATE_INPUT_ARG {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SAM_VALIDATE_INPUT_ARG {{ union }}")
    }
}

#[repr(C)]
pub struct SAM_VALIDATE_OUTPUT_ARG {
    pub ValidateAuthenticationOutput: UnionField<SAM_VALIDATE_STANDARD_OUTPUT_ARG>,
    pub ValidatePasswordChangeOutput: UnionField<SAM_VALIDATE_STANDARD_OUTPUT_ARG>,
    pub ValidatePasswordResetOutput: UnionField<SAM_VALIDATE_STANDARD_OUTPUT_ARG>,
    pub union_field: [u64; 7],
}

impl Default for SAM_VALIDATE_OUTPUT_ARG {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for SAM_VALIDATE_OUTPUT_ARG {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SAM_VALIDATE_OUTPUT_ARG {{ union }}")
    }
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamValidatePassword(
        ServerName: *mut UNICODE_STRING,
        ValidationType: PASSWORD_POLICY_VALIDATION_TYPE,
        InputArg: *mut SAM_VALIDATE_INPUT_ARG,
        OutputArg: *mut *mut SAM_VALIDATE_OUTPUT_ARG,
    ) -> NTSTATUS;
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SAM_GENERIC_OPERATION_TYPE {
    SamObjectChangeNotificationOperation = 0,
}

#[repr(C)]
pub struct SAM_OPERATION_OBJCHG_INPUT {
    pub Register: bool,
    pub EventHandle: u64,
    pub ObjectType: SECURITY_DB_OBJECT_TYPE,
    pub ProcessID: u32,
}

impl Default for SAM_OPERATION_OBJCHG_INPUT {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for SAM_OPERATION_OBJCHG_INPUT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SAM_OPERATION_OBJCHG_INPUT {{ ObjectType: {:?} }}",
            self.ObjectType
        )
    }
}

#[repr(C)]
pub struct SAM_OPERATION_OBJCHG_OUTPUT {
    pub Reserved: u32,
}

impl Default for SAM_OPERATION_OBJCHG_OUTPUT {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for SAM_OPERATION_OBJCHG_OUTPUT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SAM_OPERATION_OBJCHG_OUTPUT {{  }}")
    }
}

#[repr(C)]
pub struct SAM_GENERIC_OPERATION_INPUT {
    pub ObjChangeIn: UnionField<SAM_OPERATION_OBJCHG_INPUT>,
    pub union_field: [u64; 3],
}

impl Default for SAM_GENERIC_OPERATION_INPUT {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for SAM_GENERIC_OPERATION_INPUT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SAM_GENERIC_OPERATION_INPUT {{ union }}")
    }
}

#[repr(C)]
pub struct SAM_GENERIC_OPERATION_OUTPUT {
    pub ObjChangeOut: UnionField<SAM_OPERATION_OBJCHG_OUTPUT>,
    pub union_field: u32,
}

impl Default for SAM_GENERIC_OPERATION_OUTPUT {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for SAM_GENERIC_OPERATION_OUTPUT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SAM_GENERIC_OPERATION_OUTPUT {{ union }}")
    }
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn SamPerformGenericOperation(
        ServerName: PWSTR,
        OperationType: SAM_GENERIC_OPERATION_TYPE,
        OperationIn: *mut SAM_GENERIC_OPERATION_INPUT,
        OperationOut: *mut *mut SAM_GENERIC_OPERATION_OUTPUT,
    ) -> NTSTATUS;
}
