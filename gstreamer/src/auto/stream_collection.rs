// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Object;
#[cfg(any(feature = "v1_10", feature = "dox"))]
use Stream;
use ffi;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib::object::IsA;
use glib::object::ObjectType;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct StreamCollection(Object<ffi::GstStreamCollection, ffi::GstStreamCollectionClass, StreamCollectionClass>) @extends Object;

    match fn {
        get_type => || ffi::gst_stream_collection_get_type(),
    }
}

impl StreamCollection {
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn add_stream<P: IsA<Stream>>(&self, stream: &P) -> bool {
        unsafe {
            from_glib(ffi::gst_stream_collection_add_stream(self.to_glib_none().0, stream.as_ref().to_glib_full()))
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn get_size(&self) -> u32 {
        unsafe {
            ffi::gst_stream_collection_get_size(self.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn get_stream(&self, index: u32) -> Option<Stream> {
        unsafe {
            from_glib_none(ffi::gst_stream_collection_get_stream(self.to_glib_none().0, index))
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn get_upstream_id(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::gst_stream_collection_get_upstream_id(self.to_glib_none().0))
        }
    }

    pub fn get_property_upstream_id(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.as_ptr() as *mut gobject_ffi::GObject, b"upstream-id\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    pub fn set_property_upstream_id<'a, P: Into<Option<&'a str>>>(&self, upstream_id: P) {
        let upstream_id = upstream_id.into();
        unsafe {
            gobject_ffi::g_object_set_property(self.as_ptr() as *mut gobject_ffi::GObject, b"upstream-id\0".as_ptr() as *const _, Value::from(upstream_id).to_glib_none().0);
        }
    }

    //pub fn connect_stream_notify<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored p0: GObject.ParamSpec
    //}

    pub fn connect_property_upstream_id_notify<F: Fn(&StreamCollection) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&StreamCollection) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::upstream-id\0".as_ptr() as *const _,
                transmute(notify_upstream_id_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe impl Send for StreamCollection {}
unsafe impl Sync for StreamCollection {}

pub const NONE_STREAM_COLLECTION: Option<&StreamCollection> = None;

unsafe extern "C" fn notify_upstream_id_trampoline(this: *mut ffi::GstStreamCollection, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    let f: &&(Fn(&StreamCollection) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}
