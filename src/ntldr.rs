use windows::{
    core::{PCWSTR, PSTR, PWSTR},
    Wdk::Foundation::OBJECT_ATTRIBUTES,
    Win32::{
        Foundation::{HANDLE, NTSTATUS, UNICODE_STRING},
        System::{
            Kernel::{LIST_ENTRY, RTL_BALANCED_NODE, SINGLE_LIST_ENTRY, STRING},
            SystemServices::{
                IMAGE_BASE_RELOCATION, IMAGE_RESOURCE_DATA_ENTRY, IMAGE_RESOURCE_DIRECTORY,
                IMAGE_RESOURCE_DIRECTORY_STRING,
            },
            WindowsProgramming::{
                IMAGE_DELAYLOAD_DESCRIPTOR, IMAGE_THUNK_DATA64, PDELAYLOAD_FAILURE_DLL_CALLBACK,
            },
        },
    },
};

use crate::{
    bitfield::{BitfieldUnit, UnionField},
    ntsxs::ACTIVATION_CONTEXT,
};

pub const LDRP_PACKAGED_BINARY: u32 = 1;
pub const LDRP_MARKED_FOR_REMOVAL: u32 = 2;
pub const LDRP_IMAGE_DLL: u32 = 4;
pub const LDRP_LOAD_NOTIFICATIONS_SENT: u32 = 8;
pub const LDRP_TELEMETRY_ENTRY_PROCESSED: u32 = 16;
pub const LDRP_PROCESS_STATIC_IMPORT: u32 = 32;
pub const LDRP_IN_LEGACY_LISTS: u32 = 64;
pub const LDRP_IN_INDEXES: u32 = 128;
pub const LDRP_SHIM_DLL: u32 = 256;
pub const LDRP_IN_EXCEPTION_TABLE: u32 = 512;
pub const LDRP_LOAD_IN_PROGRESS: u32 = 4096;
pub const LDRP_LOAD_CONFIG_PROCESSED: u32 = 8192;
pub const LDRP_ENTRY_PROCESSED: u32 = 16384;
pub const LDRP_PROTECT_DELAY_LOAD: u32 = 32768;
pub const LDRP_DONT_CALL_FOR_THREADS: u32 = 262144;
pub const LDRP_PROCESS_ATTACH_CALLED: u32 = 524288;
pub const LDRP_PROCESS_ATTACH_FAILED: u32 = 1048576;
pub const LDRP_COR_DEFERRED_VALIDATE: u32 = 2097152;
pub const LDRP_COR_IMAGE: u32 = 4194304;
pub const LDRP_DONT_RELOCATE: u32 = 8388608;
pub const LDRP_COR_IL_ONLY: u32 = 16777216;
pub const LDRP_CHPE_IMAGE: u32 = 33554432;
pub const LDRP_CHPE_EMULATOR_IMAGE: u32 = 67108864;
pub const LDRP_REDIRECTED: u32 = 268435456;
pub const LDRP_COMPAT_DATABASE_PROCESSED: u32 = 2147483648;
pub const LDR_DATA_TABLE_ENTRY_SIZE_WINXP: u32 = 152;
pub const LDR_DATA_TABLE_ENTRY_SIZE_WIN7: u32 = 264;
pub const LDR_DATA_TABLE_ENTRY_SIZE_WIN8: u32 = 272;
pub const LDR_DATA_TABLE_ENTRY_SIZE_WIN10: u32 = 284;
pub const LDR_DATA_TABLE_ENTRY_SIZE_WIN11: u32 = 312;
pub const LDR_GET_DLL_HANDLE_EX_UNCHANGED_REFCOUNT: u32 = 1;
pub const LDR_GET_DLL_HANDLE_EX_PIN: u32 = 2;
pub const LDR_ADDREF_DLL_PIN: u32 = 1;
pub const LDR_GET_PROCEDURE_ADDRESS_DONT_RECORD_FORWARDER: u32 = 1;
pub const LDR_LOCK_LOADER_LOCK_FLAG_RAISE_ON_ERRORS: u32 = 1;
pub const LDR_LOCK_LOADER_LOCK_FLAG_TRY_ONLY: u32 = 2;
pub const LDR_LOCK_LOADER_LOCK_DISPOSITION_INVALID: u32 = 0;
pub const LDR_LOCK_LOADER_LOCK_DISPOSITION_LOCK_ACQUIRED: u32 = 1;
pub const LDR_LOCK_LOADER_LOCK_DISPOSITION_LOCK_NOT_ACQUIRED: u32 = 2;
pub const LDR_UNLOCK_LOADER_LOCK_FLAG_RAISE_ON_ERRORS: u32 = 1;
pub const LDR_DLL_NOTIFICATION_REASON_LOADED: u32 = 1;
pub const LDR_DLL_NOTIFICATION_REASON_UNLOADED: u32 = 2;
pub const RESOURCE_TYPE_LEVEL: u32 = 0;
pub const RESOURCE_NAME_LEVEL: u32 = 1;
pub const RESOURCE_LANGUAGE_LEVEL: u32 = 2;
pub const RESOURCE_DATA_LEVEL: u32 = 3;

