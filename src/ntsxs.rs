use windows::{
    core::GUID,
    Win32::{
        Foundation::{HANDLE, UNICODE_STRING},
        System::{
            ApplicationInstallationAndServicing::{
                ACTCTX_COMPATIBILITY_ELEMENT_TYPE, ACTCTX_REQUESTED_RUN_LEVEL,
            },
            Kernel::LIST_ENTRY,
        },
    },
};

pub const ACTIVATION_CONTEXT_DATA_FORMAT_WHISTLER: u32 = 1;
pub const ACTIVATION_CONTEXT_FLAG_NO_INHERIT: u32 = 1;
pub const ACTIVATION_CONTEXT_DATA_TOC_HEADER_DENSE: u32 = 1;
pub const ACTIVATION_CONTEXT_DATA_TOC_HEADER_INORDER: u32 = 2;
pub const ACTIVATION_CONTEXT_DATA_ASSEMBLY_ROSTER_ENTRY_INVALID: u32 = 1;
pub const ACTIVATION_CONTEXT_DATA_ASSEMBLY_ROSTER_ENTRY_ROOT: u32 = 2;
pub const ACTIVATION_CONTEXT_SECTION_FORMAT_UNKNOWN: u32 = 0;
pub const ACTIVATION_CONTEXT_SECTION_FORMAT_STRING_TABLE: u32 = 1;
pub const ACTIVATION_CONTEXT_SECTION_FORMAT_GUID_TABLE: u32 = 2;
pub const ACTIVATION_CONTEXT_STRING_SECTION_FORMAT_WHISTLER: u32 = 1;
pub const ACTIVATION_CONTEXT_STRING_SECTION_CASE_INSENSITIVE: u32 = 1;
pub const ACTIVATION_CONTEXT_STRING_SECTION_ENTRIES_IN_PSEUDOKEY_ORDER: u32 = 2;
pub const ACTIVATION_CONTEXT_GUID_SECTION_FORMAT_WHISTLER: u32 = 1;
pub const ACTIVATION_CONTEXT_GUID_SECTION_ENTRIES_IN_ORDER: u32 = 1;
pub const ACTIVATION_CONTEXT_DATA_ASSEMBLY_INFORMATION_FORMAT_WHISTLER: u32 = 1;
pub const ACTIVATION_CONTEXT_DATA_ASSEMBLY_INFORMATION_ROOT_ASSEMBLY: u32 = 1;
pub const ACTIVATION_CONTEXT_DATA_ASSEMBLY_INFORMATION_POLICY_APPLIED: u32 = 2;
pub const ACTIVATION_CONTEXT_DATA_ASSEMBLY_INFORMATION_ASSEMBLY_POLICY_APPLIED: u32 = 4;
pub const ACTIVATION_CONTEXT_DATA_ASSEMBLY_INFORMATION_ROOT_POLICY_APPLIED: u32 = 8;
pub const ACTIVATION_CONTEXT_DATA_ASSEMBLY_INFORMATION_PRIVATE_ASSEMBLY: u32 = 16;
pub const ACTIVATION_CONTEXT_DATA_DLL_REDIRECTION_FORMAT_WHISTLER: u32 = 1;
pub const ACTIVATION_CONTEXT_DATA_DLL_REDIRECTION_PATH_INCLUDES_BASE_NAME: u32 = 1;
pub const ACTIVATION_CONTEXT_DATA_DLL_REDIRECTION_PATH_OMITS_ASSEMBLY_ROOT: u32 = 2;
pub const ACTIVATION_CONTEXT_DATA_DLL_REDIRECTION_PATH_EXPAND: u32 = 4;
pub const ACTIVATION_CONTEXT_DATA_DLL_REDIRECTION_PATH_SYSTEM_DEFAULT_REDIRECTED_SYSTEM32_DLL: u32 =
    8;
