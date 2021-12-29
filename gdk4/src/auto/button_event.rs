// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Event;
use glib::translate::*;
use glib::StaticType;

glib::wrapper! {
    #[doc(alias = "GdkButtonEvent")]
    pub struct ButtonEvent(Shared<ffi::GdkButtonEvent>);

    match fn {
        ref => |ptr| ffi::gdk_event_ref(ptr as *mut ffi::GdkEvent),
        unref => |ptr| ffi::gdk_event_unref(ptr as *mut ffi::GdkEvent),
    }
}
impl glib::StaticType for ButtonEvent {
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gdk_button_event_get_type()) }
    }
}

impl ButtonEvent {
    #[doc(alias = "gdk_button_event_get_button")]
    #[doc(alias = "get_button")]
    pub fn button(&self) -> u32 {
        unsafe { ffi::gdk_button_event_get_button(self.to_glib_none().0) }
    }
}
