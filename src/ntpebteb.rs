use windows::{
    core::{GUID, PSTR},
    Win32::{
        Foundation::{HANDLE, NTSTATUS, UNICODE_STRING},
        System::{
            Kernel::{LIST_ENTRY, NT_TIB, PROCESSOR_NUMBER, SLIST_HEADER},
            Threading::CRITICAL_SECTION,
            WindowsProgramming::CLIENT_ID,
        },
    },
};

use crate::{
    bitfield::{BitfieldUnit, UnionField},
    ntpsapi::{PEB_LDR_DATA, SILO_USER_SHARED_DATA},
    ntrtl::RTL_USER_PROCESS_PARAMETERS,
    ntsxs::{ACTIVATION_CONTEXT_DATA, ACTIVATION_CONTEXT_STACK, ASSEMBLY_STORAGE_MAP},
};

pub const GDI_BATCH_BUFFER_SIZE: u32 = 310;

#[repr(C)]
pub struct API_SET_NAMESPACE {
    pub Version: u32,
    pub Size: u32,
    pub Flags: u32,
    pub Count: u32,
    pub EntryOffset: u32,
    pub HashOffset: u32,
    pub HashFactor: u32,
}

impl Default for API_SET_NAMESPACE {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for API_SET_NAMESPACE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "API_SET_NAMESPACE {{  }}")
    }
}

#[repr(C)]
pub struct API_SET_HASH_ENTRY {
    pub Hash: u32,
    pub Index: u32,
}

impl Default for API_SET_HASH_ENTRY {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for API_SET_HASH_ENTRY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "API_SET_HASH_ENTRY {{  }}")
    }
}

#[repr(C)]
pub struct API_SET_NAMESPACE_ENTRY {
    pub Flags: u32,
    pub NameOffset: u32,
    pub NameLength: u32,
    pub HashedLength: u32,
    pub ValueOffset: u32,
    pub ValueCount: u32,
}

impl Default for API_SET_NAMESPACE_ENTRY {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for API_SET_NAMESPACE_ENTRY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "API_SET_NAMESPACE_ENTRY {{  }}")
    }
}

#[repr(C)]
pub struct API_SET_VALUE_ENTRY {
    pub Flags: u32,
    pub NameOffset: u32,
    pub NameLength: u32,
    pub ValueOffset: u32,
    pub ValueLength: u32,
}

impl Default for API_SET_VALUE_ENTRY {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for API_SET_VALUE_ENTRY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "API_SET_VALUE_ENTRY {{  }}")
    }
}

#[repr(C)]
pub struct PEB {
    pub InheritedAddressSpace: bool,
    pub ReadImageFileExecOptions: bool,
    pub BeingDebugged: bool,
    pub Anonymous1: PEB_1,
    pub Mutant: HANDLE,
    pub ImageBaseAddress: *mut std::ffi::c_void,
    pub Ldr: *mut PEB_LDR_DATA,
    pub ProcessParameters: *mut RTL_USER_PROCESS_PARAMETERS,
    pub SubSystemData: *mut std::ffi::c_void,
    pub ProcessHeap: *mut std::ffi::c_void,
    pub FastPebLock: *mut CRITICAL_SECTION,
    pub AtlThunkSListPtr: *mut SLIST_HEADER,
    pub IFEOKey: *mut std::ffi::c_void,
    pub Anonymous2: PEB_2,
    pub Anonymous3: PEB_3,
    pub SystemReserved: u32,
    pub AtlThunkSListPtr32: u32,
    pub ApiSetMap: *mut API_SET_NAMESPACE,
    pub TlsExpansionCounter: u32,
    pub TlsBitmap: *mut std::ffi::c_void,
    pub TlsBitmapBits: [u32; 2],
    pub ReadOnlySharedMemoryBase: *mut std::ffi::c_void,
    pub SharedData: *mut SILO_USER_SHARED_DATA,
    pub ReadOnlyStaticServerData: *mut *mut std::ffi::c_void,
    pub AnsiCodePageData: *mut std::ffi::c_void,
    pub OemCodePageData: *mut std::ffi::c_void,
    pub UnicodeCaseTableData: *mut std::ffi::c_void,
    pub NumberOfProcessors: u32,
    pub NtGlobalFlag: u32,
    pub CriticalSectionTimeout: u64,
    pub HeapSegmentReserve: usize,
    pub HeapSegmentCommit: usize,
    pub HeapDeCommitTotalFreeThreshold: usize,
    pub HeapDeCommitFreeBlockThreshold: usize,
    pub NumberOfHeaps: u32,
    pub MaximumNumberOfHeaps: u32,
    pub ProcessHeaps: *mut *mut std::ffi::c_void,
    pub GdiSharedHandleTable: *mut std::ffi::c_void,
    pub ProcessStarterHelper: *mut std::ffi::c_void,
    pub GdiDCAttributeList: u32,
    pub LoaderLock: *mut CRITICAL_SECTION,
    pub OSMajorVersion: u32,
    pub OSMinorVersion: u32,
    pub OSBuildNumber: u16,
    pub OSCSDVersion: u16,
    pub OSPlatformId: u32,
    pub ImageSubsystem: u32,
    pub ImageSubsystemMajorVersion: u32,
    pub ImageSubsystemMinorVersion: u32,
    pub ActiveProcessAffinityMask: usize,
    pub GdiHandleBuffer: [u32; 60],
    pub PostProcessInitRoutine: *mut std::ffi::c_void,
    pub TlsExpansionBitmap: *mut std::ffi::c_void,
    pub TlsExpansionBitmapBits: [u32; 32],
    pub SessionId: u32,
    pub AppCompatFlags: u64,
    pub AppCompatFlagsUser: u64,
    pub pShimData: *mut std::ffi::c_void,
    pub AppCompatInfo: *mut std::ffi::c_void,
    pub CSDVersion: UNICODE_STRING,
    pub ActivationContextData: *mut ACTIVATION_CONTEXT_DATA,
    pub ProcessAssemblyStorageMap: *mut ASSEMBLY_STORAGE_MAP,
    pub SystemDefaultActivationContextData: *mut ACTIVATION_CONTEXT_DATA,
    pub SystemAssemblyStorageMap: *mut ASSEMBLY_STORAGE_MAP,
    pub MinimumStackCommit: usize,
    pub SparePointers: [*mut std::ffi::c_void; 2],
    pub PatchLoaderData: *mut std::ffi::c_void,
    pub ChpeV2ProcessInfo: *mut std::ffi::c_void,
    pub AppModelFeatureState: u32,
    pub SpareUlongs: [u32; 2],
    pub ActiveCodePage: u16,
    pub OemCodePage: u16,
    pub UseCaseMapping: u16,
    pub UnusedNlsField: u16,
    pub WerRegistrationData: *mut std::ffi::c_void,
    pub WerShipAssertPtr: *mut std::ffi::c_void,
    pub Anonymous4: PEB_4,
    pub pImageHeaderHash: *mut std::ffi::c_void,
    pub Anonymous5: PEB_5,
    pub CsrServerReadOnlySharedMemoryBase: u64,
    pub TppWorkerpListLock: *mut CRITICAL_SECTION,
    pub TppWorkerpList: LIST_ENTRY,
    pub WaitOnAddressHashTable: [*mut std::ffi::c_void; 128],
    pub TelemetryCoverageHeader: *mut std::ffi::c_void,
    pub CloudFileFlags: u32,
    pub CloudFileDiagFlags: u32,
    pub PlaceholderCompatibilityMode: i8,
    pub PlaceholderCompatibilityModeReserved: [i8; 7],
    pub LeapSecondData: *mut std::ffi::c_void,
    pub Anonymous6: PEB_6,
    pub NtGlobalFlag2: u32,
    pub ExtendedFeatureDisableMask: u64,
}

