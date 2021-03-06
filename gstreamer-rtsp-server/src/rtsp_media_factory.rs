// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use RTSPMediaFactory;

#[cfg(any(feature = "v1_14", feature = "dox"))]
use glib::translate::*;
use glib::IsA;
#[cfg(any(feature = "v1_14", feature = "dox"))]
use gst;
#[cfg(any(feature = "v1_14", feature = "dox"))]
use gst_rtsp_server_sys;

pub trait RTSPMediaFactoryExtManual: 'static {
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    fn add_role_from_structure(&self, structure: &gst::StructureRef);
}

impl<O: IsA<RTSPMediaFactory>> RTSPMediaFactoryExtManual for O {
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    fn add_role_from_structure(&self, structure: &gst::StructureRef) {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_media_factory_add_role_from_structure(
                self.as_ref().to_glib_none().0,
                structure.as_mut_ptr(),
            );
        }
    }
}
