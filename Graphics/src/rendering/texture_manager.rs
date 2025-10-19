use crate::core::SpriteType;
use gl::types::*;
use std::collections::HashMap;

pub struct TextureManager {
    textures: HashMap<SpriteType, GLuint>,
}

impl TextureManager {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }

    /// Load all terrain sprites
    ///
    /// # Safety
    /// Must be called with a valid OpenGL context. All OpenGL texture operations
    /// are unsafe and require proper context management.
    pub unsafe fn load_terrain_sprites(&mut self) -> Result<(), String> {
        // Load each terrain sprite
        for sprite_type in SpriteType::all_terrain() {
            if let Some(path) = sprite_type.get_texture_path() {
                // Try multiple possible paths
                let paths_to_try = [
                    path.to_string(),
                    format!("Graphics/{}", path),
                    format!("C:/Users/eero/Documents/QuestQuest/Graphics/{}", path),
                ];

                let mut last_error = String::new();
                let mut texture_id = None;

                for attempt_path in &paths_to_try {
                    match self.load_texture_from_file(attempt_path) {
                        Ok(id) => {
                            texture_id = Some(id);
                            break;
                        }
                        Err(e) => {
                            last_error = e;
                        }
                    }
                }

                let texture_id = texture_id.ok_or_else(|| {
                    format!(
                        "Failed to load {} from any path. Last error: {}",
                        path, last_error
                    )
                })?;

                self.textures.insert(sprite_type, texture_id);
            }
        }
        Ok(())
    }

    /// Load all item sprites
    ///
    /// # Safety
    /// Must be called with a valid OpenGL context. All OpenGL texture operations
    /// are unsafe and require proper context management.
    pub unsafe fn load_item_sprites(&mut self) -> Result<(), String> {
        // Load item sprites (currently just Item/sword)
        let item_sprites = [SpriteType::Item];

        for sprite_type in item_sprites {
            if let Some(path) = sprite_type.get_texture_path() {
                // Try multiple possible paths
                let paths_to_try = [
                    path.to_string(),
                    format!("Graphics/{}", path),
                    format!("C:/Users/eero/Documents/QuestQuest/Graphics/{}", path),
                ];

                let mut last_error = String::new();
                let mut texture_id = None;

                for attempt_path in &paths_to_try {
                    match self.load_texture_from_file(attempt_path) {
                        Ok(id) => {
                            texture_id = Some(id);
                            break;
                        }
                        Err(e) => {
                            last_error = e;
                        }
                    }
                }

                let texture_id = texture_id.ok_or_else(|| {
                    format!(
                        "Failed to load {} from any path. Last error: {}",
                        path, last_error
                    )
                })?;

                self.textures.insert(sprite_type, texture_id);
            }
        }
        Ok(())
    }

    /// Load a single texture from file
    unsafe fn load_texture_from_file(&self, path: &str) -> Result<GLuint, String> {
        // Load image using the image crate
        let img = image::open(path)
            .map_err(|e| format!("Failed to load image {}: {}", path, e))?
            .to_rgba8();

        let (width, height) = img.dimensions();
        let data = img.into_raw();

        // Generate OpenGL texture
        let mut texture_id = 0;
        gl::GenTextures(1, &mut texture_id);
        gl::BindTexture(gl::TEXTURE_2D, texture_id);

        // Upload texture data
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as i32,
            width as i32,
            height as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            data.as_ptr() as *const _,
        );

        // Set texture parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);

        // Generate mipmaps
        gl::GenerateMipmap(gl::TEXTURE_2D);

        Ok(texture_id)
    }

    /// Get texture ID for a sprite type
    pub fn get_texture(&self, sprite_type: SpriteType) -> Option<GLuint> {
        self.textures.get(&sprite_type).copied()
    }

    /// Bind a texture for rendering
    ///
    /// # Safety
    /// Must be called with a valid OpenGL context. Texture binding operations
    /// are unsafe OpenGL calls that require proper context management.
    pub unsafe fn bind_texture(&self, sprite_type: SpriteType) -> bool {
        if let Some(texture_id) = self.get_texture(sprite_type) {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
            true
        } else {
            // Bind a default white texture or disable texturing
            gl::BindTexture(gl::TEXTURE_2D, 0);
            false
        }
    }

    /// Binds all terrain textures to their respective texture units (0-6)
    /// and item textures to texture unit 7
    ///
    /// # Safety
    /// Must be called with a valid OpenGL context. All textures must be loaded first.
    pub unsafe fn bind_all_textures(&self) {
        let terrain_types = [
            SpriteType::Forest,
            SpriteType::Forest2,
            SpriteType::Grasslands,
            SpriteType::HauntedWoods,
            SpriteType::Hills,
            SpriteType::Mountain,
            SpriteType::Swamp,
        ];

        for (i, sprite_type) in terrain_types.iter().enumerate() {
            if let Some(&texture_id) = self.textures.get(sprite_type) {
                gl::ActiveTexture(gl::TEXTURE0 + i as u32);
                gl::BindTexture(gl::TEXTURE_2D, texture_id);
            }
        }

        // Bind item texture to texture unit 7
        if let Some(&texture_id) = self.textures.get(&SpriteType::Item) {
            gl::ActiveTexture(gl::TEXTURE7);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
        }
    }

    /// Clean up textures
    ///
    /// # Safety
    /// Must be called with a valid OpenGL context. Texture deletion operations
    /// are unsafe OpenGL calls that require proper context management.
    pub unsafe fn cleanup(&mut self) {
        for texture_id in self.textures.values() {
            gl::DeleteTextures(1, texture_id);
        }
        self.textures.clear();
    }
}

impl Default for TextureManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for TextureManager {
    fn drop(&mut self) {
        // Note: This should only be called when OpenGL context is still valid
        // In practice, cleanup should be called explicitly before context destruction
    }
}
