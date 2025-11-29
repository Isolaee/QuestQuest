use crate::rendering::CombatConfirmation;

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
	pub fn draw<F: Fn(&[f32])>(
		window_width: f32,
		window_height: f32,
		dialog_x: f32,
		dialog_y: f32,
		dialog_width: f32,
		dialog_height: f32,
		confirmation: &CombatConfirmation,
		draw_vertices: F,
	) {
		let mut vertices: Vec<f32> = Vec::new();

		// --- 1. Render semi-transparent full-screen overlay ---
		let overlay_color = [0.0, 0.0, 0.0, 0.7];
		let overlay_depth = -0.99;
		let overlay_tex_id = -2.0;
		let overlay_vertices = [
			// Triangle 1
			-1.0,  1.0, overlay_depth, 0.0, 0.0, overlay_tex_id, overlay_color[0], overlay_color[1], overlay_color[2],
			 1.0,  1.0, overlay_depth, 1.0, 0.0, overlay_tex_id, overlay_color[0], overlay_color[1], overlay_color[2],
			-1.0, -1.0, overlay_depth, 0.0, 1.0, overlay_tex_id, overlay_color[0], overlay_color[1], overlay_color[2],
			// Triangle 2
			 1.0,  1.0, overlay_depth, 1.0, 0.0, overlay_tex_id, overlay_color[0], overlay_color[1], overlay_color[2],
			 1.0, -1.0, overlay_depth, 1.0, 1.0, overlay_tex_id, overlay_color[0], overlay_color[1], overlay_color[2],
			-1.0, -1.0, overlay_depth, 0.0, 1.0, overlay_tex_id, overlay_color[0], overlay_color[1], overlay_color[2],
		];
		vertices.extend_from_slice(&overlay_vertices);

		// --- 2. Render dialog background panel ---
		let to_ndc_x = |x: f32| (x / window_width) * 2.0 - 1.0;
		let to_ndc_y = |y: f32| 1.0 - (y / window_height) * 2.0;

		let x1 = to_ndc_x(dialog_x);
		let y1 = to_ndc_y(dialog_y);
		let x2 = to_ndc_x(dialog_x + dialog_width);
		let y2 = to_ndc_y(dialog_y + dialog_height);

		let bg_color = [0.7, 0.5, 0.3, 1.0];
		let depth = -0.98;
		let tex_id = -2.0;
		vertices.extend_from_slice(&[
			// Triangle 1
			x1, y1, depth, 0.0, 0.0, tex_id, bg_color[0], bg_color[1], bg_color[2],
			x2, y1, depth, 1.0, 0.0, tex_id, bg_color[0], bg_color[1], bg_color[2],
			x1, y2, depth, 0.0, 1.0, tex_id, bg_color[0], bg_color[1], bg_color[2],
			// Triangle 2
			x2, y1, depth, 1.0, 0.0, tex_id, bg_color[0], bg_color[1], bg_color[2],
			x2, y2, depth, 1.0, 1.0, tex_id, bg_color[0], bg_color[1], bg_color[2],
			x1, y2, depth, 0.0, 1.0, tex_id, bg_color[0], bg_color[1], bg_color[2],
		]);

		// Submit all vertices for drawing
		draw_vertices(&vertices);
	}
}
