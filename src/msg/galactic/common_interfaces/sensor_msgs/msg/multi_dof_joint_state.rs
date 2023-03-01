// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn sensor_msgs__msg__MultiDOFJointState__init(msg: *mut MultiDOFJointState) -> bool;
    fn sensor_msgs__msg__MultiDOFJointState__fini(msg: *mut MultiDOFJointState);
    fn sensor_msgs__msg__MultiDOFJointState__are_equal(lhs: *const MultiDOFJointState, rhs: *const MultiDOFJointState) -> bool;
    fn sensor_msgs__msg__MultiDOFJointState__Sequence__init(msg: *mut MultiDOFJointStateSeqRaw, size: usize) -> bool;
    fn sensor_msgs__msg__MultiDOFJointState__Sequence__fini(msg: *mut MultiDOFJointStateSeqRaw);
    fn sensor_msgs__msg__MultiDOFJointState__Sequence__are_equal(lhs: *const MultiDOFJointStateSeqRaw, rhs: *const MultiDOFJointStateSeqRaw) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__MultiDOFJointState() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct MultiDOFJointState {
    pub header: std_msgs::msg::Header,
    pub joint_names: crate::msg::RosStringSeq<0, 0>,
    pub transforms: geometry_msgs::msg::TransformSeq<0>,
    pub twist: geometry_msgs::msg::TwistSeq<0>,
    pub wrench: geometry_msgs::msg::WrenchSeq<0>,
}

impl MultiDOFJointState {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__msg__MultiDOFJointState__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for MultiDOFJointState {
    fn drop(&mut self) {
        unsafe { sensor_msgs__msg__MultiDOFJointState__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct MultiDOFJointStateSeqRaw {
    data: *mut MultiDOFJointState,
    size: usize,
    capacity: usize,
}

/// Sequence of MultiDOFJointState.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct MultiDOFJointStateSeq<const N: usize> {
    data: *mut MultiDOFJointState,
    size: usize,
    capacity: usize,
}

impl<const N: usize> MultiDOFJointStateSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: MultiDOFJointStateSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__msg__MultiDOFJointState__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn null() -> Self {
        let msg: MultiDOFJointStateSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {data: msg.data, size: msg.size, capacity: msg.capacity }
    }

    pub fn as_slice(&self) -> &[MultiDOFJointState] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [MultiDOFJointState] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, MultiDOFJointState> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, MultiDOFJointState> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for MultiDOFJointStateSeq<N> {
    fn drop(&mut self) {
        let mut msg = MultiDOFJointStateSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { sensor_msgs__msg__MultiDOFJointState__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for MultiDOFJointStateSeq<N> {}
unsafe impl<const N: usize> Sync for MultiDOFJointStateSeq<N> {}


impl TypeSupport for MultiDOFJointState {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__MultiDOFJointState()
        }
    }
}

impl PartialEq for MultiDOFJointState {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            sensor_msgs__msg__MultiDOFJointState__are_equal(self, other)
        }
    }
}

impl<const N: usize> PartialEq for MultiDOFJointStateSeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = MultiDOFJointStateSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
            let msg2 = MultiDOFJointStateSeqRaw{data: other.data, size: other.size, capacity: other.capacity};
            sensor_msgs__msg__MultiDOFJointState__Sequence__are_equal(&msg1, &msg2)
        }
    }
}

