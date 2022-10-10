#![feature(cell_leak)]

mod client;
mod ui;

use crate::ui::{
    clickgui::{init_clickgui, run_clickgui, ClickGuiMessage},
    debug_console::{init_debug_console, run_debug_console},
};
use jni::{JNIEnv, JavaVM};
use std::{
    ffi::CString,
    sync::{
        mpsc,
        mpsc::{Receiver, Sender},
        Arc, Mutex,
    },
    thread::sleep,
    time::Duration,
};
use ui::message::Message;
use winapi::{
    _core::ptr::null_mut,
    shared::minwindef::{DWORD, HINSTANCE, LPVOID},
    um::{
        handleapi::CloseHandle,
        libloaderapi::FreeLibraryAndExitThread,
        processthreadsapi::CreateThread,
        winnt::DLL_PROCESS_ATTACH,
        winuser::{
            FindWindowA, GetAsyncKeyState, GetForegroundWindow, MessageBoxA, MB_OK, VK_DOWN,
            VK_LEFT, VK_RIGHT,
        },
    },
};

#[cfg(target_os = "windows")]

pub fn message_box(text: &str) {
    let caption = CString::new("bingushack").unwrap();
    let text = CString::new(text).unwrap();
    unsafe {
        MessageBoxA(null_mut(), text.as_ptr(), caption.as_ptr(), MB_OK);
    }
}

unsafe extern "system" fn main_loop(base: LPVOID) -> u32 {
    // check hwid, only on release
    #[cfg(not(build = "debug"))]
    if obfstr::obfstr!(env!("HWID")) != {
        use uniqueid::{IdentifierBuilder, IdentifierType};

        let mut builder = IdentifierBuilder::default();

        builder.name("bingus");
        builder.add(IdentifierType::CPU);
        builder.add(IdentifierType::RAM);
        builder.add(IdentifierType::DISK);

        builder.build().to_string(true)
    } {
        message_box("consider buying the client at http://bingushack.cc");
        panic!();
    }

    message_box("injected");

    // channel to send and recieve messages involving the thread for the guis
    let (tx, rx): (Sender<Message>, Receiver<Message>) = mpsc::channel();
    let clickgui_thread = std::thread::spawn(move || {
        // after clickgui is enabled, you can use tx_clickgui to send messages to the clickgui
        // and rx_clickgui to receive messages from the clickgui
        let (_tx_clickgui, _rx_clickgui): (
            Arc<Mutex<Sender<ClickGuiMessage>>>,
            Arc<Mutex<Receiver<ClickGuiMessage>>>,
        ) = {
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
                        std::thread::spawn(|| {
                            run_debug_console(debug_console);
                        });
                    }
                    Message::SpawnGui => {
                        // send message "spawn gui" to debug console with debug_console_sender
                        // spawns the proper clickgui as well
                        if let Some(sender) = debug_console_sender.clone() {
                            sender.send(String::from("spawn gui")).unwrap();
                        }

                        std::thread::spawn(|| {
                            let jvm: JavaVM = {
                                use jni::sys::JNI_GetCreatedJavaVMs;

                                let jvm_ptr = Vec::with_capacity(1).as_mut_ptr();
                                JNI_GetCreatedJavaVMs(jvm_ptr, 1, null_mut());

                                JavaVM::from_raw(*jvm_ptr).unwrap()
                            };

                            let jni_env: JNIEnv<'_> =
                                jvm.attach_current_thread_as_daemon().unwrap();

                            // i've always wanted to do this
                            let jni_env: JNIEnv<'static> =
                                std::mem::transmute::<JNIEnv<'_>, JNIEnv<'static>>(jni_env);

                            let client = init_clickgui(jni_env).0;
                            run_clickgui(client);
                        });
                    }
                    Message::KillThread => break,
                },
                Err(_) => {}
            };
        }
        0
    });

    // this is ugly
    let mut hwnd: winapi::shared::windef::HWND;
    {
        let window_name = CString::new("Minecraft 1.19.2").unwrap();
        hwnd = FindWindowA(null_mut(), window_name.as_ptr());
        let window_name =
            CString::new("Minecraft 1.19.2 - Multiplayer (3rd-party Server)").unwrap();
        if hwnd == null_mut() {
            hwnd = FindWindowA(null_mut(), window_name.as_ptr());
        }
        let window_name = CString::new("Minecraft 1.19.2 - Singleplayer").unwrap();
        if hwnd == null_mut() {
            hwnd = FindWindowA(null_mut(), window_name.as_ptr());
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
    FreeLibraryAndExitThread(base as _, eject_code);

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
                    null_mut(),
                );
                CloseHandle(bingus_thread);
            }
            true as i32
        }
        _ => true as i32, // it went a-ok because we dont know what happened so lol fuck off
    }
}
