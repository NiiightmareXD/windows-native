use crate::bitfield::{BitfieldUnit, UnionField};

pub const GDI_MAX_HANDLE_COUNT: u32 = 65535;
pub const GDI_HANDLE_INDEX_SHIFT: u32 = 0;
pub const GDI_HANDLE_INDEX_BITS: u32 = 16;
pub const GDI_HANDLE_INDEX_MASK: u32 = 65535;
pub const GDI_HANDLE_TYPE_SHIFT: u32 = 16;
pub const GDI_HANDLE_TYPE_BITS: u32 = 5;
pub const GDI_HANDLE_TYPE_MASK: u32 = 31;
pub const GDI_HANDLE_ALTTYPE_SHIFT: u32 = 21;
pub const GDI_HANDLE_ALTTYPE_BITS: u32 = 2;
pub const GDI_HANDLE_ALTTYPE_MASK: u32 = 3;
pub const GDI_HANDLE_STOCK_SHIFT: u32 = 23;
pub const GDI_HANDLE_STOCK_BITS: u32 = 1;
pub const GDI_HANDLE_STOCK_MASK: u32 = 1;
pub const GDI_HANDLE_UNIQUE_SHIFT: u32 = 24;
pub const GDI_HANDLE_UNIQUE_BITS: u32 = 8;
pub const GDI_HANDLE_UNIQUE_MASK: u32 = 255;
pub const GDI_DEF_TYPE: u32 = 0;
pub const GDI_DC_TYPE: u32 = 1;
pub const GDI_DD_DIRECTDRAW_TYPE: u32 = 2;
pub const GDI_DD_SURFACE_TYPE: u32 = 3;
pub const GDI_RGN_TYPE: u32 = 4;
pub const GDI_SURF_TYPE: u32 = 5;
pub const GDI_CLIENTOBJ_TYPE: u32 = 6;
pub const GDI_PATH_TYPE: u32 = 7;
pub const GDI_PAL_TYPE: u32 = 8;
pub const GDI_ICMLCS_TYPE: u32 = 9;
pub const GDI_LFONT_TYPE: u32 = 10;
pub const GDI_RFONT_TYPE: u32 = 11;
pub const GDI_PFE_TYPE: u32 = 12;
pub const GDI_PFT_TYPE: u32 = 13;
pub const GDI_ICMCXF_TYPE: u32 = 14;
pub const GDI_ICMDLL_TYPE: u32 = 15;
pub const GDI_BRUSH_TYPE: u32 = 16;
pub const GDI_PFF_TYPE: u32 = 17;
pub const GDI_CACHE_TYPE: u32 = 18;
pub const GDI_SPACE_TYPE: u32 = 19;
pub const GDI_DBRUSH_TYPE: u32 = 20;
pub const GDI_META_TYPE: u32 = 21;
pub const GDI_EFSTATE_TYPE: u32 = 22;
pub const GDI_BMFD_TYPE: u32 = 23;
pub const GDI_VTFD_TYPE: u32 = 24;
pub const GDI_TTFD_TYPE: u32 = 25;
pub const GDI_RC_TYPE: u32 = 26;
pub const GDI_TEMP_TYPE: u32 = 27;
pub const GDI_DRVOBJ_TYPE: u32 = 28;
pub const GDI_DCIOBJ_TYPE: u32 = 29;
pub const GDI_SPOOL_TYPE: u32 = 30;
pub const GDI_ALTTYPE_1: u32 = 2097152;
pub const GDI_ALTTYPE_2: u32 = 4194304;
pub const GDI_ALTTYPE_3: u32 = 6291456;
pub const GDI_CLIENT_BITMAP_TYPE: u32 = 327680;
pub const GDI_CLIENT_BRUSH_TYPE: u32 = 1048576;
pub const GDI_CLIENT_CLIENTOBJ_TYPE: u32 = 393216;
pub const GDI_CLIENT_DC_TYPE: u32 = 65536;
pub const GDI_CLIENT_FONT_TYPE: u32 = 655360;
pub const GDI_CLIENT_PALETTE_TYPE: u32 = 524288;
pub const GDI_CLIENT_REGION_TYPE: u32 = 262144;
pub const GDI_CLIENT_ALTDC_TYPE: u32 = 2162688;
pub const GDI_CLIENT_DIBSECTION_TYPE: u32 = 2424832;
pub const GDI_CLIENT_EXTPEN_TYPE: u32 = 5242880;
pub const GDI_CLIENT_METADC16_TYPE: u32 = 6684672;
pub const GDI_CLIENT_METAFILE_TYPE: u32 = 4587520;
pub const GDI_CLIENT_METAFILE16_TYPE: u32 = 2490368;
pub const GDI_CLIENT_PEN_TYPE: u32 = 3145728;
#[repr(C)]
pub struct GDI_HANDLE_ENTRY {
    pub Anonymous1: GDI_HANDLE_ENTRY_1,
    pub Owner: GDI_HANDLE_ENTRY_2,
    pub Unique: u16,
    pub Type: u8,
    pub Flags: u8,
    pub UserPointer: *mut std::ffi::c_void,
}
#[repr(C)]
pub struct GDI_HANDLE_ENTRY_1 {
    pub Object: UnionField<*mut std::ffi::c_void>,
    pub NextFree: UnionField<*mut std::ffi::c_void>,
    pub union_field: u64,
}
impl Default for GDI_HANDLE_ENTRY_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for GDI_HANDLE_ENTRY_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GDI_HANDLE_ENTRY_1 {{ union }}")
    }
}
#[repr(C)]
pub struct GDI_HANDLE_ENTRY_2 {
    pub Anonymous1: UnionField<GDI_HANDLE_ENTRY_2_1>,
    pub Value: UnionField<u32>,
    pub union_field: u32,
}
#[repr(C)]
pub struct GDI_HANDLE_ENTRY_2_1 {
    pub ProcessId: u16,
    _bitfield_align_1: [u16; 0],
    _bitfield_1: BitfieldUnit<[u8; 2]>,
}
impl Default for GDI_HANDLE_ENTRY_2_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for GDI_HANDLE_ENTRY_2_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GDI_HANDLE_ENTRY_2_1 {{ Lock : {:?}, Count : {:?} }}", self.Lock(), self.Count())
    }
}
impl GDI_HANDLE_ENTRY_2_1 {
    #[inline]
    pub fn Lock(&self) -> u16 {
        self._bitfield_1.get(0usize, 1u8) as u16
    }
    #[inline]
    pub fn set_Lock(&mut self, val: u16) {
        self._bitfield_1.set(0usize, 1u8, val as u64)
    }
    #[inline]
    pub fn Count(&self) -> u16 {
        self._bitfield_1.get(1usize, 15u8) as u16
    }
    #[inline]
    pub fn set_Count(&mut self, val: u16) {
        self._bitfield_1.set(1usize, 15u8, val as u64)
    }
    #[inline]
    pub fn new_bitfield_1(Lock: u16, Count: u16) -> BitfieldUnit<[u8; 2]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 2]> = Default::default();
        bitfield_unit.set(0usize, 1u8, Lock as u64);
        bitfield_unit.set(1usize, 15u8, Count as u64);
        bitfield_unit
    }
}
impl Default for GDI_HANDLE_ENTRY_2 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for GDI_HANDLE_ENTRY_2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GDI_HANDLE_ENTRY_2 {{ union }}")
    }
}
impl Default for GDI_HANDLE_ENTRY {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for GDI_HANDLE_ENTRY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GDI_HANDLE_ENTRY {{ Anonymous1: {:?}, Owner: {:?} }}", self.Anonymous1, self.Owner)
    }
}
#[repr(C)]
pub struct GDI_SHARED_MEMORY {
    pub Handles: [GDI_HANDLE_ENTRY; 65535],
}
impl Default for GDI_SHARED_MEMORY {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for GDI_SHARED_MEMORY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GDI_SHARED_MEMORY {{ Handles: {:?} }}", self.Handles)
    }
}