pub const ACTIVATION_CONTEXT_DATA_WINDOW_CLASS_REDIRECTION_FORMAT_WHISTLER: u32 = 1;
pub const ACTIVATION_CONTEXT_DATA_COM_SERVER_REDIRECTION_FORMAT_WHISTLER: u32 = 1;
pub const ACTIVATION_CONTEXT_DATA_COM_SERVER_REDIRECTION_THREADING_MODEL_INVALID: u32 = 0;
pub const ACTIVATION_CONTEXT_DATA_COM_SERVER_REDIRECTION_THREADING_MODEL_APARTMENT: u32 = 1;
pub const ACTIVATION_CONTEXT_DATA_COM_SERVER_REDIRECTION_THREADING_MODEL_FREE: u32 = 2;
pub const ACTIVATION_CONTEXT_DATA_COM_SERVER_REDIRECTION_THREADING_MODEL_SINGLE: u32 = 3;
pub const ACTIVATION_CONTEXT_DATA_COM_SERVER_REDIRECTION_THREADING_MODEL_BOTH: u32 = 4;
pub const ACTIVATION_CONTEXT_DATA_COM_SERVER_REDIRECTION_THREADING_MODEL_NEUTRAL: u32 = 5;
pub const ACTIVATION_CONTEXT_DATA_COM_SERVER_MISCSTATUS_FLAG_OFFSET: u32 = 8;
pub const ACTIVATION_CONTEXT_DATA_COM_SERVER_MISCSTATUS_HAS_DEFAULT: u32 = 256;
pub const ACTIVATION_CONTEXT_DATA_COM_SERVER_MISCSTATUS_HAS_ICON: u32 = 512;
pub const ACTIVATION_CONTEXT_DATA_COM_SERVER_MISCSTATUS_HAS_CONTENT: u32 = 1024;
pub const ACTIVATION_CONTEXT_DATA_COM_SERVER_MISCSTATUS_HAS_THUMBNAIL: u32 = 2048;
pub const ACTIVATION_CONTEXT_DATA_COM_SERVER_MISCSTATUS_HAS_DOCPRINT: u32 = 4096;
pub const ACTIVATION_CONTEXT_DATA_COM_SERVER_REDIRECTION_SHIM_TYPE_OTHER: u32 = 1;
pub const ACTIVATION_CONTEXT_DATA_COM_SERVER_REDIRECTION_SHIM_TYPE_CLR_CLASS: u32 = 2;
pub const ACTIVATION_CONTEXT_DATA_COM_INTERFACE_REDIRECTION_FORMAT_WHISTLER: u32 = 1;
pub const ACTIVATION_CONTEXT_DATA_COM_INTERFACE_REDIRECTION_FLAG_NUM_METHODS_VALID: u32 = 1;
pub const ACTIVATION_CONTEXT_DATA_COM_INTERFACE_REDIRECTION_FLAG_BASE_INTERFACE_VALID: u32 = 2;
pub const ACTIVATION_CONTEXT_DATA_COM_TYPE_LIBRARY_REDIRECTION_FORMAT_WHISTLER: u32 = 1;
pub const ACTIVATION_CONTEXT_DATA_COM_PROGID_REDIRECTION_FORMAT_WHISTLER: u32 = 1;
pub const ACTIVATION_CONTEXT_DATA_CLR_SURROGATE_FORMAT_WHISTLER: u32 = 1;
pub const ACTIVATION_CONTEXT_DATA_APPLICATION_SETTINGS_FORMAT_LONGHORN: u32 = 1;
pub const SXS_WINDOWS_SETTINGS_NAMESPACE: &[u8; 54] =
    b"http://schemas.microsoft.com/SMI/2005/WindowsSettings\0";
pub const SXS_WINDOWS_SETTINGS_2011_NAMESPACE: &[u8; 54] =
    b"http://schemas.microsoft.com/SMI/2011/WindowsSettings\0";
pub const SXS_WINDOWS_SETTINGS_2013_NAMESPACE: &[u8; 54] =
    b"http://schemas.microsoft.com/SMI/2013/WindowsSettings\0";
pub const SXS_WINDOWS_SETTINGS_2014_NAMESPACE: &[u8; 54] =
    b"http://schemas.microsoft.com/SMI/2014/WindowsSettings\0";
pub const SXS_WINDOWS_SETTINGS_2016_NAMESPACE: &[u8; 54] =
    b"http://schemas.microsoft.com/SMI/2016/WindowsSettings\0";
pub const SXS_WINDOWS_SETTINGS_2017_NAMESPACE: &[u8; 54] =
    b"http://schemas.microsoft.com/SMI/2017/WindowsSettings\0";
