use windows::{
    core::GUID,
    Win32::{
        Foundation::{NTSTATUS},
        System::{
            Kernel::{LIST_ENTRY32, PROCESSOR_NUMBER, SINGLE_LIST_ENTRY32, STRING32},
            SystemServices::NT_TIB32,
            WindowsProgramming::CLIENT_ID,
        },
    },
};

use crate::{
    bitfield::{BitfieldUnit, UnionField},
    ntldr::{LDR_DDAG_STATE, LDR_DLL_LOAD_REASON, LDR_HOT_PATCH_STATE},
};

pub const WOW64_SYSTEM_DIRECTORY: &[u8; 8] = b"SysWOW64";
pub const WOW64_SYSTEM_DIRECTORY_U: &[u8; 9] = b"SysWOW64\0";
pub const WOW64_X86_TAG: &[u8; 6] = b" (x86)";
pub const WOW64_X86_TAG_U: &[u8; 7] = b" (x86)\0";
pub const WOW64_CPUFLAGS_MSFT64: u32 = 1;
pub const WOW64_CPUFLAGS_SOFTWARE: u32 = 2;
pub const WOW64_CPUFLAGS_IA64: u32 = 4;
pub const LDR_DATA_TABLE_ENTRY_SIZE_WINXP_32: u32 = 80;
pub const LDR_DATA_TABLE_ENTRY_SIZE_WIN7_32: u32 = 144;
pub const LDR_DATA_TABLE_ENTRY_SIZE_WIN8_32: u32 = 152;
pub const LDR_DATA_TABLE_ENTRY_SIZE_WIN10_32: u32 = 164;
pub const LDR_DATA_TABLE_ENTRY_SIZE_WIN11_32: u32 = 184;

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum WOW64_SHARED_INFORMATION {
    SharedNtdll32LdrInitializeThunk = 0,
    SharedNtdll32KiUserExceptionDispatcher = 1,
    SharedNtdll32KiUserApcDispatcher = 2,
    SharedNtdll32KiUserCallbackDispatcher = 3,
    SharedNtdll32ExpInterlockedPopEntrySListFault = 4,
    SharedNtdll32ExpInterlockedPopEntrySListResume = 5,
    SharedNtdll32ExpInterlockedPopEntrySListEnd = 6,
    SharedNtdll32RtlUserThreadStart = 7,
    SharedNtdll32pQueryProcessDebugInformationRemote = 8,
    SharedNtdll32BaseAddress = 9,
    SharedNtdll32LdrSystemDllInitBlock = 10,
    Wow64SharedPageEntriesCount = 11,
}

#[repr(C)]
pub struct RTL_BALANCED_NODE32 {
    pub Anonymous1: RTL_BALANCED_NODE32_1,
    pub Anonymous2: RTL_BALANCED_NODE32_2,
}

#[repr(C)]
pub struct RTL_BALANCED_NODE32_1 {
    pub Children: UnionField<[u32; 2]>,
    pub Anonymous1: UnionField<RTL_BALANCED_NODE32_1_1>,
    pub union_field: [u32; 2],
}

#[repr(C)]
pub struct RTL_BALANCED_NODE32_1_1 {
    pub Left: u32,
    pub Right: u32,
}

impl Default for RTL_BALANCED_NODE32_1_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_BALANCED_NODE32_1_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RTL_BALANCED_NODE32_1_1 {{  }}")
    }
}

impl Default for RTL_BALANCED_NODE32_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_BALANCED_NODE32_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RTL_BALANCED_NODE32_1 {{ union }}")
    }
}

#[repr(C)]
pub struct RTL_BALANCED_NODE32_2 {
    _bitfield_align_1: [u8; 0],
    _bitfield_1: UnionField<BitfieldUnit<[u8; 1]>>,
    pub ParentValue: UnionField<u32>,
    pub union_field: u32,
}

impl Default for RTL_BALANCED_NODE32_2 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_BALANCED_NODE32_2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RTL_BALANCED_NODE32_2 {{ union }}")
    }
}

impl RTL_BALANCED_NODE32_2 {
    #[inline]
    pub fn Red(&self) -> u32 {
        unsafe { self._bitfield_1.as_ref().get(0usize, 1u8) as u32 }
    }

    #[inline]
    pub fn set_Red(&mut self, val: u32) {
        unsafe { self._bitfield_1.as_mut().set(0usize, 1u8, val as u64) }
    }

    #[inline]
    pub fn Balance(&self) -> u32 {
        unsafe { self._bitfield_1.as_ref().get(1usize, 2u8) as u32 }
    }

    #[inline]
    pub fn set_Balance(&mut self, val: u32) {
        unsafe { self._bitfield_1.as_mut().set(1usize, 2u8, val as u64) }
    }

    #[inline]
    pub fn new_bitfield_1(Red: u32, Balance: u32) -> BitfieldUnit<[u8; 1]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 1]> = Default::default();

        bitfield_unit.set(0usize, 1u8, Red as u64);

        bitfield_unit.set(1usize, 2u8, Balance as u64);

        bitfield_unit
    }
}

impl Default for RTL_BALANCED_NODE32 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_BALANCED_NODE32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RTL_BALANCED_NODE32 {{ Anonymous1: {:?}, Anonymous2: {:?} }}",
            self.Anonymous1, self.Anonymous2
        )
    }
}

#[repr(C)]
pub struct RTL_RB_TREE32 {
    pub Root: u32,
    pub Min: u32,
}

impl Default for RTL_RB_TREE32 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_RB_TREE32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RTL_RB_TREE32 {{  }}")
    }
}

#[repr(C)]
pub struct PEB_LDR_DATA32 {
    pub Length: u32,
    pub Initialized: bool,
    pub SsHandle: u32,
    pub InLoadOrderModuleList: LIST_ENTRY32,
    pub InMemoryOrderModuleList: LIST_ENTRY32,
    pub InInitializationOrderModuleList: LIST_ENTRY32,
    pub EntryInProgress: u32,
    pub ShutdownInProgress: bool,
    pub ShutdownThreadId: u32,
}

impl Default for PEB_LDR_DATA32 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PEB_LDR_DATA32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PEB_LDR_DATA32 {{  }}")
    }
}

#[repr(C)]
pub struct LDR_SERVICE_TAG_RECORD32 {
    pub Next: u32,
    pub ServiceTag: u32,
}

impl Default for LDR_SERVICE_TAG_RECORD32 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for LDR_SERVICE_TAG_RECORD32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LDR_SERVICE_TAG_RECORD32 {{  }}")
    }
}

#[repr(C)]
pub struct LDRP_CSLIST32 {
    pub Tail: u32,
}

impl Default for LDRP_CSLIST32 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for LDRP_CSLIST32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LDRP_CSLIST32 {{  }}")
    }
}

