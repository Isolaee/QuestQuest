use super::text_renderer::TextRenderer;
use crate::rendering::{CombatConfirmation, CombatLogDisplay};

pub struct CombatPanel;

impl CombatPanel {
    /// Draws the combat confirmation dialog, including overlay and background panel.
    ///
    /// # Arguments
    /// * `window_width` - Width of the window in pixels
    /// * `window_height` - Height of the window in pixels
    /// * `dialog_x`, `dialog_y` - Top-left position of the dialog
    /// * `dialog_width`, `dialog_height` - Size of the dialog
    /// * `confirmation` - Combat confirmation data
    /// * `draw_vertices` - Closure to submit the generated vertices for drawing
    #[allow(clippy::too_many_arguments)]
    pub fn draw<F: Fn(&[f32])>(
        window_width: f32,
        window_height: f32,
        dialog_x: f32,
        dialog_y: f32,
        dialog_width: f32,
        dialog_height: f32,
        _confirmation: &CombatConfirmation,
        draw_vertices: F,
    ) {
        let mut vertices: Vec<f32> = Vec::new();

        // Convert window coordinates to NDC
        let to_ndc_x = |x: f32| (x / window_width) * 2.0 - 1.0;
        let to_ndc_y = |y: f32| 1.0 - (y / window_height) * 2.0;

        let depth_overlay = -0.99;
        let depth_panel = -0.98;
        let depth_border = -0.97;
        let tex_id = -2.0;

        // Helper to push a rectangle (two triangles) into vertex list.
        // Vertex format: x_ndc, y_ndc, depth, u, v, tex_id, r, g, b
        let mut push_rect = |x: f32, y: f32, w: f32, h: f32, depth: f32, color: [f32; 3]| {
            let x1 = to_ndc_x(x);
            let y1 = to_ndc_y(y);
            let x2 = to_ndc_x(x + w);
            let y2 = to_ndc_y(y + h);
            vertices.extend_from_slice(&[
                x1, y1, depth, 0.0, 0.0, tex_id, color[0], color[1], color[2], x2, y1, depth, 1.0,
                0.0, tex_id, color[0], color[1], color[2], x1, y2, depth, 0.0, 1.0, tex_id,
                color[0], color[1], color[2], x2, y1, depth, 1.0, 0.0, tex_id, color[0], color[1],
                color[2], x2, y2, depth, 1.0, 1.0, tex_id, color[0], color[1], color[2], x1, y2,
                depth, 0.0, 1.0, tex_id, color[0], color[1], color[2],
            ]);
        };

        // 1) Full-screen overlay (to dim the game scene behind dialog)
        let overlay_color = [0.05, 0.04, 0.03]; // dark tint
        push_rect(
            0.0,
            0.0,
            window_width,
            window_height,
            depth_overlay,
            overlay_color,
        );

        // 2) Dialog background panel (styled similar to EncyclopediaPanel)
        let panel_color = [0.45, 0.38, 0.30];
        push_rect(
            dialog_x,
            dialog_y,
            dialog_width,
            dialog_height,
            depth_panel,
            panel_color,
        );

        // 3) Panel border (drawn as four thin rectangles around the dialog)
        let border_color = [0.25, 0.18, 0.10];
        let border_w = 4.0_f32; // border thickness in pixels
                                // Top
        push_rect(
            dialog_x,
            dialog_y,
            dialog_width,
            border_w,
            depth_border,
            border_color,
        );
        // Bottom
        push_rect(
            dialog_x,
            dialog_y + dialog_height - border_w,
            dialog_width,
            border_w,
            depth_border,
            border_color,
        );
        // Left
        push_rect(
            dialog_x,
            dialog_y,
            border_w,
            dialog_height,
            depth_border,
            border_color,
        );
        // Right
        push_rect(
            dialog_x + dialog_width - border_w,
            dialog_y,
            border_w,
            dialog_height,
            depth_border,
            border_color,
        );

        // 4) Dialog content background (slightly lighter inner area)
        let inner_margin = 8.0_f32;
        let inner_color = [0.7, 0.5, 0.3];
        push_rect(
            dialog_x + inner_margin,
            dialog_y + inner_margin,
            dialog_width - 2.0 * inner_margin,
            dialog_height - 2.0 * inner_margin,
            depth_panel + 0.0001,
            inner_color,
        );

        // Submit all vertices for drawing
        draw_vertices(&vertices);
    }

