use windows::{
    core::PWSTR,
    Win32::{
        Foundation::{FILETIME, HANDLE, HWND, UNICODE_STRING},
        Security::PSID,
    },
};

use crate::{
    bitfield::{BitfieldUnit, UnionField},
    ntrtl::RTL_TIME_ZONE_INFORMATION,
};

pub const WINSTATION_QUERY: u32 = 1;
pub const WINSTATION_SET: u32 = 2;
pub const WINSTATION_RESET: u32 = 4;
pub const WINSTATION_VIRTUAL: u32 = 8;
pub const WINSTATION_SHADOW: u32 = 16;
pub const WINSTATION_LOGON: u32 = 32;
pub const WINSTATION_LOGOFF: u32 = 64;
pub const WINSTATION_MSG: u32 = 128;
pub const WINSTATION_CONNECT: u32 = 256;
pub const WINSTATION_DISCONNECT: u32 = 512;
pub const WINSTATION_GUEST_ACCESS: u32 = 32;
pub const WINSTATION_CURRENT_GUEST_ACCESS: u32 = 72;
pub const WINSTATION_USER_ACCESS: u32 = 289;
pub const WINSTATION_CURRENT_USER_ACCESS: u32 = 590;
pub const WINSTATION_ALL_ACCESS: u32 = 983999;
pub const WDPREFIX_LENGTH: u32 = 12;
pub const CALLBACK_LENGTH: u32 = 50;
pub const DLLNAME_LENGTH: u32 = 32;
pub const CDNAME_LENGTH: u32 = 32;
pub const WDNAME_LENGTH: u32 = 32;
pub const PDNAME_LENGTH: u32 = 32;
pub const DEVICENAME_LENGTH: u32 = 128;
pub const MODEMNAME_LENGTH: u32 = 128;
pub const STACK_ADDRESS_LENGTH: u32 = 128;
pub const MAX_BR_NAME: u32 = 65;
pub const DIRECTORY_LENGTH: u32 = 256;
pub const INITIALPROGRAM_LENGTH: u32 = 256;
pub const PASSWORD_LENGTH: u32 = 14;
pub const NASISPECIFICNAME_LENGTH: u32 = 14;
pub const NASIUSERNAME_LENGTH: u32 = 47;
pub const NASIPASSWORD_LENGTH: u32 = 24;
pub const NASISESSIONNAME_LENGTH: u32 = 16;
pub const NASIFILESERVER_LENGTH: u32 = 47;
pub const CLIENTDATANAME_LENGTH: u32 = 7;
pub const IMEFILENAME_LENGTH: u32 = 32;
pub const CLIENTLICENSE_LENGTH: u32 = 32;
pub const CLIENTMODEM_LENGTH: u32 = 40;
pub const CLIENT_PRODUCT_ID_LENGTH: u32 = 32;
pub const MAX_COUNTER_EXTENSIONS: u32 = 2;
pub const TERMSRV_TOTAL_SESSIONS: u32 = 1;
pub const TERMSRV_DISC_SESSIONS: u32 = 2;
pub const TERMSRV_RECON_SESSIONS: u32 = 3;
pub const TERMSRV_CURRENT_ACTIVE_SESSIONS: u32 = 4;
pub const TERMSRV_CURRENT_DISC_SESSIONS: u32 = 5;
pub const TERMSRV_PENDING_SESSIONS: u32 = 6;
pub const TERMSRV_SUCC_TOTAL_LOGONS: u32 = 7;
pub const TERMSRV_SUCC_LOCAL_LOGONS: u32 = 8;
pub const TERMSRV_SUCC_REMOTE_LOGONS: u32 = 9;
pub const TERMSRV_SUCC_SESSION0_LOGONS: u32 = 10;
pub const TERMSRV_CURRENT_TERMINATING_SESSIONS: u32 = 11;
pub const TERMSRV_CURRENT_LOGGEDON_SESSIONS: u32 = 12;
pub const MAX_THINWIRECACHE: u32 = 4;
pub const PROTOCOL_CONSOLE: u32 = 0;
pub const PROTOCOL_OTHERS: u32 = 1;
pub const PROTOCOL_RDP: u32 = 2;
pub const TS_PROCESS_INFO_MAGIC_NT4: u32 = 592008274;
pub const SIZEOF_TS4_SYSTEM_THREAD_INFORMATION: u32 = 64;
pub const SIZEOF_TS4_SYSTEM_PROCESS_INFORMATION: u32 = 136;
pub const WSD_LOGOFF: u32 = 1;
pub const WSD_SHUTDOWN: u32 = 2;
pub const WSD_REBOOT: u32 = 4;
pub const WSD_POWEROFF: u32 = 8;
pub const WEVENT_NONE: u32 = 0;
pub const WEVENT_CREATE: u32 = 1;
pub const WEVENT_DELETE: u32 = 2;
pub const WEVENT_RENAME: u32 = 4;
pub const WEVENT_CONNECT: u32 = 8;
pub const WEVENT_DISCONNECT: u32 = 16;
pub const WEVENT_LOGON: u32 = 32;
pub const WEVENT_LOGOFF: u32 = 64;
pub const WEVENT_STATECHANGE: u32 = 128;
pub const WEVENT_LICENSE: u32 = 256;
pub const WEVENT_ALL: u32 = 2147483647;
pub const WEVENT_FLUSH: u32 = 2147483648;
pub const WNOTIFY_ALL_SESSIONS: u32 = 1;
pub const LOGONID_CURRENT: i32 = -1;
pub const WINSTATION_CURRENT_SERVER: HANDLE = HANDLE(0isize as *mut std::ffi::c_void);
pub const WINSTATION_CURRENT_SERVER_HANDLE: HANDLE = HANDLE(0isize as *mut std::ffi::c_void);
pub const WINSTATION_CURRENT_SERVER_NAME: *mut std::ffi::c_void = std::ptr::null_mut();
pub const WINSTATION_CURRENT_SESSION: u32 = 4294967295;
pub const WINSTATION_ANY_SESSION: u32 = 4294967294;
pub const SERVERNAME_CURRENT: PWSTR = PWSTR(std::ptr::null_mut());

#[repr(C)]
pub struct VARDATA_WIRE {
    pub Size: u16,
    pub Offset: u16,
}

impl Default for VARDATA_WIRE {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for VARDATA_WIRE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VARDATA_WIRE {{  }}")
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum WINSTATIONSTATECLASS {
    State_Active = 0,
    State_Connected = 1,
    State_ConnectQuery = 2,
    State_Shadow = 3,
    State_Disconnected = 4,
    State_Idle = 5,
    State_Listen = 6,
    State_Reset = 7,
    State_Down = 8,
    State_Init = 9,
}

#[repr(C)]
pub struct SESSIONIDW {
    pub Anonymous1: SESSIONIDW_1,
    pub WinStationName: [u16; 33],
    pub State: WINSTATIONSTATECLASS,
}

#[repr(C)]
pub struct SESSIONIDW_1 {
    pub SessionId: UnionField<u32>,
    pub LogonId: UnionField<u32>,
    pub union_field: u32,
}

impl Default for SESSIONIDW_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for SESSIONIDW_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SESSIONIDW_1 {{ union }}")
    }
}

impl Default for SESSIONIDW {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for SESSIONIDW {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SESSIONIDW {{ Anonymous1: {:?}, WinStationName: {:?}, State: {:?} }}",
            self.Anonymous1, self.WinStationName, self.State
        )
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum WINSTATIONINFOCLASS {
    WinStationCreateData = 0,
    WinStationConfiguration = 1,
    WinStationPdParams = 2,
    WinStationWd = 3,
    WinStationPd = 4,
    WinStationPrinter = 5,
    WinStationClient = 6,
    WinStationModules = 7,
    WinStationInformation = 8,
    WinStationTrace = 9,
    WinStationBeep = 10,
    WinStationEncryptionOff = 11,
    WinStationEncryptionPerm = 12,
    WinStationNtSecurity = 13,
    WinStationUserToken = 14,
    WinStationUnused1 = 15,
    WinStationVideoData = 16,
    WinStationInitialProgram = 17,
    WinStationCd = 18,
    WinStationSystemTrace = 19,
    WinStationVirtualData = 20,
    WinStationClientData = 21,
    WinStationSecureDesktopEnter = 22,
    WinStationSecureDesktopExit = 23,
    WinStationLoadBalanceSessionTarget = 24,
    WinStationLoadIndicator = 25,
    WinStationShadowInfo = 26,
    WinStationDigProductId = 27,
    WinStationLockedState = 28,
    WinStationRemoteAddress = 29,
    WinStationIdleTime = 30,
    WinStationLastReconnectType = 31,
    WinStationDisallowAutoReconnect = 32,
    WinStationMprNotifyInfo = 33,
    WinStationExecSrvSystemPipe = 34,
    WinStationSmartCardAutoLogon = 35,
    WinStationIsAdminLoggedOn = 36,
    WinStationReconnectedFromId = 37,
    WinStationEffectsPolicy = 38,
    WinStationType = 39,
    WinStationInformationEx = 40,
    WinStationValidationInfo = 41,
}

#[repr(C)]
pub struct WINSTATIONCREATE {
    _bitfield_align_1: [u8; 0],
    _bitfield_1: BitfieldUnit<[u8; 1]>,
    pub MaxInstanceCount: u32,
}

impl Default for WINSTATIONCREATE {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for WINSTATIONCREATE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "WINSTATIONCREATE {{ fEnableWinStation : {:?} }}",
            self.fEnableWinStation()
        )
    }
}

