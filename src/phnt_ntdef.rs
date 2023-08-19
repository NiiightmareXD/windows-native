use windows::Win32::Foundation::BOOLEAN;

pub const NT_FACILITY_MASK: u32 = 4095;
pub const NT_FACILITY_SHIFT: u32 = 16;
pub const RTL_BALANCED_NODE_RESERVED_PARENT_MASK: u32 = 3;
pub const OBJ_PROTECT_CLOSE: u32 = 1;
pub const OBJ_INHERIT: u32 = 2;
pub const OBJ_AUDIT_OBJECT_CLOSE: u32 = 4;
pub const OBJ_PERMANENT: u32 = 16;
pub const OBJ_EXCLUSIVE: u32 = 32;
pub const OBJ_CASE_INSENSITIVE: u32 = 64;
pub const OBJ_OPENIF: u32 = 128;
pub const OBJ_OPENLINK: u32 = 256;
pub const OBJ_KERNEL_HANDLE: u32 = 512;
pub const OBJ_FORCE_ACCESS_CHECK: u32 = 1024;
pub const OBJ_IGNORE_IMPERSONATED_DEVICEMAP: u32 = 2048;
pub const OBJ_DONT_REPARSE: u32 = 4096;
pub const OBJ_VALID_ATTRIBUTES: u32 = 8178;
#[repr(C)]
#[repr(align(16))]
pub struct QUAD_PTR {
    pub DoNotUseThisField1: usize,
    pub DoNotUseThisField2: usize,
}
impl Default for QUAD_PTR {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for QUAD_PTR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "QUAD_PTR {{  }}")
    }
}
#[repr(C)]
pub struct KSYSTEM_TIME {
    pub LowPart: u32,
    pub High1Time: i32,
    pub High2Time: i32,
}
impl Default for KSYSTEM_TIME {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for KSYSTEM_TIME {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "KSYSTEM_TIME {{  }}")
    }
}
#[repr(transparent)]
#[derive(Copy, Hash, PartialEq, Eq)]
pub struct PREGHANDLE(pub u64);
impl PREGHANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == 0
    }
}
impl std::default::Default for PREGHANDLE {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::clone::Clone for PREGHANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl std::fmt::Debug for PREGHANDLE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("HANDLE").field(&self.0).finish()
    }
}
impl windows::core::TypeKind for PREGHANDLE {
    type TypeKind = windows::core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Hash, PartialEq, Eq)]
pub struct TRACEHANDLE(pub u64);
impl TRACEHANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == 0
    }
}
impl std::default::Default for TRACEHANDLE {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::clone::Clone for TRACEHANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl std::fmt::Debug for TRACEHANDLE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("HANDLE").field(&self.0).finish()
    }
}
impl windows::core::TypeKind for TRACEHANDLE {
    type TypeKind = windows::core::CopyType;
}
pub type PENCLAVE_ROUTINE =
    Option<unsafe extern "system" fn(lpThreadParameter: *mut std::ffi::c_void) -> u32>;
pub type WAITORTIMERCALLBACKFUNC =
    Option<unsafe extern "system" fn(_: *mut std::ffi::c_void, _: BOOLEAN)>;