#[repr(C)]
pub struct PEB_1 {
    pub BitField: UnionField<bool>,
    pub Anonymous1: UnionField<PEB_1_1>,
    pub union_field: u8,
}

#[repr(C, packed)]
pub struct PEB_1_1 {
    _bitfield_align_1: [u8; 0],
    _bitfield_1: BitfieldUnit<[u8; 1]>,
}

impl Default for PEB_1_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PEB_1_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PEB_1_1 {{ ImageUsesLargePages : {:?}, IsProtectedProcess : {:?}, IsImageDynamicallyRelocated : {:?}, SkipPatchingUser32Forwarders : {:?}, IsPackagedProcess : {:?}, IsAppContainer : {:?}, IsProtectedProcessLight : {:?}, IsLongPathAwareProcess : {:?} }}",
            self.ImageUsesLargePages(),
            self.IsProtectedProcess(),
            self.IsImageDynamicallyRelocated(),
            self.SkipPatchingUser32Forwarders(),
            self.IsPackagedProcess(),
            self.IsAppContainer(),
            self.IsProtectedProcessLight(),
            self.IsLongPathAwareProcess()
        )
    }
}

impl PEB_1_1 {
    #[inline]
    pub fn ImageUsesLargePages(&self) -> bool {
        unsafe { std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u8) }
    }

    #[inline]
    pub fn set_ImageUsesLargePages(&mut self, val: bool) {
        self._bitfield_1.set(0usize, 1u8, val as u64)
    }

    #[inline]
    pub fn IsProtectedProcess(&self) -> bool {
        unsafe { std::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u8) }
    }

    #[inline]
    pub fn set_IsProtectedProcess(&mut self, val: bool) {
        self._bitfield_1.set(1usize, 1u8, val as u64)
    }

    #[inline]
    pub fn IsImageDynamicallyRelocated(&self) -> bool {
        unsafe { std::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u8) }
    }

    #[inline]
    pub fn set_IsImageDynamicallyRelocated(&mut self, val: bool) {
        self._bitfield_1.set(2usize, 1u8, val as u64)
    }

    #[inline]
    pub fn SkipPatchingUser32Forwarders(&self) -> bool {
        unsafe { std::mem::transmute(self._bitfield_1.get(3usize, 1u8) as u8) }
    }

    #[inline]
    pub fn set_SkipPatchingUser32Forwarders(&mut self, val: bool) {
        self._bitfield_1.set(3usize, 1u8, val as u64)
    }

    #[inline]
    pub fn IsPackagedProcess(&self) -> bool {
        unsafe { std::mem::transmute(self._bitfield_1.get(4usize, 1u8) as u8) }
    }

    #[inline]
    pub fn set_IsPackagedProcess(&mut self, val: bool) {
        self._bitfield_1.set(4usize, 1u8, val as u64)
    }

    #[inline]
    pub fn IsAppContainer(&self) -> bool {
        unsafe { std::mem::transmute(self._bitfield_1.get(5usize, 1u8) as u8) }
    }

    #[inline]
    pub fn set_IsAppContainer(&mut self, val: bool) {
        self._bitfield_1.set(5usize, 1u8, val as u64)
    }

    #[inline]
    pub fn IsProtectedProcessLight(&self) -> bool {
        unsafe { std::mem::transmute(self._bitfield_1.get(6usize, 1u8) as u8) }
    }

    #[inline]
    pub fn set_IsProtectedProcessLight(&mut self, val: bool) {
        self._bitfield_1.set(6usize, 1u8, val as u64)
    }

    #[inline]
    pub fn IsLongPathAwareProcess(&self) -> bool {
        unsafe { std::mem::transmute(self._bitfield_1.get(7usize, 1u8) as u8) }
    }

    #[inline]
    pub fn set_IsLongPathAwareProcess(&mut self, val: bool) {
        self._bitfield_1.set(7usize, 1u8, val as u64)
    }

    #[inline]
    pub fn new_bitfield_1(
        ImageUsesLargePages: bool,
        IsProtectedProcess: bool,
        IsImageDynamicallyRelocated: bool,
        SkipPatchingUser32Forwarders: bool,
        IsPackagedProcess: bool,
        IsAppContainer: bool,
        IsProtectedProcessLight: bool,
        IsLongPathAwareProcess: bool,
    ) -> BitfieldUnit<[u8; 1]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 1]> = Default::default();

        bitfield_unit.set(0usize, 1u8, {
            ImageUsesLargePages as u64
        });

        bitfield_unit.set(1usize, 1u8, {
            IsProtectedProcess as u64
        });

        bitfield_unit.set(2usize, 1u8, {
            IsImageDynamicallyRelocated as u64
        });

        bitfield_unit.set(3usize, 1u8, {
            SkipPatchingUser32Forwarders as u64
        });

        bitfield_unit.set(4usize, 1u8, {
            IsPackagedProcess as u64
        });

        bitfield_unit.set(5usize, 1u8, {
            IsAppContainer as u64
        });

        bitfield_unit.set(6usize, 1u8, {
            IsProtectedProcessLight as u64
        });

        bitfield_unit.set(7usize, 1u8, {
            IsLongPathAwareProcess as u64
        });

        bitfield_unit
    }
}

