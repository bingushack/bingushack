mod ui;

use winapi::um::processthreadsapi::CreateThread;
use winapi::_core::ptr::null_mut;
use winapi::um::libloaderapi::{FreeLibraryAndExitThread, DisableThreadLibraryCalls};
use winapi::um::winuser::{FindWindowA, VK_LEFT, MessageBoxA, MB_YESNOCANCEL, GetAsyncKeyState, VK_RIGHT, MB_OK, GetForegroundWindow};
use std::ffi::CString;
use winapi::shared::windef::HWND;
use winapi::shared::minwindef::{LPVOID, DWORD, HINSTANCE};
use winapi::um::handleapi::CloseHandle;
use winapi::um::winnt::DLL_PROCESS_ATTACH;
use crate::ui::debug_console::debug_console::run_debug_console;


//todo macro for cstring


unsafe extern "system" fn main_loop(base: LPVOID) -> u32 {
    MessageBoxA(
        null_mut(),
        CString::new("injected").unwrap().as_ptr(),
        CString::new("bingushack").unwrap().as_ptr(),
        MB_OK,
    );

    //let jvm_handle = GetModuleHandleA(CString::new("jvm.dll").unwrap().as_ptr());


    std::thread::spawn(|| {
        run_debug_console();
    });


    // this is ugly
    let mut hwnd: HWND = null_mut();
    {
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