pub const SXS_WINDOWS_SETTINGS_2019_NAMESPACE: &[u8; 54] =
    b"http://schemas.microsoft.com/SMI/2019/WindowsSettings\0";
pub const SXS_WINDOWS_SETTINGS_2020_NAMESPACE: &[u8; 54] =
    b"http://schemas.microsoft.com/SMI/2020/WindowsSettings\0";
pub const ASSEMBLY_STORAGE_MAP_ASSEMBLY_ARRAY_IS_HEAP_ALLOCATED: u32 = 1;
pub const ACTIVATION_CONTEXT_NOTIFICATION_DESTROY: u32 = 1;
pub const ACTIVATION_CONTEXT_NOTIFICATION_ZOMBIFY: u32 = 2;
pub const ACTIVATION_CONTEXT_NOTIFICATION_USED: u32 = 3;
pub const RTL_ACTIVATION_CONTEXT_STACK_FRAME_FLAG_RELEASE_ON_DEACTIVATION: u32 = 1;
pub const RTL_ACTIVATION_CONTEXT_STACK_FRAME_FLAG_NO_DEACTIVATE: u32 = 2;
pub const RTL_ACTIVATION_CONTEXT_STACK_FRAME_FLAG_ON_FREE_LIST: u32 = 4;
pub const RTL_ACTIVATION_CONTEXT_STACK_FRAME_FLAG_HEAP_ALLOCATED: u32 = 8;
pub const RTL_ACTIVATION_CONTEXT_STACK_FRAME_FLAG_NOT_REALLY_ACTIVATED: u32 = 16;
pub const ACTIVATION_CONTEXT_STACK_FLAG_QUERIES_DISABLED: u32 = 1;
pub const ACTIVATION_CONTEXT_DATA_MAGIC: &[u8; 4] = b"xtcA";
pub const ACTIVATION_CONTEXT_STRING_SECTION_MAGIC: &[u8; 4] = b"dHsS";
pub const ACTIVATION_CONTEXT_GUID_SECTION_MAGIC: &[u8; 4] = b"dHsG";

#[repr(C)]
pub struct ACTIVATION_CONTEXT_DATA {
    pub Magic: u32,
    pub HeaderSize: u32,
    pub FormatVersion: u32,
    pub TotalSize: u32,
    pub DefaultTocOffset: u32,
    pub ExtendedTocOffset: u32,
    pub AssemblyRosterOffset: u32,
    pub Flags: u32,
}

impl Default for ACTIVATION_CONTEXT_DATA {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ACTIVATION_CONTEXT_DATA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ACTIVATION_CONTEXT_DATA {{  }}")
    }
}

#[repr(C)]
pub struct ACTIVATION_CONTEXT_DATA_TOC_HEADER {
    pub HeaderSize: u32,
    pub EntryCount: u32,
    pub FirstEntryOffset: u32,
    pub Flags: u32,
}

impl Default for ACTIVATION_CONTEXT_DATA_TOC_HEADER {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ACTIVATION_CONTEXT_DATA_TOC_HEADER {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ACTIVATION_CONTEXT_DATA_TOC_HEADER {{  }}")
    }
}

#[repr(C)]
pub struct ACTIVATION_CONTEXT_DATA_TOC_ENTRY {
    pub Id: u32,
    pub Offset: u32,
    pub Length: u32,
    pub Format: u32,
}

impl Default for ACTIVATION_CONTEXT_DATA_TOC_ENTRY {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ACTIVATION_CONTEXT_DATA_TOC_ENTRY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ACTIVATION_CONTEXT_DATA_TOC_ENTRY {{  }}")
    }
}

#[repr(C)]
pub struct ACTIVATION_CONTEXT_DATA_EXTENDED_TOC_HEADER {
    pub HeaderSize: u32,
    pub EntryCount: u32,
    pub FirstEntryOffset: u32,
    pub Flags: u32,
}

impl Default for ACTIVATION_CONTEXT_DATA_EXTENDED_TOC_HEADER {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ACTIVATION_CONTEXT_DATA_EXTENDED_TOC_HEADER {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ACTIVATION_CONTEXT_DATA_EXTENDED_TOC_HEADER {{  }}")
    }
}

#[repr(C)]
pub struct ACTIVATION_CONTEXT_DATA_EXTENDED_TOC_ENTRY {
    pub ExtensionGuid: GUID,
    pub TocOffset: u32,
    pub Length: u32,
}

