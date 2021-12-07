use eframe::{epi, egui};
use eframe::egui::CtxRef;
use eframe::epi::Frame;

struct DebugConsole {
    lines: Vec<String>,
}

impl Default for DebugConsole {
    fn default() -> Self {
        Self {
            lines: Vec::new(),
        }
    }
}

impl epi::App for DebugConsole {
    fn update(&mut self, ctx: &CtxRef, frame: &mut Frame<'_>) {
        egui::CentralPanel::default().show(ctx, |ui| {
            for line in &self.lines {
                ui.label(line);
            }
        });

        frame.set_window_size(ctx.used_size());
    }

    fn name(&self) -> &str {
        "bingushack debug console"
    }
}



pub fn run_debug_console() -> u32 {
    let mut options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(DebugConsole::default()), options)
}
