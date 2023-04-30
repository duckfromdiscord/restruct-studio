use egui_file::FileDialog;
use std::{path::PathBuf, fs};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct RestructApp {
    title: String,
    code: String,
    #[serde(skip)]
    xml_file: Option<PathBuf>,
    #[serde(skip)]
    xml_dialog: Option<FileDialog>,
}

impl Default for RestructApp {
    fn default() -> Self {
        Self {
            title: "restruct-studio"
            .into(),
            code: ""
            .into(),
            xml_file: None,
            xml_dialog: None,
        }
    }
}

impl RestructApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {

        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for RestructApp {

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self { title,
            code,
            xml_file: _,
            xml_dialog: _ } = self;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if (ui.button("Open")).clicked() {
                        let mut dialog = FileDialog::open_file(self.xml_file.clone());
                        dialog.open();
                        self.xml_dialog = Some(dialog);
                  }
                    if ui.button("Quit").clicked() {
                        frame.close();
                    }
                });
            });
        });

        if let Some(dialog) = &mut self.xml_dialog {
            if dialog.show(ctx).selected() {
                if let Some(file) = dialog.path() {
                    match restruct_lang::intoc2s::xml_to_c2s(fs::read_to_string(file).unwrap()) {
                        Ok(sheet) => {
                            *code = sheet.code.to_string() ;
                            *title = "restruct-studio - ".to_owned() + &sheet.sheet_name ;
                            frame.set_window_title(title);
                        },
                        Err(err) => {
                            panic!("{}", err);
                        }
                    }
                }
            }
    }

        egui::CentralPanel::default().show(ctx, |ui| {
            
            

            let mut theme = crate::highlighting::CodeTheme::from_memory(ui.ctx());
            ui.collapsing("Theme", |ui| {
                ui.group(|ui| {
                    theme.ui(ui);
                    theme.clone().store_in_memory(ui.ctx());
                });
            });

            let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
                let mut layout_job =
                    crate::highlighting::highlight(ui.ctx(), &theme, string, "c2s");
                layout_job.wrap.max_width = wrap_width;
                ui.fonts(|f| f.layout_job(layout_job))
            };

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add(
                    egui::TextEdit::multiline(code)
                        .font(egui::TextStyle::Monospace)
                        .code_editor()
                        .desired_rows(10)
                        .lock_focus(true)
                        .desired_width(f32::INFINITY)
                        .layouter(&mut layouter),
                );
            });
            
            egui::warn_if_debug_build(ui);
        });

    }
}
