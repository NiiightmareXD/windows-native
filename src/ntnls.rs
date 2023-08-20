use windows::Win32::Foundation::BOOLEAN;

#[repr(C)]
pub struct CPTABLEINFO {
    pub CodePage: u16,
    pub MaximumCharacterSize: u16,
    pub DefaultChar: u16,
    pub UniDefaultChar: u16,
    pub TransDefaultChar: u16,
    pub TransUniDefaultChar: u16,
    pub DBCSCodePage: u16,
    pub LeadByte: [u8; 12usize],
    pub MultiByteTable: *mut u16,
    pub WideCharTable: *mut std::ffi::c_void,
    pub DBCSRanges: *mut u16,
    pub DBCSOffsets: *mut u16,
}
impl Default for CPTABLEINFO {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for CPTABLEINFO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CPTABLEINFO {{ LeadByte: {:?} }}", self.LeadByte)
    }
}
#[repr(C)]
pub struct NLSTABLEINFO {
    pub OemTableInfo: CPTABLEINFO,
    pub AnsiTableInfo: CPTABLEINFO,
    pub UpperCaseTable: *mut u16,
    pub LowerCaseTable: *mut u16,
}
impl Default for NLSTABLEINFO {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for NLSTABLEINFO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NLSTABLEINFO {{ OemTableInfo: {:?}, AnsiTableInfo: {:?} }}", self.OemTableInfo, self.AnsiTableInfo)
    }
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub static mut NlsAnsiCodePage: u16;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub static mut NlsMbCodePageTag: BOOLEAN;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub static mut NlsMbOemCodePageTag: BOOLEAN;
}
