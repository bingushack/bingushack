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

// utility method for showing a small window, for debugging
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

    // show a message box if the client got injected
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

        let mut debug_console_sender: Option<Sender<String>> = None;
        let mut debug_console;
        loop {
            match rx.recv() {
                Ok(message) => match message {
                    Message::SpawnDebugConsole => {
                        // get the debug console object and a sender to it
                        let tmp = init_debug_console();
                        debug_console = tmp.0;
                        debug_console_sender = Some(tmp.1);
                        // spawn the debug console thread and run the debug console
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

                        // spawn a thread for the clickgui
                        std::thread::spawn(|| {
                            // get a jni object of the java vm
                            let jvm: JavaVM = {
                                // ffi'd method of getting already-created jvms.
                                use jni::sys::JNI_GetCreatedJavaVMs;
                                // this is because the "normal" jni library in rust does not easily allow you to get jvms that you did not launch

                                // empty buffer
                                let jvm_ptr = Vec::with_capacity(1).as_mut_ptr();
                                // pass the buffer, the amount of jvms to get, and a buffer for the amount of jvms gotten
                                // the amount of jvms gotten is a null pointer because we don't care about it
                                JNI_GetCreatedJavaVMs(jvm_ptr, 1, null_mut());

                                // get a nice jvm object from the raw pointer
                                JavaVM::from_raw(*jvm_ptr).unwrap()
                            };

                            // get the jni environment as a daemon thread
                            let jni_env: JNIEnv<'_> =
                                jvm.attach_current_thread_as_daemon().unwrap();

                            // i've always wanted to do this
                            let jni_env: JNIEnv<'static> =
                                std::mem::transmute::<JNIEnv<'_>, JNIEnv<'static>>(jni_env);
                            // transmute the jni environment to a static lifetime to make it easier to use also it's a daemon thread

                            // get the clickgui
                            // don't care about the sender it returns yet
                            let client = init_clickgui(jni_env).0;
                            run_clickgui(client);
                        });
                    }
                    Message::KillThread => break,  // exit the loop and start the process of ejection
                },
                Err(_) => {}
            };
        }
        0  // return 0 as an exit code for the dll
    });

    // this is ugly
    let mut hwnd: winapi::shared::windef::HWND;
    {
        let window_name = CString::new("Minecraft 1.19.2").unwrap();
        hwnd = FindWindowA(null_mut(), window_name.as_ptr());  // make put this
        let window_name =
            CString::new("Minecraft 1.19.2 - Multiplayer (3rd-party Server)").unwrap();
        if hwnd == null_mut() {
            // here?
            hwnd = FindWindowA(null_mut(), window_name.as_ptr());
        }
        let window_name = CString::new("Minecraft 1.19.2 - Singleplayer").unwrap();  // and this
        if hwnd == null_mut() {
            // here?
            hwnd = FindWindowA(null_mut(), window_name.as_ptr());
        }
    }

    loop {
        // if the window is not the foreground window, give the cpu a bit of a break
        if hwnd != GetForegroundWindow() {
            sleep(Duration::from_millis(50));
            continue;
        }

        // todo: invalidate these if the clickgui has panicked
        // todo: might be able to get rid of these senders and just spawn the thread directly here. maybe
        //
        // send a message to the clickgui_thread thread to spawn a window or kill the thread
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

    // tell the clickgui_thread to kill itself
    tx.send(Message::KillThread).unwrap();
    // wait for the clickgui_thread to exit
    let eject_code = clickgui_thread.join().unwrap();
    // let the user know that the client is ejecting
    message_box("ejected");
    // winapi method to exit dll's thread
    // this will exit the function and the dll will be unloaded
    FreeLibraryAndExitThread(base as _, eject_code);

    unreachable!()
}

// magic C stuff
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
