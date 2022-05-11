use teamdeck_vacations::{APPLICATION_NAME, TeamdeckVacationsApp};

fn main() {
    let window_options = eframe::NativeOptions::default();
    eframe::run_native(
        APPLICATION_NAME,
        window_options,
        Box::new(|_cc| Box::new(TeamdeckVacationsApp)),
    )
}
