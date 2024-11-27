use eframe::emath::{pos2, Rect, Rot2};
use eframe::epaint::{Color32, Rounding, Shape, Stroke};
use egui::emath::TSTransform;
use egui::{Id, ImageOptions, Ui, Vec2, Widget};
use egui_plot::{HLine, LineStyle, PlotBounds, PlotGeometry, PlotPoint, PlotTransform};
use std::ops::RangeInclusive;
use egui::epaint::TextShape;

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

struct PlotShape {
    pub(super) shape: Shape,
    pub(super) stroke: egui::Stroke,
    pub(super) name: String,
    pub(super) highlight: bool,
    pub(super) allow_hover: bool,
    pub(super) style: LineStyle,
    id: Option<Id>,
}

impl PlotShape {
    pub fn new(shape: Shape) -> Self {
        Self {
            shape,
            stroke: Stroke::new(10.0, Color32::RED),
            name: String::default(),
            highlight: false,
            allow_hover: true,
            style: LineStyle::Solid,
            id: None,
        }
    }

    #[inline]
    pub fn highlight(mut self, highlight: bool) -> Self {
        self.highlight = highlight;
        self
    }

    #[inline]
    pub fn allow_hover(mut self, hovering: bool) -> Self {
        self.allow_hover = hovering;
        self
    }

    #[inline]
    pub fn stroke(mut self, stroke: impl Into<Stroke>) -> Self {
        self.stroke = stroke.into();
        self
    }

    #[inline]
    pub fn width(mut self, width: impl Into<f32>) -> Self {
        self.stroke.width = width.into();
        self
    }

    #[inline]
    pub fn color(mut self, color: impl Into<Color32>) -> Self {
        self.stroke.color = color.into();
        self
    }

    #[inline]
    pub fn style(mut self, style: LineStyle) -> Self {
        self.style = style;
        self
    }

    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn name(mut self, name: impl ToString) -> Self {
        self.name = name.to_string();
        self
    }

    #[inline]
    pub fn id(mut self, id: Id) -> Self {
        self.id = Some(id);
        self
    }
}

impl egui_plot::PlotItem for PlotShape {
    fn shapes(&self, ui: &Ui, transform: &PlotTransform, shapes: &mut Vec<Shape>) {
        let mut shape = self.shape.clone();
        let t = transform.position_from_point(&PlotPoint::new(0.0, 0.0));
        let t2 = transform.position_from_point(&PlotPoint::new(100.0, 1.0));
        let screen_rect = egui::Rect::from([[t.x, t.y].into(), [t2.x, t2.y].into()]);

        // shape.transform(TSTransform::new(transform));
        let z = transform.bounds().width() as f32 / transform.frame().width();
        // shape.scale(1.0 / z);
        // shape.translate(Vec2::new(t.x, t.y));

        egui::paint_texture_at(
            ui.painter(),
            screen_rect,
            &ImageOptions {
                uv: Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
                bg_fill: Color32::RED,
                tint: Color32::BLACK,
                rotation: Some((Rot2::from_angle(0.0), Vec2::splat(0.5))),
                rounding: Rounding::ZERO,
            },
            &(self.shape.texture_id(), screen_rect.size()).into(),
        );

        // shapes.append(&mut Vec::from([shape]));
    }

    fn initialize(&mut self, _x_range: RangeInclusive<f64>) {}

    fn name(&self) -> &str {
        &self.name
    }

    fn color(&self) -> Color32 {
        self.stroke.color
    }

    fn highlight(&mut self) {
        self.highlight = true
    }

    fn highlighted(&self) -> bool {
        self.highlight
    }

    fn allow_hover(&self) -> bool {
        self.allow_hover
    }

    fn geometry(&self) -> PlotGeometry<'_> {
        PlotGeometry::None
    }

    fn bounds(&self) -> PlotBounds {
        let rect = self.shape.visual_bounding_rect();
        PlotBounds::from_min_max(
            [rect.min.x as f64, rect.min.y as f64],
            [rect.max.x as f64, rect.max.y as f64],
        )
    }

    fn id(&self) -> Option<Id> {
        self.id
    }
}

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
                plot.show(ui, |plot_ui| {
                    let plot_bounds = plot_ui.plot_bounds();
                    let plot_size = plot_ui.response().rect;
                    let scale = 1.0 / (plot_bounds.width() as f32 / plot_size.width());

                   
                    let mut shape = ctx.fonts(|f| {
                        egui::Shape::text(
                            f,
                            self.p0,
                            egui::Align2::CENTER_CENTER,
                            self.label.clone(),
                            Default::default(),
                            egui::Color32::RED,
                        )
                    });

                    plot_ui.add(PlotShape::new(egui::Shape::rect_filled(
                        egui::Rect::from([self.p0, self.p1]),
                        0.0,
                        egui::Color32::RED,
                    )));

                    plot_ui.add(PlotShape::new(shape));

                    // plot_ui.bar_chart(egui_plot::BarChart::new(Vec::from([egui_plot::Bar::new(
                    //     1.0, 1.0,
                    // )])));
                    // plot_ui.text(egui_plot::Text::new(
                    //     egui_plot::PlotPoint::new(self.p0[0], self.p0[1]),
                    //     egui::RichText::new(self.label.clone()).size(1.0 * scale),
                    // ));
                });

                egui::warn_if_debug_build(ui);
            });
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
