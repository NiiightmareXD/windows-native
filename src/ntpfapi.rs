use windows::Win32::Foundation::HANDLE;

use crate::{
    bitfield::{BitfieldUnit, UnionField},
    ntexapi::SYSTEM_MEMORY_LIST_INFORMATION,
    ntmmapi::MMPFN_IDENTITY,
};

pub const PF_BOOT_CONTROL_VERSION: u32 = 1;
pub const PREFETCHER_INFORMATION_VERSION: u32 = 23;
pub const PF_PFN_PRIO_REQUEST_VERSION: u32 = 1;
pub const PF_PFN_PRIO_REQUEST_QUERY_MEMORY_LIST: u32 = 1;
pub const PF_PFN_PRIO_REQUEST_VALID_FLAGS: u32 = 1;
pub const PF_PRIVSOURCE_QUERY_REQUEST_VERSION: u32 = 8;
pub const PF_PRIVSOURCE_QUERY_REQUEST_FLAGS_QUERYWSPAGES: u32 = 1;
pub const PF_PRIVSOURCE_QUERY_REQUEST_FLAGS_QUERYCOMPRESSEDPAGES: u32 = 2;
pub const PF_PRIVSOURCE_QUERY_REQUEST_FLAGS_QUERYSKIPPAGES: u32 = 4;
pub const PF_SCENARIO_PHASE_INFO_VERSION: u32 = 4;
pub const PF_ROBUSTNESS_CONTROL_VERSION: u32 = 1;
pub const PF_MEMORY_LIST_INFO_VERSION: u32 = 1;
pub const PF_PHYSICAL_MEMORY_RANGE_INFO_V1_VERSION: u32 = 1;
pub const PF_PHYSICAL_MEMORY_RANGE_INFO_V2_VERSION: u32 = 2;
pub const PF_REPURPOSED_BY_PREFETCH_INFO_VERSION: u32 = 1;
pub const PF_VIRTUAL_QUERY_VERSION: u32 = 1;
pub const PF_MIN_WS_AGE_RATE_CONTROL_VERSION: u32 = 1;
pub const PF_DEPRIORITIZE_OLD_PAGES_VERSION: u32 = 3;
pub const PF_GPU_UTILIZATION_INFO_VERSION: u32 = 1;
pub const SUPERFETCH_INFORMATION_VERSION: u32 = 45;
pub const PREFETCHER_INFORMATION_MAGIC: &[u8; 4] = b"kuhC";
pub const SUPERFETCH_INFORMATION_MAGIC: &[u8; 4] = b"kuhC";

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum PF_BOOT_PHASE_ID {
    PfKernelInitPhase = 0,
    PfBootDriverInitPhase = 90,
    PfSystemDriverInitPhase = 120,
    PfSessionManagerInitPhase = 150,
    PfSMRegistryInitPhase = 180,
    PfVideoInitPhase = 210,
    PfPostVideoInitPhase = 240,
    PfBootAcceptedRegistryInitPhase = 270,
    PfUserShellReadyPhase = 300,
    PfMaxBootPhaseId = 900,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum PF_ENABLE_STATUS {
    PfSvNotSpecified = 0,
    PfSvEnabled = 1,
    PfSvDisabled = 2,
    PfSvMaxEnableStatus = 3,
}

#[repr(C)]
pub struct PF_TRACE_LIMITS {
    pub MaxNumPages: u32,
    pub MaxNumSections: u32,
    pub TimerPeriod: i64,
}

impl Default for PF_TRACE_LIMITS {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PF_TRACE_LIMITS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PF_TRACE_LIMITS {{  }}")
    }
}

#[repr(C)]
pub struct PF_SYSTEM_PREFETCH_PARAMETERS {
    pub EnableStatus: [PF_ENABLE_STATUS; 2],
    pub TraceLimits: [PF_TRACE_LIMITS; 2],
    pub MaxNumActiveTraces: u32,
    pub MaxNumSavedTraces: u32,
    pub RootDirPath: [u16; 32],
    pub HostingApplicationList: [u16; 128],
}

