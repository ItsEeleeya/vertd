mod converters {
    #![allow(unused)]
    #[macro_use]
    mod macros {}
    use anyhow::Result;
    use serde::{Deserialize, Serialize};
    use specta::Type;
    use std::any::Any;
    use std::fmt::Debug;
    use std::path::{Path, PathBuf};
    use std::sync::Arc;
    use strum::{Display, EnumString};
    #[strum(serialize_all = "lowercase")]
    pub enum MediaType {
        Video,
        Audio,
        Image,
        Document,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for MediaType {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    MediaType::Video => "Video",
                    MediaType::Audio => "Audio",
                    MediaType::Image => "Image",
                    MediaType::Document => "Document",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for MediaType {
        #[inline]
        fn clone(&self) -> MediaType {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for MediaType {}
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for MediaType {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for MediaType {
        #[inline]
        fn eq(&self, other: &MediaType) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for MediaType {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::hash::Hash for MediaType {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_discr, state)
        }
    }
    #[allow(clippy::use_self)]
    impl ::core::str::FromStr for MediaType {
        type Err = ::strum::ParseError;
        #[inline]
        fn from_str(
            s: &str,
        ) -> ::core::result::Result<MediaType, <Self as ::core::str::FromStr>::Err> {
            ::core::result::Result::Ok(
                match s {
                    "video" => MediaType::Video,
                    "audio" => MediaType::Audio,
                    "image" => MediaType::Image,
                    "document" => MediaType::Document,
                    _ => {
                        return ::core::result::Result::Err(
                            ::strum::ParseError::VariantNotFound,
                        );
                    }
                },
            )
        }
    }
    #[allow(clippy::use_self)]
    impl ::core::convert::TryFrom<&str> for MediaType {
        type Error = ::strum::ParseError;
        #[inline]
        fn try_from(
            s: &str,
        ) -> ::core::result::Result<
            MediaType,
            <Self as ::core::convert::TryFrom<&str>>::Error,
        > {
            ::core::str::FromStr::from_str(s)
        }
    }
    impl ::core::fmt::Display for MediaType {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match *self {
                MediaType::Video => ::core::fmt::Display::fmt("video", f),
                MediaType::Audio => ::core::fmt::Display::fmt("audio", f),
                MediaType::Image => ::core::fmt::Display::fmt("image", f),
                MediaType::Document => ::core::fmt::Display::fmt("document", f),
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for MediaType {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    MediaType::Video => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "MediaType",
                            0u32,
                            "Video",
                        )
                    }
                    MediaType::Audio => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "MediaType",
                            1u32,
                            "Audio",
                        )
                    }
                    MediaType::Image => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "MediaType",
                            2u32,
                            "Image",
                        )
                    }
                    MediaType::Document => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "MediaType",
                            3u32,
                            "Document",
                        )
                    }
                }
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for MediaType {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "variant identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::invalid_value(
                                        _serde::de::Unexpected::Unsigned(__value),
                                        &"variant index 0 <= i < 4",
                                    ),
                                )
                            }
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "Video" => _serde::__private::Ok(__Field::__field0),
                            "Audio" => _serde::__private::Ok(__Field::__field1),
                            "Image" => _serde::__private::Ok(__Field::__field2),
                            "Document" => _serde::__private::Ok(__Field::__field3),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"Video" => _serde::__private::Ok(__Field::__field0),
                            b"Audio" => _serde::__private::Ok(__Field::__field1),
                            b"Image" => _serde::__private::Ok(__Field::__field2),
                            b"Document" => _serde::__private::Ok(__Field::__field3),
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<MediaType>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = MediaType;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "enum MediaType",
                        )
                    }
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::EnumAccess<'de>,
                    {
                        match _serde::de::EnumAccess::variant(__data)? {
                            (__Field::__field0, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(MediaType::Video)
                            }
                            (__Field::__field1, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(MediaType::Audio)
                            }
                            (__Field::__field2, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(MediaType::Image)
                            }
                            (__Field::__field3, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(MediaType::Document)
                            }
                        }
                    }
                }
                #[doc(hidden)]
                const VARIANTS: &'static [&'static str] = &[
                    "Video",
                    "Audio",
                    "Image",
                    "Document",
                ];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "MediaType",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<MediaType>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    const _: () = {
        const IMPL_LOCATION: specta::ImplLocation = specta::internal::construct::impl_location(
            "desktop/src-tauri/src/converters/mod.rs:15:91",
        );
        const DEFINITION_GENERICS: &[specta::datatype::DataType] = &[];
        #[automatically_derived]
        impl specta::Type for MediaType {
            fn inline(
                type_map: &mut specta::TypeCollection,
                generics: specta::Generics,
            ) -> specta::datatype::DataType {
                let generics = match generics {
                    specta::Generics::Definition => DEFINITION_GENERICS,
                    specta::Generics::Provided(generics) => generics,
                };
                specta::datatype::DataType::Enum(
                    specta::internal::construct::r#enum(
                        "MediaType".into(),
                        specta::internal::construct::sid(
                            "MediaType",
                            "::vert_desktop_lib::converters:15:91",
                        ),
                        specta::datatype::EnumRepr::External,
                        false,
                        ::alloc::vec::Vec::new(),
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([
                                (
                                    "Video".into(),
                                    specta::internal::construct::enum_variant(
                                        false,
                                        None,
                                        "".into(),
                                        specta::internal::construct::enum_variant_unit(),
                                    ),
                                ),
                                (
                                    "Audio".into(),
                                    specta::internal::construct::enum_variant(
                                        false,
                                        None,
                                        "".into(),
                                        specta::internal::construct::enum_variant_unit(),
                                    ),
                                ),
                                (
                                    "Image".into(),
                                    specta::internal::construct::enum_variant(
                                        false,
                                        None,
                                        "".into(),
                                        specta::internal::construct::enum_variant_unit(),
                                    ),
                                ),
                                (
                                    "Document".into(),
                                    specta::internal::construct::enum_variant(
                                        false,
                                        None,
                                        "".into(),
                                        specta::internal::construct::enum_variant_unit(),
                                    ),
                                ),
                            ]),
                        ),
                    ),
                )
            }
            fn reference(
                type_map: &mut specta::TypeCollection,
                generics: &[specta::datatype::DataType],
            ) -> specta::datatype::reference::Reference {
                {
                    let generics = ::alloc::vec::Vec::new();
                    specta::datatype::reference::reference::<
                        Self,
                    >(
                        type_map,
                        specta::internal::construct::data_type_reference(
                            "MediaType".into(),
                            specta::internal::construct::sid(
                                "MediaType",
                                "::vert_desktop_lib::converters:15:91",
                            ),
                            generics,
                        ),
                    )
                }
            }
        }
        #[automatically_derived]
        impl specta::NamedType for MediaType {
            fn sid() -> specta::SpectaID {
                specta::internal::construct::sid(
                    "MediaType",
                    "::vert_desktop_lib::converters:15:91",
                )
            }
            fn named_data_type(
                type_map: &mut specta::TypeCollection,
                generics: &[specta::datatype::DataType],
            ) -> specta::datatype::NamedDataType {
                specta::internal::construct::named_data_type(
                    "MediaType".into(),
                    "".into(),
                    None,
                    Self::sid(),
                    IMPL_LOCATION,
                    <Self as specta::Type>::inline(
                        type_map,
                        specta::Generics::Provided(generics),
                    ),
                )
            }
            fn definition_named_data_type(
                type_map: &mut specta::TypeCollection,
            ) -> specta::datatype::NamedDataType {
                specta::internal::construct::named_data_type(
                    "MediaType".into(),
                    "".into(),
                    None,
                    Self::sid(),
                    IMPL_LOCATION,
                    <Self as specta::Type>::inline(
                        type_map,
                        specta::Generics::Definition,
                    ),
                )
            }
        }
    };
    #[strum(serialize_all = "lowercase")]
    pub enum FileFormat {
        MP4,
        WebM,
        GIF,
        AVI,
        MKV,
        WMV,
        MOV,
        MTS,
        FLV,
        OGV,
        MP3,
        WAV,
        FLAC,
        OGG,
        AAC,
        M4A,
        OPUS,
        JPG,
        JPEG,
        PNG,
        WEBP,
        BMP,
        TIFF,
        AVIF,
        ICO,
        PDF,
        DOCX,
        ODT,
        TXT,
        HTML,
        MD,
        EPUB,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for FileFormat {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    FileFormat::MP4 => "MP4",
                    FileFormat::WebM => "WebM",
                    FileFormat::GIF => "GIF",
                    FileFormat::AVI => "AVI",
                    FileFormat::MKV => "MKV",
                    FileFormat::WMV => "WMV",
                    FileFormat::MOV => "MOV",
                    FileFormat::MTS => "MTS",
                    FileFormat::FLV => "FLV",
                    FileFormat::OGV => "OGV",
                    FileFormat::MP3 => "MP3",
                    FileFormat::WAV => "WAV",
                    FileFormat::FLAC => "FLAC",
                    FileFormat::OGG => "OGG",
                    FileFormat::AAC => "AAC",
                    FileFormat::M4A => "M4A",
                    FileFormat::OPUS => "OPUS",
                    FileFormat::JPG => "JPG",
                    FileFormat::JPEG => "JPEG",
                    FileFormat::PNG => "PNG",
                    FileFormat::WEBP => "WEBP",
                    FileFormat::BMP => "BMP",
                    FileFormat::TIFF => "TIFF",
                    FileFormat::AVIF => "AVIF",
                    FileFormat::ICO => "ICO",
                    FileFormat::PDF => "PDF",
                    FileFormat::DOCX => "DOCX",
                    FileFormat::ODT => "ODT",
                    FileFormat::TXT => "TXT",
                    FileFormat::HTML => "HTML",
                    FileFormat::MD => "MD",
                    FileFormat::EPUB => "EPUB",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for FileFormat {
        #[inline]
        fn clone(&self) -> FileFormat {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for FileFormat {}
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for FileFormat {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for FileFormat {
        #[inline]
        fn eq(&self, other: &FileFormat) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for FileFormat {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::hash::Hash for FileFormat {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_discr, state)
        }
    }
    #[allow(clippy::use_self)]
    impl ::core::str::FromStr for FileFormat {
        type Err = ::strum::ParseError;
        #[inline]
        fn from_str(
            s: &str,
        ) -> ::core::result::Result<FileFormat, <Self as ::core::str::FromStr>::Err> {
            ::core::result::Result::Ok(
                match s {
                    "mp4" => FileFormat::MP4,
                    "webm" => FileFormat::WebM,
                    "gif" => FileFormat::GIF,
                    "avi" => FileFormat::AVI,
                    "mkv" => FileFormat::MKV,
                    "wmv" => FileFormat::WMV,
                    "mov" => FileFormat::MOV,
                    "mts" => FileFormat::MTS,
                    "flv" => FileFormat::FLV,
                    "ogv" => FileFormat::OGV,
                    "mp3" => FileFormat::MP3,
                    "wav" => FileFormat::WAV,
                    "flac" => FileFormat::FLAC,
                    "ogg" => FileFormat::OGG,
                    "aac" => FileFormat::AAC,
                    "m4a" => FileFormat::M4A,
                    "opus" => FileFormat::OPUS,
                    "jpg" => FileFormat::JPG,
                    "jpeg" => FileFormat::JPEG,
                    "png" => FileFormat::PNG,
                    "webp" => FileFormat::WEBP,
                    "bmp" => FileFormat::BMP,
                    "tiff" => FileFormat::TIFF,
                    "avif" => FileFormat::AVIF,
                    "ico" => FileFormat::ICO,
                    "pdf" => FileFormat::PDF,
                    "docx" => FileFormat::DOCX,
                    "odt" => FileFormat::ODT,
                    "txt" => FileFormat::TXT,
                    "html" => FileFormat::HTML,
                    "md" => FileFormat::MD,
                    "epub" => FileFormat::EPUB,
                    _ => {
                        return ::core::result::Result::Err(
                            ::strum::ParseError::VariantNotFound,
                        );
                    }
                },
            )
        }
    }
    #[allow(clippy::use_self)]
    impl ::core::convert::TryFrom<&str> for FileFormat {
        type Error = ::strum::ParseError;
        #[inline]
        fn try_from(
            s: &str,
        ) -> ::core::result::Result<
            FileFormat,
            <Self as ::core::convert::TryFrom<&str>>::Error,
        > {
            ::core::str::FromStr::from_str(s)
        }
    }
    impl ::core::fmt::Display for FileFormat {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match *self {
                FileFormat::MP4 => ::core::fmt::Display::fmt("mp4", f),
                FileFormat::WebM => ::core::fmt::Display::fmt("webm", f),
                FileFormat::GIF => ::core::fmt::Display::fmt("gif", f),
                FileFormat::AVI => ::core::fmt::Display::fmt("avi", f),
                FileFormat::MKV => ::core::fmt::Display::fmt("mkv", f),
                FileFormat::WMV => ::core::fmt::Display::fmt("wmv", f),
                FileFormat::MOV => ::core::fmt::Display::fmt("mov", f),
                FileFormat::MTS => ::core::fmt::Display::fmt("mts", f),
                FileFormat::FLV => ::core::fmt::Display::fmt("flv", f),
                FileFormat::OGV => ::core::fmt::Display::fmt("ogv", f),
                FileFormat::MP3 => ::core::fmt::Display::fmt("mp3", f),
                FileFormat::WAV => ::core::fmt::Display::fmt("wav", f),
                FileFormat::FLAC => ::core::fmt::Display::fmt("flac", f),
                FileFormat::OGG => ::core::fmt::Display::fmt("ogg", f),
                FileFormat::AAC => ::core::fmt::Display::fmt("aac", f),
                FileFormat::M4A => ::core::fmt::Display::fmt("m4a", f),
                FileFormat::OPUS => ::core::fmt::Display::fmt("opus", f),
                FileFormat::JPG => ::core::fmt::Display::fmt("jpg", f),
                FileFormat::JPEG => ::core::fmt::Display::fmt("jpeg", f),
                FileFormat::PNG => ::core::fmt::Display::fmt("png", f),
                FileFormat::WEBP => ::core::fmt::Display::fmt("webp", f),
                FileFormat::BMP => ::core::fmt::Display::fmt("bmp", f),
                FileFormat::TIFF => ::core::fmt::Display::fmt("tiff", f),
                FileFormat::AVIF => ::core::fmt::Display::fmt("avif", f),
                FileFormat::ICO => ::core::fmt::Display::fmt("ico", f),
                FileFormat::PDF => ::core::fmt::Display::fmt("pdf", f),
                FileFormat::DOCX => ::core::fmt::Display::fmt("docx", f),
                FileFormat::ODT => ::core::fmt::Display::fmt("odt", f),
                FileFormat::TXT => ::core::fmt::Display::fmt("txt", f),
                FileFormat::HTML => ::core::fmt::Display::fmt("html", f),
                FileFormat::MD => ::core::fmt::Display::fmt("md", f),
                FileFormat::EPUB => ::core::fmt::Display::fmt("epub", f),
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for FileFormat {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    FileFormat::MP4 => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "FileFormat",
                            0u32,
                            "MP4",
                        )
                    }
                    FileFormat::WebM => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "FileFormat",
                            1u32,
                            "WebM",
                        )
                    }
                    FileFormat::GIF => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "FileFormat",
                            2u32,
                            "GIF",
                        )
                    }
                    FileFormat::AVI => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "FileFormat",
                            3u32,
                            "AVI",
                        )
                    }
                    FileFormat::MKV => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "FileFormat",
                            4u32,
                            "MKV",
                        )
                    }
                    FileFormat::WMV => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "FileFormat",
                            5u32,
                            "WMV",
                        )
                    }
                    FileFormat::MOV => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "FileFormat",
                            6u32,
                            "MOV",
                        )
                    }
                    FileFormat::MTS => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "FileFormat",
                            7u32,
                            "MTS",
                        )
                    }
                    FileFormat::FLV => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "FileFormat",
                            8u32,
                            "FLV",
                        )
                    }
                    FileFormat::OGV => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "FileFormat",
                            9u32,
                            "OGV",
                        )
                    }
                    FileFormat::MP3 => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "FileFormat",
                            10u32,
                            "MP3",
                        )
                    }
                    FileFormat::WAV => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "FileFormat",
                            11u32,
                            "WAV",
                        )
                    }
                    FileFormat::FLAC => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "FileFormat",
                            12u32,
                            "FLAC",
                        )
                    }
                    FileFormat::OGG => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "FileFormat",
                            13u32,
                            "OGG",
                        )
                    }
                    FileFormat::AAC => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "FileFormat",
                            14u32,
                            "AAC",
                        )
                    }
                    FileFormat::M4A => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "FileFormat",
                            15u32,
                            "M4A",
                        )
                    }
                    FileFormat::OPUS => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "FileFormat",
                            16u32,
                            "OPUS",
                        )
                    }
                    FileFormat::JPG => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "FileFormat",
                            17u32,
                            "JPG",
                        )
                    }
                    FileFormat::JPEG => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "FileFormat",
                            18u32,
                            "JPEG",
                        )
                    }
                    FileFormat::PNG => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "FileFormat",
                            19u32,
                            "PNG",
                        )
                    }
                    FileFormat::WEBP => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "FileFormat",
                            20u32,
                            "WEBP",
                        )
                    }
                    FileFormat::BMP => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "FileFormat",
                            21u32,
                            "BMP",
                        )
                    }
                    FileFormat::TIFF => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "FileFormat",
                            22u32,
                            "TIFF",
                        )
                    }
                    FileFormat::AVIF => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "FileFormat",
                            23u32,
                            "AVIF",
                        )
                    }
                    FileFormat::ICO => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "FileFormat",
                            24u32,
                            "ICO",
                        )
                    }
                    FileFormat::PDF => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "FileFormat",
                            25u32,
                            "PDF",
                        )
                    }
                    FileFormat::DOCX => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "FileFormat",
                            26u32,
                            "DOCX",
                        )
                    }
                    FileFormat::ODT => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "FileFormat",
                            27u32,
                            "ODT",
                        )
                    }
                    FileFormat::TXT => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "FileFormat",
                            28u32,
                            "TXT",
                        )
                    }
                    FileFormat::HTML => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "FileFormat",
                            29u32,
                            "HTML",
                        )
                    }
                    FileFormat::MD => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "FileFormat",
                            30u32,
                            "MD",
                        )
                    }
                    FileFormat::EPUB => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "FileFormat",
                            31u32,
                            "EPUB",
                        )
                    }
                }
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for FileFormat {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __field6,
                    __field7,
                    __field8,
                    __field9,
                    __field10,
                    __field11,
                    __field12,
                    __field13,
                    __field14,
                    __field15,
                    __field16,
                    __field17,
                    __field18,
                    __field19,
                    __field20,
                    __field21,
                    __field22,
                    __field23,
                    __field24,
                    __field25,
                    __field26,
                    __field27,
                    __field28,
                    __field29,
                    __field30,
                    __field31,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "variant identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            4u64 => _serde::__private::Ok(__Field::__field4),
                            5u64 => _serde::__private::Ok(__Field::__field5),
                            6u64 => _serde::__private::Ok(__Field::__field6),
                            7u64 => _serde::__private::Ok(__Field::__field7),
                            8u64 => _serde::__private::Ok(__Field::__field8),
                            9u64 => _serde::__private::Ok(__Field::__field9),
                            10u64 => _serde::__private::Ok(__Field::__field10),
                            11u64 => _serde::__private::Ok(__Field::__field11),
                            12u64 => _serde::__private::Ok(__Field::__field12),
                            13u64 => _serde::__private::Ok(__Field::__field13),
                            14u64 => _serde::__private::Ok(__Field::__field14),
                            15u64 => _serde::__private::Ok(__Field::__field15),
                            16u64 => _serde::__private::Ok(__Field::__field16),
                            17u64 => _serde::__private::Ok(__Field::__field17),
                            18u64 => _serde::__private::Ok(__Field::__field18),
                            19u64 => _serde::__private::Ok(__Field::__field19),
                            20u64 => _serde::__private::Ok(__Field::__field20),
                            21u64 => _serde::__private::Ok(__Field::__field21),
                            22u64 => _serde::__private::Ok(__Field::__field22),
                            23u64 => _serde::__private::Ok(__Field::__field23),
                            24u64 => _serde::__private::Ok(__Field::__field24),
                            25u64 => _serde::__private::Ok(__Field::__field25),
                            26u64 => _serde::__private::Ok(__Field::__field26),
                            27u64 => _serde::__private::Ok(__Field::__field27),
                            28u64 => _serde::__private::Ok(__Field::__field28),
                            29u64 => _serde::__private::Ok(__Field::__field29),
                            30u64 => _serde::__private::Ok(__Field::__field30),
                            31u64 => _serde::__private::Ok(__Field::__field31),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::invalid_value(
                                        _serde::de::Unexpected::Unsigned(__value),
                                        &"variant index 0 <= i < 32",
                                    ),
                                )
                            }
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "MP4" => _serde::__private::Ok(__Field::__field0),
                            "WebM" => _serde::__private::Ok(__Field::__field1),
                            "GIF" => _serde::__private::Ok(__Field::__field2),
                            "AVI" => _serde::__private::Ok(__Field::__field3),
                            "MKV" => _serde::__private::Ok(__Field::__field4),
                            "WMV" => _serde::__private::Ok(__Field::__field5),
                            "MOV" => _serde::__private::Ok(__Field::__field6),
                            "MTS" => _serde::__private::Ok(__Field::__field7),
                            "FLV" => _serde::__private::Ok(__Field::__field8),
                            "OGV" => _serde::__private::Ok(__Field::__field9),
                            "MP3" => _serde::__private::Ok(__Field::__field10),
                            "WAV" => _serde::__private::Ok(__Field::__field11),
                            "FLAC" => _serde::__private::Ok(__Field::__field12),
                            "OGG" => _serde::__private::Ok(__Field::__field13),
                            "AAC" => _serde::__private::Ok(__Field::__field14),
                            "M4A" => _serde::__private::Ok(__Field::__field15),
                            "OPUS" => _serde::__private::Ok(__Field::__field16),
                            "JPG" => _serde::__private::Ok(__Field::__field17),
                            "JPEG" => _serde::__private::Ok(__Field::__field18),
                            "PNG" => _serde::__private::Ok(__Field::__field19),
                            "WEBP" => _serde::__private::Ok(__Field::__field20),
                            "BMP" => _serde::__private::Ok(__Field::__field21),
                            "TIFF" => _serde::__private::Ok(__Field::__field22),
                            "AVIF" => _serde::__private::Ok(__Field::__field23),
                            "ICO" => _serde::__private::Ok(__Field::__field24),
                            "PDF" => _serde::__private::Ok(__Field::__field25),
                            "DOCX" => _serde::__private::Ok(__Field::__field26),
                            "ODT" => _serde::__private::Ok(__Field::__field27),
                            "TXT" => _serde::__private::Ok(__Field::__field28),
                            "HTML" => _serde::__private::Ok(__Field::__field29),
                            "MD" => _serde::__private::Ok(__Field::__field30),
                            "EPUB" => _serde::__private::Ok(__Field::__field31),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"MP4" => _serde::__private::Ok(__Field::__field0),
                            b"WebM" => _serde::__private::Ok(__Field::__field1),
                            b"GIF" => _serde::__private::Ok(__Field::__field2),
                            b"AVI" => _serde::__private::Ok(__Field::__field3),
                            b"MKV" => _serde::__private::Ok(__Field::__field4),
                            b"WMV" => _serde::__private::Ok(__Field::__field5),
                            b"MOV" => _serde::__private::Ok(__Field::__field6),
                            b"MTS" => _serde::__private::Ok(__Field::__field7),
                            b"FLV" => _serde::__private::Ok(__Field::__field8),
                            b"OGV" => _serde::__private::Ok(__Field::__field9),
                            b"MP3" => _serde::__private::Ok(__Field::__field10),
                            b"WAV" => _serde::__private::Ok(__Field::__field11),
                            b"FLAC" => _serde::__private::Ok(__Field::__field12),
                            b"OGG" => _serde::__private::Ok(__Field::__field13),
                            b"AAC" => _serde::__private::Ok(__Field::__field14),
                            b"M4A" => _serde::__private::Ok(__Field::__field15),
                            b"OPUS" => _serde::__private::Ok(__Field::__field16),
                            b"JPG" => _serde::__private::Ok(__Field::__field17),
                            b"JPEG" => _serde::__private::Ok(__Field::__field18),
                            b"PNG" => _serde::__private::Ok(__Field::__field19),
                            b"WEBP" => _serde::__private::Ok(__Field::__field20),
                            b"BMP" => _serde::__private::Ok(__Field::__field21),
                            b"TIFF" => _serde::__private::Ok(__Field::__field22),
                            b"AVIF" => _serde::__private::Ok(__Field::__field23),
                            b"ICO" => _serde::__private::Ok(__Field::__field24),
                            b"PDF" => _serde::__private::Ok(__Field::__field25),
                            b"DOCX" => _serde::__private::Ok(__Field::__field26),
                            b"ODT" => _serde::__private::Ok(__Field::__field27),
                            b"TXT" => _serde::__private::Ok(__Field::__field28),
                            b"HTML" => _serde::__private::Ok(__Field::__field29),
                            b"MD" => _serde::__private::Ok(__Field::__field30),
                            b"EPUB" => _serde::__private::Ok(__Field::__field31),
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<FileFormat>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = FileFormat;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "enum FileFormat",
                        )
                    }
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::EnumAccess<'de>,
                    {
                        match _serde::de::EnumAccess::variant(__data)? {
                            (__Field::__field0, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(FileFormat::MP4)
                            }
                            (__Field::__field1, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(FileFormat::WebM)
                            }
                            (__Field::__field2, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(FileFormat::GIF)
                            }
                            (__Field::__field3, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(FileFormat::AVI)
                            }
                            (__Field::__field4, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(FileFormat::MKV)
                            }
                            (__Field::__field5, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(FileFormat::WMV)
                            }
                            (__Field::__field6, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(FileFormat::MOV)
                            }
                            (__Field::__field7, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(FileFormat::MTS)
                            }
                            (__Field::__field8, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(FileFormat::FLV)
                            }
                            (__Field::__field9, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(FileFormat::OGV)
                            }
                            (__Field::__field10, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(FileFormat::MP3)
                            }
                            (__Field::__field11, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(FileFormat::WAV)
                            }
                            (__Field::__field12, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(FileFormat::FLAC)
                            }
                            (__Field::__field13, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(FileFormat::OGG)
                            }
                            (__Field::__field14, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(FileFormat::AAC)
                            }
                            (__Field::__field15, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(FileFormat::M4A)
                            }
                            (__Field::__field16, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(FileFormat::OPUS)
                            }
                            (__Field::__field17, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(FileFormat::JPG)
                            }
                            (__Field::__field18, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(FileFormat::JPEG)
                            }
                            (__Field::__field19, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(FileFormat::PNG)
                            }
                            (__Field::__field20, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(FileFormat::WEBP)
                            }
                            (__Field::__field21, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(FileFormat::BMP)
                            }
                            (__Field::__field22, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(FileFormat::TIFF)
                            }
                            (__Field::__field23, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(FileFormat::AVIF)
                            }
                            (__Field::__field24, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(FileFormat::ICO)
                            }
                            (__Field::__field25, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(FileFormat::PDF)
                            }
                            (__Field::__field26, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(FileFormat::DOCX)
                            }
                            (__Field::__field27, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(FileFormat::ODT)
                            }
                            (__Field::__field28, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(FileFormat::TXT)
                            }
                            (__Field::__field29, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(FileFormat::HTML)
                            }
                            (__Field::__field30, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(FileFormat::MD)
                            }
                            (__Field::__field31, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(FileFormat::EPUB)
                            }
                        }
                    }
                }
                #[doc(hidden)]
                const VARIANTS: &'static [&'static str] = &[
                    "MP4",
                    "WebM",
                    "GIF",
                    "AVI",
                    "MKV",
                    "WMV",
                    "MOV",
                    "MTS",
                    "FLV",
                    "OGV",
                    "MP3",
                    "WAV",
                    "FLAC",
                    "OGG",
                    "AAC",
                    "M4A",
                    "OPUS",
                    "JPG",
                    "JPEG",
                    "PNG",
                    "WEBP",
                    "BMP",
                    "TIFF",
                    "AVIF",
                    "ICO",
                    "PDF",
                    "DOCX",
                    "ODT",
                    "TXT",
                    "HTML",
                    "MD",
                    "EPUB",
                ];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "FileFormat",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<FileFormat>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl FileFormat {
        /// Determines the general media type of the format.
        pub fn media_type(&self) -> MediaType {
            match self {
                FileFormat::MP4 => MediaType::Video,
                FileFormat::WebM => MediaType::Video,
                FileFormat::GIF => MediaType::Video,
                FileFormat::AVI => MediaType::Video,
                FileFormat::MKV => MediaType::Video,
                FileFormat::WMV => MediaType::Video,
                FileFormat::MOV => MediaType::Video,
                FileFormat::MTS => MediaType::Video,
                FileFormat::FLV => MediaType::Video,
                FileFormat::OGV => MediaType::Video,
                FileFormat::MP3 => MediaType::Audio,
                FileFormat::WAV => MediaType::Audio,
                FileFormat::FLAC => MediaType::Audio,
                FileFormat::OGG => MediaType::Audio,
                FileFormat::AAC => MediaType::Audio,
                FileFormat::M4A => MediaType::Audio,
                FileFormat::OPUS => MediaType::Audio,
                FileFormat::JPG => MediaType::Image,
                FileFormat::JPEG => MediaType::Image,
                FileFormat::PNG => MediaType::Image,
                FileFormat::WEBP => MediaType::Image,
                FileFormat::BMP => MediaType::Image,
                FileFormat::TIFF => MediaType::Image,
                FileFormat::AVIF => MediaType::Image,
                FileFormat::ICO => MediaType::Image,
                FileFormat::PDF => MediaType::Document,
                FileFormat::DOCX => MediaType::Document,
                FileFormat::ODT => MediaType::Document,
                FileFormat::TXT => MediaType::Document,
                FileFormat::HTML => MediaType::Document,
                FileFormat::MD => MediaType::Document,
                FileFormat::EPUB => MediaType::Document,
            }
        }
        /// Provides a typical file extension (lowercase, without dot).
        pub fn default_extension(&self) -> &'static str {
            match self {
                FileFormat::MP4 => "mp4",
                FileFormat::WebM => "webm",
                FileFormat::GIF => "gif",
                FileFormat::AVI => "avi",
                FileFormat::MKV => "mkv",
                FileFormat::WMV => "wmv",
                FileFormat::MOV => "mov",
                FileFormat::MTS => "mts",
                FileFormat::FLV => "flv",
                FileFormat::OGV => "ogv",
                FileFormat::MP3 => "mp3",
                FileFormat::WAV => "wav",
                FileFormat::FLAC => "flac",
                FileFormat::OGG => "ogg",
                FileFormat::AAC => "aac",
                FileFormat::M4A => "m4a",
                FileFormat::OPUS => "opus",
                FileFormat::JPG => "jpg",
                FileFormat::JPEG => "jpg",
                FileFormat::PNG => "png",
                FileFormat::WEBP => "webp",
                FileFormat::BMP => "bmp",
                FileFormat::TIFF => "tiff",
                FileFormat::AVIF => "avif",
                FileFormat::ICO => "ico",
                FileFormat::PDF => "pdf",
                FileFormat::DOCX => "docx",
                FileFormat::ODT => "odt",
                FileFormat::TXT => "txt",
                FileFormat::HTML => "html",
                FileFormat::MD => "md",
                FileFormat::EPUB => "epub",
            }
        }
    }
}

