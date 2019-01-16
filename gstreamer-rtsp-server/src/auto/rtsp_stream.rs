// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use RTSPAddress;
use RTSPAddressPool;
use RTSPPublishClockMode;
use RTSPStreamTransport;
use ffi;
use gio;
use glib;
use glib::GString;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use gst;
use gst_ffi;
use gst_rtsp;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;

glib_wrapper! {
    pub struct RTSPStream(Object<ffi::GstRTSPStream, ffi::GstRTSPStreamClass, RTSPStreamClass>);

    match fn {
        get_type => || ffi::gst_rtsp_stream_get_type(),
    }
}

impl RTSPStream {
    pub fn new<P: IsA<gst::Element>, Q: IsA<gst::Pad>>(idx: u32, payloader: &P, pad: &Q) -> RTSPStream {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_rtsp_stream_new(idx, payloader.as_ref().to_glib_none().0, pad.as_ref().to_glib_none().0))
        }
    }
}

unsafe impl Send for RTSPStream {}
unsafe impl Sync for RTSPStream {}

pub const NONE_RTSP_STREAM: Option<&RTSPStream> = None;

pub trait RTSPStreamExt: 'static {
    fn add_transport<P: IsA<RTSPStreamTransport>>(&self, trans: &P) -> Result<(), glib::error::BoolError>;

    //fn allocate_udp_sockets(&self, family: gio::SocketFamily, transport: /*Ignored*/&mut gst_rtsp::RTSPTransport, use_client_settings: bool) -> bool;

    //fn complete_stream(&self, transport: /*Ignored*/&gst_rtsp::RTSPTransport) -> bool;

    fn get_address_pool(&self) -> Option<RTSPAddressPool>;

    fn get_buffer_size(&self) -> u32;

    fn get_caps(&self) -> Option<gst::Caps>;

    fn get_control(&self) -> Option<GString>;

    fn get_current_seqnum(&self) -> u16;

    fn get_dscp_qos(&self) -> i32;

    fn get_index(&self) -> u32;

    fn get_joined_bin(&self) -> Option<gst::Bin>;

    fn get_mtu(&self) -> u32;

    fn get_multicast_address(&self, family: gio::SocketFamily) -> Option<RTSPAddress>;

    fn get_multicast_iface(&self) -> Option<GString>;

    fn get_profiles(&self) -> gst_rtsp::RTSPProfile;

    fn get_protocols(&self) -> gst_rtsp::RTSPLowerTrans;

    fn get_pt(&self) -> u32;

    fn get_publish_clock_mode(&self) -> RTSPPublishClockMode;

    fn get_retransmission_pt(&self) -> u32;

    fn get_retransmission_time(&self) -> gst::ClockTime;

    fn get_rtcp_multicast_socket(&self, family: gio::SocketFamily) -> Option<gio::Socket>;

    fn get_rtcp_socket(&self, family: gio::SocketFamily) -> Option<gio::Socket>;

    fn get_rtp_multicast_socket(&self, family: gio::SocketFamily) -> Option<gio::Socket>;

    fn get_rtp_socket(&self, family: gio::SocketFamily) -> Option<gio::Socket>;

    fn get_rtpinfo(&self) -> Option<(u32, u32, u32, gst::ClockTime)>;

    fn get_rtpsession(&self) -> Option<glib::Object>;

    //fn get_server_port(&self, server_port: /*Ignored*/gst_rtsp::RTSPRange, family: gio::SocketFamily);

    fn get_sinkpad(&self) -> Option<gst::Pad>;

    fn get_srcpad(&self) -> Option<gst::Pad>;

    fn get_srtp_encoder(&self) -> Option<gst::Element>;

    fn get_ssrc(&self) -> u32;

    fn has_control<'a, P: Into<Option<&'a str>>>(&self, control: P) -> bool;

    fn is_blocking(&self) -> bool;

    fn is_client_side(&self) -> bool;

    fn is_complete(&self) -> bool;

    fn is_receiver(&self) -> bool;

    fn is_sender(&self) -> bool;

    //fn is_transport_supported(&self, transport: /*Ignored*/&mut gst_rtsp::RTSPTransport) -> bool;

    fn join_bin<P: IsA<gst::Bin>, Q: IsA<gst::Element>>(&self, bin: &P, rtpbin: &Q, state: gst::State) -> Result<(), glib::error::BoolError>;

    fn leave_bin<P: IsA<gst::Bin>, Q: IsA<gst::Element>>(&self, bin: &P, rtpbin: &Q) -> Result<(), glib::error::BoolError>;

    fn remove_transport<P: IsA<RTSPStreamTransport>>(&self, trans: &P) -> Result<(), glib::error::BoolError>;

    fn request_aux_sender(&self, sessid: u32) -> Option<gst::Element>;

    fn reserve_address(&self, address: &str, port: u32, n_ports: u32, ttl: u32) -> Option<RTSPAddress>;

    fn seekable(&self) -> bool;

    fn set_address_pool<'a, P: IsA<RTSPAddressPool> + 'a, Q: Into<Option<&'a P>>>(&self, pool: Q);

    fn set_blocked(&self, blocked: bool) -> bool;

    fn set_buffer_size(&self, size: u32);

    fn set_client_side(&self, client_side: bool);

    fn set_control<'a, P: Into<Option<&'a str>>>(&self, control: P);

    fn set_dscp_qos(&self, dscp_qos: i32);

    fn set_mtu(&self, mtu: u32);

    fn set_multicast_iface<'a, P: Into<Option<&'a str>>>(&self, multicast_iface: P);

    fn set_profiles(&self, profiles: gst_rtsp::RTSPProfile);

    fn set_protocols(&self, protocols: gst_rtsp::RTSPLowerTrans);

    fn set_pt_map(&self, pt: u32, caps: &gst::Caps);

    fn set_publish_clock_mode(&self, mode: RTSPPublishClockMode);

    fn set_retransmission_pt(&self, rtx_pt: u32);

    fn set_retransmission_time(&self, time: gst::ClockTime);

    fn set_seqnum_offset(&self, seqnum: u16);

    //fn transport_filter<'a, P: Into<Option<&'a /*Unimplemented*/RTSPStreamTransportFilterFunc>>, Q: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, func: P, user_data: Q) -> Vec<RTSPStreamTransport>;

    fn unblock_linked(&self) -> bool;

    fn update_crypto<'a, P: Into<Option<&'a gst::Caps>>>(&self, ssrc: u32, crypto: P) -> bool;

    fn connect_new_rtcp_encoder<F: Fn(&Self, &gst::Element) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_new_rtp_encoder<F: Fn(&Self, &gst::Element) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_control_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_profiles_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_protocols_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<RTSPStream>> RTSPStreamExt for O {
    fn add_transport<P: IsA<RTSPStreamTransport>>(&self, trans: &P) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(ffi::gst_rtsp_stream_add_transport(self.as_ref().to_glib_none().0, trans.as_ref().to_glib_none().0), "Failed to add transport")
        }
    }

    //fn allocate_udp_sockets(&self, family: gio::SocketFamily, transport: /*Ignored*/&mut gst_rtsp::RTSPTransport, use_client_settings: bool) -> bool {
    //    unsafe { TODO: call ffi::gst_rtsp_stream_allocate_udp_sockets() }
    //}

    //fn complete_stream(&self, transport: /*Ignored*/&gst_rtsp::RTSPTransport) -> bool {
    //    unsafe { TODO: call ffi::gst_rtsp_stream_complete_stream() }
    //}

    fn get_address_pool(&self) -> Option<RTSPAddressPool> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_stream_get_address_pool(self.as_ref().to_glib_none().0))
        }
    }

    fn get_buffer_size(&self) -> u32 {
        unsafe {
            ffi::gst_rtsp_stream_get_buffer_size(self.as_ref().to_glib_none().0)
        }
    }

    fn get_caps(&self) -> Option<gst::Caps> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_stream_get_caps(self.as_ref().to_glib_none().0))
        }
    }

    fn get_control(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_stream_get_control(self.as_ref().to_glib_none().0))
        }
    }

    fn get_current_seqnum(&self) -> u16 {
        unsafe {
            ffi::gst_rtsp_stream_get_current_seqnum(self.as_ref().to_glib_none().0)
        }
    }

    fn get_dscp_qos(&self) -> i32 {
        unsafe {
            ffi::gst_rtsp_stream_get_dscp_qos(self.as_ref().to_glib_none().0)
        }
    }

    fn get_index(&self) -> u32 {
        unsafe {
            ffi::gst_rtsp_stream_get_index(self.as_ref().to_glib_none().0)
        }
    }

    fn get_joined_bin(&self) -> Option<gst::Bin> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_stream_get_joined_bin(self.as_ref().to_glib_none().0))
        }
    }

    fn get_mtu(&self) -> u32 {
        unsafe {
            ffi::gst_rtsp_stream_get_mtu(self.as_ref().to_glib_none().0)
        }
    }

    fn get_multicast_address(&self, family: gio::SocketFamily) -> Option<RTSPAddress> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_stream_get_multicast_address(self.as_ref().to_glib_none().0, family.to_glib()))
        }
    }

    fn get_multicast_iface(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_stream_get_multicast_iface(self.as_ref().to_glib_none().0))
        }
    }

    fn get_profiles(&self) -> gst_rtsp::RTSPProfile {
        unsafe {
            from_glib(ffi::gst_rtsp_stream_get_profiles(self.as_ref().to_glib_none().0))
        }
    }

    fn get_protocols(&self) -> gst_rtsp::RTSPLowerTrans {
        unsafe {
            from_glib(ffi::gst_rtsp_stream_get_protocols(self.as_ref().to_glib_none().0))
        }
    }

    fn get_pt(&self) -> u32 {
        unsafe {
            ffi::gst_rtsp_stream_get_pt(self.as_ref().to_glib_none().0)
        }
    }

    fn get_publish_clock_mode(&self) -> RTSPPublishClockMode {
        unsafe {
            from_glib(ffi::gst_rtsp_stream_get_publish_clock_mode(self.as_ref().to_glib_none().0))
        }
    }

    fn get_retransmission_pt(&self) -> u32 {
        unsafe {
            ffi::gst_rtsp_stream_get_retransmission_pt(self.as_ref().to_glib_none().0)
        }
    }

    fn get_retransmission_time(&self) -> gst::ClockTime {
        unsafe {
            from_glib(ffi::gst_rtsp_stream_get_retransmission_time(self.as_ref().to_glib_none().0))
        }
    }

    fn get_rtcp_multicast_socket(&self, family: gio::SocketFamily) -> Option<gio::Socket> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_stream_get_rtcp_multicast_socket(self.as_ref().to_glib_none().0, family.to_glib()))
        }
    }

    fn get_rtcp_socket(&self, family: gio::SocketFamily) -> Option<gio::Socket> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_stream_get_rtcp_socket(self.as_ref().to_glib_none().0, family.to_glib()))
        }
    }

    fn get_rtp_multicast_socket(&self, family: gio::SocketFamily) -> Option<gio::Socket> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_stream_get_rtp_multicast_socket(self.as_ref().to_glib_none().0, family.to_glib()))
        }
    }

    fn get_rtp_socket(&self, family: gio::SocketFamily) -> Option<gio::Socket> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_stream_get_rtp_socket(self.as_ref().to_glib_none().0, family.to_glib()))
        }
    }

    fn get_rtpinfo(&self) -> Option<(u32, u32, u32, gst::ClockTime)> {
        unsafe {
            let mut rtptime = mem::uninitialized();
            let mut seq = mem::uninitialized();
            let mut clock_rate = mem::uninitialized();
            let mut running_time = mem::uninitialized();
            let ret = from_glib(ffi::gst_rtsp_stream_get_rtpinfo(self.as_ref().to_glib_none().0, &mut rtptime, &mut seq, &mut clock_rate, &mut running_time));
            if ret { Some((rtptime, seq, clock_rate, from_glib(running_time))) } else { None }
        }
    }

    fn get_rtpsession(&self) -> Option<glib::Object> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_stream_get_rtpsession(self.as_ref().to_glib_none().0))
        }
    }

    //fn get_server_port(&self, server_port: /*Ignored*/gst_rtsp::RTSPRange, family: gio::SocketFamily) {
    //    unsafe { TODO: call ffi::gst_rtsp_stream_get_server_port() }
    //}

    fn get_sinkpad(&self) -> Option<gst::Pad> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_stream_get_sinkpad(self.as_ref().to_glib_none().0))
        }
    }

    fn get_srcpad(&self) -> Option<gst::Pad> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_stream_get_srcpad(self.as_ref().to_glib_none().0))
        }
    }

    fn get_srtp_encoder(&self) -> Option<gst::Element> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_stream_get_srtp_encoder(self.as_ref().to_glib_none().0))
        }
    }

    fn get_ssrc(&self) -> u32 {
        unsafe {
            let mut ssrc = mem::uninitialized();
            ffi::gst_rtsp_stream_get_ssrc(self.as_ref().to_glib_none().0, &mut ssrc);
            ssrc
        }
    }

    fn has_control<'a, P: Into<Option<&'a str>>>(&self, control: P) -> bool {
        let control = control.into();
        unsafe {
            from_glib(ffi::gst_rtsp_stream_has_control(self.as_ref().to_glib_none().0, control.to_glib_none().0))
        }
    }

    fn is_blocking(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_rtsp_stream_is_blocking(self.as_ref().to_glib_none().0))
        }
    }

    fn is_client_side(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_rtsp_stream_is_client_side(self.as_ref().to_glib_none().0))
        }
    }

    fn is_complete(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_rtsp_stream_is_complete(self.as_ref().to_glib_none().0))
        }
    }

    fn is_receiver(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_rtsp_stream_is_receiver(self.as_ref().to_glib_none().0))
        }
    }

    fn is_sender(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_rtsp_stream_is_sender(self.as_ref().to_glib_none().0))
        }
    }

    //fn is_transport_supported(&self, transport: /*Ignored*/&mut gst_rtsp::RTSPTransport) -> bool {
    //    unsafe { TODO: call ffi::gst_rtsp_stream_is_transport_supported() }
    //}

    fn join_bin<P: IsA<gst::Bin>, Q: IsA<gst::Element>>(&self, bin: &P, rtpbin: &Q, state: gst::State) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(ffi::gst_rtsp_stream_join_bin(self.as_ref().to_glib_none().0, bin.as_ref().to_glib_none().0, rtpbin.as_ref().to_glib_none().0, state.to_glib()), "Failed to join bin")
        }
    }

    fn leave_bin<P: IsA<gst::Bin>, Q: IsA<gst::Element>>(&self, bin: &P, rtpbin: &Q) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(ffi::gst_rtsp_stream_leave_bin(self.as_ref().to_glib_none().0, bin.as_ref().to_glib_none().0, rtpbin.as_ref().to_glib_none().0), "Failed to leave bin")
        }
    }

    fn remove_transport<P: IsA<RTSPStreamTransport>>(&self, trans: &P) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(ffi::gst_rtsp_stream_remove_transport(self.as_ref().to_glib_none().0, trans.as_ref().to_glib_none().0), "Failed to remove transport")
        }
    }

    fn request_aux_sender(&self, sessid: u32) -> Option<gst::Element> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_stream_request_aux_sender(self.as_ref().to_glib_none().0, sessid))
        }
    }

    fn reserve_address(&self, address: &str, port: u32, n_ports: u32, ttl: u32) -> Option<RTSPAddress> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_stream_reserve_address(self.as_ref().to_glib_none().0, address.to_glib_none().0, port, n_ports, ttl))
        }
    }

    fn seekable(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_rtsp_stream_seekable(self.as_ref().to_glib_none().0))
        }
    }

    fn set_address_pool<'a, P: IsA<RTSPAddressPool> + 'a, Q: Into<Option<&'a P>>>(&self, pool: Q) {
        let pool = pool.into();
        unsafe {
            ffi::gst_rtsp_stream_set_address_pool(self.as_ref().to_glib_none().0, pool.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    fn set_blocked(&self, blocked: bool) -> bool {
        unsafe {
            from_glib(ffi::gst_rtsp_stream_set_blocked(self.as_ref().to_glib_none().0, blocked.to_glib()))
        }
    }

    fn set_buffer_size(&self, size: u32) {
        unsafe {
            ffi::gst_rtsp_stream_set_buffer_size(self.as_ref().to_glib_none().0, size);
        }
    }

    fn set_client_side(&self, client_side: bool) {
        unsafe {
            ffi::gst_rtsp_stream_set_client_side(self.as_ref().to_glib_none().0, client_side.to_glib());
        }
    }

    fn set_control<'a, P: Into<Option<&'a str>>>(&self, control: P) {
        let control = control.into();
        unsafe {
            ffi::gst_rtsp_stream_set_control(self.as_ref().to_glib_none().0, control.to_glib_none().0);
        }
    }

    fn set_dscp_qos(&self, dscp_qos: i32) {
        unsafe {
            ffi::gst_rtsp_stream_set_dscp_qos(self.as_ref().to_glib_none().0, dscp_qos);
        }
    }

    fn set_mtu(&self, mtu: u32) {
        unsafe {
            ffi::gst_rtsp_stream_set_mtu(self.as_ref().to_glib_none().0, mtu);
        }
    }

    fn set_multicast_iface<'a, P: Into<Option<&'a str>>>(&self, multicast_iface: P) {
        let multicast_iface = multicast_iface.into();
        unsafe {
            ffi::gst_rtsp_stream_set_multicast_iface(self.as_ref().to_glib_none().0, multicast_iface.to_glib_none().0);
        }
    }

    fn set_profiles(&self, profiles: gst_rtsp::RTSPProfile) {
        unsafe {
            ffi::gst_rtsp_stream_set_profiles(self.as_ref().to_glib_none().0, profiles.to_glib());
        }
    }

    fn set_protocols(&self, protocols: gst_rtsp::RTSPLowerTrans) {
        unsafe {
            ffi::gst_rtsp_stream_set_protocols(self.as_ref().to_glib_none().0, protocols.to_glib());
        }
    }

    fn set_pt_map(&self, pt: u32, caps: &gst::Caps) {
        unsafe {
            ffi::gst_rtsp_stream_set_pt_map(self.as_ref().to_glib_none().0, pt, caps.to_glib_none().0);
        }
    }

    fn set_publish_clock_mode(&self, mode: RTSPPublishClockMode) {
        unsafe {
            ffi::gst_rtsp_stream_set_publish_clock_mode(self.as_ref().to_glib_none().0, mode.to_glib());
        }
    }

    fn set_retransmission_pt(&self, rtx_pt: u32) {
        unsafe {
            ffi::gst_rtsp_stream_set_retransmission_pt(self.as_ref().to_glib_none().0, rtx_pt);
        }
    }

    fn set_retransmission_time(&self, time: gst::ClockTime) {
        unsafe {
            ffi::gst_rtsp_stream_set_retransmission_time(self.as_ref().to_glib_none().0, time.to_glib());
        }
    }

    fn set_seqnum_offset(&self, seqnum: u16) {
        unsafe {
            ffi::gst_rtsp_stream_set_seqnum_offset(self.as_ref().to_glib_none().0, seqnum);
        }
    }

    //fn transport_filter<'a, P: Into<Option<&'a /*Unimplemented*/RTSPStreamTransportFilterFunc>>, Q: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, func: P, user_data: Q) -> Vec<RTSPStreamTransport> {
    //    unsafe { TODO: call ffi::gst_rtsp_stream_transport_filter() }
    //}

    fn unblock_linked(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_rtsp_stream_unblock_linked(self.as_ref().to_glib_none().0))
        }
    }

    fn update_crypto<'a, P: Into<Option<&'a gst::Caps>>>(&self, ssrc: u32, crypto: P) -> bool {
        let crypto = crypto.into();
        unsafe {
            from_glib(ffi::gst_rtsp_stream_update_crypto(self.as_ref().to_glib_none().0, ssrc, crypto.to_glib_none().0))
        }
    }

    fn connect_new_rtcp_encoder<F: Fn(&Self, &gst::Element) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &gst::Element) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"new-rtcp-encoder\0".as_ptr() as *const _,
                transmute(new_rtcp_encoder_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_new_rtp_encoder<F: Fn(&Self, &gst::Element) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &gst::Element) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"new-rtp-encoder\0".as_ptr() as *const _,
                transmute(new_rtp_encoder_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_control_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::control\0".as_ptr() as *const _,
                transmute(notify_control_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_profiles_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::profiles\0".as_ptr() as *const _,
                transmute(notify_profiles_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_protocols_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::protocols\0".as_ptr() as *const _,
                transmute(notify_protocols_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn new_rtcp_encoder_trampoline<P>(this: *mut ffi::GstRTSPStream, object: *mut gst_ffi::GstElement, f: glib_ffi::gpointer)
where P: IsA<RTSPStream> {
    let f: &&(Fn(&P, &gst::Element) + Send + Sync + 'static) = transmute(f);
    f(&RTSPStream::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(object))
}

unsafe extern "C" fn new_rtp_encoder_trampoline<P>(this: *mut ffi::GstRTSPStream, object: *mut gst_ffi::GstElement, f: glib_ffi::gpointer)
where P: IsA<RTSPStream> {
    let f: &&(Fn(&P, &gst::Element) + Send + Sync + 'static) = transmute(f);
    f(&RTSPStream::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(object))
}

unsafe extern "C" fn notify_control_trampoline<P>(this: *mut ffi::GstRTSPStream, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<RTSPStream> {
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&RTSPStream::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_profiles_trampoline<P>(this: *mut ffi::GstRTSPStream, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<RTSPStream> {
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&RTSPStream::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_protocols_trampoline<P>(this: *mut ffi::GstRTSPStream, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<RTSPStream> {
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&RTSPStream::from_glib_borrow(this).unsafe_cast())
}
