#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::fmt;
//use image;
use eframe::{egui};
//use egui_extras::RetainedImage;
use egui::{RichText, Color32, Align, Direction, Sense, Label, FontId};
//mod backendTest;
mod file_handler;

fn main() -> Result<(), eframe::Error> {
    // Log to stdout (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(600.0, 300.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Docu-Doc",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}


//--------------------begining definitions-----------------------------------
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
enum dirSrc {
    Downloads,
    Photos,
    Custom,
}

impl fmt::Display for dirSrc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            dirSrc::Downloads => write!(f, "Downloads, "),
            dirSrc::Photos => write!(f, "Photos, "),
            dirSrc::Custom => write!(f, "Custom, "),
        }
    }
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
enum sortMethod {
    FileEnding,
    Date,
}

impl fmt::Display for sortMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            sortMethod::FileEnding => write!(f, "File Ending, "),
            sortMethod::Date => write!(f, "Date, "),
        }
    }
}


struct MyApp {
    dirSrcV: dirSrc,
    sortMethodV: sortMethod,
    labelText: String,
    openWin: bool,
    openWinHelp: bool,
    my_string: String,
    
    #[cfg_attr(feature = "serde", serde(skip))]
    texture: Option<egui::TextureHandle>,
    

}



impl Default for MyApp {
    fn default() -> Self {
        Self {
            dirSrcV: dirSrc::Downloads,
            sortMethodV: sortMethod::FileEnding,
            labelText: "Click me to see your selections!".to_string(),
            openWin: false,
            openWinHelp: false,
            my_string: Default::default(),
            texture: None,
 
        }
    }
}
//--------------------ending definitions-----------------------------------
//--------------------begining running the app-----------------------------------
impl eframe::App for MyApp {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		let my_frame = egui::containers::Frame {
              inner_margin: egui::style::Margin { left: 10., right: 10., top: 10., bottom: 10. },
              outer_margin: egui::style::Margin { left: 10., right: 10., top: 10., bottom: 10. },
              rounding: egui::Rounding { nw: 1.0, ne: 1.0, sw: 1.0, se: 1.0 },
              shadow: eframe::epaint::Shadow { extrusion: 1.0, color: Color32::from_rgb(225, 198, 153) },
              fill: Color32::from_rgb(225, 198, 153) ,
              stroke: egui::Stroke::new(2.0, Color32::BLACK),
          };
          
        
        
