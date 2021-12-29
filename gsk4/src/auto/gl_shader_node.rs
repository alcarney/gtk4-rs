// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::GLShader;
use crate::RenderNode;
use glib::translate::*;
use glib::StaticType;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GskGLShaderNode")]
    pub struct GLShaderNode(Shared<ffi::GskGLShaderNode>);

    match fn {
        ref => |ptr| ffi::gsk_render_node_ref(ptr as *mut ffi::GskRenderNode),
        unref => |ptr| ffi::gsk_render_node_unref(ptr as *mut ffi::GskRenderNode),
    }
}

impl glib::StaticType for GLShaderNode {
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gsk_gl_shader_node_get_type()) }
    }
}

impl GLShaderNode {
    #[doc(alias = "gsk_gl_shader_node_get_args")]
    #[doc(alias = "get_args")]
    pub fn args(&self) -> Option<glib::Bytes> {
        unsafe { from_glib_none(ffi::gsk_gl_shader_node_get_args(self.to_glib_none().0)) }
    }

    #[doc(alias = "gsk_gl_shader_node_get_child")]
    #[doc(alias = "get_child")]
    pub fn child(&self, idx: u32) -> Option<RenderNode> {
        unsafe {
            from_glib_none(ffi::gsk_gl_shader_node_get_child(
                self.to_glib_none().0,
                idx,
            ))
        }
    }

    #[doc(alias = "gsk_gl_shader_node_get_n_children")]
    #[doc(alias = "get_n_children")]
    pub fn n_children(&self) -> u32 {
        unsafe { ffi::gsk_gl_shader_node_get_n_children(self.to_glib_none().0) }
    }

    #[doc(alias = "gsk_gl_shader_node_get_shader")]
    #[doc(alias = "get_shader")]
    pub fn shader(&self) -> Option<GLShader> {
        unsafe { from_glib_none(ffi::gsk_gl_shader_node_get_shader(self.to_glib_none().0)) }
    }
}

impl fmt::Display for GLShaderNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("GLShaderNode")
    }
}