pub type PLDR_INIT_ROUTINE = std::option::Option<
    unsafe extern "system" fn(
        DllHandle: *mut std::ffi::c_void,
        Reason: u32,
        Context: *mut std::ffi::c_void,
    ) -> bool,
>;

#[repr(C)]
pub struct LDR_SERVICE_TAG_RECORD {
    pub Next: *mut LDR_SERVICE_TAG_RECORD,
    pub ServiceTag: u32,
}

impl Default for LDR_SERVICE_TAG_RECORD {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for LDR_SERVICE_TAG_RECORD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LDR_SERVICE_TAG_RECORD {{ Next: {:?} }}", self.Next)
    }
}

#[repr(C)]
pub struct LDRP_CSLIST {
    pub Tail: *mut SINGLE_LIST_ENTRY,
}

impl Default for LDRP_CSLIST {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for LDRP_CSLIST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LDRP_CSLIST {{  }}")
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum LDR_DDAG_STATE {
    LdrModulesMerged = -5,
    LdrModulesInitError = -4,
    LdrModulesSnapError = -3,
    LdrModulesUnloaded = -2,
    LdrModulesUnloading = -1,
    LdrModulesPlaceHolder = 0,
    LdrModulesMapping = 1,
    LdrModulesMapped = 2,
    LdrModulesWaitingForDependencies = 3,
    LdrModulesSnapping = 4,
    LdrModulesSnapped = 5,
    LdrModulesCondensed = 6,
    LdrModulesReadyToInit = 7,
    LdrModulesInitializing = 8,
    LdrModulesReadyToRun = 9,
}

#[repr(C)]
pub struct LDR_DDAG_NODE {
    pub Modules: LIST_ENTRY,
    pub ServiceTagList: *mut LDR_SERVICE_TAG_RECORD,
    pub LoadCount: u32,
    pub LoadWhileUnloadingCount: u32,
    pub LowestLink: u32,
    pub Anonymous1: LDR_DDAG_NODE_1,
    pub IncomingDependencies: LDRP_CSLIST,
    pub State: LDR_DDAG_STATE,
    pub CondenseLink: SINGLE_LIST_ENTRY,
    pub PreorderNumber: u32,
}

#[repr(C)]
pub struct LDR_DDAG_NODE_1 {
    pub Dependencies: UnionField<LDRP_CSLIST>,
    pub RemovalLink: UnionField<SINGLE_LIST_ENTRY>,
    pub union_field: u64,
}

impl Default for LDR_DDAG_NODE_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for LDR_DDAG_NODE_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LDR_DDAG_NODE_1 {{ union }}")
    }
}

impl Default for LDR_DDAG_NODE {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for LDR_DDAG_NODE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LDR_DDAG_NODE {{ ServiceTagList: {:?}, Anonymous1: {:?}, IncomingDependencies: {:?}, State: {:?} }}",
            self.ServiceTagList, self.Anonymous1, self.IncomingDependencies, self.State
        )
    }
}

#[repr(C)]
pub struct LDR_DEPENDENCY_RECORD {
    pub DependencyLink: SINGLE_LIST_ENTRY,
    pub DependencyNode: *mut LDR_DDAG_NODE,
    pub IncomingDependencyLink: SINGLE_LIST_ENTRY,
    pub IncomingDependencyNode: *mut LDR_DDAG_NODE,
}

