use windows::{
    core::{GUID, PWSTR},
    Win32::Foundation::{HANDLE, NTSTATUS, UNICODE_STRING},
};

use crate::bitfield::{BitfieldUnit, UnionField};

pub const BCD_OBJECT_DESCRIPTION_VERSION: u32 = 1;
pub const BCD_ELEMENT_DESCRIPTION_VERSION: u32 = 1;
pub const BCD_OBJECT_OSLOADER_TYPE: u32 = 270532611;
pub const GUID_BAD_MEMORY_GROUP: GUID = GUID::from_values(
    0x5189B25C,
    0x5558,
    0x4BF2,
    [0xBC, 0xA4, 0x28, 0x9B, 0x11, 0xBD, 0x29, 0xE2],
);
pub const GUID_BOOT_LOADER_SETTINGS_GROUP: GUID = GUID::from_values(
    0x6EFB52BF,
    0x1766,
    0x41DB,
    [0xA6, 0xB3, 0x0E, 0xE5, 0xEF, 0xF7, 0x2B, 0xD7],
);
pub const GUID_CURRENT_BOOT_ENTRY: GUID = GUID::from_values(
    0xFA926493,
    0x6F1C,
    0x4193,
    [0xA4, 0x14, 0x58, 0xF0, 0xB2, 0x45, 0x6D, 0x1E],
);
pub const GUID_DEBUGGER_SETTINGS_GROUP: GUID = GUID::from_values(
    0x4636856E,
    0x540F,
    0x4170,
    [0xA1, 0x30, 0xA8, 0x47, 0x76, 0xF4, 0xC6, 0x54],
);
pub const GUID_DEFAULT_BOOT_ENTRY: GUID = GUID::from_values(
    0x1CAE1EB7,
    0xA0DF,
    0x4D4D,
    [0x98, 0x51, 0x48, 0x60, 0xE3, 0x4E, 0xF5, 0x35],
);
pub const GUID_EMS_SETTINGS_GROUP: GUID = GUID::from_values(
    0x0CE4991B,
    0xE6B3,
    0x4B16,
    [0xB2, 0x3C, 0x5E, 0x0D, 0x92, 0x50, 0xE5, 0xD9],
);
pub const GUID_FIRMWARE_BOOTMGR: GUID = GUID::from_values(
    0xA5A30FA2,
    0x3D06,
    0x4E9F,
    [0xB5, 0xF4, 0xA0, 0x1D, 0xF9, 0xD1, 0xFC, 0xBA],
);
pub const GUID_GLOBAL_SETTINGS_GROUP: GUID = GUID::from_values(
    0x7EA2E1AC,
    0x2E61,
    0x4728,
    [0xAA, 0xA3, 0x89, 0x6D, 0x9D, 0x0A, 0x9F, 0x0E],
);
pub const GUID_HYPERVISOR_SETTINGS_GROUP: GUID = GUID::from_values(
    0x7FF607E0,
    0x4395,
    0x11DB,
    [0xB0, 0xDE, 0x08, 0x00, 0x20, 0x0C, 0x9A, 0x66],
);
pub const GUID_KERNEL_DEBUGGER_SETTINGS_GROUP: GUID = GUID::from_values(
    0x313E8EED,
    0x7098,
    0x4586,
    [0xA9, 0xBF, 0x30, 0x9C, 0x61, 0xF8, 0xD4, 0x49],
);
pub const GUID_RESUME_LOADER_SETTINGS_GROUP: GUID = GUID::from_values(
    0x1AFA9C49,
    0x16AB,
    0x4A5C,
    [0x4A, 0x90, 0x21, 0x28, 0x02, 0xDA, 0x94, 0x60],
);
pub const GUID_WINDOWS_BOOTMGR: GUID = GUID::from_values(
    0x9DEA862C,
    0x5CDD,
    0x4E70,
    [0xAC, 0xC1, 0xF3, 0x2B, 0x34, 0x4D, 0x47, 0x95],
);
pub const GUID_WINDOWS_LEGACY_NTLDR: GUID = GUID::from_values(
    0x466F5A88,
    0x0AF2,
    0x4F76,
    [0x90, 0x38, 0x09, 0x5B, 0x17, 0x0D, 0xC2, 0x1C],
);
pub const GUID_WINDOWS_MEMORY_TESTER: GUID = GUID::from_values(
    0xB2721D73,
    0x1DB4,
    0x4C62,
    [0xBF, 0x78, 0xC5, 0x48, 0xA8, 0x80, 0x14, 0x2D],
);
pub const GUID_WINDOWS_OS_TARGET_TEMPLATE_EFI: GUID = GUID::from_values(
    0xB012B84D,
    0xC47C,
    0x4ED5,
    [0xB7, 0x22, 0xC0, 0xC4, 0x21, 0x63, 0xE5, 0x69],
);
pub const GUID_WINDOWS_OS_TARGET_TEMPLATE_PCAT: GUID = GUID::from_values(
    0xA1943BBC,
    0xEA85,
    0x487C,
    [0x97, 0xC7, 0xC9, 0xED, 0xE9, 0x08, 0xA3, 0x8A],
);
pub const GUID_WINDOWS_RESUME_TARGET_TEMPLATE_EFI: GUID = GUID::from_values(
    0x0C334284,
    0x9A41,
    0x4DE1,
    [0x99, 0xB3, 0xA7, 0xE8, 0x7E, 0x8F, 0xF0, 0x7E],
);
pub const GUID_WINDOWS_RESUME_TARGET_TEMPLATE_PCAT: GUID = GUID::from_values(
    0x98B02A23,
    0x0674,
    0x4CE7,
    [0xBD, 0xAD, 0xE0, 0xA1, 0x5A, 0x8F, 0xF9, 0x7B],
);
pub const GUID_WINDOWS_SETUP_EFI: GUID = GUID::from_values(
    0x7254A080,
    0x1510,
    0x4E85,
    [0xAC, 0x0F, 0xE7, 0xFB, 0x3D, 0x44, 0x47, 0x36],
);
pub const GUID_WINDOWS_SETUP_PCAT: GUID = GUID::from_values(
    0xCBD971BF,
    0xB7B8,
    0x4885,
    [0x95, 0x1A, 0xFA, 0x03, 0x04, 0x4F, 0x5D, 0x71],
);
pub const GUID_WINDOWS_SETUP_RAMDISK_OPTIONS: GUID = GUID::from_values(
    0xAE5534E0,
    0xA924,
    0x466C,
    [0xB8, 0x36, 0x75, 0x85, 0x39, 0xA3, 0xEE, 0x3A],
);
pub const GUID_WINDOWS_SETUP_BOOT_ENTRY: GUID = GUID::from_values(
    0x7619dcc9,
    0xfafe,
    0x11d9,
    [0xb4, 0x11, 0x00, 0x04, 0x76, 0xeb, 0xa2, 0x5f],
);

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum BCD_MESSAGE_TYPE {
    BCD_MESSAGE_TYPE_NONE = 0,
    BCD_MESSAGE_TYPE_TRACE = 1,
    BCD_MESSAGE_TYPE_INFORMATION = 2,
    BCD_MESSAGE_TYPE_WARNING = 3,
    BCD_MESSAGE_TYPE_ERROR = 4,
    BCD_MESSAGE_TYPE_MAXIMUM = 5,
}

pub type BCD_MESSAGE_CALLBACK =
    std::option::Option<unsafe extern "system" fn(type_: BCD_MESSAGE_TYPE, Message: PWSTR)>;

#[link(name = "bcd.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn BcdSetLogging(
        BcdLoggingLevel: BCD_MESSAGE_TYPE,
        BcdMessageCallbackRoutine: BCD_MESSAGE_CALLBACK,
    ) -> NTSTATUS;
}

#[link(name = "bcd.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn BcdInitializeBcdSyncMutant();
}

#[link(name = "bcd.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn BcdGetSystemStorePath(BcdSystemStorePath: *mut PWSTR) -> NTSTATUS;
}

#[link(name = "bcd.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn BcdSetSystemStoreDevice(SystemPartition: UNICODE_STRING) -> NTSTATUS;
}

#[link(name = "bcd.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn BcdOpenSystemStore(BcdStoreHandle: *mut HANDLE) -> NTSTATUS;
}

