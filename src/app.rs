#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
enum Enum {
    First,
    Second,
    Third,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,
    enabled: bool,
    visible: bool,
    boolean: bool,
    radio: Enum,
    scalar: f32,
    string: String,
    color: egui::Color32,
    animate_progress_bar: bool,

    #[serde(skip)] // This how you opt-out of serialization of a field
    date: Option<chrono::NaiveDate>,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            enabled: true,
            visible: true,
            boolean: false,
            radio: Enum::First,
            scalar: 42.0,
            string: Default::default(),
            color: egui::Color32::LIGHT_BLUE.linear_multiply(0.5),
            animate_progress_bar: false,
            date: None,
            value: 2.7,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This gives us image support:
        egui_extras::install_image_loaders(&cc.egui_ctx);

        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    fn gallery_grid_contents(&mut self, ui: &mut egui::Ui) {
        let Self {
            label: _,
            enabled: _,
            visible: _,
            boolean,
            radio,
            scalar,
            string,
            color,
            animate_progress_bar,
            date,
            value: _,
        } = self;

        ui.add(doc_link_label("Label", "label"));
        ui.label("Welcome to the widget gallery!");
        ui.end_row();

        ui.add(doc_link_label("Hyperlink", "Hyperlink"));
        use egui::special_emojis::GITHUB;
        ui.hyperlink_to(
            format!("{GITHUB} egui on GitHub"),
            "https://github.com/emilk/egui",
        );
        ui.end_row();

        ui.add(doc_link_label("TextEdit", "TextEdit"));
        ui.add(egui::TextEdit::singleline(string).hint_text("Write something here"));
        ui.end_row();

        ui.add(doc_link_label("Button", "button"));
        if ui.button("Click me!").clicked() {
            *boolean = !*boolean;
        }
        ui.end_row();

        ui.add(doc_link_label("Link", "link"));
        if ui.link("Click me!").clicked() {
            *boolean = !*boolean;
        }
        ui.end_row();

        ui.add(doc_link_label("Checkbox", "checkbox"));
        ui.checkbox(boolean, "Checkbox");
        ui.end_row();

        ui.add(doc_link_label("RadioButton", "radio"));
        ui.horizontal(|ui| {
            ui.radio_value(radio, Enum::First, "First");
            ui.radio_value(radio, Enum::Second, "Second");
            ui.radio_value(radio, Enum::Third, "Third");
        });
        ui.end_row();

        ui.add(doc_link_label("SelectableLabel", "SelectableLabel"));
        ui.horizontal(|ui| {
            ui.selectable_value(radio, Enum::First, "First");
            ui.selectable_value(radio, Enum::Second, "Second");
            ui.selectable_value(radio, Enum::Third, "Third");
        });
        ui.end_row();

        ui.add(doc_link_label("ComboBox", "ComboBox"));

        egui::ComboBox::from_label("Take your pick")
            .selected_text(format!("{radio:?}"))
            .show_ui(ui, |ui| {
                ui.style_mut().wrap = Some(false);
                ui.set_min_width(60.0);
                ui.selectable_value(radio, Enum::First, "First");
                ui.selectable_value(radio, Enum::Second, "Second");
                ui.selectable_value(radio, Enum::Third, "Third");
            });
        ui.end_row();

        ui.add(doc_link_label("Slider", "Slider"));
        ui.add(egui::Slider::new(scalar, 0.0..=360.0).suffix("°"));
        ui.end_row();

        ui.add(doc_link_label("DragValue", "DragValue"));
        ui.add(egui::DragValue::new(scalar).speed(1.0));
        ui.end_row();

        ui.add(doc_link_label("ProgressBar", "ProgressBar"));
        let progress = *scalar / 360.0;
        let progress_bar = egui::ProgressBar::new(progress)
            .show_percentage()
            .animate(*animate_progress_bar);
        *animate_progress_bar = ui
            .add(progress_bar)
            .on_hover_text("The progress bar can be animated!")
            .hovered();
        ui.end_row();

        ui.add(doc_link_label("Color picker", "color_edit"));
        ui.color_edit_button_srgba(color);
        ui.end_row();

        ui.add(doc_link_label("Image", "Image"));
        let egui_icon = egui::include_image!("../assets/icon-256.png");
        ui.add(egui::Image::new(egui_icon.clone()));
        ui.end_row();

        ui.add(doc_link_label(
            "Button with image",
            "Button::image_and_text",
        ));
        if ui
            .add(egui::Button::image_and_text(egui_icon, "Click me!"))
            .clicked()
        {
            *boolean = !*boolean;
        }
        ui.end_row();

        let date = date.get_or_insert_with(|| chrono::offset::Utc::now().date_naive());
        ui.add(doc_link_label_with_crate(
            "egui_extras",
            "DatePickerButton",
            "DatePickerButton",
        ));
        ui.add(egui_extras::DatePickerButton::new(date));
        ui.end_row();

        ui.add(doc_link_label("Separator", "separator"));
        ui.separator();
        ui.end_row();

        ui.add(doc_link_label("CollapsingHeader", "collapsing"));
        ui.collapsing("Click to see what is hidden!", |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.label("It's a ");
                ui.add(doc_link_label("Spinner", "spinner"));
                ui.add_space(4.0);
                ui.add(egui::Spinner::new());
            });
        });
        ui.end_row();

        ui.add(doc_link_label_with_crate("egui_plot", "Plot", "plot"));
        example_plot(ui);
        ui.end_row();

        ui.hyperlink_to(
            "Custom widget:",
            super::toggle_switch::url_to_file_source_code(),
        );
        ui.add(crate::toggle_switch::toggle(boolean)).on_hover_text(
            "It's easy to create your own widgets!\n\
            This toggle switch is just 15 lines of code.",
        );
        ui.end_row();
    }
}