impl Default for LDR_DEPENDENCY_RECORD {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for LDR_DEPENDENCY_RECORD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LDR_DEPENDENCY_RECORD {{ DependencyNode: {:?}, IncomingDependencyNode: {:?} }}",
            self.DependencyNode, self.IncomingDependencyNode
        )
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum LDR_DLL_LOAD_REASON {
    LoadReasonStaticDependency = 0,
    LoadReasonStaticForwarderDependency = 1,
    LoadReasonDynamicForwarderDependency = 2,
    LoadReasonDelayloadDependency = 3,
    LoadReasonDynamicLoad = 4,
    LoadReasonAsImageLoad = 5,
    LoadReasonAsDataLoad = 6,
    LoadReasonEnclavePrimary = 7,
    LoadReasonEnclaveDependency = 8,
    LoadReasonPatchImage = 9,
    LoadReasonUnknown = -1,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum LDR_HOT_PATCH_STATE {
    LdrHotPatchBaseImage = 0,
    LdrHotPatchNotApplied = 1,
    LdrHotPatchAppliedReverse = 2,
    LdrHotPatchAppliedForward = 3,
    LdrHotPatchFailedToPatch = 4,
    LdrHotPatchStateMax = 5,
}

#[repr(C)]
pub struct LDR_DATA_TABLE_ENTRY {
    pub InLoadOrderLinks: LIST_ENTRY,
    pub InMemoryOrderLinks: LIST_ENTRY,
    pub Anonymous1: LDR_DATA_TABLE_ENTRY_1,
    pub DllBase: *mut std::ffi::c_void,
    pub EntryPoint: PLDR_INIT_ROUTINE,
    pub SizeOfImage: u32,
    pub FullDllName: UNICODE_STRING,
    pub BaseDllName: UNICODE_STRING,
    pub Anonymous2: LDR_DATA_TABLE_ENTRY_2,
    pub ObsoleteLoadCount: u16,
    pub TlsIndex: u16,
    pub HashLinks: LIST_ENTRY,
    pub TimeDateStamp: u32,
    pub EntryPointActivationContext: *mut ACTIVATION_CONTEXT,
    pub Lock: *mut std::ffi::c_void,
    pub DdagNode: *mut LDR_DDAG_NODE,
    pub NodeModuleLink: LIST_ENTRY,
    pub LoadContext: *mut std::ffi::c_void,
    pub ParentDllBase: *mut std::ffi::c_void,
    pub SwitchBackContext: *mut std::ffi::c_void,
    pub BaseAddressIndexNode: RTL_BALANCED_NODE,
    pub MappingInfoIndexNode: RTL_BALANCED_NODE,
    pub OriginalBase: usize,
    pub LoadTime: i64,
    pub BaseNameHashValue: u32,
    pub LoadReason: LDR_DLL_LOAD_REASON,
    pub ImplicitPathOptions: u32,
    pub ReferenceCount: u32,
    pub DependentLoadFlags: u32,
    pub SigningLevel: u8,
    pub CheckSum: u32,
    pub ActivePatchImageBase: *mut std::ffi::c_void,
    pub HotPatchState: LDR_HOT_PATCH_STATE,
}

#[repr(C)]
pub struct LDR_DATA_TABLE_ENTRY_1 {
    pub InInitializationOrderLinks: UnionField<LIST_ENTRY>,
    pub InProgressLinks: UnionField<LIST_ENTRY>,
    pub union_field: [u64; 2],
}

impl Default for LDR_DATA_TABLE_ENTRY_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for LDR_DATA_TABLE_ENTRY_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LDR_DATA_TABLE_ENTRY_1 {{ union }}")
    }
}

#[repr(C)]
pub struct LDR_DATA_TABLE_ENTRY_2 {
    pub FlagGroup: UnionField<[u8; 4]>,
    pub Flags: UnionField<u32>,
    pub Anonymous1: UnionField<LDR_DATA_TABLE_ENTRY_2_1>,
    pub union_field: u32,
}

#[repr(C)]
#[repr(align(4))]
pub struct LDR_DATA_TABLE_ENTRY_2_1 {
    _bitfield_align_1: [u8; 0],
    _bitfield_1: BitfieldUnit<[u8; 4]>,
}

impl Default for LDR_DATA_TABLE_ENTRY_2_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for LDR_DATA_TABLE_ENTRY_2_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LDR_DATA_TABLE_ENTRY_2_1 {{ PackagedBinary : {:?}, MarkedForRemoval : {:?}, ImageDll : {:?}, LoadNotificationsSent : {:?}, TelemetryEntryProcessed : {:?}, ProcessStaticImport : {:?}, InLegacyLists : {:?}, InIndexes : {:?}, ShimDll : {:?}, InExceptionTable : {:?}, ReservedFlags1 : {:?}, LoadInProgress : {:?}, LoadConfigProcessed : {:?}, EntryProcessed : {:?}, ProtectDelayLoad : {:?}, ReservedFlags3 : {:?}, DontCallForThreads : {:?}, ProcessAttachCalled : {:?}, ProcessAttachFailed : {:?}, CorDeferredValidate : {:?}, CorImage : {:?}, DontRelocate : {:?}, CorILOnly : {:?}, ChpeImage : {:?}, ChpeEmulatorImage : {:?}, ReservedFlags5 : {:?}, Redirected : {:?}, ReservedFlags6 : {:?}, CompatDatabaseProcessed : {:?} }}",
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
            self.ChpeEmulatorImage(),
            self.ReservedFlags5(),
            self.Redirected(),
            self.ReservedFlags6(),
            self.CompatDatabaseProcessed()
        )
    }
}

impl LDR_DATA_TABLE_ENTRY_2_1 {
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
    pub fn ChpeEmulatorImage(&self) -> u32 {
        self._bitfield_1.get(26usize, 1u8) as u32
    }

    #[inline]
    pub fn set_ChpeEmulatorImage(&mut self, val: u32) {
        self._bitfield_1.set(26usize, 1u8, val as u64)
    }

    #[inline]
    pub fn ReservedFlags5(&self) -> u32 {
        self._bitfield_1.get(27usize, 1u8) as u32
    }

