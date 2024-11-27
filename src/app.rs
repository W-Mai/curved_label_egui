use eframe::emath::{pos2, Rect};
use eframe::epaint::{Color32, Shape, Stroke};
use egui::{emath, Frame, Pos2, Sense, Ui, Vec2};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct MainApp {
    label: String,
    offset: f64,
    space: f64,
    height: f64,

    control_points: [Pos2; 4],

    #[serde[skip]]
    aux_stroke: Stroke,
    #[serde[skip]]
    fill: Color32,
    #[serde[skip]]
    stroke: Stroke,
}

impl Default for MainApp {
    fn default() -> Self {
        Self {
            label: "Hello World!".to_owned(),
            offset: 0.0,
            space: 10.0,
            height: 10.0,
            control_points: [
                pos2(50.0, 50.0),
                pos2(60.0, 250.0),
                pos2(200.0, 200.0),
                pos2(250.0, 50.0),
            ],
            aux_stroke: Stroke::new(1.0, Color32::RED.linear_multiply(0.25)),
            fill: Color32::from_rgb(50, 100, 150).linear_multiply(0.25),
            stroke: Stroke::new(1.0, Color32::from_rgb(25, 200, 100)),
        }
    }
}

fn compute_bezier3_point(
    t: f64,
    p0: (f64, f64),
    p1: (f64, f64),
    p2: (f64, f64),
    p3: (f64, f64),
) -> (f64, f64) {
    let x = (1.0 - t).powi(3) * p0.0
        + 3.0 * (1.0 - t).powi(2) * t * p1.0
        + 3.0 * (1.0 - t) * t.powi(2) * p2.0
        + t.powi(3) * p3.0;
    let y = (1.0 - t).powi(3) * p0.1
        + 3.0 * (1.0 - t).powi(2) * t * p1.1
        + 3.0 * (1.0 - t) * t.powi(2) * p2.1
        + t.powi(3) * p3.1;
    (x, y)
}

fn compute_bezier3_derivative(
    t: f64,
    p0: (f64, f64),
    p1: (f64, f64),
    p2: (f64, f64),
    p3: (f64, f64),
) -> (f64, f64) {
    let x = 3.0 * (1.0 - t).powi(2) * (p1.0 - p0.0)
        + 6.0 * (1.0 - t) * t * (p2.0 - p1.0)
        + 3.0 * t.powi(2) * (p3.0 - p2.0);
    let y = 3.0 * (1.0 - t).powi(2) * (p1.1 - p0.1)
        + 6.0 * (1.0 - t) * t * (p2.1 - p1.1)
        + 3.0 * t.powi(2) * (p3.1 - p2.1);
    (x, y)
}

fn calculate_delta_arc_length(d_t: f64, d_x: f64, d_y: f64) -> f64 {
    (d_x.powi(2) + d_y.powi(2)).sqrt() * d_t
}

#[allow(clippy::too_many_arguments)]
fn find_t_for_arc_length(
    mut current_len: f64,
    total_len: f64,
    mut current_t: f64,
    delta_t: f64,
    p0: (f64, f64),
    p1: (f64, f64),
    p2: (f64, f64),
    p3: (f64, f64),
) -> f64 {
    while current_len < total_len {
        current_t += delta_t;
        let b_prime_t = compute_bezier3_derivative(current_t, p0, p1, p2, p3);
        let delta_len = calculate_delta_arc_length(delta_t, b_prime_t.0, b_prime_t.1);
        current_len += delta_len;
    }
    current_t
}

