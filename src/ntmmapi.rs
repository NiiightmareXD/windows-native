use windows::{
    Wdk::{Foundation::OBJECT_ATTRIBUTES, System::Memory::SECTION_INHERIT},
    Win32::{
        Foundation::{HANDLE, NTSTATUS, UNICODE_STRING},
        System::{
            Memory::{CFG_CALL_TARGET_INFO, MEM_EXTENDED_PARAMETER},
            IO::IO_STATUS_BLOCK,
        },
    },
};

use crate::{
    bitfield::{BitfieldUnit, UnionField},
    phnt_ntdef::PENCLAVE_ROUTINE,
};

pub const MEM_DOS_LIM: u32 = 1073741824;
pub const SEC_BASED: u32 = 2097152;
pub const SEC_NO_CHANGE: u32 = 4194304;
pub const SEC_GLOBAL: u32 = 536870912;
pub const MMPFNLIST_ZERO: u32 = 0;
pub const MMPFNLIST_FREE: u32 = 1;
pub const MMPFNLIST_STANDBY: u32 = 2;
pub const MMPFNLIST_MODIFIED: u32 = 3;
pub const MMPFNLIST_MODIFIEDNOWRITE: u32 = 4;
pub const MMPFNLIST_BAD: u32 = 5;
pub const MMPFNLIST_ACTIVE: u32 = 6;
pub const MMPFNLIST_TRANSITION: u32 = 7;
pub const MMPFNUSE_PROCESSPRIVATE: u32 = 0;
pub const MMPFNUSE_FILE: u32 = 1;
pub const MMPFNUSE_PAGEFILEMAPPED: u32 = 2;
pub const MMPFNUSE_PAGETABLE: u32 = 3;
pub const MMPFNUSE_PAGEDPOOL: u32 = 4;
pub const MMPFNUSE_NONPAGEDPOOL: u32 = 5;
pub const MMPFNUSE_SYSTEMPTE: u32 = 6;
pub const MMPFNUSE_SESSIONPRIVATE: u32 = 7;
pub const MMPFNUSE_METAFILE: u32 = 8;
pub const MMPFNUSE_AWEPAGE: u32 = 9;
pub const MMPFNUSE_DRIVERLOCKPAGE: u32 = 10;
pub const MMPFNUSE_KERNELSTACK: u32 = 11;
pub const MEM_EXECUTE_OPTION_ENABLE: u32 = 1;
pub const MEM_EXECUTE_OPTION_DISABLE: u32 = 2;
pub const MEM_EXECUTE_OPTION_DISABLE_THUNK_EMULATION: u32 = 4;
pub const MEM_EXECUTE_OPTION_PERMANENT: u32 = 8;
pub const MEM_EXECUTE_OPTION_EXECUTE_DISPATCH_ENABLE: u32 = 16;
pub const MEM_EXECUTE_OPTION_IMAGE_DISPATCH_ENABLE: u32 = 32;
pub const MEM_EXECUTE_OPTION_VALID_FLAGS: u32 = 63;
pub const MEMORY_PARTITION_ALL_ACCESS: u32 = 2031619;
pub const MAP_PROCESS: u32 = 1;
pub const MAP_SYSTEM: u32 = 2;
pub const MemoryWorkingSetInformation: u32 = 1;
pub const MemoryMappedFilenameInformation: u32 = 2;
pub const MemoryRegionInformation: u32 = 3;
pub const MemoryWorkingSetExInformation: u32 = 4;
pub const MemorySharedCommitInformation: u32 = 5;
pub const MemoryImageInformation: u32 = 6;
pub const MemoryRegionInformationEx: u32 = 7;
pub const MemoryPrivilegedBasicInformation: u32 = 8;
pub const MemoryEnclaveImageInformation: u32 = 9;
pub const MemoryBasicInformationCapped: u32 = 10;
pub const MemoryPhysicalContiguityInformation: u32 = 11;
pub const MemoryBadInformation: u32 = 12;
pub const MemoryBadInformationAllProcesses: u32 = 13;
pub const SystemMemoryPartitionMoveMemory: u32 = 1;
pub const SystemMemoryPartitionAddPagefile: u32 = 2;
pub const SystemMemoryPartitionCombineMemory: u32 = 3;
pub const SystemMemoryPartitionInitialAddMemory: u32 = 4;
pub const SystemMemoryPartitionGetMemoryEvents: u32 = 5;
pub const SystemMemoryPartitionSetAttributes: u32 = 6;
pub const SystemMemoryPartitionNodeInformation: u32 = 7;
pub const SystemMemoryPartitionCreateLargePages: u32 = 8;
pub const SystemMemoryPartitionMemoryChargeAttributes: u32 = 11;
pub const SystemMemoryPartitionClearAttributes: u32 = 12;
pub const SystemMemoryPartitionSetMemoryThresholds: u32 = 13;
pub const SystemMemoryPartitionMax: u32 = 14;
pub const PROC_THREAD_ATTRIBUTE_EXTENDED_FLAGS: u32 = 393217;
pub const PROC_THREAD_ATTRIBUTE_PACKAGE_FULL_NAME: u32 = 131080;
pub const PROC_THREAD_ATTRIBUTE_CONSOLE_REFERENCE: u32 = 131082;
pub const PROC_THREAD_ATTRIBUTE_OSMAXVERSIONTESTED: u32 = 131084;
pub const PROC_THREAD_ATTRIBUTE_SAFE_OPEN_PROMPT_ORIGIN_CLAIM: u32 = 131089;
pub const PROC_THREAD_ATTRIBUTE_BNO_ISOLATION: u32 = 131091;
pub const PROC_THREAD_ATTRIBUTE_ISOLATION_MANIFEST: u32 = 131095;
pub const PROC_THREAD_ATTRIBUTE_CREATE_STORE: u32 = 131100;
pub const MEM_64K_PAGES: u32 = 541065216;

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MEMORY_INFORMATION_CLASS {
    MemoryBasicInformation = 0,
    MemoryWorkingSetInformation = 1,
    MemoryMappedFilenameInformation = 2,
    MemoryRegionInformation = 3,
    MemoryWorkingSetExInformation = 4,
    MemorySharedCommitInformation = 5,
    MemoryImageInformation = 6,
    MemoryRegionInformationEx = 7,
    MemoryPrivilegedBasicInformation = 8,
    MemoryEnclaveImageInformation = 9,
    MemoryBasicInformationCapped = 10,
    MemoryPhysicalContiguityInformation = 11,
    MemoryBadInformation = 12,
    MemoryBadInformationAllProcesses = 13,
    MaxMemoryInfoClass = 14,
}

#[repr(C)]
#[repr(align(8))]
pub struct MEMORY_WORKING_SET_BLOCK {
    _bitfield_align_1: [u64; 0],
    _bitfield_1: BitfieldUnit<[u8; 8]>,
}

impl Default for MEMORY_WORKING_SET_BLOCK {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for MEMORY_WORKING_SET_BLOCK {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MEMORY_WORKING_SET_BLOCK {{ Protection : {:?}, ShareCount : {:?}, Shared : {:?}, Node : {:?}, VirtualPage : {:?} }}",
            self.Protection(),
            self.ShareCount(),
            self.Shared(),
            self.Node(),
            self.VirtualPage()
        )
    }
}

impl MEMORY_WORKING_SET_BLOCK {
    #[inline]
    pub fn Protection(&self) -> usize {
        unsafe { std::mem::transmute(self._bitfield_1.get(0usize, 5u8)) }
    }

    #[inline]
    pub fn set_Protection(&mut self, val: usize) {
        unsafe {
            let val: u64 = std::mem::transmute(val);

            self._bitfield_1.set(0usize, 5u8, val as u64)
        }
    }

    #[inline]
    pub fn ShareCount(&self) -> usize {
        unsafe { std::mem::transmute(self._bitfield_1.get(5usize, 3u8)) }
    }

    #[inline]
    pub fn set_ShareCount(&mut self, val: usize) {
        unsafe {
            let val: u64 = std::mem::transmute(val);

            self._bitfield_1.set(5usize, 3u8, val as u64)
        }
    }

    #[inline]
    pub fn Shared(&self) -> usize {
        unsafe { std::mem::transmute(self._bitfield_1.get(8usize, 1u8)) }
    }

    #[inline]
    pub fn set_Shared(&mut self, val: usize) {
        unsafe {
            let val: u64 = std::mem::transmute(val);

            self._bitfield_1.set(8usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn Node(&self) -> usize {
        unsafe { std::mem::transmute(self._bitfield_1.get(9usize, 3u8)) }
    }

    #[inline]
    pub fn set_Node(&mut self, val: usize) {
        unsafe {
            let val: u64 = std::mem::transmute(val);

            self._bitfield_1.set(9usize, 3u8, val as u64)
        }
    }

    #[inline]
    pub fn VirtualPage(&self) -> usize {
        unsafe { std::mem::transmute(self._bitfield_1.get(12usize, 52u8)) }
    }

    #[inline]
    pub fn set_VirtualPage(&mut self, val: usize) {
        unsafe {
            let val: u64 = std::mem::transmute(val);

            self._bitfield_1.set(12usize, 52u8, val as u64)
        }
    }

    #[inline]
    pub fn new_bitfield_1(
        Protection: usize,
        ShareCount: usize,
        Shared: usize,
        Node: usize,
        VirtualPage: usize,
    ) -> BitfieldUnit<[u8; 8]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 8]> = Default::default();

        bitfield_unit.set(0usize, 5u8, {
            let Protection: u64 = unsafe { std::mem::transmute(Protection) };

            Protection as u64
        });

        bitfield_unit.set(5usize, 3u8, {
            let ShareCount: u64 = unsafe { std::mem::transmute(ShareCount) };

            ShareCount as u64
        });

        bitfield_unit.set(8usize, 1u8, {
            let Shared: u64 = unsafe { std::mem::transmute(Shared) };

            Shared as u64
        });

        bitfield_unit.set(9usize, 3u8, {
            let Node: u64 = unsafe { std::mem::transmute(Node) };

            Node as u64
        });

        bitfield_unit.set(12usize, 52u8, {
            let VirtualPage: u64 = unsafe { std::mem::transmute(VirtualPage) };

            VirtualPage as u64
        });

        bitfield_unit
    }
}

#[repr(C)]
pub struct MEMORY_WORKING_SET_INFORMATION {
    pub NumberOfEntries: usize,
    pub WorkingSetInfo: [MEMORY_WORKING_SET_BLOCK; 1],
}

impl Default for MEMORY_WORKING_SET_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for MEMORY_WORKING_SET_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MEMORY_WORKING_SET_INFORMATION {{ WorkingSetInfo: {:?} }}",
            self.WorkingSetInfo
        )
    }
}

#[repr(C)]
pub struct MEMORY_REGION_INFORMATION {
    pub AllocationBase: *mut std::ffi::c_void,
    pub AllocationProtect: u32,
    pub Anonymous1: MEMORY_REGION_INFORMATION_1,
    pub RegionSize: usize,
    pub CommitSize: usize,
    pub PartitionId: usize,
    pub NodePreference: usize,
}