#[repr(C)]
pub struct LDR_DDAG_NODE32 {
    pub Modules: LIST_ENTRY32,
    pub ServiceTagList: u32,
    pub LoadCount: u32,
    pub LoadWhileUnloadingCount: u32,
    pub LowestLink: u32,
    pub Anonymous1: LDR_DDAG_NODE32_1,
    pub IncomingDependencies: LDRP_CSLIST32,
    pub State: LDR_DDAG_STATE,
    pub CondenseLink: SINGLE_LIST_ENTRY32,
    pub PreorderNumber: u32,
}

#[repr(C)]
pub struct LDR_DDAG_NODE32_1 {
    pub Dependencies: UnionField<LDRP_CSLIST32>,
    pub RemovalLink: UnionField<SINGLE_LIST_ENTRY32>,
    pub union_field: u32,
}

impl Default for LDR_DDAG_NODE32_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for LDR_DDAG_NODE32_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LDR_DDAG_NODE32_1 {{ union }}")
    }
}

impl Default for LDR_DDAG_NODE32 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for LDR_DDAG_NODE32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LDR_DDAG_NODE32 {{ Anonymous1: {:?}, IncomingDependencies: {:?} }}",
            self.Anonymous1, self.IncomingDependencies
        )
    }
}

#[repr(C)]
pub struct LDR_DATA_TABLE_ENTRY32 {
    pub InLoadOrderLinks: LIST_ENTRY32,
    pub InMemoryOrderLinks: LIST_ENTRY32,
    pub Anonymous1: LDR_DATA_TABLE_ENTRY32_1,
    pub DllBase: u32,
    pub EntryPoint: u32,
    pub SizeOfImage: u32,
    pub FullDllName: STRING32,
    pub BaseDllName: STRING32,
    pub Anonymous2: LDR_DATA_TABLE_ENTRY32_2,
    pub ObsoleteLoadCount: u16,
    pub TlsIndex: u16,
    pub HashLinks: LIST_ENTRY32,
    pub TimeDateStamp: u32,
    pub EntryPointActivationContext: u32,
    pub Lock: u32,
    pub DdagNode: u32,
    pub NodeModuleLink: LIST_ENTRY32,
    pub LoadContext: u32,
    pub ParentDllBase: u32,
    pub SwitchBackContext: u32,
    pub BaseAddressIndexNode: RTL_BALANCED_NODE32,
    pub MappingInfoIndexNode: RTL_BALANCED_NODE32,
    pub OriginalBase: u32,
    pub LoadTime: i64,
    pub BaseNameHashValue: u32,
    pub LoadReason: LDR_DLL_LOAD_REASON,
    pub ImplicitPathOptions: u32,
    pub ReferenceCount: u32,
    pub DependentLoadFlags: u32,
    pub SigningLevel: u8,
    pub CheckSum: u32,
    pub ActivePatchImageBase: u32,
    pub HotPatchState: LDR_HOT_PATCH_STATE,
}

#[repr(C)]
pub struct LDR_DATA_TABLE_ENTRY32_1 {
    pub InInitializationOrderLinks: UnionField<LIST_ENTRY32>,
    pub InProgressLinks: UnionField<LIST_ENTRY32>,
    pub union_field: [u32; 2],
}

impl Default for LDR_DATA_TABLE_ENTRY32_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for LDR_DATA_TABLE_ENTRY32_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LDR_DATA_TABLE_ENTRY32_1 {{ union }}")
    }
}

#[repr(C)]
pub struct LDR_DATA_TABLE_ENTRY32_2 {
    pub FlagGroup: UnionField<[u8; 4]>,
    pub Flags: UnionField<u32>,
    pub Anonymous1: UnionField<LDR_DATA_TABLE_ENTRY32_2_1>,
    pub union_field: u32,
}

#[repr(C)]
#[repr(align(4))]
pub struct LDR_DATA_TABLE_ENTRY32_2_1 {
    _bitfield_align_1: [u8; 0],
    _bitfield_1: BitfieldUnit<[u8; 4]>,
}

impl Default for LDR_DATA_TABLE_ENTRY32_2_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for LDR_DATA_TABLE_ENTRY32_2_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LDR_DATA_TABLE_ENTRY32_2_1 {{ PackagedBinary : {:?}, MarkedForRemoval : {:?}, ImageDll : {:?}, LoadNotificationsSent : {:?}, TelemetryEntryProcessed : {:?}, ProcessStaticImport : {:?}, InLegacyLists : {:?}, InIndexes : {:?}, ShimDll : {:?}, InExceptionTable : {:?}, ReservedFlags1 : {:?}, LoadInProgress : {:?}, LoadConfigProcessed : {:?}, EntryProcessed : {:?}, ProtectDelayLoad : {:?}, ReservedFlags3 : {:?}, DontCallForThreads : {:?}, ProcessAttachCalled : {:?}, ProcessAttachFailed : {:?}, CorDeferredValidate : {:?}, CorImage : {:?}, DontRelocate : {:?}, CorILOnly : {:?}, ChpeImage : {:?}, ReservedFlags5 : {:?}, Redirected : {:?}, ReservedFlags6 : {:?}, CompatDatabaseProcessed : {:?} }}",
            self.PackagedBinary(),
            self.MarkedForRemoval(),
            self.ImageDll(),
            self.LoadNotificationsSent(),
            self.TelemetryEntryProcessed(),
            self.ProcessStaticImport(),
            self.InLegacyLists(),
            self.InIndexes(),
            self.ShimDll(),
            self.InExceptionTable(),
            self.ReservedFlags1(),
            self.LoadInProgress(),
            self.LoadConfigProcessed(),
            self.EntryProcessed(),
            self.ProtectDelayLoad(),
            self.ReservedFlags3(),
            self.DontCallForThreads(),
            self.ProcessAttachCalled(),
            self.ProcessAttachFailed(),
            self.CorDeferredValidate(),
            self.CorImage(),
            self.DontRelocate(),
            self.CorILOnly(),
            self.ChpeImage(),
            self.ReservedFlags5(),
            self.Redirected(),
            self.ReservedFlags6(),
            self.CompatDatabaseProcessed()
        )
    }
}

impl LDR_DATA_TABLE_ENTRY32_2_1 {
    #[inline]
    pub fn PackagedBinary(&self) -> u32 {
        self._bitfield_1.get(0usize, 1u8) as u32
    }

    #[inline]
    pub fn set_PackagedBinary(&mut self, val: u32) {
        self._bitfield_1.set(0usize, 1u8, val as u64)
    }

    #[inline]
    pub fn MarkedForRemoval(&self) -> u32 {
        self._bitfield_1.get(1usize, 1u8) as u32
    }

    #[inline]
    pub fn set_MarkedForRemoval(&mut self, val: u32) {
        self._bitfield_1.set(1usize, 1u8, val as u64)
    }

