use eframe::egui::{CentralPanel, Context};
use eframe::{App, Frame};

pub const APPLICATION_NAME: &str = "Teamdeck Vacations";

pub struct TeamdeckVacationsApp;

impl App for TeamdeckVacationsApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading(APPLICATION_NAME);
        });
    }
}