impl Default for PEB_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PEB_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PEB_1 {{ union }}")
    }
}

#[repr(C)]
pub struct PEB_2 {
    pub CrossProcessFlags: UnionField<u32>,
    pub Anonymous1: UnionField<PEB_2_1>,
    pub union_field: u32,
}

#[repr(C)]
#[repr(align(4))]
pub struct PEB_2_1 {
    _bitfield_align_1: [u32; 0],
    _bitfield_1: BitfieldUnit<[u8; 4]>,
}

impl Default for PEB_2_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PEB_2_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PEB_2_1 {{ ProcessInJob : {:?}, ProcessInitializing : {:?}, ProcessUsingVEH : {:?}, ProcessUsingVCH : {:?}, ProcessUsingFTH : {:?}, ProcessPreviouslyThrottled : {:?}, ProcessCurrentlyThrottled : {:?}, ProcessImagesHotPatched : {:?}, ReservedBits0 : {:?} }}",
            self.ProcessInJob(),
            self.ProcessInitializing(),
            self.ProcessUsingVEH(),
            self.ProcessUsingVCH(),
            self.ProcessUsingFTH(),
            self.ProcessPreviouslyThrottled(),
            self.ProcessCurrentlyThrottled(),
            self.ProcessImagesHotPatched(),
            self.ReservedBits0()
        )
    }
}

impl PEB_2_1 {
    #[inline]
    pub fn ProcessInJob(&self) -> u32 {
        self._bitfield_1.get(0usize, 1u8) as u32
    }

    #[inline]
    pub fn set_ProcessInJob(&mut self, val: u32) {
        self._bitfield_1.set(0usize, 1u8, val as u64)
    }

    #[inline]
    pub fn ProcessInitializing(&self) -> u32 {
        self._bitfield_1.get(1usize, 1u8) as u32
    }

    #[inline]
    pub fn set_ProcessInitializing(&mut self, val: u32) {
        self._bitfield_1.set(1usize, 1u8, val as u64)
    }

    #[inline]
    pub fn ProcessUsingVEH(&self) -> u32 {
        self._bitfield_1.get(2usize, 1u8) as u32
    }

    #[inline]
    pub fn set_ProcessUsingVEH(&mut self, val: u32) {
        self._bitfield_1.set(2usize, 1u8, val as u64)
    }

    #[inline]
    pub fn ProcessUsingVCH(&self) -> u32 {
        self._bitfield_1.get(3usize, 1u8) as u32
    }

    #[inline]
    pub fn set_ProcessUsingVCH(&mut self, val: u32) {
        self._bitfield_1.set(3usize, 1u8, val as u64)
    }

    #[inline]
    pub fn ProcessUsingFTH(&self) -> u32 {
        self._bitfield_1.get(4usize, 1u8) as u32
    }

    #[inline]
    pub fn set_ProcessUsingFTH(&mut self, val: u32) {
        self._bitfield_1.set(4usize, 1u8, val as u64)
    }

    #[inline]
    pub fn ProcessPreviouslyThrottled(&self) -> u32 {
        self._bitfield_1.get(5usize, 1u8) as u32
    }

    #[inline]
    pub fn set_ProcessPreviouslyThrottled(&mut self, val: u32) {
        self._bitfield_1.set(5usize, 1u8, val as u64)
    }

    #[inline]
    pub fn ProcessCurrentlyThrottled(&self) -> u32 {
        self._bitfield_1.get(6usize, 1u8) as u32
    }

