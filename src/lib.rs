#![feature(cell_leak, exclusive_wrapper)]

mod client;
mod ui;

use crate::ui::{
    clickgui::{init_clickgui, run_clickgui, ClickGuiMessage},
    debug_console::{init_debug_console, run_debug_console},
};
use glutin::platform::windows::HGLRC;
use jni::{JNIEnv, JavaVM};
use widestring::WideCString;
use std::{
    ffi::CString,
    sync::{
        mpsc,
        mpsc::{Receiver, Sender},
        Arc, Mutex,
    },
    thread::sleep,
    time::{Duration, SystemTime},
};
use ui::{message::Message, clickgui::ClickGui};
use winapi::{
    _core::ptr::null_mut,
    shared::{minwindef::{DWORD, HINSTANCE, LPVOID, HMODULE}, windef::{HDC, HGLRC__}},
    um::{
        handleapi::CloseHandle,
        libloaderapi::{FreeLibraryAndExitThread, GetProcAddress, GetModuleHandleW},
        processthreadsapi::CreateThread,
        winnt::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH},
        winuser::{
            FindWindowA, GetAsyncKeyState, GetForegroundWindow, MessageBoxA, MB_OK, VK_DOWN,
            VK_LEFT, VK_RIGHT, WindowFromDC, GetClientRect,
        }, wingdi::{wglGetProcAddress, SwapBuffers, wglMakeCurrent, wglGetCurrentContext, wglCreateContext, wglDeleteContext},
    },
};
use once_cell::sync::OnceCell;
use std::sync::atomic::AtomicPtr;


#[cfg(target_os = "windows")]



pub static mut STATIC_HDC: Mutex<HDC> = Mutex::new(null_mut());
static mut CLICKGUI_SENDER: Mutex<Option<Sender<ClickGuiMessage>>> = Mutex::new(None);
static mut TX: Mutex<Option<Sender<Message>>> = Mutex::new(None);

static WAITING_CELL: OnceCell<SystemTime> = OnceCell::new();
static mut NEW_CONTEXT: OnceCell<AtomicPtr<HGLRC__>> = OnceCell::new();
static mut OLD_CONTEXT: OnceCell<AtomicPtr<HGLRC__>> = OnceCell::new();



// utility method for showing a small window, for debugging
pub fn message_box(text: &str) {
    let caption = CString::new("bingushack").unwrap();
    let text = CString::new(text).unwrap();
    unsafe {
        MessageBoxA(null_mut(), text.as_ptr(), caption.as_ptr(), MB_OK);
    }
}

pub fn log_to_file(text: &str) {
    let file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("bingushack.log");
    
    if let Ok(mut file) = file {
        std::io::Write::write_all(&mut file, text.as_bytes()).unwrap();
        std::io::Write::write_all(&mut file, "\n".as_bytes()).unwrap();
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
    *TX.lock().unwrap() = Some(tx.clone());
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
                            let tmp = init_clickgui(jni_env);
                            CLICKGUI_SENDER = Mutex::new(Some(tmp.1));
                            run_clickgui(tmp.0);
                        });
                    }
                    Message::KillThread => break,  // exit the loop and start the process of ejection
                    Message::RenderEvent => {
                        log_to_file("got hooked send");
                        let mut clickgui_sender = CLICKGUI_SENDER.lock().unwrap();
                        log_to_file("got clickgui");
                        if clickgui_sender.is_some() {
                            log_to_file("0");
                            let tmp = clickgui_sender.take().unwrap();
                            tmp.send(ClickGuiMessage::RunRenderEvent).unwrap();
                            CLICKGUI_SENDER = Mutex::new(Some(tmp));  // bad bad bad bad
                            log_to_file("1");
                        }
                    }
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
            // enable hooks
            crochet::enable!(swapbuffers_hook).expect("could not enable hook");
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
        DLL_PROCESS_DETACH => {
            // disable hooks
            crochet::disable!(swapbuffers_hook).expect("could not disable hook");

            true as i32
        }
        _ => true as i32, // it went a-ok because we dont know what happened so lol fuck off
    }
}



#[crochet::hook("opengl32.dll", "wglSwapBuffers")]
fn swapbuffers_hook(hdc: winapi::shared::windef::HDC) -> winapi::ctypes::c_int {
    log_to_file("start of hook");
    let is_ready = WAITING_CELL.get_or_init(|| {
        // initialize all the opengl stuff
        {
            unsafe {
                let _ = OLD_CONTEXT.get_or_init(|| AtomicPtr::new(wglGetCurrentContext()));

                let _ = NEW_CONTEXT.get_or_init(|| AtomicPtr::new(wglCreateContext(hdc)));
                let local_new_context = NEW_CONTEXT.get_mut().unwrap();
                wglMakeCurrent(hdc, *local_new_context.get_mut());
            }

            let opengl32_module: HMODULE;
            let opengl32_str = WideCString::from_str("opengl32.dll").unwrap();

            unsafe {
                opengl32_module = GetModuleHandleW(opengl32_str.as_ptr());
            }
            if opengl32_module == null_mut() {
                message_box("opengl32.dll not found. what the fuck did you do??");
            }

            gl::load_with(|s| unsafe {
                let gl_fn_cstr = CString::new(s).unwrap();
                let gl_fn_cstr_ptr = gl_fn_cstr.as_ptr();
                let check = wglGetProcAddress(gl_fn_cstr_ptr);
                if check == null_mut() {
                    GetProcAddress(opengl32_module, gl_fn_cstr_ptr)
                } else {
                    check
                }
            } as *const _);
        }
        SystemTime::now()
    });

    if is_ready.elapsed().unwrap().as_millis() > 5000 {
        unsafe {
            let local_new_context = NEW_CONTEXT.get_mut().unwrap();
            wglMakeCurrent(hdc, *local_new_context.get_mut());
        }
        render_event(hdc);
        log_to_file("called render_event");
    }


    
    // send a message to the clickgui_thread to run the render event
    unsafe {
        //*STATIC_HDC.get_mut().unwrap() = hdc;
        if let Some(tx) = &*TX.lock().unwrap() {
            // tx.send(Message::RenderEvent).unwrap();
        }
    }

    unsafe {
        let local_old_context = OLD_CONTEXT.get_mut().unwrap();
        wglMakeCurrent(hdc, *local_old_context.get_mut());
    }
    log_to_file("calling original");
    call_original!(hdc)
}

