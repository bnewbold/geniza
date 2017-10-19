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
pub struct Index {
    // message fields
    field_type: ::protobuf::SingularField<::std::string::String>,
    content: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Index {}

impl Index {
    pub fn new() -> Index {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Index {
        static mut instance: ::protobuf::lazy::Lazy<Index> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Index,
        };
        unsafe {
            instance.get(Index::new)
        }
    }

    // required string type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type.clear();
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ::std::string::String) {
        self.field_type = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field_type(&mut self) -> &mut ::std::string::String {
        if self.field_type.is_none() {
            self.field_type.set_default();
        }
        self.field_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_field_type(&mut self) -> ::std::string::String {
        self.field_type.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_field_type(&self) -> &str {
        match self.field_type.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_field_type_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.field_type
    }

    // optional bytes content = 2;

    pub fn clear_content(&mut self) {
        self.content.clear();
    }

    pub fn has_content(&self) -> bool {
        self.content.is_some()
    }

    // Param is passed by value, moved
    pub fn set_content(&mut self, v: ::std::vec::Vec<u8>) {
        self.content = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_content(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.content.is_none() {
            self.content.set_default();
        }
        self.content.as_mut().unwrap()
    }

    // Take field
    pub fn take_content(&mut self) -> ::std::vec::Vec<u8> {
        self.content.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_content(&self) -> &[u8] {
        match self.content.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_content_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.content
    }

    fn mut_content_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.content
    }
}

impl ::protobuf::Message for Index {
    fn is_initialized(&self) -> bool {
        if self.field_type.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.field_type)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.content)?;
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
        if let Some(ref v) = self.field_type.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.content.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.field_type.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.content.as_ref() {
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Index {
    fn new() -> Index {
        Index::new()
    }

    fn descriptor_static(_: ::std::option::Option<Index>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "type",
                    Index::get_field_type_for_reflect,
                    Index::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "content",
                    Index::get_content_for_reflect,
                    Index::mut_content_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Index>(
                    "Index",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Index {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_content();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Index {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Index {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Stat {
    // message fields
    mode: ::std::option::Option<u32>,
    uid: ::std::option::Option<u32>,
    gid: ::std::option::Option<u32>,
    size: ::std::option::Option<u64>,
    blocks: ::std::option::Option<u64>,
    offset: ::std::option::Option<u64>,
    byteOffset: ::std::option::Option<u64>,
    mtime: ::std::option::Option<u64>,
    ctime: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Stat {}

impl Stat {
    pub fn new() -> Stat {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Stat {
        static mut instance: ::protobuf::lazy::Lazy<Stat> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Stat,
        };
        unsafe {
            instance.get(Stat::new)
        }
    }

    // required uint32 mode = 1;

    pub fn clear_mode(&mut self) {
        self.mode = ::std::option::Option::None;
    }

    pub fn has_mode(&self) -> bool {
        self.mode.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mode(&mut self, v: u32) {
        self.mode = ::std::option::Option::Some(v);
    }

    pub fn get_mode(&self) -> u32 {
        self.mode.unwrap_or(0)
    }

    fn get_mode_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.mode
    }

    fn mut_mode_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.mode
    }

    // optional uint32 uid = 2;

    pub fn clear_uid(&mut self) {
        self.uid = ::std::option::Option::None;
    }

    pub fn has_uid(&self) -> bool {
        self.uid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_uid(&mut self, v: u32) {
        self.uid = ::std::option::Option::Some(v);
    }

    pub fn get_uid(&self) -> u32 {
        self.uid.unwrap_or(0)
    }

    fn get_uid_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.uid
    }

    fn mut_uid_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.uid
    }

    // optional uint32 gid = 3;

    pub fn clear_gid(&mut self) {
        self.gid = ::std::option::Option::None;
    }

    pub fn has_gid(&self) -> bool {
        self.gid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gid(&mut self, v: u32) {
        self.gid = ::std::option::Option::Some(v);
    }

    pub fn get_gid(&self) -> u32 {
        self.gid.unwrap_or(0)
    }

    fn get_gid_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.gid
    }

    fn mut_gid_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.gid
    }

    // optional uint64 size = 4;

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

    // optional uint64 blocks = 5;

    pub fn clear_blocks(&mut self) {
        self.blocks = ::std::option::Option::None;
    }

    pub fn has_blocks(&self) -> bool {
        self.blocks.is_some()
    }

    // Param is passed by value, moved
    pub fn set_blocks(&mut self, v: u64) {
        self.blocks = ::std::option::Option::Some(v);
    }

    pub fn get_blocks(&self) -> u64 {
        self.blocks.unwrap_or(0)
    }

    fn get_blocks_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.blocks
    }

    fn mut_blocks_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.blocks
    }

    // optional uint64 offset = 6;

    pub fn clear_offset(&mut self) {
        self.offset = ::std::option::Option::None;
    }

    pub fn has_offset(&self) -> bool {
        self.offset.is_some()
    }

    // Param is passed by value, moved
    pub fn set_offset(&mut self, v: u64) {
        self.offset = ::std::option::Option::Some(v);
    }

    pub fn get_offset(&self) -> u64 {
        self.offset.unwrap_or(0)
    }

    fn get_offset_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.offset
    }

    fn mut_offset_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.offset
    }

    // optional uint64 byteOffset = 7;

    pub fn clear_byteOffset(&mut self) {
        self.byteOffset = ::std::option::Option::None;
    }

    pub fn has_byteOffset(&self) -> bool {
        self.byteOffset.is_some()
    }

    // Param is passed by value, moved
    pub fn set_byteOffset(&mut self, v: u64) {
        self.byteOffset = ::std::option::Option::Some(v);
    }

    pub fn get_byteOffset(&self) -> u64 {
        self.byteOffset.unwrap_or(0)
    }

    fn get_byteOffset_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.byteOffset
    }

    fn mut_byteOffset_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.byteOffset
    }

    // optional uint64 mtime = 8;

    pub fn clear_mtime(&mut self) {
        self.mtime = ::std::option::Option::None;
    }

    pub fn has_mtime(&self) -> bool {
        self.mtime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mtime(&mut self, v: u64) {
        self.mtime = ::std::option::Option::Some(v);
    }

    pub fn get_mtime(&self) -> u64 {
        self.mtime.unwrap_or(0)
    }

    fn get_mtime_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.mtime
    }

    fn mut_mtime_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.mtime
    }

    // optional uint64 ctime = 9;

    pub fn clear_ctime(&mut self) {
        self.ctime = ::std::option::Option::None;
    }

    pub fn has_ctime(&self) -> bool {
        self.ctime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ctime(&mut self, v: u64) {
        self.ctime = ::std::option::Option::Some(v);
    }

    pub fn get_ctime(&self) -> u64 {
        self.ctime.unwrap_or(0)
    }

    fn get_ctime_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.ctime
    }

    fn mut_ctime_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.ctime
    }
}

impl ::protobuf::Message for Stat {
    fn is_initialized(&self) -> bool {
        if self.mode.is_none() {
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
                    let tmp = is.read_uint32()?;
                    self.mode = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.uid = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.gid = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.size = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.blocks = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.offset = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.byteOffset = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.mtime = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.ctime = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.mode {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.uid {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.gid {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.size {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.blocks {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.offset {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.byteOffset {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.mtime {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.ctime {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.mode {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.uid {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.gid {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.size {
            os.write_uint64(4, v)?;
        }
        if let Some(v) = self.blocks {
            os.write_uint64(5, v)?;
        }
        if let Some(v) = self.offset {
            os.write_uint64(6, v)?;
        }
        if let Some(v) = self.byteOffset {
            os.write_uint64(7, v)?;
        }
        if let Some(v) = self.mtime {
            os.write_uint64(8, v)?;
        }
        if let Some(v) = self.ctime {
            os.write_uint64(9, v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Stat {
    fn new() -> Stat {
        Stat::new()
    }

    fn descriptor_static(_: ::std::option::Option<Stat>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "mode",
                    Stat::get_mode_for_reflect,
                    Stat::mut_mode_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "uid",
                    Stat::get_uid_for_reflect,
                    Stat::mut_uid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "gid",
                    Stat::get_gid_for_reflect,
                    Stat::mut_gid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "size",
                    Stat::get_size_for_reflect,
                    Stat::mut_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "blocks",
                    Stat::get_blocks_for_reflect,
                    Stat::mut_blocks_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "offset",
                    Stat::get_offset_for_reflect,
                    Stat::mut_offset_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "byteOffset",
                    Stat::get_byteOffset_for_reflect,
                    Stat::mut_byteOffset_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "mtime",
                    Stat::get_mtime_for_reflect,
                    Stat::mut_mtime_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "ctime",
                    Stat::get_ctime_for_reflect,
                    Stat::mut_ctime_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Stat>(
                    "Stat",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Stat {
    fn clear(&mut self) {
        self.clear_mode();
        self.clear_uid();
        self.clear_gid();
        self.clear_size();
        self.clear_blocks();
        self.clear_offset();
        self.clear_byteOffset();
        self.clear_mtime();
        self.clear_ctime();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Stat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Stat {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x10hyperdrive.proto\"5\n\x05Index\x12\x12\n\x04type\x18\x01\x20\x02(\
    \tR\x04type\x12\x18\n\x07content\x18\x02\x20\x01(\x0cR\x07content\"\xce\
    \x01\n\x04Stat\x12\x12\n\x04mode\x18\x01\x20\x02(\rR\x04mode\x12\x10\n\
    \x03uid\x18\x02\x20\x01(\rR\x03uid\x12\x10\n\x03gid\x18\x03\x20\x01(\rR\
    \x03gid\x12\x12\n\x04size\x18\x04\x20\x01(\x04R\x04size\x12\x16\n\x06blo\
    cks\x18\x05\x20\x01(\x04R\x06blocks\x12\x16\n\x06offset\x18\x06\x20\x01(\
    \x04R\x06offset\x12\x1e\n\nbyteOffset\x18\x07\x20\x01(\x04R\nbyteOffset\
    \x12\x14\n\x05mtime\x18\x08\x20\x01(\x04R\x05mtime\x12\x14\n\x05ctime\
    \x18\t\x20\x01(\x04R\x05ctimeJ\xd2\x06\n\x06\x12\x04\x02\0\x11\x01\n-\n\
    \x02\x04\0\x12\x04\x02\0\x05\x012!\x20From\x20hyperdrive:lib/messages.js\
    \n\n\n\n\x03\x04\0\x01\x12\x03\x02\x08\r\n\x0b\n\x04\x04\0\x02\0\x12\x03\
    \x03\x02\x1b\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03\x03\x02\n\n\x0c\n\x05\
    \x04\0\x02\0\x05\x12\x03\x03\x0b\x11\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\
    \x03\x12\x16\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x03\x19\x1a\n\x0b\n\x04\
    \x04\0\x02\x01\x12\x03\x04\x02\x1d\n\x0c\n\x05\x04\0\x02\x01\x04\x12\x03\
    \x04\x02\n\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x04\x0b\x10\n\x0c\n\x05\
    \x04\0\x02\x01\x01\x12\x03\x04\x11\x18\n\x0c\n\x05\x04\0\x02\x01\x03\x12\
    \x03\x04\x1b\x1c\n\n\n\x02\x04\x01\x12\x04\x07\0\x11\x01\n\n\n\x03\x04\
    \x01\x01\x12\x03\x07\x08\x0c\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x08\x02\
    \x1b\n\x0c\n\x05\x04\x01\x02\0\x04\x12\x03\x08\x02\n\n\x0c\n\x05\x04\x01\
    \x02\0\x05\x12\x03\x08\x0b\x11\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x08\
    \x12\x16\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x08\x19\x1a\n\x0b\n\x04\
    \x04\x01\x02\x01\x12\x03\t\x02\x1a\n\x0c\n\x05\x04\x01\x02\x01\x04\x12\
    \x03\t\x02\n\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x03\t\x0b\x11\n\x0c\n\
    \x05\x04\x01\x02\x01\x01\x12\x03\t\x12\x15\n\x0c\n\x05\x04\x01\x02\x01\
    \x03\x12\x03\t\x18\x19\n\x0b\n\x04\x04\x01\x02\x02\x12\x03\n\x02\x1a\n\
    \x0c\n\x05\x04\x01\x02\x02\x04\x12\x03\n\x02\n\n\x0c\n\x05\x04\x01\x02\
    \x02\x05\x12\x03\n\x0b\x11\n\x0c\n\x05\x04\x01\x02\x02\x01\x12\x03\n\x12\
    \x15\n\x0c\n\x05\x04\x01\x02\x02\x03\x12\x03\n\x18\x19\n\x0b\n\x04\x04\
    \x01\x02\x03\x12\x03\x0b\x02\x1b\n\x0c\n\x05\x04\x01\x02\x03\x04\x12\x03\
    \x0b\x02\n\n\x0c\n\x05\x04\x01\x02\x03\x05\x12\x03\x0b\x0b\x11\n\x0c\n\
    \x05\x04\x01\x02\x03\x01\x12\x03\x0b\x12\x16\n\x0c\n\x05\x04\x01\x02\x03\
    \x03\x12\x03\x0b\x19\x1a\n\x0b\n\x04\x04\x01\x02\x04\x12\x03\x0c\x02\x1d\
    \n\x0c\n\x05\x04\x01\x02\x04\x04\x12\x03\x0c\x02\n\n\x0c\n\x05\x04\x01\
    \x02\x04\x05\x12\x03\x0c\x0b\x11\n\x0c\n\x05\x04\x01\x02\x04\x01\x12\x03\
    \x0c\x12\x18\n\x0c\n\x05\x04\x01\x02\x04\x03\x12\x03\x0c\x1b\x1c\n\x0b\n\
    \x04\x04\x01\x02\x05\x12\x03\r\x02\x1d\n\x0c\n\x05\x04\x01\x02\x05\x04\
    \x12\x03\r\x02\n\n\x0c\n\x05\x04\x01\x02\x05\x05\x12\x03\r\x0b\x11\n\x0c\
    \n\x05\x04\x01\x02\x05\x01\x12\x03\r\x12\x18\n\x0c\n\x05\x04\x01\x02\x05\
    \x03\x12\x03\r\x1b\x1c\n\x0b\n\x04\x04\x01\x02\x06\x12\x03\x0e\x02!\n\
    \x0c\n\x05\x04\x01\x02\x06\x04\x12\x03\x0e\x02\n\n\x0c\n\x05\x04\x01\x02\
    \x06\x05\x12\x03\x0e\x0b\x11\n\x0c\n\x05\x04\x01\x02\x06\x01\x12\x03\x0e\
    \x12\x1c\n\x0c\n\x05\x04\x01\x02\x06\x03\x12\x03\x0e\x1f\x20\n\x0b\n\x04\
    \x04\x01\x02\x07\x12\x03\x0f\x02\x1c\n\x0c\n\x05\x04\x01\x02\x07\x04\x12\
    \x03\x0f\x02\n\n\x0c\n\x05\x04\x01\x02\x07\x05\x12\x03\x0f\x0b\x11\n\x0c\
    \n\x05\x04\x01\x02\x07\x01\x12\x03\x0f\x12\x17\n\x0c\n\x05\x04\x01\x02\
    \x07\x03\x12\x03\x0f\x1a\x1b\n\x0b\n\x04\x04\x01\x02\x08\x12\x03\x10\x02\
    \x1c\n\x0c\n\x05\x04\x01\x02\x08\x04\x12\x03\x10\x02\n\n\x0c\n\x05\x04\
    \x01\x02\x08\x05\x12\x03\x10\x0b\x11\n\x0c\n\x05\x04\x01\x02\x08\x01\x12\
    \x03\x10\x12\x17\n\x0c\n\x05\x04\x01\x02\x08\x03\x12\x03\x10\x1a\x1b\
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
