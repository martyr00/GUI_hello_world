use eframe::{run_native, epi::App, NativeOptions};
use eframe::egui::{CentralPanel, CtxRef, Ui};
use eframe::epi::Frame;

struct Calculator;

impl App for Calculator {
    fn update(&mut self, ctx: &CtxRef, _frame: &mut Frame<'_>) {
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            ui.label("Hello world");
        });
    }

    fn name(&self) -> &str {
        "Calculator"
    }
}

fn main() {
    let app = Calculator;
    let win_optional = NativeOptions::default();
    run_native(Box::new(app), win_optional);
}