#[repr(C)]
pub struct MEMORY_REGION_INFORMATION_1 {
    pub RegionType: UnionField<u32>,
    pub Anonymous1: UnionField<MEMORY_REGION_INFORMATION_1_1>,
    pub union_field: u32,
}

#[repr(C)]
#[repr(align(4))]
pub struct MEMORY_REGION_INFORMATION_1_1 {
    _bitfield_align_1: [u32; 0],
    _bitfield_1: BitfieldUnit<[u8; 4]>,
}

impl Default for MEMORY_REGION_INFORMATION_1_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for MEMORY_REGION_INFORMATION_1_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MEMORY_REGION_INFORMATION_1_1 {{ Private : {:?}, MappedDataFile : {:?}, MappedImage : {:?}, MappedPageFile : {:?}, MappedPhysical : {:?}, DirectMapped : {:?}, SoftwareEnclave : {:?}, PageSize64K : {:?}, PlaceholderReservation : {:?}, MappedAwe : {:?}, MappedWriteWatch : {:?}, PageSizeLarge : {:?}, PageSizeHuge : {:?}, Reserved : {:?} }}",
            self.Private(),
            self.MappedDataFile(),
            self.MappedImage(),
            self.MappedPageFile(),
            self.MappedPhysical(),
            self.DirectMapped(),
            self.SoftwareEnclave(),
            self.PageSize64K(),
            self.PlaceholderReservation(),
            self.MappedAwe(),
            self.MappedWriteWatch(),
            self.PageSizeLarge(),
            self.PageSizeHuge(),
            self.Reserved()
        )
    }
}

impl MEMORY_REGION_INFORMATION_1_1 {
    #[inline]
    pub fn Private(&self) -> u32 {
        self._bitfield_1.get(0usize, 1u8) as u32
    }

    #[inline]
    pub fn set_Private(&mut self, val: u32) {
        self._bitfield_1.set(0usize, 1u8, val as u64)
    }

    #[inline]
    pub fn MappedDataFile(&self) -> u32 {
        self._bitfield_1.get(1usize, 1u8) as u32
    }

    #[inline]
    pub fn set_MappedDataFile(&mut self, val: u32) {
        self._bitfield_1.set(1usize, 1u8, val as u64)
    }

    #[inline]
    pub fn MappedImage(&self) -> u32 {
        self._bitfield_1.get(2usize, 1u8) as u32
    }

    #[inline]
    pub fn set_MappedImage(&mut self, val: u32) {
        self._bitfield_1.set(2usize, 1u8, val as u64)
    }

    #[inline]
    pub fn MappedPageFile(&self) -> u32 {
        self._bitfield_1.get(3usize, 1u8) as u32
    }

    #[inline]
    pub fn set_MappedPageFile(&mut self, val: u32) {
        self._bitfield_1.set(3usize, 1u8, val as u64)
    }

    #[inline]
    pub fn MappedPhysical(&self) -> u32 {
        self._bitfield_1.get(4usize, 1u8) as u32
    }

    #[inline]
    pub fn set_MappedPhysical(&mut self, val: u32) {
        self._bitfield_1.set(4usize, 1u8, val as u64)
    }

    #[inline]
    pub fn DirectMapped(&self) -> u32 {
        self._bitfield_1.get(5usize, 1u8) as u32
    }

    #[inline]
    pub fn set_DirectMapped(&mut self, val: u32) {
        self._bitfield_1.set(5usize, 1u8, val as u64)
    }

    #[inline]
    pub fn SoftwareEnclave(&self) -> u32 {
        self._bitfield_1.get(6usize, 1u8) as u32
    }

    #[inline]
    pub fn set_SoftwareEnclave(&mut self, val: u32) {
        self._bitfield_1.set(6usize, 1u8, val as u64)
    }

    #[inline]
    pub fn PageSize64K(&self) -> u32 {
        self._bitfield_1.get(7usize, 1u8) as u32
    }

    #[inline]
    pub fn set_PageSize64K(&mut self, val: u32) {
        self._bitfield_1.set(7usize, 1u8, val as u64)
    }

    #[inline]
    pub fn PlaceholderReservation(&self) -> u32 {
        self._bitfield_1.get(8usize, 1u8) as u32
    }

    #[inline]
    pub fn set_PlaceholderReservation(&mut self, val: u32) {
        self._bitfield_1.set(8usize, 1u8, val as u64)
    }

    #[inline]
    pub fn MappedAwe(&self) -> u32 {
        self._bitfield_1.get(9usize, 1u8) as u32
    }

    #[inline]
    pub fn set_MappedAwe(&mut self, val: u32) {
        self._bitfield_1.set(9usize, 1u8, val as u64)
    }

    #[inline]
    pub fn MappedWriteWatch(&self) -> u32 {
        self._bitfield_1.get(10usize, 1u8) as u32
    }

    #[inline]
    pub fn set_MappedWriteWatch(&mut self, val: u32) {
        self._bitfield_1.set(10usize, 1u8, val as u64)
    }

    #[inline]
    pub fn PageSizeLarge(&self) -> u32 {
        self._bitfield_1.get(11usize, 1u8) as u32
    }

    #[inline]
    pub fn set_PageSizeLarge(&mut self, val: u32) {
        self._bitfield_1.set(11usize, 1u8, val as u64)
    }

    #[inline]
    pub fn PageSizeHuge(&self) -> u32 {
        self._bitfield_1.get(12usize, 1u8) as u32
    }

    #[inline]
    pub fn set_PageSizeHuge(&mut self, val: u32) {
        self._bitfield_1.set(12usize, 1u8, val as u64)
    }

    #[inline]
    pub fn Reserved(&self) -> u32 {
        self._bitfield_1.get(13usize, 19u8) as u32
    }

    #[inline]
    pub fn set_Reserved(&mut self, val: u32) {
        self._bitfield_1.set(13usize, 19u8, val as u64)
    }

    #[inline]
    pub fn new_bitfield_1(
        Private: u32,
        MappedDataFile: u32,
        MappedImage: u32,
        MappedPageFile: u32,
        MappedPhysical: u32,
        DirectMapped: u32,
        SoftwareEnclave: u32,
        PageSize64K: u32,
        PlaceholderReservation: u32,
        MappedAwe: u32,
        MappedWriteWatch: u32,
        PageSizeLarge: u32,
        PageSizeHuge: u32,
        Reserved: u32,
    ) -> BitfieldUnit<[u8; 4]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 4]> = Default::default();

        bitfield_unit.set(0usize, 1u8, Private as u64);

        bitfield_unit.set(1usize, 1u8, MappedDataFile as u64);

        bitfield_unit.set(2usize, 1u8, MappedImage as u64);

        bitfield_unit.set(3usize, 1u8, MappedPageFile as u64);

        bitfield_unit.set(4usize, 1u8, MappedPhysical as u64);

        bitfield_unit.set(5usize, 1u8, DirectMapped as u64);

        bitfield_unit.set(6usize, 1u8, SoftwareEnclave as u64);

        bitfield_unit.set(7usize, 1u8, PageSize64K as u64);

        bitfield_unit.set(8usize, 1u8, PlaceholderReservation as u64);

        bitfield_unit.set(9usize, 1u8, MappedAwe as u64);

        bitfield_unit.set(10usize, 1u8, MappedWriteWatch as u64);

        bitfield_unit.set(11usize, 1u8, PageSizeLarge as u64);

        bitfield_unit.set(12usize, 1u8, PageSizeHuge as u64);

        bitfield_unit.set(13usize, 19u8, Reserved as u64);

        bitfield_unit
    }
}

impl Default for MEMORY_REGION_INFORMATION_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for MEMORY_REGION_INFORMATION_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MEMORY_REGION_INFORMATION_1 {{ union }}")
    }
}

impl Default for MEMORY_REGION_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for MEMORY_REGION_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MEMORY_REGION_INFORMATION {{ Anonymous1: {:?} }}",
            self.Anonymous1
        )
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MEMORY_WORKING_SET_EX_LOCATION {
    MemoryLocationInvalid = 0,
    MemoryLocationResident = 1,
    MemoryLocationPagefile = 2,
    MemoryLocationReserved = 3,
}

#[repr(C)]
pub struct MEMORY_WORKING_SET_EX_BLOCK {
    pub Anonymous1: MEMORY_WORKING_SET_EX_BLOCK_1,
}

#[repr(C)]
pub struct MEMORY_WORKING_SET_EX_BLOCK_1 {
    pub Anonymous1: UnionField<MEMORY_WORKING_SET_EX_BLOCK_1_1>,
    pub Invalid: UnionField<MEMORY_WORKING_SET_EX_BLOCK_1_2>,
    pub union_field: u64,
}

#[repr(C)]
#[repr(align(8))]
pub struct MEMORY_WORKING_SET_EX_BLOCK_1_1 {
    _bitfield_align_1: [u32; 0],
    _bitfield_1: BitfieldUnit<[u8; 8]>,
}

impl Default for MEMORY_WORKING_SET_EX_BLOCK_1_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for MEMORY_WORKING_SET_EX_BLOCK_1_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MEMORY_WORKING_SET_EX_BLOCK_1_1 {{ Valid : {:?}, ShareCount : {:?}, Win32Protection : {:?}, Shared : {:?}, Node : {:?}, Locked : {:?}, LargePage : {:?}, Priority : {:?}, Reserved : {:?}, SharedOriginal : {:?}, Bad : {:?}, Win32GraphicsProtection : {:?}, ReservedUlong : {:?} }}",
            self.Valid(),
            self.ShareCount(),
            self.Win32Protection(),
            self.Shared(),
            self.Node(),
            self.Locked(),
            self.LargePage(),
            self.Priority(),
            self.Reserved(),
            self.SharedOriginal(),
            self.Bad(),
            self.Win32GraphicsProtection(),
            self.ReservedUlong()
        )
    }
}

impl MEMORY_WORKING_SET_EX_BLOCK_1_1 {
    #[inline]
    pub fn Valid(&self) -> usize {
        unsafe { std::mem::transmute(self._bitfield_1.get(0usize, 1u8)) }
    }

