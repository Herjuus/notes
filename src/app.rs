use std::thread;

#[derive(Clone, serde::Deserialize, serde::Serialize)]
struct Project {
    name: String,
    content: String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    projects: Vec<Project>,
    current_id: usize,

    #[serde(skip)]
    loading: bool,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            projects: [].to_vec(),
            current_id: 0,
            loading: false,
        }
    }
}

impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    fn create_project(&mut self, ctx: &egui::Context) {
        let new_project = Project {
            name: "New Page".to_owned(),
            content: "".to_owned(),
        };

        self.projects.push(new_project);
    }

    fn delete_current_project(&mut self, ctx: &egui::Context) {
        self.projects.remove(self.current_id);
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
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                    ui.set_visible(self.loading);
                    ui.spinner();
                });
            });
        });

        egui::SidePanel::left("LeftPanel")
            .resizable(true)
            .min_width(250.0)
            .max_width(500.0)
            .show(ctx, |ui| {
                for project in self.projects.iter() {
                    if ui.add_sized([ui.available_width(), 40.], egui::Button::new(&project.name)).clicked() {
                        let index = self.projects.iter().position(|x| x.name == project.name).unwrap();
                        self.current_id = index;
                    }
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| {

            ui.horizontal(|ui| {
                if let Some(project) = &mut self.projects.get_mut(self.current_id) {
                    ui.text_edit_singleline(&mut project.name);
                }

                if let Some(project) = &mut self.projects.get_mut(self.current_id) {
                    if ui.button("Delete").clicked() {
                        self.delete_current_project(ctx);
                    }
                }
            });

            if let Some(project) = &mut self.projects.get_mut(self.current_id) {
                ui.add_sized([ui.available_width(), ui.available_height()], egui::text_edit::TextEdit::multiline(&mut project.content));
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                egui::warn_if_debug_build(ui);
            });
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
        thread::sleep(std::time::Duration::from_secs(1));
    }
}