use std::{
    fs, io::Write, path::PathBuf
};

#[derive(Default)]
pub struct ImageFish {
    files: Option<Vec<PathBuf>>,
    destination: Option<PathBuf>,
}

impl ImageFish {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}


impl eframe::App for ImageFish {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Image Fish");

            if ui.button("Select Images").clicked() {
                self.files = rfd::FileDialog::new()
                    .add_filter("image", &["jpg", "jpeg", "png"])
                    .set_directory("/")
                    .pick_files();  
            }

            if ui.button("Select Destination").clicked() {
                self.destination = rfd::FileDialog::new().pick_folder();
            }

            if ui.button("Export All").clicked() {
                let folder = self.destination.clone().unwrap();

                for file in self.files.clone().unwrap() {
                    let filename = rfd::FileHandle::from(file.clone()).file_name();
                    let file = img_to_64_px(&file).unwrap();

                    raster::save(&file, &folder.join(filename).to_str().unwrap()).unwrap();
                }
            }
        });
    }
}

fn img_to_64_px(img: &PathBuf) -> Result<raster::Image, raster::error::RasterError> {
    let mut i = raster::open(img.to_str().unwrap())?;

    if i.width <= i.height {
        raster::editor::resize(&mut i, 64, 64, raster::ResizeMode::ExactWidth)?;
    } else {
        raster::editor::resize(&mut i, 64, 64, raster::ResizeMode::ExactHeight)?;
    }

    Ok(i)
}
