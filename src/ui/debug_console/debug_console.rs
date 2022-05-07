#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use winsafe::{prelude::*, co, ErrResult, HWND};
use my_window::MyWindow;

pub fn run_debug_console() {
    if let Err(e) = run_app() {
        HWND::NULL.MessageBox(
            &e.to_string(), "Uncaught error", co::MB::ICONERROR).unwrap();
    }
}

fn run_app() -> ErrResult<i32> {
    MyWindow::new()?.run() // create our main window and run it
}

mod my_window {
    use winsafe::{prelude::*, gui};
    use winsafe::{ErrResult, HINSTANCE, IdIdiStr, POINT, SIZE};

    #[derive(Clone)]
    pub struct MyWindow {
        wnd: gui::WindowMain, // responsible for managing the window
        btn_hello: gui::Button,     // a button
    }

    impl MyWindow {
        pub fn new() -> ErrResult<MyWindow> {
            let wnd = gui::WindowMain::new( // instantiate the window manager
                                            gui::WindowMainOpts {
                                                title: "My window title".to_owned(),
                                                size: SIZE::new(300, 150),
                                                ..Default::default() // leave all other options as default
                                            },
            );

            let btn_hello = gui::Button::new(
                &wnd, // the window manager is the parent of our button
                gui::ButtonOpts {
                    text: "&Click me".to_owned(),
                    position: POINT::new(20, 20),
                    ..Default::default()
                },
            );

            let new_self = Self { wnd, btn_hello };
            new_self.events(); // attach our events
            Ok(new_self)
        }

        pub fn run(&self) -> ErrResult<i32> {
            self.wnd.run_main(None) // simply let the window manager do the hard work
        }

        fn events(&self) {
            self.btn_hello.on().bn_clicked({
                let self2 = self.wnd.clone(); // clone so it can be passed into the closure
                move || {
                    self2.hwnd().SetWindowText("Hello, world!")?;
                    Ok(())
                }
            });
        }
    }
}