    #[inline]
    pub fn set_Valid(&mut self, val: usize) {
        unsafe {
            let val: u64 = std::mem::transmute(val);

            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn ShareCount(&self) -> usize {
        unsafe { std::mem::transmute(self._bitfield_1.get(1usize, 3u8)) }
    }

    #[inline]
    pub fn set_ShareCount(&mut self, val: usize) {
        unsafe {
            let val: u64 = std::mem::transmute(val);

            self._bitfield_1.set(1usize, 3u8, val as u64)
        }
    }

    #[inline]
    pub fn Win32Protection(&self) -> usize {
        unsafe { std::mem::transmute(self._bitfield_1.get(4usize, 11u8)) }
    }

    #[inline]
    pub fn set_Win32Protection(&mut self, val: usize) {
        unsafe {
            let val: u64 = std::mem::transmute(val);

            self._bitfield_1.set(4usize, 11u8, val as u64)
        }
    }

    #[inline]
    pub fn Shared(&self) -> usize {
        unsafe { std::mem::transmute(self._bitfield_1.get(15usize, 1u8)) }
    }

    #[inline]
    pub fn set_Shared(&mut self, val: usize) {
        unsafe {
            let val: u64 = std::mem::transmute(val);

            self._bitfield_1.set(15usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn Node(&self) -> usize {
        unsafe { std::mem::transmute(self._bitfield_1.get(16usize, 6u8)) }
    }

    #[inline]
    pub fn set_Node(&mut self, val: usize) {
        unsafe {
            let val: u64 = std::mem::transmute(val);

            self._bitfield_1.set(16usize, 6u8, val as u64)
        }
    }

    #[inline]
    pub fn Locked(&self) -> usize {
        unsafe { std::mem::transmute(self._bitfield_1.get(22usize, 1u8)) }
    }

    #[inline]
    pub fn set_Locked(&mut self, val: usize) {
        unsafe {
            let val: u64 = std::mem::transmute(val);

            self._bitfield_1.set(22usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn LargePage(&self) -> usize {
        unsafe { std::mem::transmute(self._bitfield_1.get(23usize, 1u8)) }
    }

    #[inline]
    pub fn set_LargePage(&mut self, val: usize) {
        unsafe {
            let val: u64 = std::mem::transmute(val);

            self._bitfield_1.set(23usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn Priority(&self) -> usize {
        unsafe { std::mem::transmute(self._bitfield_1.get(24usize, 3u8)) }
    }

    #[inline]
    pub fn set_Priority(&mut self, val: usize) {
        unsafe {
            let val: u64 = std::mem::transmute(val);

            self._bitfield_1.set(24usize, 3u8, val as u64)
        }
    }

    #[inline]
    pub fn Reserved(&self) -> usize {
        unsafe { std::mem::transmute(self._bitfield_1.get(27usize, 3u8)) }
    }

    #[inline]
    pub fn set_Reserved(&mut self, val: usize) {
        unsafe {
            let val: u64 = std::mem::transmute(val);

            self._bitfield_1.set(27usize, 3u8, val as u64)
        }
    }

    #[inline]
    pub fn SharedOriginal(&self) -> usize {
        unsafe { std::mem::transmute(self._bitfield_1.get(30usize, 1u8)) }
    }

    #[inline]
    pub fn set_SharedOriginal(&mut self, val: usize) {
        unsafe {
            let val: u64 = std::mem::transmute(val);

            self._bitfield_1.set(30usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn Bad(&self) -> usize {
        unsafe { std::mem::transmute(self._bitfield_1.get(31usize, 1u8)) }
    }

    #[inline]
    pub fn set_Bad(&mut self, val: usize) {
        unsafe {
            let val: u64 = std::mem::transmute(val);

            self._bitfield_1.set(31usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn Win32GraphicsProtection(&self) -> usize {
        unsafe { std::mem::transmute(self._bitfield_1.get(32usize, 4u8)) }
    }

    #[inline]
    pub fn set_Win32GraphicsProtection(&mut self, val: usize) {
        unsafe {
            let val: u64 = std::mem::transmute(val);

            self._bitfield_1.set(32usize, 4u8, val as u64)
        }
    }

    #[inline]
    pub fn ReservedUlong(&self) -> usize {
        unsafe { std::mem::transmute(self._bitfield_1.get(36usize, 28u8)) }
    }

    #[inline]
    pub fn set_ReservedUlong(&mut self, val: usize) {
        unsafe {
            let val: u64 = std::mem::transmute(val);

            self._bitfield_1.set(36usize, 28u8, val as u64)
        }
    }

    #[inline]
    pub fn new_bitfield_1(
        Valid: usize,
        ShareCount: usize,
        Win32Protection: usize,
        Shared: usize,
        Node: usize,
        Locked: usize,
        LargePage: usize,
        Priority: usize,
        Reserved: usize,
        SharedOriginal: usize,
        Bad: usize,
        Win32GraphicsProtection: usize,
        ReservedUlong: usize,
    ) -> BitfieldUnit<[u8; 8]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 8]> = Default::default();

        bitfield_unit.set(0usize, 1u8, {
            let Valid: u64 = unsafe { std::mem::transmute(Valid) };

            Valid as u64
        });

        bitfield_unit.set(1usize, 3u8, {
            let ShareCount: u64 = unsafe { std::mem::transmute(ShareCount) };

            ShareCount as u64
        });

        bitfield_unit.set(4usize, 11u8, {
            let Win32Protection: u64 = unsafe { std::mem::transmute(Win32Protection) };

            Win32Protection as u64
        });

        bitfield_unit.set(15usize, 1u8, {
            let Shared: u64 = unsafe { std::mem::transmute(Shared) };

            Shared as u64
        });

        bitfield_unit.set(16usize, 6u8, {
            let Node: u64 = unsafe { std::mem::transmute(Node) };

            Node as u64
        });

        bitfield_unit.set(22usize, 1u8, {
            let Locked: u64 = unsafe { std::mem::transmute(Locked) };

            Locked as u64
        });

        bitfield_unit.set(23usize, 1u8, {
            let LargePage: u64 = unsafe { std::mem::transmute(LargePage) };

            LargePage as u64
        });

        bitfield_unit.set(24usize, 3u8, {
            let Priority: u64 = unsafe { std::mem::transmute(Priority) };

            Priority as u64
        });

        bitfield_unit.set(27usize, 3u8, {
            let Reserved: u64 = unsafe { std::mem::transmute(Reserved) };

            Reserved as u64
        });

        bitfield_unit.set(30usize, 1u8, {
            let SharedOriginal: u64 = unsafe { std::mem::transmute(SharedOriginal) };

            SharedOriginal as u64
        });

        bitfield_unit.set(31usize, 1u8, {
            let Bad: u64 = unsafe { std::mem::transmute(Bad) };

            Bad as u64
        });

        bitfield_unit.set(32usize, 4u8, {
            let Win32GraphicsProtection: u64 =
                unsafe { std::mem::transmute(Win32GraphicsProtection) };

            Win32GraphicsProtection as u64
        });

        bitfield_unit.set(36usize, 28u8, {
            let ReservedUlong: u64 = unsafe { std::mem::transmute(ReservedUlong) };

            ReservedUlong as u64
        });

        bitfield_unit
    }
}

#[repr(C)]
#[repr(align(8))]
pub struct MEMORY_WORKING_SET_EX_BLOCK_1_2 {
    _bitfield_align_1: [u32; 0],
    _bitfield_1: BitfieldUnit<[u8; 8]>,
}

impl Default for MEMORY_WORKING_SET_EX_BLOCK_1_2 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for MEMORY_WORKING_SET_EX_BLOCK_1_2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MEMORY_WORKING_SET_EX_BLOCK_1_2 {{ Valid : {:?}, Reserved0 : {:?}, Shared : {:?}, Reserved1 : {:?}, PageTable : {:?}, Location : {:?}, Priority : {:?}, ModifiedList : {:?}, Reserved2 : {:?}, SharedOriginal : {:?}, Bad : {:?}, ReservedUlong : {:?} }}",
            self.Valid(),
            self.Reserved0(),
            self.Shared(),
            self.Reserved1(),
            self.PageTable(),
            self.Location(),
            self.Priority(),
            self.ModifiedList(),
            self.Reserved2(),
            self.SharedOriginal(),
            self.Bad(),
            self.ReservedUlong()
        )
    }
}

impl MEMORY_WORKING_SET_EX_BLOCK_1_2 {
    #[inline]
    pub fn Valid(&self) -> usize {
        unsafe { std::mem::transmute(self._bitfield_1.get(0usize, 1u8)) }
    }

    #[inline]
    pub fn set_Valid(&mut self, val: usize) {
        unsafe {
            let val: u64 = std::mem::transmute(val);

            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn Reserved0(&self) -> usize {
        unsafe { std::mem::transmute(self._bitfield_1.get(1usize, 14u8)) }
    }

    #[inline]
    pub fn set_Reserved0(&mut self, val: usize) {
        unsafe {
            let val: u64 = std::mem::transmute(val);

            self._bitfield_1.set(1usize, 14u8, val as u64)
        }
    }

    #[inline]
    pub fn Shared(&self) -> usize {
        unsafe { std::mem::transmute(self._bitfield_1.get(15usize, 1u8)) }
    }

    #[inline]
    pub fn set_Shared(&mut self, val: usize) {
        unsafe {
            let val: u64 = std::mem::transmute(val);

            self._bitfield_1.set(15usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn Reserved1(&self) -> usize {
        unsafe { std::mem::transmute(self._bitfield_1.get(16usize, 5u8)) }
    }

    #[inline]
    pub fn set_Reserved1(&mut self, val: usize) {
        unsafe {
            let val: u64 = std::mem::transmute(val);

            self._bitfield_1.set(16usize, 5u8, val as u64)
        }
    }

    #[inline]
    pub fn PageTable(&self) -> usize {
        unsafe { std::mem::transmute(self._bitfield_1.get(21usize, 1u8)) }
    }

    #[inline]
    pub fn set_PageTable(&mut self, val: usize) {
        unsafe {
            let val: u64 = std::mem::transmute(val);

            self._bitfield_1.set(21usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn Location(&self) -> usize {
        unsafe { std::mem::transmute(self._bitfield_1.get(22usize, 2u8)) }
    }

    #[inline]
    pub fn set_Location(&mut self, val: usize) {
        unsafe {
            let val: u64 = std::mem::transmute(val);

            self._bitfield_1.set(22usize, 2u8, val as u64)
        }
    }

    #[inline]
    pub fn Priority(&self) -> usize {
        unsafe { std::mem::transmute(self._bitfield_1.get(24usize, 3u8)) }
    }

    #[inline]
    pub fn set_Priority(&mut self, val: usize) {
        unsafe {
            let val: u64 = std::mem::transmute(val);

            self._bitfield_1.set(24usize, 3u8, val as u64)
        }
    }

    #[inline]
    pub fn ModifiedList(&self) -> usize {
        unsafe { std::mem::transmute(self._bitfield_1.get(27usize, 1u8)) }
    }

    #[inline]
    pub fn set_ModifiedList(&mut self, val: usize) {
        unsafe {
            let val: u64 = std::mem::transmute(val);

            self._bitfield_1.set(27usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn Reserved2(&self) -> usize {
        unsafe { std::mem::transmute(self._bitfield_1.get(28usize, 2u8)) }
    }

    #[inline]
    pub fn set_Reserved2(&mut self, val: usize) {
        unsafe {
            let val: u64 = std::mem::transmute(val);

            self._bitfield_1.set(28usize, 2u8, val as u64)
        }
    }

    #[inline]
    pub fn SharedOriginal(&self) -> usize {
        unsafe { std::mem::transmute(self._bitfield_1.get(30usize, 1u8)) }
    }

    #[inline]
    pub fn set_SharedOriginal(&mut self, val: usize) {
        unsafe {
            let val: u64 = std::mem::transmute(val);

            self._bitfield_1.set(30usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn Bad(&self) -> usize {
        unsafe { std::mem::transmute(self._bitfield_1.get(31usize, 1u8)) }
    }

    #[inline]
    pub fn set_Bad(&mut self, val: usize) {
        unsafe {
            let val: u64 = std::mem::transmute(val);

            self._bitfield_1.set(31usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn ReservedUlong(&self) -> usize {
        unsafe { std::mem::transmute(self._bitfield_1.get(32usize, 32u8)) }
    }

    #[inline]
    pub fn set_ReservedUlong(&mut self, val: usize) {
        unsafe {
            let val: u64 = std::mem::transmute(val);

            self._bitfield_1.set(32usize, 32u8, val as u64)
        }
    }

    #[inline]
    pub fn new_bitfield_1(
        Valid: usize,
        Reserved0: usize,
        Shared: usize,
        Reserved1: usize,
        PageTable: usize,
        Location: usize,
        Priority: usize,
        ModifiedList: usize,
        Reserved2: usize,
        SharedOriginal: usize,
        Bad: usize,
        ReservedUlong: usize,
    ) -> BitfieldUnit<[u8; 8]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 8]> = Default::default();

        bitfield_unit.set(0usize, 1u8, {
            let Valid: u64 = unsafe { std::mem::transmute(Valid) };

            Valid as u64
        });

        bitfield_unit.set(1usize, 14u8, {
            let Reserved0: u64 = unsafe { std::mem::transmute(Reserved0) };

            Reserved0 as u64
        });

        bitfield_unit.set(15usize, 1u8, {
            let Shared: u64 = unsafe { std::mem::transmute(Shared) };

            Shared as u64
        });

        bitfield_unit.set(16usize, 5u8, {
            let Reserved1: u64 = unsafe { std::mem::transmute(Reserved1) };

            Reserved1 as u64
        });

        bitfield_unit.set(21usize, 1u8, {
            let PageTable: u64 = unsafe { std::mem::transmute(PageTable) };

            PageTable as u64
        });

        bitfield_unit.set(22usize, 2u8, {
            let Location: u64 = unsafe { std::mem::transmute(Location) };

            Location as u64
        });

        bitfield_unit.set(24usize, 3u8, {
            let Priority: u64 = unsafe { std::mem::transmute(Priority) };

            Priority as u64
        });

        bitfield_unit.set(27usize, 1u8, {
            let ModifiedList: u64 = unsafe { std::mem::transmute(ModifiedList) };

            ModifiedList as u64
        });

        bitfield_unit.set(28usize, 2u8, {
            let Reserved2: u64 = unsafe { std::mem::transmute(Reserved2) };

            Reserved2 as u64
        });

        bitfield_unit.set(30usize, 1u8, {
            let SharedOriginal: u64 = unsafe { std::mem::transmute(SharedOriginal) };

            SharedOriginal as u64
        });

        bitfield_unit.set(31usize, 1u8, {
            let Bad: u64 = unsafe { std::mem::transmute(Bad) };

            Bad as u64
        });

        bitfield_unit.set(32usize, 32u8, {
            let ReservedUlong: u64 = unsafe { std::mem::transmute(ReservedUlong) };

            ReservedUlong as u64
        });

        bitfield_unit
    }
}

impl Default for MEMORY_WORKING_SET_EX_BLOCK_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for MEMORY_WORKING_SET_EX_BLOCK_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MEMORY_WORKING_SET_EX_BLOCK_1 {{ union }}")
    }
}

impl Default for MEMORY_WORKING_SET_EX_BLOCK {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for MEMORY_WORKING_SET_EX_BLOCK {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MEMORY_WORKING_SET_EX_BLOCK {{ Anonymous1: {:?} }}",
            self.Anonymous1
        )
    }
}

#[repr(C)]
pub struct MEMORY_WORKING_SET_EX_INFORMATION {
    pub VirtualAddress: *mut std::ffi::c_void,
    pub u1: MEMORY_WORKING_SET_EX_INFORMATION_1,
}

#[repr(C)]
pub struct MEMORY_WORKING_SET_EX_INFORMATION_1 {
    pub VirtualAttributes: UnionField<MEMORY_WORKING_SET_EX_BLOCK>,
    pub Long: UnionField<usize>,
    pub union_field: u64,
}

impl Default for MEMORY_WORKING_SET_EX_INFORMATION_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for MEMORY_WORKING_SET_EX_INFORMATION_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MEMORY_WORKING_SET_EX_INFORMATION_1 {{ union }}")
    }
}

impl Default for MEMORY_WORKING_SET_EX_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for MEMORY_WORKING_SET_EX_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MEMORY_WORKING_SET_EX_INFORMATION {{ u1: {:?} }}",
            self.u1
        )
    }
}

#[repr(C)]
pub struct MEMORY_SHARED_COMMIT_INFORMATION {
    pub CommitSize: usize,
}

impl Default for MEMORY_SHARED_COMMIT_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for MEMORY_SHARED_COMMIT_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MEMORY_SHARED_COMMIT_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct MEMORY_IMAGE_INFORMATION {
    pub ImageBase: *mut std::ffi::c_void,
    pub SizeOfImage: usize,
    pub Anonymous1: MEMORY_IMAGE_INFORMATION_1,
}

#[repr(C)]
pub struct MEMORY_IMAGE_INFORMATION_1 {
    pub ImageFlags: UnionField<u32>,
    pub Anonymous1: UnionField<MEMORY_IMAGE_INFORMATION_1_1>,
    pub union_field: u32,
}

#[repr(C)]
#[repr(align(4))]
pub struct MEMORY_IMAGE_INFORMATION_1_1 {
    _bitfield_align_1: [u32; 0],
    _bitfield_1: BitfieldUnit<[u8; 4]>,
}

impl Default for MEMORY_IMAGE_INFORMATION_1_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for MEMORY_IMAGE_INFORMATION_1_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MEMORY_IMAGE_INFORMATION_1_1 {{ ImagePartialMap : {:?}, ImageNotExecutable : {:?}, ImageSigningLevel : {:?}, Reserved : {:?} }}",
            self.ImagePartialMap(),
            self.ImageNotExecutable(),
            self.ImageSigningLevel(),
            self.Reserved()
        )
    }
}

impl MEMORY_IMAGE_INFORMATION_1_1 {
    #[inline]
    pub fn ImagePartialMap(&self) -> u32 {
        self._bitfield_1.get(0usize, 1u8) as u32
    }

    #[inline]
    pub fn set_ImagePartialMap(&mut self, val: u32) {
        self._bitfield_1.set(0usize, 1u8, val as u64)
    }

    #[inline]
    pub fn ImageNotExecutable(&self) -> u32 {
        self._bitfield_1.get(1usize, 1u8) as u32
    }

    #[inline]
    pub fn set_ImageNotExecutable(&mut self, val: u32) {
        self._bitfield_1.set(1usize, 1u8, val as u64)
    }

    #[inline]
    pub fn ImageSigningLevel(&self) -> u32 {
        self._bitfield_1.get(2usize, 4u8) as u32
    }

    #[inline]
    pub fn set_ImageSigningLevel(&mut self, val: u32) {
        self._bitfield_1.set(2usize, 4u8, val as u64)
    }

    #[inline]
    pub fn Reserved(&self) -> u32 {
        self._bitfield_1.get(6usize, 26u8) as u32
    }

    #[inline]
    pub fn set_Reserved(&mut self, val: u32) {
        self._bitfield_1.set(6usize, 26u8, val as u64)
    }

    #[inline]
    pub fn new_bitfield_1(
        ImagePartialMap: u32,
        ImageNotExecutable: u32,
        ImageSigningLevel: u32,
        Reserved: u32,
    ) -> BitfieldUnit<[u8; 4]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 4]> = Default::default();

        bitfield_unit.set(0usize, 1u8, ImagePartialMap as u64);

        bitfield_unit.set(1usize, 1u8, ImageNotExecutable as u64);

        bitfield_unit.set(2usize, 4u8, ImageSigningLevel as u64);

        bitfield_unit.set(6usize, 26u8, Reserved as u64);

        bitfield_unit
    }
}

impl Default for MEMORY_IMAGE_INFORMATION_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for MEMORY_IMAGE_INFORMATION_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MEMORY_IMAGE_INFORMATION_1 {{ union }}")
    }
}

impl Default for MEMORY_IMAGE_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for MEMORY_IMAGE_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MEMORY_IMAGE_INFORMATION {{ Anonymous1: {:?} }}",
            self.Anonymous1
        )
    }
}

#[repr(C)]
pub struct MEMORY_ENCLAVE_IMAGE_INFORMATION {
    pub ImageInfo: MEMORY_IMAGE_INFORMATION,
    pub UniqueID: [u8; 32],
    pub AuthorID: [u8; 32],
}

impl Default for MEMORY_ENCLAVE_IMAGE_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for MEMORY_ENCLAVE_IMAGE_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MEMORY_ENCLAVE_IMAGE_INFORMATION {{ ImageInfo: {:?}, UniqueID: {:?}, AuthorID: {:?} }}",
            self.ImageInfo, self.UniqueID, self.AuthorID
        )
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MEMORY_PHYSICAL_CONTIGUITY_UNIT_STATE {
    MemoryNotContiguous = 0,
    MemoryAlignedAndContiguous = 1,
    MemoryNotResident = 2,
    MemoryNotEligibleToMakeContiguous = 3,
    MemoryContiguityStateMax = 4,
}

#[repr(C)]
pub struct MEMORY_PHYSICAL_CONTIGUITY_UNIT_INFORMATION {
    pub Anonymous1: MEMORY_PHYSICAL_CONTIGUITY_UNIT_INFORMATION_1,
}

#[repr(C)]
pub struct MEMORY_PHYSICAL_CONTIGUITY_UNIT_INFORMATION_1 {
    pub Anonymous1: UnionField<MEMORY_PHYSICAL_CONTIGUITY_UNIT_INFORMATION_1_1>,
    pub AllInformation: UnionField<u32>,
    pub union_field: u32,
}

#[repr(C)]
#[repr(align(4))]
pub struct MEMORY_PHYSICAL_CONTIGUITY_UNIT_INFORMATION_1_1 {
    _bitfield_align_1: [u32; 0],
    _bitfield_1: BitfieldUnit<[u8; 4]>,
}

impl Default for MEMORY_PHYSICAL_CONTIGUITY_UNIT_INFORMATION_1_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for MEMORY_PHYSICAL_CONTIGUITY_UNIT_INFORMATION_1_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MEMORY_PHYSICAL_CONTIGUITY_UNIT_INFORMATION_1_1 {{ State : {:?}, Reserved : {:?} }}",
            self.State(),
            self.Reserved()
        )
    }
}

impl MEMORY_PHYSICAL_CONTIGUITY_UNIT_INFORMATION_1_1 {
    #[inline]
    pub fn State(&self) -> u32 {
        self._bitfield_1.get(0usize, 2u8) as u32
    }

    #[inline]
    pub fn set_State(&mut self, val: u32) {
        self._bitfield_1.set(0usize, 2u8, val as u64)
    }

    #[inline]
    pub fn Reserved(&self) -> u32 {
        self._bitfield_1.get(2usize, 30u8) as u32
    }

    #[inline]
    pub fn set_Reserved(&mut self, val: u32) {
        self._bitfield_1.set(2usize, 30u8, val as u64)
    }

    #[inline]
    pub fn new_bitfield_1(State: u32, Reserved: u32) -> BitfieldUnit<[u8; 4]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 4]> = Default::default();

        bitfield_unit.set(0usize, 2u8, State as u64);

        bitfield_unit.set(2usize, 30u8, Reserved as u64);

        bitfield_unit
    }
}

impl Default for MEMORY_PHYSICAL_CONTIGUITY_UNIT_INFORMATION_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for MEMORY_PHYSICAL_CONTIGUITY_UNIT_INFORMATION_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MEMORY_PHYSICAL_CONTIGUITY_UNIT_INFORMATION_1 {{ union }}"
        )
    }
}