impl Default for ACTIVATION_CONTEXT_DATA_EXTENDED_TOC_ENTRY {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ACTIVATION_CONTEXT_DATA_EXTENDED_TOC_ENTRY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ACTIVATION_CONTEXT_DATA_EXTENDED_TOC_ENTRY {{  }}")
    }
}

#[repr(C)]
pub struct ACTIVATION_CONTEXT_DATA_ASSEMBLY_ROSTER_HEADER {
    pub HeaderSize: u32,
    pub HashAlgorithm: u32,
    pub EntryCount: u32,
    pub FirstEntryOffset: u32,
    pub AssemblyInformationSectionOffset: u32,
}

impl Default for ACTIVATION_CONTEXT_DATA_ASSEMBLY_ROSTER_HEADER {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ACTIVATION_CONTEXT_DATA_ASSEMBLY_ROSTER_HEADER {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ACTIVATION_CONTEXT_DATA_ASSEMBLY_ROSTER_HEADER {{  }}")
    }
}

#[repr(C)]
pub struct ACTIVATION_CONTEXT_DATA_ASSEMBLY_ROSTER_ENTRY {
    pub Flags: u32,
    pub PseudoKey: u32,
    pub AssemblyNameOffset: u32,
    pub AssemblyNameLength: u32,
    pub AssemblyInformationOffset: u32,
    pub AssemblyInformationLength: u32,
}

impl Default for ACTIVATION_CONTEXT_DATA_ASSEMBLY_ROSTER_ENTRY {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ACTIVATION_CONTEXT_DATA_ASSEMBLY_ROSTER_ENTRY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ACTIVATION_CONTEXT_DATA_ASSEMBLY_ROSTER_ENTRY {{  }}")
    }
}

#[repr(C)]
pub struct ACTIVATION_CONTEXT_STRING_SECTION_HEADER {
    pub Magic: u32,
    pub HeaderSize: u32,
    pub FormatVersion: u32,
    pub DataFormatVersion: u32,
    pub Flags: u32,
    pub ElementCount: u32,
    pub ElementListOffset: u32,
    pub HashAlgorithm: u32,
    pub SearchStructureOffset: u32,
    pub UserDataOffset: u32,
    pub UserDataSize: u32,
}

impl Default for ACTIVATION_CONTEXT_STRING_SECTION_HEADER {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ACTIVATION_CONTEXT_STRING_SECTION_HEADER {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ACTIVATION_CONTEXT_STRING_SECTION_HEADER {{  }}")
    }
}

#[repr(C)]
pub struct ACTIVATION_CONTEXT_STRING_SECTION_ENTRY {
    pub PseudoKey: u32,
    pub KeyOffset: u32,
    pub KeyLength: u32,
    pub Offset: u32,
    pub Length: u32,
    pub AssemblyRosterIndex: u32,
}

impl Default for ACTIVATION_CONTEXT_STRING_SECTION_ENTRY {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ACTIVATION_CONTEXT_STRING_SECTION_ENTRY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ACTIVATION_CONTEXT_STRING_SECTION_ENTRY {{  }}")
    }
}

#[repr(C)]
pub struct ACTIVATION_CONTEXT_STRING_SECTION_HASH_TABLE {
    pub BucketTableEntryCount: u32,
    pub BucketTableOffset: u32,
}

impl Default for ACTIVATION_CONTEXT_STRING_SECTION_HASH_TABLE {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ACTIVATION_CONTEXT_STRING_SECTION_HASH_TABLE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ACTIVATION_CONTEXT_STRING_SECTION_HASH_TABLE {{  }}")
    }
}

#[repr(C)]
pub struct ACTIVATION_CONTEXT_STRING_SECTION_HASH_BUCKET {
    pub ChainCount: u32,
    pub ChainOffset: u32,
}

impl Default for ACTIVATION_CONTEXT_STRING_SECTION_HASH_BUCKET {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ACTIVATION_CONTEXT_STRING_SECTION_HASH_BUCKET {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ACTIVATION_CONTEXT_STRING_SECTION_HASH_BUCKET {{  }}")
    }
}