impl Default for PF_SYSTEM_PREFETCH_PARAMETERS {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PF_SYSTEM_PREFETCH_PARAMETERS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PF_SYSTEM_PREFETCH_PARAMETERS {{ EnableStatus: {:?}, TraceLimits: {:?}, RootDirPath: {:?}, HostingApplicationList: {:?} }}",
            self.EnableStatus, self.TraceLimits, self.RootDirPath, self.HostingApplicationList
        )
    }
}

#[repr(C)]
pub struct PF_BOOT_CONTROL {
    pub Version: u32,
    pub DisableBootPrefetching: u32,
}

impl Default for PF_BOOT_CONTROL {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PF_BOOT_CONTROL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PF_BOOT_CONTROL {{  }}")
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum PREFETCHER_INFORMATION_CLASS {
    PrefetcherRetrieveTrace = 1,
    PrefetcherSystemParameters = 2,
    PrefetcherBootPhase = 3,
    PrefetcherSpare1 = 4,
    PrefetcherBootControl = 5,
    PrefetcherScenarioPolicyControl = 6,
    PrefetcherSpare2 = 7,
    PrefetcherAppLaunchScenarioControl = 8,
    PrefetcherInformationMax = 9,
}

#[repr(C)]
pub struct PREFETCHER_INFORMATION {
    pub Version: u32,
    pub Magic: u32,
    pub PrefetcherInformationClass: PREFETCHER_INFORMATION_CLASS,
    pub PrefetcherInformation: *mut std::ffi::c_void,
    pub PrefetcherInformationLength: u32,
}

impl Default for PREFETCHER_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PREFETCHER_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PREFETCHER_INFORMATION {{ PrefetcherInformationClass: {:?} }}",
            self.PrefetcherInformationClass
        )
    }
}

#[repr(C)]
pub struct PF_SYSTEM_SUPERFETCH_PARAMETERS {
    pub EnabledComponents: u32,
    pub BootID: u32,
    pub SavedSectInfoTracesMax: u32,
    pub SavedPageAccessTracesMax: u32,
    pub ScenarioPrefetchTimeoutStandby: u32,
    pub ScenarioPrefetchTimeoutHibernate: u32,
    pub ScenarioPrefetchTimeoutHiberBoot: u32,
}

impl Default for PF_SYSTEM_SUPERFETCH_PARAMETERS {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PF_SYSTEM_SUPERFETCH_PARAMETERS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PF_SYSTEM_SUPERFETCH_PARAMETERS {{  }}")
    }
}

impl PF_EVENT_TYPE {}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum PF_EVENT_TYPE {
    PfEventTypeImageLoad = 0,
    PfEventTypeAppLaunch = 1,
    PfEventTypeStartTrace = 2,
    PfEventTypeEndTrace = 3,
    PfEventTypeTimestamp = 4,
    PfEventTypeOperation = 5,
    PfEventTypeRepurpose = 6,
    PfEventTypeForegroundProcess = 7,
    PfEventTypeTimeRange = 8,
    PfEventTypeUserInput = 9,
    PfEventTypeFileAccess = 10,
    PfEventTypeUnmap = 11,
    PfEventTypeMemInfo = 12,
    PfEventTypeFileDelete = 13,
    PfEventTypeAppExit = 14,
    PfEventTypeSystemTime = 15,
    PfEventTypePower = 16,
    PfEventTypeSessionChange = 17,
    PfEventTypeHardFaultTimeStamp = 18,
    PfEventTypeVirtualFree = 19,
    PfEventTypePerfInfo = 20,
    PfEventTypeProcessSnapshot = 21,
    PfEventTypeUserSnapshot = 22,
    PfEventTypeStreamSequenceNumber = 23,
    PfEventTypeFileTruncate = 24,
    PfEventTypeFileRename = 25,
    PfEventTypeFileCreate = 26,
    PfEventTypeAgCxContext = 27,
    PfEventTypePowerAction = 28,
    PfEventTypeHardFaultTS = 29,
    PfEventTypeRobustInfo = 30,
    PfEventTypeFileDefrag = 31,
    PfEventTypeMax = 32,
}

