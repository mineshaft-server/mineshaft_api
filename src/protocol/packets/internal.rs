pub use crate::protocol::util::*;
pub use crate::protocol::types::{
  chat::Chat,
  nbt::Tag,
  position::Position,
  slot::Slot
};
pub use crate::util::identifier::Identifier;
extern crate log;
pub use log::error;

#[macro_export]
macro_rules! internal_type {
  (chat) => {Chat};
  (identifier) => {Identifier};
  (slot) => {Slot};
  (string) => {String};
  (uuid) => {u128};
  (varint) => {i32};
  (varlong) => {i64};
  ($any:ty) => {$any};
}

#[macro_export]
macro_rules! default_type_value {
  (bool) => {false};
  (chat) => {Chat::new_object()};
  (identifier) => {Identifier::new("", "")};
  (slot) => {Slot { item_count: 0, item_type: 0, nbt: None}};
  (string) => {String::from("")};
  (varint) => {0};
  (varlong) => {0};
  ($any:ty) => {0};
}

#[macro_export]
macro_rules! write_field {
  ($func:ident $target:ident $field:ident $raw:ident) => {$func(&mut $raw, &$target.$field);};
}

#[macro_export]
macro_rules! write {
  (i8 $target:ident $field:ident $raw:ident) => { write_field!(write_byte $target $field $raw) };
  (u8 $target:ident $field:ident $raw:ident) => { write_field!(write_ubyte $target $field $raw) };
  (i16 $target:ident $field:ident $raw:ident) => { write_field!(write_short $target $field $raw) };
  (u16 $target:ident $field:ident $raw:ident) => { write_field!(write_ushort $target $field $raw) };
  (i32 $target:ident $field:ident $raw:ident) => { write_field!(write_int $target $field $raw) };
  (u32 $target:ident $field:ident $raw:ident) => { write_field!(write_uint $target $field $raw) };
  (i64 $target:ident $field:ident $raw:ident) => { write_field!(write_long $target $field $raw) };
  (u64 $target:ident $field:ident $raw:ident) => { write_field!(write_ulong $target $field $raw) };
  (f32 $target:ident $field:ident $raw:ident) => { write_field!(write_float $target $field $raw) };
  (f64 $target:ident $field:ident $raw:ident) => { write_field!(write_double $target $field $raw) };
  (bool $target:ident $field:ident $raw:ident) => {write_field!(write_bool $target $field $raw:ident)};
  (chat $target:ident $field:ident $raw:ident) => {write_field!(write_chat $target $field $raw) };
  (identifier $target:ident $field:ident $raw:ident) => { write_string(&mut $raw, &$target.$field.to_string()); };
  (nbt $target:ident $field:ident $raw:ident) => { write_field!(write_nbt $target $field $raw) };
  (position $target:ident $field:ident $raw:ident) => { write_field!(write_position $target $field $raw) };
  (slot $target:ident $field:ident $raw:ident) => { write_field!(write_slot $target $field $raw) };
  (string $target:ident $field:ident $raw:ident) => { write_field!(write_string $target $field ) };
  (uuid $target:ident $field:ident $raw:ident) => { write_field!(write_uuid $target $field $raw)};
  (varint $target:ident $field:ident $raw:ident) => { write_field!(write_varint $target $field $raw) };
  (varlong $target:ident $field:ident $raw:ident) => { write_field!(write_varlong $target $field $raw) };
}

