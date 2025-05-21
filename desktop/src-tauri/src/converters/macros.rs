/// Defines the FileFormat enum and implements helper methods.
///
/// Usage:
/// define_file_formats! {
///     // Video Formats
///     MediaType::Video => {
///         MP4("mp4"), WebM("webm"), GIF("gif"), AVI("avi"), MKV("mkv"),
///         WMV("wmv"), MOV("mov"), MTS("mts"), FLV("flv"), OGV("ogv"),
///     },
///     // Audio Formats
///     MediaType::Audio => {
///         MP3("mp3"), WAV("wav"), FLAC("flac"), OGG("ogg"), AAC("aac"),
///         M4A("m4a"), OPUS("opus"),
///     },
///     // Image Formats
///     MediaType::Image => {
///         // Note: JPG and JPEG map to the same extension
///         JPG("jpg"), JPEG("jpg"), PNG("png"), WEBP("webp"), BMP("bmp"),
///         TIFF("tiff"), AVIF("avif"), ICO("ico"),
///     },
///     // Document Formats
///     MediaType::Document => {
///         PDF("pdf"), DOCX("docx"), ODT("odt"), TXT("txt"), HTML("html"),
///         MD("md"), EPUB("epub"),
///     }
///     // Add other MediaType groups as needed
/// }
#[macro_export]
macro_rules! define_file_formats {
    ( $( $media_type:path => { $( $variant:ident($extension:literal) ),* $(,)? } ),* $(,)? ) => {

        // use serde::{Deserialize, Serialize};
        // use $crate::converters::; // Assuming MediaType is in converters module

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum_macros::EnumString, strum_macros::Display, Serialize, Deserialize)]
        #[strum(serialize_all = "lowercase")]
        pub enum FileFormat {
            $( // Repeat for each media type block
                $( // Repeat for each format variant within the block
                    $variant,
                )*
            )*
        }

        impl FileFormat {
            /// Determines the general media type of the format.
            pub fn media_type(&self) -> $crate::converters::MediaKind {
                match self {
                    $( // Repeat for each media type block
                        $( // Repeat for each format variant
                            FileFormat::$variant => $media_type,
                        )*
                    )*
                }
            }

            /// Provides a typical file extension (lowercase, without dot).
            pub fn default_extension(&self) -> &'static str {
                match self {
                    $( // Repeat for each media type block
                        $( // Repeat for each format variant
                           FileFormat::$variant => $extension,
                        )*
                    )*
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_try_from_conversion_options {
    // $TargetType: The specific options struct (e.g., VideoConversionOptions)
    // $Variant: The enum variant in ConversionOptions (e.g., Video)
    ($TargetType:ty, $Variant:ident) => {
        impl TryFrom<ConversionOptions> for $TargetType {
            type Error = $crate::converters::error::ConverterError; // Assuming ConverterError is at this path

            fn try_from(value: ConversionOptions) -> Result<Self, Self::Error> {
                match value {
                    // If the enum variant matches what we expect for this $TargetType
                    $crate::converters::options::ConversionOptions::$Variant(opts) => Ok(opts),
                    // Otherwise, it's an invalid options type for this conversion
                    _other => Err(
                        $crate::converters::error::ConverterError::InternalInvalidOptions(format!(
                            "Expected {} options but got a different type.",
                            stringify!($Variant) // Turns the variant identifier into a string
                        )),
                    ),
                }
            }
        }
    };
}