impl Default for MEMORY_PHYSICAL_CONTIGUITY_UNIT_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for MEMORY_PHYSICAL_CONTIGUITY_UNIT_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MEMORY_PHYSICAL_CONTIGUITY_UNIT_INFORMATION {{ Anonymous1: {:?} }}",
            self.Anonymous1
        )
    }
}

#[repr(C)]
pub struct MEMORY_PHYSICAL_CONTIGUITY_INFORMATION {
    pub VirtualAddress: *mut std::ffi::c_void,
    pub Size: usize,
    pub ContiguityUnitSize: usize,
    pub Flags: u32,
    pub ContiguityUnitInformation: *mut MEMORY_PHYSICAL_CONTIGUITY_UNIT_INFORMATION,
}

impl Default for MEMORY_PHYSICAL_CONTIGUITY_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for MEMORY_PHYSICAL_CONTIGUITY_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MEMORY_PHYSICAL_CONTIGUITY_INFORMATION {{ ContiguityUnitInformation: {:?} }}",
            self.ContiguityUnitInformation
        )
    }
}

#[repr(C)]
#[repr(align(8))]
pub struct MEMORY_FRAME_INFORMATION {
    _bitfield_align_1: [u64; 0],
    _bitfield_1: BitfieldUnit<[u8; 8]>,
}

