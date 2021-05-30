/*!
# `Refract` - Error
*/

#[cfg(feature = "cli")] use argyle::ArgyleError;
use crate::ImageKind;
use std::{
	error::Error,
	fmt,
};



#[allow(missing_docs)]
#[derive(Debug, Copy, Clone)]
/// # Errors.
pub enum RefractError {
	Color,
	Decode,
	Encode,
	Image,
	ImageDecode(ImageKind),
	ImageEncode(ImageKind),
	NoBest(ImageKind),
	NothingDoing,
	Overflow,
	TooBig,

	#[cfg(feature = "cli")]
	/// # Passthrough menu error.
	Menu(ArgyleError),

	#[cfg(feature = "cli")]
	NoCompression,

	#[cfg(any(feature = "cli", feature = "gtk"))]
	NoEncoders,

	#[cfg(feature = "cli")]
	NoImages,

	#[cfg(any(feature = "cli", feature = "gtk"))]
	Read,

	#[cfg(any(feature = "cli", feature = "gtk"))]
	Write,

	#[cfg(feature = "gtk")]
	AlreadyRunning,

	#[cfg(feature = "gtk")]
	GtkInit,

	#[cfg(feature = "gtk")]
	MissingSource,
}

impl Error for RefractError {}

impl AsRef<str> for RefractError {
	#[inline]
	fn as_ref(&self) -> &str { self.as_str() }
}

impl fmt::Display for RefractError {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl RefractError {
	#[must_use]
	/// # As Str.
	///
	/// Return the error as an English string slice.
	pub const fn as_str(self) -> &'static str {
		match self {
			Self::Color => "Unsupported color encoding.",
			Self::Decode => "The image could not be decoded.",
			Self::Encode => "The image could not be encoded.",
			Self::Image => "Invalid image.",
			Self::ImageDecode(k) => match k {
				ImageKind::Avif => "Refract cannot decode AVIF images.",
				ImageKind::Jxl => "Refract cannot decode JPEG XL images.",
				ImageKind::Webp => "Refract cannot decode WebP images.",
				_ => "",
			},
			Self::ImageEncode(k) => match k {
				ImageKind::Jpeg => "Refract cannot encode JPEG files.",
				ImageKind::Png => "Refract cannot encode PNG files.",
				_ => "",
			},
			Self::NoBest(k) => match k {
				ImageKind::Avif => "No acceptable AVIF candidate was found.",
				ImageKind::Jxl => "No acceptable JPEG XL candidate was found.",
				ImageKind::Webp => "No acceptable WebP candidate was found.",
				_ => "",
			},
			Self::NothingDoing => "There is nothing else to do.",
			Self::Overflow => "The image dimensions are out of range.",
			Self::TooBig => "The encoded image was too big.",

			#[cfg(feature = "cli")]
			Self::Menu(e) => e.as_str(),

			#[cfg(feature = "cli")]
			Self::NoCompression => "Lossless and lossy encoding cannot both be disabled.",

			#[cfg(any(feature = "cli", feature = "gtk"))]
			Self::NoEncoders => "At least one encoder must be enabled.",

			#[cfg(feature = "cli")]
			Self::NoImages => "No images were found.",

			#[cfg(any(feature = "cli", feature = "gtk"))]
			Self::Read => "Unable to read the source file.",

			#[cfg(any(feature = "cli", feature = "gtk"))]
			Self::Write => "Unable to save the file.",

			#[cfg(feature = "gtk")]
			Self::AlreadyRunning => "The encoder is already running.",

			#[cfg(feature = "gtk")]
			Self::GtkInit => "Failed to initialize GTK.",

			#[cfg(feature = "gtk")]
			Self::MissingSource => "A source image must be set before a candidate image.",
		}
	}
}
