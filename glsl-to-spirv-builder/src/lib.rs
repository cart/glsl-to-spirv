// Copyright (c) 2016 The vulkano developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

mod glslang_c_interface;

use glslang_c_interface::*;

pub type SpirvOutput = Vec<u32>;

pub fn compile(
    code: &str,
    ty: ShaderType,
    shader_defs: Option<&[String]>,
) -> Result<SpirvOutput, String> {
    compile_inner(Some((code, ty)), shader_defs)
}

// Eventually the API will look like this, with an iterator for multiple shader stages.
// However for the moment GLSLang doesn't like that, so we only pass one shader at a time.
fn compile_inner<'a, I>(shaders: I, shader_defs: Option<&[String]>) -> Result<SpirvOutput, String>
where
    I: IntoIterator<Item = (&'a str, ShaderType)>,
{
    let mut preamble = String::new();
    if let Some(defs) = shader_defs {
        for def in defs {
            preamble.push_str("#define ");

            let mut def = def.clone();
            if let Some(end) = def.find('\n') {
                def.truncate(end);
            }

            let def = def.replacen('=', " ", 1);

            preamble.push_str(&def);
            preamble.push('\n');
        }
    }

    let mut data = Vec::new();

    unsafe {
        glslang_initialize_process();

        for (source, ty) in shaders.into_iter() {
            let stage = match ty {
                ShaderType::Vertex => glslang_stage_t_GLSLANG_STAGE_VERTEX,
                ShaderType::Fragment => glslang_stage_t_GLSLANG_STAGE_FRAGMENT,
                ShaderType::Geometry => glslang_stage_t_GLSLANG_STAGE_GEOMETRY,
                ShaderType::TessellationControl => glslang_stage_t_GLSLANG_STAGE_TESSCONTROL,
                ShaderType::TessellationEvaluation => glslang_stage_t_GLSLANG_STAGE_TESSEVALUATION,
                ShaderType::Compute => glslang_stage_t_GLSLANG_STAGE_COMPUTE,
            };

            let mut source = String::from(source);
            if shader_defs.is_some() {
                if let Some(version) = source.find(r##"#version"##) {
                    if let Some(newline) = &source[version..].find('\n') {
                        source.insert_str(version + newline + 1, &preamble);
                    }
                }
            }

            let c_str = CString::new(source.as_str()).unwrap();
            let code: *const c_char = c_str.as_ptr();

            let input = &glslang_input_t {
                language: glslang_source_t_GLSLANG_SOURCE_GLSL,
                stage,
                client: glslang_client_t_GLSLANG_CLIENT_VULKAN,
                client_version: glslang_target_client_version_t_GLSLANG_TARGET_VULKAN_1_0,
                target_language: glslang_target_language_t_GLSLANG_TARGET_SPV,
                target_language_version: glslang_target_language_version_t_GLSLANG_TARGET_SPV_1_0,
                code,
                default_version: 100,
                default_profile: glslang_profile_t_GLSLANG_NO_PROFILE,
                force_default_version_and_profile: 0,
                forward_compatible: 0,
                messages: glslang_messages_t_GLSLANG_MSG_DEFAULT_BIT
                    | glslang_messages_t_GLSLANG_MSG_SPV_RULES_BIT
                    | glslang_messages_t_GLSLANG_MSG_VULKAN_RULES_BIT,
                resource: DEFAULT_RESOURCE_LIMITS,
            };

            let shader = glslang_shader_create(input);

            if glslang_shader_preprocess(shader, input) == 0 {
                let c_info: &CStr = CStr::from_ptr(glslang_shader_get_info_log(shader));
                let c_debug: &CStr = CStr::from_ptr(glslang_shader_get_info_debug_log(shader));

                let mut error = String::from("glslang_shader_preprocess:\n");
                error.push_str(&format!("Info log:\n{}\n", c_info.to_str().unwrap()));
                error.push_str(&format!("Debug log:\n{}\n", c_debug.to_str().unwrap()));

                return Err(error);
            }
            if glslang_shader_parse(shader, input) == 0 {
                let c_info: &CStr = CStr::from_ptr(glslang_shader_get_info_log(shader));
                let c_debug: &CStr = CStr::from_ptr(glslang_shader_get_info_debug_log(shader));

                let mut error = String::from("glslang_shader_parse:\n");
                error.push_str(&format!("Info log:\n{}\n", c_info.to_str().unwrap()));
                error.push_str(&format!("Debug log:\n{}\n", c_debug.to_str().unwrap()));

                return Err(error);
            }

            let program = glslang_program_create();
            glslang_program_add_shader(program, shader);

            if glslang_program_link(
                program,
                glslang_messages_t_GLSLANG_MSG_SPV_RULES_BIT
                    | glslang_messages_t_GLSLANG_MSG_VULKAN_RULES_BIT,
            ) == 0
            {
                let c_info: &CStr = CStr::from_ptr(glslang_program_get_info_log(program));
                let c_debug: &CStr = CStr::from_ptr(glslang_program_get_info_debug_log(program));

                let mut error = String::from("glslang_program_link:\n");
                error.push_str(&format!("Info log:\n{}\n", c_info.to_str().unwrap()));
                error.push_str(&format!("Debug log:\n{}\n", c_debug.to_str().unwrap()));

                return Err(error);
            }

            glslang_program_SPIRV_generate(program, input.stage);

            if glslang_program_SPIRV_get_messages(program) != std::ptr::null() {
                let c_messages: &CStr = CStr::from_ptr(glslang_program_SPIRV_get_messages(program));

                println!("{:?}", c_messages);
            }

            let size = glslang_program_SPIRV_get_size(program) as usize;
            let ptr = glslang_program_SPIRV_get_ptr(program) as *mut u32;
            data = std::slice::from_raw_parts(ptr, size).to_vec();

            glslang_program_delete(program);
            glslang_shader_delete(shader);
        }

        glslang_finalize_process();
    }

    return Ok(data);
}

/// Type of shader.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShaderType {
    Vertex,
    Fragment,
    Geometry,
    TessellationControl,
    TessellationEvaluation,
    Compute,
}

