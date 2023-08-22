use windows::core::{PCWSTR, PWSTR};

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum TAG_INFO_LEVEL {
    eTagInfoLevelNameFromTag = 1,
    eTagInfoLevelNamesReferencingModule = 2,
    eTagInfoLevelNameTagMapping = 3,
    eTagInfoLevelMax = 4,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum TAG_TYPE {
    eTagTypeService = 1,
    eTagTypeMax = 2,
}

#[repr(C)]
pub struct TAG_INFO_NAME_FROM_TAG_IN_PARAMS {
    pub dwPid: u32,
    pub dwTag: u32,
}

impl Default for TAG_INFO_NAME_FROM_TAG_IN_PARAMS {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for TAG_INFO_NAME_FROM_TAG_IN_PARAMS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TAG_INFO_NAME_FROM_TAG_IN_PARAMS {{  }}")
    }
}

#[repr(C)]
pub struct TAG_INFO_NAME_FROM_TAG_OUT_PARAMS {
    pub eTagType: u32,
    pub pszName: PWSTR,
}

impl Default for TAG_INFO_NAME_FROM_TAG_OUT_PARAMS {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for TAG_INFO_NAME_FROM_TAG_OUT_PARAMS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TAG_INFO_NAME_FROM_TAG_OUT_PARAMS {{  }}")
    }
}

#[repr(C)]
pub struct TAG_INFO_NAME_FROM_TAG {
    pub InParams: TAG_INFO_NAME_FROM_TAG_IN_PARAMS,
    pub OutParams: TAG_INFO_NAME_FROM_TAG_OUT_PARAMS,
}

impl Default for TAG_INFO_NAME_FROM_TAG {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for TAG_INFO_NAME_FROM_TAG {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TAG_INFO_NAME_FROM_TAG {{ InParams: {:?}, OutParams: {:?} }}",
            self.InParams, self.OutParams
        )
    }
}

#[repr(C)]
pub struct TAG_INFO_NAMES_REFERENCING_MODULE_IN_PARAMS {
    pub dwPid: u32,
    pub pszModule: PWSTR,
}

impl Default for TAG_INFO_NAMES_REFERENCING_MODULE_IN_PARAMS {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for TAG_INFO_NAMES_REFERENCING_MODULE_IN_PARAMS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TAG_INFO_NAMES_REFERENCING_MODULE_IN_PARAMS {{  }}")
    }
}

#[repr(C)]
pub struct TAG_INFO_NAMES_REFERENCING_MODULE_OUT_PARAMS {
    pub eTagType: u32,
    pub pmszNames: PWSTR,
}

impl Default for TAG_INFO_NAMES_REFERENCING_MODULE_OUT_PARAMS {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for TAG_INFO_NAMES_REFERENCING_MODULE_OUT_PARAMS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TAG_INFO_NAMES_REFERENCING_MODULE_OUT_PARAMS {{  }}")
    }
}

#[repr(C)]
pub struct TAG_INFO_NAMES_REFERENCING_MODULE {
    pub InParams: TAG_INFO_NAMES_REFERENCING_MODULE_IN_PARAMS,
    pub OutParams: TAG_INFO_NAMES_REFERENCING_MODULE_OUT_PARAMS,
}

impl Default for TAG_INFO_NAMES_REFERENCING_MODULE {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for TAG_INFO_NAMES_REFERENCING_MODULE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TAG_INFO_NAMES_REFERENCING_MODULE {{ InParams: {:?}, OutParams: {:?} }}",
            self.InParams, self.OutParams
        )
    }
}

#[repr(C)]
pub struct TAG_INFO_NAME_TAG_MAPPING_IN_PARAMS {
    pub dwPid: u32,
}

impl Default for TAG_INFO_NAME_TAG_MAPPING_IN_PARAMS {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for TAG_INFO_NAME_TAG_MAPPING_IN_PARAMS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TAG_INFO_NAME_TAG_MAPPING_IN_PARAMS {{  }}")
    }
}

#[repr(C)]
pub struct TAG_INFO_NAME_TAG_MAPPING_ELEMENT {
    pub eTagType: u32,
    pub dwTag: u32,
    pub pszName: PWSTR,
    pub pszGroupName: PWSTR,
}

impl Default for TAG_INFO_NAME_TAG_MAPPING_ELEMENT {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for TAG_INFO_NAME_TAG_MAPPING_ELEMENT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TAG_INFO_NAME_TAG_MAPPING_ELEMENT {{  }}")
    }
}

#[repr(C)]
pub struct TAG_INFO_NAME_TAG_MAPPING_OUT_PARAMS {
    pub cElements: u32,
    pub pNameTagMappingElements: *mut TAG_INFO_NAME_TAG_MAPPING_ELEMENT,
}

impl Default for TAG_INFO_NAME_TAG_MAPPING_OUT_PARAMS {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for TAG_INFO_NAME_TAG_MAPPING_OUT_PARAMS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TAG_INFO_NAME_TAG_MAPPING_OUT_PARAMS {{ pNameTagMappingElements: {:?} }}",
            self.pNameTagMappingElements
        )
    }
}

#[repr(C)]
pub struct TAG_INFO_NAME_TAG_MAPPING {
    pub InParams: TAG_INFO_NAME_TAG_MAPPING_IN_PARAMS,
    pub pOutParams: *mut TAG_INFO_NAME_TAG_MAPPING_OUT_PARAMS,
}

impl Default for TAG_INFO_NAME_TAG_MAPPING {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for TAG_INFO_NAME_TAG_MAPPING {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TAG_INFO_NAME_TAG_MAPPING {{ InParams: {:?}, pOutParams: {:?} }}",
            self.InParams, self.pOutParams
        )
    }
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn I_QueryTagInformation(
        MachineName: PCWSTR,
        InfoLevel: TAG_INFO_LEVEL,
        TagInfo: *mut std::ffi::c_void,
    ) -> u32;
}

pub type PQUERY_TAG_INFORMATION = std::option::Option<
    unsafe extern "system" fn(
        MachineName: PCWSTR,
        InfoLevel: TAG_INFO_LEVEL,
        TagInfo: *mut std::ffi::c_void,
    ) -> u32,
>;