    #[inline]
    pub fn ImageDll(&self) -> u32 {
        self._bitfield_1.get(2usize, 1u8) as u32
    }

    #[inline]
    pub fn set_ImageDll(&mut self, val: u32) {
        self._bitfield_1.set(2usize, 1u8, val as u64)
    }

    #[inline]
    pub fn LoadNotificationsSent(&self) -> u32 {
        self._bitfield_1.get(3usize, 1u8) as u32
    }

    #[inline]
    pub fn set_LoadNotificationsSent(&mut self, val: u32) {
        self._bitfield_1.set(3usize, 1u8, val as u64)
    }

    #[inline]
    pub fn TelemetryEntryProcessed(&self) -> u32 {
        self._bitfield_1.get(4usize, 1u8) as u32
    }

    #[inline]
    pub fn set_TelemetryEntryProcessed(&mut self, val: u32) {
        self._bitfield_1.set(4usize, 1u8, val as u64)
    }

    #[inline]
    pub fn ProcessStaticImport(&self) -> u32 {
        self._bitfield_1.get(5usize, 1u8) as u32
    }

    #[inline]
    pub fn set_ProcessStaticImport(&mut self, val: u32) {
        self._bitfield_1.set(5usize, 1u8, val as u64)
    }

    #[inline]
    pub fn InLegacyLists(&self) -> u32 {
        self._bitfield_1.get(6usize, 1u8) as u32
    }

    #[inline]
    pub fn set_InLegacyLists(&mut self, val: u32) {
        self._bitfield_1.set(6usize, 1u8, val as u64)
    }

    #[inline]
    pub fn InIndexes(&self) -> u32 {
        self._bitfield_1.get(7usize, 1u8) as u32
    }

    #[inline]
    pub fn set_InIndexes(&mut self, val: u32) {
        self._bitfield_1.set(7usize, 1u8, val as u64)
    }

    #[inline]
    pub fn ShimDll(&self) -> u32 {
        self._bitfield_1.get(8usize, 1u8) as u32
    }

    #[inline]
    pub fn set_ShimDll(&mut self, val: u32) {
        self._bitfield_1.set(8usize, 1u8, val as u64)
    }

    #[inline]
    pub fn InExceptionTable(&self) -> u32 {
        self._bitfield_1.get(9usize, 1u8) as u32
    }

    #[inline]
    pub fn set_InExceptionTable(&mut self, val: u32) {
        self._bitfield_1.set(9usize, 1u8, val as u64)
    }

    #[inline]
    pub fn ReservedFlags1(&self) -> u32 {
        self._bitfield_1.get(10usize, 2u8) as u32
    }

    #[inline]
    pub fn set_ReservedFlags1(&mut self, val: u32) {
        self._bitfield_1.set(10usize, 2u8, val as u64)
    }

    #[inline]
    pub fn LoadInProgress(&self) -> u32 {
        self._bitfield_1.get(12usize, 1u8) as u32
    }

    #[inline]
    pub fn set_LoadInProgress(&mut self, val: u32) {
        self._bitfield_1.set(12usize, 1u8, val as u64)
    }

    #[inline]
    pub fn LoadConfigProcessed(&self) -> u32 {
        self._bitfield_1.get(13usize, 1u8) as u32
    }

    #[inline]
    pub fn set_LoadConfigProcessed(&mut self, val: u32) {
        self._bitfield_1.set(13usize, 1u8, val as u64)
    }

    #[inline]
    pub fn EntryProcessed(&self) -> u32 {
        self._bitfield_1.get(14usize, 1u8) as u32
    }

    #[inline]
    pub fn set_EntryProcessed(&mut self, val: u32) {
        self._bitfield_1.set(14usize, 1u8, val as u64)
    }

    #[inline]
    pub fn ProtectDelayLoad(&self) -> u32 {
        self._bitfield_1.get(15usize, 1u8) as u32
    }

    #[inline]
    pub fn set_ProtectDelayLoad(&mut self, val: u32) {
        self._bitfield_1.set(15usize, 1u8, val as u64)
    }

    #[inline]
    pub fn ReservedFlags3(&self) -> u32 {
        self._bitfield_1.get(16usize, 2u8) as u32
    }

    #[inline]
    pub fn set_ReservedFlags3(&mut self, val: u32) {
        self._bitfield_1.set(16usize, 2u8, val as u64)
    }

    #[inline]
    pub fn DontCallForThreads(&self) -> u32 {
        self._bitfield_1.get(18usize, 1u8) as u32
    }

    #[inline]
    pub fn set_DontCallForThreads(&mut self, val: u32) {
        self._bitfield_1.set(18usize, 1u8, val as u64)
    }

    #[inline]
    pub fn ProcessAttachCalled(&self) -> u32 {
        self._bitfield_1.get(19usize, 1u8) as u32
    }

    #[inline]
    pub fn set_ProcessAttachCalled(&mut self, val: u32) {
        self._bitfield_1.set(19usize, 1u8, val as u64)
    }

    #[inline]
    pub fn ProcessAttachFailed(&self) -> u32 {
        self._bitfield_1.get(20usize, 1u8) as u32
    }

    #[inline]
    pub fn set_ProcessAttachFailed(&mut self, val: u32) {
        self._bitfield_1.set(20usize, 1u8, val as u64)
    }

    #[inline]
    pub fn CorDeferredValidate(&self) -> u32 {
        self._bitfield_1.get(21usize, 1u8) as u32
    }

    #[inline]
    pub fn set_CorDeferredValidate(&mut self, val: u32) {
        self._bitfield_1.set(21usize, 1u8, val as u64)
    }

    #[inline]
    pub fn CorImage(&self) -> u32 {
        self._bitfield_1.get(22usize, 1u8) as u32
    }

    #[inline]
    pub fn set_CorImage(&mut self, val: u32) {
        self._bitfield_1.set(22usize, 1u8, val as u64)
    }

    #[inline]
    pub fn DontRelocate(&self) -> u32 {
        self._bitfield_1.get(23usize, 1u8) as u32
    }

    #[inline]
    pub fn set_DontRelocate(&mut self, val: u32) {
        self._bitfield_1.set(23usize, 1u8, val as u64)
    }

    #[inline]
    pub fn CorILOnly(&self) -> u32 {
        self._bitfield_1.get(24usize, 1u8) as u32
    }

    #[inline]
    pub fn set_CorILOnly(&mut self, val: u32) {
        self._bitfield_1.set(24usize, 1u8, val as u64)
    }

    #[inline]
    pub fn ChpeImage(&self) -> u32 {
        self._bitfield_1.get(25usize, 1u8) as u32
    }

    #[inline]
    pub fn set_ChpeImage(&mut self, val: u32) {
        self._bitfield_1.set(25usize, 1u8, val as u64)
    }