    /// Draws the combat dialog and renders its text using the provided `TextRenderer`.
    /// This combines the quad geometry with the text layout so the whole dialog
    /// is implemented inside the UI module.
    #[allow(clippy::too_many_arguments)]
    pub fn draw_with_text<F: Fn(&[f32])>(
        window_width: f32,
        window_height: f32,
        dialog_x: f32,
        dialog_y: f32,
        dialog_width: f32,
        dialog_height: f32,
        confirmation: &CombatConfirmation,
        combat_display: &CombatLogDisplay,
        text_renderer: &mut TextRenderer,
        draw_vertices: F,
    ) {
        // First draw the background, border and inner area
        Self::draw(
            window_width,
            window_height,
            dialog_x,
            dialog_y,
            dialog_width,
            dialog_height,
            confirmation,
            draw_vertices,
        );

        // Now render all text elements inside the dialog using the provided TextRenderer
        let window_w = window_width;
        let window_h = window_height;

        // Title
        let title = "COMBAT!";
        let title_size = 30.0;
        let title_x = dialog_x + (dialog_width - title.len() as f32 * title_size * 0.6) / 2.0;
        let title_y = dialog_y + 10.0;
        text_renderer.render_text(
            title,
            title_x,
            title_y,
            title_size,
            [1.0, 0.9, 0.4, 1.0], // Gold color
            window_w,
            window_h,
        );

        // Attacker stats (left panel)
        let title_height = 50.0;
        let sprite_area_height = 120.0;
        let attacker_x = dialog_x + 40.0;
        let mut attacker_y = dialog_y + title_height + sprite_area_height + 20.0;
        let text_size = 16.0;
        let line_height = 25.0;

        text_renderer.render_text(
            "ATTACKER",
            attacker_x,
            attacker_y,
            18.0,
            [0.6, 0.8, 1.0, 1.0], // Light blue
            window_w,
            window_h,
        );
        attacker_y += line_height + 5.0;

        text_renderer.render_text(
            &confirmation.attacker_name,
            attacker_x,
            attacker_y,
            text_size,
            [1.0, 1.0, 1.0, 1.0],
            window_w,
            window_h,
        );
        attacker_y += line_height;

        let hp_text = format!(
            "HP: {}/{}",
            confirmation.attacker_hp, confirmation.attacker_max_hp
        );
        text_renderer.render_text(
            &hp_text,
            attacker_x,
            attacker_y,
            text_size,
            [1.0, 1.0, 1.0, 1.0],
            window_w,
            window_h,
        );
        attacker_y += line_height;

        let atk_text = format!("ATK: {}", confirmation.attacker_attack);
        text_renderer.render_text(
            &atk_text,
            attacker_x,
            attacker_y,
            text_size,
            [1.0, 1.0, 1.0, 1.0],
            window_w,
            window_h,
        );
        attacker_y += line_height;

        let def_text = format!("DEF: {}", confirmation.attacker_defense);
        text_renderer.render_text(
            &def_text,
            attacker_x,
            attacker_y,
            text_size,
            [1.0, 1.0, 1.0, 1.0],
            window_w,
            window_h,
        );
        attacker_y += line_height;

        let atkr_text = format!("{}/round", confirmation.attacker_attacks_per_round);
        text_renderer.render_text(
            &atkr_text,
            attacker_x,
            attacker_y,
            text_size,
            [1.0, 1.0, 1.0, 1.0],
            window_w,
            window_h,
        );

        // Defender stats (right panel)
        let defender_x = dialog_x + dialog_width / 2.0 + 40.0;
        let mut defender_y = dialog_y + title_height + sprite_area_height + 20.0;

        text_renderer.render_text(
            "DEFENDER",
            defender_x,
            defender_y,
            18.0,
            [1.0, 0.7, 0.7, 1.0], // Light red
            window_w,
            window_h,
        );
        defender_y += line_height + 5.0;

        text_renderer.render_text(
            &confirmation.defender_name,
            defender_x,
            defender_y,
            text_size,
            [1.0, 1.0, 1.0, 1.0],
            window_w,
            window_h,
        );
        defender_y += line_height;

        let hp_text = format!(
            "HP: {}/{}",
            confirmation.defender_hp, confirmation.defender_max_hp
        );
        text_renderer.render_text(
            &hp_text,
            defender_x,
            defender_y,
            text_size,
            [1.0, 1.0, 1.0, 1.0],
            window_w,
            window_h,
        );
        defender_y += line_height;

        let atk_text = format!("ATK: {}", confirmation.defender_attack);
        text_renderer.render_text(
            &atk_text,
            defender_x,
            defender_y,
            text_size,
            [1.0, 1.0, 1.0, 1.0],
            window_w,
            window_h,
        );
        defender_y += line_height;

        let def_text = format!("DEF: {}", confirmation.defender_defense);
        text_renderer.render_text(
            &def_text,
            defender_x,
            defender_y,
            text_size,
            [1.0, 1.0, 1.0, 1.0],
            window_w,
            window_h,
        );
        defender_y += line_height;

        let atkr_text = format!("{}/round", confirmation.defender_attacks_per_round);
        text_renderer.render_text(
            &atkr_text,
            defender_x,
            defender_y,
            text_size,
            [1.0, 1.0, 1.0, 1.0],
            window_w,
            window_h,
        );

        // Attack option labels for attacker (left column)
        if !confirmation.attacker_attacks.is_empty() {
            let (dialog_x, dialog_y) = combat_display.position;
            let (dialog_width, _dialog_height) = combat_display.size;
            let attack_box_height = 30.0;
            let attack_box_spacing = 5.0;
            let title_height = 50.0;
            let sprite_area_height = 120.0;
            let text_line_height = 25.0;
            let panel_padding = 15.0;
            let panel_height = text_line_height * 6.0 + panel_padding * 2.0;
            let panel_y = dialog_y + title_height + sprite_area_height + 10.0;
            let attack_section_y = panel_y + panel_height + 10.0;
            let panel_margin = 30.0;
            let panel_spacing = 20.0;
            let _panel_width = (dialog_width - 2.0 * panel_margin - panel_spacing) / 2.0;
            let attacker_x = dialog_x + panel_margin;

            for (i, attack) in confirmation.attacker_attacks.iter().enumerate() {
                let attack_y =
                    attack_section_y + (i as f32) * (attack_box_height + attack_box_spacing);
                let attack_x = attacker_x + 10.0;
                let attack_text_y = attack_y + (attack_box_height - 14.0) / 2.0 + 5.0;

                // Check if this attack is selected
                let is_selected = combat_display.selected_attack_index == Some(i);

                // Attack name and damage combined - brighter color if selected
                let attack_text = format!("{} ({}x{})", attack.name, attack.damage, attack.range);
                let text_color = if is_selected {
                    [1.0, 1.0, 1.0, 1.0]
                } else {
                    [1.0, 1.0, 0.8, 1.0]
                };

                text_renderer.render_text(
                    &attack_text,
                    attack_x,
                    attack_text_y,
                    14.0,
                    text_color,
                    window_w,
                    window_h,
                );
            }
        }

        // Attack option labels for defender (right column)
        if !confirmation.defender_attacks.is_empty() {
            let (dialog_x, dialog_y) = combat_display.position;
            let (dialog_width, _dialog_height) = combat_display.size;
            let attack_box_height = 30.0;
            let attack_box_spacing = 5.0;
            let title_height = 50.0;
            let sprite_area_height = 120.0;
            let text_line_height = 25.0;
            let panel_padding = 15.0;
            let panel_height = text_line_height * 6.0 + panel_padding * 2.0;
            let panel_y = dialog_y + title_height + sprite_area_height + 10.0;
            let attack_section_y = panel_y + panel_height + 10.0;
            let panel_margin = 30.0;
            let panel_spacing = 20.0;
            let panel_width = (dialog_width - 2.0 * panel_margin - panel_spacing) / 2.0;
            let defender_x = dialog_x + panel_margin + panel_width + panel_spacing;

            for (_i, attack) in confirmation.defender_attacks.iter().enumerate() {
                let attack_y =
                    attack_section_y + (_i as f32) * (attack_box_height + attack_box_spacing);
                let attack_x = defender_x + 10.0;
                let attack_text_y = attack_y + (attack_box_height - 14.0) / 2.0 + 5.0;

                let attack_text = format!("{} ({}x{})", attack.name, attack.damage, attack.range);
                text_renderer.render_text(
                    &attack_text,
                    attack_x,
                    attack_text_y,
                    14.0,
                    [1.0, 1.0, 0.8, 1.0],
                    window_w,
                    window_h,
                );
            }
        }

        // Button labels
        let ok_btn = &combat_display.ok_button;
        let ok_label_x = ok_btn.position.0 + (ok_btn.size.0 - 2.0 * 12.0 * 0.6) / 2.0; // Center "OK"
        let ok_label_y = ok_btn.position.1 + (ok_btn.size.1 - 20.0) / 2.0 + 5.0;
        text_renderer.render_text(
            "OK",
            ok_label_x,
            ok_label_y,
            20.0,
            [1.0, 1.0, 1.0, 1.0],
            window_w,
            window_h,
        );

        let cancel_btn = &combat_display.cancel_button;
        let cancel_label_x = cancel_btn.position.0 + (cancel_btn.size.0 - 6.0 * 12.0 * 0.6) / 2.0; // Center "Cancel"
        let cancel_label_y = cancel_btn.position.1 + (cancel_btn.size.1 - 20.0) / 2.0 + 5.0;
        text_renderer.render_text(
            "Cancel",
            cancel_label_x,
            cancel_label_y,
            20.0,
            [1.0, 1.0, 1.0, 1.0],
            window_w,
            window_h,
        );
    }
}

