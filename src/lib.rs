use std::{thread, mem};
use winapi::um::processthreadsapi::CreateThread;
use winapi::_core::ptr::null_mut;
use winapi::um::libloaderapi::{GetModuleHandleA, FreeLibraryAndExitThread, DisableThreadLibraryCalls};
use winapi::um::winuser::{FindWindowA, VK_LEFT, MessageBoxA, MB_YESNOCANCEL, GetAsyncKeyState, VK_RIGHT, MB_OK, GetForegroundWindow};
use std::ffi::{CString, OsString};
use winapi::shared::windef::HWND;
use winapi::um::tlhelp32::{MAX_MODULE_NAME32, Module32FirstW, Module32NextW, CreateToolhelp32Snapshot, MODULEENTRY32W, TH32CS_SNAPMODULE};
use winapi::shared::minwindef::{MAX_PATH, HMODULE, TRUE, LPVOID, DWORD, HINSTANCE};
use std::path::Path;
use winapi::um::handleapi::{INVALID_HANDLE_VALUE, CloseHandle};
use std::os::windows::ffi::OsStringExt;
use winapi::um::winnt::{HANDLE, DLL_PROCESS_ATTACH};
use widestring::WideCString;

//todo macro for cstring


unsafe extern "system" fn main_loop(base: LPVOID) -> u32 {
    MessageBoxA(
        null_mut(),
        CString::new("injected").unwrap().as_ptr(),
        CString::new("bingushack").unwrap().as_ptr(),
        MB_OK,
    );

    //let jvm_handle = GetModuleHandleA(CString::new("jvm.dll").unwrap().as_ptr());


    // this is ugly
    let mut hwnd: HWND = null_mut();

    hwnd = FindWindowA(
        null_mut(),
        CString::new("Minecraft* 1.17.1").unwrap().as_ptr(),
    );
    if hwnd == null_mut() {
        hwnd = FindWindowA(
            null_mut(),
            CString::new("Minecraft* 1.17.1 - Multiplayer (3rd-party Server)").unwrap().as_ptr(),
        );
    }
    if hwnd == null_mut() {
        hwnd = FindWindowA(
            null_mut(),
            CString::new("Minecraft* 1.17.1 - Singleplayer").unwrap().as_ptr(),
        );
    }

    loop {
        if hwnd != GetForegroundWindow() {
            continue;
        }



        if GetAsyncKeyState(VK_RIGHT) & 0x01 == 1 {
            break;
        }
        if GetAsyncKeyState(VK_LEFT) & 0x01 == 1 {
            MessageBoxA(
                null_mut(),
                CString::new("hello world!").unwrap().as_ptr(),
                CString::new("bingushack").unwrap().as_ptr(),
                MB_YESNOCANCEL,
            );
        }
    }

    MessageBoxA(
        null_mut(),
        CString::new("ejected").unwrap().as_ptr(),
        CString::new("bingushack").unwrap().as_ptr(),
        MB_OK,
    );
    FreeLibraryAndExitThread(
        base as _,
        0
    );

    unreachable!()
}


#[no_mangle]
pub extern "stdcall" fn DllMain(
    hinst_dll: HINSTANCE,
    fdw_reason: DWORD,
    _lpv_reserved: LPVOID,
) -> i32 {

    match fdw_reason {
        DLL_PROCESS_ATTACH => {
            unsafe {
                DisableThreadLibraryCalls(hinst_dll);
                let bingus_thread = CreateThread(
                    null_mut(),
                    0,
                    Some(main_loop),
                    hinst_dll as _,
                    0,
                    null_mut()
                );
                CloseHandle(bingus_thread);
            }
            return true as i32;
        },
        _ => return true as i32,  // it went a-ok because we dont know what happened so lol fuck off
    }
}
