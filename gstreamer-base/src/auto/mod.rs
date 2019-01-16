// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

mod adapter;
pub use self::adapter::{Adapter, AdapterClass, NONE_ADAPTER};

#[cfg(any(feature = "v1_14", feature = "dox"))]
mod aggregator;
#[cfg(any(feature = "v1_14", feature = "dox"))]
pub use self::aggregator::{Aggregator, AggregatorClass, NONE_AGGREGATOR};
#[cfg(any(feature = "v1_14", feature = "dox"))]
pub use self::aggregator::AggregatorExt;

#[cfg(any(feature = "v1_14", feature = "dox"))]
mod aggregator_pad;
#[cfg(any(feature = "v1_14", feature = "dox"))]
pub use self::aggregator_pad::{AggregatorPad, AggregatorPadClass, NONE_AGGREGATOR_PAD};
#[cfg(any(feature = "v1_14", feature = "dox"))]
pub use self::aggregator_pad::AggregatorPadExt;

mod base_sink;
pub use self::base_sink::{BaseSink, BaseSinkClass, NONE_BASE_SINK};
pub use self::base_sink::BaseSinkExt;

mod base_src;
pub use self::base_src::{BaseSrc, BaseSrcClass, NONE_BASE_SRC};
pub use self::base_src::BaseSrcExt;

mod base_transform;
pub use self::base_transform::{BaseTransform, BaseTransformClass, NONE_BASE_TRANSFORM};
pub use self::base_transform::BaseTransformExt;

mod push_src;
pub use self::push_src::{PushSrc, PushSrcClass, NONE_PUSH_SRC};

pub mod functions;

#[doc(hidden)]
pub mod traits {
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub use super::AggregatorExt;
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub use super::AggregatorPadExt;
    pub use super::BaseSinkExt;
    pub use super::BaseSrcExt;
    pub use super::BaseTransformExt;
}
