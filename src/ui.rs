use chrono::prelude::*;
use chrono::{Date, DateTime, Datelike, Duration, Local, Month};
use eframe::egui::{
    Align, Button, CentralPanel, ComboBox, Context, Label, Layout, RichText, TextStyle,
    TopBottomPanel, Ui,
};
use eframe::{egui, App, Frame};
use egui_extras::{Size, TableBuilder};
use num_traits::FromPrimitive;
use serde::{Deserialize, Serialize};
use std::ops::Sub;

pub const APPLICATION_NAME: &str = "Teamdeck Vacations";

const DARK_MODE: bool = true;

pub struct TeamdeckVacationsApp;

#[derive(Clone, Serialize, Deserialize)]
pub struct DateSelectorState {
    month: u32,
    year: i32,
}

impl Default for DateSelectorState {
    fn default() -> Self {
        let current_date = Local::today();
        let month = current_date.month();
        let year = current_date.year();
        Self { month, year }
    }
}

impl App for TeamdeckVacationsApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        self.render_top_panel(ctx, frame);
        self.render_vacations(ctx);
    }
}

impl TeamdeckVacationsApp {
    fn render_top_panel(&mut self, ctx: &Context, frame: &mut Frame) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(12.0);
            egui::menu::bar(ui, |ui| {
                ui.with_layout(Layout::left_to_right(), |ui| {
                    let label = RichText::new(APPLICATION_NAME).text_style(TextStyle::Heading);
                    ui.label(label);
                });
                ui.with_layout(Layout::right_to_left(), |ui| {
                    let theme_btn = ui.button(if DARK_MODE {
                        "🌞 Light Theme"
                    } else {
                        "🌙 Dark Theme"
                    });
                    ui.add_space(12.0);
                    let print_btn = ui.button("📝 Print");
                    ui.add_space(12.0);
                    DateSelectorPanel::default().draw(ui);
                });
            });
            ui.add_space(12.0);
        });
    }

    fn render_vacations(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            let initial_column_size = 150.0;
            let minimum_column_size = 80.0;
            let maximum_column_size = 250.0;
            let column_size = Size::initial(initial_column_size)
                .at_least(minimum_column_size)
                .at_most(maximum_column_size);
            TableBuilder::new(ui)
                .striped(true)
                .cell_layout(Layout::left_to_right().with_cross_align(Align::Center))
                .columns(column_size, 5)
                .column(Size::remainder().at_least(initial_column_size))
                .resizable(true)
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.label(heading_text("Employee Name"));
                    });
                    header.col(|ui| {
                        ui.label(heading_text("Absence From"));
                    });
                    header.col(|ui| {
                        ui.label(heading_text("Absence To"));
                    });
                    header.col(|ui| {
                        ui.label(heading_text("Requested At"));
                    });
                    header.col(|ui| {
                        ui.label(heading_text("Approved By"));
                    });
                    header.col(|ui| {
                        ui.label(heading_text("Approved At"));
                    });
                })
                .body(|mut body| {
                    let row_height = 18.0;
                    for row in 1..=20 {
                        let name = format!("Employee {}", row);
                        let absence_from = Local::today().sub(Duration::days(2 * row));
                        let absence_to = Local::today().sub(Duration::days(row));
                        let requested_at = Local::today().sub(Duration::days(4 * row));
                        let approved_by = format!("Manager {}", row);
                        let approved_at = Local::today().sub(Duration::days(3 * row));

                        body.row(row_height, |mut row| {
                            row.col(|ui| {
                                ui.label(name);
                            });
                            row.col(|ui| {
                                ui.label(absence_from.to_string());
                            });
                            row.col(|ui| {
                                ui.label(absence_to.to_string());
                            });
                            row.col(|ui| {
                                ui.label(requested_at.to_string());
                            });
                            row.col(|ui| {
                                ui.label(approved_by);
                            });
                            row.col(|ui| {
                                ui.label(approved_at.to_string());
                            });
                        });
                    }
                })
        });
    }
}

struct DateSelectorPanel;
impl Default for DateSelectorPanel {
    fn default() -> Self {
        Self
    }
}

impl DateSelectorPanel {
    fn draw(&mut self, ui: &mut Ui) {
        let id = ui.make_persistent_id("date_selector");
        let mut selector_state = ui
            .memory()
            .data
            .get_persisted::<DateSelectorState>(id)
            .unwrap_or_default();
        ComboBox::from_label("Year")
            .selected_text(format!("{}", selector_state.year))
            .show_ui(ui, |ui| {
                let current_year = Local::today().year();
                let years_count = 5;
                for y in ((current_year - years_count)..=current_year).rev() {
                    if ui
                        .selectable_value(&mut selector_state.year, y, format!("{}", y))
                        .changed()
                    {
                        ui.memory()
                            .data
                            .insert_persisted(id, selector_state.clone());
                    }
                }
            });
        ComboBox::from_label("Month")
            .selected_text(format!("{}", month_name(selector_state.month)))
            .show_ui(ui, |ui| {
                for m in 1..=12 {
                    if ui
                        .selectable_value(&mut selector_state.month, m, month_name(m))
                        .changed()
                    {
                        ui.memory()
                            .data
                            .insert_persisted(id, selector_state.clone());
                    }
                }
            });
    }
}

fn month_name(i: u32) -> &'static str {
    match i {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => panic!("Unknown month: {}", i),
    }
}

fn heading_text(text: &str) -> RichText {
    RichText::new(text).text_style(TextStyle::Heading)
}