#[link(name = "bcd.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn BcdOpenStoreFromFile(
        BcdFilePath: UNICODE_STRING,
        BcdStoreHandle: *mut HANDLE,
    ) -> NTSTATUS;
}

#[link(name = "bcd.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn BcdCreateStore(BcdFilePath: UNICODE_STRING, BcdStoreHandle: *mut HANDLE) -> NTSTATUS;
}

#[link(name = "bcd.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn BcdExportStore(BcdFilePath: UNICODE_STRING) -> NTSTATUS;
}

#[link(name = "bcd.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn BcdExportStoreEx(
        BcdStoreHandle: HANDLE,
        Flags: u32,
        BcdFilePath: UNICODE_STRING,
    ) -> NTSTATUS;
}

#[link(name = "bcd.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn BcdImportStore(BcdFilePath: UNICODE_STRING) -> NTSTATUS;
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum BCD_IMPORT_FLAGS {
    BCD_IMPORT_NONE = 0,
    BCD_IMPORT_DELETE_FIRMWARE_OBJECTS = 1,
}

#[link(name = "bcd.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn BcdImportStoreWithFlags(
        BcdFilePath: UNICODE_STRING,
        BcdImportFlags: BCD_IMPORT_FLAGS,
    ) -> NTSTATUS;
}

#[link(name = "bcd.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn BcdDeleteObjectReferences(BcdStoreHandle: HANDLE, Identifier: *mut GUID) -> NTSTATUS;
}

#[link(name = "bcd.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn BcdDeleteSystemStore() -> NTSTATUS;
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum BCD_OPEN_FLAGS {
    BCD_OPEN_NONE = 0,
    BCD_OPEN_OPEN_STORE_OFFLINE = 1,
    BCD_OPEN_SYNC_FIRMWARE_ENTRIES = 2,
}

#[link(name = "bcd.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn BcdOpenStore(
        BcdFilePath: UNICODE_STRING,
        BcdOpenFlags: BCD_OPEN_FLAGS,
        BcdStoreHandle: *mut HANDLE,
    ) -> NTSTATUS;
}

#[link(name = "bcd.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn BcdCloseStore(BcdStoreHandle: HANDLE) -> NTSTATUS;
}

#[link(name = "bcd.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn BcdFlushStore(BcdStoreHandle: HANDLE) -> NTSTATUS;
}

#[link(name = "bcd.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn BcdForciblyUnloadStore(BcdStoreHandle: HANDLE) -> NTSTATUS;
}

#[link(name = "bcd.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn BcdMarkAsSystemStore(BcdStoreHandle: HANDLE) -> NTSTATUS;
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum BCD_OBJECT_TYPE {
    BCD_OBJECT_TYPE_NONE = 0,
    BCD_OBJECT_TYPE_APPLICATION = 1,
    BCD_OBJECT_TYPE_INHERITED = 2,
    BCD_OBJECT_TYPE_DEVICE = 3,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum BCD_APPLICATION_OBJECT_TYPE {
    BCD_APPLICATION_OBJECT_NONE = 0,
    BCD_APPLICATION_OBJECT_FIRMWARE_BOOT_MANAGER = 1,
    BCD_APPLICATION_OBJECT_WINDOWS_BOOT_MANAGER = 2,
    BCD_APPLICATION_OBJECT_WINDOWS_BOOT_LOADER = 3,
    BCD_APPLICATION_OBJECT_WINDOWS_RESUME_APPLICATION = 4,
    BCD_APPLICATION_OBJECT_MEMORY_TESTER = 5,
    BCD_APPLICATION_OBJECT_LEGACY_NTLDR = 6,
    BCD_APPLICATION_OBJECT_LEGACY_SETUPLDR = 7,
    BCD_APPLICATION_OBJECT_BOOT_SECTOR = 8,
    BCD_APPLICATION_OBJECT_STARTUP_MODULE = 9,
    BCD_APPLICATION_OBJECT_GENERIC_APPLICATION = 10,
    BCD_APPLICATION_OBJECT_RESERVED = 1048575,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum BCD_APPLICATION_IMAGE_TYPE {
    BCD_APPLICATION_IMAGE_NONE = 0,
    BCD_APPLICATION_IMAGE_FIRMWARE_APPLICATION = 1,
    BCD_APPLICATION_IMAGE_BOOT_APPLICATION = 2,
    BCD_APPLICATION_IMAGE_LEGACY_LOADER = 3,
    BCD_APPLICATION_IMAGE_REALMODE_CODE = 4,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum BCD_INHERITED_CLASS_TYPE {
    BCD_INHERITED_CLASS_NONE = 0,
    BCD_INHERITED_CLASS_LIBRARY = 1,
    BCD_INHERITED_CLASS_APPLICATION = 2,
    BCD_INHERITED_CLASS_DEVICE = 3,
}

#[repr(C)]
pub struct BCD_OBJECT_DATATYPE {
    pub PackedValue: UnionField<u32>,
    pub Anonymous1: UnionField<BCD_OBJECT_DATATYPE_1>,
    pub union_field: u32,
}

#[repr(C)]
pub struct BCD_OBJECT_DATATYPE_1 {
    pub Anonymous1: UnionField<BCD_OBJECT_DATATYPE_1_1>,
    pub Application: UnionField<BCD_OBJECT_DATATYPE_1_2>,
    pub Inherit: UnionField<BCD_OBJECT_DATATYPE_1_3>,
    pub Device: UnionField<BCD_OBJECT_DATATYPE_1_4>,
    pub union_field: u32,
}

#[repr(C)]
#[repr(align(4))]
pub struct BCD_OBJECT_DATATYPE_1_1 {
    _bitfield_align_1: [u32; 0],
    _bitfield_1: BitfieldUnit<[u8; 4]>,
}

impl Default for BCD_OBJECT_DATATYPE_1_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for BCD_OBJECT_DATATYPE_1_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BCD_OBJECT_DATATYPE_1_1 {{ Reserved : {:?}, ObjectType : {:?} }}",
            self.Reserved(),
            self.ObjectType()
        )
    }
}

impl BCD_OBJECT_DATATYPE_1_1 {
    #[inline]
    pub fn Reserved(&self) -> u32 {
        self._bitfield_1.get(0usize, 28u8) as u32
    }

    #[inline]
    pub fn set_Reserved(&mut self, val: u32) {
        self._bitfield_1.set(0usize, 28u8, val as u64)
    }

    #[inline]
    pub fn ObjectType(&self) -> BCD_OBJECT_TYPE {
        unsafe { std::mem::transmute(self._bitfield_1.get(28usize, 4u8) as u32) }
    }

    #[inline]
    pub fn set_ObjectType(&mut self, val: BCD_OBJECT_TYPE) {
        self._bitfield_1.set(28usize, 4u8, val as u64)
    }

    #[inline]
    pub fn new_bitfield_1(Reserved: u32, ObjectType: BCD_OBJECT_TYPE) -> BitfieldUnit<[u8; 4]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 4]> = Default::default();

        bitfield_unit.set(0usize, 28u8, Reserved as u64);

        bitfield_unit.set(28usize, 4u8, {
            let ObjectType: u32 = unsafe { std::mem::transmute(ObjectType) };

            ObjectType as u64
        });

        bitfield_unit
    }
}

#[repr(C)]
#[repr(align(4))]
pub struct BCD_OBJECT_DATATYPE_1_2 {
    _bitfield_align_1: [u32; 0],
    _bitfield_1: BitfieldUnit<[u8; 4]>,
}

impl Default for BCD_OBJECT_DATATYPE_1_2 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for BCD_OBJECT_DATATYPE_1_2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BCD_OBJECT_DATATYPE_1_2 {{ ApplicationType : {:?}, ImageType : {:?}, Reserved : {:?}, ObjectType : {:?} }}",
            self.ApplicationType(),
            self.ImageType(),
            self.Reserved(),
            self.ObjectType()
        )
    }
}

impl BCD_OBJECT_DATATYPE_1_2 {
    #[inline]
    pub fn ApplicationType(&self) -> BCD_APPLICATION_OBJECT_TYPE {
        unsafe { std::mem::transmute(self._bitfield_1.get(0usize, 20u8) as u32) }
    }