fn render_event(hdc: HDC) {
    // if enabled
    if true {
        unsafe {
            //esp(*STATIC_HDC.lock().unwrap(),1.0)
            esp(hdc, 1.0)
        }
    }
}

fn esp(hdc: HDC, _alpha: gl::types::GLfloat) {
    /*
    let rc_cli = null_mut();
    unsafe {
        GetClientRect(WindowFromDC(hdc), rc_cli);
    }
    log_to_file("b");
    let rc_cli = unsafe { *rc_cli };  // crash
    log_to_file("c");
    let width = rc_cli.right - rc_cli.left;
    let height = rc_cli.bottom - rc_cli.top;
    log_to_file("d");
    */
    draw_triangle(hdc, 400, 400);
    /*
    unsafe {
        Viewport(0, 0, width, height);
    }
    */
}

fn draw_triangle(hdc: HDC, w: i32, h: i32) {
    unsafe {
        wglMakeCurrent(hdc, wglGetCurrentContext());
    }

    use gl::types::{GLfloat, GLenum, GLuint, GLchar, GLint, GLboolean, GLsizeiptr};
    use std::str;
    use std::ptr;
    use std::mem;

    static VERTEX_DATA: [GLfloat; 6] = [0.0, 0.5, 0.5, -0.5, -0.5, -0.5];
    static VS_SRC: &'static str = "
#version 150
in vec2 position;
void main() {
    gl_Position = vec4(position, 0.0, 1.0);
}";

    static FS_SRC: &'static str = "
#version 150
out vec4 out_color;
void main() {
    out_color = vec4(1.0, 1.0, 1.0, 1.0);
}";


    fn compile_shader(src: &str, ty: GLenum) -> GLuint {
        let shader;
        unsafe {
            shader = gl::CreateShader(ty);
            // Attempt to compile the shader
            let c_str = CString::new(src.as_bytes()).unwrap();
            gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
            gl::CompileShader(shader);

            // Get the compile status
            let mut status = gl::FALSE as GLint;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

            // Fail on error
            if status != (gl::TRUE as GLint) {
                let mut len = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = Vec::with_capacity(len as usize);
                buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
                gl::GetShaderInfoLog(
                    shader,
                    len,
                    ptr::null_mut(),
                    buf.as_mut_ptr() as *mut GLchar,
                );
                panic!(
                    "{}",
                    str::from_utf8(&buf)
                        .ok()
                        .expect("ShaderInfoLog not valid utf8")
                );
            }
        }
        shader
    }

    fn link_program(vs: GLuint, fs: GLuint) -> GLuint {
        unsafe {
            let program = gl::CreateProgram();
            gl::AttachShader(program, vs);
            gl::AttachShader(program, fs);
            gl::LinkProgram(program);
            // Get the link status
            let mut status = gl::FALSE as GLint;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);
    
            // Fail on error
            if status != (gl::TRUE as GLint) {
                let mut len: GLint = 0;
                gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = Vec::with_capacity(len as usize);
                buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
                gl::GetProgramInfoLog(
                    program,
                    len,
                    ptr::null_mut(),
                    buf.as_mut_ptr() as *mut GLchar,
                );
                panic!(
                    "{}",
                    str::from_utf8(&buf)
                        .ok()
                        .expect("ProgramInfoLog not valid utf8")
                );
            }
            program
        }
    }

    let vs = compile_shader(VS_SRC, gl::VERTEX_SHADER);
    let fs = compile_shader(FS_SRC, gl::FRAGMENT_SHADER);
    let program = link_program(vs, fs);

    let mut vao = 0;
    let mut vbo = 0;


    unsafe {
        // Create Vertex Array Object
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);  // crash


        
        // Create a Vertex Buffer Object and copy the vertex data to it
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (VERTEX_DATA.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            mem::transmute(&VERTEX_DATA[0]),
            gl::STATIC_DRAW,
        );


        // Use shader program
        gl::UseProgram(program);
        gl::BindFragDataLocation(program, 0, CString::new("out_color").unwrap().as_ptr());
        
        // Specify the layout of the vertex data
        let pos_attr = gl::GetAttribLocation(program, CString::new("position").unwrap().as_ptr());
        gl::EnableVertexAttribArray(pos_attr as GLuint);
        gl::VertexAttribPointer(
            pos_attr as GLuint,
            2,
            gl::FLOAT,
            gl::FALSE as GLboolean,
            0,
            ptr::null(),
        );
    }

    unsafe {
        // Clear the screen to black
        gl::ClearColor(0.7, 0.1, 0.1, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        // Draw a triangle from the 3 vertices
        gl::DrawArrays(gl::TRIANGLES, 0, 3);
    }
}