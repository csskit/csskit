use super::prelude::*;
use crate::units::{CSSFloat, CSSInt};

keyword_set!(pub enum MsHighContrastMediaFeatureKeyword { None: "none", Active: "active" });

discrete_feature!(
	#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub enum MsHighContrastMediaFeature<"-ms-high-contrast", MsHighContrastMediaFeatureKeyword>
);

keyword_set!(pub enum MsViewStateMediaFeatureKeyword {
	Snapped: "snapped",
	FullscreenPortait: "fullscreen-portrait",
	FullscreenLandscape: "fullscreen-landscape",
});

discrete_feature!(
	#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub enum MsViewStateMediaFeature<"-ms-view-state", MsViewStateMediaFeatureKeyword>
);

keyword_set!(pub enum MsImeAlignMediaFeatureKeyword { Auto: "auto" });

discrete_feature!(
	#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub enum MsImeAlignMediaFeature<"-ms-ime-align", MsImeAlignMediaFeatureKeyword>
);

keyword_set!(pub enum MsDevicePixelRatioMediaFeatureKeyword {
	DevicePixelRatio: "-ms-device-pixel-ratio",
	MaxDevicePixelRatio: "-ms-max-device-pixel-ratio",
	MinDevicePixelRatio: "-ms-min-device-pixel-ratio",
});

impl RangedFeatureKeyword for MsDevicePixelRatioMediaFeatureKeyword {
	fn is_legacy(&self) -> bool {
		matches!(self, Self::MaxDevicePixelRatio(_) | Self::MinDevicePixelRatio(_))
	}
}

ranged_feature!(
	#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub enum MsDevicePixelRatioMediaFeature<MsDevicePixelRatioMediaFeatureKeyword, CSSFloat>
);

keyword_set!(pub enum MsColumnCountMediaFeatureKeyword {
	ColumnCount: "-ms-column-count",
	MaxColumnCount: "-ms-max-column-count",
	MinColumnCount: "-ms-min-column-count",
});

impl RangedFeatureKeyword for MsColumnCountMediaFeatureKeyword {
	fn is_legacy(&self) -> bool {
		matches!(self, Self::MaxColumnCount(_) | Self::MinColumnCount(_))
	}
}

ranged_feature!(
	#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub enum MsColumnCountMediaFeature<MsColumnCountMediaFeatureKeyword, CSSInt>
);