//! START
/// // src/converters/video.rs
use crate::converters::progress::{ProgressUpdate, VideoProgressDetails};
use crate::converters::{ConversionTask, Converter, FileFormat, MediaType};
use anyhow::{anyhow, Context, Result};
use log::{debug, error, info, trace}; // Use logging
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::Arc;
use std::time::Duration; // For parsing time strings
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::mpsc;

// --- Video Specific Options ---
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct VideoConversionOptions {
    // Example options - use strum for enums if needed
    pub speed_preset: String, // e.g., "medium", "fast", "slow"
    pub crf: Option<u8>,      // e.g., 23
    pub gpu_acceleration: Option<String>, // e.g., "auto", "nvidia", "amd", "intel", "apple", "none"
                                // Add more options: resolution, bitrate, etc.
}

// --- Video Converter Implementation ---
#[derive(Debug)]
pub struct VideoConverter {
    // Path to ffmpeg executable, could be configured
    ffmpeg_path: PathBuf,
}

impl VideoConverter {
    pub fn new() -> Self {
        // Detect ffmpeg path or use a default
        Self {
            ffmpeg_path: "ffmpeg".into(), // Simple default
        }
    }

    // Helper to parse ffmpeg's progress output line
    fn parse_ffmpeg_progress(line: &str, details: &mut VideoProgressDetails) -> bool {
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();
            match key {
                "frame" => details.frame = value.parse().ok(),
                "fps" => details.fps = value.parse().ok(),
                "bitrate" => {
                    // Usually in kbit/s, remove suffix
                    details.bitrate_kbit = value
                        .trim_end_matches("kbits/s")
                        .trim()
                        .parse::<f64>()
                        .ok();
                }
                "total_size" => details.size_kb = value.parse::<u64>().ok().map(|b| b / 1024), // Convert bytes to KB
                "out_time_us" | "out_time_ms" => { // Prefer ms if available
                    if let Ok(us) = value.parse::<u64>() {
                       let secs = us / 1_000_000;
                       let micros = us % 1_000_000;
                       details.time_processed = Some(format!("{}.{:06}", secs, micros)); // Rough format
                    }
                }
                 "out_time" => details.time_processed = Some(value.to_string()), // HH:MM:SS.ms format
                "speed" => {
                     details.speed = value.trim_end_matches('x').trim().parse().ok();
                }
                "progress" => return value == "end", // Check for end signal
                _ => {}              // Ignore other fields
            }
        }
        false // Not the end signal
    }

    // Function to estimate total duration (could use ffprobe beforehand)
    // For now, returns None, meaning percentage might rely only on 'progress=end'
    async fn estimate_duration_seconds(_input_path: &Path) -> Result<Option<f64>> {
        // TODO: Implement ffprobe call to get duration
        // Example ffprobe command:
        // ffprobe -v error -show_entries format=duration -of default=noprint_wrappers=1:nokey=1 "input.mp4"
        Ok(None) // Placeholder
    }

    // Helper to calculate percentage based on time processed and total duration
    fn calculate_percentage(details: &VideoProgressDetails, total_duration_secs: Option<f64>) -> f32 {
        match (total_duration_secs, &details.time_processed) {
            (Some(total_secs), Some(time_str)) if total_secs > 0.0 => {
                 // Attempt to parse time_str (HH:MM:SS.ms or seconds.micros)
                 // This parsing is complex, needs a robust function
                 // Simplified example: assumes time_str is "SS.micros"
                 if let Ok(processed_secs) = time_str.split('.').next().unwrap_or("0").parse::<f64>() {
                    (processed_secs * 100.0 / total_secs).clamp(0.0, 100.0)
                 } else {
                     0.0 // Cannot parse time
                 }
            }
            _ => 0.0, // Cannot calculate percentage without duration or time
        }
    }
}