#[repr(C)]
pub struct PF_LOG_EVENT_DATA {
    _bitfield_align_1: [u32; 0],
    _bitfield_1: BitfieldUnit<[u8; 4]>,
    pub EventData: *mut std::ffi::c_void,
}

impl Default for PF_LOG_EVENT_DATA {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PF_LOG_EVENT_DATA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PF_LOG_EVENT_DATA {{ EventType : {:?}, Flags : {:?}, DataSize : {:?} }}",
            self.EventType(),
            self.Flags(),
            self.DataSize()
        )
    }
}

impl PF_LOG_EVENT_DATA {
    #[inline]
    pub fn EventType(&self) -> u32 {
        self._bitfield_1.get(0usize, 5u8) as u32
    }

    #[inline]
    pub fn set_EventType(&mut self, val: u32) {
        self._bitfield_1.set(0usize, 5u8, val as u64)
    }

    #[inline]
    pub fn Flags(&self) -> u32 {
        self._bitfield_1.get(5usize, 2u8) as u32
    }

    #[inline]
    pub fn set_Flags(&mut self, val: u32) {
        self._bitfield_1.set(5usize, 2u8, val as u64)
    }

    #[inline]
    pub fn DataSize(&self) -> u32 {
        self._bitfield_1.get(7usize, 25u8) as u32
    }

    #[inline]
    pub fn set_DataSize(&mut self, val: u32) {
        self._bitfield_1.set(7usize, 25u8, val as u64)
    }

    #[inline]
    pub fn new_bitfield_1(EventType: u32, Flags: u32, DataSize: u32) -> BitfieldUnit<[u8; 4]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 4]> = Default::default();

        bitfield_unit.set(0usize, 5u8, EventType as u64);

        bitfield_unit.set(5usize, 2u8, Flags as u64);

        bitfield_unit.set(7usize, 25u8, DataSize as u64);

        bitfield_unit
    }
}

#[repr(C)]
pub struct PF_PFN_PRIO_REQUEST {
    pub Version: u32,
    pub RequestFlags: u32,
    pub PfnCount: usize,
    pub MemInfo: SYSTEM_MEMORY_LIST_INFORMATION,
    pub PageData: [MMPFN_IDENTITY; 256],
}

impl Default for PF_PFN_PRIO_REQUEST {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PF_PFN_PRIO_REQUEST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PF_PFN_PRIO_REQUEST {{ PageData: {:?} }}", self.PageData)
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum PFS_PRIVATE_PAGE_SOURCE_TYPE {
    PfsPrivateSourceKernel = 0,
    PfsPrivateSourceSession = 1,
    PfsPrivateSourceProcess = 2,
    PfsPrivateSourceMax = 3,
}

#[repr(C)]
pub struct PFS_PRIVATE_PAGE_SOURCE {
    pub Type: PFS_PRIVATE_PAGE_SOURCE_TYPE,
    pub Anonymous1: PFS_PRIVATE_PAGE_SOURCE_1,
    pub ImagePathHash: u32,
    pub UniqueProcessHash: usize,
}

#[repr(C)]
pub struct PFS_PRIVATE_PAGE_SOURCE_1 {
    pub SessionId: UnionField<u32>,
    pub ProcessId: UnionField<u32>,
    pub union_field: u32,
}

impl Default for PFS_PRIVATE_PAGE_SOURCE_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PFS_PRIVATE_PAGE_SOURCE_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PFS_PRIVATE_PAGE_SOURCE_1 {{ union }}")
    }
}

impl Default for PFS_PRIVATE_PAGE_SOURCE {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PFS_PRIVATE_PAGE_SOURCE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PFS_PRIVATE_PAGE_SOURCE {{ Type: {:?}, Anonymous1: {:?} }}",
            self.Type, self.Anonymous1
        )
    }
}

