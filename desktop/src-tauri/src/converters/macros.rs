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
