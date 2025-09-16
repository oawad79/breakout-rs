use std::{
    //collections::HashMap,
    collections::HashMap,
    fs::read_to_string,
    rc::Rc,
    sync::{
        LazyLock,
        Mutex,
    },
};

use glow::Context;
use image::GenericImageView;

use crate::{
    shader::Shader,
    texture::Texture2D,
};

pub struct ResourceManager {
    pub gl: Rc<Context>,
}

thread_local! {
    static SHADERS: LazyLock<Mutex<HashMap<String, Shader>>> =
        LazyLock::new(|| Mutex::new(HashMap::new()));

    static TEXTURES: LazyLock<Mutex<HashMap<String, Texture2D>>> =
        LazyLock::new(|| Mutex::new(HashMap::new()));
}

impl ResourceManager {
    pub fn load_shader_from_file(
        &self,
        vertex_path: &str,
        fragment_path: &str,
        geometry_path: Option<&str>,
    ) -> Shader {
        let vertex_code = read_to_string(vertex_path).expect("Failed to read vertex shader");
        let fragment_code = read_to_string(fragment_path).expect("Failed to read fragment shader");
        let geometry_code = geometry_path
            .map(|geom_path| read_to_string(geom_path).expect("Failed to read geometry shader"));

        let shader = Shader::new(self.gl.clone(), vertex_code, fragment_code, geometry_code);
        SHADERS.with(|shaders| {
            shaders
                .lock()
                .unwrap()
                .insert("sprite".to_string(), shader.clone());
        });
        shader
    }

    pub fn load_texture_from_file(&self, path: &str, name: &str) -> Texture2D {
        let texture = Texture2D::new(self.gl.clone());
        println!("Loading texture from file: {}", path);
        let img = image::open(path).expect("Failed to load texture");
        //let img = img.flipv();

        let (width, height) = img.dimensions();

        let data = img.to_rgba8().into_raw();

        println!("Texture width: {}, height: {}", width, height);

        texture.generate(width, height, data.as_slice());
        TEXTURES.with(|textures| {
            textures
                .lock()
                .unwrap()
                .insert(name.to_string(), texture.clone());
        });
        texture
    }

    pub fn get_texture(&self, name: String) -> Texture2D {
        TEXTURES.with(|textures| {
            textures
                .lock()
                .unwrap()
                .get(&name)
                .expect(&format!("Texture '{}' not found", name))
                .clone()
        })
    }
}

impl Drop for ResourceManager {
    fn drop(&mut self) {
        SHADERS.with(|shaders| {
            let shaders = shaders.lock().unwrap();
            for (_, shader) in shaders.iter() {
                shader.clean();
            }
        });

        // TEXTURES.with(|textures| {
        //     let textures = textures.lock().unwrap();
        //     for (_, texture) in textures.iter() {
        //         texture.clean();
        //     }
        // });
    }
}
