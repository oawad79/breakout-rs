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

use crate::shader::Shader;

pub struct ResourceManager {
    pub gl: Rc<Context>,
}

thread_local! {
    static SHADERS: LazyLock<Mutex<HashMap<String, Shader>>> =
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
}

impl Drop for ResourceManager {
    fn drop(&mut self) {
        SHADERS.with(|shaders| {
            let shaders = shaders.lock().unwrap();
            for (_, shader) in shaders.iter() {
                shader.clean();
            }
        });
    }
}
