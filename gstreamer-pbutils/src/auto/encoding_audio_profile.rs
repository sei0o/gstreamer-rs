// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use EncodingProfile;
use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct EncodingAudioProfile(Object<ffi::GstEncodingAudioProfile, ffi::GstEncodingAudioProfileClass, EncodingAudioProfileClass>) @extends EncodingProfile;

    match fn {
        get_type => || ffi::gst_encoding_audio_profile_get_type(),
    }
}

impl EncodingAudioProfile {}

unsafe impl Send for EncodingAudioProfile {}
unsafe impl Sync for EncodingAudioProfile {}

pub const NONE_ENCODING_AUDIO_PROFILE: Option<&EncodingAudioProfile> = None;
