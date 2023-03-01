// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;
pub const DEBUG: u8 = 10;
pub const INFO: u8 = 20;
pub const WARN: u8 = 30;
pub const ERROR: u8 = 40;
pub const FATAL: u8 = 50;

extern "C" {
    fn rcl_interfaces__msg__Log__init(msg: *mut Log) -> bool;
    fn rcl_interfaces__msg__Log__fini(msg: *mut Log);
    fn rcl_interfaces__msg__Log__are_equal(lhs: *const Log, rhs: *const Log) -> bool;
    fn rcl_interfaces__msg__Log__Sequence__init(msg: *mut LogSeqRaw, size: usize) -> bool;
    fn rcl_interfaces__msg__Log__Sequence__fini(msg: *mut LogSeqRaw);
    fn rcl_interfaces__msg__Log__Sequence__are_equal(lhs: *const LogSeqRaw, rhs: *const LogSeqRaw) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__msg__Log() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct Log {
    pub stamp: builtin_interfaces::UnsafeTime,
    pub level: u8,
    pub name: crate::msg::RosString<0>,
    pub msg: crate::msg::RosString<0>,
    pub file: crate::msg::RosString<0>,
    pub function: crate::msg::RosString<0>,
    pub line: u32,
}

impl Log {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { rcl_interfaces__msg__Log__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for Log {
    fn drop(&mut self) {
        unsafe { rcl_interfaces__msg__Log__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct LogSeqRaw {
    data: *mut Log,
    size: usize,
    capacity: usize,
}

/// Sequence of Log.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct LogSeq<const N: usize> {
    data: *mut Log,
    size: usize,
    capacity: usize,
}

impl<const N: usize> LogSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: LogSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { rcl_interfaces__msg__Log__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn null() -> Self {
        let msg: LogSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {data: msg.data, size: msg.size, capacity: msg.capacity }
    }

    pub fn as_slice(&self) -> &[Log] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [Log] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Log> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Log> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for LogSeq<N> {
    fn drop(&mut self) {
        let mut msg = LogSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { rcl_interfaces__msg__Log__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for LogSeq<N> {}
unsafe impl<const N: usize> Sync for LogSeq<N> {}


impl TypeSupport for Log {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__msg__Log()
        }
    }
}

impl PartialEq for Log {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            rcl_interfaces__msg__Log__are_equal(self, other)
        }
    }
}

impl<const N: usize> PartialEq for LogSeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = LogSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
            let msg2 = LogSeqRaw{data: other.data, size: other.size, capacity: other.capacity};
            rcl_interfaces__msg__Log__Sequence__are_equal(&msg1, &msg2)
        }
    }
}