    #[inline]
    pub fn set_ReservedFlags5(&mut self, val: u32) {
        self._bitfield_1.set(27usize, 1u8, val as u64)
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
        ChpeEmulatorImage: u32,
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

        bitfield_unit.set(26usize, 1u8, ChpeEmulatorImage as u64);

        bitfield_unit.set(27usize, 1u8, ReservedFlags5 as u64);

        bitfield_unit.set(28usize, 1u8, Redirected as u64);

        bitfield_unit.set(29usize, 2u8, ReservedFlags6 as u64);

        bitfield_unit.set(31usize, 1u8, CompatDatabaseProcessed as u64);

        bitfield_unit
    }
}

impl Default for LDR_DATA_TABLE_ENTRY_2 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for LDR_DATA_TABLE_ENTRY_2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LDR_DATA_TABLE_ENTRY_2 {{ union }}")
    }
}

impl Default for LDR_DATA_TABLE_ENTRY {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for LDR_DATA_TABLE_ENTRY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LDR_DATA_TABLE_ENTRY {{ Anonymous1: {:?}, EntryPoint: {:?}, Anonymous2: {:?}, EntryPointActivationContext: {:?}, DdagNode: {:?}, LoadContext: {:?}, LoadReason: {:?}, HotPatchState: {:?} }}",
            self.Anonymous1,
            self.EntryPoint,
            self.Anonymous2,
            self.EntryPointActivationContext,
            self.DdagNode,
            self.LoadContext,
            self.LoadReason,
            self.HotPatchState
        )
    }
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrLoadDll(
        DllPath: PWSTR,
        DllCharacteristics: *mut u32,
        DllName: *mut UNICODE_STRING,
        DllHandle: *mut *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrUnloadDll(DllHandle: *mut std::ffi::c_void) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrGetDllHandle(
        DllPath: PWSTR,
        DllCharacteristics: *mut u32,
        DllName: *mut UNICODE_STRING,
        DllHandle: *mut *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrGetDllHandleEx(
        Flags: u32,
        DllPath: PWSTR,
        DllCharacteristics: *mut u32,
        DllName: *mut UNICODE_STRING,
        DllHandle: *mut *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrGetDllHandleByMapping(
        BaseAddress: *mut std::ffi::c_void,
        DllHandle: *mut *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrGetDllHandleByName(
        BaseDllName: *mut UNICODE_STRING,
        FullDllName: *mut UNICODE_STRING,
        DllHandle: *mut *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrGetDllFullName(
        DllHandle: *mut std::ffi::c_void,
        FullDllName: *mut UNICODE_STRING,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrGetDllPath(
        DllName: PCWSTR,
        Flags: u32,
        DllPath: *mut PWSTR,
        SearchPaths: *mut PWSTR,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrGetDllDirectory(DllDirectory: *mut UNICODE_STRING) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrSetDllDirectory(DllDirectory: *mut UNICODE_STRING) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrAddRefDll(Flags: u32, DllHandle: *mut std::ffi::c_void) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrGetProcedureAddress(
        DllHandle: *mut std::ffi::c_void,
        ProcedureName: *mut STRING,
        ProcedureNumber: u32,
        ProcedureAddress: *mut *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrGetProcedureAddressEx(
        DllHandle: *mut std::ffi::c_void,
        ProcedureName: *mut STRING,
        ProcedureNumber: u32,
        ProcedureAddress: *mut *mut std::ffi::c_void,
        Flags: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrGetKnownDllSectionHandle(
        DllName: PCWSTR,
        KnownDlls32: bool,
        Section: *mut HANDLE,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrGetProcedureAddressForCaller(
        DllHandle: *mut std::ffi::c_void,
        ProcedureName: *mut STRING,
        ProcedureNumber: u32,
        ProcedureAddress: *mut *mut std::ffi::c_void,
        Flags: u32,
        Callback: *mut *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrLockLoaderLock(
        Flags: u32,
        Disposition: *mut u32,
        Cookie: *mut *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrUnlockLoaderLock(Flags: u32, Cookie: *mut std::ffi::c_void) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrRelocateImage(
        NewBase: *mut std::ffi::c_void,
        LoaderName: PSTR,
        Success: NTSTATUS,
        Conflict: NTSTATUS,
        Invalid: NTSTATUS,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrRelocateImageWithBias(
        NewBase: *mut std::ffi::c_void,
        Bias: i64,
        LoaderName: PSTR,
        Success: NTSTATUS,
        Conflict: NTSTATUS,
        Invalid: NTSTATUS,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrProcessRelocationBlock(
        VA: usize,
        SizeOfBlock: u32,
        NextOffset: *mut u16,
        Diff: isize,
    ) -> *mut IMAGE_BASE_RELOCATION;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrProcessRelocationBlockEx(
        Machine: u32,
        VA: usize,
        SizeOfBlock: u32,
        NextOffset: *mut u16,
        Diff: isize,
    ) -> *mut IMAGE_BASE_RELOCATION;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrVerifyMappedImageMatchesChecksum(
        BaseAddress: *mut std::ffi::c_void,
        NumberOfBytes: usize,
        FileLength: u32,
    ) -> bool;
}

pub type PLDR_IMPORT_MODULE_CALLBACK = std::option::Option<
    unsafe extern "system" fn(Parameter: *mut std::ffi::c_void, ModuleName: PSTR),
>;

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrVerifyImageMatchesChecksum(
        ImageFileHandle: HANDLE,
        ImportCallbackRoutine: PLDR_IMPORT_MODULE_CALLBACK,
        ImportCallbackParameter: *mut std::ffi::c_void,
        ImageCharacteristics: *mut u16,
    ) -> NTSTATUS;
}

#[repr(C)]
pub struct LDR_IMPORT_CALLBACK_INFO {
    pub ImportCallbackRoutine: PLDR_IMPORT_MODULE_CALLBACK,
    pub ImportCallbackParameter: *mut std::ffi::c_void,
}

impl Default for LDR_IMPORT_CALLBACK_INFO {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for LDR_IMPORT_CALLBACK_INFO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LDR_IMPORT_CALLBACK_INFO {{ ImportCallbackRoutine: {:?} }}",
            self.ImportCallbackRoutine
        )
    }
}

#[repr(C)]
pub struct LDR_SECTION_INFO {
    pub SectionHandle: HANDLE,
    pub DesiredAccess: u32,
    pub ObjA: *mut OBJECT_ATTRIBUTES,
    pub SectionPageProtection: u32,
    pub AllocationAttributes: u32,
}

impl Default for LDR_SECTION_INFO {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for LDR_SECTION_INFO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LDR_SECTION_INFO {{  }}")
    }
}

#[repr(C)]
pub struct LDR_VERIFY_IMAGE_INFO {
    pub Size: u32,
    pub Flags: u32,
    pub CallbackInfo: LDR_IMPORT_CALLBACK_INFO,
    pub SectionInfo: LDR_SECTION_INFO,
    pub ImageCharacteristics: u16,
}

impl Default for LDR_VERIFY_IMAGE_INFO {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for LDR_VERIFY_IMAGE_INFO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LDR_VERIFY_IMAGE_INFO {{ CallbackInfo: {:?}, SectionInfo: {:?} }}",
            self.CallbackInfo, self.SectionInfo
        )
    }
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrVerifyImageMatchesChecksumEx(
        ImageFileHandle: HANDLE,
        VerifyInfo: *mut LDR_VERIFY_IMAGE_INFO,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrQueryModuleServiceTags(
        DllHandle: *mut std::ffi::c_void,
        ServiceTagBuffer: *mut u32,
        BufferSize: *mut u32,
    ) -> NTSTATUS;
}

#[repr(C)]
pub struct LDR_DLL_LOADED_NOTIFICATION_DATA {
    pub Flags: u32,
    pub FullDllName: *mut UNICODE_STRING,
    pub BaseDllName: *mut UNICODE_STRING,
    pub DllBase: *mut std::ffi::c_void,
    pub SizeOfImage: u32,
}

impl Default for LDR_DLL_LOADED_NOTIFICATION_DATA {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for LDR_DLL_LOADED_NOTIFICATION_DATA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LDR_DLL_LOADED_NOTIFICATION_DATA {{  }}")
    }
}

#[repr(C)]
pub struct LDR_DLL_UNLOADED_NOTIFICATION_DATA {
    pub Flags: u32,
    pub FullDllName: *const UNICODE_STRING,
    pub BaseDllName: *const UNICODE_STRING,
    pub DllBase: *mut std::ffi::c_void,
    pub SizeOfImage: u32,
}

impl Default for LDR_DLL_UNLOADED_NOTIFICATION_DATA {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for LDR_DLL_UNLOADED_NOTIFICATION_DATA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LDR_DLL_UNLOADED_NOTIFICATION_DATA {{  }}")
    }
}

#[repr(C)]
pub struct LDR_DLL_NOTIFICATION_DATA {
    pub Loaded: UnionField<LDR_DLL_LOADED_NOTIFICATION_DATA>,
    pub Unloaded: UnionField<LDR_DLL_UNLOADED_NOTIFICATION_DATA>,
    pub union_field: [u64; 5],
}

impl Default for LDR_DLL_NOTIFICATION_DATA {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for LDR_DLL_NOTIFICATION_DATA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LDR_DLL_NOTIFICATION_DATA {{ union }}")
    }
}

pub type PLDR_DLL_NOTIFICATION_FUNCTION = std::option::Option<
    unsafe extern "system" fn(
        NotificationReason: u32,
        NotificationData: *mut LDR_DLL_NOTIFICATION_DATA,
        Context: *mut std::ffi::c_void,
    ),
>;

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrRegisterDllNotification(
        Flags: u32,
        NotificationFunction: PLDR_DLL_NOTIFICATION_FUNCTION,
        Context: *mut std::ffi::c_void,
        Cookie: *mut *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrUnregisterDllNotification(Cookie: *mut std::ffi::c_void) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrStandardizeSystemPath(SystemPath: *mut UNICODE_STRING) -> *mut UNICODE_STRING;
}

#[repr(C)]
pub struct LDR_FAILURE_DATA {
    pub Status: NTSTATUS,
    pub DllName: [u16; 32],
    pub AdditionalInfo: [u16; 32],
}

impl Default for LDR_FAILURE_DATA {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for LDR_FAILURE_DATA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LDR_FAILURE_DATA {{ DllName: {:?}, AdditionalInfo: {:?} }}",
            self.DllName, self.AdditionalInfo
        )
    }
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrGetFailureData() -> *mut LDR_FAILURE_DATA;
}

#[repr(C)]
pub struct PS_MITIGATION_OPTIONS_MAP {
    pub Map: [usize; 3],
}

impl Default for PS_MITIGATION_OPTIONS_MAP {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PS_MITIGATION_OPTIONS_MAP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PS_MITIGATION_OPTIONS_MAP {{ Map: {:?} }}", self.Map)
    }
}

#[repr(C)]
pub struct PS_MITIGATION_AUDIT_OPTIONS_MAP {
    pub Map: [usize; 3],
}

impl Default for PS_MITIGATION_AUDIT_OPTIONS_MAP {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PS_MITIGATION_AUDIT_OPTIONS_MAP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PS_MITIGATION_AUDIT_OPTIONS_MAP {{ Map: {:?} }}",
            self.Map
        )
    }
}

