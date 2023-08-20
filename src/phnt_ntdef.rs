use windows::Win32::Foundation::BOOLEAN;

pub const NT_FACILITY_MASK: u32 = 4095;
pub const NT_FACILITY_SHIFT: u32 = 16;
pub const OBJ_PROTECT_CLOSE: u32 = 1;
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
pub type PENCLAVE_ROUTINE = Option<unsafe extern "system" fn(lpThreadParameter: *mut std::ffi::c_void) -> u32>;
pub type WAITORTIMERCALLBACKFUNC = Option<unsafe extern "system" fn(_: *mut std::ffi::c_void, _: BOOLEAN)>;