    #[inline]
    pub fn set_ProcessCurrentlyThrottled(&mut self, val: u32) {
        self._bitfield_1.set(6usize, 1u8, val as u64)
    }

    #[inline]
    pub fn ProcessImagesHotPatched(&self) -> u32 {
        self._bitfield_1.get(7usize, 1u8) as u32
    }

    #[inline]
    pub fn set_ProcessImagesHotPatched(&mut self, val: u32) {
        self._bitfield_1.set(7usize, 1u8, val as u64)
    }

    #[inline]
    pub fn ReservedBits0(&self) -> u32 {
        self._bitfield_1.get(8usize, 24u8) as u32
    }

    #[inline]
    pub fn set_ReservedBits0(&mut self, val: u32) {
        self._bitfield_1.set(8usize, 24u8, val as u64)
    }

    #[inline]
    pub fn new_bitfield_1(
        ProcessInJob: u32,
        ProcessInitializing: u32,
        ProcessUsingVEH: u32,
        ProcessUsingVCH: u32,
        ProcessUsingFTH: u32,
        ProcessPreviouslyThrottled: u32,
        ProcessCurrentlyThrottled: u32,
        ProcessImagesHotPatched: u32,
        ReservedBits0: u32,
    ) -> BitfieldUnit<[u8; 4]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 4]> = Default::default();

        bitfield_unit.set(0usize, 1u8, ProcessInJob as u64);

        bitfield_unit.set(1usize, 1u8, ProcessInitializing as u64);

        bitfield_unit.set(2usize, 1u8, ProcessUsingVEH as u64);

        bitfield_unit.set(3usize, 1u8, ProcessUsingVCH as u64);

        bitfield_unit.set(4usize, 1u8, ProcessUsingFTH as u64);

        bitfield_unit.set(5usize, 1u8, ProcessPreviouslyThrottled as u64);

        bitfield_unit.set(6usize, 1u8, ProcessCurrentlyThrottled as u64);

        bitfield_unit.set(7usize, 1u8, ProcessImagesHotPatched as u64);

        bitfield_unit.set(8usize, 24u8, ReservedBits0 as u64);

        bitfield_unit
    }
}

impl Default for PEB_2 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PEB_2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PEB_2 {{ union }}")
    }
}

#[repr(C)]
pub struct PEB_3 {
    pub KernelCallbackTable: UnionField<*mut std::ffi::c_void>,
    pub UserSharedInfoPtr: UnionField<*mut std::ffi::c_void>,
    pub union_field: u64,
}

impl Default for PEB_3 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PEB_3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PEB_3 {{ union }}")
    }
}

#[repr(C)]
pub struct PEB_4 {
    pub pContextData: UnionField<*mut std::ffi::c_void>,
    pub pUnused: UnionField<*mut std::ffi::c_void>,
    pub EcCodeBitMap: UnionField<*mut std::ffi::c_void>,
    pub union_field: u64,
}

impl Default for PEB_4 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PEB_4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PEB_4 {{ union }}")
    }
}

#[repr(C)]
pub struct PEB_5 {
    pub TracingFlags: UnionField<u32>,
    pub Anonymous1: UnionField<PEB_5_1>,
    pub union_field: u32,
}

#[repr(C)]
#[repr(align(4))]
pub struct PEB_5_1 {
    _bitfield_align_1: [u32; 0],
    _bitfield_1: BitfieldUnit<[u8; 4]>,
}

impl Default for PEB_5_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PEB_5_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PEB_5_1 {{ HeapTracingEnabled : {:?}, CritSecTracingEnabled : {:?}, LibLoaderTracingEnabled : {:?}, SpareTracingBits : {:?} }}",
            self.HeapTracingEnabled(),
            self.CritSecTracingEnabled(),
            self.LibLoaderTracingEnabled(),
            self.SpareTracingBits()
        )
    }
}

impl PEB_5_1 {
    #[inline]
    pub fn HeapTracingEnabled(&self) -> u32 {
        self._bitfield_1.get(0usize, 1u8) as u32
    }

    #[inline]
    pub fn set_HeapTracingEnabled(&mut self, val: u32) {
        self._bitfield_1.set(0usize, 1u8, val as u64)
    }

    #[inline]
    pub fn CritSecTracingEnabled(&self) -> u32 {
        self._bitfield_1.get(1usize, 1u8) as u32
    }

    #[inline]
    pub fn set_CritSecTracingEnabled(&mut self, val: u32) {
        self._bitfield_1.set(1usize, 1u8, val as u64)
    }

    #[inline]
    pub fn LibLoaderTracingEnabled(&self) -> u32 {
        self._bitfield_1.get(2usize, 1u8) as u32
    }

    #[inline]
    pub fn set_LibLoaderTracingEnabled(&mut self, val: u32) {
        self._bitfield_1.set(2usize, 1u8, val as u64)
    }

    #[inline]
    pub fn SpareTracingBits(&self) -> u32 {
        self._bitfield_1.get(3usize, 29u8) as u32
    }

    #[inline]
    pub fn set_SpareTracingBits(&mut self, val: u32) {
        self._bitfield_1.set(3usize, 29u8, val as u64)
    }

    #[inline]
    pub fn new_bitfield_1(
        HeapTracingEnabled: u32,
        CritSecTracingEnabled: u32,
        LibLoaderTracingEnabled: u32,
        SpareTracingBits: u32,
    ) -> BitfieldUnit<[u8; 4]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 4]> = Default::default();

        bitfield_unit.set(0usize, 1u8, HeapTracingEnabled as u64);

        bitfield_unit.set(1usize, 1u8, CritSecTracingEnabled as u64);

        bitfield_unit.set(2usize, 1u8, LibLoaderTracingEnabled as u64);

        bitfield_unit.set(3usize, 29u8, SpareTracingBits as u64);

        bitfield_unit
    }
}