    #[inline]
    pub fn ReservedFlags5(&self) -> u32 {
        self._bitfield_1.get(26usize, 2u8) as u32
    }

    #[inline]
    pub fn set_ReservedFlags5(&mut self, val: u32) {
        self._bitfield_1.set(26usize, 2u8, val as u64)
    }

    #[inline]
    pub fn Redirected(&self) -> u32 {
        self._bitfield_1.get(28usize, 1u8) as u32
    }

    #[inline]
    pub fn set_Redirected(&mut self, val: u32) {
        self._bitfield_1.set(28usize, 1u8, val as u64)
    }

    #[inline]
    pub fn ReservedFlags6(&self) -> u32 {
        self._bitfield_1.get(29usize, 2u8) as u32
    }

    #[inline]
    pub fn set_ReservedFlags6(&mut self, val: u32) {
        self._bitfield_1.set(29usize, 2u8, val as u64)
    }

    #[inline]
    pub fn CompatDatabaseProcessed(&self) -> u32 {
        self._bitfield_1.get(31usize, 1u8) as u32
    }

    #[inline]
    pub fn set_CompatDatabaseProcessed(&mut self, val: u32) {
        self._bitfield_1.set(31usize, 1u8, val as u64)
    }

    #[inline]
    pub fn new_bitfield_1(
        PackagedBinary: u32,
        MarkedForRemoval: u32,
        ImageDll: u32,
        LoadNotificationsSent: u32,
        TelemetryEntryProcessed: u32,
        ProcessStaticImport: u32,
        InLegacyLists: u32,
        InIndexes: u32,
        ShimDll: u32,
        InExceptionTable: u32,
        ReservedFlags1: u32,
        LoadInProgress: u32,
        LoadConfigProcessed: u32,
        EntryProcessed: u32,
        ProtectDelayLoad: u32,
        ReservedFlags3: u32,
        DontCallForThreads: u32,
        ProcessAttachCalled: u32,
        ProcessAttachFailed: u32,
        CorDeferredValidate: u32,
        CorImage: u32,
        DontRelocate: u32,
        CorILOnly: u32,
        ChpeImage: u32,
        ReservedFlags5: u32,
        Redirected: u32,
        ReservedFlags6: u32,
        CompatDatabaseProcessed: u32,
    ) -> BitfieldUnit<[u8; 4]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 4]> = Default::default();

        bitfield_unit.set(0usize, 1u8, PackagedBinary as u64);

        bitfield_unit.set(1usize, 1u8, MarkedForRemoval as u64);

        bitfield_unit.set(2usize, 1u8, ImageDll as u64);

        bitfield_unit.set(3usize, 1u8, LoadNotificationsSent as u64);

        bitfield_unit.set(4usize, 1u8, TelemetryEntryProcessed as u64);

        bitfield_unit.set(5usize, 1u8, ProcessStaticImport as u64);

        bitfield_unit.set(6usize, 1u8, InLegacyLists as u64);

        bitfield_unit.set(7usize, 1u8, InIndexes as u64);

        bitfield_unit.set(8usize, 1u8, ShimDll as u64);

        bitfield_unit.set(9usize, 1u8, InExceptionTable as u64);

        bitfield_unit.set(10usize, 2u8, ReservedFlags1 as u64);

        bitfield_unit.set(12usize, 1u8, LoadInProgress as u64);

        bitfield_unit.set(13usize, 1u8, LoadConfigProcessed as u64);

        bitfield_unit.set(14usize, 1u8, EntryProcessed as u64);

        bitfield_unit.set(15usize, 1u8, ProtectDelayLoad as u64);

        bitfield_unit.set(16usize, 2u8, ReservedFlags3 as u64);

        bitfield_unit.set(18usize, 1u8, DontCallForThreads as u64);

        bitfield_unit.set(19usize, 1u8, ProcessAttachCalled as u64);

        bitfield_unit.set(20usize, 1u8, ProcessAttachFailed as u64);

        bitfield_unit.set(21usize, 1u8, CorDeferredValidate as u64);

        bitfield_unit.set(22usize, 1u8, CorImage as u64);

        bitfield_unit.set(23usize, 1u8, DontRelocate as u64);

        bitfield_unit.set(24usize, 1u8, CorILOnly as u64);

        bitfield_unit.set(25usize, 1u8, ChpeImage as u64);

        bitfield_unit.set(26usize, 2u8, ReservedFlags5 as u64);

        bitfield_unit.set(28usize, 1u8, Redirected as u64);

        bitfield_unit.set(29usize, 2u8, ReservedFlags6 as u64);

        bitfield_unit.set(31usize, 1u8, CompatDatabaseProcessed as u64);

        bitfield_unit
    }
}

impl Default for LDR_DATA_TABLE_ENTRY32_2 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for LDR_DATA_TABLE_ENTRY32_2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LDR_DATA_TABLE_ENTRY32_2 {{ union }}")
    }
}

impl Default for LDR_DATA_TABLE_ENTRY32 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for LDR_DATA_TABLE_ENTRY32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LDR_DATA_TABLE_ENTRY32 {{ Anonymous1: {:?}, Anonymous2: {:?}, BaseAddressIndexNode: {:?}, MappingInfoIndexNode: {:?} }}",
            self.Anonymous1, self.Anonymous2, self.BaseAddressIndexNode, self.MappingInfoIndexNode
        )
    }
}

#[repr(C)]
pub struct CURDIR32 {
    pub DosPath: STRING32,
    pub Handle: u32,
}

impl Default for CURDIR32 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for CURDIR32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CURDIR32 {{  }}")
    }
}

#[repr(C)]
pub struct RTL_DRIVE_LETTER_CURDIR32 {
    pub Flags: u16,
    pub Length: u16,
    pub TimeStamp: u32,
    pub DosPath: STRING32,
}

impl Default for RTL_DRIVE_LETTER_CURDIR32 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_DRIVE_LETTER_CURDIR32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RTL_DRIVE_LETTER_CURDIR32 {{  }}")
    }
}

