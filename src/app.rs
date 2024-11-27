use eframe::epaint::{Color32, Shape};
use egui::{Id, Ui, Widget};
use egui_plot::{PlotBounds, PlotGeometry, PlotTransform};
use std::ops::RangeInclusive;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct MainApp {
    label: String,

    p0: egui::Pos2,
    p1: egui::Pos2,
    p2: egui::Pos2,
    p3: egui::Pos2,
}

impl Default for MainApp {
    fn default() -> Self {
        Self {
            label: "Hello World!".to_owned(),
            p0: Default::default(),
            p1: Default::default(),
            p2: Default::default(),
            p3: Default::default(),
        }
    }
}

// struct PlotShape {}
//
// impl egui_plot::PlotItem for PlotShape {
//     fn shapes(&self, ui: &Ui, transform: &PlotTransform, shapes: &mut Vec<Shape>) {
//         todo!()
//     }
//
//     fn initialize(&mut self, x_range: RangeInclusive<f64>) {
//         todo!()
//     }
//
//     fn name(&self) -> &str {
//         todo!()
//     }
//
//     fn color(&self) -> Color32 {
//         todo!()
//     }
//
//     fn highlight(&mut self) {
//         todo!()
//     }
//
//     fn highlighted(&self) -> bool {
//         todo!()
//     }
//
//     fn allow_hover(&self) -> bool {
//         todo!()
//     }
//
//     fn geometry(&self) -> PlotGeometry<'_> {
//         todo!()
//     }
//
//     fn bounds(&self) -> PlotBounds {
//         todo!()
//     }
//
//     fn id(&self) -> Option<Id> {
//         todo!()
//     }
// }

impl MainApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for MainApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::SidePanel::left("Left Panel")
            .resizable(true)
            .show(ctx, |ui| {
                ui.heading("Bézier");

                ui.label("p0");
                egui::DragValue::new(&mut self.p0.x).ui(ui);
                egui::DragValue::new(&mut self.p0.y).ui(ui);

                ui.label("p1");
                egui::DragValue::new(&mut self.p1.x).ui(ui);
                egui::DragValue::new(&mut self.p1.y).ui(ui);

                ui.label("p2");
                egui::DragValue::new(&mut self.p2.x).ui(ui);
                egui::DragValue::new(&mut self.p2.y).ui(ui);

                ui.label("p3");
                egui::DragValue::new(&mut self.p3.x).ui(ui);
                egui::DragValue::new(&mut self.p3.y).ui(ui);

                ui.separator();

                ui.text_edit_singleline(&mut self.label);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                let plot = egui_plot::Plot::new("plot").data_aspect(1.0);
                // plot.show(ui, |plot_ui| {
                //     let plot_bounds = plot_ui.plot_bounds();
                //     let plot_size = plot_ui.response().rect;
                //     let scale = 1.0 / (plot_bounds.width() as f32 / plot_size.width());
                //
                //     let shape = ctx.fonts(|f| {
                //         egui::Shape::text(
                //             f,
                //             self.p0,
                //             egui::Align2::CENTER_CENTER,
                //             self.label.clone(),
                //             Default::default(),
                //             Default::default(),
                //         )
                //     });
                //
                //     plot_ui.bar_chart(egui_plot::BarChart::new(Vec::from([egui_plot::Bar::new(
                //         1.0, 1.0,
                //     )])));
                //     plot_ui.text(egui_plot::Text::new(
                //         egui_plot::PlotPoint::new(self.p0[0], self.p0[1]),
                //         egui::RichText::new(self.label.clone()).size(1.0 * scale),
                //     ));
                // });

                let mut shape = ctx.fonts(|f| {
                    egui::Shape::text(
                        f,
                        self.p0,
                        egui::Align2::CENTER_CENTER,
                        self.label.clone(),
                        Default::default(),
                        Default::default(),
                    )
                });
                shape.scale(100.0);

                ui.painter().extend([shape]);

                egui::warn_if_debug_build(ui);
            });
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
