// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::RenderNode;
use crate::RoundedRect;
use glib::translate::*;
use glib::StaticType;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GskOutsetShadowNode")]
    pub struct OutsetShadowNode(Shared<ffi::GskOutsetShadowNode>);

    match fn {
        ref => |ptr| ffi::gsk_render_node_ref(ptr as *mut ffi::GskRenderNode),
        unref => |ptr| ffi::gsk_render_node_unref(ptr as *mut ffi::GskRenderNode),
    }
}

impl glib::StaticType for OutsetShadowNode {
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gsk_outset_shadow_node_get_type()) }
    }
}

impl OutsetShadowNode {
    #[doc(alias = "gsk_outset_shadow_node_new")]
    pub fn new(
        outline: &RoundedRect,
        color: &gdk::RGBA,
        dx: f32,
        dy: f32,
        spread: f32,
        blur_radius: f32,
    ) -> OutsetShadowNode {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gsk_outset_shadow_node_new(
                outline.to_glib_none().0,
                color.to_glib_none().0,
                dx,
                dy,
                spread,
                blur_radius,
            ))
        }
    }

    #[doc(alias = "gsk_outset_shadow_node_get_blur_radius")]
    #[doc(alias = "get_blur_radius")]
    pub fn blur_radius(&self) -> f32 {
        unsafe { ffi::gsk_outset_shadow_node_get_blur_radius(self.to_glib_none().0) }
    }

    #[doc(alias = "gsk_outset_shadow_node_get_color")]
    #[doc(alias = "get_color")]
    pub fn color(&self) -> Option<gdk::RGBA> {
        unsafe { from_glib_none(ffi::gsk_outset_shadow_node_get_color(self.to_glib_none().0)) }
    }

    #[doc(alias = "gsk_outset_shadow_node_get_dx")]
    #[doc(alias = "get_dx")]
    pub fn dx(&self) -> f32 {
        unsafe { ffi::gsk_outset_shadow_node_get_dx(self.to_glib_none().0) }
    }

    #[doc(alias = "gsk_outset_shadow_node_get_dy")]
    #[doc(alias = "get_dy")]
    pub fn dy(&self) -> f32 {
        unsafe { ffi::gsk_outset_shadow_node_get_dy(self.to_glib_none().0) }
    }

    #[doc(alias = "gsk_outset_shadow_node_get_outline")]
    #[doc(alias = "get_outline")]
    pub fn outline(&self) -> Option<RoundedRect> {
        unsafe {
            from_glib_none(ffi::gsk_outset_shadow_node_get_outline(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gsk_outset_shadow_node_get_spread")]
    #[doc(alias = "get_spread")]
    pub fn spread(&self) -> f32 {
        unsafe { ffi::gsk_outset_shadow_node_get_spread(self.to_glib_none().0) }
    }
}

impl fmt::Display for OutsetShadowNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("OutsetShadowNode")
    }
}