#[repr(C)]
pub struct RTL_USER_PROCESS_PARAMETERS32 {
    pub MaximumLength: u32,
    pub Length: u32,
    pub Flags: u32,
    pub DebugFlags: u32,
    pub ConsoleHandle: u32,
    pub ConsoleFlags: u32,
    pub StandardInput: u32,
    pub StandardOutput: u32,
    pub StandardError: u32,
    pub CurrentDirectory: CURDIR32,
    pub DllPath: STRING32,
    pub ImagePathName: STRING32,
    pub CommandLine: STRING32,
    pub Environment: u32,
    pub StartingX: u32,
    pub StartingY: u32,
    pub CountX: u32,
    pub CountY: u32,
    pub CountCharsX: u32,
    pub CountCharsY: u32,
    pub FillAttribute: u32,
    pub WindowFlags: u32,
    pub ShowWindowFlags: u32,
    pub WindowTitle: STRING32,
    pub DesktopInfo: STRING32,
    pub ShellInfo: STRING32,
    pub RuntimeData: STRING32,
    pub CurrentDirectories: [RTL_DRIVE_LETTER_CURDIR32; 32],
    pub EnvironmentSize: u32,
    pub EnvironmentVersion: u32,
    pub PackageDependencyData: u32,
    pub ProcessGroupId: u32,
    pub LoaderThreads: u32,
    pub RedirectionDllName: STRING32,
    pub HeapPartitionName: STRING32,
    pub DefaultThreadpoolCpuSetMasks: u32,
    pub DefaultThreadpoolCpuSetMaskCount: u32,
    pub DefaultThreadpoolThreadMaximum: u32,
}

impl Default for RTL_USER_PROCESS_PARAMETERS32 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_USER_PROCESS_PARAMETERS32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RTL_USER_PROCESS_PARAMETERS32 {{ CurrentDirectory: {:?}, CurrentDirectories: {:?} }}",
            self.CurrentDirectory, self.CurrentDirectories
        )
    }
}

#[repr(C)]
pub struct PEB32 {
    pub InheritedAddressSpace: bool,
    pub ReadImageFileExecOptions: bool,
    pub BeingDebugged: bool,
    pub Anonymous1: PEB32_1,
    pub Mutant: u32,
    pub ImageBaseAddress: u32,
    pub Ldr: u32,
    pub ProcessParameters: u32,
    pub SubSystemData: u32,
    pub ProcessHeap: u32,
    pub FastPebLock: u32,
    pub AtlThunkSListPtr: u32,
    pub IFEOKey: u32,
    pub Anonymous2: PEB32_2,
    pub Anonymous3: PEB32_3,
    pub SystemReserved: u32,
    pub AtlThunkSListPtr32: u32,
    pub ApiSetMap: u32,
    pub TlsExpansionCounter: u32,
    pub TlsBitmap: u32,
    pub TlsBitmapBits: [u32; 2],
    pub ReadOnlySharedMemoryBase: u32,
    pub HotpatchInformation: u32,
    pub ReadOnlyStaticServerData: u32,
    pub AnsiCodePageData: u32,
    pub OemCodePageData: u32,
    pub UnicodeCaseTableData: u32,
    pub NumberOfProcessors: u32,
    pub NtGlobalFlag: u32,
    pub CriticalSectionTimeout: i64,
    pub HeapSegmentReserve: u32,
    pub HeapSegmentCommit: u32,
    pub HeapDeCommitTotalFreeThreshold: u32,
    pub HeapDeCommitFreeBlockThreshold: u32,
    pub NumberOfHeaps: u32,
    pub MaximumNumberOfHeaps: u32,
    pub ProcessHeaps: u32,
    pub GdiSharedHandleTable: u32,
    pub ProcessStarterHelper: u32,
    pub GdiDCAttributeList: u32,
    pub LoaderLock: u32,
    pub OSMajorVersion: u32,
    pub OSMinorVersion: u32,
    pub OSBuildNumber: u16,
    pub OSCSDVersion: u16,
    pub OSPlatformId: u32,
    pub ImageSubsystem: u32,
    pub ImageSubsystemMajorVersion: u32,
    pub ImageSubsystemMinorVersion: u32,
    pub ActiveProcessAffinityMask: u32,
    pub GdiHandleBuffer: [u32; 34],
    pub PostProcessInitRoutine: u32,
    pub TlsExpansionBitmap: u32,
    pub TlsExpansionBitmapBits: [u32; 32],
    pub SessionId: u32,
    pub AppCompatFlags: u64,
    pub AppCompatFlagsUser: u64,
    pub pShimData: u32,
    pub AppCompatInfo: u32,
    pub CSDVersion: STRING32,
    pub ActivationContextData: u32,
    pub ProcessAssemblyStorageMap: u32,
    pub SystemDefaultActivationContextData: u32,
    pub SystemAssemblyStorageMap: u32,
    pub MinimumStackCommit: u32,
    pub SparePointers: [u32; 2],
    pub PatchLoaderData: u32,
    pub ChpeV2ProcessInfo: u32,
    pub AppModelFeatureState: u32,
    pub SpareUlongs: [u32; 2],
    pub ActiveCodePage: u16,
    pub OemCodePage: u16,
    pub UseCaseMapping: u16,
    pub UnusedNlsField: u16,
    pub WerRegistrationData: u32,
    pub WerShipAssertPtr: u32,
    pub Anonymous4: PEB32_4,
    pub pImageHeaderHash: u32,
    pub Anonymous5: PEB32_5,
    pub CsrServerReadOnlySharedMemoryBase: u64,
    pub TppWorkerpListLock: u32,
    pub TppWorkerpList: LIST_ENTRY32,
    pub WaitOnAddressHashTable: [u32; 128],
    pub TelemetryCoverageHeader: u32,
    pub CloudFileFlags: u32,
    pub CloudFileDiagFlags: u32,
    pub PlaceholderCompatibilityMode: i8,
    pub PlaceholderCompatibilityModeReserved: [i8; 7],
    pub LeapSecondData: u32,
    pub Anonymous6: PEB32_6,
    pub NtGlobalFlag2: u32,
    pub ExtendedFeatureDisableMask: u64,
}

#[repr(C)]
pub struct PEB32_1 {
    pub BitField: UnionField<bool>,
    pub Anonymous1: UnionField<PEB32_1_1>,
    pub union_field: u8,
}

#[repr(C, packed)]
pub struct PEB32_1_1 {
    _bitfield_align_1: [u8; 0],
    _bitfield_1: BitfieldUnit<[u8; 1]>,
}

impl Default for PEB32_1_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PEB32_1_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PEB32_1_1 {{ ImageUsesLargePages : {:?}, IsProtectedProcess : {:?}, IsImageDynamicallyRelocated : {:?}, SkipPatchingUser32Forwarders : {:?}, IsPackagedProcess : {:?}, IsAppContainer : {:?}, IsProtectedProcessLight : {:?}, IsLongPathAwareProcess : {:?} }}",
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

impl PEB32_1_1 {
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

impl Default for PEB32_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PEB32_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PEB32_1 {{ union }}")
    }
}

#[repr(C)]
pub struct PEB32_2 {
    pub CrossProcessFlags: UnionField<u32>,
    pub Anonymous1: UnionField<PEB32_2_1>,
    pub union_field: u32,
}

#[repr(C)]
#[repr(align(4))]
pub struct PEB32_2_1 {
    _bitfield_align_1: [u32; 0],
    _bitfield_1: BitfieldUnit<[u8; 4]>,
}

