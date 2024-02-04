use egui_plot::{Corner, Legend, Line, Plot, PlotPoints};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize, Default, PartialEq)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct PlotApp {
    #[cfg(feature = "chrono")]
    #[serde(skip)] // This how you opt-out of serialization of a field
    date: Option<chrono::NaiveDate>,
    #[serde(skip)] // This how you opt-out of serialization of a field
    config: Legend,
}

impl PlotApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    fn draw_contents(&mut self, ui: &mut egui::Ui) {
        let Self {
            #[cfg(feature = "chrono")]
            date,
            config: _,
        } = self;

        #[cfg(feature = "chrono")]
        {
            let date = date.get_or_insert_with(|| chrono::offset::Utc::now().date_naive());
            ui.add(egui_extras::DatePickerButton::new(date));
            ui.separator();
        }

        self.plot_curve(ui);
    }

    fn plot_curve(&mut self, ui: &mut egui::Ui) -> egui::Response {
        let Self {
            #[cfg(feature = "chrono")]
                date: _,
            config,
        } = self;

        egui::Grid::new("settings").show(ui, |ui| {
            ui.label("Text style:");
            ui.horizontal(|ui| {
                let all_text_styles = ui.style().text_styles();
                for style in all_text_styles {
                    ui.selectable_value(&mut config.text_style, style.clone(), style.to_string());
                }
            });
            ui.end_row();

            ui.label("Position:");
            ui.horizontal(|ui| {
                Corner::all().for_each(|position| {
                    ui.selectable_value(&mut config.position, position, format!("{position:?}"));
                });
            });
            ui.end_row();

            ui.label("Opacity:");
            ui.add(
                egui::DragValue::new(&mut config.background_alpha)
                    .speed(0.02)
                    .clamp_range(0.0..=1.0),
            );
            ui.end_row();
        });
        let legend_plot = Plot::new("legend_demo")
            .y_axis_width(2)
            .legend(config.clone())
            .data_aspect(1.0);
        legend_plot
            .show(ui, |plot_ui| {
                plot_ui.line(Self::line_with_slope(0.5).name("lines"));
                plot_ui.line(Self::line_with_slope(1.0).name("lines"));
                plot_ui.line(Self::line_with_slope(2.0).name("lines"));
                plot_ui.line(Self::sin().name("sin(x)"));
                plot_ui.line(Self::cos().name("cos(x)"));
            })
            .response
    }

    fn line_with_slope(slope: f64) -> Line {
        Line::new(PlotPoints::from_explicit_callback(
            move |x| slope * x,
            ..,
            100,
        ))
    }

    fn sin() -> Line {
        Line::new(PlotPoints::from_explicit_callback(
            move |x| x.sin(),
            ..,
            100,
        ))
    }

    fn cos() -> Line {
        Line::new(PlotPoints::from_explicit_callback(
            move |x| x.cos(),
            ..,
            100,
        ))
    }
}

impl eframe::App for PlotApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Plot App");

            self.draw_contents(ui);

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