#[repr(C)]
pub struct PF_PRIVSOURCE_INFO {
    pub DbInfo: PFS_PRIVATE_PAGE_SOURCE,
    pub EProcess: *mut std::ffi::c_void,
    pub WsPrivatePages: usize,
    pub TotalPrivatePages: usize,
    pub SessionID: u32,
    pub ImageName: [i8; 16],
    pub Anonymous1: PF_PRIVSOURCE_INFO_1,
    pub WsTotalPages: usize,
    pub DeepFreezeTimeMs: u32,
    _bitfield_align_1: [u32; 0],
    _bitfield_1: BitfieldUnit<[u8; 4]>,
}

#[repr(C)]
pub struct PF_PRIVSOURCE_INFO_1 {
    pub WsSwapPages: UnionField<usize>,
    pub SessionPagedPoolPages: UnionField<usize>,
    pub StoreSizePages: UnionField<usize>,
    pub union_field: u64,
}

impl Default for PF_PRIVSOURCE_INFO_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PF_PRIVSOURCE_INFO_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PF_PRIVSOURCE_INFO_1 {{ union }}")
    }
}

impl Default for PF_PRIVSOURCE_INFO {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PF_PRIVSOURCE_INFO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PF_PRIVSOURCE_INFO {{ DbInfo: {:?}, ImageName: {:?}, Anonymous1: {:?}, ModernApp : {:?}, DeepFrozen : {:?}, Foreground : {:?}, PerProcessStore : {:?}, Spare : {:?} }}",
            self.DbInfo,
            self.ImageName,
            self.Anonymous1,
            self.ModernApp(),
            self.DeepFrozen(),
            self.Foreground(),
            self.PerProcessStore(),
            self.Spare()
        )
    }
}

impl PF_PRIVSOURCE_INFO {
    #[inline]
    pub fn ModernApp(&self) -> u32 {
        self._bitfield_1.get(0usize, 1u8) as u32
    }

    #[inline]
    pub fn set_ModernApp(&mut self, val: u32) {
        self._bitfield_1.set(0usize, 1u8, val as u64)
    }

    #[inline]
    pub fn DeepFrozen(&self) -> u32 {
        self._bitfield_1.get(1usize, 1u8) as u32
    }

    #[inline]
    pub fn set_DeepFrozen(&mut self, val: u32) {
        self._bitfield_1.set(1usize, 1u8, val as u64)
    }

    #[inline]
    pub fn Foreground(&self) -> u32 {
        self._bitfield_1.get(2usize, 1u8) as u32
    }

    #[inline]
    pub fn set_Foreground(&mut self, val: u32) {
        self._bitfield_1.set(2usize, 1u8, val as u64)
    }

    #[inline]
    pub fn PerProcessStore(&self) -> u32 {
        self._bitfield_1.get(3usize, 1u8) as u32
    }

    #[inline]
    pub fn set_PerProcessStore(&mut self, val: u32) {
        self._bitfield_1.set(3usize, 1u8, val as u64)
    }

    #[inline]
    pub fn Spare(&self) -> u32 {
        self._bitfield_1.get(4usize, 28u8) as u32
    }

    #[inline]
    pub fn set_Spare(&mut self, val: u32) {
        self._bitfield_1.set(4usize, 28u8, val as u64)
    }

    #[inline]
    pub fn new_bitfield_1(
        ModernApp: u32,
        DeepFrozen: u32,
        Foreground: u32,
        PerProcessStore: u32,
        Spare: u32,
    ) -> BitfieldUnit<[u8; 4]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 4]> = Default::default();

        bitfield_unit.set(0usize, 1u8, ModernApp as u64);

        bitfield_unit.set(1usize, 1u8, DeepFrozen as u64);

        bitfield_unit.set(2usize, 1u8, Foreground as u64);

        bitfield_unit.set(3usize, 1u8, PerProcessStore as u64);

        bitfield_unit.set(4usize, 28u8, Spare as u64);

        bitfield_unit
    }
}

#[repr(C)]
pub struct PF_PRIVSOURCE_QUERY_REQUEST {
    pub Version: u32,
    pub Flags: u32,
    pub InfoCount: u32,
    pub InfoArray: [PF_PRIVSOURCE_INFO; 1],
}