    #[inline]
    pub fn set_ApplicationType(&mut self, val: BCD_APPLICATION_OBJECT_TYPE) {
        unsafe {
            let val: u32 = std::mem::transmute(val);

            self._bitfield_1.set(0usize, 20u8, val as u64)
        }
    }

    #[inline]
    pub fn ImageType(&self) -> BCD_APPLICATION_IMAGE_TYPE {
        unsafe { std::mem::transmute(self._bitfield_1.get(20usize, 4u8) as u32) }
    }

    #[inline]
    pub fn set_ImageType(&mut self, val: BCD_APPLICATION_IMAGE_TYPE) {
        unsafe {
            let val: u32 = std::mem::transmute(val);

            self._bitfield_1.set(20usize, 4u8, val as u64)
        }
    }

    #[inline]
    pub fn Reserved(&self) -> u32 {
        self._bitfield_1.get(24usize, 4u8) as u32
    }

    #[inline]
    pub fn set_Reserved(&mut self, val: u32) {
        self._bitfield_1.set(24usize, 4u8, val as u64)
    }

    #[inline]
    pub fn ObjectType(&self) -> BCD_OBJECT_TYPE {
        unsafe { std::mem::transmute(self._bitfield_1.get(28usize, 4u8) as u32) }
    }

    #[inline]
    pub fn set_ObjectType(&mut self, val: BCD_OBJECT_TYPE) {
        unsafe {
            let val: u32 = std::mem::transmute(val);

            self._bitfield_1.set(28usize, 4u8, val as u64)
        }
    }

    #[inline]
    pub fn new_bitfield_1(
        ApplicationType: BCD_APPLICATION_OBJECT_TYPE,
        ImageType: BCD_APPLICATION_IMAGE_TYPE,
        Reserved: u32,
        ObjectType: BCD_OBJECT_TYPE,
    ) -> BitfieldUnit<[u8; 4]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 4]> = Default::default();

        bitfield_unit.set(0usize, 20u8, {
            let ApplicationType: u32 = unsafe { std::mem::transmute(ApplicationType) };

            ApplicationType as u64
        });

        bitfield_unit.set(20usize, 4u8, {
            let ImageType: u32 = unsafe { std::mem::transmute(ImageType) };

            ImageType as u64
        });

        bitfield_unit.set(24usize, 4u8, Reserved as u64);

        bitfield_unit.set(28usize, 4u8, {
            let ObjectType: u32 = unsafe { std::mem::transmute(ObjectType) };

            ObjectType as u64
        });

        bitfield_unit
    }
}

#[repr(C)]
#[repr(align(4))]
pub struct BCD_OBJECT_DATATYPE_1_3 {
    _bitfield_align_1: [u32; 0],
    _bitfield_1: BitfieldUnit<[u8; 4]>,
}

impl Default for BCD_OBJECT_DATATYPE_1_3 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for BCD_OBJECT_DATATYPE_1_3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BCD_OBJECT_DATATYPE_1_3 {{ Value : {:?}, Class : {:?}, Reserved : {:?}, ObjectType : {:?} }}",
            self.Value(),
            self.Class(),
            self.Reserved(),
            self.ObjectType()
        )
    }
}

impl BCD_OBJECT_DATATYPE_1_3 {
    #[inline]
    pub fn Value(&self) -> u32 {
        self._bitfield_1.get(0usize, 20u8) as u32
    }

    #[inline]
    pub fn set_Value(&mut self, val: u32) {
        self._bitfield_1.set(0usize, 20u8, val as u64)
    }

    #[inline]
    pub fn Class(&self) -> BCD_INHERITED_CLASS_TYPE {
        unsafe { std::mem::transmute(self._bitfield_1.get(20usize, 4u8) as u32) }
    }

    #[inline]
    pub fn set_Class(&mut self, val: BCD_INHERITED_CLASS_TYPE) {
        unsafe {
            let val: u32 = std::mem::transmute(val);

            self._bitfield_1.set(20usize, 4u8, val as u64)
        }
    }

    #[inline]
    pub fn Reserved(&self) -> u32 {
        self._bitfield_1.get(24usize, 4u8) as u32
    }

    #[inline]
    pub fn set_Reserved(&mut self, val: u32) {
        self._bitfield_1.set(24usize, 4u8, val as u64)
    }

    #[inline]
    pub fn ObjectType(&self) -> BCD_OBJECT_TYPE {
        unsafe { std::mem::transmute(self._bitfield_1.get(28usize, 4u8) as u32) }
    }

    #[inline]
    pub fn set_ObjectType(&mut self, val: BCD_OBJECT_TYPE) {
        unsafe {
            let val: u32 = std::mem::transmute(val);

            self._bitfield_1.set(28usize, 4u8, val as u64)
        }
    }

    #[inline]
    pub fn new_bitfield_1(
        Value: u32,
        Class: BCD_INHERITED_CLASS_TYPE,
        Reserved: u32,
        ObjectType: BCD_OBJECT_TYPE,
    ) -> BitfieldUnit<[u8; 4]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 4]> = Default::default();

        bitfield_unit.set(0usize, 20u8, Value as u64);

        bitfield_unit.set(20usize, 4u8, {
            let Class: u32 = unsafe { std::mem::transmute(Class) };

            Class as u64
        });

        bitfield_unit.set(24usize, 4u8, Reserved as u64);

        bitfield_unit.set(28usize, 4u8, {
            let ObjectType: u32 = unsafe { std::mem::transmute(ObjectType) };

            ObjectType as u64
        });

        bitfield_unit
    }
}

#[repr(C)]
#[repr(align(4))]
pub struct BCD_OBJECT_DATATYPE_1_4 {
    _bitfield_align_1: [u32; 0],
    _bitfield_1: BitfieldUnit<[u8; 4]>,
}

impl Default for BCD_OBJECT_DATATYPE_1_4 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for BCD_OBJECT_DATATYPE_1_4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BCD_OBJECT_DATATYPE_1_4 {{ Reserved : {:?}, ObjectType : {:?} }}",
            self.Reserved(),
            self.ObjectType()
        )
    }
}

impl BCD_OBJECT_DATATYPE_1_4 {
    #[inline]
    pub fn Reserved(&self) -> u32 {
        self._bitfield_1.get(0usize, 28u8) as u32
    }

    #[inline]
    pub fn set_Reserved(&mut self, val: u32) {
        self._bitfield_1.set(0usize, 28u8, val as u64)
    }

    #[inline]
    pub fn ObjectType(&self) -> BCD_OBJECT_TYPE {
        unsafe { std::mem::transmute(self._bitfield_1.get(28usize, 4u8) as u32) }
    }

    #[inline]
    pub fn set_ObjectType(&mut self, val: BCD_OBJECT_TYPE) {
        unsafe {
            let val: u32 = std::mem::transmute(val);

            self._bitfield_1.set(28usize, 4u8, val as u64)
        }
    }

    #[inline]
    pub fn new_bitfield_1(Reserved: u32, ObjectType: BCD_OBJECT_TYPE) -> BitfieldUnit<[u8; 4]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 4]> = Default::default();

        bitfield_unit.set(0usize, 28u8, Reserved as u64);

        bitfield_unit.set(28usize, 4u8, {
            let ObjectType: u32 = unsafe { std::mem::transmute(ObjectType) };

            ObjectType as u64
        });

        bitfield_unit
    }
}

impl Default for BCD_OBJECT_DATATYPE_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for BCD_OBJECT_DATATYPE_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BCD_OBJECT_DATATYPE_1 {{ union }}")
    }
}

impl Default for BCD_OBJECT_DATATYPE {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for BCD_OBJECT_DATATYPE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BCD_OBJECT_DATATYPE {{ union }}")
    }
}

#[repr(C)]
pub struct BCD_OBJECT_DESCRIPTION {
    pub Version: u32,
    pub Type: u32,
}

impl Default for BCD_OBJECT_DESCRIPTION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for BCD_OBJECT_DESCRIPTION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BCD_OBJECT_DESCRIPTION {{  }}")
    }
}

#[repr(C)]
pub struct BCD_OBJECT {
    pub Identifer: GUID,
    pub Description: *mut BCD_OBJECT_DESCRIPTION,
}

