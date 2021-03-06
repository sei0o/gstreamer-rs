// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v1_10", feature = "dox"))]
use glib::object::ObjectType as ObjectType_;
#[cfg(any(feature = "v1_10", feature = "dox"))]
use glib::signal::connect_raw;
#[cfg(any(feature = "v1_10", feature = "dox"))]
use glib::signal::SignalHandlerId;
use glib::translate::*;
#[cfg(any(feature = "v1_10", feature = "dox"))]
use glib::GString;
#[cfg(any(feature = "v1_10", feature = "dox"))]
use glib_sys;
use gst_sys;
#[cfg(any(feature = "v1_10", feature = "dox"))]
use std::boxed::Box as Box_;
#[cfg(any(feature = "v1_10", feature = "dox"))]
use std::mem::transmute;
#[cfg(any(feature = "v1_10", feature = "dox"))]
use Caps;
use Object;
#[cfg(any(feature = "v1_10", feature = "dox"))]
use StreamFlags;
#[cfg(any(feature = "v1_10", feature = "dox"))]
use StreamType;
#[cfg(any(feature = "v1_10", feature = "dox"))]
use TagList;

glib_wrapper! {
    pub struct Stream(Object<gst_sys::GstStream, gst_sys::GstStreamClass, StreamClass>) @extends Object;

    match fn {
        get_type => || gst_sys::gst_stream_get_type(),
    }
}

impl Stream {
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn get_caps(&self) -> Option<Caps> {
        unsafe { from_glib_full(gst_sys::gst_stream_get_caps(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn get_stream_flags(&self) -> StreamFlags {
        unsafe { from_glib(gst_sys::gst_stream_get_stream_flags(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn get_stream_id(&self) -> Option<GString> {
        unsafe { from_glib_none(gst_sys::gst_stream_get_stream_id(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn get_stream_type(&self) -> StreamType {
        unsafe { from_glib(gst_sys::gst_stream_get_stream_type(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn get_tags(&self) -> Option<TagList> {
        unsafe { from_glib_full(gst_sys::gst_stream_get_tags(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn set_caps(&self, caps: Option<&Caps>) {
        unsafe {
            gst_sys::gst_stream_set_caps(self.to_glib_none().0, caps.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn set_stream_flags(&self, flags: StreamFlags) {
        unsafe {
            gst_sys::gst_stream_set_stream_flags(self.to_glib_none().0, flags.to_glib());
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn set_stream_type(&self, stream_type: StreamType) {
        unsafe {
            gst_sys::gst_stream_set_stream_type(self.to_glib_none().0, stream_type.to_glib());
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn set_tags(&self, tags: Option<&TagList>) {
        unsafe {
            gst_sys::gst_stream_set_tags(self.to_glib_none().0, tags.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn connect_property_caps_notify<F: Fn(&Stream) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_caps_trampoline<F: Fn(&Stream) + Send + Sync + 'static>(
            this: *mut gst_sys::GstStream,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::caps\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_caps_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn connect_property_stream_flags_notify<F: Fn(&Stream) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_stream_flags_trampoline<
            F: Fn(&Stream) + Send + Sync + 'static,
        >(
            this: *mut gst_sys::GstStream,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::stream-flags\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_stream_flags_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn connect_property_stream_type_notify<F: Fn(&Stream) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_stream_type_trampoline<
            F: Fn(&Stream) + Send + Sync + 'static,
        >(
            this: *mut gst_sys::GstStream,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::stream-type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_stream_type_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn connect_property_tags_notify<F: Fn(&Stream) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_tags_trampoline<F: Fn(&Stream) + Send + Sync + 'static>(
            this: *mut gst_sys::GstStream,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::tags\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_tags_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

unsafe impl Send for Stream {}
unsafe impl Sync for Stream {}
