use wasm_bindgen::{
    JsCast,
    JsValue,
    prelude::*,
};
use {
    web_sys::*,
    WebGlRenderingContext as GL
};

pub fn initialize_webgl_context() -> Result<GL, JsValue>{
    use event_listener::*;

    let window = window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("rustCanvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
    let gl: GL = canvas.get_context("webgl")?.unwrap().dyn_into()?;

    attach_mouse_scroll_handler(&canvas)?;
    attach_mouse_down_handler(&canvas)?;
    attach_mouse_up_handler(&canvas)?;
    attach_mouse_move_handler(&canvas)?;

    gl.clear_color(0., 0.0, 0.0, 1.0); //RGBA
    gl.clear_depth(1.);
    gl.enable(GL::DEPTH_TEST);
    gl.enable(GL::CULL_FACE);
    gl.depth_func(GL::LESS); 

    Ok(gl)
}

pub fn link_program(
    gl: &WebGlRenderingContext,
    vert_source: &str,
    frag_source: &str,
) -> Result<WebGlProgram, String> {
    let program = gl
        .create_program()
        .ok_or_else(|| String::from("Error creating program"))?;

    let vert_shader = compile_shader(
        &gl,
        GL::VERTEX_SHADER,
        vert_source,
    ).unwrap();

    let frag_shader = compile_shader(
        &gl,
        GL::FRAGMENT_SHADER,
        frag_source,
    ).unwrap();

    gl.attach_shader(&program, &vert_shader);
    gl.attach_shader(&program, &frag_shader);
    gl.link_program(&program);

    if gl.get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(gl.get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}

fn compile_shader(
    gl: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = gl
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Error creating shader"))?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);
    
    if gl.get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false) 
    {
        Ok(shader)
    } else {
        Err(gl.get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unable to get shader info log")))
    }
}

mod event_listener{
    use wasm_bindgen::{
        JsCast,
        JsValue,
        prelude::*,
    };
    use web_sys::*;

    pub fn attach_mouse_down_handler(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
        let listener = move |event: web_sys::WheelEvent| {
            //handler
            super::super::app_state::update_mouse_down(event.client_x() as f32, event.client_y() as f32, true);
        };
        //create listener on heap
        let listener = Closure::wrap(Box::new(listener) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mousedown", listener.as_ref().unchecked_ref())?;
        //create memory leak on purpose
        //listener is requiered for the duration of the program running
        listener.forget();
    
        Ok(())
    }
    
    pub fn attach_mouse_up_handler(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
        let listener = move |event: web_sys::WheelEvent| {
            //handler
            super::super::app_state::update_mouse_down(event.client_x() as f32, event.client_y() as f32, false);
        };
    
        let listener = Closure::wrap(Box::new(listener) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mouseup", listener.as_ref().unchecked_ref())?;
        listener.forget();
    
        Ok(())
    }
    
    pub fn attach_mouse_move_handler(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
        let listener = move |event: web_sys::WheelEvent| {
            super::super::app_state::update_mouse_position(event.client_x() as f32, event.client_y() as f32);
        };
    
        let listener = Closure::wrap(Box::new(listener) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mousemove", listener.as_ref().unchecked_ref())?;
        listener.forget();
    
        Ok(())
    }

    pub fn attach_mouse_scroll_handler(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
        let listener = move |event: web_sys::WheelEvent| {
            super::super::app_state::update_mouse_scroll(event.delta_y());
        };

        let listener = Closure::wrap(Box::new(listener) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mousewheel", listener.as_ref().unchecked_ref())?;
        listener.forget();

        Ok(())
    }

    pub fn attach_video_pause_handler(target: &EventTarget) -> Result<(), JsValue> {
        let listener = move |custom_event: web_sys::CustomEvent| {
            match custom_event.detail().as_bool().unwrap(){ 
                true => super::super::app_state::update_video_pause(true), 
                false => super::super::app_state::update_video_pause(false)
            }
        };

        let listener = Closure::wrap(Box::new(listener) as Box<dyn FnMut(_)>);
        target.add_event_listener_with_callback("pause", listener.as_ref().unchecked_ref())?;
        listener.forget();
        Ok(())
    }

    pub fn attach_video_reset_handler(target: &EventTarget) -> Result<(), JsValue> {
        let listener = move |event: web_sys::Event| {
            super::super::app_state::reset_video();
        };

        let listener = Closure::wrap(Box::new(listener) as Box<dyn FnMut(_)>);
        target.add_event_listener_with_callback("pause", listener.as_ref().unchecked_ref())?;
        listener.forget();
        Ok(())
    }
    
}