impl Default for MEMORY_FRAME_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for MEMORY_FRAME_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MEMORY_FRAME_INFORMATION {{ UseDescription : {:?}, ListDescription : {:?}, Cold : {:?}, Pinned : {:?}, DontUse : {:?}, Priority : {:?}, NonTradeable : {:?}, Reserved : {:?} }}",
            self.UseDescription(),
            self.ListDescription(),
            self.Cold(),
            self.Pinned(),
            self.DontUse(),
            self.Priority(),
            self.NonTradeable(),
            self.Reserved()
        )
    }
}

impl MEMORY_FRAME_INFORMATION {
    #[inline]
    pub fn UseDescription(&self) -> u64 {
        self._bitfield_1.get(0usize, 4u8)
    }

    #[inline]
    pub fn set_UseDescription(&mut self, val: u64) {
        self._bitfield_1.set(0usize, 4u8, val)
    }

    #[inline]
    pub fn ListDescription(&self) -> u64 {
        self._bitfield_1.get(4usize, 3u8)
    }

    #[inline]
    pub fn set_ListDescription(&mut self, val: u64) {
        self._bitfield_1.set(4usize, 3u8, val)
    }

    #[inline]
    pub fn Cold(&self) -> u64 {
        self._bitfield_1.get(7usize, 1u8)
    }

    #[inline]
    pub fn set_Cold(&mut self, val: u64) {
        self._bitfield_1.set(7usize, 1u8, val)
    }

    #[inline]
    pub fn Pinned(&self) -> u64 {
        self._bitfield_1.get(8usize, 1u8)
    }

    #[inline]
    pub fn set_Pinned(&mut self, val: u64) {
        self._bitfield_1.set(8usize, 1u8, val)
    }

    #[inline]
    pub fn DontUse(&self) -> u64 {
        self._bitfield_1.get(9usize, 48u8)
    }

    #[inline]
    pub fn set_DontUse(&mut self, val: u64) {
        self._bitfield_1.set(9usize, 48u8, val)
    }

    #[inline]
    pub fn Priority(&self) -> u64 {
        self._bitfield_1.get(57usize, 3u8)
    }

    #[inline]
    pub fn set_Priority(&mut self, val: u64) {
        self._bitfield_1.set(57usize, 3u8, val)
    }

    #[inline]
    pub fn NonTradeable(&self) -> u64 {
        self._bitfield_1.get(60usize, 1u8)
    }

    #[inline]
    pub fn set_NonTradeable(&mut self, val: u64) {
        self._bitfield_1.set(60usize, 1u8, val)
    }

    #[inline]
    pub fn Reserved(&self) -> u64 {
        self._bitfield_1.get(61usize, 3u8)
    }

    #[inline]
    pub fn set_Reserved(&mut self, val: u64) {
        self._bitfield_1.set(61usize, 3u8, val)
    }

    #[inline]
    pub fn new_bitfield_1(
        UseDescription: u64,
        ListDescription: u64,
        Cold: u64,
        Pinned: u64,
        DontUse: u64,
        Priority: u64,
        NonTradeable: u64,
        Reserved: u64,
    ) -> BitfieldUnit<[u8; 8]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 8]> = Default::default();

        bitfield_unit.set(0usize, 4u8, UseDescription);

        bitfield_unit.set(4usize, 3u8, ListDescription);

        bitfield_unit.set(7usize, 1u8, Cold);

        bitfield_unit.set(8usize, 1u8, Pinned);

        bitfield_unit.set(9usize, 48u8, DontUse);

        bitfield_unit.set(57usize, 3u8, Priority);

        bitfield_unit.set(60usize, 1u8, NonTradeable);

        bitfield_unit.set(61usize, 3u8, Reserved);

        bitfield_unit
    }
}

#[repr(C)]
#[repr(align(8))]
pub struct FILEOFFSET_INFORMATION {
    _bitfield_align_1: [u64; 0],
    _bitfield_1: BitfieldUnit<[u8; 8]>,
}

impl Default for FILEOFFSET_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for FILEOFFSET_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FILEOFFSET_INFORMATION {{ DontUse : {:?}, Offset : {:?}, Reserved : {:?} }}",
            self.DontUse(),
            self.Offset(),
            self.Reserved()
        )
    }
}

impl FILEOFFSET_INFORMATION {
    #[inline]
    pub fn DontUse(&self) -> u64 {
        self._bitfield_1.get(0usize, 9u8)
    }

    #[inline]
    pub fn set_DontUse(&mut self, val: u64) {
        self._bitfield_1.set(0usize, 9u8, val)
    }

    #[inline]
    pub fn Offset(&self) -> u64 {
        self._bitfield_1.get(9usize, 48u8)
    }

    #[inline]
    pub fn set_Offset(&mut self, val: u64) {
        self._bitfield_1.set(9usize, 48u8, val)
    }

    #[inline]
    pub fn Reserved(&self) -> u64 {
        self._bitfield_1.get(57usize, 7u8)
    }

    #[inline]
    pub fn set_Reserved(&mut self, val: u64) {
        self._bitfield_1.set(57usize, 7u8, val)
    }

    #[inline]
    pub fn new_bitfield_1(DontUse: u64, Offset: u64, Reserved: u64) -> BitfieldUnit<[u8; 8]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 8]> = Default::default();

        bitfield_unit.set(0usize, 9u8, DontUse);

        bitfield_unit.set(9usize, 48u8, Offset);

        bitfield_unit.set(57usize, 7u8, Reserved);

        bitfield_unit
    }
}

#[repr(C)]
#[repr(align(8))]
pub struct PAGEDIR_INFORMATION {
    _bitfield_align_1: [u64; 0],
    _bitfield_1: BitfieldUnit<[u8; 8]>,
}

impl Default for PAGEDIR_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PAGEDIR_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PAGEDIR_INFORMATION {{ DontUse : {:?}, PageDirectoryBase : {:?}, Reserved : {:?} }}",
            self.DontUse(),
            self.PageDirectoryBase(),
            self.Reserved()
        )
    }
}

impl PAGEDIR_INFORMATION {
    #[inline]
    pub fn DontUse(&self) -> u64 {
        self._bitfield_1.get(0usize, 9u8)
    }

    #[inline]
    pub fn set_DontUse(&mut self, val: u64) {
        self._bitfield_1.set(0usize, 9u8, val)
    }

    #[inline]
    pub fn PageDirectoryBase(&self) -> u64 {
        self._bitfield_1.get(9usize, 48u8)
    }

    #[inline]
    pub fn set_PageDirectoryBase(&mut self, val: u64) {
        self._bitfield_1.set(9usize, 48u8, val)
    }

    #[inline]
    pub fn Reserved(&self) -> u64 {
        self._bitfield_1.get(57usize, 7u8)
    }

    #[inline]
    pub fn set_Reserved(&mut self, val: u64) {
        self._bitfield_1.set(57usize, 7u8, val)
    }

    #[inline]
    pub fn new_bitfield_1(
        DontUse: u64,
        PageDirectoryBase: u64,
        Reserved: u64,
    ) -> BitfieldUnit<[u8; 8]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 8]> = Default::default();

        bitfield_unit.set(0usize, 9u8, DontUse);

        bitfield_unit.set(9usize, 48u8, PageDirectoryBase);

        bitfield_unit.set(57usize, 7u8, Reserved);

        bitfield_unit
    }
}

#[repr(C)]
#[repr(align(8))]
pub struct UNIQUE_PROCESS_INFORMATION {
    _bitfield_align_1: [u64; 0],
    _bitfield_1: BitfieldUnit<[u8; 8]>,
}

impl Default for UNIQUE_PROCESS_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for UNIQUE_PROCESS_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "UNIQUE_PROCESS_INFORMATION {{ DontUse : {:?}, UniqueProcessKey : {:?}, Reserved : {:?} }}",
            self.DontUse(),
            self.UniqueProcessKey(),
            self.Reserved()
        )
    }
}

impl UNIQUE_PROCESS_INFORMATION {
    #[inline]
    pub fn DontUse(&self) -> u64 {
        self._bitfield_1.get(0usize, 9u8)
    }

    #[inline]
    pub fn set_DontUse(&mut self, val: u64) {
        self._bitfield_1.set(0usize, 9u8, val)
    }

    #[inline]
    pub fn UniqueProcessKey(&self) -> u64 {
        self._bitfield_1.get(9usize, 48u8)
    }

    #[inline]
    pub fn set_UniqueProcessKey(&mut self, val: u64) {
        self._bitfield_1.set(9usize, 48u8, val)
    }

