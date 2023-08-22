use windows::Win32::Foundation::BOOLEAN;

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {

    pub static mut NlsAnsiCodePage: u16;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {

    pub static mut NlsMbCodePageTag: BOOLEAN;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {

    pub static mut NlsMbOemCodePageTag: BOOLEAN;
}