impl WINSTATIONCREATE {
    #[inline]
    pub fn fEnableWinStation(&self) -> u32 {
        self._bitfield_1.get(0usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fEnableWinStation(&mut self, val: u32) {
        self._bitfield_1.set(0usize, 1u8, val as u64)
    }

    #[inline]
    pub fn new_bitfield_1(fEnableWinStation: u32) -> BitfieldUnit<[u8; 1]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 1]> = Default::default();

        bitfield_unit.set(0usize, 1u8, fEnableWinStation as u64);

        bitfield_unit
    }
}

#[repr(C)]
pub struct WINSTACONFIGWIRE {
    pub Comment: [u16; 61],
    pub OEMId: [i8; 4],
    pub UserConfig: VARDATA_WIRE,
    pub NewFields: VARDATA_WIRE,
}

impl Default for WINSTACONFIGWIRE {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for WINSTACONFIGWIRE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "WINSTACONFIGWIRE {{ Comment: {:?}, OEMId: {:?}, UserConfig: {:?}, NewFields: {:?} }}",
            self.Comment, self.OEMId, self.UserConfig, self.NewFields
        )
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum CALLBACKCLASS {
    Callback_Disable = 0,
    Callback_Roving = 1,
    Callback_Fixed = 2,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SHADOWCLASS {
    Shadow_Disable = 0,
    Shadow_EnableInputNotify = 1,
    Shadow_EnableInputNoNotify = 2,
    Shadow_EnableNoInputNotify = 3,
    Shadow_EnableNoInputNoNotify = 4,
}

#[repr(C)]
pub struct USERCONFIG {
    _bitfield_align_1: [u8; 0],
    _bitfield_1: BitfieldUnit<[u8; 6]>,
    pub padding_0: u16,
    pub UserName: [u16; 21],
    pub Domain: [u16; 18],
    pub Password: [u16; 15],
    pub WorkDirectory: [u16; 257],
    pub InitialProgram: [u16; 257],
    pub CallbackNumber: [u16; 51],
    pub Callback: CALLBACKCLASS,
    pub Shadow: SHADOWCLASS,
    pub MaxConnectionTime: u32,
    pub MaxDisconnectionTime: u32,
    pub MaxIdleTime: u32,
    pub KeyboardLayout: u32,
    pub MinEncryptionLevel: u8,
    pub NWLogonServer: [u16; 48],
    pub PublishedName: [u16; 65],
    pub WFProfilePath: [u16; 257],
    pub WFHomeDir: [u16; 257],
    pub WFHomeDirDrive: [u16; 4],
}

impl Default for USERCONFIG {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for USERCONFIG {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "USERCONFIG {{ fInheritAutoLogon : {:?}, fInheritResetBroken : {:?}, fInheritReconnectSame : {:?}, fInheritInitialProgram : {:?}, fInheritCallback : {:?}, fInheritCallbackNumber : {:?}, fInheritShadow : {:?}, fInheritMaxSessionTime : {:?}, fInheritMaxDisconnectionTime : {:?}, fInheritMaxIdleTime : {:?}, fInheritAutoClient : {:?}, fInheritSecurity : {:?}, fPromptForPassword : {:?}, fResetBroken : {:?}, fReconnectSame : {:?}, fLogonDisabled : {:?}, fWallPaperDisabled : {:?}, fAutoClientDrives : {:?}, fAutoClientLpts : {:?}, fForceClientLptDef : {:?}, fRequireEncryption : {:?}, fDisableEncryption : {:?}, fUnused1 : {:?}, fHomeDirectoryMapRoot : {:?}, fUseDefaultGina : {:?}, fCursorBlinkDisabled : {:?}, fPublishedApp : {:?}, fHideTitleBar : {:?}, fMaximize : {:?}, fDisableCpm : {:?}, fDisableCdm : {:?}, fDisableCcm : {:?}, fDisableLPT : {:?}, fDisableClip : {:?}, fDisableExe : {:?}, fDisableCam : {:?}, fDisableAutoReconnect : {:?}, ColorDepth : {:?}, fInheritColorDepth : {:?}, fErrorInvalidProfile : {:?}, fPasswordIsScPin : {:?}, fDisablePNPRedir : {:?}, UserName: {:?}, Domain: {:?}, Password: {:?}, WorkDirectory: {:?}, InitialProgram: {:?}, CallbackNumber: {:?}, Callback: {:?}, Shadow: {:?}, NWLogonServer: {:?}, PublishedName: {:?}, WFProfilePath: {:?}, WFHomeDir: {:?}, WFHomeDirDrive: {:?} }}",
            self.fInheritAutoLogon(),
            self.fInheritResetBroken(),
            self.fInheritReconnectSame(),
            self.fInheritInitialProgram(),
            self.fInheritCallback(),
            self.fInheritCallbackNumber(),
            self.fInheritShadow(),
            self.fInheritMaxSessionTime(),
            self.fInheritMaxDisconnectionTime(),
            self.fInheritMaxIdleTime(),
            self.fInheritAutoClient(),
            self.fInheritSecurity(),
            self.fPromptForPassword(),
            self.fResetBroken(),
            self.fReconnectSame(),
            self.fLogonDisabled(),
            self.fWallPaperDisabled(),
            self.fAutoClientDrives(),
            self.fAutoClientLpts(),
            self.fForceClientLptDef(),
            self.fRequireEncryption(),
            self.fDisableEncryption(),
            self.fUnused1(),
            self.fHomeDirectoryMapRoot(),
            self.fUseDefaultGina(),
            self.fCursorBlinkDisabled(),
            self.fPublishedApp(),
            self.fHideTitleBar(),
            self.fMaximize(),
            self.fDisableCpm(),
            self.fDisableCdm(),
            self.fDisableCcm(),
            self.fDisableLPT(),
            self.fDisableClip(),
            self.fDisableExe(),
            self.fDisableCam(),
            self.fDisableAutoReconnect(),
            self.ColorDepth(),
            self.fInheritColorDepth(),
            self.fErrorInvalidProfile(),
            self.fPasswordIsScPin(),
            self.fDisablePNPRedir(),
            self.UserName,
            self.Domain,
            self.Password,
            self.WorkDirectory,
            self.InitialProgram,
            self.CallbackNumber,
            self.Callback,
            self.Shadow,
            self.NWLogonServer,
            self.PublishedName,
            self.WFProfilePath,
            self.WFHomeDir,
            self.WFHomeDirDrive
        )
    }
}