#[repr(C)]
pub struct PS_SYSTEM_DLL_INIT_BLOCK {
    pub Size: u32,
    pub SystemDllWowRelocation: usize,
    pub SystemDllNativeRelocation: usize,
    pub Wow64SharedInformation: [usize; 16],
    pub RngData: u32,
    pub Anonymous1: PS_SYSTEM_DLL_INIT_BLOCK_1,
    pub MitigationOptionsMap: PS_MITIGATION_OPTIONS_MAP,
    pub CfgBitMap: usize,
    pub CfgBitMapSize: usize,
    pub Wow64CfgBitMap: usize,
    pub Wow64CfgBitMapSize: usize,
    pub MitigationAuditOptionsMap: PS_MITIGATION_AUDIT_OPTIONS_MAP,
}

#[repr(C)]
pub struct PS_SYSTEM_DLL_INIT_BLOCK_1 {
    pub Flags: UnionField<u32>,
    pub Anonymous1: UnionField<PS_SYSTEM_DLL_INIT_BLOCK_1_1>,
    pub union_field: u32,
}

#[repr(C)]
#[repr(align(4))]
pub struct PS_SYSTEM_DLL_INIT_BLOCK_1_1 {
    _bitfield_align_1: [u32; 0],
    _bitfield_1: BitfieldUnit<[u8; 4]>,
}