impl eframe::App for TemplateApp {
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
            ui.heading("egui web app");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut self.label);
            });

            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                self.value += 1.0;
            }

            ui.separator();

            ui.add_enabled_ui(self.enabled, |ui| {
                ui.set_visible(self.visible);

                egui::Grid::new("my_grid")
                    .num_columns(2)
                    .spacing([40.0, 4.0])
                    .striped(true)
                    .show(ui, |ui| {
                        self.gallery_grid_contents(ui);
                    });
            });

            ui.separator();

            ui.horizontal(|ui| {
                ui.checkbox(&mut self.visible, "Visible")
                    .on_hover_text("Uncheck to hide all the widgets.");
                if self.visible {
                    ui.checkbox(&mut self.enabled, "Interactive")
                        .on_hover_text("Uncheck to inspect how the widgets look when disabled.");
                }
            });

            ui.separator();

            ui.vertical_centered(|ui| {
                let tooltip_text = "The full egui documentation.\nYou can also click the different widgets names in the left column.";
                ui.hyperlink("https://docs.rs/egui/").on_hover_text(tooltip_text);
                ui.add(egui::github_link_file!(
                    "https://github.com/wzzju/egui_app/blob/main/",
                    "Source code."
                ));
            });


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

fn example_plot(ui: &mut egui::Ui) -> egui::Response {
    use egui_plot::{Line, PlotPoints};
    let n = 128;
    let line_points: PlotPoints = (0..=n)
        .map(|i| {
            use std::f64::consts::TAU;
            let x = egui::remap(i as f64, 0.0..=n as f64, -TAU..=TAU);
            [x, x.sin()]
        })
        .collect();
    let line = Line::new(line_points);
    egui_plot::Plot::new("example_plot")
        .height(150.0)
        .show_axes(true)
        .data_aspect(1.0)
        .show(ui, |plot_ui| plot_ui.line(line))
        .response
}

fn doc_link_label<'a>(title: &'a str, search_term: &'a str) -> impl egui::Widget + 'a {
    doc_link_label_with_crate("egui", title, search_term)
}

fn doc_link_label_with_crate<'a>(
    crate_name: &'a str,
    title: &'a str,
    search_term: &'a str,
) -> impl egui::Widget + 'a {
    let label = format!("{title}:");
    let url = format!("https://docs.rs/{crate_name}?search={search_term}");
    move |ui: &mut egui::Ui| {
        ui.hyperlink_to(label, url).on_hover_ui(|ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label("Search egui docs for");
                ui.code(search_term);
            });
        })
    }
}
