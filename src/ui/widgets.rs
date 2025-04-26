use egui::{ahash::HashMap, Stroke, Ui, TextStyle};
use egui_plot::{Legend, Plot, PlotPoints, Polygon, BarChart, Bar, Line};
use crate::ui::app::Unit;

use crate::{battle::BattleContext, models::misc::Avatar};

use super::{app::App, helpers};

pub struct PieSegment {
    pub points: Vec<[f64; 2]>,
    pub value: f64,
}

impl App {
    pub fn show_damage_distribution_widget(&mut self, ui: &mut Ui) {
        let battle_context = BattleContext::get_instance();
        let available = ui.available_size();
        Plot::new("damage_pie")
            .legend(Legend::default()
                .position(egui_plot::Corner::RightTop)
                .text_style(TextStyle::Small))
            .height(available.y)
            .width(available.x)
            .data_aspect(1.0)
            .clamp_grid(true)
            .show_grid(false)
            .show_background(false)
            .show_axes([false; 2])
            .allow_drag(false)
            .allow_zoom(false)
            .allow_scroll(false)
            .show(ui, |plot_ui: &mut egui_plot::PlotUi<'_>| {
                let total_damage = battle_context.total_damage as f64;
                if total_damage > 0.0 {
                    let segments =
                        create_pie_segments(&battle_context.real_time_damages, &battle_context.lineup);
                    for (avatar, segment, i) in segments {
                        let color = helpers::get_character_color(i);
                        let percentage = segment.value / total_damage * 100.0;
    
                        let plot_points = PlotPoints::new(segment.points);
                        let polygon = Polygon::new(plot_points)
                            .stroke(Stroke::new(1.5, color))
                            .name(format!(
                                "{}: {:.1}% ({} dmg)",
                                avatar.name,
                                percentage,
                                helpers::format_damage(segment.value)
                            ));
    
                        plot_ui.polygon(polygon);
                    }
                }
        });
    }
    
    pub fn show_damage_bar_widget(&mut self, ui: &mut Ui) {
        let battle_context = BattleContext::get_instance();
        let available = ui.available_size();
        Plot::new("damage_bars")
            .legend(Legend::default())
            .height(available.y)
            .width(available.x)
            .allow_drag(false)
            .allow_zoom(false)
            .allow_scroll(false)
            .show_background(false)
            .y_axis_formatter(|y, _| helpers::format_damage(y.value))
            .x_axis_formatter(|x, _| {
                let index = x.value.floor() as usize;
                battle_context.lineup.get(index)
                    .map(|avatar| avatar.name.clone())
                    .unwrap_or_default()
            })
            .show(ui, |plot_ui| {
                let bars_data = create_bar_data(&battle_context);
                let bars: Vec<Bar> = bars_data
                    .iter()
                    .enumerate()
                    .map(|(pos, (avatar, value, color_idx))| {
                        Bar::new(pos as f64, *value)
                            .name(&avatar.name)
                            .fill(helpers::get_character_color(*color_idx))
                            .width(0.7)
                    })
                    .collect();
    
                plot_ui.bar_chart(BarChart::new(bars));
            });
    }
    
    pub fn show_turn_damage_plot(&mut self, ui: &mut Ui) {
        let battle_context = BattleContext::get_instance();
        let available = ui.available_size();
        Plot::new("turn_damage_plot")
            .legend(Legend::default()
                .position(egui_plot::Corner::RightTop)
                .text_style(TextStyle::Small))
            .height(available.y)
            .width(available.x)
            .include_y(0.0)
            .x_axis_label("Turn")
            .y_axis_label("Damage")
            .y_axis_formatter(|y, _| helpers::format_damage(y.value))
            .show(ui, |plot_ui| {
                for (i, avatar) in battle_context.lineup.iter().enumerate() {
                    let color = helpers::get_character_color(i);
                    let points = battle_context.turn_history
                        .iter()
                        .enumerate()
                        .map(|(turn_idx, turn)| {
                            [turn_idx as f64 + 1.0, turn.avatars_turn_damage[i]]
                        })
                        .collect::<Vec<[f64; 2]>>();
    
                    if !points.is_empty() {
                        plot_ui.line(
                            Line::new(PlotPoints::from(points))
                                .name(&avatar.name)
                                .color(color)
                                .width(2.0)
                        );
                    }
                }
            });
    }
    
    pub fn show_av_damage_plot(&mut self, ui: &mut Ui) {
        let battle_context = BattleContext::get_instance();
        let available = ui.available_size();
        Plot::new("av_damage_plot")
            .legend(Legend::default()
                .position(egui_plot::Corner::RightTop)
                .text_style(TextStyle::Small))
            .height(available.y)
            .width(available.x)
            .include_y(0.0) 
            .x_axis_label("Action Value")
            .y_axis_label("Damage")
            .y_axis_formatter(|y, _| helpers::format_damage(y.value))
            .show(ui, |plot_ui| {
                for (i, avatar) in battle_context.lineup.iter().enumerate() {
                    let color = helpers::get_character_color(i);
                    let points = battle_context.av_history
                        .iter()
                        .map(|turn| [turn.action_value, turn.avatars_turn_damage[i]])
                        .collect::<Vec<[f64; 2]>>();
    
                    if !points.is_empty() {
                        plot_ui.line(
                            Line::new(PlotPoints::from(points))
                                .name(&avatar.name)
                                .color(color)
                                .width(2.0)
                        );
                    }
                }
            });
    }
    
    pub fn show_real_time_damage_graph(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.radio_value(&mut self.graph_x_unit, Unit::Turn, "Turn");
                ui.radio_value(&mut self.graph_x_unit, Unit::ActionValue, "Action Value");
            });
            ui.add_space(8.0);
            
            match self.graph_x_unit {
                Unit::Turn => self.show_turn_damage_plot(ui),
                Unit::ActionValue => self.show_av_damage_plot(ui),
            }
        });
    }
    
    pub fn show_av_metrics(&mut self, ui: &mut Ui) {
        let battle_context = BattleContext::get_instance();
        ui.label("Current Turn");
        ui.horizontal(|ui| {
            ui.label("AV:");
            ui.label(format!("{:.2}", battle_context.current_turn_info.action_value));
        });
        ui.horizontal(|ui| {
            ui.label("Total Damage:");
            ui.label(helpers::format_damage(battle_context.total_damage));
        });
        ui.horizontal(|ui| {
            ui.label("DpAV:");
            if battle_context.current_turn_info.action_value > 0.0 {
                ui.label(format!("{:.2}", battle_context.total_damage / battle_context.current_turn_info.action_value));
            } else {
                ui.label("0.00");
            }
        });
    }
    
}