impl Default for PEB32_2_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PEB32_2_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PEB32_2_1 {{ ProcessInJob : {:?}, ProcessInitializing : {:?}, ProcessUsingVEH : {:?}, ProcessUsingVCH : {:?}, ProcessUsingFTH : {:?}, ReservedBits0 : {:?} }}",
            self.ProcessInJob(),
            self.ProcessInitializing(),
            self.ProcessUsingVEH(),
            self.ProcessUsingVCH(),
            self.ProcessUsingFTH(),
            self.ReservedBits0()
        )
    }
}

impl PEB32_2_1 {
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
    pub fn ReservedBits0(&self) -> u32 {
        self._bitfield_1.get(5usize, 27u8) as u32
    }

    #[inline]
    pub fn set_ReservedBits0(&mut self, val: u32) {
        self._bitfield_1.set(5usize, 27u8, val as u64)
    }

    #[inline]
    pub fn new_bitfield_1(
        ProcessInJob: u32,
        ProcessInitializing: u32,
        ProcessUsingVEH: u32,
        ProcessUsingVCH: u32,
        ProcessUsingFTH: u32,
        ReservedBits0: u32,
    ) -> BitfieldUnit<[u8; 4]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 4]> = Default::default();

        bitfield_unit.set(0usize, 1u8, ProcessInJob as u64);

        bitfield_unit.set(1usize, 1u8, ProcessInitializing as u64);

        bitfield_unit.set(2usize, 1u8, ProcessUsingVEH as u64);

        bitfield_unit.set(3usize, 1u8, ProcessUsingVCH as u64);

        bitfield_unit.set(4usize, 1u8, ProcessUsingFTH as u64);

        bitfield_unit.set(5usize, 27u8, ReservedBits0 as u64);

        bitfield_unit
    }
}

impl Default for PEB32_2 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PEB32_2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PEB32_2 {{ union }}")
    }
}

#[repr(C)]
pub struct PEB32_3 {
    pub KernelCallbackTable: UnionField<u32>,
    pub UserSharedInfoPtr: UnionField<u32>,
    pub union_field: u32,
}

impl Default for PEB32_3 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PEB32_3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PEB32_3 {{ union }}")
    }
}

#[repr(C)]
pub struct PEB32_4 {
    pub pContextData: UnionField<u32>,
    pub pUnused: UnionField<u32>,
    pub EcCodeBitMap: UnionField<u32>,
    pub union_field: u32,
}

impl Default for PEB32_4 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PEB32_4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PEB32_4 {{ union }}")
    }
}

#[repr(C)]
pub struct PEB32_5 {
    pub TracingFlags: UnionField<u32>,
    pub Anonymous1: UnionField<PEB32_5_1>,
    pub union_field: u32,
}

#[repr(C)]
#[repr(align(4))]
pub struct PEB32_5_1 {
    _bitfield_align_1: [u32; 0],
    _bitfield_1: BitfieldUnit<[u8; 4]>,
}

impl Default for PEB32_5_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PEB32_5_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PEB32_5_1 {{ HeapTracingEnabled : {:?}, CritSecTracingEnabled : {:?}, LibLoaderTracingEnabled : {:?}, SpareTracingBits : {:?} }}",
            self.HeapTracingEnabled(),
            self.CritSecTracingEnabled(),
            self.LibLoaderTracingEnabled(),
            self.SpareTracingBits()
        )
    }
}

impl PEB32_5_1 {
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

impl Default for PEB32_5 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PEB32_5 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PEB32_5 {{ union }}")
    }
}

#[repr(C)]
pub struct PEB32_6 {
    pub LeapSecondFlags: UnionField<u32>,
    pub Anonymous1: UnionField<PEB32_6_1>,
    pub union_field: u32,
}

#[repr(C)]
#[repr(align(4))]
pub struct PEB32_6_1 {
    _bitfield_align_1: [u32; 0],
    _bitfield_1: BitfieldUnit<[u8; 4]>,
}

impl Default for PEB32_6_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PEB32_6_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PEB32_6_1 {{ SixtySecondEnabled : {:?}, Reserved : {:?} }}",
            self.SixtySecondEnabled(),
            self.Reserved()
        )
    }
}

impl PEB32_6_1 {
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

impl Default for PEB32_6 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PEB32_6 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PEB32_6 {{ union }}")
    }
}

impl Default for PEB32 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PEB32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PEB32 {{ Anonymous1: {:?}, Anonymous2: {:?}, Anonymous3: {:?}, TlsBitmapBits: {:?}, TlsExpansionBitmapBits: {:?}, SparePointers: {:?}, SpareUlongs: {:?}, Anonymous4: {:?}, Anonymous5: {:?}, WaitOnAddressHashTable: {:?}, PlaceholderCompatibilityModeReserved: {:?}, Anonymous6: {:?} }}",
            self.Anonymous1,
            self.Anonymous2,
            self.Anonymous3,
            self.TlsBitmapBits,
            self.TlsExpansionBitmapBits,
            self.SparePointers,
            self.SpareUlongs,
            self.Anonymous4,
            self.Anonymous5,
            self.WaitOnAddressHashTable,
            self.PlaceholderCompatibilityModeReserved,
            self.Anonymous6
        )
    }
}

#[repr(C)]
pub struct GDI_TEB_BATCH32 {
    pub Offset: u32,
    pub HDC: u32,
    pub Buffer: [u32; 310],
}

impl Default for GDI_TEB_BATCH32 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for GDI_TEB_BATCH32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GDI_TEB_BATCH32 {{ Buffer: {:?} }}", self.Buffer)
    }
}