impl USERCONFIG {
    #[inline]
    pub fn fInheritAutoLogon(&self) -> u32 {
        self._bitfield_1.get(0usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fInheritAutoLogon(&mut self, val: u32) {
        self._bitfield_1.set(0usize, 1u8, val as u64)
    }

    #[inline]
    pub fn fInheritResetBroken(&self) -> u32 {
        self._bitfield_1.get(1usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fInheritResetBroken(&mut self, val: u32) {
        self._bitfield_1.set(1usize, 1u8, val as u64)
    }

    #[inline]
    pub fn fInheritReconnectSame(&self) -> u32 {
        self._bitfield_1.get(2usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fInheritReconnectSame(&mut self, val: u32) {
        self._bitfield_1.set(2usize, 1u8, val as u64)
    }

    #[inline]
    pub fn fInheritInitialProgram(&self) -> u32 {
        self._bitfield_1.get(3usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fInheritInitialProgram(&mut self, val: u32) {
        self._bitfield_1.set(3usize, 1u8, val as u64)
    }

    #[inline]
    pub fn fInheritCallback(&self) -> u32 {
        self._bitfield_1.get(4usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fInheritCallback(&mut self, val: u32) {
        self._bitfield_1.set(4usize, 1u8, val as u64)
    }

    #[inline]
    pub fn fInheritCallbackNumber(&self) -> u32 {
        self._bitfield_1.get(5usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fInheritCallbackNumber(&mut self, val: u32) {
        self._bitfield_1.set(5usize, 1u8, val as u64)
    }

    #[inline]
    pub fn fInheritShadow(&self) -> u32 {
        self._bitfield_1.get(6usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fInheritShadow(&mut self, val: u32) {
        self._bitfield_1.set(6usize, 1u8, val as u64)
    }

    #[inline]
    pub fn fInheritMaxSessionTime(&self) -> u32 {
        self._bitfield_1.get(7usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fInheritMaxSessionTime(&mut self, val: u32) {
        self._bitfield_1.set(7usize, 1u8, val as u64)
    }

    #[inline]
    pub fn fInheritMaxDisconnectionTime(&self) -> u32 {
        self._bitfield_1.get(8usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fInheritMaxDisconnectionTime(&mut self, val: u32) {
        self._bitfield_1.set(8usize, 1u8, val as u64)
    }

    #[inline]
    pub fn fInheritMaxIdleTime(&self) -> u32 {
        self._bitfield_1.get(9usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fInheritMaxIdleTime(&mut self, val: u32) {
        self._bitfield_1.set(9usize, 1u8, val as u64)
    }

    #[inline]
    pub fn fInheritAutoClient(&self) -> u32 {
        self._bitfield_1.get(10usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fInheritAutoClient(&mut self, val: u32) {
        self._bitfield_1.set(10usize, 1u8, val as u64)
    }

    #[inline]
    pub fn fInheritSecurity(&self) -> u32 {
        self._bitfield_1.get(11usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fInheritSecurity(&mut self, val: u32) {
        self._bitfield_1.set(11usize, 1u8, val as u64)
    }

    #[inline]
    pub fn fPromptForPassword(&self) -> u32 {
        self._bitfield_1.get(12usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fPromptForPassword(&mut self, val: u32) {
        self._bitfield_1.set(12usize, 1u8, val as u64)
    }

    #[inline]
    pub fn fResetBroken(&self) -> u32 {
        self._bitfield_1.get(13usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fResetBroken(&mut self, val: u32) {
        self._bitfield_1.set(13usize, 1u8, val as u64)
    }

    #[inline]
    pub fn fReconnectSame(&self) -> u32 {
        self._bitfield_1.get(14usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fReconnectSame(&mut self, val: u32) {
        self._bitfield_1.set(14usize, 1u8, val as u64)
    }

    #[inline]
    pub fn fLogonDisabled(&self) -> u32 {
        self._bitfield_1.get(15usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fLogonDisabled(&mut self, val: u32) {
        self._bitfield_1.set(15usize, 1u8, val as u64)
    }

    #[inline]
    pub fn fWallPaperDisabled(&self) -> u32 {
        self._bitfield_1.get(16usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fWallPaperDisabled(&mut self, val: u32) {
        self._bitfield_1.set(16usize, 1u8, val as u64)
    }

    #[inline]
    pub fn fAutoClientDrives(&self) -> u32 {
        self._bitfield_1.get(17usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fAutoClientDrives(&mut self, val: u32) {
        self._bitfield_1.set(17usize, 1u8, val as u64)
    }

    #[inline]
    pub fn fAutoClientLpts(&self) -> u32 {
        self._bitfield_1.get(18usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fAutoClientLpts(&mut self, val: u32) {
        self._bitfield_1.set(18usize, 1u8, val as u64)
    }

    #[inline]
    pub fn fForceClientLptDef(&self) -> u32 {
        self._bitfield_1.get(19usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fForceClientLptDef(&mut self, val: u32) {
        self._bitfield_1.set(19usize, 1u8, val as u64)
    }

    #[inline]
    pub fn fRequireEncryption(&self) -> u32 {
        self._bitfield_1.get(20usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fRequireEncryption(&mut self, val: u32) {
        self._bitfield_1.set(20usize, 1u8, val as u64)
    }

    #[inline]
    pub fn fDisableEncryption(&self) -> u32 {
        self._bitfield_1.get(21usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fDisableEncryption(&mut self, val: u32) {
        self._bitfield_1.set(21usize, 1u8, val as u64)
    }

    #[inline]
    pub fn fUnused1(&self) -> u32 {
        self._bitfield_1.get(22usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fUnused1(&mut self, val: u32) {
        self._bitfield_1.set(22usize, 1u8, val as u64)
    }

    #[inline]
    pub fn fHomeDirectoryMapRoot(&self) -> u32 {
        self._bitfield_1.get(23usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fHomeDirectoryMapRoot(&mut self, val: u32) {
        self._bitfield_1.set(23usize, 1u8, val as u64)
    }

    #[inline]
    pub fn fUseDefaultGina(&self) -> u32 {
        self._bitfield_1.get(24usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fUseDefaultGina(&mut self, val: u32) {
        self._bitfield_1.set(24usize, 1u8, val as u64)
    }

    #[inline]
    pub fn fCursorBlinkDisabled(&self) -> u32 {
        self._bitfield_1.get(25usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fCursorBlinkDisabled(&mut self, val: u32) {
        self._bitfield_1.set(25usize, 1u8, val as u64)
    }

    #[inline]
    pub fn fPublishedApp(&self) -> u32 {
        self._bitfield_1.get(26usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fPublishedApp(&mut self, val: u32) {
        self._bitfield_1.set(26usize, 1u8, val as u64)
    }

    #[inline]
    pub fn fHideTitleBar(&self) -> u32 {
        self._bitfield_1.get(27usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fHideTitleBar(&mut self, val: u32) {
        self._bitfield_1.set(27usize, 1u8, val as u64)
    }

    #[inline]
    pub fn fMaximize(&self) -> u32 {
        self._bitfield_1.get(28usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fMaximize(&mut self, val: u32) {
        self._bitfield_1.set(28usize, 1u8, val as u64)
    }

    #[inline]
    pub fn fDisableCpm(&self) -> u32 {
        self._bitfield_1.get(29usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fDisableCpm(&mut self, val: u32) {
        self._bitfield_1.set(29usize, 1u8, val as u64)
    }

    #[inline]
    pub fn fDisableCdm(&self) -> u32 {
        self._bitfield_1.get(30usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fDisableCdm(&mut self, val: u32) {
        self._bitfield_1.set(30usize, 1u8, val as u64)
    }

    #[inline]
    pub fn fDisableCcm(&self) -> u32 {
        self._bitfield_1.get(31usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fDisableCcm(&mut self, val: u32) {
        self._bitfield_1.set(31usize, 1u8, val as u64)
    }

    #[inline]
    pub fn fDisableLPT(&self) -> u32 {
        self._bitfield_1.get(32usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fDisableLPT(&mut self, val: u32) {
        self._bitfield_1.set(32usize, 1u8, val as u64)
    }

    #[inline]
    pub fn fDisableClip(&self) -> u32 {
        self._bitfield_1.get(33usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fDisableClip(&mut self, val: u32) {
        self._bitfield_1.set(33usize, 1u8, val as u64)
    }

    #[inline]
    pub fn fDisableExe(&self) -> u32 {
        self._bitfield_1.get(34usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fDisableExe(&mut self, val: u32) {
        self._bitfield_1.set(34usize, 1u8, val as u64)
    }

    #[inline]
    pub fn fDisableCam(&self) -> u32 {
        self._bitfield_1.get(35usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fDisableCam(&mut self, val: u32) {
        self._bitfield_1.set(35usize, 1u8, val as u64)
    }

    #[inline]
    pub fn fDisableAutoReconnect(&self) -> u32 {
        self._bitfield_1.get(36usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fDisableAutoReconnect(&mut self, val: u32) {
        self._bitfield_1.set(36usize, 1u8, val as u64)
    }

    #[inline]
    pub fn ColorDepth(&self) -> u32 {
        self._bitfield_1.get(37usize, 3u8) as u32
    }

    #[inline]
    pub fn set_ColorDepth(&mut self, val: u32) {
        self._bitfield_1.set(37usize, 3u8, val as u64)
    }

    #[inline]
    pub fn fInheritColorDepth(&self) -> u32 {
        self._bitfield_1.get(40usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fInheritColorDepth(&mut self, val: u32) {
        self._bitfield_1.set(40usize, 1u8, val as u64)
    }

    #[inline]
    pub fn fErrorInvalidProfile(&self) -> u32 {
        self._bitfield_1.get(41usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fErrorInvalidProfile(&mut self, val: u32) {
        self._bitfield_1.set(41usize, 1u8, val as u64)
    }

    #[inline]
    pub fn fPasswordIsScPin(&self) -> u32 {
        self._bitfield_1.get(42usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fPasswordIsScPin(&mut self, val: u32) {
        self._bitfield_1.set(42usize, 1u8, val as u64)
    }

    #[inline]
    pub fn fDisablePNPRedir(&self) -> u32 {
        self._bitfield_1.get(43usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fDisablePNPRedir(&mut self, val: u32) {
        self._bitfield_1.set(43usize, 1u8, val as u64)
    }

    #[inline]
    pub fn new_bitfield_1(
        fInheritAutoLogon: u32,
        fInheritResetBroken: u32,
        fInheritReconnectSame: u32,
        fInheritInitialProgram: u32,
        fInheritCallback: u32,
        fInheritCallbackNumber: u32,
        fInheritShadow: u32,
        fInheritMaxSessionTime: u32,
        fInheritMaxDisconnectionTime: u32,
        fInheritMaxIdleTime: u32,
        fInheritAutoClient: u32,
        fInheritSecurity: u32,
        fPromptForPassword: u32,
        fResetBroken: u32,
        fReconnectSame: u32,
        fLogonDisabled: u32,
        fWallPaperDisabled: u32,
        fAutoClientDrives: u32,
        fAutoClientLpts: u32,
        fForceClientLptDef: u32,
        fRequireEncryption: u32,
        fDisableEncryption: u32,
        fUnused1: u32,
        fHomeDirectoryMapRoot: u32,
        fUseDefaultGina: u32,
        fCursorBlinkDisabled: u32,
        fPublishedApp: u32,
        fHideTitleBar: u32,
        fMaximize: u32,
        fDisableCpm: u32,
        fDisableCdm: u32,
        fDisableCcm: u32,
        fDisableLPT: u32,
        fDisableClip: u32,
        fDisableExe: u32,
        fDisableCam: u32,
        fDisableAutoReconnect: u32,
        ColorDepth: u32,
        fInheritColorDepth: u32,
        fErrorInvalidProfile: u32,
        fPasswordIsScPin: u32,
        fDisablePNPRedir: u32,
    ) -> BitfieldUnit<[u8; 6]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 6]> = Default::default();

        bitfield_unit.set(0usize, 1u8, fInheritAutoLogon as u64);

        bitfield_unit.set(1usize, 1u8, fInheritResetBroken as u64);

        bitfield_unit.set(2usize, 1u8, fInheritReconnectSame as u64);

        bitfield_unit.set(3usize, 1u8, fInheritInitialProgram as u64);

        bitfield_unit.set(4usize, 1u8, fInheritCallback as u64);

        bitfield_unit.set(5usize, 1u8, fInheritCallbackNumber as u64);

        bitfield_unit.set(6usize, 1u8, fInheritShadow as u64);

        bitfield_unit.set(7usize, 1u8, fInheritMaxSessionTime as u64);

        bitfield_unit.set(8usize, 1u8, fInheritMaxDisconnectionTime as u64);

        bitfield_unit.set(9usize, 1u8, fInheritMaxIdleTime as u64);

        bitfield_unit.set(10usize, 1u8, fInheritAutoClient as u64);

        bitfield_unit.set(11usize, 1u8, fInheritSecurity as u64);

        bitfield_unit.set(12usize, 1u8, fPromptForPassword as u64);

        bitfield_unit.set(13usize, 1u8, fResetBroken as u64);

        bitfield_unit.set(14usize, 1u8, fReconnectSame as u64);

        bitfield_unit.set(15usize, 1u8, fLogonDisabled as u64);

        bitfield_unit.set(16usize, 1u8, fWallPaperDisabled as u64);

        bitfield_unit.set(17usize, 1u8, fAutoClientDrives as u64);

        bitfield_unit.set(18usize, 1u8, fAutoClientLpts as u64);

        bitfield_unit.set(19usize, 1u8, fForceClientLptDef as u64);

        bitfield_unit.set(20usize, 1u8, fRequireEncryption as u64);

        bitfield_unit.set(21usize, 1u8, fDisableEncryption as u64);

        bitfield_unit.set(22usize, 1u8, fUnused1 as u64);

        bitfield_unit.set(23usize, 1u8, fHomeDirectoryMapRoot as u64);

        bitfield_unit.set(24usize, 1u8, fUseDefaultGina as u64);

        bitfield_unit.set(25usize, 1u8, fCursorBlinkDisabled as u64);

        bitfield_unit.set(26usize, 1u8, fPublishedApp as u64);

        bitfield_unit.set(27usize, 1u8, fHideTitleBar as u64);

        bitfield_unit.set(28usize, 1u8, fMaximize as u64);

        bitfield_unit.set(29usize, 1u8, fDisableCpm as u64);

        bitfield_unit.set(30usize, 1u8, fDisableCdm as u64);

        bitfield_unit.set(31usize, 1u8, fDisableCcm as u64);

        bitfield_unit.set(32usize, 1u8, fDisableLPT as u64);

        bitfield_unit.set(33usize, 1u8, fDisableClip as u64);

        bitfield_unit.set(34usize, 1u8, fDisableExe as u64);

        bitfield_unit.set(35usize, 1u8, fDisableCam as u64);

        bitfield_unit.set(36usize, 1u8, fDisableAutoReconnect as u64);

        bitfield_unit.set(37usize, 3u8, ColorDepth as u64);

        bitfield_unit.set(40usize, 1u8, fInheritColorDepth as u64);

        bitfield_unit.set(41usize, 1u8, fErrorInvalidProfile as u64);

        bitfield_unit.set(42usize, 1u8, fPasswordIsScPin as u64);

        bitfield_unit.set(43usize, 1u8, fDisablePNPRedir as u64);

        bitfield_unit
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SDCLASS {
    SdNone = 0,
    SdConsole = 1,
    SdNetwork = 2,
    SdAsync = 3,
    SdOemTransport = 4,
}

pub type DEVICENAME = [u16; 129];

pub type MODEMNAME = [u16; 129];

pub type NASISPECIFICNAME = [u16; 15];

pub type NASIUSERNAME = [u16; 48];

pub type NASIPASSWORD = [u16; 25];

pub type NASISESIONNAME = [u16; 17];

pub type NASIFILESERVER = [u16; 48];

pub type WDNAME = [u16; 33];

pub type WDPREFIX = [u16; 13];

pub type CDNAME = [u16; 33];

pub type DLLNAME = [u16; 33];

pub type PDNAME = [u16; 33];

#[repr(C)]
pub struct NETWORKCONFIG {
    pub LanAdapter: i32,
    pub NetworkName: DEVICENAME,
    pub Flags: u32,
}

impl Default for NETWORKCONFIG {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for NETWORKCONFIG {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NETWORKCONFIG {{ NetworkName: {:?} }}", self.NetworkName)
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FLOWCONTROLCLASS {
    FlowControl_None = 0,
    FlowControl_Hardware = 1,
    FlowControl_Software = 2,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum RECEIVEFLOWCONTROLCLASS {
    ReceiveFlowControl_None = 0,
    ReceiveFlowControl_RTS = 1,
    ReceiveFlowControl_DTR = 2,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum TRANSMITFLOWCONTROLCLASS {
    TransmitFlowControl_None = 0,
    TransmitFlowControl_CTS = 1,
    TransmitFlowControl_DSR = 2,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ASYNCCONNECTCLASS {
    Connect_CTS = 0,
    Connect_DSR = 1,
    Connect_RI = 2,
    Connect_DCD = 3,
    Connect_FirstChar = 4,
    Connect_Perm = 5,
}

#[repr(C)]
pub struct FLOWCONTROLCONFIG {
    _bitfield_align_1: [u8; 0],
    _bitfield_1: BitfieldUnit<[u8; 1]>,
    pub padding_0: [u8; 3],
    pub XonChar: i8,
    pub XoffChar: i8,
    pub Type: FLOWCONTROLCLASS,
    pub HardwareReceive: RECEIVEFLOWCONTROLCLASS,
    pub HardwareTransmit: TRANSMITFLOWCONTROLCLASS,
}

impl Default for FLOWCONTROLCONFIG {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for FLOWCONTROLCONFIG {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FLOWCONTROLCONFIG {{ fEnableSoftwareTx : {:?}, fEnableSoftwareRx : {:?}, fEnableDTR : {:?}, fEnableRTS : {:?}, Type: {:?}, HardwareReceive: {:?}, HardwareTransmit: {:?} }}",
            self.fEnableSoftwareTx(),
            self.fEnableSoftwareRx(),
            self.fEnableDTR(),
            self.fEnableRTS(),
            self.Type,
            self.HardwareReceive,
            self.HardwareTransmit
        )
    }
}

impl FLOWCONTROLCONFIG {
    #[inline]
    pub fn fEnableSoftwareTx(&self) -> u32 {
        self._bitfield_1.get(0usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fEnableSoftwareTx(&mut self, val: u32) {
        self._bitfield_1.set(0usize, 1u8, val as u64)
    }

    #[inline]
    pub fn fEnableSoftwareRx(&self) -> u32 {
        self._bitfield_1.get(1usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fEnableSoftwareRx(&mut self, val: u32) {
        self._bitfield_1.set(1usize, 1u8, val as u64)
    }

    #[inline]
    pub fn fEnableDTR(&self) -> u32 {
        self._bitfield_1.get(2usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fEnableDTR(&mut self, val: u32) {
        self._bitfield_1.set(2usize, 1u8, val as u64)
    }

    #[inline]
    pub fn fEnableRTS(&self) -> u32 {
        self._bitfield_1.get(3usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fEnableRTS(&mut self, val: u32) {
        self._bitfield_1.set(3usize, 1u8, val as u64)
    }

    #[inline]
    pub fn new_bitfield_1(
        fEnableSoftwareTx: u32,
        fEnableSoftwareRx: u32,
        fEnableDTR: u32,
        fEnableRTS: u32,
    ) -> BitfieldUnit<[u8; 1]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 1]> = Default::default();

        bitfield_unit.set(0usize, 1u8, fEnableSoftwareTx as u64);

        bitfield_unit.set(1usize, 1u8, fEnableSoftwareRx as u64);

        bitfield_unit.set(2usize, 1u8, fEnableDTR as u64);

        bitfield_unit.set(3usize, 1u8, fEnableRTS as u64);

        bitfield_unit
    }
}

#[repr(C)]
pub struct CONNECTCONFIG {
    pub Type: ASYNCCONNECTCLASS,
    _bitfield_align_1: [u8; 0],
    _bitfield_1: BitfieldUnit<[u8; 1]>,
    pub padding_0: [u8; 3],
}

impl Default for CONNECTCONFIG {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for CONNECTCONFIG {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CONNECTCONFIG {{ Type: {:?}, fEnableBreakDisconnect : {:?} }}",
            self.Type,
            self.fEnableBreakDisconnect()
        )
    }
}

impl CONNECTCONFIG {
    #[inline]
    pub fn fEnableBreakDisconnect(&self) -> u32 {
        self._bitfield_1.get(0usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fEnableBreakDisconnect(&mut self, val: u32) {
        self._bitfield_1.set(0usize, 1u8, val as u64)
    }

    #[inline]
    pub fn new_bitfield_1(fEnableBreakDisconnect: u32) -> BitfieldUnit<[u8; 1]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 1]> = Default::default();

        bitfield_unit.set(0usize, 1u8, fEnableBreakDisconnect as u64);

        bitfield_unit
    }
}

#[repr(C)]
pub struct ASYNCCONFIG {
    pub DeviceName: DEVICENAME,
    pub ModemName: MODEMNAME,
    pub BaudRate: u32,
    pub Parity: u32,
    pub StopBits: u32,
    pub ByteSize: u32,
    _bitfield_align_1: [u8; 0],
    _bitfield_1: BitfieldUnit<[u8; 1]>,
    pub FlowControl: FLOWCONTROLCONFIG,
    pub Connect: CONNECTCONFIG,
}

impl Default for ASYNCCONFIG {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for ASYNCCONFIG {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ASYNCCONFIG {{ DeviceName: {:?}, ModemName: {:?}, fEnableDsrSensitivity : {:?}, fConnectionDriver : {:?}, FlowControl: {:?}, Connect: {:?} }}",
            self.DeviceName,
            self.ModemName,
            self.fEnableDsrSensitivity(),
            self.fConnectionDriver(),
            self.FlowControl,
            self.Connect
        )
    }
}

impl ASYNCCONFIG {
    #[inline]
    pub fn fEnableDsrSensitivity(&self) -> u32 {
        self._bitfield_1.get(0usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fEnableDsrSensitivity(&mut self, val: u32) {
        self._bitfield_1.set(0usize, 1u8, val as u64)
    }

    #[inline]
    pub fn fConnectionDriver(&self) -> u32 {
        self._bitfield_1.get(1usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fConnectionDriver(&mut self, val: u32) {
        self._bitfield_1.set(1usize, 1u8, val as u64)
    }

    #[inline]
    pub fn new_bitfield_1(
        fEnableDsrSensitivity: u32,
        fConnectionDriver: u32,
    ) -> BitfieldUnit<[u8; 1]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 1]> = Default::default();

        bitfield_unit.set(0usize, 1u8, fEnableDsrSensitivity as u64);

        bitfield_unit.set(1usize, 1u8, fConnectionDriver as u64);

        bitfield_unit
    }
}

#[repr(C)]
pub struct NASICONFIG {
    pub SpecificName: NASISPECIFICNAME,
    pub UserName: NASIUSERNAME,
    pub PassWord: NASIPASSWORD,
    pub SessionName: NASISESIONNAME,
    pub FileServer: NASIFILESERVER,
    pub GlobalSession: bool,
}

impl Default for NASICONFIG {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for NASICONFIG {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "NASICONFIG {{ SpecificName: {:?}, UserName: {:?}, PassWord: {:?}, SessionName: {:?}, FileServer: {:?} }}",
            self.SpecificName, self.UserName, self.PassWord, self.SessionName, self.FileServer
        )
    }
}

#[repr(C)]
pub struct OEMTDCONFIG {
    pub Adapter: i32,
    pub DeviceName: DEVICENAME,
    pub Flags: u32,
}

impl Default for OEMTDCONFIG {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for OEMTDCONFIG {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OEMTDCONFIG {{ DeviceName: {:?} }}", self.DeviceName)
    }
}

#[repr(C)]
pub struct PDPARAMS {
    pub SdClass: SDCLASS,
    pub Anonymous1: PDPARAMS_1,
}

#[repr(C)]
pub struct PDPARAMS_1 {
    pub Network: UnionField<NETWORKCONFIG>,
    pub Async: UnionField<ASYNCCONFIG>,
    pub Nasi: UnionField<NASICONFIG>,
    pub OemTd: UnionField<OEMTDCONFIG>,
    pub union_field: [u32; 141],
}

impl Default for PDPARAMS_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PDPARAMS_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PDPARAMS_1 {{ union }}")
    }
}

impl Default for PDPARAMS {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PDPARAMS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PDPARAMS {{ SdClass: {:?}, Anonymous1: {:?} }}",
            self.SdClass, self.Anonymous1
        )
    }
}

#[repr(C)]
pub struct WDCONFIG {
    pub WdName: WDNAME,
    pub WdDLL: DLLNAME,
    pub WsxDLL: DLLNAME,
    pub WdFlag: u32,
    pub WdInputBufferLength: u32,
    pub CfgDLL: DLLNAME,
    pub WdPrefix: WDPREFIX,
}

impl Default for WDCONFIG {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for WDCONFIG {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "WDCONFIG {{ WdName: {:?}, WdDLL: {:?}, WsxDLL: {:?}, CfgDLL: {:?}, WdPrefix: {:?} }}",
            self.WdName, self.WdDLL, self.WsxDLL, self.CfgDLL, self.WdPrefix
        )
    }
}

#[repr(C)]
pub struct PDCONFIG2 {
    pub PdName: PDNAME,
    pub SdClass: SDCLASS,
    pub PdDLL: DLLNAME,
    pub PdFlag: u32,
    pub OutBufLength: u32,
    pub OutBufCount: u32,
    pub OutBufDelay: u32,
    pub InteractiveDelay: u32,
    pub PortNumber: u32,
    pub KeepAliveTimeout: u32,
}

impl Default for PDCONFIG2 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PDCONFIG2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PDCONFIG2 {{ PdName: {:?}, SdClass: {:?}, PdDLL: {:?} }}",
            self.PdName, self.SdClass, self.PdDLL
        )
    }
}

#[repr(C)]
pub struct WINSTATIONCLIENT {
    _bitfield_align_1: [u8; 0],
    _bitfield_1: BitfieldUnit<[u8; 2]>,
    pub padding_0: u16,
    pub ClientName: [u16; 21],
    pub Domain: [u16; 18],
    pub UserName: [u16; 21],
    pub Password: [u16; 15],
    pub WorkDirectory: [u16; 257],
    pub InitialProgram: [u16; 257],
    pub SerialNumber: u32,
    pub EncryptionLevel: u8,
    pub ClientAddressFamily: u32,
    pub ClientAddress: [u16; 31],
    pub HRes: u16,
    pub VRes: u16,
    pub ColorDepth: u16,
    pub ProtocolType: u16,
    pub KeyboardLayout: u32,
    pub KeyboardType: u32,
    pub KeyboardSubType: u32,
    pub KeyboardFunctionKey: u32,
    pub ImeFileName: [u16; 33],
    pub ClientDirectory: [u16; 257],
    pub ClientLicense: [u16; 33],
    pub ClientModem: [u16; 41],
    pub ClientBuildNumber: u32,
    pub ClientHardwareId: u32,
    pub ClientProductId: u16,
    pub OutBufCountHost: u16,
    pub OutBufCountClient: u16,
    pub OutBufLength: u16,
    pub AudioDriverName: [u16; 9],
    pub ClientTimeZone: RTL_TIME_ZONE_INFORMATION,
    pub ClientSessionId: u32,
    pub ClientDigProductId: [u16; 32],
    pub PerformanceFlags: u32,
    pub ActiveInputLocale: u32,
}

impl Default for WINSTATIONCLIENT {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for WINSTATIONCLIENT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "WINSTATIONCLIENT {{ fTextOnly : {:?}, fDisableCtrlAltDel : {:?}, fMouse : {:?}, fDoubleClickDetect : {:?}, fINetClient : {:?}, fPromptForPassword : {:?}, fMaximizeShell : {:?}, fEnableWindowsKey : {:?}, fRemoteConsoleAudio : {:?}, fPasswordIsScPin : {:?}, fNoAudioPlayback : {:?}, fUsingSavedCreds : {:?}, ClientName: {:?}, Domain: {:?}, UserName: {:?}, Password: {:?}, WorkDirectory: {:?}, InitialProgram: {:?}, ClientAddress: {:?}, ImeFileName: {:?}, ClientDirectory: {:?}, ClientLicense: {:?}, ClientModem: {:?}, AudioDriverName: {:?}, ClientDigProductId: {:?} }}",
            self.fTextOnly(),
            self.fDisableCtrlAltDel(),
            self.fMouse(),
            self.fDoubleClickDetect(),
            self.fINetClient(),
            self.fPromptForPassword(),
            self.fMaximizeShell(),
            self.fEnableWindowsKey(),
            self.fRemoteConsoleAudio(),
            self.fPasswordIsScPin(),
            self.fNoAudioPlayback(),
            self.fUsingSavedCreds(),
            self.ClientName,
            self.Domain,
            self.UserName,
            self.Password,
            self.WorkDirectory,
            self.InitialProgram,
            self.ClientAddress,
            self.ImeFileName,
            self.ClientDirectory,
            self.ClientLicense,
            self.ClientModem,
            self.AudioDriverName,
            self.ClientDigProductId
        )
    }
}

impl WINSTATIONCLIENT {
    #[inline]
    pub fn fTextOnly(&self) -> u32 {
        self._bitfield_1.get(0usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fTextOnly(&mut self, val: u32) {
        self._bitfield_1.set(0usize, 1u8, val as u64)
    }

    #[inline]
    pub fn fDisableCtrlAltDel(&self) -> u32 {
        self._bitfield_1.get(1usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fDisableCtrlAltDel(&mut self, val: u32) {
        self._bitfield_1.set(1usize, 1u8, val as u64)
    }

    #[inline]
    pub fn fMouse(&self) -> u32 {
        self._bitfield_1.get(2usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fMouse(&mut self, val: u32) {
        self._bitfield_1.set(2usize, 1u8, val as u64)
    }

    #[inline]
    pub fn fDoubleClickDetect(&self) -> u32 {
        self._bitfield_1.get(3usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fDoubleClickDetect(&mut self, val: u32) {
        self._bitfield_1.set(3usize, 1u8, val as u64)
    }

    #[inline]
    pub fn fINetClient(&self) -> u32 {
        self._bitfield_1.get(4usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fINetClient(&mut self, val: u32) {
        self._bitfield_1.set(4usize, 1u8, val as u64)
    }

    #[inline]
    pub fn fPromptForPassword(&self) -> u32 {
        self._bitfield_1.get(5usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fPromptForPassword(&mut self, val: u32) {
        self._bitfield_1.set(5usize, 1u8, val as u64)
    }

    #[inline]
    pub fn fMaximizeShell(&self) -> u32 {
        self._bitfield_1.get(6usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fMaximizeShell(&mut self, val: u32) {
        self._bitfield_1.set(6usize, 1u8, val as u64)
    }

    #[inline]
    pub fn fEnableWindowsKey(&self) -> u32 {
        self._bitfield_1.get(7usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fEnableWindowsKey(&mut self, val: u32) {
        self._bitfield_1.set(7usize, 1u8, val as u64)
    }

    #[inline]
    pub fn fRemoteConsoleAudio(&self) -> u32 {
        self._bitfield_1.get(8usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fRemoteConsoleAudio(&mut self, val: u32) {
        self._bitfield_1.set(8usize, 1u8, val as u64)
    }

    #[inline]
    pub fn fPasswordIsScPin(&self) -> u32 {
        self._bitfield_1.get(9usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fPasswordIsScPin(&mut self, val: u32) {
        self._bitfield_1.set(9usize, 1u8, val as u64)
    }

    #[inline]
    pub fn fNoAudioPlayback(&self) -> u32 {
        self._bitfield_1.get(10usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fNoAudioPlayback(&mut self, val: u32) {
        self._bitfield_1.set(10usize, 1u8, val as u64)
    }

    #[inline]
    pub fn fUsingSavedCreds(&self) -> u32 {
        self._bitfield_1.get(11usize, 1u8) as u32
    }

    #[inline]
    pub fn set_fUsingSavedCreds(&mut self, val: u32) {
        self._bitfield_1.set(11usize, 1u8, val as u64)
    }

    #[inline]
    pub fn new_bitfield_1(
        fTextOnly: u32,
        fDisableCtrlAltDel: u32,
        fMouse: u32,
        fDoubleClickDetect: u32,
        fINetClient: u32,
        fPromptForPassword: u32,
        fMaximizeShell: u32,
        fEnableWindowsKey: u32,
        fRemoteConsoleAudio: u32,
        fPasswordIsScPin: u32,
        fNoAudioPlayback: u32,
        fUsingSavedCreds: u32,
    ) -> BitfieldUnit<[u8; 2]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 2]> = Default::default();

        bitfield_unit.set(0usize, 1u8, fTextOnly as u64);

        bitfield_unit.set(1usize, 1u8, fDisableCtrlAltDel as u64);

        bitfield_unit.set(2usize, 1u8, fMouse as u64);

        bitfield_unit.set(3usize, 1u8, fDoubleClickDetect as u64);

        bitfield_unit.set(4usize, 1u8, fINetClient as u64);

        bitfield_unit.set(5usize, 1u8, fPromptForPassword as u64);

        bitfield_unit.set(6usize, 1u8, fMaximizeShell as u64);

        bitfield_unit.set(7usize, 1u8, fEnableWindowsKey as u64);

        bitfield_unit.set(8usize, 1u8, fRemoteConsoleAudio as u64);

        bitfield_unit.set(9usize, 1u8, fPasswordIsScPin as u64);

        bitfield_unit.set(10usize, 1u8, fNoAudioPlayback as u64);

        bitfield_unit.set(11usize, 1u8, fUsingSavedCreds as u64);

        bitfield_unit
    }
}

#[repr(C)]
pub struct TSHARE_COUNTERS {
    pub Reserved: u32,
}

impl Default for TSHARE_COUNTERS {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for TSHARE_COUNTERS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TSHARE_COUNTERS {{  }}")
    }
}

#[repr(C)]
pub struct PROTOCOLCOUNTERS {
    pub WdBytes: u32,
    pub WdFrames: u32,
    pub WaitForOutBuf: u32,
    pub Frames: u32,
    pub Bytes: u32,
    pub CompressedBytes: u32,
    pub CompressFlushes: u32,
    pub Errors: u32,
    pub Timeouts: u32,
    pub AsyncFramingError: u32,
    pub AsyncOverrunError: u32,
    pub AsyncOverflowError: u32,
    pub AsyncParityError: u32,
    pub TdErrors: u32,
    pub ProtocolType: u16,
    pub Length: u16,
    pub Specific: PROTOCOLCOUNTERS_1,
}

#[repr(C)]
pub struct PROTOCOLCOUNTERS_1 {
    pub TShareCounters: UnionField<TSHARE_COUNTERS>,
    pub Reserved: UnionField<[u32; 100]>,
    pub union_field: [u32; 100],
}

impl Default for PROTOCOLCOUNTERS_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PROTOCOLCOUNTERS_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PROTOCOLCOUNTERS_1 {{ union }}")
    }
}

impl Default for PROTOCOLCOUNTERS {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PROTOCOLCOUNTERS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PROTOCOLCOUNTERS {{ Specific: {:?} }}", self.Specific)
    }
}

#[repr(C)]
pub struct THINWIRECACHE {
    pub CacheReads: u32,
    pub CacheHits: u32,
}

impl Default for THINWIRECACHE {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for THINWIRECACHE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "THINWIRECACHE {{  }}")
    }
}

#[repr(C)]
pub struct RESERVED_CACHE {
    pub ThinWireCache: [THINWIRECACHE; 4],
}

impl Default for RESERVED_CACHE {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for RESERVED_CACHE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RESERVED_CACHE {{ ThinWireCache: {:?} }}",
            self.ThinWireCache
        )
    }
}

#[repr(C)]
pub struct TSHARE_CACHE {
    pub Reserved: u32,
}

impl Default for TSHARE_CACHE {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for TSHARE_CACHE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TSHARE_CACHE {{  }}")
    }
}

#[repr(C)]
pub struct CACHE_STATISTICS {
    pub ProtocolType: u16,
    pub Length: u16,
    pub Specific: CACHE_STATISTICS_1,
}

#[repr(C)]
pub struct CACHE_STATISTICS_1 {
    pub ReservedCacheStats: UnionField<RESERVED_CACHE>,
    pub TShareCacheStats: UnionField<TSHARE_CACHE>,
    pub Reserved: UnionField<[u32; 20]>,
    pub union_field: [u32; 20],
}

impl Default for CACHE_STATISTICS_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for CACHE_STATISTICS_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CACHE_STATISTICS_1 {{ union }}")
    }
}

impl Default for CACHE_STATISTICS {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for CACHE_STATISTICS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CACHE_STATISTICS {{ Specific: {:?} }}", self.Specific)
    }
}

pub type PCACHE_STATISTICS = *mut CACHE_STATISTICS;

#[repr(C)]
pub struct PROTOCOLSTATUS {
    pub Output: PROTOCOLCOUNTERS,
    pub Input: PROTOCOLCOUNTERS,
    pub Cache: CACHE_STATISTICS,
    pub AsyncSignal: u32,
    pub AsyncSignalMask: u32,
}

impl Default for PROTOCOLSTATUS {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for PROTOCOLSTATUS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PROTOCOLSTATUS {{ Output: {:?}, Input: {:?}, Cache: {:?} }}",
            self.Output, self.Input, self.Cache
        )
    }
}

#[repr(C)]
pub struct WINSTATIONINFORMATION {
    pub ConnectState: WINSTATIONSTATECLASS,
    pub WinStationName: [u16; 33],
    pub LogonId: u32,
    pub ConnectTime: i64,
    pub DisconnectTime: i64,
    pub LastInputTime: i64,
    pub LogonTime: i64,
    pub Status: PROTOCOLSTATUS,
    pub Domain: [u16; 18],
    pub UserName: [u16; 21],
    pub CurrentTime: i64,
}

impl Default for WINSTATIONINFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for WINSTATIONINFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "WINSTATIONINFORMATION {{ ConnectState: {:?}, WinStationName: {:?}, Status: {:?}, Domain: {:?}, UserName: {:?} }}",
            self.ConnectState, self.WinStationName, self.Status, self.Domain, self.UserName
        )
    }
}

#[repr(C)]
pub struct WINSTATIONUSERTOKEN {
    pub ProcessId: HANDLE,
    pub ThreadId: HANDLE,
    pub UserToken: HANDLE,
}

impl Default for WINSTATIONUSERTOKEN {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for WINSTATIONUSERTOKEN {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WINSTATIONUSERTOKEN {{  }}")
    }
}

#[repr(C)]
pub struct WINSTATIONVIDEODATA {
    pub HResolution: u16,
    pub VResolution: u16,
    pub fColorDepth: u16,
}

impl Default for WINSTATIONVIDEODATA {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for WINSTATIONVIDEODATA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WINSTATIONVIDEODATA {{  }}")
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum CDCLASS {
    CdNone = 0,
    CdModem = 1,
    CdClass_Maximum = 2,
}

#[repr(C)]
pub struct CDCONFIG {
    pub CdClass: CDCLASS,
    pub CdName: CDNAME,
    pub CdDLL: DLLNAME,
    pub CdFlag: u32,
}

impl Default for CDCONFIG {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for CDCONFIG {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CDCONFIG {{ CdClass: {:?}, CdName: {:?}, CdDLL: {:?} }}",
            self.CdClass, self.CdName, self.CdDLL
        )
    }
}

pub type CLIENTDATANAME = [i8; 8];

pub type PCLIENTDATANAME = *mut i8;

#[repr(C)]
pub struct WINSTATIONCLIENTDATA {
    pub DataName: CLIENTDATANAME,
    pub fUnicodeData: bool,
}

impl Default for WINSTATIONCLIENTDATA {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for WINSTATIONCLIENTDATA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "WINSTATIONCLIENTDATA {{ DataName: {:?} }}",
            self.DataName
        )
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum LOADFACTORTYPE {
    ErrorConstraint = 0,
    PagedPoolConstraint = 1,
    NonPagedPoolConstraint = 2,
    AvailablePagesConstraint = 3,
    SystemPtesConstraint = 4,
    CPUConstraint = 5,
}

#[repr(C)]
pub struct WINSTATIONLOADINDICATORDATA {
    pub RemainingSessionCapacity: u32,
    pub LoadFactor: LOADFACTORTYPE,
    pub TotalSessions: u32,
    pub DisconnectedSessions: u32,
    pub IdleCPU: i64,
    pub TotalCPU: i64,
    pub RawSessionCapacity: u32,
    pub reserved: [u32; 9],
}

impl Default for WINSTATIONLOADINDICATORDATA {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for WINSTATIONLOADINDICATORDATA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "WINSTATIONLOADINDICATORDATA {{ LoadFactor: {:?}, reserved: {:?} }}",
            self.LoadFactor, self.reserved
        )
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SHADOWSTATECLASS {
    State_NoShadow = 0,
    State_Shadowing = 1,
    State_Shadowed = 2,
}

#[repr(C)]
pub struct WINSTATIONSHADOW {
    pub ShadowState: SHADOWSTATECLASS,
    pub ShadowClass: SHADOWCLASS,
    pub SessionId: u32,
    pub ProtocolType: u32,
}

impl Default for WINSTATIONSHADOW {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for WINSTATIONSHADOW {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "WINSTATIONSHADOW {{ ShadowState: {:?}, ShadowClass: {:?} }}",
            self.ShadowState, self.ShadowClass
        )
    }
}

#[repr(C)]
pub struct WINSTATIONPRODID {
    pub DigProductId: [u16; 32],
    pub ClientDigProductId: [u16; 32],
    pub OuterMostDigProductId: [u16; 32],
    pub CurrentSessionId: u32,
    pub ClientSessionId: u32,
    pub OuterMostSessionId: u32,
}

impl Default for WINSTATIONPRODID {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for WINSTATIONPRODID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "WINSTATIONPRODID {{ DigProductId: {:?}, ClientDigProductId: {:?}, OuterMostDigProductId: {:?} }}",
            self.DigProductId, self.ClientDigProductId, self.OuterMostDigProductId
        )
    }
}

#[repr(C)]
pub struct WINSTATIONREMOTEADDRESS {
    pub sin_family: u16,
    pub Anonymous1: WINSTATIONREMOTEADDRESS_1,
}

#[repr(C)]
pub struct WINSTATIONREMOTEADDRESS_1 {
    pub ipv4: UnionField<WINSTATIONREMOTEADDRESS_1_1>,
    pub ipv6: UnionField<WINSTATIONREMOTEADDRESS_1_2>,
    pub union_field: [u32; 7],
}

#[repr(C)]
pub struct WINSTATIONREMOTEADDRESS_1_1 {
    pub sin_port: u16,
    pub sin_addr: u32,
    pub sin_zero: [u8; 8],
}

impl Default for WINSTATIONREMOTEADDRESS_1_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for WINSTATIONREMOTEADDRESS_1_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "WINSTATIONREMOTEADDRESS_1_1 {{ sin_zero: {:?} }}",
            self.sin_zero
        )
    }
}

#[repr(C)]
pub struct WINSTATIONREMOTEADDRESS_1_2 {
    pub sin6_port: u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr: [u16; 8],
    pub sin6_scope_id: u32,
}

impl Default for WINSTATIONREMOTEADDRESS_1_2 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for WINSTATIONREMOTEADDRESS_1_2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "WINSTATIONREMOTEADDRESS_1_2 {{ sin6_addr: {:?} }}",
            self.sin6_addr
        )
    }
}

impl Default for WINSTATIONREMOTEADDRESS_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for WINSTATIONREMOTEADDRESS_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WINSTATIONREMOTEADDRESS_1 {{ union }}")
    }
}

impl Default for WINSTATIONREMOTEADDRESS {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for WINSTATIONREMOTEADDRESS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "WINSTATIONREMOTEADDRESS {{ Anonymous1: {:?} }}",
            self.Anonymous1
        )
    }
}

#[repr(C)]
pub struct WINSTATIONINFORMATIONEX_LEVEL1 {
    pub SessionId: u32,
    pub SessionState: WINSTATIONSTATECLASS,
    pub SessionFlags: i32,
    pub WinStationName: [u16; 33],
    pub UserName: [u16; 21],
    pub DomainName: [u16; 18],
    pub LogonTime: i64,
    pub ConnectTime: i64,
    pub DisconnectTime: i64,
    pub LastInputTime: i64,
    pub CurrentTime: i64,
    pub ProtocolStatus: PROTOCOLSTATUS,
}

impl Default for WINSTATIONINFORMATIONEX_LEVEL1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for WINSTATIONINFORMATIONEX_LEVEL1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "WINSTATIONINFORMATIONEX_LEVEL1 {{ SessionState: {:?}, WinStationName: {:?}, UserName: {:?}, DomainName: {:?}, ProtocolStatus: {:?} }}",
            self.SessionState,
            self.WinStationName,
            self.UserName,
            self.DomainName,
            self.ProtocolStatus
        )
    }
}

#[repr(C)]
pub struct WINSTATIONINFORMATIONEX_LEVEL2 {
    pub SessionId: u32,
    pub SessionState: WINSTATIONSTATECLASS,
    pub SessionFlags: i32,
    pub WinStationName: [u16; 33],
    pub SamCompatibleUserName: [u16; 21],
    pub SamCompatibleDomainName: [u16; 18],
    pub LogonTime: i64,
    pub ConnectTime: i64,
    pub DisconnectTime: i64,
    pub LastInputTime: i64,
    pub CurrentTime: i64,
    pub ProtocolStatus: PROTOCOLSTATUS,
    pub UserName: [u16; 257],
    pub DomainName: [u16; 256],
}

impl Default for WINSTATIONINFORMATIONEX_LEVEL2 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for WINSTATIONINFORMATIONEX_LEVEL2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "WINSTATIONINFORMATIONEX_LEVEL2 {{ SessionState: {:?}, WinStationName: {:?}, SamCompatibleUserName: {:?}, SamCompatibleDomainName: {:?}, ProtocolStatus: {:?}, UserName: {:?}, DomainName: {:?} }}",
            self.SessionState,
            self.WinStationName,
            self.SamCompatibleUserName,
            self.SamCompatibleDomainName,
            self.ProtocolStatus,
            self.UserName,
            self.DomainName
        )
    }
}

#[repr(C)]
pub struct WINSTATIONINFORMATIONEX_LEVEL {
    pub WinStationInfoExLevel1: UnionField<WINSTATIONINFORMATIONEX_LEVEL1>,
    pub WinStationInfoExLevel2: UnionField<WINSTATIONINFORMATIONEX_LEVEL2>,
    pub union_field: [u64; 280],
}

impl Default for WINSTATIONINFORMATIONEX_LEVEL {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for WINSTATIONINFORMATIONEX_LEVEL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WINSTATIONINFORMATIONEX_LEVEL {{ union }}")
    }
}

#[repr(C)]
pub struct WINSTATIONINFORMATIONEX {
    pub Level: u32,
    pub Data: WINSTATIONINFORMATIONEX_LEVEL,
}

impl Default for WINSTATIONINFORMATIONEX {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for WINSTATIONINFORMATIONEX {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WINSTATIONINFORMATIONEX {{ Data: {:?} }}", self.Data)
    }
}

#[repr(C)]
pub struct TS_PROCESS_INFORMATION_NT4 {
    pub MagicNumber: u32,
    pub LogonId: u32,
    pub ProcessSid: *mut std::ffi::c_void,
    pub Pad: u32,
}

impl Default for TS_PROCESS_INFORMATION_NT4 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for TS_PROCESS_INFORMATION_NT4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TS_PROCESS_INFORMATION_NT4 {{  }}")
    }
}

#[repr(C)]
pub struct TS_SYS_PROCESS_INFORMATION {
    pub NextEntryOffset: u32,
    pub NumberOfThreads: u32,
    pub SpareLi1: i64,
    pub SpareLi2: i64,
    pub SpareLi3: i64,
    pub CreateTime: i64,
    pub UserTime: i64,
    pub KernelTime: i64,
    pub ImageName: UNICODE_STRING,
    pub BasePriority: i32,
    pub UniqueProcessId: u32,
    pub InheritedFromUniqueProcessId: u32,
    pub HandleCount: u32,
    pub SessionId: u32,
    pub SpareUl3: u32,
    pub PeakVirtualSize: usize,
    pub VirtualSize: usize,
    pub PageFaultCount: u32,
    pub PeakWorkingSetSize: u32,
    pub WorkingSetSize: u32,
    pub QuotaPeakPagedPoolUsage: usize,
    pub QuotaPagedPoolUsage: usize,
    pub QuotaPeakNonPagedPoolUsage: usize,
    pub QuotaNonPagedPoolUsage: usize,
    pub PagefileUsage: usize,
    pub PeakPagefileUsage: usize,
    pub PrivatePageCount: usize,
}

impl Default for TS_SYS_PROCESS_INFORMATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for TS_SYS_PROCESS_INFORMATION {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TS_SYS_PROCESS_INFORMATION {{  }}")
    }
}

#[repr(C)]
pub struct TS_ALL_PROCESSES_INFO {
    pub pTsProcessInfo: *mut TS_SYS_PROCESS_INFORMATION,
    pub SizeOfSid: u32,
    pub pSid: PSID,
}

impl Default for TS_ALL_PROCESSES_INFO {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for TS_ALL_PROCESSES_INFO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TS_ALL_PROCESSES_INFO {{ pTsProcessInfo: {:?} }}",
            self.pTsProcessInfo
        )
    }
}

#[repr(C)]
pub struct TS_COUNTER_HEADER {
    pub dwCounterID: u32,
    pub bResult: bool,
}

impl Default for TS_COUNTER_HEADER {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for TS_COUNTER_HEADER {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TS_COUNTER_HEADER {{  }}")
    }
}

#[repr(C)]
pub struct TS_COUNTER {
    pub CounterHead: TS_COUNTER_HEADER,
    pub dwValue: u32,
    pub StartTime: i64,
}

impl Default for TS_COUNTER {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Debug for TS_COUNTER {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TS_COUNTER {{ CounterHead: {:?} }}", self.CounterHead)
    }
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn WinStationFreeMemory(Buffer: *mut std::ffi::c_void) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn WinStationOpenServerW(ServerName: PWSTR) -> HANDLE;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn WinStationCloseServer(ServerHandle: HANDLE) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn WinStationServerPing(ServerHandle: HANDLE) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn WinStationGetTermSrvCountersValue(
        ServerHandle: HANDLE,
        Count: u32,
        Counters: *mut TS_COUNTER,
    ) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn WinStationShutdownSystem(ServerHandle: HANDLE, ShutdownFlags: u32) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn WinStationWaitSystemEvent(
        ServerHandle: HANDLE,
        EventMask: u32,
        EventFlags: *mut u32,
    ) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn WinStationRegisterConsoleNotification(
        ServerHandle: HANDLE,
        WindowHandle: HWND,
        Flags: u32,
    ) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn WinStationUnRegisterConsoleNotification(
        ServerHandle: HANDLE,
        WindowHandle: HWND,
    ) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn WinStationEnumerateW(
        ServerHandle: HANDLE,
        SessionIds: *mut *mut SESSIONIDW,
        Count: *mut u32,
    ) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn WinStationQueryInformationW(
        ServerHandle: HANDLE,
        SessionId: u32,
        WinStationInformationClass: WINSTATIONINFOCLASS,
        pWinStationInformation: *mut std::ffi::c_void,
        WinStationInformationLength: u32,
        pReturnLength: *mut u32,
    ) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn WinStationSetInformationW(
        ServerHandle: HANDLE,
        SessionId: u32,
        WinStationInformationClass: WINSTATIONINFOCLASS,
        pWinStationInformation: *mut std::ffi::c_void,
        WinStationInformationLength: u32,
    ) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn WinStationNameFromLogonIdW(
        ServerHandle: HANDLE,
        SessionId: u32,
        pWinStationName: PWSTR,
    ) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn LogonIdFromWinStationNameW(
        ServerHandle: HANDLE,
        pWinStationName: PWSTR,
        SessionId: *mut u32,
    ) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn WinStationSendMessageW(
        ServerHandle: HANDLE,
        SessionId: u32,
        Title: PWSTR,
        TitleLength: u32,
        Message: PWSTR,
        MessageLength: u32,
        Style: u32,
        Timeout: u32,
        Response: *mut u32,
        DoNotWait: bool,
    ) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn WinStationConnectW(
        ServerHandle: HANDLE,
        SessionId: u32,
        TargetSessionId: u32,
        pPassword: PWSTR,
        bWait: bool,
    ) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn WinStationDisconnect(ServerHandle: HANDLE, SessionId: u32, bWait: bool) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn WinStationReset(ServerHandle: HANDLE, SessionId: u32, bWait: bool) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn WinStationShadow(
        ServerHandle: HANDLE,
        TargetServerName: PWSTR,
        TargetSessionId: u32,
        HotKeyVk: u8,
        HotkeyModifiers: u16,
    ) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn WinStationShadowStop(ServerHandle: HANDLE, SessionId: u32, bWait: bool) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn WinStationEnumerateProcesses(
        ServerHandle: HANDLE,
        Processes: *mut *mut std::ffi::c_void,
    ) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn WinStationGetAllProcesses(
        ServerHandle: HANDLE,
        Level: u32,
        NumberOfProcesses: *mut u32,
        Processes: *mut *mut TS_ALL_PROCESSES_INFO,
    ) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn WinStationFreeGAPMemory(
        Level: u32,
        Processes: *mut TS_ALL_PROCESSES_INFO,
        NumberOfProcesses: u32,
    ) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn WinStationTerminateProcess(
        ServerHandle: HANDLE,
        ProcessId: u32,
        ExitCode: u32,
    ) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn WinStationGetProcessSid(
        ServerHandle: HANDLE,
        ProcessId: u32,
        ProcessStartTime: FILETIME,
        pProcessUserSid: *mut std::ffi::c_void,
        dwSidSize: *mut u32,
    ) -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn WinStationSwitchToServicesSession() -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn WinStationRevertFromServicesSession() -> bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn _WinStationWaitForConnect() -> bool;
}
