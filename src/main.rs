use teamdeck_vacations::{TeamdeckVacationsApp, APPLICATION_NAME};

fn main() {
    let window_options = eframe::NativeOptions::default();
    eframe::run_native(
        APPLICATION_NAME,
        window_options,
        Box::new(|_cc| Box::new(TeamdeckVacationsApp)),
    )
}