impl Default for PS_SYSTEM_DLL_INIT_BLOCK_1_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PS_SYSTEM_DLL_INIT_BLOCK_1_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PS_SYSTEM_DLL_INIT_BLOCK_1_1 {{ CfgOverride : {:?}, Reserved : {:?} }}",
            self.CfgOverride(),
            self.Reserved()
        )
    }
}

impl PS_SYSTEM_DLL_INIT_BLOCK_1_1 {
    #[inline]
    pub fn CfgOverride(&self) -> u32 {
        self._bitfield_1.get(0usize, 1u8) as u32
    }

    #[inline]
    pub fn set_CfgOverride(&mut self, val: u32) {
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
    pub fn new_bitfield_1(CfgOverride: u32, Reserved: u32) -> BitfieldUnit<[u8; 4]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 4]> = Default::default();

        bitfield_unit.set(0usize, 1u8, CfgOverride as u64);

        bitfield_unit.set(1usize, 31u8, Reserved as u64);

        bitfield_unit
    }
}

impl Default for PS_SYSTEM_DLL_INIT_BLOCK_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PS_SYSTEM_DLL_INIT_BLOCK_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PS_SYSTEM_DLL_INIT_BLOCK_1 {{ union }}")
    }
}

impl Default for PS_SYSTEM_DLL_INIT_BLOCK {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PS_SYSTEM_DLL_INIT_BLOCK {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PS_SYSTEM_DLL_INIT_BLOCK {{ Wow64SharedInformation: {:?}, Anonymous1: {:?}, MitigationOptionsMap: {:?}, MitigationAuditOptionsMap: {:?} }}",
            self.Wow64SharedInformation,
            self.Anonymous1,
            self.MitigationOptionsMap,
            self.MitigationAuditOptionsMap
        )
    }
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {

    pub static mut LdrSystemDllInitBlock: PS_SYSTEM_DLL_INIT_BLOCK;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrAddLoadAsDataTable(
        Module: *mut std::ffi::c_void,
        FilePath: PWSTR,
        Size: usize,
        Handle: HANDLE,
        ActCtx: *mut ACTIVATION_CONTEXT,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrRemoveLoadAsDataTable(
        InitModule: *mut std::ffi::c_void,
        BaseModule: *mut *mut std::ffi::c_void,
        Size: *mut usize,
        Flags: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrGetFileNameFromLoadAsDataTable(
        Module: *mut std::ffi::c_void,
        pFileNamePrt: *mut *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrDisableThreadCalloutsForDll(DllImageBase: *mut std::ffi::c_void) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrAccessResource(
        DllHandle: *mut std::ffi::c_void,
        ResourceDataEntry: *mut IMAGE_RESOURCE_DATA_ENTRY,
        ResourceBuffer: *mut *mut std::ffi::c_void,
        ResourceLength: *mut u32,
    ) -> NTSTATUS;
}

#[repr(C)]
pub struct LDR_RESOURCE_INFO {
    pub Type: usize,
    pub Name: usize,
    pub Language: usize,
}

impl Default for LDR_RESOURCE_INFO {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for LDR_RESOURCE_INFO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LDR_RESOURCE_INFO {{  }}")
    }
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrFindResource_U(
        DllHandle: *mut std::ffi::c_void,
        ResourceInfo: *mut LDR_RESOURCE_INFO,
        Level: u32,
        ResourceDataEntry: *mut *mut IMAGE_RESOURCE_DATA_ENTRY,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrFindResourceEx_U(
        Flags: u32,
        DllHandle: *mut std::ffi::c_void,
        ResourceInfo: *mut LDR_RESOURCE_INFO,
        Level: u32,
        ResourceDataEntry: *mut *mut IMAGE_RESOURCE_DATA_ENTRY,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrFindResourceDirectory_U(
        DllHandle: *mut std::ffi::c_void,
        ResourceInfo: *mut LDR_RESOURCE_INFO,
        Level: u32,
        ResourceDirectory: *mut *mut IMAGE_RESOURCE_DIRECTORY,
    ) -> NTSTATUS;
}

#[repr(C)]
pub struct LDR_ENUM_RESOURCE_ENTRY {
    pub Path: [LDR_ENUM_RESOURCE_ENTRY_1; 3],
    pub Data: *mut std::ffi::c_void,
    pub Size: u32,
    pub Reserved: u32,
}

#[repr(C)]
pub struct LDR_ENUM_RESOURCE_ENTRY_1 {
    pub NameOrId: UnionField<usize>,
    pub Name: UnionField<*mut IMAGE_RESOURCE_DIRECTORY_STRING>,
    pub Anonymous1: UnionField<LDR_ENUM_RESOURCE_ENTRY_1_1>,
    pub union_field: u64,
}

#[repr(C)]
pub struct LDR_ENUM_RESOURCE_ENTRY_1_1 {
    pub Id: u16,
    pub NameIsPresent: u16,
}

impl Default for LDR_ENUM_RESOURCE_ENTRY_1_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for LDR_ENUM_RESOURCE_ENTRY_1_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LDR_ENUM_RESOURCE_ENTRY_1_1 {{  }}")
    }
}

impl Default for LDR_ENUM_RESOURCE_ENTRY_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for LDR_ENUM_RESOURCE_ENTRY_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LDR_ENUM_RESOURCE_ENTRY_1 {{ union }}")
    }
}