#[repr(C)]
pub struct ACTIVATION_CONTEXT_GUID_SECTION_HEADER {
    pub Magic: u32,
    pub HeaderSize: u32,
    pub FormatVersion: u32,
    pub DataFormatVersion: u32,
    pub Flags: u32,
    pub ElementCount: u32,
    pub ElementListOffset: u32,
    pub SearchStructureOffset: u32,
    pub UserDataOffset: u32,
    pub UserDataSize: u32,
}

impl Default for ACTIVATION_CONTEXT_GUID_SECTION_HEADER {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ACTIVATION_CONTEXT_GUID_SECTION_HEADER {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ACTIVATION_CONTEXT_GUID_SECTION_HEADER {{  }}")
    }
}

#[repr(C)]
pub struct ACTIVATION_CONTEXT_GUID_SECTION_ENTRY {
    pub Guid: GUID,
    pub Offset: u32,
    pub Length: u32,
    pub AssemblyRosterIndex: u32,
}

impl Default for ACTIVATION_CONTEXT_GUID_SECTION_ENTRY {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ACTIVATION_CONTEXT_GUID_SECTION_ENTRY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ACTIVATION_CONTEXT_GUID_SECTION_ENTRY {{  }}")
    }
}

#[repr(C)]
pub struct ACTIVATION_CONTEXT_GUID_SECTION_HASH_TABLE {
    pub BucketTableEntryCount: u32,
    pub BucketTableOffset: u32,
}

impl Default for ACTIVATION_CONTEXT_GUID_SECTION_HASH_TABLE {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ACTIVATION_CONTEXT_GUID_SECTION_HASH_TABLE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ACTIVATION_CONTEXT_GUID_SECTION_HASH_TABLE {{  }}")
    }
}

#[repr(C)]
pub struct ACTIVATION_CONTEXT_GUID_SECTION_HASH_BUCKET {
    pub ChainCount: u32,
    pub ChainOffset: u32,
}

impl Default for ACTIVATION_CONTEXT_GUID_SECTION_HASH_BUCKET {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ACTIVATION_CONTEXT_GUID_SECTION_HASH_BUCKET {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ACTIVATION_CONTEXT_GUID_SECTION_HASH_BUCKET {{  }}")
    }
}

#[repr(C, packed(4))]
pub struct ACTIVATION_CONTEXT_DATA_ASSEMBLY_INFORMATION {
    pub Size: u32,
    pub Flags: u32,
    pub EncodedAssemblyIdentityLength: u32,
    pub EncodedAssemblyIdentityOffset: u32,
    pub ManifestPathType: u32,
    pub ManifestPathLength: u32,
    pub ManifestPathOffset: u32,
    pub ManifestLastWriteTime: i64,
    pub PolicyPathType: u32,
    pub PolicyPathLength: u32,
    pub PolicyPathOffset: u32,
    pub PolicyLastWriteTime: i64,
    pub MetadataSatelliteRosterIndex: u32,
    pub Unused2: u32,
    pub ManifestVersionMajor: u32,
    pub ManifestVersionMinor: u32,
    pub PolicyVersionMajor: u32,
    pub PolicyVersionMinor: u32,
    pub AssemblyDirectoryNameLength: u32,
    pub AssemblyDirectoryNameOffset: u32,
    pub NumOfFilesInAssembly: u32,
    pub LanguageLength: u32,
    pub LanguageOffset: u32,
    pub RunLevel: ACTCTX_REQUESTED_RUN_LEVEL,
    pub UiAccess: u32,
}

impl Default for ACTIVATION_CONTEXT_DATA_ASSEMBLY_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ACTIVATION_CONTEXT_DATA_ASSEMBLY_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ACTIVATION_CONTEXT_DATA_ASSEMBLY_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct ACTIVATION_CONTEXT_DATA_ASSEMBLY_GLOBAL_INFORMATION {
    pub Size: u32,
    pub Flags: u32,
    pub PolicyCoherencyGuid: GUID,
    pub PolicyOverrideGuid: GUID,
    pub ApplicationDirectoryPathType: u32,
    pub ApplicationDirectoryLength: u32,
    pub ApplicationDirectoryOffset: u32,
    pub ResourceName: u32,
}