impl Default for BCD_OBJECT {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for BCD_OBJECT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BCD_OBJECT {{ Description: {:?} }}", self.Description)
    }
}

#[link(name = "bcd.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn BcdEnumerateObjects(
        BcdStoreHandle: HANDLE,
        BcdEnumDescriptor: *mut BCD_OBJECT_DESCRIPTION,
        Buffer: *mut std::ffi::c_void,
        BufferSize: *mut u32,
        ObjectCount: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "bcd.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn BcdOpenObject(
        BcdStoreHandle: HANDLE,
        Identifier: *const GUID,
        BcdObjectHandle: *mut HANDLE,
    ) -> NTSTATUS;
}

#[link(name = "bcd.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn BcdCreateObject(
        BcdStoreHandle: HANDLE,
        Identifier: *mut GUID,
        Description: *mut BCD_OBJECT_DESCRIPTION,
        BcdObjectHandle: *mut HANDLE,
    ) -> NTSTATUS;
}

#[link(name = "bcd.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn BcdDeleteObject(BcdObjectHandle: HANDLE) -> NTSTATUS;
}

#[link(name = "bcd.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn BcdCloseObject(BcdObjectHandle: HANDLE) -> NTSTATUS;
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum BCD_COPY_FLAGS {
    BCD_COPY_NONE = 0,
    BCD_COPY_COPY_CREATE_NEW_OBJECT_IDENTIFIER = 1,
    BCD_COPY_COPY_DELETE_EXISTING_OBJECT = 2,
    BCD_COPY_COPY_UNKNOWN_FIRMWARE_APPLICATION = 4,
    BCD_COPY_IGNORE_SETUP_TEMPLATE_ELEMENTS = 8,
    BCD_COPY_RETAIN_ELEMENT_DATA = 16,
    BCD_COPY_MIGRATE_ELEMENT_DATA = 32,
}

#[link(name = "bcd.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn BcdCopyObject(
        BcdStoreHandle: HANDLE,
        BcdObjectHandle: HANDLE,
        BcdCopyFlags: BCD_COPY_FLAGS,
        TargetStoreHandle: HANDLE,
        TargetObjectHandle: *mut HANDLE,
    ) -> NTSTATUS;
}

#[link(name = "bcd.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn BcdCopyObjectEx(
        BcdStoreHandle: HANDLE,
        BcdObjectHandle: HANDLE,
        BcdCopyFlags: BCD_COPY_FLAGS,
        TargetStoreHandle: HANDLE,
        TargetObjectId: *mut GUID,
        TargetObjectHandle: *mut HANDLE,
    ) -> NTSTATUS;
}

#[link(name = "bcd.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn BcdCopyObjects(
        BcdStoreHandle: HANDLE,
        Characteristics: BCD_OBJECT_DESCRIPTION,
        BcdCopyFlags: BCD_COPY_FLAGS,
        TargetStoreHandle: HANDLE,
    ) -> NTSTATUS;
}

#[link(name = "bcd.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn BcdMigrateObjectElementValues(
        TemplateObjectHandle: HANDLE,
        SourceObjectHandle: HANDLE,
        TargetObjectHandle: HANDLE,
    ) -> NTSTATUS;
}

#[link(name = "bcd.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn BcdQueryObject(
        BcdObjectHandle: HANDLE,
        BcdVersion: u32,
        Description: BCD_OBJECT_DESCRIPTION,
        Identifier: *mut GUID,
    ) -> NTSTATUS;
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum BCD_ELEMENT_DATATYPE_FORMAT {
    BCD_ELEMENT_DATATYPE_FORMAT_UNKNOWN = 0,
    BCD_ELEMENT_DATATYPE_FORMAT_DEVICE = 1,
    BCD_ELEMENT_DATATYPE_FORMAT_STRING = 2,
    BCD_ELEMENT_DATATYPE_FORMAT_OBJECT = 3,
    BCD_ELEMENT_DATATYPE_FORMAT_OBJECTLIST = 4,
    BCD_ELEMENT_DATATYPE_FORMAT_INTEGER = 5,
    BCD_ELEMENT_DATATYPE_FORMAT_BOOLEAN = 6,
    BCD_ELEMENT_DATATYPE_FORMAT_INTEGERLIST = 7,
    BCD_ELEMENT_DATATYPE_FORMAT_BINARY = 8,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum BCD_ELEMENT_DATATYPE_CLASS {
    BCD_ELEMENT_DATATYPE_CLASS_NONE = 0,
    BCD_ELEMENT_DATATYPE_CLASS_LIBRARY = 1,
    BCD_ELEMENT_DATATYPE_CLASS_APPLICATION = 2,
    BCD_ELEMENT_DATATYPE_CLASS_DEVICE = 3,
    BCD_ELEMENT_DATATYPE_CLASS_SETUPTEMPLATE = 4,
    BCD_ELEMENT_DATATYPE_CLASS_OEM = 5,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum BCD_ELEMENT_DEVICE_TYPE {
    BCD_ELEMENT_DEVICE_TYPE_NONE = 0,
    BCD_ELEMENT_DEVICE_TYPE_BOOT_DEVICE = 1,
    BCD_ELEMENT_DEVICE_TYPE_PARTITION = 2,
    BCD_ELEMENT_DEVICE_TYPE_FILE = 3,
    BCD_ELEMENT_DEVICE_TYPE_RAMDISK = 4,
    BCD_ELEMENT_DEVICE_TYPE_UNKNOWN = 5,
    BCD_ELEMENT_DEVICE_TYPE_QUALIFIED_PARTITION = 6,
    BCD_ELEMENT_DEVICE_TYPE_VMBUS = 7,
    BCD_ELEMENT_DEVICE_TYPE_LOCATE_DEVICE = 8,
    BCD_ELEMENT_DEVICE_TYPE_URI = 9,
    BCD_ELEMENT_DEVICE_TYPE_COMPOSITE = 10,
}

#[repr(C)]
pub struct BCD_ELEMENT_DATATYPE {
    pub PackedValue: UnionField<u32>,
    pub Anonymous1: UnionField<BCD_ELEMENT_DATATYPE_1>,
    pub union_field: u32,
}

#[repr(C)]
#[repr(align(4))]
pub struct BCD_ELEMENT_DATATYPE_1 {
    _bitfield_align_1: [u32; 0],
    _bitfield_1: BitfieldUnit<[u8; 4]>,
}

impl Default for BCD_ELEMENT_DATATYPE_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for BCD_ELEMENT_DATATYPE_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BCD_ELEMENT_DATATYPE_1 {{ SubType : {:?}, Format : {:?}, Class : {:?} }}",
            self.SubType(),
            self.Format(),
            self.Class()
        )
    }
}

impl BCD_ELEMENT_DATATYPE_1 {
    #[inline]
    pub fn SubType(&self) -> u32 {
        self._bitfield_1.get(0usize, 24u8) as u32
    }

    #[inline]
    pub fn set_SubType(&mut self, val: u32) {
        self._bitfield_1.set(0usize, 24u8, val as u64)
    }

    #[inline]
    pub fn Format(&self) -> BCD_ELEMENT_DATATYPE_FORMAT {
        unsafe { std::mem::transmute(self._bitfield_1.get(24usize, 4u8) as u32) }
    }

    #[inline]
    pub fn set_Format(&mut self, val: BCD_ELEMENT_DATATYPE_FORMAT) {
        unsafe {
            let val: u32 = std::mem::transmute(val);

            self._bitfield_1.set(24usize, 4u8, val as u64)
        }
    }

    #[inline]
    pub fn Class(&self) -> BCD_ELEMENT_DATATYPE_CLASS {
        unsafe { std::mem::transmute(self._bitfield_1.get(28usize, 4u8) as u32) }
    }

    #[inline]
    pub fn set_Class(&mut self, val: BCD_ELEMENT_DATATYPE_CLASS) {
        unsafe {
            let val: u32 = std::mem::transmute(val);

            self._bitfield_1.set(28usize, 4u8, val as u64)
        }
    }