// values copied from glslang default resource limits
const DEFAULT_RESOURCE_LIMITS: &glslang_resource_t = &glslang_resource_t {
    max_lights: 32,
    max_clip_planes: 6,
    max_texture_units: 32,
    max_texture_coords: 32,
    max_vertex_attribs: 64,
    max_vertex_uniform_components: 4096,
    max_varying_floats: 64,
    max_vertex_texture_image_units: 32,
    max_combined_texture_image_units: 80,
    max_texture_image_units: 32,
    max_fragment_uniform_components: 4096,
    max_draw_buffers: 32,
    max_vertex_uniform_vectors: 128,
    max_varying_vectors: 8,
    max_fragment_uniform_vectors: 16,
    max_vertex_output_vectors: 16,
    max_fragment_input_vectors: 15,
    min_program_texel_offset: -8,
    max_program_texel_offset: 7,
    max_clip_distances: 8,
    max_compute_work_group_count_x: 65535,
    max_compute_work_group_count_y: 65535,
    max_compute_work_group_count_z: 65535,
    max_compute_work_group_size_x: 1024,
    max_compute_work_group_size_y: 1024,
    max_compute_work_group_size_z: 64,
    max_compute_uniform_components: 1024,
    max_compute_texture_image_units: 16,
    max_compute_image_uniforms: 8,
    max_compute_atomic_counters: 8,
    max_compute_atomic_counter_buffers: 1,
    max_varying_components: 60,
    max_vertex_output_components: 64,
    max_geometry_input_components: 64,
    max_geometry_output_components: 128,
    max_fragment_input_components: 128,
    max_image_units: 8,
    max_combined_image_units_and_fragment_outputs: 8,
    max_combined_shader_output_resources: 8,
    max_image_samples: 0,
    max_vertex_image_uniforms: 0,
    max_tess_control_image_uniforms: 0,
    max_tess_evaluation_image_uniforms: 0,
    max_geometry_image_uniforms: 0,
    max_fragment_image_uniforms: 8,
    max_combined_image_uniforms: 8,
    max_geometry_texture_image_units: 16,
    max_geometry_output_vertices: 256,
    max_geometry_total_output_components: 1024,
    max_geometry_uniform_components: 1024,
    max_geometry_varying_components: 64,
    max_tess_control_input_components: 128,
    max_tess_control_output_components: 128,
    max_tess_control_texture_image_units: 16,
    max_tess_control_uniform_components: 1024,
    max_tess_control_total_output_components: 4096,
    max_tess_evaluation_input_components: 128,
    max_tess_evaluation_output_components: 128,
    max_tess_evaluation_texture_image_units: 16,
    max_tess_evaluation_uniform_components: 1024,
    max_tess_patch_components: 120,
    max_patch_vertices: 32,
    max_tess_gen_level: 64,
    max_viewports: 16,
    max_vertex_atomic_counters: 0,
    max_tess_control_atomic_counters: 0,
    max_tess_evaluation_atomic_counters: 0,
    max_geometry_atomic_counters: 0,
    max_fragment_atomic_counters: 8,
    max_combined_atomic_counters: 8,
    max_atomic_counter_bindings: 1,
    max_vertex_atomic_counter_buffers: 0,
    max_tess_control_atomic_counter_buffers: 0,
    max_tess_evaluation_atomic_counter_buffers: 0,
    max_geometry_atomic_counter_buffers: 0,
    max_fragment_atomic_counter_buffers: 1,
    max_combined_atomic_counter_buffers: 1,
    max_atomic_counter_buffer_size: 16384,
    max_transform_feedback_buffers: 4,
    max_transform_feedback_interleaved_components: 64,
    max_cull_distances: 8,
    max_combined_clip_and_cull_distances: 8,
    max_samples: 4,
    max_mesh_output_vertices_nv: 256,
    max_mesh_output_primitives_nv: 512,
    max_mesh_work_group_size_x_nv: 32,
    max_mesh_work_group_size_y_nv: 1,
    max_mesh_work_group_size_z_nv: 1,
    max_task_work_group_size_x_nv: 32,
    max_task_work_group_size_y_nv: 1,
    max_task_work_group_size_z_nv: 1,
    max_mesh_view_count_nv: 4,
    limits: glslang_limits_s {
        non_inductive_for_loops: true,
        while_loops: true,
        do_while_loops: true,
        general_uniform_indexing: true,
        general_attribute_matrix_vector_indexing: true,
        general_varying_indexing: true,
        general_sampler_indexing: true,
        general_variable_indexing: true,
        general_constant_matrix_vector_indexing: true,
    },
};