#[repr(C)]
pub struct TEB32 {
    pub NtTib: NT_TIB32,
    pub EnvironmentPointer: u32,
    pub ClientId: CLIENT_ID,
    pub ActiveRpcHandle: u32,
    pub ThreadLocalStoragePointer: u32,
    pub ProcessEnvironmentBlock: u32,
    pub LastErrorValue: u32,
    pub CountOfOwnedCriticalSections: u32,
    pub CsrClientThread: u32,
    pub Win32ThreadInfo: u32,
    pub User32Reserved: [u32; 26],
    pub UserReserved: [u32; 5],
    pub WOW32Reserved: u32,
    pub CurrentLocale: u32,
    pub FpSoftwareStatusRegister: u32,
    pub ReservedForDebuggerInstrumentation: [u32; 16],
    pub SystemReserved1: [u32; 36],
    pub WorkingOnBehalfTicket: [u8; 8],
    pub ExceptionCode: NTSTATUS,
    pub ActivationContextStackPointer: u32,
    pub InstrumentationCallbackSp: u32,
    pub InstrumentationCallbackPreviousPc: u32,
    pub InstrumentationCallbackPreviousSp: u32,
    pub InstrumentationCallbackDisabled: bool,
    pub SpareBytes: [u8; 23],
    pub TxFsContext: u32,
    pub GdiTebBatch: GDI_TEB_BATCH32,
    pub RealClientId: CLIENT_ID,
    pub GdiCachedProcessHandle: u32,
    pub GdiClientPID: u32,
    pub GdiClientTID: u32,
    pub GdiThreadLocalInfo: u32,
    pub Win32ClientInfo: [u32; 62],
    pub glDispatchTable: [u32; 233],
    pub glReserved1: [u32; 29],
    pub glReserved2: u32,
    pub glSectionInfo: u32,
    pub glSection: u32,
    pub glTable: u32,
    pub glCurrentRC: u32,
    pub glContext: u32,
    pub LastStatusValue: NTSTATUS,
    pub StaticUnicodeString: STRING32,
    pub StaticUnicodeBuffer: [u16; 261],
    pub DeallocationStack: u32,
    pub TlsSlots: [u32; 64],
    pub TlsLinks: LIST_ENTRY32,
    pub Vdm: u32,
    pub ReservedForNtRpc: u32,
    pub DbgSsReserved: [u32; 2],
    pub HardErrorMode: u32,
    pub Instrumentation: [u32; 9],
    pub ActivityId: GUID,
    pub SubProcessTag: u32,
    pub PerflibData: u32,
    pub EtwTraceData: u32,
    pub WinSockData: u32,
    pub GdiBatchCount: u32,
    pub Anonymous1: TEB32_1,
    pub GuaranteedStackBytes: u32,
    pub ReservedForPerf: u32,
    pub ReservedForOle: u32,
    pub WaitingOnLoaderLock: u32,
    pub SavedPriorityState: u32,
    pub ReservedForCodeCoverage: u32,
    pub ThreadPoolData: u32,
    pub TlsExpansionSlots: u32,
    pub MuiGeneration: u32,
    pub IsImpersonating: u32,
    pub NlsCache: u32,
    pub pShimData: u32,
    pub HeapVirtualAffinity: u16,
    pub LowFragHeapDataSlot: u16,
    pub CurrentTransactionHandle: u32,
    pub ActiveFrame: u32,
    pub FlsData: u32,
    pub PreferredLanguages: u32,
    pub UserPrefLanguages: u32,
    pub MergedPrefLanguages: u32,
    pub MuiImpersonation: u32,
    pub Anonymous2: TEB32_2,
    pub Anonymous3: TEB32_3,
    pub TxnScopeEnterCallback: u32,
    pub TxnScopeExitCallback: u32,
    pub TxnScopeContext: u32,
    pub LockCount: u32,
    pub WowTebOffset: i32,
    pub ResourceRetValue: u32,
    pub ReservedForWdf: u32,
    pub ReservedForCrt: u64,
    pub EffectiveContainerId: GUID,
}

#[repr(C)]
pub struct TEB32_1 {
    pub CurrentIdealProcessor: UnionField<PROCESSOR_NUMBER>,
    pub IdealProcessorValue: UnionField<u32>,
    pub Anonymous1: UnionField<TEB32_1_1>,
    pub union_field: u32,
}

#[repr(C)]
pub struct TEB32_1_1 {
    pub ReservedPad0: u8,
    pub ReservedPad1: u8,
    pub ReservedPad2: u8,
    pub IdealProcessor: u8,
}

impl Default for TEB32_1_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for TEB32_1_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TEB32_1_1 {{  }}")
    }
}

impl Default for TEB32_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for TEB32_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TEB32_1 {{ union }}")
    }
}

#[repr(C)]
pub struct TEB32_2 {
    pub CrossTebFlags: UnionField<u16>,
    _bitfield_align_1: [u16; 0],
    _bitfield_1: UnionField<BitfieldUnit<[u8; 2]>>,
    pub union_field: u16,
}

impl Default for TEB32_2 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for TEB32_2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TEB32_2 {{ union }}")
    }
}

impl TEB32_2 {
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
pub struct TEB32_3 {
    pub SameTebFlags: UnionField<u16>,
    pub Anonymous1: UnionField<TEB32_3_1>,
    pub union_field: u16,
}

#[repr(C)]
#[repr(align(2))]
pub struct TEB32_3_1 {
    _bitfield_align_1: [u8; 0],
    _bitfield_1: BitfieldUnit<[u8; 2]>,
}

impl Default for TEB32_3_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for TEB32_3_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TEB32_3_1 {{ SafeThunkCall : {:?}, InDebugPrint : {:?}, HasFiberData : {:?}, SkipThreadAttach : {:?}, WerInShipAssertCode : {:?}, RanProcessInit : {:?}, ClonedThread : {:?}, SuppressDebugMsg : {:?}, DisableUserStackWalk : {:?}, RtlExceptionAttached : {:?}, InitialThread : {:?}, SessionAware : {:?}, LoadOwner : {:?}, LoaderWorker : {:?}, SpareSameTebBits : {:?} }}",
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
            self.SpareSameTebBits()
        )
    }
}

impl TEB32_3_1 {
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
    pub fn SpareSameTebBits(&self) -> u16 {
        self._bitfield_1.get(14usize, 2u8) as u16
    }

    #[inline]
    pub fn set_SpareSameTebBits(&mut self, val: u16) {
        self._bitfield_1.set(14usize, 2u8, val as u64)
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
        SpareSameTebBits: u16,
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

        bitfield_unit.set(14usize, 2u8, SpareSameTebBits as u64);

        bitfield_unit
    }
}

impl Default for TEB32_3 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for TEB32_3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TEB32_3 {{ union }}")
    }
}

impl Default for TEB32 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for TEB32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TEB32 {{ User32Reserved: {:?}, UserReserved: {:?}, ReservedForDebuggerInstrumentation: {:?}, SystemReserved1: {:?}, WorkingOnBehalfTicket: {:?}, SpareBytes: {:?}, GdiTebBatch: {:?}, Win32ClientInfo: {:?}, glDispatchTable: {:?}, glReserved1: {:?}, StaticUnicodeBuffer: {:?}, TlsSlots: {:?}, DbgSsReserved: {:?}, Instrumentation: {:?}, Anonymous1: {:?}, Anonymous2: {:?}, Anonymous3: {:?} }}",
            self.User32Reserved,
            self.UserReserved,
            self.ReservedForDebuggerInstrumentation,
            self.SystemReserved1,
            self.WorkingOnBehalfTicket,
            self.SpareBytes,
            self.GdiTebBatch,
            self.Win32ClientInfo,
            self.glDispatchTable,
            self.glReserved1,
            self.StaticUnicodeBuffer,
            self.TlsSlots,
            self.DbgSsReserved,
            self.Instrumentation,
            self.Anonymous1,
            self.Anonymous2,
            self.Anonymous3
        )
    }
}

