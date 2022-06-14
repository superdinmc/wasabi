mod keyboard;
mod keyboard_layout;
mod scene;

use egui::{style::Margin, Frame, Visuals};

use self::{keyboard::GuiKeyboard, scene::GuiRenderScene};

use super::{GuiRenderer, GuiState};

pub struct GuiWasabiWindow {
    render_scene: GuiRenderScene,
    keyboard_layout: keyboard_layout::KeyboardLayout,
    keyboard: GuiKeyboard,
}

impl GuiWasabiWindow {
    pub fn new(renderer: &mut GuiRenderer) -> GuiWasabiWindow {
        GuiWasabiWindow {
            render_scene: GuiRenderScene::new(renderer),
            keyboard_layout: keyboard_layout::KeyboardLayout::new(&Default::default()),
            keyboard: GuiKeyboard::new(),
        }
    }

    /// Defines the layout of our UI
    pub fn layout(&mut self, state: &mut GuiState) {
        let ctx = state.gui.context();

        ctx.set_visuals(Visuals::dark());

        // Render the top panel
        egui::TopBottomPanel::top("Top panel")
            .height_range(100.0..=100.0)
            .show(&ctx, |ui| ui.heading("Settings or whatever here"));

        // Calculate available space left for keyboard and notes
        // We must render notes before keyboard because the notes
        // renderer tells us the key colors
        let available = ctx.available_rect();
        let height = available.height();
        let keyboard_height = 100.0;
        let notes_height = height - keyboard_height;

        // Render the notes
        egui::TopBottomPanel::top("Note panel")
            .height_range(notes_height..=notes_height)
            .show(&ctx, |mut ui| self.render_scene.layout(state, &mut ui));

        // Render the keyboard
        egui::TopBottomPanel::top("Keyboard panel")
            .height_range(keyboard_height..=keyboard_height)
            .frame(Frame::default().margin(Margin::same(0.0)))
            .show(&ctx, |ui| {
                self.keyboard
                    .draw(ui, &self.keyboard_layout.get_view_for_keys(0, 127));
            });
    }
}