        egui::CentralPanel::default().frame(my_frame).show(ctx, |ui| {

            pub struct Layout {
                pub main_dir: Direction,
                pub main_wrap: bool,
                pub main_align: Align,
                pub main_justify: bool,
                pub cross_align: Align,
                pub cross_justify: bool,
            }

            #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
            pub struct WidgetGallery {
                dirSrcV: dirSrc,
                sortMethodV: sortMethod,
                
                #[cfg_attr(feature = "serde", serde(skip))]
                texture: Option<egui::TextureHandle>,
                
            }
            impl Default for WidgetGallery {
                fn default() -> Self {
                    Self {
                        dirSrcV: dirSrc::Downloads,
                        sortMethodV: sortMethod::FileEnding,
                        texture: None,

                    }
                }
            }

                    let Self {
                        dirSrcV,
                        sortMethodV,
                        labelText,
                        mut openWin,
                        mut openWinHelp,
                        my_string,
                        texture,
                    } = self;


            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
								
                ui.spacing_mut().item_spacing = egui::vec2(10.0, 50.0);
                				         
                ui.heading(RichText::new("Docu-Doc").color(Color32::from_rgb(0, 0, 0)));
                //});

                if ui.button("?").clicked() {
                    self.openWinHelp = true;
                }

            });

            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {




                ui.spacing_mut().item_spacing = egui::vec2(10.0, 50.0);

                egui::ComboBox::from_label(RichText::new("Folder Selection").color(Color32::from_rgb(0, 0, 0)) )
                            .selected_text(format!("{:?}", dirSrcV))
                            .show_ui(ui, |ui| {
                                ui.style_mut().wrap = Some(false);
                                ui.set_min_width(60.0);
                                ui.selectable_value(dirSrcV, dirSrc::Downloads, "Downloads");
                                ui.selectable_value(dirSrcV, dirSrc::Photos, "Photos");
                                ui.selectable_value(dirSrcV, dirSrc::Custom, "Custom");
                            })

                            ;


                ui.add(egui::TextEdit::singleline(my_string).hint_text("Put your custom path here!"));


                //ui.add(Label::new( self.my_string));

                //ui.add(Label::new(&self.my_string));

                egui::ComboBox::from_label(RichText::new("Sort By Selection").color(Color32::from_rgb(0, 0, 0)) )
                            .selected_text(format!("{:?}", sortMethodV))
                            .show_ui(ui, |ui| {
                                ui.style_mut().wrap = Some(false);
                                ui.set_min_width(60.0);
                                ui.selectable_value(sortMethodV, sortMethod::FileEnding, "File Ending");
                                ui.selectable_value(sortMethodV, sortMethod::Date, "Date");
                            });
            });



            ui.with_layout(egui::Layout::right_to_left(egui::Align::BOTTOM), |ui| {

                //egui::Window::new("My Window").open(status).show(ctx, |ui| {
                //    ui.label("Hello World!");
                //});

                if ui.button("Sort!").clicked() {
                    // return to main menu
                    //updateLabel(my_string, &ctx)
                    ui.add(egui::Spinner::new());
                    //Var declare
                    let mut path_choice = String::new();
                    let mut algorithm_choice = "0".to_string();
                    
                    if dirSrcV.to_string() == "Downloads, " {
						path_choice = "C:\\Users\\steve\\Downloads".trim_end().to_string(); 
					}
					if dirSrcV.to_string() == "Photos, " {
						path_choice = "C:\\Users\\steve\\Pictures".trim_end().to_string(); 
					}
					if dirSrcV.to_string() == "Custom, " {
						if my_string.to_string() == "" {
							path_choice = "C:\\Users\\steve\\Downloads".trim_end().to_string(); 
						}else{
							path_choice = my_string.trim_end().to_string();
						}
					}
                    

                    //Get directory to sort
                    //  not needed: std::io::stdin().read_line(&mut path_choice).unwrap();
                    

                    //algorithm selection
                    // EXT sort is 1 and Date sort is 2
                    println!("sortMethodV Value: {:?}", sortMethodV.to_string());
                    if sortMethodV.to_string() == "File Ending, " {
                        println!("FileEnding if success!");
                        algorithm_choice = "1".to_string();
                    }
                    if sortMethodV.to_string() == "Date, " {
                        println!("FileEnding if success!");
                        algorithm_choice = "2".to_string();
                    }
					
					println!("Path choice: {:?}", path_choice);
					println!("Algo Choice: {:?}", algorithm_choice);
					
                    //Calling the sort
                    file_handler::algorithm_select(&path_choice, path_choice.clone(), (*algorithm_choice).to_string());
                    
                    self.openWin = true;

                }


                egui::Window::new("Docu-Doc: Output").open(&mut openWin).collapsible(false).title_bar(false).resizable(false).show(ctx, |ui| {
                    ui.label("Sorting Success!");

                    if ui.button("Confirm").clicked(){
                        self.openWin = false;
                    }

                });
                
                egui::Window::new("Help").open(&mut openWinHelp).frame(my_frame).collapsible(false).title_bar(false).resizable(false).show(ctx, |ui| {
                    
                    ui.label(RichText::new("Docu-Doc Help:").color(Color32::from_rgb(0, 0, 0)));
                    
                    ui.label(RichText::new("Welcome to Docu-Doc! Where one sort a day keeps the clutter away!").color(Color32::from_rgb(0, 0, 0)));
                    
                    ui.label(RichText::new("Select a pre-set folder to sort, or input a path of your own. If you are going to input a custom path, make sure to select 'Custom' on the file selection. If you do not select 'Custom', it will use the path of whatever is selected in the menu.").color(Color32::from_rgb(0, 0, 0)));
                    
                    ui.label(RichText::new("The algorithm selection is how we sort the files in the directory you choose. If you pick 'Extention Sort' we will sort all the files in the directory by their extention at the end. If you pick 'Date Sort' then we will sort all the files within the Directory by the date they were last modified.").color(Color32::from_rgb(0, 0, 0)));
                    
                    ui.label(RichText::new("Be careful not to put in any directories you are not ok with being sorted! Since the app is under heavy development, there is currently no way to undo sorting, so please use extra Caution!").color(Color32::from_rgb(0, 0, 0)));

                    if ui.button("Confirm").clicked(){
                        self.openWinHelp = false;
                    }

                });
            });



        });
    }
}
//--------------------ending running the app-----------------------------------

//--------------------begining supporting functions-----------------------------------

fn updateLabel(label_text: &mut String, ctx: &egui::Context) {
    //label_text.clear();
    label_text.push_str("It worked!");

    //label_text.push_str(&backendTest::test(param1));
}

fn sort(mut windowBool: &mut bool, label_text: &mut String, param1: &mut String, param2: &mut String,  ctx: &egui::Context) {
    //Create a window
    //let tempBool = &mut true;
    //Update Window
    label_text.clear();
    label_text.push_str(param1);
    label_text.push_str(param2);
    //windowBool = &mut true;
    //label_text.push_str(&backendTest::test(param1));
}



//--------------------ending supporting functions-----------------------------------
