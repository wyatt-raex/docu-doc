#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::{RichText, Color32, Align, Direction};


fn main() -> Result<(), eframe::Error> {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    //tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(600.0, 300.0)),
        ..Default::default()
    };
    eframe::run_native(
        "egui Version of Folder Manager",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
enum dirSrc {
    Downloads,
    Documents,
    Photos,
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
enum sortMethod {
    Name,
    FileEnding,
    MetaData,
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
enum dirDestination {
    Recursive,
    SelectedOnly,
}


pub struct Margin {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

struct MyApp {
    dirSrcV: dirSrc,
    sortMethodV: sortMethod,
    dirDestinationV: dirDestination,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            dirSrcV: dirSrc::Downloads,
            sortMethodV: sortMethod::Name,
            dirDestinationV: dirDestination::Recursive,
        }
    }
}


impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            pub struct Layout {
                pub main_dir: Direction,
                pub main_wrap: bool,
                pub main_align: Align,
                pub main_justify: bool,
                pub cross_align: Align,
                pub cross_justify: bool,
            }


// --------------------------- Start definition area? ------------------------------

            #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
            pub struct WidgetGallery {
                dirSrcV: dirSrc,
                sortMethodV: sortMethod,
                dirDestinationV: dirDestination,
            }
            impl Default for WidgetGallery {
                fn default() -> Self {
                    Self {
                        dirSrcV: dirSrc::Downloads,
                        sortMethodV: sortMethod::Name,
                        dirDestinationV: dirDestination::Recursive,
                    }
                }
            }
// --------------------------- End definition area? ------------------------------

// --------------------------- Start main-ish area? ------------------------------
            //impl WidgetGallery {
                //fn gallery_grid_contents(&mut self, ui: &mut egui::Ui) {
                    let Self {
                        dirSrcV,
                        sortMethodV,
                        dirDestinationV,
                    } = self;

            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                ui.spacing_mut().item_spacing = egui::vec2(10.0, 50.0);
                ui.heading("egui Version of Folder Manager");
                //});

                if ui.button("?").clicked() {
                    // display help here
                }

            });

            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {

                ui.spacing_mut().item_spacing = egui::vec2(10.0, 50.0);

                egui::ComboBox::from_label(RichText::new("Folder Selection").color(Color32::from_rgb(110, 255, 110)) )
                            .selected_text(format!("{:?}", dirSrcV))
                            .show_ui(ui, |ui| {
                                ui.style_mut().wrap = Some(false);
                                ui.set_min_width(60.0);
                                ui.selectable_value(dirSrcV, dirSrc::Downloads, "Downloads");
                                ui.selectable_value(dirSrcV, dirSrc::Documents, "Documents");
                                ui.selectable_value(dirSrcV, dirSrc::Photos, "Photos");
                            })

                            ;




                egui::ComboBox::from_label(RichText::new("Sort By Selection").color(Color32::from_rgb(110, 255, 110)) )
                            .selected_text(format!("{:?}", sortMethodV))
                            .show_ui(ui, |ui| {
                                ui.style_mut().wrap = Some(false);
                                ui.set_min_width(60.0);
                                ui.selectable_value(sortMethodV, sortMethod::Name, "Name");
                                ui.selectable_value(sortMethodV, sortMethod::FileEnding, "File Ending");
                                ui.selectable_value(sortMethodV, sortMethod::MetaData, "MetaData");
                            });
                egui::ComboBox::from_label(RichText::new("Scope of Search").color(Color32::from_rgb(110, 255, 110)) )
                            .selected_text(format!("{:?}", dirDestinationV))
                            .show_ui(ui, |ui| {
                                ui.style_mut().wrap = Some(false);
                                ui.set_min_width(60.0);
                                ui.selectable_value(dirDestinationV, dirDestination::Recursive, "Recursive");
                                ui.selectable_value(dirDestinationV, dirDestination::SelectedOnly, "Selected Folder Only");
                            });
            });



            ui.with_layout(egui::Layout::right_to_left(egui::Align::BOTTOM), |ui| {

                if ui.button("Sort!").clicked() {
                    // Connect to backend here
                    ui.add(egui::Spinner::new());
                }

                if ui.button("Cancel").clicked() {
                    // return to main menu
                }

            });



// --------------------------- End main-ish area? ------------------------------
        });
    }
}
