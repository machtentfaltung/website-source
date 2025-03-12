use eframe::egui::{self, Button, Layout, Pos2, RichText, Vec2};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct Application {
    show_more_window: bool,
    show_about_window: bool,
    compact: bool,
}

impl Default for Application {
    fn default() -> Self {
        Self {
            show_more_window: false,
            show_about_window: false,
            compact: false,
        }
    }
}

impl Application {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}

fn main_window_ui(app: &mut Application, ui: &mut egui::Ui) {
    ui.heading(egui::RichText::new("Hello there!").heading().strong());
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("My name is ");
        ui.label(egui::RichText::new("Matei Pralea").strong());
        ui.label(egui::RichText::new(" (matei9k).").italics());
    });
    ui.add_space(4.);
    ui.hyperlink_to(
        format!("{} github.com/matei9k", egui::special_emojis::GITHUB),
        "https://github.com/matei9k",
    );

    ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
        ui.with_layout(egui::Layout::bottom_up(egui::Align::Max), |ui| {
            let text = if app.show_more_window {
                "Show Less About Me"
            } else {
                "Show More About Me"
            };

            if ui.add_sized([150., 25.], Button::new(text)).clicked() {
                app.show_more_window = !app.show_more_window;
            }
        });
    });
}

impl eframe::App for Application {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let screen_size = ctx.input(|i| i.screen_rect());
        self.compact = if screen_size.width() <= 450. {
            true
        } else {
            false
        };

        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                if ui.button("About").clicked() {
                    self.show_about_window = !self.show_about_window;
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                    egui::widgets::global_theme_preference_buttons(ui);
                });
            })
        });

        if self.compact {
            egui::CentralPanel::default().show(ctx, |ui| main_window_ui(self, ui));
        } else {
            egui::Window::new("My Website")
                .collapsible(false)
                .resizable(false)
                .fixed_size([300., 100.])
                .show(ctx, |ui| main_window_ui(self, ui));
        }

        if self.show_more_window {
            egui::Window::new("More About Me")
                .collapsible(false)
                .resizable(false)
                .open(&mut self.show_more_window)
                .fixed_size([275., 100.])
                .show(ctx, |ui| {
                    ui.with_layout(egui::Layout::top_down_justified(egui::Align::Min), |ui| {
                        ui.label("I began my programming journey around 2020 with C# and .NET, developing various WinForms applications for fun. Since then, I have worked with languages such as JavaScript, Python, Swift, C, and Rust, creating a wide range of programs, including Discord bots, iOS apps, and desktop applications.");
                    });
                });
        }

        if self.show_about_window {
            egui::Window::new("About")
                .collapsible(false)
                .resizable(false)
                .open(&mut self.show_about_window)
                .show(ctx, |ui| {
                    ui.heading(RichText::new("My Website").strong());
                    ui.link("https://matei9k.github.io");
                    ui.separator();
                    ui.label(RichText::new("License Information").strong());
                    ui.horizontal_wrapped(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.0;
                        ui.label("This program is ");                     
                        ui.label(egui::RichText::new("Free Software").strong());
                            ui.label(": you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.");
                    });
                    ui.separator();
                    ui.hyperlink_to(
                        format!("{} Website Source Code", egui::special_emojis::GITHUB),
                        "https://github.com/matei9k/website-source",
                    );
                });
        }
    }
}
