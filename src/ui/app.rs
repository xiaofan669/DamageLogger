use std::sync::Once;
use egui::{epaint::text::{FontInsert, InsertFontFamily}, CentralPanel, Color32, Context, FontId, Frame, Label, Slider, Vec2, Window};
use windows::Win32::{System::Console::GetConsoleWindow, UI::WindowsAndMessaging::{ShowWindow, SW_HIDE, SW_RESTORE, SW_SHOW}};
use crate::ui::widgets;
use egui::TextStyle::Name;
use egui::TextStyle::Heading;
use egui::TextStyle::Monospace;
use egui::TextStyle::Body;
use egui::TextStyle::Button;
use egui::TextStyle::Small;
use egui::FontFamily::Proportional;

#[derive(Default, PartialEq)]
pub enum Unit {
    #[default]
    Turn,
    ActionValue
}

pub struct Keybind {
    pub key: egui::Key,
    pub modifiers: Option<egui::Modifiers>
}

pub struct AppState {
    pub keybind: Option<Keybind>,
    pub show_menu: bool,
    pub show_console: bool,
    
    show_windows: bool,
    show_damage_distribution: bool,
    show_damage_bars: bool,
    show_real_time_damage: bool,
    show_av_metrics: bool,
    widget_opacity: f32,
    pub graph_x_unit: Unit,
    pub text_scale: f32,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            keybind: None,
            show_menu: false,
            show_console: false,
            show_windows: false,
            show_damage_distribution: false,
            show_damage_bars: false,
            show_real_time_damage: false,
            show_av_metrics: false,
            widget_opacity: 0.15,
            graph_x_unit: Unit::Turn,
            text_scale: 1.25,
        }
    }
}

impl AppState {
    pub fn set_keybind(&mut self, key: egui::Key, modifiers: Option<egui::Modifiers>) {
        self.keybind = Some({Keybind {
            key,
            modifiers
        }});
    }
}

pub fn ui(ctx: &Context, app_state: &mut AppState) {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        let path = r"StarRail_Data\StreamingAssets\MiHoYoSDKRes\HttpServerResources\font\zh-cn.ttf";
        match std::fs::read(path) {
            Ok(font) => {
                {
                    // Start with the default fonts (we will be adding to them rather than replacing them).
                    ctx.add_font(FontInsert::new(
                        "game_font",
                        egui::FontData::from_owned(font),
                        vec![
                            InsertFontFamily {
                                family: egui::FontFamily::Proportional,
                                priority: egui::epaint::text::FontPriority::Highest,
                            },
                            InsertFontFamily {
                                family: egui::FontFamily::Monospace,
                                priority: egui::epaint::text::FontPriority::Lowest,
                            },
                        ],
                    ));                    
                }
            },
            Err(e) => log::error!("{} : Failed to load {}. Defaulting to default font", e, path),
        }

        ctx.style_mut(|style| {
            let factor = app_state.text_scale;
            style.visuals.widgets.noninteractive.fg_stroke.color = Color32::WHITE;
            style.text_styles = [
                (Heading, FontId::new(factor * 30.0, Proportional)),
                (Name("Heading2".into()), FontId::new(factor * 25.0, Proportional)),
                (Name("Context".into()), FontId::new(factor * 23.0, Proportional)),
                (Body, FontId::new(factor * 18.0, Proportional)),
                (Monospace, FontId::new(factor * 14.0, Proportional)),
                (Button, FontId::new(factor * 14.0, Proportional)),
                (Small, FontId::new(factor * 10.0, Proportional)),
            ].into();
        });    
    });

    if app_state.show_menu {
        CentralPanel::default()
        .frame(Frame {
            fill: Color32::GRAY.gamma_multiply(0.25),
            ..Default::default()
        })
        .show(ctx, |ui: &mut egui::Ui| {
            Window::new("Overlay Menu")
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .resizable(false)
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("Widget Controls");
                        
                        ui.checkbox(&mut app_state.show_console, "Show Logs");
                        ui.checkbox(&mut app_state.show_damage_distribution, "Show Damage Distribution");
                        ui.checkbox(&mut app_state.show_damage_bars, "Show Damage Bars");
                        ui.checkbox(&mut app_state.show_real_time_damage, "Show Real-Time Damage");
                        ui.checkbox(&mut app_state.show_av_metrics, "Show AV Metrics");
                        
                        ui.separator();
                        ui.label("Window Opacity");
                        ui.add(
                            Slider::new(&mut app_state.widget_opacity, 0.0..=1.0)
                                .text("")
                        );

                        ui.separator();
                        ui.label("Text Size");
                        if ui.add(Slider::new(&mut app_state.text_scale, 0.5..=3.0).text("")).changed() {
                            ctx.style_mut(|style| {
                                let factor = app_state.text_scale;
                                style.text_styles = [
                                    (Heading, FontId::new(factor * 30.0, Proportional)),
                                    (Name("Heading2".into()), FontId::new(factor * 25.0, Proportional)),
                                    (Name("Context".into()), FontId::new(factor * 23.0, Proportional)),
                                    (Body, FontId::new(factor * 18.0, Proportional)),
                                    (Monospace, FontId::new(factor * 14.0, Proportional)),
                                    (Button, FontId::new(factor * 14.0, Proportional)),
                                    (Small, FontId::new(factor * 10.0, Proportional)),
                                ].into();
                            });
                        }
                        
                        ui.separator();
                        if ui.button("Close Menu").clicked() {
                            app_state.show_menu = false;
                        }
                    });
                });
        });
    }

    if app_state.show_console {
        egui::Window::new("Log")
            .resizable(true)
            .default_height(300.0)
            .default_width(400.0)
            .min_width(200.0)
            .min_height(100.0)
            .show(ctx, |ui| {
                let available = ui.available_size();
                ui.set_min_size(available);
                ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                    egui_logger::logger_ui().show(ui);
                });
            });    
    }

    let opacity = app_state.widget_opacity.clamp(0.0, 1.0);
    let window_frame = egui::Frame::none()
        .fill(Color32::from_black_alpha((255.0 * opacity) as u8))
        .inner_margin(8.0)
        .rounding(5.0);

    let transparent_frame = egui::Frame::none()
        .inner_margin(8.0)
        .rounding(5.0);

    if app_state.show_damage_distribution {
        egui::containers::Window::new("Damage Distribution")
            .frame(transparent_frame)
            .resizable(true)
            .min_width(200.0)
            .min_height(200.0)
            .show(ctx, |ui| {
                widgets::show_damage_distribution_widget(app_state, ui);
            });
    }

    if app_state.show_damage_bars {
        egui::containers::Window::new("Damage by Character")
            .frame(window_frame)
            .resizable(true)
            .min_width(200.0)
            .min_height(200.0)
            .show(ctx, |ui| {
                widgets::show_damage_bar_widget(app_state, ui);
            });
    }

    if app_state.show_real_time_damage {
        egui::containers::Window::new("Real-Time Damage")
            .frame(window_frame)
            .resizable(true)
            .min_width(200.0)
            .min_height(200.0)
            .show(ctx, |ui| {
                widgets::show_real_time_damage_graph(app_state, ui);
            });
    }

    if app_state.show_av_metrics {
        egui::containers::Window::new("Action Value Metrics")
            .frame(window_frame)
            .resizable(true)
            .min_width(200.0)
            .min_height(150.0)
            .show(ctx, |ui| {
                widgets::show_av_metrics(app_state, ui);
            });
    }
}
