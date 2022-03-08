mod ui;

use winapi::um::processthreadsapi::CreateThread;
use winapi::_core::ptr::null_mut;
use winapi::um::libloaderapi::FreeLibraryAndExitThread;
use winapi::um::winuser::{
    FindWindowA,
    VK_LEFT,
    MessageBoxA,
    MB_YESNOCANCEL,
    GetAsyncKeyState,
    VK_RIGHT,
    MB_OK,
    GetForegroundWindow
};
use std::ffi::CString;
use winapi::shared::windef::HWND;
use winapi::shared::minwindef::{LPVOID, DWORD, HINSTANCE};
use winapi::um::handleapi::CloseHandle;
use winapi::um::winnt::DLL_PROCESS_ATTACH;
use crate::ui::debug_console::debug_console::run_debug_console;
use std::thread::sleep;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver, TryRecvError};
use crate::ui::debug_console::message::Message;

#[cfg(target_os="windows")]

//todo macro for cstring


unsafe extern "system" fn main_loop(base: LPVOID) -> u32 {
    MessageBoxA(
        null_mut(),
        CString::new("injected").unwrap().as_ptr(),
        CString::new("bingushack").unwrap().as_ptr(),
        MB_OK,
    );

    //let jvm_handle = GetModuleHandleA(CString::new("jvm.dll").unwrap().as_ptr());


    let (tx, rx): (Sender<Message>, Receiver<Message>) = mpsc::channel();
    let gui_thread = std::thread::spawn(move || {
        let mut last_exit_code = u32::MAX;
        let mut gui_alive = false;
        'gui_thread: loop {
            let (ttx, trx): (Sender<Message>, Receiver<Message>) = mpsc::channel();
            match rx.recv() {
                Ok(message) => match message {
                    Message::SpawnGUI => if !gui_alive {
                        gui_alive = true;
                        last_exit_code = run_debug_console(trx);
                        gui_alive = false;
                    },
                    Message::KillGUI => {
                        ttx.send(Message::KillGUI);
                        gui_alive = false;
                    },
                    Message::KillThread => {
                        ttx.send(Message::KillGUI);
                        gui_alive = false;
                        break 'gui_thread;
                    },
                }
                Err(_) => {},
            }
        }
        last_exit_code
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
            sleep(Duration::from_millis(50));
            continue;
        }



        if GetAsyncKeyState(VK_RIGHT) & 0x01 == 1 {
            break;
        }
        if GetAsyncKeyState(VK_LEFT) & 0x01 == 1 {
            tx.send(Message::SpawnGUI);
        }
    }

    tx.send(Message::KillThread);
    let eject_code = gui_thread.join().unwrap();
    MessageBoxA(
        null_mut(),
        CString::new("ejected").unwrap().as_ptr(),
        CString::new("bingushack").unwrap().as_ptr(),
        MB_OK,
    );
    FreeLibraryAndExitThread(
        base as _,
        eject_code
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
