use std::sync::Arc;

use crate::{rcl, topic::subscriber::RCLSubscription};

pub struct SubscriberLoanedMessage<T> {
    subscription: Arc<RCLSubscription>,
    chunk: *mut T,
}

impl<T> SubscriberLoanedMessage<T> {
    pub(crate) fn new(subscription: Arc<RCLSubscription>, chunk: *mut T) -> Self {
        Self {
            subscription,
            chunk,
        }
    }

    pub(crate) fn get(&self) -> &mut T {
        unsafe { &mut *self.chunk }
    }

    pub(crate) fn as_mut_ptr(&self) -> *mut T {
        self.chunk
    }
}

impl<T> Drop for SubscriberLoanedMessage<T> {
    fn drop(&mut self) {
        let guard = rcl::MT_UNSAFE_FN.lock();
        let _ = guard.rcl_return_loaned_message_from_subscription(
            self.subscription.subscription.as_ref(),
            self.chunk as *const _ as *mut _,
        );
    }
}