impl Default for ACTIVATION_CONTEXT_DATA_ASSEMBLY_GLOBAL_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ACTIVATION_CONTEXT_DATA_ASSEMBLY_GLOBAL_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ACTIVATION_CONTEXT_DATA_ASSEMBLY_GLOBAL_INFORMATION {{  }}"
        )
    }
}

#[repr(C)]
pub struct ACTIVATION_CONTEXT_DATA_DLL_REDIRECTION {
    pub Size: u32,
    pub Flags: u32,
    pub TotalPathLength: u32,
    pub PathSegmentCount: u32,
    pub PathSegmentOffset: u32,
}

impl Default for ACTIVATION_CONTEXT_DATA_DLL_REDIRECTION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ACTIVATION_CONTEXT_DATA_DLL_REDIRECTION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ACTIVATION_CONTEXT_DATA_DLL_REDIRECTION {{  }}")
    }
}

#[repr(C)]
pub struct ACTIVATION_CONTEXT_DATA_DLL_REDIRECTION_PATH_SEGMENT {
    pub Length: u32,
    pub Offset: u32,
}

impl Default for ACTIVATION_CONTEXT_DATA_DLL_REDIRECTION_PATH_SEGMENT {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ACTIVATION_CONTEXT_DATA_DLL_REDIRECTION_PATH_SEGMENT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ACTIVATION_CONTEXT_DATA_DLL_REDIRECTION_PATH_SEGMENT {{  }}"
        )
    }
}

#[repr(C)]
pub struct ACTIVATION_CONTEXT_DATA_WINDOW_CLASS_REDIRECTION {
    pub Size: u32,
    pub Flags: u32,
    pub VersionSpecificClassNameLength: u32,
    pub VersionSpecificClassNameOffset: u32,
    pub DllNameLength: u32,
    pub DllNameOffset: u32,
}

impl Default for ACTIVATION_CONTEXT_DATA_WINDOW_CLASS_REDIRECTION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ACTIVATION_CONTEXT_DATA_WINDOW_CLASS_REDIRECTION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ACTIVATION_CONTEXT_DATA_WINDOW_CLASS_REDIRECTION {{  }}")
    }
}

#[repr(C)]
pub struct ACTIVATION_CONTEXT_DATA_COM_SERVER_REDIRECTION {
    pub Size: u32,
    pub Flags: u32,
    pub ThreadingModel: u32,
    pub ReferenceClsid: GUID,
    pub ConfiguredClsid: GUID,
    pub ImplementedClsid: GUID,
    pub TypeLibraryId: GUID,
    pub ModuleLength: u32,
    pub ModuleOffset: u32,
    pub ProgIdLength: u32,
    pub ProgIdOffset: u32,
    pub ShimDataLength: u32,
    pub ShimDataOffset: u32,
    pub MiscStatusDefault: u32,
    pub MiscStatusContent: u32,
    pub MiscStatusThumbnail: u32,
    pub MiscStatusIcon: u32,
    pub MiscStatusDocPrint: u32,
}

impl Default for ACTIVATION_CONTEXT_DATA_COM_SERVER_REDIRECTION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ACTIVATION_CONTEXT_DATA_COM_SERVER_REDIRECTION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ACTIVATION_CONTEXT_DATA_COM_SERVER_REDIRECTION {{  }}")
    }
}

#[repr(C)]
pub struct ACTIVATION_CONTEXT_DATA_COM_SERVER_REDIRECTION_SHIM {
    pub Size: u32,
    pub Flags: u32,
    pub Type: u32,
    pub ModuleLength: u32,
    pub ModuleOffset: u32,
    pub TypeLength: u32,
    pub TypeOffset: u32,
    pub ShimVersionLength: u32,
    pub ShimVersionOffset: u32,
    pub DataLength: u32,
    pub DataOffset: u32,
}

impl Default for ACTIVATION_CONTEXT_DATA_COM_SERVER_REDIRECTION_SHIM {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ACTIVATION_CONTEXT_DATA_COM_SERVER_REDIRECTION_SHIM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ACTIVATION_CONTEXT_DATA_COM_SERVER_REDIRECTION_SHIM {{  }}"
        )
    }
}

#[repr(C)]
pub struct ACTIVATION_CONTEXT_DATA_COM_INTERFACE_REDIRECTION {
    pub Size: u32,
    pub Flags: u32,
    pub ProxyStubClsid32: GUID,
    pub NumMethods: u32,
    pub TypeLibraryId: GUID,
    pub BaseInterface: GUID,
    pub NameLength: u32,
    pub NameOffset: u32,
}

