// This file was generated by gir (https://github.com/gtk-rs/gir @ 6855214)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use ffi;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct RTSPAddress(Boxed<ffi::GstRTSPAddress>);

    match fn {
        copy => |ptr| ffi::gst_rtsp_address_copy(mut_override(ptr)),
        free => |ptr| ffi::gst_rtsp_address_free(ptr),
        get_type => || ffi::gst_rtsp_address_get_type(),
    }
}

unsafe impl Send for RTSPAddress {}