fn create_bar_data(battle_context: &BattleContext) -> Vec<(&Avatar, f64, usize)> {        
    let total_damage: Vec<f64> = battle_context.turn_history.iter()
        .flat_map(|turn| turn.avatars_turn_damage.iter())
        .copied()
        .collect();

    battle_context.lineup.iter()
        .enumerate()
        .map(|(i, avatar)| {
            let damage = total_damage
                .chunks(battle_context.lineup.len())
                .map(|chunk| chunk.get(i).copied().unwrap_or(0.0))
                .sum::<f64>();
            (avatar, damage, i)
        })
        .collect()
}

fn create_pie_segments(real_time_damages: &Vec<f64>, avatars: &Vec<Avatar>) -> Vec<(Avatar, PieSegment, usize)> {
    let total = real_time_damages.into_iter().sum::<f64>();
    let mut segments = Vec::new();
    let mut start_angle = -std::f64::consts::FRAC_PI_2; 

    for (i, name) in avatars.iter().enumerate() {
        let damage = real_time_damages[i];
        let fraction = damage as f64 / total;
        let angle = fraction * std::f64::consts::TAU;
        let end_angle = start_angle + angle;

        segments.push((
            name.clone(),
            PieSegment {
                points: create_pie_slice(start_angle, end_angle),
                value: damage as f64,
            },
            i
        ));

        start_angle = end_angle;
    }

    segments
}    

fn create_pie_slice(start_angle: f64, end_angle: f64) -> Vec<[f64; 2]> {
    let center = [0.0, 0.0];
    let radius = 0.8; 
    let mut points = vec![center];
    
    let steps = 50;
    let p = (end_angle - start_angle)/(steps as f64);
    for i in 0..=steps {
        let angle = start_angle + p*i as f64;
        let (sin, cos) = angle.sin_cos();
        points.push([cos * radius, sin * radius]);
    }
    points.push(center);
    
    points
}