impl Default for ACTIVATION_CONTEXT_DATA_COM_INTERFACE_REDIRECTION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ACTIVATION_CONTEXT_DATA_COM_INTERFACE_REDIRECTION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ACTIVATION_CONTEXT_DATA_COM_INTERFACE_REDIRECTION {{  }}"
        )
    }
}

#[repr(C)]
pub struct ACTIVATION_CONTEXT_DATA_TYPE_LIBRARY_VERSION {
    pub Major: u16,
    pub Minor: u16,
}

impl Default for ACTIVATION_CONTEXT_DATA_TYPE_LIBRARY_VERSION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ACTIVATION_CONTEXT_DATA_TYPE_LIBRARY_VERSION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ACTIVATION_CONTEXT_DATA_TYPE_LIBRARY_VERSION {{  }}")
    }
}

#[repr(C)]
pub struct ACTIVATION_CONTEXT_DATA_COM_TYPE_LIBRARY_REDIRECTION {
    pub Size: u32,
    pub Flags: u32,
    pub NameLength: u32,
    pub NameOffset: u32,
    pub ResourceId: u16,
    pub LibraryFlags: u16,
    pub HelpDirLength: u32,
    pub HelpDirOffset: u32,
    pub Version: ACTIVATION_CONTEXT_DATA_TYPE_LIBRARY_VERSION,
}

impl Default for ACTIVATION_CONTEXT_DATA_COM_TYPE_LIBRARY_REDIRECTION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ACTIVATION_CONTEXT_DATA_COM_TYPE_LIBRARY_REDIRECTION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ACTIVATION_CONTEXT_DATA_COM_TYPE_LIBRARY_REDIRECTION {{ Version: {:?} }}",
            self.Version
        )
    }
}

#[repr(C)]
pub struct ACTIVATION_CONTEXT_DATA_COM_PROGID_REDIRECTION {
    pub Size: u32,
    pub Flags: u32,
    pub ConfiguredClsidOffset: u32,
}

impl Default for ACTIVATION_CONTEXT_DATA_COM_PROGID_REDIRECTION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ACTIVATION_CONTEXT_DATA_COM_PROGID_REDIRECTION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ACTIVATION_CONTEXT_DATA_COM_PROGID_REDIRECTION {{  }}")
    }
}

#[repr(C)]
pub struct ACTIVATION_CONTEXT_DATA_CLR_SURROGATE {
    pub Size: u32,
    pub Flags: u32,
    pub SurrogateIdent: GUID,
    pub VersionOffset: u32,
    pub VersionLength: u32,
    pub TypeNameOffset: u32,
    pub TypeNameLength: u32,
}

impl Default for ACTIVATION_CONTEXT_DATA_CLR_SURROGATE {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ACTIVATION_CONTEXT_DATA_CLR_SURROGATE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ACTIVATION_CONTEXT_DATA_CLR_SURROGATE {{  }}")
    }
}

#[repr(C)]
pub struct ACTIVATION_CONTEXT_DATA_APPLICATION_SETTINGS {
    pub Size: u32,
    pub Flags: u32,
    pub SettingNamespaceLength: u32,
    pub SettingNamespaceOffset: u32,
    pub SettingNameLength: u32,
    pub SettingNameOffset: u32,
    pub SettingValueLength: u32,
    pub SettingValueOffset: u32,
}

impl Default for ACTIVATION_CONTEXT_DATA_APPLICATION_SETTINGS {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ACTIVATION_CONTEXT_DATA_APPLICATION_SETTINGS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ACTIVATION_CONTEXT_DATA_APPLICATION_SETTINGS {{  }}")
    }
}

#[repr(C)]
pub struct COMPATIBILITY_CONTEXT_ELEMENT_LEGACY {
    pub Id: GUID,
    pub Type: ACTCTX_COMPATIBILITY_ELEMENT_TYPE,
}

impl Default for COMPATIBILITY_CONTEXT_ELEMENT_LEGACY {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for COMPATIBILITY_CONTEXT_ELEMENT_LEGACY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "COMPATIBILITY_CONTEXT_ELEMENT_LEGACY {{  }}")
    }
}

