#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {

    pub static mut NlsAnsiCodePage: u16;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {

    pub static mut NlsMbCodePageTag: bool;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {

    pub static mut NlsMbOemCodePageTag: bool;
}