impl Default for PEB_5 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PEB_5 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PEB_5 {{ union }}")
    }
}

#[repr(C)]
pub struct PEB_6 {
    pub LeapSecondFlags: UnionField<u32>,
    pub Anonymous1: UnionField<PEB_6_1>,
    pub union_field: u32,
}

#[repr(C)]
#[repr(align(4))]
pub struct PEB_6_1 {
    _bitfield_align_1: [u32; 0],
    _bitfield_1: BitfieldUnit<[u8; 4]>,
}

impl Default for PEB_6_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PEB_6_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PEB_6_1 {{ SixtySecondEnabled : {:?}, Reserved : {:?} }}",
            self.SixtySecondEnabled(),
            self.Reserved()
        )
    }
}

impl PEB_6_1 {
    #[inline]
    pub fn SixtySecondEnabled(&self) -> u32 {
        self._bitfield_1.get(0usize, 1u8) as u32
    }

    #[inline]
    pub fn set_SixtySecondEnabled(&mut self, val: u32) {
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
    pub fn new_bitfield_1(SixtySecondEnabled: u32, Reserved: u32) -> BitfieldUnit<[u8; 4]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 4]> = Default::default();

        bitfield_unit.set(0usize, 1u8, SixtySecondEnabled as u64);

        bitfield_unit.set(1usize, 31u8, Reserved as u64);

        bitfield_unit
    }
}

impl Default for PEB_6 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PEB_6 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PEB_6 {{ union }}")
    }
}

impl Default for PEB {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PEB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PEB {{ Anonymous1: {:?}, ProcessParameters: {:?}, Anonymous2: {:?}, Anonymous3: {:?}, ApiSetMap: {:?}, TlsBitmapBits: {:?}, SharedData: {:?}, ReadOnlyStaticServerData: {:?}, ProcessHeaps: {:?}, TlsExpansionBitmapBits: {:?}, SparePointers: {:?}, SpareUlongs: {:?}, Anonymous4: {:?}, Anonymous5: {:?}, WaitOnAddressHashTable: {:?}, PlaceholderCompatibilityModeReserved: {:?}, LeapSecondData: {:?}, Anonymous6: {:?} }}",
            self.Anonymous1,
            self.ProcessParameters,
            self.Anonymous2,
            self.Anonymous3,
            self.ApiSetMap,
            self.TlsBitmapBits,
            self.SharedData,
            self.ReadOnlyStaticServerData,
            self.ProcessHeaps,
            self.TlsExpansionBitmapBits,
            self.SparePointers,
            self.SpareUlongs,
            self.Anonymous4,
            self.Anonymous5,
            self.WaitOnAddressHashTable,
            self.PlaceholderCompatibilityModeReserved,
            self.LeapSecondData,
            self.Anonymous6
        )
    }
}

#[repr(C)]
pub struct GDI_TEB_BATCH {
    pub Offset: u32,
    pub HDC: usize,
    pub Buffer: [u32; 310],
}

impl Default for GDI_TEB_BATCH {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for GDI_TEB_BATCH {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GDI_TEB_BATCH {{ Buffer: {:?} }}", self.Buffer)
    }
}

#[repr(C)]
pub struct TEB_ACTIVE_FRAME_CONTEXT {
    pub Flags: u32,
    pub FrameName: PSTR,
}

impl Default for TEB_ACTIVE_FRAME_CONTEXT {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for TEB_ACTIVE_FRAME_CONTEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TEB_ACTIVE_FRAME_CONTEXT {{  }}")
    }
}

#[repr(C)]
pub struct TEB_ACTIVE_FRAME {
    pub Flags: u32,
    pub Previous: *mut TEB_ACTIVE_FRAME,
    pub Context: *mut TEB_ACTIVE_FRAME_CONTEXT,
}

impl Default for TEB_ACTIVE_FRAME {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for TEB_ACTIVE_FRAME {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TEB_ACTIVE_FRAME {{ Previous: {:?}, Context: {:?} }}",
            self.Previous, self.Context
        )
    }
}

