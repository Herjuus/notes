use egui::scroll_area::State;

#[derive(Clone, serde::Deserialize, serde::Serialize)]
struct Project {
    name: String,
    content: String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    // label: String,
    // content: String,

    projects: Vec<Project>,
    current_project: Option<Project>,

    // #[serde(skip)] // This how you opt-out of serialization of a field
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            // label: "Hello World!".to_owned(),
            // content: "".to_owned(),
            projects: [].to_vec(),
            current_project: None,
        }
    }
}

impl TemplateApp {
    fn create_project(&mut self, ctx: &egui::Context) {
        let mut new_project = Project {
            name: "New Project".to_owned(),
            content: "".to_owned(),
        };

        self.projects.push(new_project);
    }

    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                {
                    ui.menu_button("File", |ui| {
                        if ui.button("New").clicked() {
                            self.create_project(ctx);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        if ui.button("Quit").clicked() {
                            _frame.close();
                        }
                    });
                    ui.add_space(16.0);
                }

            });
        });

        egui::SidePanel::left("LeftPanel")
            .resizable(true)
            .min_width(250.0)
            .max_width(500.0)
            .show(ctx, |ui| {
                for project in self.projects.iter() {
                    if ui.add_sized([ui.available_width(), 40.], egui::Button::new(&project.name)).clicked() {
                        self.current_project = Some(project.clone());
                        // self.content = project.content.clone();
                    }
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| {

            if let Some(project) = &mut self.current_project {
                ui.text_edit_singleline(&mut project.name);
            }

            if let Some(project) = &mut self.current_project {
                ui.add_sized([ui.available_width(), ui.available_height()], egui::text_edit::TextEdit::multiline(&mut project.content));
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                egui::warn_if_debug_build(ui);
            });
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}