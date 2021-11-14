extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use web_sys::{
    *,
    WebGlRenderingContext as GL
};

#[macro_use]
extern crate lazy_static;

mod common_funcs;
mod gl_setup;
mod programs;
mod shaders;
mod app_state;
mod constants;

#[wasm_bindgen]
extern "C"{
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

//all the data that is stored on the user client, i.e. the browser
#[wasm_bindgen]
pub struct Client {
    gl: GL,
    program_globe: programs::Globe,
}

#[wasm_bindgen]
impl Client{
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {

        console_error_panic_hook::set_once();
        let gl = gl_setup::initialize_webgl_context().unwrap();

        
        Self{
            program_globe: programs::Globe::new(&gl),
            //program_color_2d: programs::Color2D::new(&gl),
            gl: gl,
        }
    }

    pub fn update(&mut self, time: f32, height: f32, width: f32) -> Result<(), JsValue>{
        app_state::update_dynamic_data(time, height, width);
        //log(&format!("{}", WheelEvent.delta_mode()));
        Ok(())
    }

    pub fn render(&self){
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        let curr_state = app_state::get_curr_state();
        //log(&format!("{}", curr_state.rotation_x_axis));

        self.program_globe.render(
            &self.gl,
            curr_state.control_bottom,
            curr_state.control_top,
            curr_state.control_left,
            curr_state.control_right,
            curr_state.canvas_height,
            curr_state.canvas_width,
            curr_state.rotation_x_axis,
            curr_state.rotation_y_axis,
            curr_state.mouse_scroll,
            //&common_funcs::matrixes::get_updated_3d_y_values(curr_state.time),
        );
    }
}