#[repr(C)]
pub struct TEB {
    pub NtTib: NT_TIB,
    pub EnvironmentPointer: *mut std::ffi::c_void,
    pub ClientId: CLIENT_ID,
    pub ActiveRpcHandle: *mut std::ffi::c_void,
    pub ThreadLocalStoragePointer: *mut std::ffi::c_void,
    pub ProcessEnvironmentBlock: *mut PEB,
    pub LastErrorValue: u32,
    pub CountOfOwnedCriticalSections: u32,
    pub CsrClientThread: *mut std::ffi::c_void,
    pub Win32ThreadInfo: *mut std::ffi::c_void,
    pub User32Reserved: [u32; 26],
    pub UserReserved: [u32; 5],
    pub WOW32Reserved: *mut std::ffi::c_void,
    pub CurrentLocale: u32,
    pub FpSoftwareStatusRegister: u32,
    pub ReservedForDebuggerInstrumentation: [*mut std::ffi::c_void; 16],
    pub SystemReserved1: [*mut std::ffi::c_void; 30],
    pub PlaceholderCompatibilityMode: i8,
    pub PlaceholderHydrationAlwaysExplicit: bool,
    pub PlaceholderReserved: [i8; 10],
    pub ProxiedProcessId: u32,
    pub ActivationStack: ACTIVATION_CONTEXT_STACK,
    pub WorkingOnBehalfTicket: [u8; 8],
    pub ExceptionCode: NTSTATUS,
    pub ActivationContextStackPointer: *mut ACTIVATION_CONTEXT_STACK,
    pub InstrumentationCallbackSp: usize,
    pub InstrumentationCallbackPreviousPc: usize,
    pub InstrumentationCallbackPreviousSp: usize,
    pub TxFsContext: u32,
    pub InstrumentationCallbackDisabled: bool,
    pub UnalignedLoadStoreExceptions: bool,
    pub GdiTebBatch: GDI_TEB_BATCH,
    pub RealClientId: CLIENT_ID,
    pub GdiCachedProcessHandle: HANDLE,
    pub GdiClientPID: u32,
    pub GdiClientTID: u32,
    pub GdiThreadLocalInfo: *mut std::ffi::c_void,
    pub Win32ClientInfo: [usize; 62],
    pub glDispatchTable: [*mut std::ffi::c_void; 233],
    pub glReserved1: [usize; 29],
    pub glReserved2: *mut std::ffi::c_void,
    pub glSectionInfo: *mut std::ffi::c_void,
    pub glSection: *mut std::ffi::c_void,
    pub glTable: *mut std::ffi::c_void,
    pub glCurrentRC: *mut std::ffi::c_void,
    pub glContext: *mut std::ffi::c_void,
    pub LastStatusValue: NTSTATUS,
    pub StaticUnicodeString: UNICODE_STRING,
    pub StaticUnicodeBuffer: [u16; 261],
    pub DeallocationStack: *mut std::ffi::c_void,
    pub TlsSlots: [*mut std::ffi::c_void; 64],
    pub TlsLinks: LIST_ENTRY,
    pub Vdm: *mut std::ffi::c_void,
    pub ReservedForNtRpc: *mut std::ffi::c_void,
    pub DbgSsReserved: [*mut std::ffi::c_void; 2],
    pub HardErrorMode: u32,
    pub Instrumentation: [*mut std::ffi::c_void; 11],
    pub ActivityId: GUID,
    pub SubProcessTag: *mut std::ffi::c_void,
    pub PerflibData: *mut std::ffi::c_void,
    pub EtwTraceData: *mut std::ffi::c_void,
    pub WinSockData: *mut std::ffi::c_void,
    pub GdiBatchCount: u32,
    pub Anonymous1: TEB_1,
    pub GuaranteedStackBytes: u32,
    pub ReservedForPerf: *mut std::ffi::c_void,
    pub ReservedForOle: *mut std::ffi::c_void,
    pub WaitingOnLoaderLock: u32,
    pub SavedPriorityState: *mut std::ffi::c_void,
    pub ReservedForCodeCoverage: usize,
    pub ThreadPoolData: *mut std::ffi::c_void,
    pub TlsExpansionSlots: *mut *mut std::ffi::c_void,
    pub DeallocationBStore: *mut std::ffi::c_void,
    pub BStoreLimit: *mut std::ffi::c_void,
    pub MuiGeneration: u32,
    pub IsImpersonating: u32,
    pub NlsCache: *mut std::ffi::c_void,
    pub pShimData: *mut std::ffi::c_void,
    pub HeapData: u32,
    pub CurrentTransactionHandle: HANDLE,
    pub ActiveFrame: *mut TEB_ACTIVE_FRAME,
    pub FlsData: *mut std::ffi::c_void,
    pub PreferredLanguages: *mut std::ffi::c_void,
    pub UserPrefLanguages: *mut std::ffi::c_void,
    pub MergedPrefLanguages: *mut std::ffi::c_void,
    pub MuiImpersonation: u32,
    pub Anonymous2: TEB_2,
    pub Anonymous3: TEB_3,
    pub TxnScopeEnterCallback: *mut std::ffi::c_void,
    pub TxnScopeExitCallback: *mut std::ffi::c_void,
    pub TxnScopeContext: *mut std::ffi::c_void,
    pub LockCount: u32,
    pub WowTebOffset: i32,
    pub ResourceRetValue: *mut std::ffi::c_void,
    pub ReservedForWdf: *mut std::ffi::c_void,
    pub ReservedForCrt: u64,
    pub EffectiveContainerId: GUID,
    pub LastSleepCounter: u64,
    pub SpinCallCount: u32,
    pub ExtendedFeatureDisableMask: u64,
}

#[repr(C)]
pub struct TEB_1 {
    pub CurrentIdealProcessor: UnionField<PROCESSOR_NUMBER>,
    pub IdealProcessorValue: UnionField<u32>,
    pub Anonymous1: UnionField<TEB_1_1>,
    pub union_field: u32,
}

#[repr(C)]
pub struct TEB_1_1 {
    pub ReservedPad0: u8,
    pub ReservedPad1: u8,
    pub ReservedPad2: u8,
    pub IdealProcessor: u8,
}

impl Default for TEB_1_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for TEB_1_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TEB_1_1 {{  }}")
    }
}

impl Default for TEB_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for TEB_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TEB_1 {{ union }}")
    }
}

#[repr(C)]
pub struct TEB_2 {
    pub CrossTebFlags: UnionField<u16>,
    _bitfield_align_1: [u16; 0],
    _bitfield_1: UnionField<BitfieldUnit<[u8; 2]>>,
    pub union_field: u16,
}