impl Default for LDR_ENUM_RESOURCE_ENTRY {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for LDR_ENUM_RESOURCE_ENTRY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LDR_ENUM_RESOURCE_ENTRY {{ Path: {:?} }}", self.Path)
    }
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrEnumResources(
        DllHandle: *mut std::ffi::c_void,
        ResourceInfo: *mut LDR_RESOURCE_INFO,
        Level: u32,
        ResourceCount: *mut u32,
        Resources: *mut LDR_ENUM_RESOURCE_ENTRY,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrFindEntryForAddress(
        DllHandle: *mut std::ffi::c_void,
        Entry: *mut *mut LDR_DATA_TABLE_ENTRY,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrLoadAlternateResourceModule(
        DllHandle: *mut std::ffi::c_void,
        ResourceDllBase: *mut *mut std::ffi::c_void,
        ResourceOffset: *mut usize,
        Flags: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrLoadAlternateResourceModuleEx(
        DllHandle: *mut std::ffi::c_void,
        LanguageId: u16,
        ResourceDllBase: *mut *mut std::ffi::c_void,
        ResourceOffset: *mut usize,
        Flags: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrUnloadAlternateResourceModule(DllHandle: *mut std::ffi::c_void) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrUnloadAlternateResourceModuleEx(
        DllHandle: *mut std::ffi::c_void,
        Flags: u32,
    ) -> bool;
}

#[repr(C)]
pub struct RTL_PROCESS_MODULE_INFORMATION {
    pub Section: HANDLE,
    pub MappedBase: *mut std::ffi::c_void,
    pub ImageBase: *mut std::ffi::c_void,
    pub ImageSize: u32,
    pub Flags: u32,
    pub LoadOrderIndex: u16,
    pub InitOrderIndex: u16,
    pub LoadCount: u16,
    pub OffsetToFileName: u16,
    pub FullPathName: [u8; 256],
}

impl Default for RTL_PROCESS_MODULE_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_PROCESS_MODULE_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RTL_PROCESS_MODULE_INFORMATION {{ FullPathName: {:?} }}",
            self.FullPathName
        )
    }
}