#[repr(C)]
pub struct ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION_LEGACY {
    pub ElementCount: u32,
    pub Elements: [COMPATIBILITY_CONTEXT_ELEMENT_LEGACY; 1],
}

impl Default for ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION_LEGACY {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION_LEGACY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION_LEGACY {{ Elements: {:?} }}",
            self.Elements
        )
    }
}

#[repr(C)]
pub struct ASSEMBLY_STORAGE_MAP_ENTRY {
    pub Flags: u32,
    pub DosPath: UNICODE_STRING,
    pub Handle: HANDLE,
}

impl Default for ASSEMBLY_STORAGE_MAP_ENTRY {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ASSEMBLY_STORAGE_MAP_ENTRY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ASSEMBLY_STORAGE_MAP_ENTRY {{  }}")
    }
}

#[repr(C)]
pub struct ASSEMBLY_STORAGE_MAP {
    pub Flags: u32,
    pub AssemblyCount: u32,
    pub AssemblyArray: *mut *mut ASSEMBLY_STORAGE_MAP_ENTRY,
}

impl Default for ASSEMBLY_STORAGE_MAP {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ASSEMBLY_STORAGE_MAP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ASSEMBLY_STORAGE_MAP {{ AssemblyArray: {:?} }}",
            self.AssemblyArray
        )
    }
}

pub type PACTIVATION_CONTEXT_NOTIFY_ROUTINE = std::option::Option<
    unsafe extern "system" fn(
        NotificationType: u32,
        ActivationContext: *mut ACTIVATION_CONTEXT,
        ActivationContextData: *mut ACTIVATION_CONTEXT_DATA,
        NotificationContext: *mut std::ffi::c_void,
        NotificationData: *mut std::ffi::c_void,
        DisableThisNotification: *mut bool,
    ),
>;

#[repr(C)]
pub struct ACTIVATION_CONTEXT {
    pub RefCount: i32,
    pub Flags: u32,
    pub ActivationContextData: *mut ACTIVATION_CONTEXT_DATA,
    pub NotificationRoutine: PACTIVATION_CONTEXT_NOTIFY_ROUTINE,
    pub NotificationContext: *mut std::ffi::c_void,
    pub SentNotifications: [u32; 8],
    pub DisabledNotifications: [u32; 8],
    pub StorageMap: ASSEMBLY_STORAGE_MAP,
    pub InlineStorageMapEntries: [*mut ASSEMBLY_STORAGE_MAP_ENTRY; 32],
}

impl Default for ACTIVATION_CONTEXT {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ACTIVATION_CONTEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ACTIVATION_CONTEXT {{ ActivationContextData: {:?}, NotificationRoutine: {:?}, SentNotifications: {:?}, DisabledNotifications: {:?}, StorageMap: {:?}, InlineStorageMapEntries: {:?} }}",
            self.ActivationContextData,
            self.NotificationRoutine,
            self.SentNotifications,
            self.DisabledNotifications,
            self.StorageMap,
            self.InlineStorageMapEntries
        )
    }
}

#[repr(C)]
pub struct RTL_ACTIVATION_CONTEXT_STACK_FRAME {
    pub Previous: *mut RTL_ACTIVATION_CONTEXT_STACK_FRAME,
    pub ActivationContext: *mut ACTIVATION_CONTEXT,
    pub Flags: u32,
}

impl Default for RTL_ACTIVATION_CONTEXT_STACK_FRAME {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_ACTIVATION_CONTEXT_STACK_FRAME {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RTL_ACTIVATION_CONTEXT_STACK_FRAME {{ Previous: {:?} }}",
            self.Previous
        )
    }
}

#[repr(C)]
pub struct ACTIVATION_CONTEXT_STACK {
    pub ActiveFrame: *mut RTL_ACTIVATION_CONTEXT_STACK_FRAME,
    pub FrameListCache: LIST_ENTRY,
    pub Flags: u32,
    pub NextCookieSequenceNumber: u32,
    pub StackId: u32,
}

impl Default for ACTIVATION_CONTEXT_STACK {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ACTIVATION_CONTEXT_STACK {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ACTIVATION_CONTEXT_STACK {{ ActiveFrame: {:?} }}",
            self.ActiveFrame
        )
    }
}