    #[inline]
    pub fn new_bitfield_1(
        SubType: u32,
        Format: BCD_ELEMENT_DATATYPE_FORMAT,
        Class: BCD_ELEMENT_DATATYPE_CLASS,
    ) -> BitfieldUnit<[u8; 4]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 4]> = Default::default();

        bitfield_unit.set(0usize, 24u8, SubType as u64);

        bitfield_unit.set(24usize, 4u8, {
            let Format: u32 = unsafe { std::mem::transmute(Format) };

            Format as u64
        });

        bitfield_unit.set(28usize, 4u8, {
            let Class: u32 = unsafe { std::mem::transmute(Class) };

            Class as u64
        });

        bitfield_unit
    }
}

impl Default for BCD_ELEMENT_DATATYPE {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for BCD_ELEMENT_DATATYPE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BCD_ELEMENT_DATATYPE {{ union }}")
    }
}

#[link(name = "bcd.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn BcdEnumerateElementTypes(
        BcdObjectHandle: HANDLE,
        Buffer: *mut std::ffi::c_void,
        BufferSize: *mut u32,
        ElementCount: *mut u32,
    ) -> NTSTATUS;
}

#[repr(C)]
pub struct BCD_ELEMENT_DEVICE_QUALIFIED_PARTITION {
    pub PartitionStyle: u32,
    pub Reserved: u32,
    pub Anonymous1: BCD_ELEMENT_DEVICE_QUALIFIED_PARTITION_1,
}

#[repr(C)]
pub struct BCD_ELEMENT_DEVICE_QUALIFIED_PARTITION_1 {
    pub Mbr: BCD_ELEMENT_DEVICE_QUALIFIED_PARTITION_1_1,
    pub Gpt: BCD_ELEMENT_DEVICE_QUALIFIED_PARTITION_1_2,
}

#[repr(C)]
pub struct BCD_ELEMENT_DEVICE_QUALIFIED_PARTITION_1_1 {
    pub DiskSignature: UnionField<u32>,
    pub PartitionOffset: UnionField<u64>,
    pub union_field: u64,
}

impl Default for BCD_ELEMENT_DEVICE_QUALIFIED_PARTITION_1_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for BCD_ELEMENT_DEVICE_QUALIFIED_PARTITION_1_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BCD_ELEMENT_DEVICE_QUALIFIED_PARTITION_1_1 {{ union }}")
    }
}

#[repr(C)]
pub struct BCD_ELEMENT_DEVICE_QUALIFIED_PARTITION_1_2 {
    pub DiskSignature: UnionField<GUID>,
    pub PartitionSignature: UnionField<GUID>,
    pub union_field: [u32; 4],
}

impl Default for BCD_ELEMENT_DEVICE_QUALIFIED_PARTITION_1_2 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for BCD_ELEMENT_DEVICE_QUALIFIED_PARTITION_1_2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BCD_ELEMENT_DEVICE_QUALIFIED_PARTITION_1_2 {{ union }}")
    }
}

impl Default for BCD_ELEMENT_DEVICE_QUALIFIED_PARTITION_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for BCD_ELEMENT_DEVICE_QUALIFIED_PARTITION_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BCD_ELEMENT_DEVICE_QUALIFIED_PARTITION_1 {{ Mbr: {:?}, Gpt: {:?} }}",
            self.Mbr, self.Gpt
        )
    }
}

impl Default for BCD_ELEMENT_DEVICE_QUALIFIED_PARTITION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for BCD_ELEMENT_DEVICE_QUALIFIED_PARTITION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BCD_ELEMENT_DEVICE_QUALIFIED_PARTITION {{ Anonymous1: {:?} }}",
            self.Anonymous1
        )
    }
}

#[repr(C)]
pub struct BCD_ELEMENT_DEVICE {
    pub DeviceType: u32,
    pub AdditionalOptions: GUID,
    pub Anonymous1: BCD_ELEMENT_DEVICE_1,
}

#[repr(C)]
pub struct BCD_ELEMENT_DEVICE_1 {
    pub File: BCD_ELEMENT_DEVICE_1_1,
    pub Partition: BCD_ELEMENT_DEVICE_1_2,
    pub Locate: BCD_ELEMENT_DEVICE_1_3,
    pub Vmbus: BCD_ELEMENT_DEVICE_1_4,
    pub Unknown: BCD_ELEMENT_DEVICE_1_5,
    pub QualifiedPartition: BCD_ELEMENT_DEVICE_QUALIFIED_PARTITION,
}

#[repr(C)]
pub struct BCD_ELEMENT_DEVICE_1_1 {
    pub ParentOffset: UnionField<u32>,
    pub Path: UnionField<[u16; 1]>,
    pub union_field: u32,
}

impl Default for BCD_ELEMENT_DEVICE_1_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for BCD_ELEMENT_DEVICE_1_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BCD_ELEMENT_DEVICE_1_1 {{ union }}")
    }
}

#[repr(C)]
pub struct BCD_ELEMENT_DEVICE_1_2 {
    pub Path: UnionField<[u16; 1]>,
    pub union_field: u16,
}

impl Default for BCD_ELEMENT_DEVICE_1_2 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for BCD_ELEMENT_DEVICE_1_2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BCD_ELEMENT_DEVICE_1_2 {{ union }}")
    }
}

#[repr(C)]
pub struct BCD_ELEMENT_DEVICE_1_3 {
    pub Type: UnionField<u32>,
    pub ParentOffset: UnionField<u32>,
    pub ElementType: UnionField<u32>,
    pub Path: UnionField<[u16; 1]>,
    pub union_field: u32,
}

impl Default for BCD_ELEMENT_DEVICE_1_3 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for BCD_ELEMENT_DEVICE_1_3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BCD_ELEMENT_DEVICE_1_3 {{ union }}")
    }
}

#[repr(C)]
pub struct BCD_ELEMENT_DEVICE_1_4 {
    pub InterfaceInstance: UnionField<GUID>,
    pub union_field: [u32; 4],
}

impl Default for BCD_ELEMENT_DEVICE_1_4 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for BCD_ELEMENT_DEVICE_1_4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BCD_ELEMENT_DEVICE_1_4 {{ union }}")
    }
}

#[repr(C)]
pub struct BCD_ELEMENT_DEVICE_1_5 {
    pub Data: UnionField<[u32; 1]>,
    pub union_field: u32,
}

impl Default for BCD_ELEMENT_DEVICE_1_5 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for BCD_ELEMENT_DEVICE_1_5 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BCD_ELEMENT_DEVICE_1_5 {{ union }}")
    }
}

impl Default for BCD_ELEMENT_DEVICE_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for BCD_ELEMENT_DEVICE_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BCD_ELEMENT_DEVICE_1 {{ File: {:?}, Partition: {:?}, Locate: {:?}, Vmbus: {:?}, Unknown: {:?}, QualifiedPartition: {:?} }}",
            self.File,
            self.Partition,
            self.Locate,
            self.Vmbus,
            self.Unknown,
            self.QualifiedPartition
        )
    }
}

impl Default for BCD_ELEMENT_DEVICE {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for BCD_ELEMENT_DEVICE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BCD_ELEMENT_DEVICE {{ Anonymous1: {:?} }}",
            self.Anonymous1
        )
    }
}

#[repr(C)]
pub struct BCD_ELEMENT_STRING {
    pub Value: [u16; 1],
}

impl Default for BCD_ELEMENT_STRING {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for BCD_ELEMENT_STRING {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BCD_ELEMENT_STRING {{ Value: {:?} }}", self.Value)
    }
}

#[repr(C)]
pub struct BCD_ELEMENT_OBJECT {
    pub Object: GUID,
}

impl Default for BCD_ELEMENT_OBJECT {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for BCD_ELEMENT_OBJECT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BCD_ELEMENT_OBJECT {{  }}")
    }
}

#[repr(C)]
pub struct BCD_ELEMENT_OBJECT_LIST {
    pub ObjectList: [GUID; 1],
}

impl Default for BCD_ELEMENT_OBJECT_LIST {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for BCD_ELEMENT_OBJECT_LIST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BCD_ELEMENT_OBJECT_LIST {{ ObjectList: {:?} }}",
            self.ObjectList
        )
    }
}

#[repr(C)]
pub struct BCD_ELEMENT_INTEGER {
    pub Value: u64,
}