impl Default for PF_PRIVSOURCE_QUERY_REQUEST {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PF_PRIVSOURCE_QUERY_REQUEST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PF_PRIVSOURCE_QUERY_REQUEST {{ InfoArray: {:?} }}",
            self.InfoArray
        )
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum PF_PHASED_SCENARIO_TYPE {
    PfScenarioTypeNone = 0,
    PfScenarioTypeStandby = 1,
    PfScenarioTypeHibernate = 2,
    PfScenarioTypeFUS = 3,
    PfScenarioTypeMax = 4,
}

#[repr(C)]
pub struct PF_SCENARIO_PHASE_INFO {
    pub Version: u32,
    pub ScenType: PF_PHASED_SCENARIO_TYPE,
    pub PhaseId: u32,
    pub SequenceNumber: u32,
    pub Flags: u32,
    pub FUSUserId: u32,
}

impl Default for PF_SCENARIO_PHASE_INFO {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PF_SCENARIO_PHASE_INFO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PF_SCENARIO_PHASE_INFO {{ ScenType: {:?} }}",
            self.ScenType
        )
    }
}

#[repr(C)]
pub struct PF_MEMORY_LIST_NODE {
    _bitfield_align_1: [u64; 0],
    _bitfield_1: BitfieldUnit<[u8; 8]>,
    pub StandbyLowPageCount: u64,
    pub StandbyMediumPageCount: u64,
    pub StandbyHighPageCount: u64,
    pub FreePageCount: u64,
    pub ModifiedPageCount: u64,
}

impl Default for PF_MEMORY_LIST_NODE {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PF_MEMORY_LIST_NODE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PF_MEMORY_LIST_NODE {{ Node : {:?}, Spare : {:?} }}",
            self.Node(),
            self.Spare()
        )
    }
}

impl PF_MEMORY_LIST_NODE {
    #[inline]
    pub fn Node(&self) -> u64 {
        self._bitfield_1.get(0usize, 8u8)
    }

    #[inline]
    pub fn set_Node(&mut self, val: u64) {
        self._bitfield_1.set(0usize, 8u8, val)
    }

    #[inline]
    pub fn Spare(&self) -> u64 {
        self._bitfield_1.get(8usize, 56u8)
    }

    #[inline]
    pub fn set_Spare(&mut self, val: u64) {
        self._bitfield_1.set(8usize, 56u8, val)
    }

    #[inline]
    pub fn new_bitfield_1(Node: u64, Spare: u64) -> BitfieldUnit<[u8; 8]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 8]> = Default::default();

        bitfield_unit.set(0usize, 8u8, Node);

        bitfield_unit.set(8usize, 56u8, Spare);

        bitfield_unit
    }
}

#[repr(C)]
pub struct PF_ROBUST_PROCESS_ENTRY {
    pub ImagePathHash: u32,
    pub Pid: u32,
    pub Alignment: u32,
}

impl Default for PF_ROBUST_PROCESS_ENTRY {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PF_ROBUST_PROCESS_ENTRY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PF_ROBUST_PROCESS_ENTRY {{  }}")
    }
}

#[repr(C)]
pub struct PF_ROBUST_FILE_ENTRY {
    pub FilePathHash: u32,
}

impl Default for PF_ROBUST_FILE_ENTRY {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PF_ROBUST_FILE_ENTRY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PF_ROBUST_FILE_ENTRY {{  }}")
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum PF_ROBUSTNESS_CONTROL_COMMAND {
    PfRpControlUpdate = 0,
    PfRpControlReset = 1,
    PfRpControlRobustAllStart = 2,
    PfRpControlRobustAllStop = 3,
    PfRpControlCommandMax = 4,
}

#[repr(C)]
pub struct PF_ROBUSTNESS_CONTROL {
    pub Version: u32,
    pub Command: PF_ROBUSTNESS_CONTROL_COMMAND,
    pub DeprioProcessCount: u32,
    pub ExemptProcessCount: u32,
    pub DeprioFileCount: u32,
    pub ExemptFileCount: u32,
    pub ProcessEntries: [PF_ROBUST_PROCESS_ENTRY; 1],
    pub FileEntries: [PF_ROBUST_FILE_ENTRY; 1],
}

impl Default for PF_ROBUSTNESS_CONTROL {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PF_ROBUSTNESS_CONTROL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PF_ROBUSTNESS_CONTROL {{ Command: {:?}, ProcessEntries: {:?}, FileEntries: {:?} }}",
            self.Command, self.ProcessEntries, self.FileEntries
        )
    }
}

