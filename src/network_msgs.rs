// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct Feed {
    // message fields
    discoveryKey: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    nonce: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Feed {}

impl Feed {
    pub fn new() -> Feed {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Feed {
        static mut instance: ::protobuf::lazy::Lazy<Feed> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Feed,
        };
        unsafe {
            instance.get(Feed::new)
        }
    }

    // required bytes discoveryKey = 1;

    pub fn clear_discoveryKey(&mut self) {
        self.discoveryKey.clear();
    }

    pub fn has_discoveryKey(&self) -> bool {
        self.discoveryKey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_discoveryKey(&mut self, v: ::std::vec::Vec<u8>) {
        self.discoveryKey = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_discoveryKey(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.discoveryKey.is_none() {
            self.discoveryKey.set_default();
        }
        self.discoveryKey.as_mut().unwrap()
    }

    // Take field
    pub fn take_discoveryKey(&mut self) -> ::std::vec::Vec<u8> {
        self.discoveryKey.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_discoveryKey(&self) -> &[u8] {
        match self.discoveryKey.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_discoveryKey_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.discoveryKey
    }

    fn mut_discoveryKey_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.discoveryKey
    }

    // optional bytes nonce = 2;

    pub fn clear_nonce(&mut self) {
        self.nonce.clear();
    }

    pub fn has_nonce(&self) -> bool {
        self.nonce.is_some()
    }

    // Param is passed by value, moved
    pub fn set_nonce(&mut self, v: ::std::vec::Vec<u8>) {
        self.nonce = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_nonce(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.nonce.is_none() {
            self.nonce.set_default();
        }
        self.nonce.as_mut().unwrap()
    }

    // Take field
    pub fn take_nonce(&mut self) -> ::std::vec::Vec<u8> {
        self.nonce.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_nonce(&self) -> &[u8] {
        match self.nonce.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_nonce_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.nonce
    }

    fn mut_nonce_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.nonce
    }
}

impl ::protobuf::Message for Feed {
    fn is_initialized(&self) -> bool {
        if self.discoveryKey.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.discoveryKey)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.nonce)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.discoveryKey.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        if let Some(ref v) = self.nonce.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.discoveryKey.as_ref() {
            os.write_bytes(1, &v)?;
        }
        if let Some(ref v) = self.nonce.as_ref() {
            os.write_bytes(2, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Feed {
    fn new() -> Feed {
        Feed::new()
    }

    fn descriptor_static(_: ::std::option::Option<Feed>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "discoveryKey",
                    Feed::get_discoveryKey_for_reflect,
                    Feed::mut_discoveryKey_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "nonce",
                    Feed::get_nonce_for_reflect,
                    Feed::mut_nonce_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Feed>(
                    "Feed",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Feed {
    fn clear(&mut self) {
        self.clear_discoveryKey();
        self.clear_nonce();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Feed {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Feed {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Handshake {
    // message fields
    id: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    live: ::std::option::Option<bool>,
    userData: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    extensions: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Handshake {}

impl Handshake {
    pub fn new() -> Handshake {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Handshake {
        static mut instance: ::protobuf::lazy::Lazy<Handshake> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Handshake,
        };
        unsafe {
            instance.get(Handshake::new)
        }
    }

    // optional bytes id = 1;

    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::vec::Vec<u8>) {
        self.id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.id.is_none() {
            self.id.set_default();
        }
        self.id.as_mut().unwrap()
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::vec::Vec<u8> {
        self.id.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_id(&self) -> &[u8] {
        match self.id.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_id_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.id
    }

    // optional bool live = 2;

    pub fn clear_live(&mut self) {
        self.live = ::std::option::Option::None;
    }

    pub fn has_live(&self) -> bool {
        self.live.is_some()
    }

    // Param is passed by value, moved
    pub fn set_live(&mut self, v: bool) {
        self.live = ::std::option::Option::Some(v);
    }

    pub fn get_live(&self) -> bool {
        self.live.unwrap_or(false)
    }

    fn get_live_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.live
    }

    fn mut_live_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.live
    }

    // optional bytes userData = 3;

    pub fn clear_userData(&mut self) {
        self.userData.clear();
    }

    pub fn has_userData(&self) -> bool {
        self.userData.is_some()
    }

    // Param is passed by value, moved
    pub fn set_userData(&mut self, v: ::std::vec::Vec<u8>) {
        self.userData = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_userData(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.userData.is_none() {
            self.userData.set_default();
        }
        self.userData.as_mut().unwrap()
    }

    // Take field
    pub fn take_userData(&mut self) -> ::std::vec::Vec<u8> {
        self.userData.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_userData(&self) -> &[u8] {
        match self.userData.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_userData_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.userData
    }

    fn mut_userData_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.userData
    }

    // repeated string extensions = 4;

    pub fn clear_extensions(&mut self) {
        self.extensions.clear();
    }

    // Param is passed by value, moved
    pub fn set_extensions(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.extensions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_extensions(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.extensions
    }

    // Take field
    pub fn take_extensions(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.extensions, ::protobuf::RepeatedField::new())
    }

    pub fn get_extensions(&self) -> &[::std::string::String] {
        &self.extensions
    }

    fn get_extensions_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.extensions
    }

    fn mut_extensions_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.extensions
    }
}

impl ::protobuf::Message for Handshake {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.id)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.live = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.userData)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.extensions)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.id.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        if let Some(v) = self.live {
            my_size += 2;
        }
        if let Some(ref v) = self.userData.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        }
        for value in &self.extensions {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.id.as_ref() {
            os.write_bytes(1, &v)?;
        }
        if let Some(v) = self.live {
            os.write_bool(2, v)?;
        }
        if let Some(ref v) = self.userData.as_ref() {
            os.write_bytes(3, &v)?;
        }
        for v in &self.extensions {
            os.write_string(4, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Handshake {
    fn new() -> Handshake {
        Handshake::new()
    }

    fn descriptor_static(_: ::std::option::Option<Handshake>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "id",
                    Handshake::get_id_for_reflect,
                    Handshake::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "live",
                    Handshake::get_live_for_reflect,
                    Handshake::mut_live_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "userData",
                    Handshake::get_userData_for_reflect,
                    Handshake::mut_userData_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "extensions",
                    Handshake::get_extensions_for_reflect,
                    Handshake::mut_extensions_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Handshake>(
                    "Handshake",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Handshake {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_live();
        self.clear_userData();
        self.clear_extensions();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Handshake {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Handshake {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Info {
    // message fields
    uploading: ::std::option::Option<bool>,
    downloading: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Info {}

impl Info {
    pub fn new() -> Info {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Info {
        static mut instance: ::protobuf::lazy::Lazy<Info> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Info,
        };
        unsafe {
            instance.get(Info::new)
        }
    }

    // optional bool uploading = 1;

    pub fn clear_uploading(&mut self) {
        self.uploading = ::std::option::Option::None;
    }

    pub fn has_uploading(&self) -> bool {
        self.uploading.is_some()
    }

    // Param is passed by value, moved
    pub fn set_uploading(&mut self, v: bool) {
        self.uploading = ::std::option::Option::Some(v);
    }

    pub fn get_uploading(&self) -> bool {
        self.uploading.unwrap_or(false)
    }

    fn get_uploading_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.uploading
    }

    fn mut_uploading_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.uploading
    }

    // optional bool downloading = 2;

    pub fn clear_downloading(&mut self) {
        self.downloading = ::std::option::Option::None;
    }

    pub fn has_downloading(&self) -> bool {
        self.downloading.is_some()
    }

    // Param is passed by value, moved
    pub fn set_downloading(&mut self, v: bool) {
        self.downloading = ::std::option::Option::Some(v);
    }

    pub fn get_downloading(&self) -> bool {
        self.downloading.unwrap_or(false)
    }

    fn get_downloading_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.downloading
    }

    fn mut_downloading_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.downloading
    }
}

impl ::protobuf::Message for Info {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.uploading = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.downloading = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.uploading {
            my_size += 2;
        }
        if let Some(v) = self.downloading {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.uploading {
            os.write_bool(1, v)?;
        }
        if let Some(v) = self.downloading {
            os.write_bool(2, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Info {
    fn new() -> Info {
        Info::new()
    }

    fn descriptor_static(_: ::std::option::Option<Info>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "uploading",
                    Info::get_uploading_for_reflect,
                    Info::mut_uploading_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "downloading",
                    Info::get_downloading_for_reflect,
                    Info::mut_downloading_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Info>(
                    "Info",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Info {
    fn clear(&mut self) {
        self.clear_uploading();
        self.clear_downloading();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Info {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Info {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Have {
    // message fields
    start: ::std::option::Option<u64>,
    length: ::std::option::Option<u64>,
    bitfield: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Have {}

impl Have {
    pub fn new() -> Have {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Have {
        static mut instance: ::protobuf::lazy::Lazy<Have> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Have,
        };
        unsafe {
            instance.get(Have::new)
        }
    }

    // required uint64 start = 1;

    pub fn clear_start(&mut self) {
        self.start = ::std::option::Option::None;
    }

    pub fn has_start(&self) -> bool {
        self.start.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start(&mut self, v: u64) {
        self.start = ::std::option::Option::Some(v);
    }

    pub fn get_start(&self) -> u64 {
        self.start.unwrap_or(0)
    }

    fn get_start_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.start
    }

    fn mut_start_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.start
    }

    // optional uint64 length = 2;

    pub fn clear_length(&mut self) {
        self.length = ::std::option::Option::None;
    }

    pub fn has_length(&self) -> bool {
        self.length.is_some()
    }

    // Param is passed by value, moved
    pub fn set_length(&mut self, v: u64) {
        self.length = ::std::option::Option::Some(v);
    }

    pub fn get_length(&self) -> u64 {
        self.length.unwrap_or(1u64)
    }

    fn get_length_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.length
    }

    fn mut_length_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.length
    }

    // optional bytes bitfield = 3;

    pub fn clear_bitfield(&mut self) {
        self.bitfield.clear();
    }

    pub fn has_bitfield(&self) -> bool {
        self.bitfield.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bitfield(&mut self, v: ::std::vec::Vec<u8>) {
        self.bitfield = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_bitfield(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.bitfield.is_none() {
            self.bitfield.set_default();
        }
        self.bitfield.as_mut().unwrap()
    }

    // Take field
    pub fn take_bitfield(&mut self) -> ::std::vec::Vec<u8> {
        self.bitfield.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_bitfield(&self) -> &[u8] {
        match self.bitfield.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_bitfield_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.bitfield
    }

    fn mut_bitfield_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.bitfield
    }
}

impl ::protobuf::Message for Have {
    fn is_initialized(&self) -> bool {
        if self.start.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.start = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.length = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.bitfield)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.start {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.length {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.bitfield.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.start {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.length {
            os.write_uint64(2, v)?;
        }
        if let Some(ref v) = self.bitfield.as_ref() {
            os.write_bytes(3, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Have {
    fn new() -> Have {
        Have::new()
    }

    fn descriptor_static(_: ::std::option::Option<Have>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "start",
                    Have::get_start_for_reflect,
                    Have::mut_start_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "length",
                    Have::get_length_for_reflect,
                    Have::mut_length_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "bitfield",
                    Have::get_bitfield_for_reflect,
                    Have::mut_bitfield_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Have>(
                    "Have",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Have {
    fn clear(&mut self) {
        self.clear_start();
        self.clear_length();
        self.clear_bitfield();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Have {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Have {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Unhave {
    // message fields
    start: ::std::option::Option<u64>,
    length: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Unhave {}

impl Unhave {
    pub fn new() -> Unhave {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Unhave {
        static mut instance: ::protobuf::lazy::Lazy<Unhave> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Unhave,
        };
        unsafe {
            instance.get(Unhave::new)
        }
    }

    // required uint64 start = 1;

    pub fn clear_start(&mut self) {
        self.start = ::std::option::Option::None;
    }

    pub fn has_start(&self) -> bool {
        self.start.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start(&mut self, v: u64) {
        self.start = ::std::option::Option::Some(v);
    }

    pub fn get_start(&self) -> u64 {
        self.start.unwrap_or(0)
    }

    fn get_start_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.start
    }

    fn mut_start_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.start
    }

    // optional uint64 length = 2;

    pub fn clear_length(&mut self) {
        self.length = ::std::option::Option::None;
    }

    pub fn has_length(&self) -> bool {
        self.length.is_some()
    }

    // Param is passed by value, moved
    pub fn set_length(&mut self, v: u64) {
        self.length = ::std::option::Option::Some(v);
    }

    pub fn get_length(&self) -> u64 {
        self.length.unwrap_or(1u64)
    }

    fn get_length_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.length
    }

    fn mut_length_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.length
    }
}

impl ::protobuf::Message for Unhave {
    fn is_initialized(&self) -> bool {
        if self.start.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.start = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.length = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.start {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.length {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.start {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.length {
            os.write_uint64(2, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Unhave {
    fn new() -> Unhave {
        Unhave::new()
    }

    fn descriptor_static(_: ::std::option::Option<Unhave>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "start",
                    Unhave::get_start_for_reflect,
                    Unhave::mut_start_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "length",
                    Unhave::get_length_for_reflect,
                    Unhave::mut_length_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Unhave>(
                    "Unhave",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Unhave {
    fn clear(&mut self) {
        self.clear_start();
        self.clear_length();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Unhave {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Unhave {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Want {
    // message fields
    start: ::std::option::Option<u64>,
    length: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Want {}

impl Want {
    pub fn new() -> Want {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Want {
        static mut instance: ::protobuf::lazy::Lazy<Want> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Want,
        };
        unsafe {
            instance.get(Want::new)
        }
    }

    // required uint64 start = 1;

    pub fn clear_start(&mut self) {
        self.start = ::std::option::Option::None;
    }

    pub fn has_start(&self) -> bool {
        self.start.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start(&mut self, v: u64) {
        self.start = ::std::option::Option::Some(v);
    }

    pub fn get_start(&self) -> u64 {
        self.start.unwrap_or(0)
    }

    fn get_start_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.start
    }

    fn mut_start_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.start
    }

    // optional uint64 length = 2;

    pub fn clear_length(&mut self) {
        self.length = ::std::option::Option::None;
    }

    pub fn has_length(&self) -> bool {
        self.length.is_some()
    }

    // Param is passed by value, moved
    pub fn set_length(&mut self, v: u64) {
        self.length = ::std::option::Option::Some(v);
    }

    pub fn get_length(&self) -> u64 {
        self.length.unwrap_or(0)
    }

    fn get_length_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.length
    }

    fn mut_length_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.length
    }
}

impl ::protobuf::Message for Want {
    fn is_initialized(&self) -> bool {
        if self.start.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.start = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.length = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.start {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.length {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.start {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.length {
            os.write_uint64(2, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Want {
    fn new() -> Want {
        Want::new()
    }

    fn descriptor_static(_: ::std::option::Option<Want>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "start",
                    Want::get_start_for_reflect,
                    Want::mut_start_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "length",
                    Want::get_length_for_reflect,
                    Want::mut_length_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Want>(
                    "Want",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Want {
    fn clear(&mut self) {
        self.clear_start();
        self.clear_length();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Want {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Want {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Unwant {
    // message fields
    start: ::std::option::Option<u64>,
    length: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Unwant {}

impl Unwant {
    pub fn new() -> Unwant {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Unwant {
        static mut instance: ::protobuf::lazy::Lazy<Unwant> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Unwant,
        };
        unsafe {
            instance.get(Unwant::new)
        }
    }

    // required uint64 start = 1;

    pub fn clear_start(&mut self) {
        self.start = ::std::option::Option::None;
    }

    pub fn has_start(&self) -> bool {
        self.start.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start(&mut self, v: u64) {
        self.start = ::std::option::Option::Some(v);
    }

    pub fn get_start(&self) -> u64 {
        self.start.unwrap_or(0)
    }

    fn get_start_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.start
    }

    fn mut_start_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.start
    }

    // optional uint64 length = 2;

    pub fn clear_length(&mut self) {
        self.length = ::std::option::Option::None;
    }

    pub fn has_length(&self) -> bool {
        self.length.is_some()
    }

    // Param is passed by value, moved
    pub fn set_length(&mut self, v: u64) {
        self.length = ::std::option::Option::Some(v);
    }

    pub fn get_length(&self) -> u64 {
        self.length.unwrap_or(0)
    }

    fn get_length_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.length
    }

    fn mut_length_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.length
    }
}

impl ::protobuf::Message for Unwant {
    fn is_initialized(&self) -> bool {
        if self.start.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.start = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.length = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.start {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.length {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.start {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.length {
            os.write_uint64(2, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Unwant {
    fn new() -> Unwant {
        Unwant::new()
    }

    fn descriptor_static(_: ::std::option::Option<Unwant>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "start",
                    Unwant::get_start_for_reflect,
                    Unwant::mut_start_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "length",
                    Unwant::get_length_for_reflect,
                    Unwant::mut_length_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Unwant>(
                    "Unwant",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Unwant {
    fn clear(&mut self) {
        self.clear_start();
        self.clear_length();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Unwant {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Unwant {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Request {
    // message fields
    index: ::std::option::Option<u64>,
    bytes: ::std::option::Option<u64>,
    hash: ::std::option::Option<bool>,
    nodes: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Request {}

impl Request {
    pub fn new() -> Request {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Request {
        static mut instance: ::protobuf::lazy::Lazy<Request> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Request,
        };
        unsafe {
            instance.get(Request::new)
        }
    }

    // required uint64 index = 1;

    pub fn clear_index(&mut self) {
        self.index = ::std::option::Option::None;
    }

    pub fn has_index(&self) -> bool {
        self.index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_index(&mut self, v: u64) {
        self.index = ::std::option::Option::Some(v);
    }

    pub fn get_index(&self) -> u64 {
        self.index.unwrap_or(0)
    }

    fn get_index_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.index
    }

    fn mut_index_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.index
    }

    // optional uint64 bytes = 2;

    pub fn clear_bytes(&mut self) {
        self.bytes = ::std::option::Option::None;
    }

    pub fn has_bytes(&self) -> bool {
        self.bytes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bytes(&mut self, v: u64) {
        self.bytes = ::std::option::Option::Some(v);
    }

    pub fn get_bytes(&self) -> u64 {
        self.bytes.unwrap_or(0)
    }

    fn get_bytes_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.bytes
    }

    fn mut_bytes_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.bytes
    }

    // optional bool hash = 3;

    pub fn clear_hash(&mut self) {
        self.hash = ::std::option::Option::None;
    }

    pub fn has_hash(&self) -> bool {
        self.hash.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hash(&mut self, v: bool) {
        self.hash = ::std::option::Option::Some(v);
    }

    pub fn get_hash(&self) -> bool {
        self.hash.unwrap_or(false)
    }

    fn get_hash_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.hash
    }

    fn mut_hash_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.hash
    }

    // optional uint64 nodes = 4;

    pub fn clear_nodes(&mut self) {
        self.nodes = ::std::option::Option::None;
    }

    pub fn has_nodes(&self) -> bool {
        self.nodes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_nodes(&mut self, v: u64) {
        self.nodes = ::std::option::Option::Some(v);
    }

    pub fn get_nodes(&self) -> u64 {
        self.nodes.unwrap_or(0)
    }

    fn get_nodes_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.nodes
    }

    fn mut_nodes_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.nodes
    }
}

impl ::protobuf::Message for Request {
    fn is_initialized(&self) -> bool {
        if self.index.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.index = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.bytes = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.hash = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.nodes = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.index {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.bytes {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.hash {
            my_size += 2;
        }
        if let Some(v) = self.nodes {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.index {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.bytes {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.hash {
            os.write_bool(3, v)?;
        }
        if let Some(v) = self.nodes {
            os.write_uint64(4, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Request {
    fn new() -> Request {
        Request::new()
    }

    fn descriptor_static(_: ::std::option::Option<Request>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "index",
                    Request::get_index_for_reflect,
                    Request::mut_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "bytes",
                    Request::get_bytes_for_reflect,
                    Request::mut_bytes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "hash",
                    Request::get_hash_for_reflect,
                    Request::mut_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "nodes",
                    Request::get_nodes_for_reflect,
                    Request::mut_nodes_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Request>(
                    "Request",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Request {
    fn clear(&mut self) {
        self.clear_index();
        self.clear_bytes();
        self.clear_hash();
        self.clear_nodes();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Request {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Request {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Cancel {
    // message fields
    index: ::std::option::Option<u64>,
    bytes: ::std::option::Option<u64>,
    hash: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Cancel {}

impl Cancel {
    pub fn new() -> Cancel {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Cancel {
        static mut instance: ::protobuf::lazy::Lazy<Cancel> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Cancel,
        };
        unsafe {
            instance.get(Cancel::new)
        }
    }

    // required uint64 index = 1;

    pub fn clear_index(&mut self) {
        self.index = ::std::option::Option::None;
    }

    pub fn has_index(&self) -> bool {
        self.index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_index(&mut self, v: u64) {
        self.index = ::std::option::Option::Some(v);
    }

    pub fn get_index(&self) -> u64 {
        self.index.unwrap_or(0)
    }

    fn get_index_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.index
    }

    fn mut_index_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.index
    }

    // optional uint64 bytes = 2;

    pub fn clear_bytes(&mut self) {
        self.bytes = ::std::option::Option::None;
    }

    pub fn has_bytes(&self) -> bool {
        self.bytes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bytes(&mut self, v: u64) {
        self.bytes = ::std::option::Option::Some(v);
    }

    pub fn get_bytes(&self) -> u64 {
        self.bytes.unwrap_or(0)
    }

    fn get_bytes_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.bytes
    }

    fn mut_bytes_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.bytes
    }

    // optional bool hash = 3;

    pub fn clear_hash(&mut self) {
        self.hash = ::std::option::Option::None;
    }

    pub fn has_hash(&self) -> bool {
        self.hash.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hash(&mut self, v: bool) {
        self.hash = ::std::option::Option::Some(v);
    }

    pub fn get_hash(&self) -> bool {
        self.hash.unwrap_or(false)
    }

    fn get_hash_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.hash
    }

    fn mut_hash_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.hash
    }
}

impl ::protobuf::Message for Cancel {
    fn is_initialized(&self) -> bool {
        if self.index.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.index = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.bytes = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.hash = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.index {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.bytes {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.hash {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.index {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.bytes {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.hash {
            os.write_bool(3, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Cancel {
    fn new() -> Cancel {
        Cancel::new()
    }

    fn descriptor_static(_: ::std::option::Option<Cancel>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "index",
                    Cancel::get_index_for_reflect,
                    Cancel::mut_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "bytes",
                    Cancel::get_bytes_for_reflect,
                    Cancel::mut_bytes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "hash",
                    Cancel::get_hash_for_reflect,
                    Cancel::mut_hash_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Cancel>(
                    "Cancel",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Cancel {
    fn clear(&mut self) {
        self.clear_index();
        self.clear_bytes();
        self.clear_hash();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Cancel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Cancel {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Data {
    // message fields
    index: ::std::option::Option<u64>,
    value: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    nodes: ::protobuf::RepeatedField<Data_Node>,
    signature: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Data {}

impl Data {
    pub fn new() -> Data {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Data {
        static mut instance: ::protobuf::lazy::Lazy<Data> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Data,
        };
        unsafe {
            instance.get(Data::new)
        }
    }

    // required uint64 index = 1;

    pub fn clear_index(&mut self) {
        self.index = ::std::option::Option::None;
    }

    pub fn has_index(&self) -> bool {
        self.index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_index(&mut self, v: u64) {
        self.index = ::std::option::Option::Some(v);
    }

    pub fn get_index(&self) -> u64 {
        self.index.unwrap_or(0)
    }

    fn get_index_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.index
    }

    fn mut_index_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.index
    }

    // optional bytes value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.value.is_none() {
            self.value.set_default();
        }
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        self.value.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_value(&self) -> &[u8] {
        match self.value.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_value_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.value
    }

    // repeated .Data.Node nodes = 3;

    pub fn clear_nodes(&mut self) {
        self.nodes.clear();
    }

    // Param is passed by value, moved
    pub fn set_nodes(&mut self, v: ::protobuf::RepeatedField<Data_Node>) {
        self.nodes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_nodes(&mut self) -> &mut ::protobuf::RepeatedField<Data_Node> {
        &mut self.nodes
    }

    // Take field
    pub fn take_nodes(&mut self) -> ::protobuf::RepeatedField<Data_Node> {
        ::std::mem::replace(&mut self.nodes, ::protobuf::RepeatedField::new())
    }

    pub fn get_nodes(&self) -> &[Data_Node] {
        &self.nodes
    }

    fn get_nodes_for_reflect(&self) -> &::protobuf::RepeatedField<Data_Node> {
        &self.nodes
    }

    fn mut_nodes_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Data_Node> {
        &mut self.nodes
    }

    // optional bytes signature = 4;

    pub fn clear_signature(&mut self) {
        self.signature.clear();
    }

    pub fn has_signature(&self) -> bool {
        self.signature.is_some()
    }

    // Param is passed by value, moved
    pub fn set_signature(&mut self, v: ::std::vec::Vec<u8>) {
        self.signature = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_signature(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.signature.is_none() {
            self.signature.set_default();
        }
        self.signature.as_mut().unwrap()
    }

    // Take field
    pub fn take_signature(&mut self) -> ::std::vec::Vec<u8> {
        self.signature.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_signature(&self) -> &[u8] {
        match self.signature.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_signature_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.signature
    }

    fn mut_signature_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.signature
    }
}

impl ::protobuf::Message for Data {
    fn is_initialized(&self) -> bool {
        if self.index.is_none() {
            return false;
        }
        for v in &self.nodes {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.index = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.value)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.nodes)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.signature)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.index {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.value.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        for value in &self.nodes {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.signature.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.index {
            os.write_uint64(1, v)?;
        }
        if let Some(ref v) = self.value.as_ref() {
            os.write_bytes(2, &v)?;
        }
        for v in &self.nodes {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.signature.as_ref() {
            os.write_bytes(4, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Data {
    fn new() -> Data {
        Data::new()
    }

    fn descriptor_static(_: ::std::option::Option<Data>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "index",
                    Data::get_index_for_reflect,
                    Data::mut_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "value",
                    Data::get_value_for_reflect,
                    Data::mut_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Data_Node>>(
                    "nodes",
                    Data::get_nodes_for_reflect,
                    Data::mut_nodes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "signature",
                    Data::get_signature_for_reflect,
                    Data::mut_signature_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Data>(
                    "Data",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Data {
    fn clear(&mut self) {
        self.clear_index();
        self.clear_value();
        self.clear_nodes();
        self.clear_signature();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Data {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Data {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Data_Node {
    // message fields
    index: ::std::option::Option<u64>,
    hash: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    size: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Data_Node {}

impl Data_Node {
    pub fn new() -> Data_Node {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Data_Node {
        static mut instance: ::protobuf::lazy::Lazy<Data_Node> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Data_Node,
        };
        unsafe {
            instance.get(Data_Node::new)
        }
    }

    // required uint64 index = 1;

    pub fn clear_index(&mut self) {
        self.index = ::std::option::Option::None;
    }

    pub fn has_index(&self) -> bool {
        self.index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_index(&mut self, v: u64) {
        self.index = ::std::option::Option::Some(v);
    }

    pub fn get_index(&self) -> u64 {
        self.index.unwrap_or(0)
    }

    fn get_index_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.index
    }

    fn mut_index_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.index
    }

    // required bytes hash = 2;

    pub fn clear_hash(&mut self) {
        self.hash.clear();
    }

    pub fn has_hash(&self) -> bool {
        self.hash.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.hash = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.hash.is_none() {
            self.hash.set_default();
        }
        self.hash.as_mut().unwrap()
    }

    // Take field
    pub fn take_hash(&mut self) -> ::std::vec::Vec<u8> {
        self.hash.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_hash(&self) -> &[u8] {
        match self.hash.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_hash_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.hash
    }

    fn mut_hash_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.hash
    }

    // required uint64 size = 3;

    pub fn clear_size(&mut self) {
        self.size = ::std::option::Option::None;
    }

    pub fn has_size(&self) -> bool {
        self.size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_size(&mut self, v: u64) {
        self.size = ::std::option::Option::Some(v);
    }

    pub fn get_size(&self) -> u64 {
        self.size.unwrap_or(0)
    }

    fn get_size_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.size
    }

    fn mut_size_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.size
    }
}

impl ::protobuf::Message for Data_Node {
    fn is_initialized(&self) -> bool {
        if self.index.is_none() {
            return false;
        }
        if self.hash.is_none() {
            return false;
        }
        if self.size.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.index = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.hash)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.size = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.index {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.hash.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        if let Some(v) = self.size {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.index {
            os.write_uint64(1, v)?;
        }
        if let Some(ref v) = self.hash.as_ref() {
            os.write_bytes(2, &v)?;
        }
        if let Some(v) = self.size {
            os.write_uint64(3, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Data_Node {
    fn new() -> Data_Node {
        Data_Node::new()
    }

    fn descriptor_static(_: ::std::option::Option<Data_Node>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "index",
                    Data_Node::get_index_for_reflect,
                    Data_Node::mut_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "hash",
                    Data_Node::get_hash_for_reflect,
                    Data_Node::mut_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "size",
                    Data_Node::get_size_for_reflect,
                    Data_Node::mut_size_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Data_Node>(
                    "Data_Node",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Data_Node {
    fn clear(&mut self) {
        self.clear_index();
        self.clear_hash();
        self.clear_size();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Data_Node {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Data_Node {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\tdat.proto\"@\n\x04Feed\x12\"\n\x0cdiscoveryKey\x18\x01\x20\x02(\x0cR\
    \x0cdiscoveryKey\x12\x14\n\x05nonce\x18\x02\x20\x01(\x0cR\x05nonce\"k\n\
    \tHandshake\x12\x0e\n\x02id\x18\x01\x20\x01(\x0cR\x02id\x12\x12\n\x04liv\
    e\x18\x02\x20\x01(\x08R\x04live\x12\x1a\n\x08userData\x18\x03\x20\x01(\
    \x0cR\x08userData\x12\x1e\n\nextensions\x18\x04\x20\x03(\tR\nextensions\
    \"F\n\x04Info\x12\x1c\n\tuploading\x18\x01\x20\x01(\x08R\tuploading\x12\
    \x20\n\x0bdownloading\x18\x02\x20\x01(\x08R\x0bdownloading\"S\n\x04Have\
    \x12\x14\n\x05start\x18\x01\x20\x02(\x04R\x05start\x12\x19\n\x06length\
    \x18\x02\x20\x01(\x04:\x011R\x06length\x12\x1a\n\x08bitfield\x18\x03\x20\
    \x01(\x0cR\x08bitfield\"9\n\x06Unhave\x12\x14\n\x05start\x18\x01\x20\x02\
    (\x04R\x05start\x12\x19\n\x06length\x18\x02\x20\x01(\x04:\x011R\x06lengt\
    h\"4\n\x04Want\x12\x14\n\x05start\x18\x01\x20\x02(\x04R\x05start\x12\x16\
    \n\x06length\x18\x02\x20\x01(\x04R\x06length\"6\n\x06Unwant\x12\x14\n\
    \x05start\x18\x01\x20\x02(\x04R\x05start\x12\x16\n\x06length\x18\x02\x20\
    \x01(\x04R\x06length\"_\n\x07Request\x12\x14\n\x05index\x18\x01\x20\x02(\
    \x04R\x05index\x12\x14\n\x05bytes\x18\x02\x20\x01(\x04R\x05bytes\x12\x12\
    \n\x04hash\x18\x03\x20\x01(\x08R\x04hash\x12\x14\n\x05nodes\x18\x04\x20\
    \x01(\x04R\x05nodes\"H\n\x06Cancel\x12\x14\n\x05index\x18\x01\x20\x02(\
    \x04R\x05index\x12\x14\n\x05bytes\x18\x02\x20\x01(\x04R\x05bytes\x12\x12\
    \n\x04hash\x18\x03\x20\x01(\x08R\x04hash\"\xb8\x01\n\x04Data\x12\x14\n\
    \x05index\x18\x01\x20\x02(\x04R\x05index\x12\x14\n\x05value\x18\x02\x20\
    \x01(\x0cR\x05value\x12\x20\n\x05nodes\x18\x03\x20\x03(\x0b2\n.Data.Node\
    R\x05nodes\x12\x1c\n\tsignature\x18\x04\x20\x01(\x0cR\tsignature\x1aD\n\
    \x04Node\x12\x14\n\x05index\x18\x01\x20\x02(\x04R\x05index\x12\x12\n\x04\
    hash\x18\x02\x20\x02(\x0cR\x04hash\x12\x12\n\x04size\x18\x03\x20\x02(\
    \x04R\x04sizeJ\xce\x1a\n\x06\x12\x04\x04\0M\x01\n\x9f\x01\n\x02\x04\0\
    \x12\x04\x04\0\x07\x01\x1a7\x20type=0,\x20should\x20be\x20the\x20first\
    \x20message\x20sent\x20on\x20a\x20channel\n2Z\x20wire\x20format\x20is\
    \x20<len>(<header><message>)\n\x20header\x20is\x20a\x20varint,\x20channe\
    l\x20<<\x204\x20|\x20<4-bit-type>\n\n\n\n\x03\x04\0\x01\x12\x03\x04\x08\
    \x0c\n\x0b\n\x04\x04\0\x02\0\x12\x03\x05\x02\"\n\x0c\n\x05\x04\0\x02\0\
    \x04\x12\x03\x05\x02\n\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x05\x0b\x10\n\
    \x0c\n\x05\x04\0\x02\0\x01\x12\x03\x05\x11\x1d\n\x0c\n\x05\x04\0\x02\0\
    \x03\x12\x03\x05\x20!\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x06\x02\x1b\n\
    \x0c\n\x05\x04\0\x02\x01\x04\x12\x03\x06\x02\n\n\x0c\n\x05\x04\0\x02\x01\
    \x05\x12\x03\x06\x0b\x10\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x06\x11\
    \x16\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x06\x19\x1a\nx\n\x02\x04\x01\
    \x12\x04\n\0\x0f\x01\x1al\x20type=1,\x20overall\x20connection\x20handsha\
    ke.\x20should\x20be\x20send\x20just\x20after\x20the\x20feed\x20message\
    \x20on\x20the\x20first\x20channel\x20only\n\n\n\n\x03\x04\x01\x01\x12\
    \x03\n\x08\x11\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x0b\x02\x18\n\x0c\n\x05\
    \x04\x01\x02\0\x04\x12\x03\x0b\x02\n\n\x0c\n\x05\x04\x01\x02\0\x05\x12\
    \x03\x0b\x0b\x10\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x0b\x11\x13\n\x0c\
    \n\x05\x04\x01\x02\0\x03\x12\x03\x0b\x16\x17\nH\n\x04\x04\x01\x02\x01\
    \x12\x03\x0c\x02\x19\";\x20keep\x20the\x20connection\x20open\x20forever?\
    \x20both\x20ends\x20have\x20to\x20agree\n\n\x0c\n\x05\x04\x01\x02\x01\
    \x04\x12\x03\x0c\x02\n\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x03\x0c\x0b\
    \x0f\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03\x0c\x10\x14\n\x0c\n\x05\x04\
    \x01\x02\x01\x03\x12\x03\x0c\x17\x18\n\x0b\n\x04\x04\x01\x02\x02\x12\x03\
    \r\x02\x1e\n\x0c\n\x05\x04\x01\x02\x02\x04\x12\x03\r\x02\n\n\x0c\n\x05\
    \x04\x01\x02\x02\x05\x12\x03\r\x0b\x10\n\x0c\n\x05\x04\x01\x02\x02\x01\
    \x12\x03\r\x11\x19\n\x0c\n\x05\x04\x01\x02\x02\x03\x12\x03\r\x1c\x1d\n\
    \x0b\n\x04\x04\x01\x02\x03\x12\x03\x0e\x02!\n\x0c\n\x05\x04\x01\x02\x03\
    \x04\x12\x03\x0e\x02\n\n\x0c\n\x05\x04\x01\x02\x03\x05\x12\x03\x0e\x0b\
    \x11\n\x0c\n\x05\x04\x01\x02\x03\x01\x12\x03\x0e\x12\x1c\n\x0c\n\x05\x04\
    \x01\x02\x03\x03\x12\x03\x0e\x1f\x20\n\xc4\x01\n\x02\x04\x02\x12\x04\x14\
    \0\x17\x01\x1a\xb7\x01\x20type=2,\x20message\x20indicating\x20state\x20c\
    hanges\x20etc.\n\x20initial\x20state\x20for\x20uploading/downloading\x20\
    is\x20true\n\x20if\x20both\x20ends\x20are\x20not\x20downloading\x20and\
    \x20not\x20live\x20it\x20is\x20safe\x20to\x20consider\x20the\x20stream\
    \x20ended\n\n\n\n\x03\x04\x02\x01\x12\x03\x14\x08\x0c\n\x0b\n\x04\x04\
    \x02\x02\0\x12\x03\x15\x02\x1e\n\x0c\n\x05\x04\x02\x02\0\x04\x12\x03\x15\
    \x02\n\n\x0c\n\x05\x04\x02\x02\0\x05\x12\x03\x15\x0b\x0f\n\x0c\n\x05\x04\
    \x02\x02\0\x01\x12\x03\x15\x10\x19\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\
    \x15\x1c\x1d\n\x0b\n\x04\x04\x02\x02\x01\x12\x03\x16\x02\x20\n\x0c\n\x05\
    \x04\x02\x02\x01\x04\x12\x03\x16\x02\n\n\x0c\n\x05\x04\x02\x02\x01\x05\
    \x12\x03\x16\x0b\x0f\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x03\x16\x10\x1b\
    \n\x0c\n\x05\x04\x02\x02\x01\x03\x12\x03\x16\x1e\x1f\n&\n\x02\x04\x03\
    \x12\x04\x1a\0\x1e\x01\x1a\x1a\x20type=3,\x20what\x20do\x20we\x20have?\n\
    \n\n\n\x03\x04\x03\x01\x12\x03\x1a\x08\x0c\n\x0b\n\x04\x04\x03\x02\0\x12\
    \x03\x1b\x02\x1c\n\x0c\n\x05\x04\x03\x02\0\x04\x12\x03\x1b\x02\n\n\x0c\n\
    \x05\x04\x03\x02\0\x05\x12\x03\x1b\x0b\x11\n\x0c\n\x05\x04\x03\x02\0\x01\
    \x12\x03\x1b\x12\x17\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x03\x1b\x1a\x1b\n\
    \x1c\n\x04\x04\x03\x02\x01\x12\x03\x1c\x02+\"\x0f\x20defaults\x20to\x201\
    \n\n\x0c\n\x05\x04\x03\x02\x01\x04\x12\x03\x1c\x02\n\n\x0c\n\x05\x04\x03\
    \x02\x01\x05\x12\x03\x1c\x0b\x11\n\x0c\n\x05\x04\x03\x02\x01\x01\x12\x03\
    \x1c\x12\x18\n\x0c\n\x05\x04\x03\x02\x01\x03\x12\x03\x1c\x1b\x1c\n\x0c\n\
    \x05\x04\x03\x02\x01\x08\x12\x03\x1c\x1d*\n\x0c\n\x05\x04\x03\x02\x01\
    \x07\x12\x03\x1c()\n\x0b\n\x04\x04\x03\x02\x02\x12\x03\x1d\x02\x1e\n\x0c\
    \n\x05\x04\x03\x02\x02\x04\x12\x03\x1d\x02\n\n\x0c\n\x05\x04\x03\x02\x02\
    \x05\x12\x03\x1d\x0b\x10\n\x0c\n\x05\x04\x03\x02\x02\x01\x12\x03\x1d\x11\
    \x19\n\x0c\n\x05\x04\x03\x02\x02\x03\x12\x03\x1d\x1c\x1d\n'\n\x02\x04\
    \x04\x12\x04!\0$\x01\x1a\x1b\x20type=4,\x20what\x20did\x20we\x20lose?\n\
    \n\n\n\x03\x04\x04\x01\x12\x03!\x08\x0e\n\x0b\n\x04\x04\x04\x02\0\x12\
    \x03\"\x02\x1c\n\x0c\n\x05\x04\x04\x02\0\x04\x12\x03\"\x02\n\n\x0c\n\x05\
    \x04\x04\x02\0\x05\x12\x03\"\x0b\x11\n\x0c\n\x05\x04\x04\x02\0\x01\x12\
    \x03\"\x12\x17\n\x0c\n\x05\x04\x04\x02\0\x03\x12\x03\"\x1a\x1b\n\x1c\n\
    \x04\x04\x04\x02\x01\x12\x03#\x02+\"\x0f\x20defaults\x20to\x201\n\n\x0c\
    \n\x05\x04\x04\x02\x01\x04\x12\x03#\x02\n\n\x0c\n\x05\x04\x04\x02\x01\
    \x05\x12\x03#\x0b\x11\n\x0c\n\x05\x04\x04\x02\x01\x01\x12\x03#\x12\x18\n\
    \x0c\n\x05\x04\x04\x02\x01\x03\x12\x03#\x1b\x1c\n\x0c\n\x05\x04\x04\x02\
    \x01\x08\x12\x03#\x1d*\n\x0c\n\x05\x04\x04\x02\x01\x07\x12\x03#()\n^\n\
    \x02\x04\x05\x12\x04'\0*\x01\x1aR\x20type=5,\x20what\x20do\x20we\x20want\
    ?\x20remote\x20should\x20start\x20sending\x20have\x20messages\x20in\x20t\
    his\x20range\n\n\n\n\x03\x04\x05\x01\x12\x03'\x08\x0c\n\x0b\n\x04\x04\
    \x05\x02\0\x12\x03(\x02\x1c\n\x0c\n\x05\x04\x05\x02\0\x04\x12\x03(\x02\n\
    \n\x0c\n\x05\x04\x05\x02\0\x05\x12\x03(\x0b\x11\n\x0c\n\x05\x04\x05\x02\
    \0\x01\x12\x03(\x12\x17\n\x0c\n\x05\x04\x05\x02\0\x03\x12\x03(\x1a\x1b\n\
    @\n\x04\x04\x05\x02\x01\x12\x03)\x02\x1d\"3\x20defaults\x20to\x20Infinit\
    y\x20or\x20feed.length\x20(if\x20not\x20live)\n\n\x0c\n\x05\x04\x05\x02\
    \x01\x04\x12\x03)\x02\n\n\x0c\n\x05\x04\x05\x02\x01\x05\x12\x03)\x0b\x11\
    \n\x0c\n\x05\x04\x05\x02\x01\x01\x12\x03)\x12\x18\n\x0c\n\x05\x04\x05\
    \x02\x01\x03\x12\x03)\x1b\x1c\n1\n\x02\x04\x06\x12\x04-\00\x01\x1a%\x20t\
    ype=6,\x20what\x20don't\x20we\x20want\x20anymore?\n\n\n\n\x03\x04\x06\
    \x01\x12\x03-\x08\x0e\n\x0b\n\x04\x04\x06\x02\0\x12\x03.\x02\x1c\n\x0c\n\
    \x05\x04\x06\x02\0\x04\x12\x03.\x02\n\n\x0c\n\x05\x04\x06\x02\0\x05\x12\
    \x03.\x0b\x11\n\x0c\n\x05\x04\x06\x02\0\x01\x12\x03.\x12\x17\n\x0c\n\x05\
    \x04\x06\x02\0\x03\x12\x03.\x1a\x1b\n@\n\x04\x04\x06\x02\x01\x12\x03/\
    \x02\x1d\"3\x20defaults\x20to\x20Infinity\x20or\x20feed.length\x20(if\
    \x20not\x20live)\n\n\x0c\n\x05\x04\x06\x02\x01\x04\x12\x03/\x02\n\n\x0c\
    \n\x05\x04\x06\x02\x01\x05\x12\x03/\x0b\x11\n\x0c\n\x05\x04\x06\x02\x01\
    \x01\x12\x03/\x12\x18\n\x0c\n\x05\x04\x06\x02\x01\x03\x12\x03/\x1b\x1c\n\
    \"\n\x02\x04\x07\x12\x043\08\x01\x1a\x16\x20type=7,\x20ask\x20for\x20dat\
    a\n\n\n\n\x03\x04\x07\x01\x12\x033\x08\x0f\n\x0b\n\x04\x04\x07\x02\0\x12\
    \x034\x02\x1c\n\x0c\n\x05\x04\x07\x02\0\x04\x12\x034\x02\n\n\x0c\n\x05\
    \x04\x07\x02\0\x05\x12\x034\x0b\x11\n\x0c\n\x05\x04\x07\x02\0\x01\x12\
    \x034\x12\x17\n\x0c\n\x05\x04\x07\x02\0\x03\x12\x034\x1a\x1b\n\x0b\n\x04\
    \x04\x07\x02\x01\x12\x035\x02\x1c\n\x0c\n\x05\x04\x07\x02\x01\x04\x12\
    \x035\x02\n\n\x0c\n\x05\x04\x07\x02\x01\x05\x12\x035\x0b\x11\n\x0c\n\x05\
    \x04\x07\x02\x01\x01\x12\x035\x12\x17\n\x0c\n\x05\x04\x07\x02\x01\x03\
    \x12\x035\x1a\x1b\n\x0b\n\x04\x04\x07\x02\x02\x12\x036\x02\x19\n\x0c\n\
    \x05\x04\x07\x02\x02\x04\x12\x036\x02\n\n\x0c\n\x05\x04\x07\x02\x02\x05\
    \x12\x036\x0b\x0f\n\x0c\n\x05\x04\x07\x02\x02\x01\x12\x036\x10\x14\n\x0c\
    \n\x05\x04\x07\x02\x02\x03\x12\x036\x17\x18\n\x0b\n\x04\x04\x07\x02\x03\
    \x12\x037\x02\x1c\n\x0c\n\x05\x04\x07\x02\x03\x04\x12\x037\x02\n\n\x0c\n\
    \x05\x04\x07\x02\x03\x05\x12\x037\x0b\x11\n\x0c\n\x05\x04\x07\x02\x03\
    \x01\x12\x037\x12\x17\n\x0c\n\x05\x04\x07\x02\x03\x03\x12\x037\x1a\x1b\n\
    &\n\x02\x04\x08\x12\x04;\0?\x01\x1a\x1a\x20type=8,\x20cancel\x20a\x20req\
    uest\n\n\n\n\x03\x04\x08\x01\x12\x03;\x08\x0e\n\x0b\n\x04\x04\x08\x02\0\
    \x12\x03<\x02\x1c\n\x0c\n\x05\x04\x08\x02\0\x04\x12\x03<\x02\n\n\x0c\n\
    \x05\x04\x08\x02\0\x05\x12\x03<\x0b\x11\n\x0c\n\x05\x04\x08\x02\0\x01\
    \x12\x03<\x12\x17\n\x0c\n\x05\x04\x08\x02\0\x03\x12\x03<\x1a\x1b\n\x0b\n\
    \x04\x04\x08\x02\x01\x12\x03=\x02\x1c\n\x0c\n\x05\x04\x08\x02\x01\x04\
    \x12\x03=\x02\n\n\x0c\n\x05\x04\x08\x02\x01\x05\x12\x03=\x0b\x11\n\x0c\n\
    \x05\x04\x08\x02\x01\x01\x12\x03=\x12\x17\n\x0c\n\x05\x04\x08\x02\x01\
    \x03\x12\x03=\x1a\x1b\n\x0b\n\x04\x04\x08\x02\x02\x12\x03>\x02\x19\n\x0c\
    \n\x05\x04\x08\x02\x02\x04\x12\x03>\x02\n\n\x0c\n\x05\x04\x08\x02\x02\
    \x05\x12\x03>\x0b\x0f\n\x0c\n\x05\x04\x08\x02\x02\x01\x12\x03>\x10\x14\n\
    \x0c\n\x05\x04\x08\x02\x02\x03\x12\x03>\x17\x18\n#\n\x02\x04\t\x12\x04B\
    \0M\x01\x1a\x17\x20type=9,\x20get\x20some\x20data\n\n\n\n\x03\x04\t\x01\
    \x12\x03B\x08\x0c\n\x0c\n\x04\x04\t\x03\0\x12\x04C\x02G\x03\n\x0c\n\x05\
    \x04\t\x03\0\x01\x12\x03C\n\x0e\n\r\n\x06\x04\t\x03\0\x02\0\x12\x03D\x04\
    \x1e\n\x0e\n\x07\x04\t\x03\0\x02\0\x04\x12\x03D\x04\x0c\n\x0e\n\x07\x04\
    \t\x03\0\x02\0\x05\x12\x03D\r\x13\n\x0e\n\x07\x04\t\x03\0\x02\0\x01\x12\
    \x03D\x14\x19\n\x0e\n\x07\x04\t\x03\0\x02\0\x03\x12\x03D\x1c\x1d\n\r\n\
    \x06\x04\t\x03\0\x02\x01\x12\x03E\x04\x1c\n\x0e\n\x07\x04\t\x03\0\x02\
    \x01\x04\x12\x03E\x04\x0c\n\x0e\n\x07\x04\t\x03\0\x02\x01\x05\x12\x03E\r\
    \x12\n\x0e\n\x07\x04\t\x03\0\x02\x01\x01\x12\x03E\x13\x17\n\x0e\n\x07\
    \x04\t\x03\0\x02\x01\x03\x12\x03E\x1a\x1b\n\r\n\x06\x04\t\x03\0\x02\x02\
    \x12\x03F\x04\x1d\n\x0e\n\x07\x04\t\x03\0\x02\x02\x04\x12\x03F\x04\x0c\n\
    \x0e\n\x07\x04\t\x03\0\x02\x02\x05\x12\x03F\r\x13\n\x0e\n\x07\x04\t\x03\
    \0\x02\x02\x01\x12\x03F\x14\x18\n\x0e\n\x07\x04\t\x03\0\x02\x02\x03\x12\
    \x03F\x1b\x1c\n\x0b\n\x04\x04\t\x02\0\x12\x03I\x02\x1c\n\x0c\n\x05\x04\t\
    \x02\0\x04\x12\x03I\x02\n\n\x0c\n\x05\x04\t\x02\0\x05\x12\x03I\x0b\x11\n\
    \x0c\n\x05\x04\t\x02\0\x01\x12\x03I\x12\x17\n\x0c\n\x05\x04\t\x02\0\x03\
    \x12\x03I\x1a\x1b\n\x0b\n\x04\x04\t\x02\x01\x12\x03J\x02\x1b\n\x0c\n\x05\
    \x04\t\x02\x01\x04\x12\x03J\x02\n\n\x0c\n\x05\x04\t\x02\x01\x05\x12\x03J\
    \x0b\x10\n\x0c\n\x05\x04\t\x02\x01\x01\x12\x03J\x11\x16\n\x0c\n\x05\x04\
    \t\x02\x01\x03\x12\x03J\x19\x1a\n\x0b\n\x04\x04\t\x02\x02\x12\x03K\x02\
    \x1a\n\x0c\n\x05\x04\t\x02\x02\x04\x12\x03K\x02\n\n\x0c\n\x05\x04\t\x02\
    \x02\x06\x12\x03K\x0b\x0f\n\x0c\n\x05\x04\t\x02\x02\x01\x12\x03K\x10\x15\
    \n\x0c\n\x05\x04\t\x02\x02\x03\x12\x03K\x18\x19\n\x0b\n\x04\x04\t\x02\
    \x03\x12\x03L\x02\x1f\n\x0c\n\x05\x04\t\x02\x03\x04\x12\x03L\x02\n\n\x0c\
    \n\x05\x04\t\x02\x03\x05\x12\x03L\x0b\x10\n\x0c\n\x05\x04\t\x02\x03\x01\
    \x12\x03L\x11\x1a\n\x0c\n\x05\x04\t\x02\x03\x03\x12\x03L\x1d\x1e\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