#[repr(C)]
pub struct RTL_PROCESS_MODULES {
    pub NumberOfModules: u32,
    pub Modules: [RTL_PROCESS_MODULE_INFORMATION; 1],
}

impl Default for RTL_PROCESS_MODULES {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_PROCESS_MODULES {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RTL_PROCESS_MODULES {{ Modules: {:?} }}", self.Modules)
    }
}

#[repr(C)]
pub struct RTL_PROCESS_MODULE_INFORMATION_EX {
    pub NextOffset: u16,
    pub BaseInfo: RTL_PROCESS_MODULE_INFORMATION,
    pub ImageChecksum: u32,
    pub TimeDateStamp: u32,
    pub DefaultBase: *mut std::ffi::c_void,
}

impl Default for RTL_PROCESS_MODULE_INFORMATION_EX {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RTL_PROCESS_MODULE_INFORMATION_EX {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RTL_PROCESS_MODULE_INFORMATION_EX {{ BaseInfo: {:?} }}",
            self.BaseInfo
        )
    }
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrQueryProcessModuleInformation(
        ModuleInformation: *mut RTL_PROCESS_MODULES,
        Size: u32,
        ReturnedSize: *mut u32,
    ) -> NTSTATUS;
}

pub type PLDR_ENUM_CALLBACK = std::option::Option<
    unsafe extern "system" fn(
        ModuleInformation: *mut LDR_DATA_TABLE_ENTRY,
        Parameter: *mut std::ffi::c_void,
        Stop: *mut bool,
    ),
>;

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrEnumerateLoadedModules(
        ReservedFlag: bool,
        EnumProc: PLDR_ENUM_CALLBACK,
        Context: *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrOpenImageFileOptionsKey(
        SubKey: *mut UNICODE_STRING,
        Wow64: bool,
        NewKeyHandle: *mut HANDLE,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrQueryImageFileKeyOption(
        KeyHandle: HANDLE,
        ValueName: PCWSTR,
        Type: u32,
        Buffer: *mut std::ffi::c_void,
        BufferSize: u32,
        ReturnedLength: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrQueryImageFileExecutionOptions(
        SubKey: *mut UNICODE_STRING,
        ValueName: PCWSTR,
        ValueSize: u32,
        Buffer: *mut std::ffi::c_void,
        BufferSize: u32,
        ReturnedLength: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrQueryImageFileExecutionOptionsEx(
        SubKey: *mut UNICODE_STRING,
        ValueName: PCWSTR,
        Type: u32,
        Buffer: *mut std::ffi::c_void,
        BufferSize: u32,
        ReturnedLength: *mut u32,
        Wow64: bool,
    ) -> NTSTATUS;
}

pub type PDELAYLOAD_FAILURE_SYSTEM_ROUTINE = std::option::Option<
    unsafe extern "system" fn(DllName: *mut i8, ProcedureName: *mut i8) -> *mut std::ffi::c_void,
>;

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrQueryOptionalDelayLoadedAPI(
        ParentModuleBase: *mut std::ffi::c_void,
        DllName: *mut i8,
        ProcedureName: *mut i8,
        Flags: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrResolveDelayLoadedAPI(
        ParentModuleBase: *mut std::ffi::c_void,
        DelayloadDescriptor: *const IMAGE_DELAYLOAD_DESCRIPTOR,
        FailureDllHook: PDELAYLOAD_FAILURE_DLL_CALLBACK,
        FailureSystemHook: PDELAYLOAD_FAILURE_SYSTEM_ROUTINE,
        ThunkAddress: *mut IMAGE_THUNK_DATA64,
        Flags: u32,
    ) -> *mut std::ffi::c_void;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrResolveDelayLoadsFromDll(
        ParentModuleBase: *mut std::ffi::c_void,
        TargetDllName: *mut i8,
        Flags: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrSetDefaultDllDirectories(DirectoryFlags: u32) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrAddDllDirectory(
        NewDirectory: *mut UNICODE_STRING,
        Cookie: *mut *mut std::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrRemoveDllDirectory(Cookie: *mut std::ffi::c_void) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrShutdownProcess() -> !;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrShutdownThread() -> !;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrSetImplicitPathOptions(ImplicitPathOptions: u32) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrControlFlowGuardEnforced() -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrIsModuleSxsRedirected(DllHandle: *mut std::ffi::c_void) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LdrUpdatePackageSearchPath(SearchPathA: PWSTR) -> NTSTATUS;
}