impl Default for TEB_2 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for TEB_2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TEB_2 {{ union }}")
    }
}

impl TEB_2 {
    #[inline]
    pub fn SpareCrossTebBits(&self) -> u16 {
        unsafe { self._bitfield_1.as_ref().get(0usize, 16u8) as u16 }
    }

    #[inline]
    pub fn set_SpareCrossTebBits(&mut self, val: u16) {
        unsafe { self._bitfield_1.as_mut().set(0usize, 16u8, val as u64) }
    }

    #[inline]
    pub fn new_bitfield_1(SpareCrossTebBits: u16) -> BitfieldUnit<[u8; 2]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 2]> = Default::default();

        bitfield_unit.set(0usize, 16u8, SpareCrossTebBits as u64);

        bitfield_unit
    }
}

#[repr(C)]
pub struct TEB_3 {
    pub SameTebFlags: UnionField<u16>,
    pub Anonymous1: UnionField<TEB_3_1>,
    pub union_field: u16,
}

#[repr(C)]
#[repr(align(2))]
pub struct TEB_3_1 {
    _bitfield_align_1: [u8; 0],
    _bitfield_1: BitfieldUnit<[u8; 2]>,
}

impl Default for TEB_3_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for TEB_3_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TEB_3_1 {{ SafeThunkCall : {:?}, InDebugPrint : {:?}, HasFiberData : {:?}, SkipThreadAttach : {:?}, WerInShipAssertCode : {:?}, RanProcessInit : {:?}, ClonedThread : {:?}, SuppressDebugMsg : {:?}, DisableUserStackWalk : {:?}, RtlExceptionAttached : {:?}, InitialThread : {:?}, SessionAware : {:?}, LoadOwner : {:?}, LoaderWorker : {:?}, SkipLoaderInit : {:?}, SkipFileAPIBrokering : {:?} }}",
            self.SafeThunkCall(),
            self.InDebugPrint(),
            self.HasFiberData(),
            self.SkipThreadAttach(),
            self.WerInShipAssertCode(),
            self.RanProcessInit(),
            self.ClonedThread(),
            self.SuppressDebugMsg(),
            self.DisableUserStackWalk(),
            self.RtlExceptionAttached(),
            self.InitialThread(),
            self.SessionAware(),
            self.LoadOwner(),
            self.LoaderWorker(),
            self.SkipLoaderInit(),
            self.SkipFileAPIBrokering()
        )
    }
}

impl TEB_3_1 {
    #[inline]
    pub fn SafeThunkCall(&self) -> u16 {
        self._bitfield_1.get(0usize, 1u8) as u16
    }

    #[inline]
    pub fn set_SafeThunkCall(&mut self, val: u16) {
        self._bitfield_1.set(0usize, 1u8, val as u64)
    }

    #[inline]
    pub fn InDebugPrint(&self) -> u16 {
        self._bitfield_1.get(1usize, 1u8) as u16
    }

    #[inline]
    pub fn set_InDebugPrint(&mut self, val: u16) {
        self._bitfield_1.set(1usize, 1u8, val as u64)
    }

    #[inline]
    pub fn HasFiberData(&self) -> u16 {
        self._bitfield_1.get(2usize, 1u8) as u16
    }

    #[inline]
    pub fn set_HasFiberData(&mut self, val: u16) {
        self._bitfield_1.set(2usize, 1u8, val as u64)
    }

    #[inline]
    pub fn SkipThreadAttach(&self) -> u16 {
        self._bitfield_1.get(3usize, 1u8) as u16
    }

    #[inline]
    pub fn set_SkipThreadAttach(&mut self, val: u16) {
        self._bitfield_1.set(3usize, 1u8, val as u64)
    }

    #[inline]
    pub fn WerInShipAssertCode(&self) -> u16 {
        self._bitfield_1.get(4usize, 1u8) as u16
    }

    #[inline]
    pub fn set_WerInShipAssertCode(&mut self, val: u16) {
        self._bitfield_1.set(4usize, 1u8, val as u64)
    }

    #[inline]
    pub fn RanProcessInit(&self) -> u16 {
        self._bitfield_1.get(5usize, 1u8) as u16
    }

    #[inline]
    pub fn set_RanProcessInit(&mut self, val: u16) {
        self._bitfield_1.set(5usize, 1u8, val as u64)
    }

    #[inline]
    pub fn ClonedThread(&self) -> u16 {
        self._bitfield_1.get(6usize, 1u8) as u16
    }

    #[inline]
    pub fn set_ClonedThread(&mut self, val: u16) {
        self._bitfield_1.set(6usize, 1u8, val as u64)
    }

    #[inline]
    pub fn SuppressDebugMsg(&self) -> u16 {
        self._bitfield_1.get(7usize, 1u8) as u16
    }

    #[inline]
    pub fn set_SuppressDebugMsg(&mut self, val: u16) {
        self._bitfield_1.set(7usize, 1u8, val as u64)
    }

    #[inline]
    pub fn DisableUserStackWalk(&self) -> u16 {
        self._bitfield_1.get(8usize, 1u8) as u16
    }

    #[inline]
    pub fn set_DisableUserStackWalk(&mut self, val: u16) {
        self._bitfield_1.set(8usize, 1u8, val as u64)
    }

    #[inline]
    pub fn RtlExceptionAttached(&self) -> u16 {
        self._bitfield_1.get(9usize, 1u8) as u16
    }

    #[inline]
    pub fn set_RtlExceptionAttached(&mut self, val: u16) {
        self._bitfield_1.set(9usize, 1u8, val as u64)
    }

