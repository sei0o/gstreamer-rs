// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

mod video_filter;
pub use self::video_filter::{VideoFilter, VideoFilterClass, NONE_VIDEO_FILTER};

mod video_overlay;
pub use self::video_overlay::{VideoOverlay, NONE_VIDEO_OVERLAY};
pub use self::video_overlay::VideoOverlayExt;

mod enums;
pub use self::enums::VideoColorMatrix;
pub use self::enums::VideoColorPrimaries;
#[cfg(any(feature = "v1_12", feature = "dox"))]
pub use self::enums::VideoFieldOrder;
pub use self::enums::VideoFormat;
pub use self::enums::VideoInterlaceMode;
pub use self::enums::VideoMultiviewFramePacking;
pub use self::enums::VideoMultiviewMode;
pub use self::enums::VideoTileMode;
pub use self::enums::VideoTransferFunction;

mod flags;
pub use self::flags::VideoChromaSite;
pub use self::flags::VideoFlags;
pub use self::flags::VideoFormatFlags;
pub use self::flags::VideoFrameFlags;
pub use self::flags::VideoMultiviewFlags;
pub use self::flags::VideoOverlayFormatFlags;
#[cfg(any(feature = "v1_10", feature = "dox"))]
pub use self::flags::VideoTimeCodeFlags;

#[doc(hidden)]
pub mod traits {
    pub use super::VideoOverlayExt;
}
