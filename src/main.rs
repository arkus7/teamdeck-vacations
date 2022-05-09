use teamdeck_vacations::TeamdeckVacationsApp;

fn main() {
    let window_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Teamdeck Vacations",
        window_options,
        Box::new(|_cc| Box::new(TeamdeckVacationsApp)),
    )
}