    #[inline]
    pub fn Reserved(&self) -> u64 {
        self._bitfield_1.get(57usize, 7u8)
    }

    #[inline]
    pub fn set_Reserved(&mut self, val: u64) {
        self._bitfield_1.set(57usize, 7u8, val)
    }

    #[inline]
    pub fn new_bitfield_1(
        DontUse: u64,
        UniqueProcessKey: u64,
        Reserved: u64,
    ) -> BitfieldUnit<[u8; 8]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 8]> = Default::default();

        bitfield_unit.set(0usize, 9u8, DontUse);

        bitfield_unit.set(9usize, 48u8, UniqueProcessKey);

        bitfield_unit.set(57usize, 7u8, Reserved);

        bitfield_unit
    }
}

#[repr(C)]
pub struct MMPFN_IDENTITY {
    pub u1: MMPFN_IDENTITY_1,
    pub PageFrameIndex: usize,
    pub u2: MMPFN_IDENTITY_2,
}

#[repr(C)]
pub struct MMPFN_IDENTITY_1 {
    pub e1: UnionField<MEMORY_FRAME_INFORMATION>,
    pub e2: UnionField<FILEOFFSET_INFORMATION>,
    pub e3: UnionField<PAGEDIR_INFORMATION>,
    pub e4: UnionField<UNIQUE_PROCESS_INFORMATION>,
    pub union_field: u64,
}

impl Default for MMPFN_IDENTITY_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for MMPFN_IDENTITY_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MMPFN_IDENTITY_1 {{ union }}")
    }
}

#[repr(C)]
pub struct MMPFN_IDENTITY_2 {
    pub e1: UnionField<MMPFN_IDENTITY_2_1>,
    pub e2: UnionField<MMPFN_IDENTITY_2_2>,
    pub FileObject: UnionField<usize>,
    pub UniqueFileObjectKey: UnionField<usize>,
    pub ProtoPteAddress: UnionField<usize>,
    pub VirtualAddress: UnionField<usize>,
    pub union_field: u64,
}

#[repr(C)]
#[repr(align(8))]
pub struct MMPFN_IDENTITY_2_1 {
    _bitfield_align_1: [u8; 0],
    _bitfield_1: BitfieldUnit<[u8; 1]>,
    pub padding_0: [u8; 7],
}

impl Default for MMPFN_IDENTITY_2_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for MMPFN_IDENTITY_2_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MMPFN_IDENTITY_2_1 {{ Image : {:?}, Mismatch : {:?} }}",
            self.Image(),
            self.Mismatch()
        )
    }
}

impl MMPFN_IDENTITY_2_1 {
    #[inline]
    pub fn Image(&self) -> usize {
        unsafe { std::mem::transmute(self._bitfield_1.get(0usize, 1u8)) }
    }

    #[inline]
    pub fn set_Image(&mut self, val: usize) {
        unsafe {
            let val: u64 = std::mem::transmute(val);

            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn Mismatch(&self) -> usize {
        unsafe { std::mem::transmute(self._bitfield_1.get(1usize, 1u8)) }
    }

    #[inline]
    pub fn set_Mismatch(&mut self, val: usize) {
        unsafe {
            let val: u64 = std::mem::transmute(val);

            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn new_bitfield_1(Image: usize, Mismatch: usize) -> BitfieldUnit<[u8; 1]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 1]> = Default::default();

        bitfield_unit.set(0usize, 1u8, {
            let Image: u64 = unsafe { std::mem::transmute(Image) };

            Image as u64
        });

        bitfield_unit.set(1usize, 1u8, {
            let Mismatch: u64 = unsafe { std::mem::transmute(Mismatch) };

            Mismatch as u64
        });

        bitfield_unit
    }
}

#[repr(C)]
pub struct MMPFN_IDENTITY_2_2 {
    pub CombinedPage: usize,
}

impl Default for MMPFN_IDENTITY_2_2 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for MMPFN_IDENTITY_2_2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MMPFN_IDENTITY_2_2 {{  }}")
    }
}

impl Default for MMPFN_IDENTITY_2 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for MMPFN_IDENTITY_2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MMPFN_IDENTITY_2 {{ union }}")
    }
}

impl Default for MMPFN_IDENTITY {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for MMPFN_IDENTITY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MMPFN_IDENTITY {{ u1: {:?}, u2: {:?} }}",
            self.u1, self.u2
        )
    }
}

#[repr(C)]
pub struct MMPFN_MEMSNAP_INFORMATION {
    pub InitialPageFrameIndex: usize,
    pub Count: usize,
}

impl Default for MMPFN_MEMSNAP_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for MMPFN_MEMSNAP_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MMPFN_MEMSNAP_INFORMATION {{  }}")
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SECTION_INFORMATION_CLASS {
    SectionBasicInformation = 0,
    SectionImageInformation = 1,
    SectionRelocationInformation = 2,
    SectionOriginalBaseInformation = 3,
    SectionInternalImageInformation = 4,
    MaxSectionInfoClass = 5,
}

#[repr(C)]
pub struct SECTION_BASIC_INFORMATION {
    pub BaseAddress: *mut std::ffi::c_void,
    pub AllocationAttributes: u32,
    pub MaximumSize: i64,
}

impl Default for SECTION_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for SECTION_BASIC_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SECTION_BASIC_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct SECTION_IMAGE_INFORMATION {
    pub TransferAddress: *mut std::ffi::c_void,
    pub ZeroBits: u32,
    pub MaximumStackSize: usize,
    pub CommittedStackSize: usize,
    pub SubSystemType: u32,
    pub Anonymous1: SECTION_IMAGE_INFORMATION_1,
    pub Anonymous2: SECTION_IMAGE_INFORMATION_2,
    pub ImageCharacteristics: u16,
    pub DllCharacteristics: u16,
    pub Machine: u16,
    pub ImageContainsCode: bool,
    pub Anonymous3: SECTION_IMAGE_INFORMATION_3,
    pub LoaderFlags: u32,
    pub ImageFileSize: u32,
    pub CheckSum: u32,
}

#[repr(C)]
pub struct SECTION_IMAGE_INFORMATION_1 {
    pub Anonymous1: UnionField<SECTION_IMAGE_INFORMATION_1_1>,
    pub SubSystemVersion: UnionField<u32>,
    pub union_field: u32,
}

#[repr(C)]
pub struct SECTION_IMAGE_INFORMATION_1_1 {
    pub SubSystemMinorVersion: u16,
    pub SubSystemMajorVersion: u16,
}

impl Default for SECTION_IMAGE_INFORMATION_1_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for SECTION_IMAGE_INFORMATION_1_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SECTION_IMAGE_INFORMATION_1_1 {{  }}")
    }
}

impl Default for SECTION_IMAGE_INFORMATION_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for SECTION_IMAGE_INFORMATION_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SECTION_IMAGE_INFORMATION_1 {{ union }}")
    }
}

#[repr(C)]
pub struct SECTION_IMAGE_INFORMATION_2 {
    pub Anonymous1: UnionField<SECTION_IMAGE_INFORMATION_2_1>,
    pub OperatingSystemVersion: UnionField<u32>,
    pub union_field: u32,
}

#[repr(C)]
pub struct SECTION_IMAGE_INFORMATION_2_1 {
    pub MajorOperatingSystemVersion: u16,
    pub MinorOperatingSystemVersion: u16,
}

impl Default for SECTION_IMAGE_INFORMATION_2_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for SECTION_IMAGE_INFORMATION_2_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SECTION_IMAGE_INFORMATION_2_1 {{  }}")
    }
}

impl Default for SECTION_IMAGE_INFORMATION_2 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for SECTION_IMAGE_INFORMATION_2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SECTION_IMAGE_INFORMATION_2 {{ union }}")
    }
}

#[repr(C)]
pub struct SECTION_IMAGE_INFORMATION_3 {
    pub ImageFlags: UnionField<u8>,
    pub Anonymous1: UnionField<SECTION_IMAGE_INFORMATION_3_1>,
    pub union_field: u8,
}

#[repr(C, packed)]
pub struct SECTION_IMAGE_INFORMATION_3_1 {
    _bitfield_align_1: [u8; 0],
    _bitfield_1: BitfieldUnit<[u8; 1]>,
}

impl Default for SECTION_IMAGE_INFORMATION_3_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for SECTION_IMAGE_INFORMATION_3_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SECTION_IMAGE_INFORMATION_3_1 {{ ComPlusNativeReady : {:?}, ComPlusILOnly : {:?}, ImageDynamicallyRelocated : {:?}, ImageMappedFlat : {:?}, BaseBelow4gb : {:?}, ComPlusPrefer32bit : {:?}, Reserved : {:?} }}",
            self.ComPlusNativeReady(),
            self.ComPlusILOnly(),
            self.ImageDynamicallyRelocated(),
            self.ImageMappedFlat(),
            self.BaseBelow4gb(),
            self.ComPlusPrefer32bit(),
            self.Reserved()
        )
    }
}

impl SECTION_IMAGE_INFORMATION_3_1 {
    #[inline]
    pub fn ComPlusNativeReady(&self) -> u8 {
        self._bitfield_1.get(0usize, 1u8) as u8
    }

    #[inline]
    pub fn set_ComPlusNativeReady(&mut self, val: u8) {
        self._bitfield_1.set(0usize, 1u8, val as u64)
    }

    #[inline]
    pub fn ComPlusILOnly(&self) -> u8 {
        self._bitfield_1.get(1usize, 1u8) as u8
    }

    #[inline]
    pub fn set_ComPlusILOnly(&mut self, val: u8) {
        self._bitfield_1.set(1usize, 1u8, val as u64)
    }

    #[inline]
    pub fn ImageDynamicallyRelocated(&self) -> u8 {
        self._bitfield_1.get(2usize, 1u8) as u8
    }

    #[inline]
    pub fn set_ImageDynamicallyRelocated(&mut self, val: u8) {
        self._bitfield_1.set(2usize, 1u8, val as u64)
    }

    #[inline]
    pub fn ImageMappedFlat(&self) -> u8 {
        self._bitfield_1.get(3usize, 1u8) as u8
    }

    #[inline]
    pub fn set_ImageMappedFlat(&mut self, val: u8) {
        self._bitfield_1.set(3usize, 1u8, val as u64)
    }

    #[inline]
    pub fn BaseBelow4gb(&self) -> u8 {
        self._bitfield_1.get(4usize, 1u8) as u8
    }

    #[inline]
    pub fn set_BaseBelow4gb(&mut self, val: u8) {
        self._bitfield_1.set(4usize, 1u8, val as u64)
    }

    #[inline]
    pub fn ComPlusPrefer32bit(&self) -> u8 {
        self._bitfield_1.get(5usize, 1u8) as u8
    }

    #[inline]
    pub fn set_ComPlusPrefer32bit(&mut self, val: u8) {
        self._bitfield_1.set(5usize, 1u8, val as u64)
    }

    #[inline]
    pub fn Reserved(&self) -> u8 {
        self._bitfield_1.get(6usize, 2u8) as u8
    }

