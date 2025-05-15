use crate::models::{MusicEra, MusicGenre};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    username: String,
    music_genre: MusicGenre,
    music_era: MusicEra
}

impl Default for App {
    fn default() -> Self {
        Self {
            username: "default-user".to_string(),
            music_genre: MusicGenre::Any,
            music_era: MusicEra::Any
        }
    }
}

impl App {
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

impl eframe::App for App {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Simple EGUI label");

            simple_element(ui);
        });
    }
}

fn simple_element(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing();
        ui.label("Name: ");
        ui.label("Vangerwua Tor");

        ui.separator();

        if ui.button("Click Here").clicked() {
            println!("Hey look I've been clicked");
        }
    });
}