#[repr(C)]
pub struct PF_TIME_CONTROL {
    pub TimeAdjustment: i32,
}

impl Default for PF_TIME_CONTROL {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PF_TIME_CONTROL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PF_TIME_CONTROL {{  }}")
    }
}

#[repr(C)]
pub struct PF_MEMORY_LIST_INFO {
    pub Version: u32,
    pub Size: u32,
    pub NodeCount: u32,
    pub Nodes: [PF_MEMORY_LIST_NODE; 1],
}

impl Default for PF_MEMORY_LIST_INFO {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PF_MEMORY_LIST_INFO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PF_MEMORY_LIST_INFO {{ Nodes: {:?} }}", self.Nodes)
    }
}

#[repr(C)]
pub struct PF_PHYSICAL_MEMORY_RANGE {
    pub BasePfn: usize,
    pub PageCount: usize,
}

impl Default for PF_PHYSICAL_MEMORY_RANGE {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PF_PHYSICAL_MEMORY_RANGE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PF_PHYSICAL_MEMORY_RANGE {{  }}")
    }
}

#[repr(C)]
pub struct PF_PHYSICAL_MEMORY_RANGE_INFO_V1 {
    pub Version: u32,
    pub RangeCount: u32,
    pub Ranges: [PF_PHYSICAL_MEMORY_RANGE; 1],
}

impl Default for PF_PHYSICAL_MEMORY_RANGE_INFO_V1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PF_PHYSICAL_MEMORY_RANGE_INFO_V1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PF_PHYSICAL_MEMORY_RANGE_INFO_V1 {{ Ranges: {:?} }}",
            self.Ranges
        )
    }
}

#[repr(C)]
pub struct PF_PHYSICAL_MEMORY_RANGE_INFO_V2 {
    pub Version: u32,
    pub Flags: u32,
    pub RangeCount: u32,
    pub Ranges: [PF_PHYSICAL_MEMORY_RANGE; 1],
}

impl Default for PF_PHYSICAL_MEMORY_RANGE_INFO_V2 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PF_PHYSICAL_MEMORY_RANGE_INFO_V2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PF_PHYSICAL_MEMORY_RANGE_INFO_V2 {{ Ranges: {:?} }}",
            self.Ranges
        )
    }
}

#[repr(C)]
pub struct PF_REPURPOSED_BY_PREFETCH_INFO {
    pub Version: u32,
    pub RepurposedByPrefetch: usize,
}

impl Default for PF_REPURPOSED_BY_PREFETCH_INFO {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PF_REPURPOSED_BY_PREFETCH_INFO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PF_REPURPOSED_BY_PREFETCH_INFO {{  }}")
    }
}

#[repr(C)]
pub struct PF_VIRTUAL_QUERY {
    pub Version: u32,
    pub Anonymous1: PF_VIRTUAL_QUERY_1,
    pub QueryBuffer: *mut std::ffi::c_void,
    pub QueryBufferSize: usize,
    pub ProcessHandle: HANDLE,
}

#[repr(C)]
pub struct PF_VIRTUAL_QUERY_1 {
    pub Flags: UnionField<u32>,
    pub Anonymous1: UnionField<PF_VIRTUAL_QUERY_1_1>,
    pub union_field: u32,
}

#[repr(C)]
#[repr(align(4))]
pub struct PF_VIRTUAL_QUERY_1_1 {
    _bitfield_align_1: [u32; 0],
    _bitfield_1: BitfieldUnit<[u8; 4]>,
}