    #[inline]
    pub fn set_Reserved(&mut self, val: u8) {
        self._bitfield_1.set(6usize, 2u8, val as u64)
    }

    #[inline]
    pub fn new_bitfield_1(
        ComPlusNativeReady: u8,
        ComPlusILOnly: u8,
        ImageDynamicallyRelocated: u8,
        ImageMappedFlat: u8,
        BaseBelow4gb: u8,
        ComPlusPrefer32bit: u8,
        Reserved: u8,
    ) -> BitfieldUnit<[u8; 1]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 1]> = Default::default();

        bitfield_unit.set(0usize, 1u8, ComPlusNativeReady as u64);

        bitfield_unit.set(1usize, 1u8, ComPlusILOnly as u64);

        bitfield_unit.set(2usize, 1u8, ImageDynamicallyRelocated as u64);

        bitfield_unit.set(3usize, 1u8, ImageMappedFlat as u64);

        bitfield_unit.set(4usize, 1u8, BaseBelow4gb as u64);

        bitfield_unit.set(5usize, 1u8, ComPlusPrefer32bit as u64);

        bitfield_unit.set(6usize, 2u8, Reserved as u64);

        bitfield_unit
    }
}

impl Default for SECTION_IMAGE_INFORMATION_3 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for SECTION_IMAGE_INFORMATION_3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SECTION_IMAGE_INFORMATION_3 {{ union }}")
    }
}

impl Default for SECTION_IMAGE_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for SECTION_IMAGE_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SECTION_IMAGE_INFORMATION {{ Anonymous1: {:?}, Anonymous2: {:?}, Anonymous3: {:?} }}",
            self.Anonymous1, self.Anonymous2, self.Anonymous3
        )
    }
}

#[repr(C)]
pub struct SECTION_INTERNAL_IMAGE_INFORMATION {
    pub SectionInformation: SECTION_IMAGE_INFORMATION,
    pub Anonymous1: SECTION_INTERNAL_IMAGE_INFORMATION_1,
}

#[repr(C)]
pub struct SECTION_INTERNAL_IMAGE_INFORMATION_1 {
    pub ExtendedFlags: UnionField<u32>,
    pub Anonymous1: UnionField<SECTION_INTERNAL_IMAGE_INFORMATION_1_1>,
    pub union_field: u32,
}

#[repr(C)]
#[repr(align(4))]
pub struct SECTION_INTERNAL_IMAGE_INFORMATION_1_1 {
    _bitfield_align_1: [u32; 0],
    _bitfield_1: BitfieldUnit<[u8; 4]>,
}

impl Default for SECTION_INTERNAL_IMAGE_INFORMATION_1_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for SECTION_INTERNAL_IMAGE_INFORMATION_1_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SECTION_INTERNAL_IMAGE_INFORMATION_1_1 {{ ImageExportSuppressionEnabled : {:?}, ImageCetShadowStacksReady : {:?}, ImageXfgEnabled : {:?}, ImageCetShadowStacksStrictMode : {:?}, ImageCetSetContextIpValidationRelaxedMode : {:?}, ImageCetDynamicApisAllowInProc : {:?}, ImageCetDowngradeReserved1 : {:?}, ImageCetDowngradeReserved2 : {:?}, Reserved : {:?} }}",
            self.ImageExportSuppressionEnabled(),
            self.ImageCetShadowStacksReady(),
            self.ImageXfgEnabled(),
            self.ImageCetShadowStacksStrictMode(),
            self.ImageCetSetContextIpValidationRelaxedMode(),
            self.ImageCetDynamicApisAllowInProc(),
            self.ImageCetDowngradeReserved1(),
            self.ImageCetDowngradeReserved2(),
            self.Reserved()
        )
    }
}

impl SECTION_INTERNAL_IMAGE_INFORMATION_1_1 {
    #[inline]
    pub fn ImageExportSuppressionEnabled(&self) -> u32 {
        self._bitfield_1.get(0usize, 1u8) as u32
    }

    #[inline]
    pub fn set_ImageExportSuppressionEnabled(&mut self, val: u32) {
        self._bitfield_1.set(0usize, 1u8, val as u64)
    }

    #[inline]
    pub fn ImageCetShadowStacksReady(&self) -> u32 {
        self._bitfield_1.get(1usize, 1u8) as u32
    }

    #[inline]
    pub fn set_ImageCetShadowStacksReady(&mut self, val: u32) {
        self._bitfield_1.set(1usize, 1u8, val as u64)
    }

    #[inline]
    pub fn ImageXfgEnabled(&self) -> u32 {
        self._bitfield_1.get(2usize, 1u8) as u32
    }

    #[inline]
    pub fn set_ImageXfgEnabled(&mut self, val: u32) {
        self._bitfield_1.set(2usize, 1u8, val as u64)
    }

    #[inline]
    pub fn ImageCetShadowStacksStrictMode(&self) -> u32 {
        self._bitfield_1.get(3usize, 1u8) as u32
    }

    #[inline]
    pub fn set_ImageCetShadowStacksStrictMode(&mut self, val: u32) {
        self._bitfield_1.set(3usize, 1u8, val as u64)
    }

    #[inline]
    pub fn ImageCetSetContextIpValidationRelaxedMode(&self) -> u32 {
        self._bitfield_1.get(4usize, 1u8) as u32
    }

    #[inline]
    pub fn set_ImageCetSetContextIpValidationRelaxedMode(&mut self, val: u32) {
        self._bitfield_1.set(4usize, 1u8, val as u64)
    }

    #[inline]
    pub fn ImageCetDynamicApisAllowInProc(&self) -> u32 {
        self._bitfield_1.get(5usize, 1u8) as u32
    }

    #[inline]
    pub fn set_ImageCetDynamicApisAllowInProc(&mut self, val: u32) {
        self._bitfield_1.set(5usize, 1u8, val as u64)
    }

    #[inline]
    pub fn ImageCetDowngradeReserved1(&self) -> u32 {
        self._bitfield_1.get(6usize, 1u8) as u32
    }

    #[inline]
    pub fn set_ImageCetDowngradeReserved1(&mut self, val: u32) {
        self._bitfield_1.set(6usize, 1u8, val as u64)
    }

    #[inline]
    pub fn ImageCetDowngradeReserved2(&self) -> u32 {
        self._bitfield_1.get(7usize, 1u8) as u32
    }

    #[inline]
    pub fn set_ImageCetDowngradeReserved2(&mut self, val: u32) {
        self._bitfield_1.set(7usize, 1u8, val as u64)
    }

    #[inline]
    pub fn Reserved(&self) -> u32 {
        self._bitfield_1.get(8usize, 24u8) as u32
    }

    #[inline]
    pub fn set_Reserved(&mut self, val: u32) {
        self._bitfield_1.set(8usize, 24u8, val as u64)
    }

    #[inline]
    pub fn new_bitfield_1(
        ImageExportSuppressionEnabled: u32,
        ImageCetShadowStacksReady: u32,
        ImageXfgEnabled: u32,
        ImageCetShadowStacksStrictMode: u32,
        ImageCetSetContextIpValidationRelaxedMode: u32,
        ImageCetDynamicApisAllowInProc: u32,
        ImageCetDowngradeReserved1: u32,
        ImageCetDowngradeReserved2: u32,
        Reserved: u32,
    ) -> BitfieldUnit<[u8; 4]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 4]> = Default::default();

        bitfield_unit.set(0usize, 1u8, ImageExportSuppressionEnabled as u64);

        bitfield_unit.set(1usize, 1u8, ImageCetShadowStacksReady as u64);

        bitfield_unit.set(2usize, 1u8, ImageXfgEnabled as u64);

        bitfield_unit.set(3usize, 1u8, ImageCetShadowStacksStrictMode as u64);

        bitfield_unit.set(
            4usize,
            1u8,
            ImageCetSetContextIpValidationRelaxedMode as u64,
        );

        bitfield_unit.set(5usize, 1u8, ImageCetDynamicApisAllowInProc as u64);

        bitfield_unit.set(6usize, 1u8, ImageCetDowngradeReserved1 as u64);

        bitfield_unit.set(7usize, 1u8, ImageCetDowngradeReserved2 as u64);

        bitfield_unit.set(8usize, 24u8, Reserved as u64);

        bitfield_unit
    }
}

impl Default for SECTION_INTERNAL_IMAGE_INFORMATION_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for SECTION_INTERNAL_IMAGE_INFORMATION_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SECTION_INTERNAL_IMAGE_INFORMATION_1 {{ union }}")
    }
}

impl Default for SECTION_INTERNAL_IMAGE_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for SECTION_INTERNAL_IMAGE_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SECTION_INTERNAL_IMAGE_INFORMATION {{ SectionInformation: {:?}, Anonymous1: {:?} }}",
            self.SectionInformation, self.Anonymous1
        )
    }
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtAllocateVirtualMemoryEx(
        ProcessHandle: HANDLE,
        BaseAddress: *mut *mut std::ffi::c_void,
        RegionSize: *mut usize,
        AllocationType: u32,
        PageProtection: u32,
        ExtendedParameters: *mut MEM_EXTENDED_PARAMETER,
        ExtendedParameterCount: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtReadVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut std::ffi::c_void,
        Buffer: *mut std::ffi::c_void,
        BufferSize: usize,
        NumberOfBytesRead: *mut usize,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtReadVirtualMemoryEx(
        ProcessHandle: HANDLE,
        BaseAddress: *mut std::ffi::c_void,
        Buffer: *mut std::ffi::c_void,
        BufferSize: usize,
        NumberOfBytesRead: *mut usize,
        Flags: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtWriteVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut std::ffi::c_void,
        Buffer: *mut std::ffi::c_void,
        BufferSize: usize,
        NumberOfBytesWritten: *mut usize,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtProtectVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut *mut std::ffi::c_void,
        RegionSize: *mut usize,
        NewProtect: u32,
        OldProtect: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtFlushVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut *mut std::ffi::c_void,
        RegionSize: *mut usize,
        IoStatus: *mut IO_STATUS_BLOCK,
    ) -> NTSTATUS;
}

#[repr(C)]
pub struct CFG_CALL_TARGET_LIST_INFORMATION {
    pub NumberOfEntries: u32,
    pub Reserved: u32,
    pub NumberOfEntriesProcessed: *mut u32,
    pub CallTargetInfo: *mut CFG_CALL_TARGET_INFO,
    pub Section: *mut std::ffi::c_void,
    pub FileOffset: u64,
}