#[macro_export]
macro_rules! read {
  (i8 $target:ident $field:ident $raw:ident) => {{
    if $raw.len() > 0 {
      $target.$field = read_byte($raw);
    } else {
      error!(target: "packet read", "Unable to read byte from buffer");
      return None;
    }
  }};
  (u8 $target:ident $field:ident $raw:ident) => {{
    if $raw.len() > 0 {
      $target.$field = read_ubyte($raw);
    } else {
      error!(target: "packet read", "Unable to read unsigned byte from buffer");
      return None;
    }
  }};
  (i16 $target:ident $field:ident $raw:ident) => {{
    if $raw.len() > 1 {
      $target.$field = read_short($raw);
    } else {
      error!(target: "packet read", "Unable to read short from buffer");
      return None;
    }
  }};
  (u16 $target:ident $field:ident $raw:ident) => {{
    if $raw.len() > 1 {
      $target.$field = read_ushort($raw);
    } else {
      error!(target: "packet read", "Unable to read unsigned short from buffer");
      return None;
    }
  }};
  (i32 $target:ident $field:ident $raw:ident) => {{
    if $raw.len() > 3 {
      $target.$field = read_int($raw);
    } else {
      error!(target: "packet read", "Unable to read int from buffer");
      return None;
    }
  }};
  (u32 $target:ident $field:ident $raw:ident) => {{
    if $raw.len() > 3 {
      $target.$field = read_uint($raw);
    } else {
      error!(target: "packet read", "Unable to read unsigned int from buffer");
      return None;
    }
  }};
  (i64 $target:ident $field:ident $raw:ident) => {{
    if $raw.len() > 7 {
      $target.$field = read_long($raw);
    } else {
      error!(target: "packet read", "Unable to read long from buffer");
      return None;
    }
  }};
  (u64 $target:ident $field:ident $raw:ident) => {{
    if $raw.len() > 7 {
      $target.$field = read_ulong($raw);
    } else {
      error!(target: "packet read", "Unable to read unsigned long from buffer");
      return None;
    }
  }};
  (f32 $target:ident $field:ident $raw:ident) => {{
    if $raw.len() > 3 {
      $target.$field = read_float($raw);
    } else {
      error!(target: "packet read", "Unable to read float from buffer");
      return None;
    }
  }};
  (f64 $target:ident $field:ident $raw:ident) => {{
    if $raw.len() > 7 {
      $target.$field = read_double($raw);
    } else {
      error!(target: "packet read", "Unable to read double from buffer");
      return None;
    }
  }};
  (bool $target:ident $field:ident $raw:ident) => {{
    if $raw.len() > 0 {
      $target.$field = read_bool($raw);
    } else {
      error!(target: "packet read", "Unable to read bool from buffer");
      return None;
    }
  }};
  (chat $target:ident $field:ident $raw:ident) => {{
    if let Some(chat) = read_chat($raw) {
      $target.$field = chat;
    } else {
      error!(target: "packet read", "Unable to read chat from buffer");
      return None;
    }
  }};
  (identifier $target:ident $field:ident $raw:ident) => {{
    let tmp = read_string($raw, String::from(""));
    if tmp != "" {
      $target.$field = Identifier::from(&tmp);
    } else {
      error!(target: "packet read", "Unable to read identifier from buffer");
      return None;
    }
  }};
  (nbt $target:ident $field:ident $raw:ident) => {{
    let tag = read_nbt($raw);
    if let Tag::TagInvalid(reason) = tag {
      error!(target: "packet read", "Unable to read nbt from buffer. Reason: {}", reason);
      return None;
    } else {
      $target.$field = tag;
    }
  }};
  (position $target:ident $field:ident $raw:ident) => {{
    if $raw.len() > 7 {
      $target.$field = read_position($raw);
    } else {
      error!(target: "packet read", "Unable to read position from buffer");
      return None;
    }
  }};
  (slot $target:ident $field:ident $raw:ident) => {{
    if $raw.len() > 0 {
      if let Some(slot) = read_slot($raw) {
        $target.$field = slot
      } else {
        error!(target: "packet read", "Unable to read slot from buffer");
        return None;
      }
    } else {
      error!(target: "packet read", "Unable to read slot from buffer. Buffer insufficient");
      return None;
    }
  }};
  (string $target:ident $field:ident $raw:ident) => {{
    let tmp = read_string($raw, String::from(""));
    if tmp != "" {
      $target.$field = tmp;
    } else {
      error!(target: "packet read", "Unable to read string from buffer");
      return None;
    }
  }};
  (uuid $target:ident $field:ident $raw:ident) => {{
    if raw.len() > 15 {
      $target.$field = read_uuid($raw);
    } else {
      error!(target: "packet read", "Unable to read uuid from buffer");
      return None;
    }
  }};
  (varint $target:ident $field:ident $raw:ident) => {{
    let tmp = read_varint($raw, -1);
    if tmp != -1 {
      $target.$field = tmp;
    } else {
      error!(target: "packet read", "Unable to read varint from buffer");
      return None;
    }
  }};
  (varlong $target:ident $field:ident $raw:ident) =>  {{
    let tmp = read_varlong($raw, -1);
    if tmp != -1 {
      $target.$field = tmp;
    } else {
      error!(target: "packet read", "Unable to read varlong from buffer");
      return None;
    }
  }};
}

#[macro_export]
macro_rules! define_packet {
  ($name:ident, {$($key:ident: $type:ident),*}) => {
    #[derive(Debug)]
    pub struct $name {
      $($key: internal_type!($type),)*
    }

    impl Packet for $name {
      fn default() -> Self {
        return $name {
          $($key: default_type_value!($type),)*
        }
      }

      fn to_raw(&self, packet_id: i32) -> Vec<u8>{
        let mut raw = Vec::new();
        $(write!($type self $key raw);)*
        return raw;
      }

      fn from_raw(raw: &mut Vec<u8>) -> Option<Self> {
        let mut packet = $name::default();
        $(read!($type packet $key raw);)*
        return Some(packet);
      }
    }
  }
}