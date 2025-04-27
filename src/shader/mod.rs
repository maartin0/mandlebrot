use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader, WebGlUniformLocation};

use crate::{arbitrary_num::ArbitaryNum, matrix::Matrix3, DEPTH};

#[derive(Clone, Debug)]
pub struct CanvasState {
    vertex_count: i32,
    scale_uniform_location: WebGlUniformLocation,
    midpoint_iterations_re_uniform_location: WebGlUniformLocation,
    midpoint_iterations_im_uniform_location: WebGlUniformLocation,
}

fn compile_shader(
    context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

fn link_program(
    context: &WebGl2RenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> WebGlProgram {
    let program = context.create_program().unwrap();

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap()
    {
        program
    } else {
        panic!(
            "{}",
            context
                .get_program_info_log(&program)
                .unwrap_or_else(|| String::from("Unknown error creating program object"))
        )
    }
}

pub fn init_shaders(context: &WebGl2RenderingContext) -> CanvasState {
    console_error_panic_hook::set_once();

    let vert_shader = compile_shader(
        context,
        WebGl2RenderingContext::VERTEX_SHADER,
        include_str!("shader.vert"),
    )
    .unwrap();
    let frag_shader = compile_shader(
        context,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        include_str!("shader.frag"),
    )
    .unwrap();
    let program = link_program(&context, &vert_shader, &frag_shader);
    context.use_program(Some(&program));

    let vertices: [f32; 18] = [
        -1.0, -1.0, 0.0, 1.0, -1.0, 0.0, -1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, -1.0, 0.0, -1.0, 1.0,
        0.0,
    ];

    let position_attribute_location = context.get_attrib_location(&program, "position");
    let buffer = context.create_buffer().unwrap();
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

    // Note that `Float32Array::view` is somewhat dangerous (hence the
    // `unsafe`!). This is creating a raw view into our module's
    // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
    // (aka do a memory allocation in Rust) it'll cause the buffer to change,
    // causing the `Float32Array` to be invalid.
    //
    // As a result, after `Float32Array::view` we have to be very careful not to
    // do any memory allocations before it's dropped.
    unsafe {
        let positions_array_buf_view = js_sys::Float32Array::view(&vertices);

        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &positions_array_buf_view,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    let vao = context.create_vertex_array().unwrap();
    context.bind_vertex_array(Some(&vao));
    context.vertex_attrib_pointer_with_i32(
        position_attribute_location as u32,
        3,
        WebGl2RenderingContext::FLOAT,
        false,
        0,
        0,
    );
    context.enable_vertex_attrib_array(position_attribute_location as u32);
    context.bind_vertex_array(Some(&vao));

    CanvasState {
        scale_uniform_location: context.get_uniform_location(&program, "scale").unwrap(),
        midpoint_iterations_re_uniform_location: context
            .get_uniform_location(&program, "midpoint_iterations_re")
            .unwrap(),
        midpoint_iterations_im_uniform_location: context
            .get_uniform_location(&program, "midpoint_iterations_im")
            .unwrap(),
        vertex_count: (vertices.len() / 3) as i32,
    }
}

/// Computes iterations in the mandlebrot set with very high precision
fn compute_iterations((x, y): (ArbitaryNum, ArbitaryNum)) -> ([f32; DEPTH], [f32; DEPTH]) {
    let mut re = ArbitaryNum::zero();
    let mut im = ArbitaryNum::zero();
    let mut re_result = [0.0; DEPTH];
    let mut im_result = [0.0; DEPTH];
    for i in 0..DEPTH {
        re = re.clone() * re.clone() - im.clone() + x.clone();
        im = ArbitaryNum::two() * re.clone() * im.clone() + y.clone();
        re_result[i] = re.clone().into();
        im_result[i] = im.clone().into();
    }
    (re_result, im_result)
}

pub fn draw(context: &WebGl2RenderingContext, options: &CanvasState, transform: Matrix3) {
    let scale = [transform[0].clone().into(), transform[4].clone().into()];
    let (midpoint_x, midpoint_y, _) =
        transform * (ArbitaryNum::zero(), ArbitaryNum::zero(), ArbitaryNum::one());
    let (midpoint_iterations_re, midpoint_iterations_im) =
        compute_iterations((midpoint_x, midpoint_y));
    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    context.uniform2fv_with_f32_array(Some(&options.scale_uniform_location), &scale);
    context.uniform1fv_with_f32_array(
        Some(&options.midpoint_iterations_re_uniform_location),
        &midpoint_iterations_re,
    );
    context.uniform1fv_with_f32_array(
        Some(&options.midpoint_iterations_im_uniform_location),
        &midpoint_iterations_im,
    );
    context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, options.vertex_count);
}
