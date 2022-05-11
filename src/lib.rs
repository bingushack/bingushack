mod ui;
mod client;

use winapi::um::processthreadsapi::CreateThread;
use winapi::_core::ptr::null_mut;
use winapi::um::libloaderapi::FreeLibraryAndExitThread;
use winapi::um::winuser::{FindWindowA, VK_LEFT, MessageBoxA, GetAsyncKeyState, VK_RIGHT, MB_OK, GetForegroundWindow, VK_DOWN};
use std::ffi::CString;
use winapi::shared::minwindef::{LPVOID, DWORD, HINSTANCE};
use winapi::um::handleapi::CloseHandle;
use winapi::um::winnt::DLL_PROCESS_ATTACH;
use crate::ui::debug_console::{init_debug_console, run_debug_console};
use crate::ui::clickgui::{init_clickgui, run_clickgui, ClickGuiMessage};
use std::thread::sleep;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use ui::message::Message;
use std::sync::{Arc, Mutex};
use std::borrow::BorrowMut;

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


    // after clickgui is enabled, you can use tx_clickgui to send messages from the clickgui
    // and rx_clickgui to receive messages from the clickgui
    let (mut tx_clickgui, rx_clickgui): (Sender<ClickGuiMessage>,  Arc<Mutex<Receiver<ClickGuiMessage>>>) = {
        let (tx, rx) = mpsc::channel();
        (tx, Arc::new(Mutex::new(rx)))
    };

    // channel to send and recieve messages involving the thread for the guis
    let (tx, rx): (Sender<Message>, Receiver<Message>) = mpsc::channel();
    let clickgui_thread = std::thread::spawn(move || {
        // todo: make so that if you spawn multiple debug consoles, they all receive the same messages
        let mut debug_console_sender: Option<Sender<String>> = None;
        let mut debug_console;
        let mut clickgui;
        loop {
            match rx.recv() {
                Ok(message) => match message {
                    Message::SpawnDebugConsole => {
                        let tmp = init_debug_console();
                        debug_console = tmp.0;
                        debug_console_sender = Some(tmp.1);
                        std::thread::spawn(move || {
                            run_debug_console(debug_console);
                        });
                    },
                    Message::SpawnGui => {
                        // send message "spawn gui" to debug console with debug_console_sender
                        // spawns the proper clickgui as well
                        if let Some(sender) = debug_console_sender.clone() {
                            sender.send(String::from("spawn gui")).unwrap();
                        }

                        (clickgui, tx_clickgui) = init_clickgui(tx_clickgui);

                        std::thread::spawn(move || {
                            run_clickgui(clickgui);
                        });
                    },
                    Message::KillThread => break,
                }
                Err(_) => {},
            };
        }
        0
    });


    // this is ugly
    let mut hwnd: winapi::shared::windef::HWND = null_mut();
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


        if GetAsyncKeyState(VK_LEFT) & 0x01 == 1 {
            tx.send(Message::SpawnDebugConsole).unwrap();
        }
        if GetAsyncKeyState(VK_RIGHT) & 0x01 == 1 {
            tx.send(Message::SpawnGui).unwrap();
        }
        if GetAsyncKeyState(VK_DOWN) & 0x01 == 1 {
            break;
        }
    }

    tx.send(Message::KillThread).unwrap();
    let eject_code = clickgui_thread.join().unwrap();
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