impl Default for PF_VIRTUAL_QUERY_1_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PF_VIRTUAL_QUERY_1_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PF_VIRTUAL_QUERY_1_1 {{ FaultInPageTables : {:?}, ReportPageTables : {:?}, Spare : {:?} }}",
            self.FaultInPageTables(),
            self.ReportPageTables(),
            self.Spare()
        )
    }
}

impl PF_VIRTUAL_QUERY_1_1 {
    #[inline]
    pub fn FaultInPageTables(&self) -> u32 {
        self._bitfield_1.get(0usize, 1u8) as u32
    }

    #[inline]
    pub fn set_FaultInPageTables(&mut self, val: u32) {
        self._bitfield_1.set(0usize, 1u8, val as u64)
    }

    #[inline]
    pub fn ReportPageTables(&self) -> u32 {
        self._bitfield_1.get(1usize, 1u8) as u32
    }

    #[inline]
    pub fn set_ReportPageTables(&mut self, val: u32) {
        self._bitfield_1.set(1usize, 1u8, val as u64)
    }

    #[inline]
    pub fn Spare(&self) -> u32 {
        self._bitfield_1.get(2usize, 30u8) as u32
    }

    #[inline]
    pub fn set_Spare(&mut self, val: u32) {
        self._bitfield_1.set(2usize, 30u8, val as u64)
    }

    #[inline]
    pub fn new_bitfield_1(
        FaultInPageTables: u32,
        ReportPageTables: u32,
        Spare: u32,
    ) -> BitfieldUnit<[u8; 4]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 4]> = Default::default();

        bitfield_unit.set(0usize, 1u8, FaultInPageTables as u64);

        bitfield_unit.set(1usize, 1u8, ReportPageTables as u64);

        bitfield_unit.set(2usize, 30u8, Spare as u64);

        bitfield_unit
    }
}

impl Default for PF_VIRTUAL_QUERY_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PF_VIRTUAL_QUERY_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PF_VIRTUAL_QUERY_1 {{ union }}")
    }
}

impl Default for PF_VIRTUAL_QUERY {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PF_VIRTUAL_QUERY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PF_VIRTUAL_QUERY {{ Anonymous1: {:?} }}",
            self.Anonymous1
        )
    }
}

#[repr(C)]
pub struct PF_MIN_WS_AGE_RATE_CONTROL {
    pub Version: u32,
    pub SecondsToOldestAge: u32,
}

impl Default for PF_MIN_WS_AGE_RATE_CONTROL {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PF_MIN_WS_AGE_RATE_CONTROL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PF_MIN_WS_AGE_RATE_CONTROL {{  }}")
    }
}

#[repr(C)]
pub struct PF_DEPRIORITIZE_OLD_PAGES {
    pub Version: u32,
    pub ProcessHandle: HANDLE,
    pub Anonymous1: PF_DEPRIORITIZE_OLD_PAGES_1,
}

#[repr(C)]
pub struct PF_DEPRIORITIZE_OLD_PAGES_1 {
    pub Flags: UnionField<u32>,
    pub Anonymous1: UnionField<PF_DEPRIORITIZE_OLD_PAGES_1_1>,
    pub union_field: u32,
}

#[repr(C)]
#[repr(align(4))]
pub struct PF_DEPRIORITIZE_OLD_PAGES_1_1 {
    _bitfield_align_1: [u32; 0],
    _bitfield_1: BitfieldUnit<[u8; 4]>,
}

impl Default for PF_DEPRIORITIZE_OLD_PAGES_1_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PF_DEPRIORITIZE_OLD_PAGES_1_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PF_DEPRIORITIZE_OLD_PAGES_1_1 {{ TargetPriority : {:?}, TrimPages : {:?}, Spare : {:?} }}",
            self.TargetPriority(),
            self.TrimPages(),
            self.Spare()
        )
    }
}

impl PF_DEPRIORITIZE_OLD_PAGES_1_1 {
    #[inline]
    pub fn TargetPriority(&self) -> u32 {
        self._bitfield_1.get(0usize, 4u8) as u32
    }

    #[inline]
    pub fn set_TargetPriority(&mut self, val: u32) {
        self._bitfield_1.set(0usize, 4u8, val as u64)
    }