impl Default for BCD_ELEMENT_INTEGER {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for BCD_ELEMENT_INTEGER {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BCD_ELEMENT_INTEGER {{  }}")
    }
}

#[repr(C)]
pub struct BCD_ELEMENT_INTEGER_LIST {
    pub Value: [u64; 1],
}

impl Default for BCD_ELEMENT_INTEGER_LIST {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for BCD_ELEMENT_INTEGER_LIST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BCD_ELEMENT_INTEGER_LIST {{ Value: {:?} }}", self.Value)
    }
}

#[repr(C)]
pub struct BCD_ELEMENT_BOOLEAN {
    pub Value: bool,
}

impl Default for BCD_ELEMENT_BOOLEAN {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for BCD_ELEMENT_BOOLEAN {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BCD_ELEMENT_BOOLEAN {{  }}")
    }
}

#[repr(C)]
pub struct BCD_ELEMENT_DESCRIPTION {
    pub Version: u32,
    pub Type: u32,
    pub DataSize: u32,
}

impl Default for BCD_ELEMENT_DESCRIPTION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for BCD_ELEMENT_DESCRIPTION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BCD_ELEMENT_DESCRIPTION {{  }}")
    }
}

#[repr(C)]
pub struct BCD_ELEMENT {
    pub Description: *mut BCD_ELEMENT_DESCRIPTION,
    pub Data: *mut std::ffi::c_void,
}

impl Default for BCD_ELEMENT {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for BCD_ELEMENT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BCD_ELEMENT {{ Description: {:?} }}", self.Description)
    }
}

#[link(name = "bcd.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn BcdEnumerateElements(
        BcdObjectHandle: HANDLE,
        Buffer: *mut std::ffi::c_void,
        BufferSize: *mut u32,
        ElementCount: *mut u32,
    ) -> NTSTATUS;
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum BCD_FLAGS {
    BCD_FLAG_NONE = 0,
    BCD_FLAG_QUALIFIED_PARTITION = 1,
    BCD_FLAG_NO_DEVICE_TRANSLATION = 2,
    BCD_FLAG_ENUMERATE_INHERITED_OBJECTS = 4,
    BCD_FLAG_ENUMERATE_DEVICE_OPTIONS = 8,
    BCD_FLAG_OBSERVE_PRECEDENCE = 16,
    BCD_FLAG_DISABLE_VHD_NT_TRANSLATION = 32,
    BCD_FLAG_DISABLE_VHD_DEVICE_DETECTION = 64,
    BCD_FLAG_DISABLE_POLICY_CHECKS = 128,
}

#[link(name = "bcd.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn BcdEnumerateElementsWithFlags(
        BcdObjectHandle: HANDLE,
        BcdFlags: BCD_FLAGS,
        Buffer: *mut std::ffi::c_void,
        BufferSize: *mut u32,
        ElementCount: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "bcd.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn BcdEnumerateAndUnpackElements(
        BcdStoreHandle: HANDLE,
        BcdObjectHandle: HANDLE,
        BcdFlags: BCD_FLAGS,
        Buffer: *mut std::ffi::c_void,
        BufferSize: *mut u32,
        ElementCount: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "bcd.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn BcdGetElementData(
        BcdObjectHandle: HANDLE,
        BcdElement: u32,
        Buffer: *mut std::ffi::c_void,
        BufferSize: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "bcd.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn BcdGetElementDataWithFlags(
        BcdObjectHandle: HANDLE,
        BcdElement: u32,
        BcdFlags: BCD_FLAGS,
        Buffer: *mut std::ffi::c_void,
        BufferSize: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "bcd.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn BcdSetElementData(
        BcdObjectHandle: HANDLE,
        BcdElement: u32,
        Buffer: *mut std::ffi::c_void,
        BufferSize: u32,
    ) -> NTSTATUS;
}

#[link(name = "bcd.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn BcdSetElementDataWithFlags(
        BcdObjectHandle: HANDLE,
        BcdElement: u32,
        BcdFlags: BCD_FLAGS,
        Buffer: *mut std::ffi::c_void,
        BufferSize: u32,
    ) -> NTSTATUS;
}