#[repr(C)]
pub struct WOW64_EXECUTE_OPTIONS {
    pub Flags: UnionField<u32>,
    pub Anonymous1: UnionField<WOW64_EXECUTE_OPTIONS_1>,
    pub union_field: u32,
}

#[repr(C)]
#[repr(align(4))]
pub struct WOW64_EXECUTE_OPTIONS_1 {
    _bitfield_align_1: [u16; 0],
    _bitfield_1: BitfieldUnit<[u8; 4]>,
}

impl Default for WOW64_EXECUTE_OPTIONS_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for WOW64_EXECUTE_OPTIONS_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "WOW64_EXECUTE_OPTIONS_1 {{ StackReserveSize : {:?}, StackCommitSize : {:?}, Deprecated0 : {:?}, DisableWowAssert : {:?}, DisableTurboDispatch : {:?}, Unused : {:?}, Reserved0 : {:?}, Reserved1 : {:?}, Reserved2 : {:?}, Reserved3 : {:?} }}",
            self.StackReserveSize(),
            self.StackCommitSize(),
            self.Deprecated0(),
            self.DisableWowAssert(),
            self.DisableTurboDispatch(),
            self.Unused(),
            self.Reserved0(),
            self.Reserved1(),
            self.Reserved2(),
            self.Reserved3()
        )
    }
}

impl WOW64_EXECUTE_OPTIONS_1 {
    #[inline]
    pub fn StackReserveSize(&self) -> u32 {
        self._bitfield_1.get(0usize, 8u8) as u32
    }

    #[inline]
    pub fn set_StackReserveSize(&mut self, val: u32) {
        self._bitfield_1.set(0usize, 8u8, val as u64)
    }

    #[inline]
    pub fn StackCommitSize(&self) -> u32 {
        self._bitfield_1.get(8usize, 4u8) as u32
    }

    #[inline]
    pub fn set_StackCommitSize(&mut self, val: u32) {
        self._bitfield_1.set(8usize, 4u8, val as u64)
    }

    #[inline]
    pub fn Deprecated0(&self) -> u32 {
        self._bitfield_1.get(12usize, 1u8) as u32
    }

    #[inline]
    pub fn set_Deprecated0(&mut self, val: u32) {
        self._bitfield_1.set(12usize, 1u8, val as u64)
    }

    #[inline]
    pub fn DisableWowAssert(&self) -> u32 {
        self._bitfield_1.get(13usize, 1u8) as u32
    }

    #[inline]
    pub fn set_DisableWowAssert(&mut self, val: u32) {
        self._bitfield_1.set(13usize, 1u8, val as u64)
    }

    #[inline]
    pub fn DisableTurboDispatch(&self) -> u32 {
        self._bitfield_1.get(14usize, 1u8) as u32
    }

    #[inline]
    pub fn set_DisableTurboDispatch(&mut self, val: u32) {
        self._bitfield_1.set(14usize, 1u8, val as u64)
    }

    #[inline]
    pub fn Unused(&self) -> u32 {
        self._bitfield_1.get(15usize, 13u8) as u32
    }

    #[inline]
    pub fn set_Unused(&mut self, val: u32) {
        self._bitfield_1.set(15usize, 13u8, val as u64)
    }

    #[inline]
    pub fn Reserved0(&self) -> u32 {
        self._bitfield_1.get(28usize, 1u8) as u32
    }

    #[inline]
    pub fn set_Reserved0(&mut self, val: u32) {
        self._bitfield_1.set(28usize, 1u8, val as u64)
    }

    #[inline]
    pub fn Reserved1(&self) -> u32 {
        self._bitfield_1.get(29usize, 1u8) as u32
    }

    #[inline]
    pub fn set_Reserved1(&mut self, val: u32) {
        self._bitfield_1.set(29usize, 1u8, val as u64)
    }

    #[inline]
    pub fn Reserved2(&self) -> u32 {
        self._bitfield_1.get(30usize, 1u8) as u32
    }

    #[inline]
    pub fn set_Reserved2(&mut self, val: u32) {
        self._bitfield_1.set(30usize, 1u8, val as u64)
    }

    #[inline]
    pub fn Reserved3(&self) -> u32 {
        self._bitfield_1.get(31usize, 1u8) as u32
    }

    #[inline]
    pub fn set_Reserved3(&mut self, val: u32) {
        self._bitfield_1.set(31usize, 1u8, val as u64)
    }

    #[inline]
    pub fn new_bitfield_1(
        StackReserveSize: u32,
        StackCommitSize: u32,
        Deprecated0: u32,
        DisableWowAssert: u32,
        DisableTurboDispatch: u32,
        Unused: u32,
        Reserved0: u32,
        Reserved1: u32,
        Reserved2: u32,
        Reserved3: u32,
    ) -> BitfieldUnit<[u8; 4]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 4]> = Default::default();

        bitfield_unit.set(0usize, 8u8, StackReserveSize as u64);

        bitfield_unit.set(8usize, 4u8, StackCommitSize as u64);

        bitfield_unit.set(12usize, 1u8, Deprecated0 as u64);

        bitfield_unit.set(13usize, 1u8, DisableWowAssert as u64);

        bitfield_unit.set(14usize, 1u8, DisableTurboDispatch as u64);

        bitfield_unit.set(15usize, 13u8, Unused as u64);

        bitfield_unit.set(28usize, 1u8, Reserved0 as u64);

        bitfield_unit.set(29usize, 1u8, Reserved1 as u64);

        bitfield_unit.set(30usize, 1u8, Reserved2 as u64);

        bitfield_unit.set(31usize, 1u8, Reserved3 as u64);

        bitfield_unit
    }
}

impl Default for WOW64_EXECUTE_OPTIONS {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for WOW64_EXECUTE_OPTIONS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WOW64_EXECUTE_OPTIONS {{ union }}")
    }
}

#[repr(C)]
pub struct WOW64INFO {
    pub NativeSystemPageSize: u32,
    pub CpuFlags: u32,
    pub Wow64ExecuteFlags: WOW64_EXECUTE_OPTIONS,
    pub InstrumentationCallback: u32,
}

impl Default for WOW64INFO {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for WOW64INFO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "WOW64INFO {{ Wow64ExecuteFlags: {:?} }}",
            self.Wow64ExecuteFlags
        )
    }
}

#[repr(C)]
pub struct PEB32_WITH_WOW64INFO {
    pub Peb32: PEB32,
    pub Wow64Info: WOW64INFO,
}

impl Default for PEB32_WITH_WOW64INFO {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PEB32_WITH_WOW64INFO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PEB32_WITH_WOW64INFO {{ Peb32: {:?}, Wow64Info: {:?} }}",
            self.Peb32, self.Wow64Info
        )
    }
}
