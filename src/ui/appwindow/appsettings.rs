use std::path::PathBuf;

use crate::app::RnoteApp;
use crate::compose::color::Color;
use crate::ui::appwindow::RnoteAppWindow;

use adw::prelude::*;
use gtk4::gio;

impl RnoteAppWindow {
    /// Settings binds
    pub fn setup_settings(&self) {
        let app = self.application().unwrap().downcast::<RnoteApp>().unwrap();

        // Color scheme
        self.app_settings()
            .bind("color-scheme", &app.style_manager(), "color-scheme")
            .mapping(
                |variant, _| match variant.get::<String>().unwrap().as_str() {
                    "default" => Some(adw::ColorScheme::Default.to_value()),
                    "light" => Some(adw::ColorScheme::ForceLight.to_value()),
                    "dark" => Some(adw::ColorScheme::ForceDark.to_value()),
                    _ => None,
                },
            )
            .set_mapping(|value, _| match value.get::<adw::ColorScheme>().unwrap() {
                adw::ColorScheme::ForceDark => Some(String::from("dark").to_variant()),
                adw::ColorScheme::ForceLight => Some(String::from("light").to_variant()),
                _ => Some(String::from("default").to_variant()),
            })
            .build();

        // Workspace directory
        self.app_settings()
            .bind(
                "workspace-dir",
                &self.workspacebrowser().primary_dirlist(),
                "file",
            )
            .mapping(|variant, _| {
                let path = PathBuf::from(variant.get::<String>().unwrap());
                Some(gio::File::for_path(&path).to_value())
            })
            .set_mapping(|value, _| {
                let file = value.get::<gio::File>().unwrap();

                file.path().map(|path| path.to_string_lossy().to_variant())
            })
            .build();

        // pen sounds
        self.app_settings()
            .bind("pen-sounds", self, "pen-sounds")
            .build();

        // touch drawing
        self.app_settings()
            .bind("touch-drawing", &self.canvas(), "touch-drawing")
            .build();

        // endless sheet
        self.app_settings()
            .bind("endless-sheet", &self.canvas(), "endless-sheet")
            .build();

        // format borders
        self.app_settings()
            .bind("format-borders", &self.canvas(), "format-borders")
            .build();

        // sheet margin
        self.app_settings()
            .bind("sheet-margin", &self.canvas(), "sheet-margin")
            .build();

        // pdf import width
        self.app_settings()
            .bind("pdf-import-width", &self.canvas(), "pdf-import-width")
            .build();

        // pdf import as vector image
        self.app_settings()
            .bind("pdf-import-as-vector", &self.canvas(), "pdf-import-as-vector")
            .build();

        // Marker page
        self.app_settings()
            .bind(
                "markerpage-selected-color",
                &self.penssidebar().marker_page().colorpicker(),
                "selected",
            )
            .build();

        // Brush page
        self.app_settings()
            .bind(
                "brushpage-selected-color",
                &self.penssidebar().brush_page().colorpicker(),
                "selected",
            )
            .build();

        // Shaper page
        self.app_settings()
            .bind(
                "shaperpage-selected-color",
                &self.penssidebar().shaper_page().stroke_colorpicker(),
                "selected",
            )
            .build();
        self.app_settings()
            .bind(
                "shaperpage-selected-fill",
                &self.penssidebar().shaper_page().fill_colorpicker(),
                "selected",
            )
            .build();
    }

    /// load settings that are not bound in setup_settings. Setting changes through gsettings / dconf might not be applied until app restarts
    pub fn load_settings(&self) -> Result<(), anyhow::Error> {
        let _app = self.application().unwrap().downcast::<RnoteApp>().unwrap();

        // Load latest window state
        self.load_window_size();

        // Load sheet state
        self.canvas().load_sheet_settings(&self.app_settings())?;

        // Marker page
        let colors = self
            .app_settings()
            .get::<(u32, u32, u32, u32, u32, u32, u32, u32)>("markerpage-colors");
        let colors = [
            colors.0, colors.1, colors.2, colors.3, colors.4, colors.5, colors.6, colors.7,
        ]
        .into_iter()
        .map(|color| Color::from(color))
        .collect::<Vec<Color>>();
        self.penssidebar()
            .marker_page()
            .colorpicker()
            .load_colors(&colors);

        // Brush page
        let colors = self
            .app_settings()
            .get::<(u32, u32, u32, u32, u32, u32, u32, u32)>("brushpage-colors");
        let colors = [
            colors.0, colors.1, colors.2, colors.3, colors.4, colors.5, colors.6, colors.7,
        ]
        .into_iter()
        .map(|color| Color::from(color))
        .collect::<Vec<Color>>();
        self.penssidebar()
            .brush_page()
            .colorpicker()
            .load_colors(&colors);

        // Shaper page colors
        let colors = self.app_settings().get::<(u32, u32)>("shaperpage-colors");
        let colors = [colors.0, colors.1]
            .into_iter()
            .map(|color| Color::from(color))
            .collect::<Vec<Color>>();
        self.penssidebar()
            .shaper_page()
            .stroke_colorpicker()
            .load_colors(&colors);

        // Shaper page fills
        let colors = self.app_settings().get::<(u32, u32)>("shaperpage-fills");
        let colors = [colors.0, colors.1]
            .into_iter()
            .map(|color| Color::from(color))
            .collect::<Vec<Color>>();
        self.penssidebar()
            .shaper_page()
            .fill_colorpicker()
            .load_colors(&colors);

        // Loading all state into the UI
        adw::prelude::ActionGroupExt::activate_action(self, "refresh-ui-for-sheet", None);
        Ok(())
    }

    /// Save all state that is not bound in setup_settings
    pub fn save_to_settings(&self) -> Result<(), anyhow::Error> {
        self.save_window_size()?;

        {
            // Marker page
            let colors = self
                .penssidebar()
                .marker_page()
                .colorpicker()
                .fetch_all_colors()
                .into_iter()
                .map(|color| color.to_u32())
                .collect::<Vec<u32>>();
            let colors = (
                colors[0], colors[1], colors[2], colors[3], colors[4], colors[5], colors[6],
                colors[7],
            );
            self.app_settings()
                .set_value("markerpage-colors", &colors.to_variant())?;
        }

        {
            // Brush page
            let colors = self
                .penssidebar()
                .brush_page()
                .colorpicker()
                .fetch_all_colors()
                .into_iter()
                .map(|color| color.to_u32())
                .collect::<Vec<u32>>();
            let colors = (
                colors[0], colors[1], colors[2], colors[3], colors[4], colors[5], colors[6],
                colors[7],
            );
            self.app_settings()
                .set_value("brushpage-colors", &colors.to_variant())?;
        }

        {
            // Shaper page colors
            let colors = self
                .penssidebar()
                .shaper_page()
                .stroke_colorpicker()
                .fetch_all_colors()
                .into_iter()
                .map(|color| color.to_u32())
                .collect::<Vec<u32>>();
            let colors = (colors[0], colors[1]);
            self.app_settings()
                .set_value("shaperpage-colors", &colors.to_variant())?;

            // Shaper page fills
            let fills = self
                .penssidebar()
                .shaper_page()
                .fill_colorpicker()
                .fetch_all_colors()
                .into_iter()
                .map(|color| color.to_u32())
                .collect::<Vec<u32>>();
            let fills = (fills[0], fills[1]);
            self.app_settings()
                .set_value("shaperpage-fills", &fills.to_variant())?;

            self.canvas().save_sheet_settings(&self.app_settings())?;
        }

        Ok(())
    }
}
