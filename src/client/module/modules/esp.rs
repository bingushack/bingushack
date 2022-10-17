use super::{
    AllSettingsType, BingusModule, BingusSettings, BoxedBingusModule, SettingType, SettingValue,
};
use crate::{client::{
    mapping::MappingsManager,
    setting::{BooleanSetting, FloatSetting},
}, STATIC_HDC};
use gl::{types::GLfloat};
use jni::JNIEnv;
use winapi::{shared::windef::{RECT, HDC}, um::winuser::{GetClientRect, WindowFromDC}};
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex}, ptr::null_mut,
};

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
        if self
            .get_enabled_setting()
            .lock()
            .unwrap()
            .borrow()
            .get_value()
            .try_into()
            .unwrap()
        {
            unsafe {
                esp(*STATIC_HDC.get_mut(), 1.0)
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

fn esp(hdc: HDC, _alpha: GLfloat) {
    let rc_cli: *mut RECT = null_mut();
    unsafe {
        GetClientRect(WindowFromDC(hdc), rc_cli);
    }
    let rc_cli = unsafe { *rc_cli };

    let width = rc_cli.right - rc_cli.left;
    let height = rc_cli.bottom - rc_cli.top;

    draw_triangle(width, height);
    /*
    unsafe {
        Viewport(0, 0, width, height);
    }
    */
}

fn draw_triangle(W: i32, H: i32) {
    use glu_sys::*;
    unsafe {
        glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
        glMatrixMode(GL_PROJECTION);
        glLoadIdentity();
        glViewport(0, 0, W, H);
        gluPerspective(45.0, (W as f32 / H as f32).into(), 1.0, 10.0);
        glTranslatef(0.0, 0.0, -5.0);
        glMatrixMode(GL_MODELVIEW);
        glLoadIdentity();
        glRotatef(0.0, 1.0, 1.0, 0.0);
        glColor3f(1.0, 0.0, 0.0);
        glBegin(GL_POLYGON);
        glVertex3f(0.0, 1.0, 0.0);
        glVertex3f(1.0, -1.0, 1.0);
        glVertex3f(-1.0, -1.0, 1.0);
        glEnd();
        glColor3f(0.0, 1.0, 0.0);
        glBegin(GL_POLYGON);
        glVertex3f(0.0, 1.0, 0.0);
        glVertex3f(0.0, -1.0, -1.0);
        glVertex3f(1.0, -1.0, 1.0);
        glEnd();
        glColor3f(0.0, 0.0, 1.0);
        glBegin(GL_POLYGON);
        glVertex3f(0.0, 1.0, 0.0);
        glVertex3f(-1.0, -1.0, 1.0);
        glVertex3f(0.0, -1.0, -1.0);
        glEnd();
        glColor3f(1.0, 0.0, 0.0);
        glBegin(GL_POLYGON);
        glVertex3f(1.0, -1.0, 1.0);
        glVertex3f(0.0, -1.0, -1.0);
        glVertex3f(-1.0, -1.0, 1.0);
        glEnd();
        glLoadIdentity();
        glRasterPos2f(-3.0, -2.0);
    }
}