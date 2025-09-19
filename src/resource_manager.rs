use std::{
    collections::HashMap,
    fs::read_to_string,
    rc::Rc,
};

use glow::Context;
use image::GenericImageView;

use crate::{
    shader::Shader,
    texture::Texture2D,
};

pub struct ResourceManager {
    pub gl: Rc<Context>,
    shaders: HashMap<String, Shader>,
    textures: HashMap<String, Rc<Texture2D>>,
}

impl ResourceManager {
    pub fn new(gl: Rc<Context>) -> Self {
        Self {
            gl,
            shaders: HashMap::new(),
            textures: HashMap::new(),
        }
    }

    pub fn load_shader_from_file(
        &mut self,
        vertex_path: &str,
        fragment_path: &str,
        geometry_path: Option<&str>,
    ) -> Shader {
        let vertex_code = read_to_string(vertex_path).expect("Failed to read vertex shader");
        let fragment_code = read_to_string(fragment_path).expect("Failed to read fragment shader");
        let geometry_code = geometry_path
            .map(|geom_path| read_to_string(geom_path).expect("Failed to read geometry shader"));

        let shader = Shader::new(self.gl.clone(), vertex_code, fragment_code, geometry_code);
        self.shaders.insert("sprite".to_string(), shader.clone());

        shader
    }

    pub fn load_texture_from_file(&mut self, path: &str, name: &str) {
        let texture = Texture2D::new(self.gl.clone());
        println!("Loading texture from file: {}", path);
        let img = image::open(path).expect("Failed to load texture");
        //let img = img.flipv();

        let (width, height) = img.dimensions();

        let data = img.to_rgba8().into_raw();

        println!("Texture width: {}, height: {}", width, height);

        texture.generate(width, height, &data);
        self.textures.insert(name.to_string(), Rc::new(texture));
    }

    pub fn get_texture(&self, name: &str) -> Rc<Texture2D> {
        self.textures.get(name).unwrap().clone()
    }
}

impl Drop for ResourceManager {
    fn drop(&mut self) {
        for (_, shader) in self.shaders.iter() {
            shader.clean();
        }

        // TEXTURES.with(|textures| {
        //     let textures = textures.lock().unwrap();
        //     for (_, texture) in textures.iter() {
        //         texture.clean();
        //     }
        // });
    }
}