    #[inline]
    pub fn TrimPages(&self) -> u32 {
        self._bitfield_1.get(4usize, 2u8) as u32
    }

    #[inline]
    pub fn set_TrimPages(&mut self, val: u32) {
        self._bitfield_1.set(4usize, 2u8, val as u64)
    }

    #[inline]
    pub fn Spare(&self) -> u32 {
        self._bitfield_1.get(6usize, 26u8) as u32
    }

    #[inline]
    pub fn set_Spare(&mut self, val: u32) {
        self._bitfield_1.set(6usize, 26u8, val as u64)
    }

    #[inline]
    pub fn new_bitfield_1(
        TargetPriority: u32,
        TrimPages: u32,
        Spare: u32,
    ) -> BitfieldUnit<[u8; 4]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 4]> = Default::default();

        bitfield_unit.set(0usize, 4u8, TargetPriority as u64);

        bitfield_unit.set(4usize, 2u8, TrimPages as u64);

        bitfield_unit.set(6usize, 26u8, Spare as u64);

        bitfield_unit
    }
}

impl Default for PF_DEPRIORITIZE_OLD_PAGES_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PF_DEPRIORITIZE_OLD_PAGES_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PF_DEPRIORITIZE_OLD_PAGES_1 {{ union }}")
    }
}

impl Default for PF_DEPRIORITIZE_OLD_PAGES {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PF_DEPRIORITIZE_OLD_PAGES {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PF_DEPRIORITIZE_OLD_PAGES {{ Anonymous1: {:?} }}",
            self.Anonymous1
        )
    }
}

#[repr(C)]
pub struct PF_GPU_UTILIZATION_INFO {
    pub Version: u32,
    pub SessionId: u32,
    pub GpuTime: u64,
}

impl Default for PF_GPU_UTILIZATION_INFO {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PF_GPU_UTILIZATION_INFO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PF_GPU_UTILIZATION_INFO {{  }}")
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SUPERFETCH_INFORMATION_CLASS {
    SuperfetchRetrieveTrace = 1,
    SuperfetchSystemParameters = 2,
    SuperfetchLogEvent = 3,
    SuperfetchGenerateTrace = 4,
    SuperfetchPrefetch = 5,
    SuperfetchPfnQuery = 6,
    SuperfetchPfnSetPriority = 7,
    SuperfetchPrivSourceQuery = 8,
    SuperfetchSequenceNumberQuery = 9,
    SuperfetchScenarioPhase = 10,
    SuperfetchWorkerPriority = 11,
    SuperfetchScenarioQuery = 12,
    SuperfetchScenarioPrefetch = 13,
    SuperfetchRobustnessControl = 14,
    SuperfetchTimeControl = 15,
    SuperfetchMemoryListQuery = 16,
    SuperfetchMemoryRangesQuery = 17,
    SuperfetchTracingControl = 18,
    SuperfetchTrimWhileAgingControl = 19,
    SuperfetchRepurposedByPrefetch = 20,
    SuperfetchChannelPowerRequest = 21,
    SuperfetchMovePages = 22,
    SuperfetchVirtualQuery = 23,
    SuperfetchCombineStatsQuery = 24,
    SuperfetchSetMinWsAgeRate = 25,
    SuperfetchDeprioritizeOldPagesInWs = 26,
    SuperfetchFileExtentsQuery = 27,
    SuperfetchGpuUtilizationQuery = 28,
    SuperfetchPfnSet = 29,
    SuperfetchInformationMax = 30,
}

#[repr(C)]
pub struct SUPERFETCH_INFORMATION {
    pub Version: u32,
    pub Magic: u32,
    pub SuperfetchInformationClass: SUPERFETCH_INFORMATION_CLASS,
    pub SuperfetchInformation: *mut std::ffi::c_void,
    pub SuperfetchInformationLength: u32,
}

impl Default for SUPERFETCH_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for SUPERFETCH_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SUPERFETCH_INFORMATION {{ SuperfetchInformationClass: {:?} }}",
            self.SuperfetchInformationClass
        )
    }
}