    #[inline]
    pub fn InitialThread(&self) -> u16 {
        self._bitfield_1.get(10usize, 1u8) as u16
    }

    #[inline]
    pub fn set_InitialThread(&mut self, val: u16) {
        self._bitfield_1.set(10usize, 1u8, val as u64)
    }

    #[inline]
    pub fn SessionAware(&self) -> u16 {
        self._bitfield_1.get(11usize, 1u8) as u16
    }

    #[inline]
    pub fn set_SessionAware(&mut self, val: u16) {
        self._bitfield_1.set(11usize, 1u8, val as u64)
    }

    #[inline]
    pub fn LoadOwner(&self) -> u16 {
        self._bitfield_1.get(12usize, 1u8) as u16
    }

    #[inline]
    pub fn set_LoadOwner(&mut self, val: u16) {
        self._bitfield_1.set(12usize, 1u8, val as u64)
    }

    #[inline]
    pub fn LoaderWorker(&self) -> u16 {
        self._bitfield_1.get(13usize, 1u8) as u16
    }

    #[inline]
    pub fn set_LoaderWorker(&mut self, val: u16) {
        self._bitfield_1.set(13usize, 1u8, val as u64)
    }

    #[inline]
    pub fn SkipLoaderInit(&self) -> u16 {
        self._bitfield_1.get(14usize, 1u8) as u16
    }

    #[inline]
    pub fn set_SkipLoaderInit(&mut self, val: u16) {
        self._bitfield_1.set(14usize, 1u8, val as u64)
    }

    #[inline]
    pub fn SkipFileAPIBrokering(&self) -> u16 {
        self._bitfield_1.get(15usize, 1u8) as u16
    }

    #[inline]
    pub fn set_SkipFileAPIBrokering(&mut self, val: u16) {
        self._bitfield_1.set(15usize, 1u8, val as u64)
    }

    #[inline]
    pub fn new_bitfield_1(
        SafeThunkCall: u16,
        InDebugPrint: u16,
        HasFiberData: u16,
        SkipThreadAttach: u16,
        WerInShipAssertCode: u16,
        RanProcessInit: u16,
        ClonedThread: u16,
        SuppressDebugMsg: u16,
        DisableUserStackWalk: u16,
        RtlExceptionAttached: u16,
        InitialThread: u16,
        SessionAware: u16,
        LoadOwner: u16,
        LoaderWorker: u16,
        SkipLoaderInit: u16,
        SkipFileAPIBrokering: u16,
    ) -> BitfieldUnit<[u8; 2]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 2]> = Default::default();

        bitfield_unit.set(0usize, 1u8, SafeThunkCall as u64);

        bitfield_unit.set(1usize, 1u8, InDebugPrint as u64);

        bitfield_unit.set(2usize, 1u8, HasFiberData as u64);

        bitfield_unit.set(3usize, 1u8, SkipThreadAttach as u64);

        bitfield_unit.set(4usize, 1u8, WerInShipAssertCode as u64);

        bitfield_unit.set(5usize, 1u8, RanProcessInit as u64);

        bitfield_unit.set(6usize, 1u8, ClonedThread as u64);

        bitfield_unit.set(7usize, 1u8, SuppressDebugMsg as u64);

        bitfield_unit.set(8usize, 1u8, DisableUserStackWalk as u64);

        bitfield_unit.set(9usize, 1u8, RtlExceptionAttached as u64);

        bitfield_unit.set(10usize, 1u8, InitialThread as u64);

        bitfield_unit.set(11usize, 1u8, SessionAware as u64);

        bitfield_unit.set(12usize, 1u8, LoadOwner as u64);

        bitfield_unit.set(13usize, 1u8, LoaderWorker as u64);

        bitfield_unit.set(14usize, 1u8, SkipLoaderInit as u64);

        bitfield_unit.set(15usize, 1u8, SkipFileAPIBrokering as u64);

        bitfield_unit
    }
}

impl Default for TEB_3 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for TEB_3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TEB_3 {{ union }}")
    }
}

impl Default for TEB {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for TEB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TEB {{ ProcessEnvironmentBlock: {:?}, User32Reserved: {:?}, UserReserved: {:?}, ReservedForDebuggerInstrumentation: {:?}, SystemReserved1: {:?}, PlaceholderReserved: {:?}, WorkingOnBehalfTicket: {:?}, GdiTebBatch: {:?}, Win32ClientInfo: {:?}, glDispatchTable: {:?}, glReserved1: {:?}, StaticUnicodeBuffer: {:?}, TlsSlots: {:?}, DbgSsReserved: {:?}, Instrumentation: {:?}, Anonymous1: {:?}, TlsExpansionSlots: {:?}, ActiveFrame: {:?}, Anonymous2: {:?}, Anonymous3: {:?} }}",
            self.ProcessEnvironmentBlock,
            self.User32Reserved,
            self.UserReserved,
            self.ReservedForDebuggerInstrumentation,
            self.SystemReserved1,
            self.PlaceholderReserved,
            self.WorkingOnBehalfTicket,
            self.GdiTebBatch,
            self.Win32ClientInfo,
            self.glDispatchTable,
            self.glReserved1,
            self.StaticUnicodeBuffer,
            self.TlsSlots,
            self.DbgSsReserved,
            self.Instrumentation,
            self.Anonymous1,
            self.TlsExpansionSlots,
            self.ActiveFrame,
            self.Anonymous2,
            self.Anonymous3
        )
    }
}
