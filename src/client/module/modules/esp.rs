use super::{
    AllSettingsType, BingusModule, BingusSettings, BoxedBingusModule, SettingType, SettingValue,
};
use crate::{client::{
    mapping::MappingsManager,
    setting::{BooleanSetting, FloatSetting},
}, NEW_CONTEXT, STATIC_HDC, log_to_file};

use jni::JNIEnv;
use once_cell::sync::OnceCell;
use winapi::{shared::windef::{HDC}, um::{wingdi::{wglGetCurrentContext, wglMakeCurrent}}};
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex}, ffi::CString,
};
use gl::types::{GLfloat, GLenum, GLuint, GLchar, GLint, GLboolean, GLsizeiptr};
    use std::str;
    use std::ptr;
    use std::mem;




static mut PROGRAM: OnceCell<GLuint> = OnceCell::new();

pub struct Esp {
    enabled: SettingType,
    settings: AllSettingsType,
}

impl BingusModule for Esp {
    fn new_boxed() -> BoxedBingusModule {
        Box::new(Self {
            enabled: Arc::new(Mutex::new(RefCell::new(BingusSettings::BooleanSetting(
                BooleanSetting::new(SettingValue::from(false), "enabled"),
            )))),
            settings: Arc::new(Mutex::new(RefCell::new(vec![Rc::new(RefCell::new(
                BingusSettings::FloatSetting(FloatSetting::new(
                    SettingValue::from(0.0),
                    "does nothing",
                    0.0..=100.0,
                )),
            ))]))),
        })
    }

    fn tick(&mut self, _env: Rc<JNIEnv>, _mappings_manager: Rc<MappingsManager>) {}

    fn render_event(&self) {
        // if enabled
        if self.get_enabled() {
            unsafe {
                let local_new_context = NEW_CONTEXT.get_mut().unwrap();
                let hdc = STATIC_HDC.unwrap();
                wglMakeCurrent(hdc, *local_new_context.get_mut());
                esp(1.0)
            }
        }
    }

    fn on_load(&mut self, _env: Rc<JNIEnv>, _mappings_manager: Rc<MappingsManager>) {}

    fn on_unload(&mut self, _env: Rc<JNIEnv>, _mappings_manager: Rc<MappingsManager>) {}

    fn on_enable(&mut self, _env: Rc<JNIEnv>, _mappings_manager: Rc<MappingsManager>) {}

    fn on_disable(&mut self, _env: Rc<JNIEnv>, _mappings_manager: Rc<MappingsManager>) {}

    fn get_all_settings(&self) -> AllSettingsType {
        Arc::clone(&self.settings)
    }

    fn get_enabled_setting(&self) -> SettingType {
        Arc::clone(&self.enabled)
    }

    fn to_name(&self) -> String {
        "ESP".to_string()
    }
}

fn esp(_alpha: gl::types::GLfloat) {
    /*
    let rc_cli = null_mut();
    unsafe {
        GetClientRect(WindowFromDC(hdc), rc_cli);
    }
    let rc_cli = unsafe { *rc_cli };  // crash
    let width = rc_cli.right - rc_cli.left;
    let height = rc_cli.bottom - rc_cli.top;
    */
    draw_triangle(400, 400);
    /*
    unsafe {
        Viewport(0, 0, width, height);
    }
    */
}

fn draw_triangle(_w: i32, _h: i32) {
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


    let program: GLuint = unsafe {
        *PROGRAM.get_or_init(|| {
            let vs = compile_shader(VS_SRC, gl::VERTEX_SHADER);
            let fs = compile_shader(FS_SRC, gl::FRAGMENT_SHADER);

            link_program(vs, fs)
        })
    };

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
        let out_color_str = CString::new("out_color").unwrap();
        gl::BindFragDataLocation(program, 0, out_color_str.as_ptr());
        
        // Specify the layout of the vertex data
        let pos_str = CString::new("position").unwrap();
        let pos_attr = gl::GetAttribLocation(program, pos_str.as_ptr());
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