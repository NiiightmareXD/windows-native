use windows::{
    Wdk::Foundation::OBJECT_ATTRIBUTES,
    Win32::{
        Foundation::{BOOLEAN, HANDLE, NTSTATUS, UNICODE_STRING},
        System::IO::{IO_STATUS_BLOCK, PIO_APC_ROUTINE},
    },
};

use crate::bitfield::{BitfieldUnit, UnionField};

pub const REG_INIT_BOOT_SM: u32 = 0;
pub const REG_INIT_BOOT_SETUP: u32 = 1;
pub const REG_INIT_BOOT_ACCEPTED_BASE: u32 = 2;
pub const REG_INIT_BOOT_ACCEPTED_MAX: u32 = 1001;
pub const REG_MAX_KEY_VALUE_NAME_LENGTH: u32 = 32767;
pub const REG_MAX_KEY_NAME_LENGTH: u32 = 512;
pub const REG_FLAG_VOLATILE: u32 = 1;
pub const REG_FLAG_LINK: u32 = 2;
pub const REG_KEY_DONT_VIRTUALIZE: u32 = 2;
pub const REG_KEY_DONT_SILENT_FAIL: u32 = 4;
pub const REG_KEY_RECURSE_FLAG: u32 = 8;
pub const CM_EXTENDED_PARAMETER_TYPE_BITS: u32 = 8;
pub const VR_DEVICE_NAME: &[u8; 19usize] = b"\\Device\\VRegDriver\0";
pub const VR_FLAG_INHERIT_TRUST_CLASS: u32 = 1;
pub const VR_FLAG_WRITE_THROUGH_HIVE: u32 = 2;
pub const VR_FLAG_LOCAL_MACHINE_TRUST_CLASS: u32 = 4;
pub const VR_KEY_COMROOT: u32 = 0;
pub const VR_KEY_MACHINE_SOFTWARE: u32 = 1;
pub const VR_KEY_CONTROL_SET: u32 = 2;
pub const IOCTL_VR_INITIALIZE_JOB_FOR_VREG: u32 = 2228228;
pub const IOCTL_VR_LOAD_DIFFERENCING_HIVE: u32 = 2228232;
pub const IOCTL_VR_CREATE_NAMESPACE_NODE: u32 = 2228236;
pub const IOCTL_VR_MODIFY_FLAGS: u32 = 2228240;
pub const IOCTL_VR_CREATE_MULTIPLE_NAMESPACE_NODES: u32 = 2228244;
pub const IOCTL_VR_UNLOAD_DYNAMICALLY_LOADED_HIVES: u32 = 2228248;
pub const IOCTL_VR_GET_VIRTUAL_ROOT_KEY: u32 = 2228252;
pub const IOCTL_VR_LOAD_DIFFERENCING_HIVE_FOR_HOST: u32 = 2228256;
pub const IOCTL_VR_UNLOAD_DIFFERENCING_HIVE_FOR_HOST: u32 = 2228260;
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum KEY_INFORMATION_CLASS {
    KeyBasicInformation = 0,
    KeyNodeInformation = 1,
    KeyFullInformation = 2,
    KeyNameInformation = 3,
    KeyCachedInformation = 4,
    KeyFlagsInformation = 5,
    KeyVirtualizationInformation = 6,
    KeyHandleTagsInformation = 7,
    KeyTrustInformation = 8,
    KeyLayerInformation = 9,
    MaxKeyInfoClass = 10,
}
#[repr(C)]
pub struct KEY_BASIC_INFORMATION {
    pub LastWriteTime: i64,
    pub TitleIndex: u32,
    pub NameLength: u32,
    pub Name: [u16; 1usize],
}
impl Default for KEY_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for KEY_BASIC_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "KEY_BASIC_INFORMATION {{ Name: {:?} }}", self.Name)
    }
}
#[repr(C)]
pub struct KEY_NODE_INFORMATION {
    pub LastWriteTime: i64,
    pub TitleIndex: u32,
    pub ClassOffset: u32,
    pub ClassLength: u32,
    pub NameLength: u32,
    pub Name: [u16; 1usize],
}
impl Default for KEY_NODE_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for KEY_NODE_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "KEY_NODE_INFORMATION {{ Name: {:?} }}", self.Name)
    }
}
#[repr(C)]
pub struct KEY_FULL_INFORMATION {
    pub LastWriteTime: i64,
    pub TitleIndex: u32,
    pub ClassOffset: u32,
    pub ClassLength: u32,
    pub SubKeys: u32,
    pub MaxNameLength: u32,
    pub MaxClassLength: u32,
    pub Values: u32,
    pub MaxValueNameLength: u32,
    pub MaxValueDataLength: u32,
    pub Class: [u16; 1usize],
}
impl Default for KEY_FULL_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for KEY_FULL_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "KEY_FULL_INFORMATION {{ Class: {:?} }}", self.Class)
    }
}
#[repr(C)]
pub struct KEY_NAME_INFORMATION {
    pub NameLength: u32,
    pub Name: [u16; 1usize],
}
impl Default for KEY_NAME_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for KEY_NAME_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "KEY_NAME_INFORMATION {{ Name: {:?} }}", self.Name)
    }
}
#[repr(C)]
pub struct KEY_CACHED_INFORMATION {
    pub LastWriteTime: i64,
    pub TitleIndex: u32,
    pub SubKeys: u32,
    pub MaxNameLength: u32,
    pub Values: u32,
    pub MaxValueNameLength: u32,
    pub MaxValueDataLength: u32,
    pub NameLength: u32,
    pub Name: [u16; 1usize],
}
impl Default for KEY_CACHED_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for KEY_CACHED_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "KEY_CACHED_INFORMATION {{ Name: {:?} }}", self.Name)
    }
}
#[repr(C)]
pub struct KEY_FLAGS_INFORMATION {
    pub Wow64Flags: u32,
    pub KeyFlags: u32,
    pub ControlFlags: u32,
}
impl Default for KEY_FLAGS_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for KEY_FLAGS_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "KEY_FLAGS_INFORMATION {{  }}")
    }
}
#[repr(C)]
#[repr(align(4))]
pub struct KEY_VIRTUALIZATION_INFORMATION {
    _bitfield_align_1: [u32; 0],
    _bitfield_1: BitfieldUnit<[u8; 4usize]>,
}
impl Default for KEY_VIRTUALIZATION_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for KEY_VIRTUALIZATION_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "KEY_VIRTUALIZATION_INFORMATION {{ VirtualizationCandidate : {:?}, VirtualizationEnabled : {:?}, VirtualTarget : {:?}, VirtualStore : {:?}, VirtualSource : {:?}, Reserved : {:?} }}",
            self.VirtualizationCandidate(),
            self.VirtualizationEnabled(),
            self.VirtualTarget(),
            self.VirtualStore(),
            self.VirtualSource(),
            self.Reserved()
        )
    }
}
impl KEY_VIRTUALIZATION_INFORMATION {
    #[inline]
    pub fn VirtualizationCandidate(&self) -> u32 {
        self._bitfield_1.get(0usize, 1u8) as u32
    }
    #[inline]
    pub fn set_VirtualizationCandidate(&mut self, val: u32) {
        self._bitfield_1.set(0usize, 1u8, val as u64)
    }
    #[inline]
    pub fn VirtualizationEnabled(&self) -> u32 {
        self._bitfield_1.get(1usize, 1u8) as u32
    }
    #[inline]
    pub fn set_VirtualizationEnabled(&mut self, val: u32) {
        self._bitfield_1.set(1usize, 1u8, val as u64)
    }
    #[inline]
    pub fn VirtualTarget(&self) -> u32 {
        self._bitfield_1.get(2usize, 1u8) as u32
    }
    #[inline]
    pub fn set_VirtualTarget(&mut self, val: u32) {
        self._bitfield_1.set(2usize, 1u8, val as u64)
    }
    #[inline]
    pub fn VirtualStore(&self) -> u32 {
        self._bitfield_1.get(3usize, 1u8) as u32
    }
    #[inline]
    pub fn set_VirtualStore(&mut self, val: u32) {
        self._bitfield_1.set(3usize, 1u8, val as u64)
    }
    #[inline]
    pub fn VirtualSource(&self) -> u32 {
        self._bitfield_1.get(4usize, 1u8) as u32
    }
    #[inline]
    pub fn set_VirtualSource(&mut self, val: u32) {
        self._bitfield_1.set(4usize, 1u8, val as u64)
    }
    #[inline]
    pub fn Reserved(&self) -> u32 {
        self._bitfield_1.get(5usize, 27u8) as u32
    }
    #[inline]
    pub fn set_Reserved(&mut self, val: u32) {
        self._bitfield_1.set(5usize, 27u8, val as u64)
    }
    #[inline]
    pub fn new_bitfield_1(
        VirtualizationCandidate: u32,
        VirtualizationEnabled: u32,
        VirtualTarget: u32,
        VirtualStore: u32,
        VirtualSource: u32,
        Reserved: u32,
    ) -> BitfieldUnit<[u8; 4usize]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 4usize]> = Default::default();
        bitfield_unit.set(0usize, 1u8, VirtualizationCandidate as u64);
        bitfield_unit.set(1usize, 1u8, VirtualizationEnabled as u64);
        bitfield_unit.set(2usize, 1u8, VirtualTarget as u64);
        bitfield_unit.set(3usize, 1u8, VirtualStore as u64);
        bitfield_unit.set(4usize, 1u8, VirtualSource as u64);
        bitfield_unit.set(5usize, 27u8, Reserved as u64);
        bitfield_unit
    }
}
#[repr(C)]
#[repr(align(4))]
pub struct KEY_TRUST_INFORMATION {
    _bitfield_align_1: [u32; 0],
    _bitfield_1: BitfieldUnit<[u8; 4usize]>,
}
impl Default for KEY_TRUST_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for KEY_TRUST_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "KEY_TRUST_INFORMATION {{ TrustedKey : {:?}, Reserved : {:?} }}",
            self.TrustedKey(),
            self.Reserved()
        )
    }
}
impl KEY_TRUST_INFORMATION {
    #[inline]
    pub fn TrustedKey(&self) -> u32 {
        self._bitfield_1.get(0usize, 1u8) as u32
    }
    #[inline]
    pub fn set_TrustedKey(&mut self, val: u32) {
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
    pub fn new_bitfield_1(TrustedKey: u32, Reserved: u32) -> BitfieldUnit<[u8; 4usize]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 4usize]> = Default::default();
        bitfield_unit.set(0usize, 1u8, TrustedKey as u64);
        bitfield_unit.set(1usize, 31u8, Reserved as u64);
        bitfield_unit
    }
}
#[repr(C)]
#[repr(align(4))]
pub struct KEY_LAYER_INFORMATION {
    _bitfield_align_1: [u32; 0],
    _bitfield_1: BitfieldUnit<[u8; 4usize]>,
}
impl Default for KEY_LAYER_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for KEY_LAYER_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "KEY_LAYER_INFORMATION {{ IsTombstone : {:?}, IsSupersedeLocal : {:?}, IsSupersedeTree : {:?}, ClassIsInherited : {:?}, Reserved : {:?} }}",
            self.IsTombstone(),
            self.IsSupersedeLocal(),
            self.IsSupersedeTree(),
            self.ClassIsInherited(),
            self.Reserved()
        )
    }
}
impl KEY_LAYER_INFORMATION {
    #[inline]
    pub fn IsTombstone(&self) -> u32 {
        self._bitfield_1.get(0usize, 1u8) as u32
    }
    #[inline]
    pub fn set_IsTombstone(&mut self, val: u32) {
        self._bitfield_1.set(0usize, 1u8, val as u64)
    }
    #[inline]
    pub fn IsSupersedeLocal(&self) -> u32 {
        self._bitfield_1.get(1usize, 1u8) as u32
    }
    #[inline]
    pub fn set_IsSupersedeLocal(&mut self, val: u32) {
        self._bitfield_1.set(1usize, 1u8, val as u64)
    }
    #[inline]
    pub fn IsSupersedeTree(&self) -> u32 {
        self._bitfield_1.get(2usize, 1u8) as u32
    }
    #[inline]
    pub fn set_IsSupersedeTree(&mut self, val: u32) {
        self._bitfield_1.set(2usize, 1u8, val as u64)
    }
    #[inline]
    pub fn ClassIsInherited(&self) -> u32 {
        self._bitfield_1.get(3usize, 1u8) as u32
    }
    #[inline]
    pub fn set_ClassIsInherited(&mut self, val: u32) {
        self._bitfield_1.set(3usize, 1u8, val as u64)
    }
    #[inline]
    pub fn Reserved(&self) -> u32 {
        self._bitfield_1.get(4usize, 28u8) as u32
    }
    #[inline]
    pub fn set_Reserved(&mut self, val: u32) {
        self._bitfield_1.set(4usize, 28u8, val as u64)
    }
    #[inline]
    pub fn new_bitfield_1(
        IsTombstone: u32,
        IsSupersedeLocal: u32,
        IsSupersedeTree: u32,
        ClassIsInherited: u32,
        Reserved: u32,
    ) -> BitfieldUnit<[u8; 4usize]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 4usize]> = Default::default();
        bitfield_unit.set(0usize, 1u8, IsTombstone as u64);
        bitfield_unit.set(1usize, 1u8, IsSupersedeLocal as u64);
        bitfield_unit.set(2usize, 1u8, IsSupersedeTree as u64);
        bitfield_unit.set(3usize, 1u8, ClassIsInherited as u64);
        bitfield_unit.set(4usize, 28u8, Reserved as u64);
        bitfield_unit
    }
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum KEY_SET_INFORMATION_CLASS {
    KeyWriteTimeInformation = 0,
    KeyWow64FlagsInformation = 1,
    KeyControlFlagsInformation = 2,
    KeySetVirtualizationInformation = 3,
    KeySetDebugInformation = 4,
    KeySetHandleTagsInformation = 5,
    KeySetLayerInformation = 6,
    MaxKeySetInfoClass = 7,
}
#[repr(C)]
pub struct KEY_WRITE_TIME_INFORMATION {
    pub LastWriteTime: i64,
}
impl Default for KEY_WRITE_TIME_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for KEY_WRITE_TIME_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "KEY_WRITE_TIME_INFORMATION {{  }}")
    }
}
#[repr(C)]
pub struct KEY_WOW64_FLAGS_INFORMATION {
    pub UserFlags: u32,
}
impl Default for KEY_WOW64_FLAGS_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for KEY_WOW64_FLAGS_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "KEY_WOW64_FLAGS_INFORMATION {{  }}")
    }
}
#[repr(C)]
pub struct KEY_HANDLE_TAGS_INFORMATION {
    pub HandleTags: u32,
}
impl Default for KEY_HANDLE_TAGS_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for KEY_HANDLE_TAGS_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "KEY_HANDLE_TAGS_INFORMATION {{  }}")
    }
}
#[repr(C)]
#[repr(align(4))]
pub struct KEY_SET_LAYER_INFORMATION {
    _bitfield_align_1: [u32; 0],
    _bitfield_1: BitfieldUnit<[u8; 4usize]>,
}
impl Default for KEY_SET_LAYER_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for KEY_SET_LAYER_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "KEY_SET_LAYER_INFORMATION {{ IsTombstone : {:?}, IsSupersedeLocal : {:?}, IsSupersedeTree : {:?}, ClassIsInherited : {:?}, Reserved : {:?} }}",
            self.IsTombstone(),
            self.IsSupersedeLocal(),
            self.IsSupersedeTree(),
            self.ClassIsInherited(),
            self.Reserved()
        )
    }
}
impl KEY_SET_LAYER_INFORMATION {
    #[inline]
    pub fn IsTombstone(&self) -> u32 {
        self._bitfield_1.get(0usize, 1u8) as u32
    }
    #[inline]
    pub fn set_IsTombstone(&mut self, val: u32) {
        self._bitfield_1.set(0usize, 1u8, val as u64)
    }
    #[inline]
    pub fn IsSupersedeLocal(&self) -> u32 {
        self._bitfield_1.get(1usize, 1u8) as u32
    }
    #[inline]
    pub fn set_IsSupersedeLocal(&mut self, val: u32) {
        self._bitfield_1.set(1usize, 1u8, val as u64)
    }
    #[inline]
    pub fn IsSupersedeTree(&self) -> u32 {
        self._bitfield_1.get(2usize, 1u8) as u32
    }
    #[inline]
    pub fn set_IsSupersedeTree(&mut self, val: u32) {
        self._bitfield_1.set(2usize, 1u8, val as u64)
    }
    #[inline]
    pub fn ClassIsInherited(&self) -> u32 {
        self._bitfield_1.get(3usize, 1u8) as u32
    }
    #[inline]
    pub fn set_ClassIsInherited(&mut self, val: u32) {
        self._bitfield_1.set(3usize, 1u8, val as u64)
    }
    #[inline]
    pub fn Reserved(&self) -> u32 {
        self._bitfield_1.get(4usize, 28u8) as u32
    }
    #[inline]
    pub fn set_Reserved(&mut self, val: u32) {
        self._bitfield_1.set(4usize, 28u8, val as u64)
    }
    #[inline]
    pub fn new_bitfield_1(
        IsTombstone: u32,
        IsSupersedeLocal: u32,
        IsSupersedeTree: u32,
        ClassIsInherited: u32,
        Reserved: u32,
    ) -> BitfieldUnit<[u8; 4usize]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 4usize]> = Default::default();
        bitfield_unit.set(0usize, 1u8, IsTombstone as u64);
        bitfield_unit.set(1usize, 1u8, IsSupersedeLocal as u64);
        bitfield_unit.set(2usize, 1u8, IsSupersedeTree as u64);
        bitfield_unit.set(3usize, 1u8, ClassIsInherited as u64);
        bitfield_unit.set(4usize, 28u8, Reserved as u64);
        bitfield_unit
    }
}
#[repr(C)]
pub struct KEY_CONTROL_FLAGS_INFORMATION {
    pub ControlFlags: u32,
}
impl Default for KEY_CONTROL_FLAGS_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for KEY_CONTROL_FLAGS_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "KEY_CONTROL_FLAGS_INFORMATION {{  }}")
    }
}
#[repr(C)]
#[repr(align(4))]
pub struct KEY_SET_VIRTUALIZATION_INFORMATION {
    _bitfield_align_1: [u32; 0],
    _bitfield_1: BitfieldUnit<[u8; 4usize]>,
}
impl Default for KEY_SET_VIRTUALIZATION_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for KEY_SET_VIRTUALIZATION_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "KEY_SET_VIRTUALIZATION_INFORMATION {{ VirtualTarget : {:?}, VirtualStore : {:?}, VirtualSource : {:?}, Reserved : {:?} }}",
            self.VirtualTarget(),
            self.VirtualStore(),
            self.VirtualSource(),
            self.Reserved()
        )
    }
}
impl KEY_SET_VIRTUALIZATION_INFORMATION {
    #[inline]
    pub fn VirtualTarget(&self) -> u32 {
        self._bitfield_1.get(0usize, 1u8) as u32
    }
    #[inline]
    pub fn set_VirtualTarget(&mut self, val: u32) {
        self._bitfield_1.set(0usize, 1u8, val as u64)
    }
    #[inline]
    pub fn VirtualStore(&self) -> u32 {
        self._bitfield_1.get(1usize, 1u8) as u32
    }
    #[inline]
    pub fn set_VirtualStore(&mut self, val: u32) {
        self._bitfield_1.set(1usize, 1u8, val as u64)
    }
    #[inline]
    pub fn VirtualSource(&self) -> u32 {
        self._bitfield_1.get(2usize, 1u8) as u32
    }
    #[inline]
    pub fn set_VirtualSource(&mut self, val: u32) {
        self._bitfield_1.set(2usize, 1u8, val as u64)
    }
    #[inline]
    pub fn Reserved(&self) -> u32 {
        self._bitfield_1.get(3usize, 29u8) as u32
    }
    #[inline]
    pub fn set_Reserved(&mut self, val: u32) {
        self._bitfield_1.set(3usize, 29u8, val as u64)
    }
    #[inline]
    pub fn new_bitfield_1(
        VirtualTarget: u32,
        VirtualStore: u32,
        VirtualSource: u32,
        Reserved: u32,
    ) -> BitfieldUnit<[u8; 4usize]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 4usize]> = Default::default();
        bitfield_unit.set(0usize, 1u8, VirtualTarget as u64);
        bitfield_unit.set(1usize, 1u8, VirtualStore as u64);
        bitfield_unit.set(2usize, 1u8, VirtualSource as u64);
        bitfield_unit.set(3usize, 29u8, Reserved as u64);
        bitfield_unit
    }
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum KEY_VALUE_INFORMATION_CLASS {
    KeyValueBasicInformation = 0,
    KeyValueFullInformation = 1,
    KeyValuePartialInformation = 2,
    KeyValueFullInformationAlign64 = 3,
    KeyValuePartialInformationAlign64 = 4,
    KeyValueLayerInformation = 5,
    MaxKeyValueInfoClass = 6,
}
#[repr(C)]
pub struct KEY_VALUE_BASIC_INFORMATION {
    pub TitleIndex: u32,
    pub Type: u32,
    pub NameLength: u32,
    pub Name: [u16; 1usize],
}
impl Default for KEY_VALUE_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for KEY_VALUE_BASIC_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "KEY_VALUE_BASIC_INFORMATION {{ Name: {:?} }}", self.Name)
    }
}
#[repr(C)]
pub struct KEY_VALUE_FULL_INFORMATION {
    pub TitleIndex: u32,
    pub Type: u32,
    pub DataOffset: u32,
    pub DataLength: u32,
    pub NameLength: u32,
    pub Name: [u16; 1usize],
}
impl Default for KEY_VALUE_FULL_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for KEY_VALUE_FULL_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "KEY_VALUE_FULL_INFORMATION {{ Name: {:?} }}", self.Name)
    }
}
#[repr(C)]
pub struct KEY_VALUE_PARTIAL_INFORMATION {
    pub TitleIndex: u32,
    pub Type: u32,
    pub DataLength: u32,
    pub Data: [u8; 1usize],
}
impl Default for KEY_VALUE_PARTIAL_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for KEY_VALUE_PARTIAL_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "KEY_VALUE_PARTIAL_INFORMATION {{ Data: {:?} }}",
            self.Data
        )
    }
}
#[repr(C)]
pub struct KEY_VALUE_PARTIAL_INFORMATION_ALIGN64 {
    pub Type: u32,
    pub DataLength: u32,
    pub Data: [u8; 1usize],
}
impl Default for KEY_VALUE_PARTIAL_INFORMATION_ALIGN64 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for KEY_VALUE_PARTIAL_INFORMATION_ALIGN64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "KEY_VALUE_PARTIAL_INFORMATION_ALIGN64 {{ Data: {:?} }}",
            self.Data
        )
    }
}
#[repr(C)]
#[repr(align(4))]
pub struct KEY_VALUE_LAYER_INFORMATION {
    _bitfield_align_1: [u32; 0],
    _bitfield_1: BitfieldUnit<[u8; 4usize]>,
}
impl Default for KEY_VALUE_LAYER_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for KEY_VALUE_LAYER_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "KEY_VALUE_LAYER_INFORMATION {{ IsTombstone : {:?}, Reserved : {:?} }}",
            self.IsTombstone(),
            self.Reserved()
        )
    }
}
impl KEY_VALUE_LAYER_INFORMATION {
    #[inline]
    pub fn IsTombstone(&self) -> u32 {
        self._bitfield_1.get(0usize, 1u8) as u32
    }
    #[inline]
    pub fn set_IsTombstone(&mut self, val: u32) {
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
    pub fn new_bitfield_1(IsTombstone: u32, Reserved: u32) -> BitfieldUnit<[u8; 4usize]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 4usize]> = Default::default();
        bitfield_unit.set(0usize, 1u8, IsTombstone as u64);
        bitfield_unit.set(1usize, 31u8, Reserved as u64);
        bitfield_unit
    }
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum CM_EXTENDED_PARAMETER_TYPE {
    CmExtendedParameterInvalidType = 0,
    CmExtendedParameterTrustClassKey = 1,
    CmExtendedParameterEvent = 2,
    CmExtendedParameterFileAccessToken = 3,
    CmExtendedParameterMax = 4,
}
#[repr(C)]
pub struct CM_EXTENDED_PARAMETER {
    pub Anonymous1: CM_EXTENDED_PARAMETER_1,
    pub Anonymous2: CM_EXTENDED_PARAMETER_2,
}
#[repr(C)]
#[repr(align(8))]
pub struct CM_EXTENDED_PARAMETER_1 {
    _bitfield_align_1: [u64; 0],
    _bitfield_1: BitfieldUnit<[u8; 8usize]>,
}
impl Default for CM_EXTENDED_PARAMETER_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for CM_EXTENDED_PARAMETER_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CM_EXTENDED_PARAMETER_1 {{ Type : {:?}, Reserved : {:?} }}",
            self.Type(),
            self.Reserved()
        )
    }
}
impl CM_EXTENDED_PARAMETER_1 {
    #[inline]
    pub fn Type(&self) -> u64 {
        self._bitfield_1.get(0usize, 8u8)
    }
    #[inline]
    pub fn set_Type(&mut self, val: u64) {
        self._bitfield_1.set(0usize, 8u8, val)
    }
    #[inline]
    pub fn Reserved(&self) -> u64 {
        self._bitfield_1.get(8usize, 56u8)
    }
    #[inline]
    pub fn set_Reserved(&mut self, val: u64) {
        self._bitfield_1.set(8usize, 56u8, val)
    }
    #[inline]
    pub fn new_bitfield_1(Type: u64, Reserved: u64) -> BitfieldUnit<[u8; 8usize]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 8usize]> = Default::default();
        bitfield_unit.set(0usize, 8u8, Type);
        bitfield_unit.set(8usize, 56u8, Reserved);
        bitfield_unit
    }
}
#[repr(C)]
pub struct CM_EXTENDED_PARAMETER_2 {
    pub ULong64: UnionField<u64>,
    pub Pointer: UnionField<*mut std::ffi::c_void>,
    pub Size: UnionField<usize>,
    pub Handle: UnionField<HANDLE>,
    pub ULong: UnionField<u32>,
    pub AccessMask: UnionField<u32>,
    pub union_field: u64,
}
impl Default for CM_EXTENDED_PARAMETER_2 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for CM_EXTENDED_PARAMETER_2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CM_EXTENDED_PARAMETER_2 {{ union }}")
    }
}
impl Default for CM_EXTENDED_PARAMETER {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for CM_EXTENDED_PARAMETER {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CM_EXTENDED_PARAMETER {{ Anonymous1: {:?}, Anonymous2: {:?} }}",
            self.Anonymous1, self.Anonymous2
        )
    }
}
#[repr(C)]
pub struct KEY_VALUE_ENTRY {
    pub ValueName: *mut UNICODE_STRING,
    pub DataLength: u32,
    pub DataOffset: u32,
    pub Type: u32,
}
impl Default for KEY_VALUE_ENTRY {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for KEY_VALUE_ENTRY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "KEY_VALUE_ENTRY {{  }}")
    }
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum REG_ACTION {
    KeyAdded = 0,
    KeyRemoved = 1,
    KeyModified = 2,
}
#[repr(C)]
pub struct REG_NOTIFY_INFORMATION {
    pub NextEntryOffset: u32,
    pub Action: REG_ACTION,
    pub KeyLength: u32,
    pub Key: [u16; 1usize],
}
impl Default for REG_NOTIFY_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for REG_NOTIFY_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "REG_NOTIFY_INFORMATION {{ Action: {:?}, Key: {:?} }}",
            self.Action, self.Key
        )
    }
}
#[repr(C)]
pub struct KEY_PID_ARRAY {
    pub ProcessId: HANDLE,
    pub KeyName: UNICODE_STRING,
}
impl Default for KEY_PID_ARRAY {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for KEY_PID_ARRAY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "KEY_PID_ARRAY {{  }}")
    }
}
#[repr(C)]
pub struct KEY_OPEN_SUBKEYS_INFORMATION {
    pub Count: u32,
    pub KeyArray: [KEY_PID_ARRAY; 1usize],
}
impl Default for KEY_OPEN_SUBKEYS_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for KEY_OPEN_SUBKEYS_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "KEY_OPEN_SUBKEYS_INFORMATION {{ KeyArray: {:?} }}",
            self.KeyArray
        )
    }
}
#[repr(C)]
pub struct VR_INITIALIZE_JOB_FOR_VREG {
    pub Job: HANDLE,
}
impl Default for VR_INITIALIZE_JOB_FOR_VREG {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for VR_INITIALIZE_JOB_FOR_VREG {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VR_INITIALIZE_JOB_FOR_VREG {{  }}")
    }
}
#[repr(C)]
pub struct VR_LOAD_DIFFERENCING_HIVE {
    pub Job: HANDLE,
    pub NextLayerIsHost: u32,
    pub Flags: u32,
    pub LoadFlags: u32,
    pub KeyPathLength: u16,
    pub HivePathLength: u16,
    pub NextLayerKeyPathLength: u16,
    pub FileAccessToken: HANDLE,
    pub Strings: [u16; 1usize],
}
impl Default for VR_LOAD_DIFFERENCING_HIVE {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for VR_LOAD_DIFFERENCING_HIVE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "VR_LOAD_DIFFERENCING_HIVE {{ Strings: {:?} }}",
            self.Strings
        )
    }
}
#[repr(C)]
pub struct VR_CREATE_NAMESPACE_NODE {
    pub Job: HANDLE,
    pub ContainerPathLength: u16,
    pub HostPathLength: u16,
    pub Flags: u32,
    pub AccessMask: u32,
    pub Strings: [u16; 1usize],
}
impl Default for VR_CREATE_NAMESPACE_NODE {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for VR_CREATE_NAMESPACE_NODE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "VR_CREATE_NAMESPACE_NODE {{ Strings: {:?} }}",
            self.Strings
        )
    }
}
#[repr(C)]
pub struct VR_MODIFY_FLAGS {
    pub Job: HANDLE,
    pub AddFlags: u32,
    pub RemoveFlags: u32,
}
impl Default for VR_MODIFY_FLAGS {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for VR_MODIFY_FLAGS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VR_MODIFY_FLAGS {{  }}")
    }
}
#[repr(C)]
pub struct NAMESPACE_NODE_DATA {
    pub AccessMask: u32,
    pub ContainerPathLength: u16,
    pub HostPathLength: u16,
    pub Flags: u32,
    pub Strings: [u16; 1usize],
}
impl Default for NAMESPACE_NODE_DATA {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for NAMESPACE_NODE_DATA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NAMESPACE_NODE_DATA {{ Strings: {:?} }}", self.Strings)
    }
}
#[repr(C)]
pub struct VR_CREATE_MULTIPLE_NAMESPACE_NODES {
    pub Job: HANDLE,
    pub NumNewKeys: u32,
    pub Keys: [NAMESPACE_NODE_DATA; 1usize],
}
impl Default for VR_CREATE_MULTIPLE_NAMESPACE_NODES {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for VR_CREATE_MULTIPLE_NAMESPACE_NODES {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "VR_CREATE_MULTIPLE_NAMESPACE_NODES {{ Keys: {:?} }}",
            self.Keys
        )
    }
}
#[repr(C)]
pub struct VR_UNLOAD_DYNAMICALLY_LOADED_HIVES {
    pub Job: HANDLE,
}
impl Default for VR_UNLOAD_DYNAMICALLY_LOADED_HIVES {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for VR_UNLOAD_DYNAMICALLY_LOADED_HIVES {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VR_UNLOAD_DYNAMICALLY_LOADED_HIVES {{  }}")
    }
}
#[repr(C)]
pub struct VR_GET_VIRTUAL_ROOT {
    pub Job: HANDLE,
    pub Index: u32,
}
impl Default for VR_GET_VIRTUAL_ROOT {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for VR_GET_VIRTUAL_ROOT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VR_GET_VIRTUAL_ROOT {{  }}")
    }
}
#[repr(C)]
pub struct VR_GET_VIRTUAL_ROOT_RESULT {
    pub Key: HANDLE,
}
impl Default for VR_GET_VIRTUAL_ROOT_RESULT {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for VR_GET_VIRTUAL_ROOT_RESULT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VR_GET_VIRTUAL_ROOT_RESULT {{  }}")
    }
}
#[repr(C)]
pub struct VR_LOAD_DIFFERENCING_HIVE_FOR_HOST {
    pub LoadFlags: u32,
    pub Flags: u32,
    pub KeyPathLength: u16,
    pub HivePathLength: u16,
    pub NextLayerKeyPathLength: u16,
    pub FileAccessToken: HANDLE,
    pub Strings: [u16; 1usize],
}
impl Default for VR_LOAD_DIFFERENCING_HIVE_FOR_HOST {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for VR_LOAD_DIFFERENCING_HIVE_FOR_HOST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "VR_LOAD_DIFFERENCING_HIVE_FOR_HOST {{ Strings: {:?} }}",
            self.Strings
        )
    }
}
#[repr(C)]
pub struct VR_UNLOAD_DIFFERENCING_HIVE_FOR_HOST {
    pub Reserved: u32,
    pub TargetKeyPathLength: u16,
    pub TargetKeyPath: [u16; 1usize],
}
impl Default for VR_UNLOAD_DIFFERENCING_HIVE_FOR_HOST {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for VR_UNLOAD_DIFFERENCING_HIVE_FOR_HOST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "VR_UNLOAD_DIFFERENCING_HIVE_FOR_HOST {{ TargetKeyPath: {:?} }}",
            self.TargetKeyPath
        )
    }
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtCreateKey(
        KeyHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        TitleIndex: u32,
        Class: *mut UNICODE_STRING,
        CreateOptions: u32,
        Disposition: *mut u32,
    ) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtCreateKeyTransacted(
        KeyHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        TitleIndex: u32,
        Class: *mut UNICODE_STRING,
        CreateOptions: u32,
        TransactionHandle: HANDLE,
        Disposition: *mut u32,
    ) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtOpenKey(
        KeyHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtOpenKeyTransacted(
        KeyHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        TransactionHandle: HANDLE,
    ) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtOpenKeyEx(
        KeyHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        OpenOptions: u32,
    ) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtOpenKeyTransactedEx(
        KeyHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        OpenOptions: u32,
        TransactionHandle: HANDLE,
    ) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtDeleteKey(KeyHandle: HANDLE) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtRenameKey(KeyHandle: HANDLE, NewName: *mut UNICODE_STRING) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtDeleteValueKey(KeyHandle: HANDLE, ValueName: *mut UNICODE_STRING) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtQueryKey(
        KeyHandle: HANDLE,
        KeyInformationClass: KEY_INFORMATION_CLASS,
        KeyInformation: *mut std::ffi::c_void,
        Length: u32,
        ResultLength: *mut u32,
    ) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtSetInformationKey(
        KeyHandle: HANDLE,
        KeySetInformationClass: KEY_SET_INFORMATION_CLASS,
        KeySetInformation: *mut std::ffi::c_void,
        KeySetInformationLength: u32,
    ) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtQueryValueKey(
        KeyHandle: HANDLE,
        ValueName: *mut UNICODE_STRING,
        KeyValueInformationClass: KEY_VALUE_INFORMATION_CLASS,
        KeyValueInformation: *mut std::ffi::c_void,
        Length: u32,
        ResultLength: *mut u32,
    ) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtSetValueKey(
        KeyHandle: HANDLE,
        ValueName: *mut UNICODE_STRING,
        TitleIndex: u32,
        Type: u32,
        Data: *mut std::ffi::c_void,
        DataSize: u32,
    ) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtQueryMultipleValueKey(
        KeyHandle: HANDLE,
        ValueEntries: *mut KEY_VALUE_ENTRY,
        EntryCount: u32,
        ValueBuffer: *mut std::ffi::c_void,
        BufferLength: *mut u32,
        RequiredBufferLength: *mut u32,
    ) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtEnumerateKey(
        KeyHandle: HANDLE,
        Index: u32,
        KeyInformationClass: KEY_INFORMATION_CLASS,
        KeyInformation: *mut std::ffi::c_void,
        Length: u32,
        ResultLength: *mut u32,
    ) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtEnumerateValueKey(
        KeyHandle: HANDLE,
        Index: u32,
        KeyValueInformationClass: KEY_VALUE_INFORMATION_CLASS,
        KeyValueInformation: *mut std::ffi::c_void,
        Length: u32,
        ResultLength: *mut u32,
    ) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtFlushKey(KeyHandle: HANDLE) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtCompactKeys(Count: u32, KeyArray: *mut HANDLE) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtCompressKey(KeyHandle: HANDLE) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtLoadKey(
        TargetKey: *mut OBJECT_ATTRIBUTES,
        SourceFile: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtLoadKey2(
        TargetKey: *mut OBJECT_ATTRIBUTES,
        SourceFile: *mut OBJECT_ATTRIBUTES,
        Flags: u32,
    ) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtLoadKeyEx(
        TargetKey: *mut OBJECT_ATTRIBUTES,
        SourceFile: *mut OBJECT_ATTRIBUTES,
        Flags: u32,
        TrustClassKey: HANDLE,
        Event: HANDLE,
        DesiredAccess: u32,
        RootHandle: *mut HANDLE,
        Reserved: *mut std::ffi::c_void,
    ) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtLoadKey3(
        TargetKey: *mut OBJECT_ATTRIBUTES,
        SourceFile: *mut OBJECT_ATTRIBUTES,
        Flags: u32,
        ExtendedParameters: *mut CM_EXTENDED_PARAMETER,
        ExtendedParameterCount: u32,
        DesiredAccess: u32,
        RootHandle: *mut HANDLE,
        Reserved: *mut std::ffi::c_void,
    ) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtReplaceKey(
        NewFile: *mut OBJECT_ATTRIBUTES,
        TargetHandle: HANDLE,
        OldFile: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtSaveKey(KeyHandle: HANDLE, FileHandle: HANDLE) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtSaveKeyEx(KeyHandle: HANDLE, FileHandle: HANDLE, Format: u32) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtSaveMergedKeys(
        HighPrecedenceKeyHandle: HANDLE,
        LowPrecedenceKeyHandle: HANDLE,
        FileHandle: HANDLE,
    ) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtRestoreKey(KeyHandle: HANDLE, FileHandle: HANDLE, Flags: u32) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtUnloadKey(TargetKey: *mut OBJECT_ATTRIBUTES) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtUnloadKey2(TargetKey: *mut OBJECT_ATTRIBUTES, Flags: u32) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtUnloadKeyEx(TargetKey: *mut OBJECT_ATTRIBUTES, Event: HANDLE) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtNotifyChangeKey(
        KeyHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: *mut std::ffi::c_void,
        IoStatusBlock: *mut IO_STATUS_BLOCK,
        CompletionFilter: u32,
        WatchTree: BOOLEAN,
        Buffer: *mut std::ffi::c_void,
        BufferSize: u32,
        Asynchronous: BOOLEAN,
    ) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtNotifyChangeMultipleKeys(
        MasterKeyHandle: HANDLE,
        Count: u32,
        SubordinateObjects: *mut OBJECT_ATTRIBUTES,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: *mut std::ffi::c_void,
        IoStatusBlock: *mut IO_STATUS_BLOCK,
        CompletionFilter: u32,
        WatchTree: BOOLEAN,
        Buffer: *mut std::ffi::c_void,
        BufferSize: u32,
        Asynchronous: BOOLEAN,
    ) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtQueryOpenSubKeys(TargetKey: *mut OBJECT_ATTRIBUTES, HandleCount: *mut u32)
    -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtQueryOpenSubKeysEx(
        TargetKey: *mut OBJECT_ATTRIBUTES,
        BufferLength: u32,
        Buffer: *mut std::ffi::c_void,
        RequiredSize: *mut u32,
    ) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtInitializeRegistry(BootCondition: u16) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtLockRegistryKey(KeyHandle: HANDLE) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtLockProductActivationKeys(pPrivateVer: *mut u32, pSafeMode: *mut u32) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtFreezeRegistry(TimeOutInSeconds: u32) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtThawRegistry() -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtCreateRegistryTransaction(
        RegistryTransactionHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjAttributes: *mut OBJECT_ATTRIBUTES,
        CreateOptions: u32,
    ) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtOpenRegistryTransaction(
        RegistryTransactionHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtCommitRegistryTransaction(RegistryTransactionHandle: HANDLE, Flags: u32) -> NTSTATUS;
}
#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtRollbackRegistryTransaction(RegistryTransactionHandle: HANDLE, Flags: u32)
    -> NTSTATUS;
}
