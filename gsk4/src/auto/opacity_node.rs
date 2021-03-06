// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::RenderNode;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::glib_wrapper! {
    pub struct OpacityNode(Object<ffi::GskOpacityNode>) @extends RenderNode;

    match fn {
        get_type => || ffi::gsk_opacity_node_get_type(),
    }
}

impl OpacityNode {
    #[doc(alias = "gsk_opacity_node_new")]
    pub fn new<P: IsA<RenderNode>>(child: &P, opacity: f32) -> OpacityNode {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gsk_opacity_node_new(
                child.as_ref().to_glib_none().0,
                opacity,
            ))
        }
    }

    #[doc(alias = "gsk_opacity_node_get_child")]
    pub fn get_child(&self) -> Option<RenderNode> {
        unsafe { from_glib_none(ffi::gsk_opacity_node_get_child(self.to_glib_none().0)) }
    }

    #[doc(alias = "gsk_opacity_node_get_opacity")]
    pub fn get_opacity(&self) -> f32 {
        unsafe { ffi::gsk_opacity_node_get_opacity(self.to_glib_none().0) }
    }
}

impl fmt::Display for OpacityNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("OpacityNode")
    }
}