#[async_trait::async_trait]
impl Converter for VideoConverter {
    fn name(&self) -> &'static str {
        "FFmpeg Video Converter"
    }

    fn media_type(&self) -> MediaType {
        MediaType::Video
    }

    // Define supported formats (example)
    fn supported_input_formats(&self) -> Vec<FileFormat> {
        vec![
            FileFormat::MP4, FileFormat::WebM, FileFormat::AVI, FileFormat::MKV,
            FileFormat::WMV, FileFormat::MOV, FileFormat::MTS, FileFormat::FLV,
            FileFormat::OGV, // Add more if ffmpeg supports them
        ]
    }

    fn supported_output_formats(&self, _input_format: FileFormat) -> Vec<FileFormat> {
        // Output formats often don't depend heavily on input for video
        vec![
            FileFormat::MP4, FileFormat::WebM, FileFormat::AVI, FileFormat::MKV,
            FileFormat::MOV, FileFormat::GIF, // Add more
        ]
    }

    async fn convert(
        &self,
        task: Arc<ConversionTask>, // Use the Arc<ConversionTask>
    ) -> Result<mpsc::Receiver<ProgressUpdate>> {
        // --- Pre-computation & Setup ---
        let input_path = task.input_path.clone();
        let output_path = task.output_path.clone();
        let task_id = task.id.clone(); // Get task ID

        // Ensure output directory exists
        if let Some(parent) = output_path.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .with_context(|| format!("Failed to create output directory: {:?}", parent))?;
        }

        // Downcast options or use defaults
        let options: Arc<VideoConversionOptions> = task
            .get_options::<VideoConversionOptions>()
            .unwrap_or_else(|| {
                // Provide default options if none were given
                Arc::new(VideoConversionOptions {
                    speed_preset: "medium".to_string(),
                    crf: Some(23),
                    gpu_acceleration: Some("auto".to_string()), // Or None
                })
            });

        // Estimate total duration for percentage calculation (can be slow)
        // Consider doing this once when the file is added, not per conversion
        let total_duration_secs = Self::estimate_duration_seconds(&input_path).await.ok().flatten();


        // --- Build FFmpeg Command ---
        // (Adapt your existing command building logic here)
        let mut args: Vec<String> = vec![
            "-hide_banner".to_string(),
            "-loglevel".to_string(), "error".to_string(), // Base loglevel
            "-progress".to_string(), "pipe:1".to_string(), // Send progress to stdout
            "-y".to_string(), // Overwrite output without asking
            "-i".to_string(), input_path.to_string_lossy().to_string(),
        ];

        // Add specific options based on `options` and `task.target_format`
        args.push("-preset".to_string());
        args.push(options.speed_preset.clone());
        if let Some(crf) = options.crf {
             // Ensure CRF is suitable for the codec (e.g., libx264)
             // This needs codec-specific logic
             args.push("-crf".to_string());
             args.push(crf.to_string());
        }
        // Add GPU args, codec args (-c:v, -c:a), etc. based on options/format

        args.push(output_path.to_string_lossy().to_string());

        // --- Spawn FFmpeg Process ---
        info!(
            "Task [{}]: Spawning ffmpeg with args: {}",
            task_id,
            args.join(" ")
        );

        let mut command = Command::new(&self.ffmpeg_path);
        command
            .args(&args)
            .stdin(Stdio::null())
            .stdout(Stdio::piped()) // Capture stdout for progress
            .stderr(Stdio::piped()); // Capture stderr for errors

        let mut child = command.spawn().context("Failed to spawn ffmpeg process")?;

        let stdout = child.stdout.take().ok_or_else(|| anyhow!("Failed to capture ffmpeg stdout"))?;
        let stderr = child.stderr.take().ok_or_else(|| anyhow!("Failed to capture ffmpeg stderr"))?;

        // Create the MPSC channel for progress updates
        let (tx, rx) = mpsc::channel::<ProgressUpdate>(32); // Buffer size 32

        // --- Task to Handle Stdout (Progress) ---
        let tx_stdout = tx.clone();
        let task_id_stdout = task_id.clone();
        tokio::spawn(async move {
            let reader = BufReader::new(stdout);
            let mut lines = reader.lines();
            let mut progress_details = VideoProgressDetails::default(); // Need Default derive
            let mut end_signal_received = false;

            while let Ok(Some(line)) = lines.next_line().await {
                trace!("Task [{}]: FFmpeg stdout: {}", task_id_stdout, line);
                let mut current_details = progress_details.clone(); // Clone for this update
                if Self::parse_ffmpeg_progress(&line, &mut current_details) {
                    end_signal_received = true;
                    // Don't update progress_details state with the "progress=end" line itself
                } else {
                    progress_details = current_details.clone(); // Update state if not end signal
                }

                let percentage = Self::calculate_percentage(&progress_details, total_duration_secs);

                 // Throttle updates? Or send every time? Sending every time is simpler for now.
                 match ProgressUpdate::with_details(
                     task_id_stdout.clone(),
                     percentage,
                     Some("Encoding".to_string()),
                     progress_details.clone(), // Send the latest cumulative details
                 ) {
                     Ok(update) => {
                        if tx_stdout.send(update).await.is_err() {
                             debug!("Task [{}]: Progress receiver dropped (stdout).", task_id_stdout);
                             break; // Stop processing if receiver is gone
                         }
                     }
                     Err(e) => error!("Task [{}]: Failed to serialize progress details: {}", task_id_stdout, e),
                 }

                // If ffmpeg signals completion via progress=end, break loop
                if end_signal_received {
                   debug!("Task [{}]: Received progress=end signal.", task_id_stdout);
                   break;
                }
            }
             debug!("Task [{}]: Stdout processing finished.", task_id_stdout);
        });

        // --- Task to Handle Stderr (Errors/Logs) ---
        let tx_stderr = tx.clone();
        let task_id_stderr = task_id.clone();
        tokio::spawn(async move {
            let reader = BufReader::new(stderr);
            let mut lines = reader.lines();
            while let Ok(Some(line)) = lines.next_line().await {
                 error!("Task [{}]: FFmpeg stderr: {}", task_id_stderr, line); // Log as error
                 // Send as a warning progress update
                let update = ProgressUpdate::new_warning(task_id_stderr.clone(), line);
                if tx_stderr.send(update).await.is_err() {
                     debug!("Task [{}]: Progress receiver dropped (stderr).", task_id_stderr);
                     break;
                 }
            }
             debug!("Task [{}]: Stderr processing finished.", task_id_stderr);
        });

        // --- Task to Wait for Process Exit and Send Final Update ---
        let task_id_final = task_id.clone();
        tokio::spawn(async move {
            match child.wait().await {
                Ok(status) => {
                    if status.success() {
                        info!("Task [{}]: FFmpeg process completed successfully.", task_id_final);
                        let update = ProgressUpdate::new_done(task_id_final);
                        if tx.send(update).await.is_err() {
                           debug!("Task [{}]: Progress receiver dropped (final success).", task_id_final);
                       }
                    } else {
                         error!("Task [{}]: FFmpeg process failed with status: {}", task_id_final, status);
                         let update = ProgressUpdate::new_error(
                             task_id_final,
                             format!("FFmpeg failed with status: {}", status),
                        );
                         if tx.send(update).await.is_err() {
                            debug!("Task [{}]: Progress receiver dropped (final failure).", task_id_final);
                        }
                    }
                }
                Err(e) => {
                    error!("Task [{}]: Failed to wait for FFmpeg process: {}", task_id_final, e);
                     let update = ProgressUpdate::new_error(
                         task_id_final,
                         format!("Failed to wait for FFmpeg process: {}", e),
                    );
                     if tx.send(update).await.is_err() {
                         debug!("Task [{}]: Progress receiver dropped (wait error).", task_id_final);
                     }
                }
            }
             // Sender tx automatically drops here when task finishes, closing the channel.
        });

        Ok(rx) // Return the receiver immediately
    }
}

// Make sure VideoProgressDetails derives Default if needed above
#[derive(Debug, Clone, Serialize, Default)] // Added Default
pub struct VideoProgressDetails {
    pub frame: Option<u64>,
    pub fps: Option<f32>,
    pub bitrate_kbit: Option<f64>,
    pub size_kb: Option<u64>,
    pub time_processed: Option<String>,
    pub speed: Option<f32>,
}