fn compute_features(
    t: f64,
    p0: (f64, f64),
    p1: (f64, f64),
    p2: (f64, f64),
    p3: (f64, f64),
) -> ((f64, f64), (f64, f64), f64) {
    let b_t = compute_bezier3_point(t, p0, p1, p2, p3);
    let b_prime_t = compute_bezier3_derivative(t, p0, p1, p2, p3);
    let angle = f64::atan2(b_prime_t.1, b_prime_t.0);
    (b_t, b_prime_t, angle)
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
                ui.add(egui::Slider::new(&mut self.offset, 0.0..=1000.0).text("Offset"));
                ui.add(egui::Slider::new(&mut self.space, 1.0..=100.0).text("Space"));
                ui.add(egui::Slider::new(&mut self.height, 1.0..=100.0).text("Height"));

                ui.text_edit_singleline(&mut self.label);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            Frame::canvas(ui.style()).show(ui, |ui| {
                self.ui_content(ui);
            });

            egui::warn_if_debug_build(ui);
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

impl MainApp {
    pub fn ui_content(&mut self, ui: &mut Ui) -> egui::Response {
        let (response, painter) = ui.allocate_painter(
            Vec2::new(ui.available_width(), ui.ctx().available_rect().height()),
            Sense::hover(),
        );

        let to_screen = emath::RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.size()),
            response.rect,
        );

        let control_point_radius = 8.0;

        let control_point_shapes: Vec<Shape> = self
            .control_points
            .iter_mut()
            .enumerate()
            .map(|(i, point)| {
                let size = Vec2::splat(2.0 * control_point_radius);

                let point_in_screen = to_screen.transform_pos(*point);
                let point_rect = Rect::from_center_size(point_in_screen, size);
                let point_id = response.id.with(i);
                let point_response = ui.interact(point_rect, point_id, Sense::drag());

                *point += point_response.drag_delta();
                *point = to_screen.from().clamp(*point);

                let point_in_screen = to_screen.transform_pos(*point);
                let stroke = ui.style().interact(&point_response).fg_stroke;

                Shape::circle_stroke(point_in_screen, control_point_radius, stroke)
            })
            .collect();

        // let points_in_screen: Vec<Pos2> =
        //     self.control_points.iter().map(|p| to_screen * *p).collect();

        // let points = points_in_screen.clone().try_into().unwrap();
        // let shape = CubicBezierShape::from_points_stroke(points, true, self.fill, self.stroke);
        // painter.add(shape);
        // painter.add(PathShape::line(points_in_screen, self.aux_stroke));
        painter.extend(control_point_shapes);

        let s_i = self.space;
        let mut t_values = if self.offset != 0.0 {
            vec![]
        } else {
            vec![0.0]
        };
        let mut t = 0.0;
        let mut total_len = 0.0;
        loop {
            t = find_t_for_arc_length(
                total_len,
                total_len + s_i,
                t,
                0.001,
                (
                    self.control_points[0].x as f64,
                    self.control_points[0].y as f64,
                ),
                (
                    self.control_points[1].x as f64,
                    self.control_points[1].y as f64,
                ),
                (
                    self.control_points[2].x as f64,
                    self.control_points[2].y as f64,
                ),
                (
                    self.control_points[3].x as f64,
                    self.control_points[3].y as f64,
                ),
            );

            if t > 1.0 {
                break;
            }

            total_len += s_i;

            if total_len < self.offset {
                continue;
            }

            t_values.push(t);
        }

        for t_value in t_values.iter() {
            let (_b_t, _b_prime_t, angle) = compute_features(
                *t_value,
                (
                    self.control_points[0].x as f64,
                    self.control_points[0].y as f64,
                ),
                (
                    self.control_points[1].x as f64,
                    self.control_points[1].y as f64,
                ),
                (
                    self.control_points[2].x as f64,
                    self.control_points[2].y as f64,
                ),
                (
                    self.control_points[3].x as f64,
                    self.control_points[3].y as f64,
                ),
            );

            let start_point = to_screen.transform_pos(Pos2::from([_b_t.0 as f32, _b_t.1 as f32]));
            painter.add(Shape::circle_stroke(start_point, 1.0, self.stroke));
            painter.add(Shape::line(
                vec![
                    start_point,
                    start_point
                        + Vec2::new(
                            (self.height * angle.sin()) as f32,
                            -(self.height * angle.cos()) as f32,
                        ),
                ],
                self.stroke,
            ));
        }

        response
    }
}