impl Default for CFG_CALL_TARGET_LIST_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for CFG_CALL_TARGET_LIST_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CFG_CALL_TARGET_LIST_INFORMATION {{  }}")
    }
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtLockVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut *mut std::ffi::c_void,
        RegionSize: *mut usize,
        MapType: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtUnlockVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut *mut std::ffi::c_void,
        RegionSize: *mut usize,
        MapType: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtOpenSection(
        SectionHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtMapViewOfSection(
        SectionHandle: HANDLE,
        ProcessHandle: HANDLE,
        BaseAddress: *mut *mut std::ffi::c_void,
        ZeroBits: usize,
        CommitSize: usize,
        SectionOffset: *mut i64,
        ViewSize: *mut usize,
        InheritDisposition: SECTION_INHERIT,
        AllocationType: u32,
        Win32Protect: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtMapViewOfSectionEx(
        SectionHandle: HANDLE,
        ProcessHandle: HANDLE,
        BaseAddress: *mut *mut std::ffi::c_void,
        SectionOffset: *mut i64,
        ViewSize: *mut usize,
        AllocationType: u32,
        Win32Protect: u32,
        ExtendedParameters: *mut MEM_EXTENDED_PARAMETER,
        ExtendedParameterCount: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtUnmapViewOfSection(
        ProcessHandle: HANDLE,
        BaseAddress: *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtUnmapViewOfSectionEx(
        ProcessHandle: HANDLE,
        BaseAddress: *mut std::ffi::c_void,
        Flags: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtExtendSection(SectionHandle: HANDLE, NewSectionSize: *mut i64) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtQuerySection(
        SectionHandle: HANDLE,
        SectionInformationClass: SECTION_INFORMATION_CLASS,
        SectionInformation: *mut std::ffi::c_void,
        SectionInformationLength: usize,
        ReturnLength: *mut usize,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtAreMappedFilesTheSame(
        File1MappedAsAnImage: *mut std::ffi::c_void,
        File2MappedAsFile: *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[repr(C)]
pub struct MEMORY_PARTITION_CONFIGURATION_INFORMATION {
    pub Flags: u32,
    pub NumaNode: u32,
    pub Channel: u32,
    pub NumberOfNumaNodes: u32,
    pub ResidentAvailablePages: usize,
    pub CommittedPages: usize,
    pub CommitLimit: usize,
    pub PeakCommitment: usize,
    pub TotalNumberOfPages: usize,
    pub AvailablePages: usize,
    pub ZeroPages: usize,
    pub FreePages: usize,
    pub StandbyPages: usize,
    pub StandbyPageCountByPriority: [usize; 8],
    pub RepurposedPagesByPriority: [usize; 8],
    pub MaximumCommitLimit: usize,
    pub Reserved: usize,
    pub PartitionId: u32,
}

impl Default for MEMORY_PARTITION_CONFIGURATION_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for MEMORY_PARTITION_CONFIGURATION_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MEMORY_PARTITION_CONFIGURATION_INFORMATION {{ StandbyPageCountByPriority: {:?}, RepurposedPagesByPriority: {:?} }}",
            self.StandbyPageCountByPriority, self.RepurposedPagesByPriority
        )
    }
}

#[repr(C)]
pub struct MEMORY_PARTITION_TRANSFER_INFORMATION {
    pub NumberOfPages: usize,
    pub NumaNode: u32,
    pub Flags: u32,
}

impl Default for MEMORY_PARTITION_TRANSFER_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for MEMORY_PARTITION_TRANSFER_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MEMORY_PARTITION_TRANSFER_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct MEMORY_PARTITION_PAGEFILE_INFORMATION {
    pub PageFileName: UNICODE_STRING,
    pub MinimumSize: i64,
    pub MaximumSize: i64,
    pub Flags: u32,
}

impl Default for MEMORY_PARTITION_PAGEFILE_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for MEMORY_PARTITION_PAGEFILE_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MEMORY_PARTITION_PAGEFILE_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct MEMORY_PARTITION_PAGE_COMBINE_INFORMATION {
    pub StopHandle: HANDLE,
    pub Flags: u32,
    pub TotalNumberOfPages: usize,
}

impl Default for MEMORY_PARTITION_PAGE_COMBINE_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for MEMORY_PARTITION_PAGE_COMBINE_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MEMORY_PARTITION_PAGE_COMBINE_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct MEMORY_PARTITION_PAGE_RANGE {
    pub StartPage: usize,
    pub NumberOfPages: usize,
}

impl Default for MEMORY_PARTITION_PAGE_RANGE {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for MEMORY_PARTITION_PAGE_RANGE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MEMORY_PARTITION_PAGE_RANGE {{  }}")
    }
}

#[repr(C)]
pub struct MEMORY_PARTITION_INITIAL_ADD_INFORMATION {
    pub Flags: u32,
    pub NumberOfRanges: u32,
    pub NumberOfPagesAdded: usize,
    pub PartitionRanges: [MEMORY_PARTITION_PAGE_RANGE; 1],
}

impl Default for MEMORY_PARTITION_INITIAL_ADD_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for MEMORY_PARTITION_INITIAL_ADD_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MEMORY_PARTITION_INITIAL_ADD_INFORMATION {{ PartitionRanges: {:?} }}",
            self.PartitionRanges
        )
    }
}

#[repr(C)]
pub struct MEMORY_PARTITION_MEMORY_EVENTS_INFORMATION {
    pub Flags: MEMORY_PARTITION_MEMORY_EVENTS_INFORMATION_1,
    pub HandleAttributes: u32,
    pub DesiredAccess: u32,
    pub LowCommitCondition: HANDLE,
    pub HighCommitCondition: HANDLE,
    pub MaximumCommitCondition: HANDLE,
}

#[repr(C)]
pub struct MEMORY_PARTITION_MEMORY_EVENTS_INFORMATION_1 {
    pub Anonymous1: UnionField<MEMORY_PARTITION_MEMORY_EVENTS_INFORMATION_1_1>,
    pub AllFlags: UnionField<u32>,
    pub union_field: u32,
}

#[repr(C)]
#[repr(align(4))]
pub struct MEMORY_PARTITION_MEMORY_EVENTS_INFORMATION_1_1 {
    _bitfield_align_1: [u32; 0],
    _bitfield_1: BitfieldUnit<[u8; 4]>,
}

impl Default for MEMORY_PARTITION_MEMORY_EVENTS_INFORMATION_1_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for MEMORY_PARTITION_MEMORY_EVENTS_INFORMATION_1_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MEMORY_PARTITION_MEMORY_EVENTS_INFORMATION_1_1 {{ CommitEvents : {:?}, Spare : {:?} }}",
            self.CommitEvents(),
            self.Spare()
        )
    }
}

impl MEMORY_PARTITION_MEMORY_EVENTS_INFORMATION_1_1 {
    #[inline]
    pub fn CommitEvents(&self) -> u32 {
        self._bitfield_1.get(0usize, 1u8) as u32
    }

    #[inline]
    pub fn set_CommitEvents(&mut self, val: u32) {
        self._bitfield_1.set(0usize, 1u8, val as u64)
    }

    #[inline]
    pub fn Spare(&self) -> u32 {
        self._bitfield_1.get(1usize, 31u8) as u32
    }

    #[inline]
    pub fn set_Spare(&mut self, val: u32) {
        self._bitfield_1.set(1usize, 31u8, val as u64)
    }

    #[inline]
    pub fn new_bitfield_1(CommitEvents: u32, Spare: u32) -> BitfieldUnit<[u8; 4]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 4]> = Default::default();

        bitfield_unit.set(0usize, 1u8, CommitEvents as u64);

        bitfield_unit.set(1usize, 31u8, Spare as u64);

        bitfield_unit
    }
}

impl Default for MEMORY_PARTITION_MEMORY_EVENTS_INFORMATION_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for MEMORY_PARTITION_MEMORY_EVENTS_INFORMATION_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MEMORY_PARTITION_MEMORY_EVENTS_INFORMATION_1 {{ union }}"
        )
    }
}

impl Default for MEMORY_PARTITION_MEMORY_EVENTS_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for MEMORY_PARTITION_MEMORY_EVENTS_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MEMORY_PARTITION_MEMORY_EVENTS_INFORMATION {{ Flags: {:?} }}",
            self.Flags
        )
    }
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtCreatePartition(
        ParentPartitionHandle: HANDLE,
        PartitionHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        PreferredNode: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtOpenPartition(
        PartitionHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtMapUserPhysicalPages(
        VirtualAddress: *mut std::ffi::c_void,
        NumberOfPages: usize,
        UserPfnArray: *mut usize,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtMapUserPhysicalPagesScatter(
        VirtualAddresses: *mut *mut std::ffi::c_void,
        NumberOfPages: usize,
        UserPfnArray: *mut usize,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtAllocateUserPhysicalPages(
        ProcessHandle: HANDLE,
        NumberOfPages: *mut usize,
        UserPfnArray: *mut usize,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtAllocateUserPhysicalPagesEx(
        ProcessHandle: HANDLE,
        NumberOfPages: *mut usize,
        UserPfnArray: *mut usize,
        ExtendedParameters: *mut MEM_EXTENDED_PARAMETER,
        ExtendedParameterCount: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtFreeUserPhysicalPages(
        ProcessHandle: HANDLE,
        NumberOfPages: *mut usize,
        UserPfnArray: *mut usize,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtGetWriteWatch(
        ProcessHandle: HANDLE,
        Flags: u32,
        BaseAddress: *mut std::ffi::c_void,
        RegionSize: usize,
        UserAddressArray: *mut *mut std::ffi::c_void,
        EntriesInUserAddressArray: *mut usize,
        Granularity: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtResetWriteWatch(
        ProcessHandle: HANDLE,
        BaseAddress: *mut std::ffi::c_void,
        RegionSize: usize,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtCreatePagingFile(
        PageFileName: *mut UNICODE_STRING,
        MinimumSize: *mut i64,
        MaximumSize: *mut i64,
        Priority: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtFlushInstructionCache(
        ProcessHandle: HANDLE,
        BaseAddress: *mut std::ffi::c_void,
        Length: usize,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtFlushWriteBuffer() -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtCreateEnclave(
        ProcessHandle: HANDLE,
        BaseAddress: *mut *mut std::ffi::c_void,
        ZeroBits: usize,
        Size: usize,
        InitialCommitment: usize,
        EnclaveType: u32,
        EnclaveInformation: *mut std::ffi::c_void,
        EnclaveInformationLength: u32,
        EnclaveError: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtLoadEnclaveData(
        ProcessHandle: HANDLE,
        BaseAddress: *mut std::ffi::c_void,
        Buffer: *mut std::ffi::c_void,
        BufferSize: usize,
        Protect: u32,
        PageInformation: *mut std::ffi::c_void,
        PageInformationLength: u32,
        NumberOfBytesWritten: *mut usize,
        EnclaveError: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtInitializeEnclave(
        ProcessHandle: HANDLE,
        BaseAddress: *mut std::ffi::c_void,
        EnclaveInformation: *mut std::ffi::c_void,
        EnclaveInformationLength: u32,
        EnclaveError: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtTerminateEnclave(
        BaseAddress: *mut std::ffi::c_void,
        WaitForThread: bool,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtCallEnclave(
        Routine: PENCLAVE_ROUTINE,
        Parameter: *mut std::ffi::c_void,
        WaitForThread: bool,
        ReturnValue: *mut *mut std::ffi::c_void,
    ) -> NTSTATUS;
}