impl CombatPanel {
    /// Render the combat layer: either the pending confirmation dialog
    /// (with geometry + text) or the combat log entries.
    ///
    /// `draw_vertices` is used to submit any generated quad vertices to the
    /// renderer's VBO (closure will perform the buffer upload + draw).
    pub fn render_layer<F: Fn(&[f32])>(
        window_width: f32,
        window_height: f32,
        combat_display: &crate::rendering::CombatLogDisplay,
        text_renderer: &mut TextRenderer,
        draw_vertices: F,
    ) {
        if !combat_display.active {
            return;
        }

        if let Some(ref confirmation) = combat_display.pending_combat {
            // Draw the confirmation dialog (geometry + text)
            Self::draw_with_text(
                window_width,
                window_height,
                combat_display.position.0,
                combat_display.position.1,
                combat_display.size.0,
                combat_display.size.1,
                confirmation,
                combat_display,
                text_renderer,
                draw_vertices,
            );
        } else {
            // Render the combat log entries simple list
            let (panel_x, panel_y) = combat_display.position;
            let mut y = panel_y + 10.0;
            let line_height = 20.0;
            let text_size = 16.0;

            for entry in &combat_display.entries {
                text_renderer.render_text(
                    &entry.message,
                    panel_x + 10.0,
                    y,
                    text_size,
                    [1.0, 1.0, 1.0, 1.0],
                    window_width,
                    window_height,
                );
                y += line_height;
            }
        }
    }
}
