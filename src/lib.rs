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
use jni::{JavaVM, JNIEnv};


#[cfg(target_os="windows")]

//todo macro for cstring


pub fn message_box(text: &str) {
    unsafe {
        MessageBoxA(
            null_mut(),
            CString::new(text).unwrap().as_ptr(),
            CString::new("bingushack").unwrap().as_ptr(),
            MB_OK,
        );
    }
}


unsafe extern "system" fn main_loop(base: LPVOID) -> u32 {


    message_box("injected");




    // channel to send and recieve messages involving the thread for the guis
    let (tx, rx): (Sender<Message>, Receiver<Message>) = mpsc::channel();
    let clickgui_thread = std::thread::spawn(move || {
        // after clickgui is enabled, you can use tx_clickgui to send messages to the clickgui
        // and rx_clickgui to receive messages from the clickgui
        let (tx_clickgui, rx_clickgui): (Arc<Mutex<Sender<ClickGuiMessage>>>,  Arc<Mutex<Receiver<ClickGuiMessage>>>) = {
            let (tx, rx) = mpsc::channel();
            (Arc::new(Mutex::new(tx)), Arc::new(Mutex::new(rx)))
        };
        
        // todo: make so that if you spawn multiple debug consoles, they all receive the same messages
        let mut debug_console_sender: Option<Sender<String>> = None;
        let mut debug_console;
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

                        std::thread::spawn(move || {
                            let jvm: JavaVM = {
                                use jni::sys::JNI_GetCreatedJavaVMs;
                    
                                let jvm_ptr = Vec::with_capacity(1).as_mut_ptr();
                                JNI_GetCreatedJavaVMs(jvm_ptr, 1, null_mut());
                    
                                JavaVM::from_raw(*jvm_ptr).unwrap()
                            };

                            let jni_env: JNIEnv<'_> = jvm.attach_current_thread_as_daemon().unwrap();

                            // i've always wanted to do this
                            let jni_env: JNIEnv<'static> = std::mem::transmute::<JNIEnv<'_>, JNIEnv<'static>>(jni_env);

                            let client = init_clickgui(jni_env).0;
                            run_clickgui(client);
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
    let mut hwnd: winapi::shared::windef::HWND;
    {
        hwnd = FindWindowA(
            null_mut(),
            CString::new("Minecraft 1.18.2").unwrap().as_ptr(),
        );
        if hwnd == null_mut() {
            hwnd = FindWindowA(
                null_mut(),
                CString::new("Minecraft 1.18.2 - Multiplayer (3rd-party Server)").unwrap().as_ptr(),
            );
        }
        if hwnd == null_mut() {
            hwnd = FindWindowA(
                null_mut(),
                CString::new("Minecraft 1.18.2 - Singleplayer").unwrap().as_ptr(),
            );
        }
    }

    loop {
        if hwnd != GetForegroundWindow() {
            sleep(Duration::from_millis(50));
            continue;
        }


        // todo: invalidate these if the clickgui has panicked
        // todo: might be able to get rid of these senders and just spawn the thread directly here. maybe
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
    message_box("ejected");
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