#[link(name = "bcd.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn BcdDeleteElement(BcdObjectHandle: HANDLE, BcdElement: u32) -> NTSTATUS;
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum BcdBootMgrElementTypes {
    BcdBootMgrObjectList_DisplayOrder = 603979777,
    BcdBootMgrObjectList_BootSequence = 603979778,
    BcdBootMgrObject_DefaultObject = 587202563,
    BcdBootMgrInteger_Timeout = 620756996,
    BcdBootMgrBoolean_AttemptResume = 637534213,
    BcdBootMgrObject_ResumeObject = 587202566,
    BcdBootMgrObjectList_StartupSequence = 603979783,
    BcdBootMgrObjectList_ToolsDisplayOrder = 603979792,
    BcdBootMgrBoolean_DisplayBootMenu = 637534240,
    BcdBootMgrBoolean_NoErrorDisplay = 637534241,
    BcdBootMgrDevice_BcdDevice = 553648162,
    BcdBootMgrString_BcdFilePath = 570425379,
    BcdBootMgrBoolean_HormEnabled = 637534244,
    BcdBootMgrBoolean_HiberRoot = 637534245,
    BcdBootMgrString_PasswordOverride = 570425382,
    BcdBootMgrString_PinpassPhraseOverride = 570425383,
    BcdBootMgrBoolean_ProcessCustomActionsFirst = 637534248,
    BcdBootMgrIntegerList_CustomActionsList = 654311472,
    BcdBootMgrBoolean_PersistBootSequence = 637534257,
    BcdBootMgrBoolean_SkipStartupSequence = 637534258,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum BcdLibrary_FirstMegabytePolicy {
    FirstMegabytePolicyUseNone = 0,
    FirstMegabytePolicyUseAll = 1,
    FirstMegabytePolicyUsePrivate = 2,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum BcdLibrary_DebuggerType {
    DebuggerSerial = 0,
    Debugger1394 = 1,
    DebuggerUsb = 2,
    DebuggerNet = 3,
    DebuggerLocal = 4,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum BcdLibrary_DebuggerStartPolicy {
    DebuggerStartActive = 0,
    DebuggerStartAutoEnable = 1,
    DebuggerStartDisable = 2,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum BcdLibrary_ConfigAccessPolicy {
    ConfigAccessPolicyDefault = 0,
    ConfigAccessPolicyDisallowMmConfig = 1,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum BcdLibrary_UxDisplayMessageType {
    DisplayMessageTypeDefault = 0,
    DisplayMessageTypeResume = 1,
    DisplayMessageTypeHyperV = 2,
    DisplayMessageTypeRecovery = 3,
    DisplayMessageTypeStartupRepair = 4,
    DisplayMessageTypeSystemImageRecovery = 5,
    DisplayMessageTypeCommandPrompt = 6,
    DisplayMessageTypeSystemRestore = 7,
    DisplayMessageTypePushButtonReset = 8,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum BcdLibrary_SafeBoot {
    SafemodeMinimal = 0,
    SafemodeNetwork = 1,
    SafemodeDsRepair = 2,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum BcdLibraryElementTypes {
    BcdLibraryDevice_ApplicationDevice = 285212673,
    BcdLibraryString_ApplicationPath = 301989890,
    BcdLibraryString_Description = 301989892,
    BcdLibraryString_PreferredLocale = 301989893,
    BcdLibraryObjectList_InheritedObjects = 335544326,
    BcdLibraryInteger_TruncatePhysicalMemory = 352321543,
    BcdLibraryObjectList_RecoverySequence = 335544328,
    BcdLibraryBoolean_AutoRecoveryEnabled = 369098761,
    BcdLibraryIntegerList_BadMemoryList = 385875978,
    BcdLibraryBoolean_AllowBadMemoryAccess = 369098763,
    BcdLibraryInteger_FirstMegabytePolicy = 352321548,
    BcdLibraryInteger_RelocatePhysicalMemory = 352321549,
    BcdLibraryInteger_AvoidLowPhysicalMemory = 352321550,
    BcdLibraryBoolean_TraditionalKsegMappings = 369098767,
    BcdLibraryBoolean_DebuggerEnabled = 369098768,
    BcdLibraryInteger_DebuggerType = 352321553,
    BcdLibraryInteger_SerialDebuggerPortAddress = 352321554,
    BcdLibraryInteger_SerialDebuggerPort = 352321555,
    BcdLibraryInteger_SerialDebuggerBaudRate = 352321556,
    BcdLibraryInteger_1394DebuggerChannel = 352321557,
    BcdLibraryString_UsbDebuggerTargetName = 301989910,
    BcdLibraryBoolean_DebuggerIgnoreUsermodeExceptions = 369098775,
    BcdLibraryInteger_DebuggerStartPolicy = 352321560,
    BcdLibraryString_DebuggerBusParameters = 301989913,
    BcdLibraryInteger_DebuggerNetHostIP = 352321562,
    BcdLibraryInteger_DebuggerNetPort = 352321563,
    BcdLibraryBoolean_DebuggerNetDhcp = 369098780,
    BcdLibraryString_DebuggerNetKey = 301989917,
    BcdLibraryBoolean_DebuggerNetVM = 369098782,
    BcdLibraryString_DebuggerNetHostIpv6 = 301989919,
    BcdLibraryBoolean_EmsEnabled = 369098784,
    BcdLibraryInteger_EmsPort = 352321570,
    BcdLibraryInteger_EmsBaudRate = 352321571,
    BcdLibraryString_LoadOptionsString = 301989936,
    BcdLibraryBoolean_AttemptNonBcdStart = 369098801,
    BcdLibraryBoolean_DisplayAdvancedOptions = 369098816,
    BcdLibraryBoolean_DisplayOptionsEdit = 369098817,
    BcdLibraryInteger_FVEKeyRingAddress = 352321602,
    BcdLibraryDevice_BsdLogDevice = 285212739,
    BcdLibraryString_BsdLogPath = 301989956,
    BcdLibraryBoolean_BsdPreserveLog = 369098821,
    BcdLibraryBoolean_GraphicsModeDisabled = 369098822,
    BcdLibraryInteger_ConfigAccessPolicy = 352321607,
    BcdLibraryBoolean_DisableIntegrityChecks = 369098824,
    BcdLibraryBoolean_AllowPrereleaseSignatures = 369098825,
    BcdLibraryString_FontPath = 301989962,
    BcdLibraryInteger_SiPolicy = 352321611,
    BcdLibraryInteger_FveBandId = 352321612,
    BcdLibraryBoolean_ConsoleExtendedInput = 369098832,
    BcdLibraryInteger_InitialConsoleInput = 352321617,
    BcdLibraryInteger_GraphicsResolution = 352321618,
    BcdLibraryBoolean_RestartOnFailure = 369098835,
    BcdLibraryBoolean_GraphicsForceHighestMode = 369098836,
    BcdLibraryBoolean_IsolatedExecutionContext = 369098848,
    BcdLibraryInteger_BootUxDisplayMessage = 352321637,
    BcdLibraryInteger_BootUxDisplayMessageOverride = 352321638,
    BcdLibraryBoolean_BootUxLogoDisable = 369098855,
    BcdLibraryBoolean_BootUxTextDisable = 369098856,
    BcdLibraryBoolean_BootUxProgressDisable = 369098857,
    BcdLibraryBoolean_BootUxFadeDisable = 369098858,
    BcdLibraryBoolean_BootUxReservePoolDebug = 369098859,
    BcdLibraryBoolean_BootUxDisable = 369098860,
    BcdLibraryInteger_BootUxFadeFrames = 352321645,
    BcdLibraryBoolean_BootUxDumpStats = 369098862,
    BcdLibraryBoolean_BootUxShowStats = 369098863,
    BcdLibraryBoolean_MultiBootSystem = 369098865,
    BcdLibraryBoolean_ForceNoKeyboard = 369098866,
    BcdLibraryInteger_AliasWindowsKey = 352321651,
    BcdLibraryBoolean_BootShutdownDisabled = 369098868,
    BcdLibraryInteger_PerformanceFrequency = 352321653,
    BcdLibraryInteger_SecurebootRawPolicy = 352321654,
    BcdLibraryIntegerList_AllowedInMemorySettings = 352321655,
    BcdLibraryInteger_BootUxBitmapTransitionTime = 352321657,
    BcdLibraryBoolean_TwoBootImages = 369098874,
    BcdLibraryBoolean_ForceFipsCrypto = 369098875,
    BcdLibraryInteger_BootErrorUx = 352321661,
    BcdLibraryBoolean_AllowFlightSignatures = 369098878,
    BcdLibraryInteger_BootMeasurementLogFormat = 352321663,
    BcdLibraryInteger_DisplayRotation = 352321664,
    BcdLibraryInteger_LogControl = 352321665,
    BcdLibraryBoolean_NoFirmwareSync = 369098882,
    BcdLibraryDevice_WindowsSystemDevice = 285212804,
    BcdLibraryBoolean_NumLockOn = 369098887,
    BcdLibraryString_AdditionalCiPolicy = 301990024,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum BcdTemplateElementTypes {
    BcdSetupInteger_DeviceType = 1157627905,
    BcdSetupString_ApplicationRelativePath = 1107296258,
    BcdSetupString_RamdiskDeviceRelativePath = 1107296259,
    BcdSetupBoolean_OmitOsLoaderElements = 1174405124,
    BcdSetupIntegerList_ElementsToMigrateList = 1191182342,
    BcdSetupBoolean_RecoveryOs = 1174405136,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum BcdOSLoader_NxPolicy {
    NxPolicyOptIn = 0,
    NxPolicyOptOut = 1,
    NxPolicyAlwaysOff = 2,
    NxPolicyAlwaysOn = 3,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum BcdOSLoader_PAEPolicy {
    PaePolicyDefault = 0,
    PaePolicyForceEnable = 1,
    PaePolicyForceDisable = 2,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum BcdOSLoader_BootStatusPolicy {
    BootStatusPolicyDisplayAllFailures = 0,
    BootStatusPolicyIgnoreAllFailures = 1,
    BootStatusPolicyIgnoreShutdownFailures = 2,
    BootStatusPolicyIgnoreBootFailures = 3,
    BootStatusPolicyIgnoreCheckpointFailures = 4,
    BootStatusPolicyDisplayShutdownFailures = 5,
    BootStatusPolicyDisplayBootFailures = 6,
    BootStatusPolicyDisplayCheckpointFailures = 7,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum BcdOSLoaderElementTypes {
    BcdOSLoaderDevice_OSDevice = 553648129,
    BcdOSLoaderString_SystemRoot = 570425346,
    BcdOSLoaderObject_AssociatedResumeObject = 587202563,
    BcdOSLoaderBoolean_StampDisks = 637534212,
    BcdOSLoaderBoolean_DetectKernelAndHal = 637534224,
    BcdOSLoaderString_KernelPath = 570425361,
    BcdOSLoaderString_HalPath = 570425362,
    BcdOSLoaderString_DbgTransportPath = 570425363,
    BcdOSLoaderInteger_NxPolicy = 620757024,
    BcdOSLoaderInteger_PAEPolicy = 620757025,
    BcdOSLoaderBoolean_WinPEMode = 637534242,
    BcdOSLoaderBoolean_DisableCrashAutoReboot = 637534244,
    BcdOSLoaderBoolean_UseLastGoodSettings = 637534245,
    BcdOSLoaderBoolean_DisableCodeIntegrityChecks = 637534246,
    BcdOSLoaderBoolean_AllowPrereleaseSignatures = 637534247,
    BcdOSLoaderBoolean_NoLowMemory = 637534256,
    BcdOSLoaderInteger_RemoveMemory = 620757041,
    BcdOSLoaderInteger_IncreaseUserVa = 620757042,
    BcdOSLoaderInteger_PerformaceDataMemory = 620757043,
    BcdOSLoaderBoolean_UseVgaDriver = 637534272,
    BcdOSLoaderBoolean_DisableBootDisplay = 637534273,
    BcdOSLoaderBoolean_DisableVesaBios = 637534274,
    BcdOSLoaderBoolean_DisableVgaMode = 637534275,
    BcdOSLoaderInteger_ClusterModeAddressing = 620757072,
    BcdOSLoaderBoolean_UsePhysicalDestination = 637534289,
    BcdOSLoaderInteger_RestrictApicCluster = 620757074,
    BcdOSLoaderString_OSLoaderTypeEVStore = 570425427,
    BcdOSLoaderBoolean_UseLegacyApicMode = 637534292,
    BcdOSLoaderInteger_X2ApicPolicy = 620757077,
    BcdOSLoaderBoolean_UseBootProcessorOnly = 637534304,
    BcdOSLoaderInteger_NumberOfProcessors = 620757089,
    BcdOSLoaderBoolean_ForceMaximumProcessors = 637534306,
    BcdOSLoaderBoolean_ProcessorConfigurationFlags = 620757091,
    BcdOSLoaderBoolean_MaximizeGroupsCreated = 637534308,
    BcdOSLoaderBoolean_ForceGroupAwareness = 637534309,
    BcdOSLoaderInteger_GroupSize = 620757094,
    BcdOSLoaderInteger_UseFirmwarePciSettings = 637534320,
    BcdOSLoaderInteger_MsiPolicy = 620757105,
    BcdOSLoaderInteger_PciExpressPolicy = 620757106,
    BcdOSLoaderInteger_SafeBoot = 620757120,
    BcdOSLoaderBoolean_SafeBootAlternateShell = 637534337,
    BcdOSLoaderBoolean_BootLogInitialization = 637534352,
    BcdOSLoaderBoolean_VerboseObjectLoadMode = 637534353,
    BcdOSLoaderBoolean_KernelDebuggerEnabled = 637534368,
    BcdOSLoaderBoolean_DebuggerHalBreakpoint = 637534369,
    BcdOSLoaderBoolean_UsePlatformClock = 637534370,
    BcdOSLoaderBoolean_ForceLegacyPlatform = 637534371,
    BcdOSLoaderBoolean_UsePlatformTick = 637534372,
    BcdOSLoaderBoolean_DisableDynamicTick = 637534373,
    BcdOSLoaderInteger_TscSyncPolicy = 620757158,
    BcdOSLoaderBoolean_EmsEnabled = 637534384,
    BcdOSLoaderInteger_ForceFailure = 620757184,
    BcdOSLoaderInteger_DriverLoadFailurePolicy = 620757185,
    BcdOSLoaderInteger_BootMenuPolicy = 620757186,
    BcdOSLoaderBoolean_AdvancedOptionsOneTime = 637534403,
    BcdOSLoaderBoolean_OptionsEditOneTime = 637534404,
    BcdOSLoaderInteger_BootStatusPolicy = 620757216,
    BcdOSLoaderBoolean_DisableElamDrivers = 637534433,
    BcdOSLoaderInteger_HypervisorLaunchType = 620757232,
    BcdOSLoaderString_HypervisorPath = 620757233,
    BcdOSLoaderBoolean_HypervisorDebuggerEnabled = 637534450,
    BcdOSLoaderInteger_HypervisorDebuggerType = 620757235,
    BcdOSLoaderInteger_HypervisorDebuggerPortNumber = 620757236,
    BcdOSLoaderInteger_HypervisorDebuggerBaudrate = 620757237,
    BcdOSLoaderInteger_HypervisorDebugger1394Channel = 620757238,
    BcdOSLoaderInteger_BootUxPolicy = 620757239,
    BcdOSLoaderInteger_HypervisorSlatDisabled = 570425592,
    BcdOSLoaderString_HypervisorDebuggerBusParams = 570425593,
    BcdOSLoaderInteger_HypervisorNumProc = 620757242,
    BcdOSLoaderInteger_HypervisorRootProcPerNode = 620757243,
    BcdOSLoaderBoolean_HypervisorUseLargeVTlb = 637534460,
    BcdOSLoaderInteger_HypervisorDebuggerNetHostIp = 620757245,
    BcdOSLoaderInteger_HypervisorDebuggerNetHostPort = 620757246,
    BcdOSLoaderInteger_HypervisorDebuggerPages = 620757247,
    BcdOSLoaderInteger_TpmBootEntropyPolicy = 620757248,
    BcdOSLoaderString_HypervisorDebuggerNetKey = 570425616,
    BcdOSLoaderString_HypervisorProductSkuType = 570425618,
    BcdOSLoaderInteger_HypervisorRootProc = 570425619,
    BcdOSLoaderBoolean_HypervisorDebuggerNetDhcp = 637534484,
    BcdOSLoaderInteger_HypervisorIommuPolicy = 620757269,
    BcdOSLoaderBoolean_HypervisorUseVApic = 637534486,
    BcdOSLoaderString_HypervisorLoadOptions = 570425623,
    BcdOSLoaderInteger_HypervisorMsrFilterPolicy = 620757272,
    BcdOSLoaderInteger_HypervisorMmioNxPolicy = 620757273,
    BcdOSLoaderInteger_HypervisorSchedulerType = 620757274,
    BcdOSLoaderString_HypervisorRootProcNumaNodes = 570425627,
    BcdOSLoaderInteger_HypervisorPerfmon = 620757276,
    BcdOSLoaderInteger_HypervisorRootProcPerCore = 620757277,
    BcdOSLoaderString_HypervisorRootProcNumaNodeLps = 570425630,
    BcdOSLoaderInteger_XSavePolicy = 620757280,
    BcdOSLoaderInteger_XSaveAddFeature0 = 620757281,
    BcdOSLoaderInteger_XSaveAddFeature1 = 620757282,
    BcdOSLoaderInteger_XSaveAddFeature2 = 620757283,
    BcdOSLoaderInteger_XSaveAddFeature3 = 620757284,
    BcdOSLoaderInteger_XSaveAddFeature4 = 620757285,
    BcdOSLoaderInteger_XSaveAddFeature5 = 620757286,
    BcdOSLoaderInteger_XSaveAddFeature6 = 620757287,
    BcdOSLoaderInteger_XSaveAddFeature7 = 620757288,
    BcdOSLoaderInteger_XSaveRemoveFeature = 620757289,
    BcdOSLoaderInteger_XSaveProcessorsMask = 620757290,
    BcdOSLoaderInteger_XSaveDisable = 620757291,
    BcdOSLoaderInteger_KernelDebuggerType = 620757292,
    BcdOSLoaderString_KernelDebuggerBusParameters = 570425645,
    BcdOSLoaderInteger_KernelDebuggerPortAddress = 620757294,
    BcdOSLoaderInteger_KernelDebuggerPortNumber = 620757295,
    BcdOSLoaderInteger_ClaimedTpmCounter = 620757296,
    BcdOSLoaderInteger_KernelDebugger1394Channel = 620757297,
    BcdOSLoaderString_KernelDebuggerUsbTargetname = 570425650,
    BcdOSLoaderInteger_KernelDebuggerNetHostIp = 620757299,
    BcdOSLoaderInteger_KernelDebuggerNetHostPort = 620757300,
    BcdOSLoaderBoolean_KernelDebuggerNetDhcp = 637534517,
    BcdOSLoaderString_KernelDebuggerNetKey = 570425654,
    BcdOSLoaderString_IMCHiveName = 570425655,
    BcdOSLoaderDevice_IMCDevice = 553648440,
    BcdOSLoaderInteger_KernelDebuggerBaudrate = 620757305,
    BcdOSLoaderString_ManufacturingMode = 570425664,
    BcdOSLoaderBoolean_EventLoggingEnabled = 637534529,
    BcdOSLoaderInteger_VsmLaunchType = 620757314,
    BcdOSLoaderInteger_HypervisorEnforcedCodeIntegrity = 620757316,
    BcdOSLoaderBoolean_DtraceEnabled = 637534533,
    BcdOSLoaderDevice_SystemDataDevice = 553648464,
    BcdOSLoaderDevice_OsArcDevice = 553648465,
    BcdOSLoaderDevice_OsDataDevice = 553648467,
    BcdOSLoaderDevice_BspDevice = 553648468,
    BcdOSLoaderDevice_BspFilepath = 553648469,
    BcdOSLoaderString_KernelDebuggerNetHostIpv6 = 570425686,
    BcdOSLoaderString_HypervisorDebuggerNetHostIpv6 = 570425697,
}
