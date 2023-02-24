use bevy::prelude::*;
use crate::prelude::*;
use bevy_inspector_egui::{
    bevy_inspector::hierarchy::SelectedEntities, DefaultInspectorConfigPlugin, egui,
};


pub fn update_hierachy(
    world: &mut World,
    mut selected_entities: Local<SelectedEntities>,
    mut inactive: Local<bool>,
) {
    let mut egui_context = world.resource_mut::<bevy_egui::EguiContext>().ctx_mut().clone();
    
    egui::SidePanel::left("UI").show(&egui_context, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            // equivalent to `WorldInspectorPlugin`
            bevy_inspector_egui::bevy_inspector::ui_for_world(world, ui);

            egui::CollapsingHeader::new("Materials").show(ui, |ui| {
                bevy_inspector_egui::bevy_inspector::ui_for_assets::<StandardMaterial>(world, ui);
            });

        });
        
        if ui.add(egui::Button::new("Save")).clicked() {
            warn!("todo: save");        
        }
    });
}