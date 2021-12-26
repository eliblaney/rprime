use std::sync::{Arc, Mutex};

#[cfg(feature = "queue_registration")]
use crate::os::simple_queue_registry::SimpleQueueRegistry;

lazy_static! {
    static ref NUM_QUEUES: Mutex<u32> = Mutex::new(0);
}

#[cfg(feature = "queue_registration")]
lazy_static! {
    static ref QUEUE_REGISTRY: Arc<Mutex<Box<dyn QueueRegistry + Send>>> = Arc::new(Mutex::new(Box::new(SimpleQueueRegistry::new())));
}

pub enum QueueStatus {
    QueueOk, // message sent/received okay
    QueueNoMoreMsgs, // If non-blocking, all the messages have been drained.
    QueueUninitialized, // Queue wasn't initialized successfully
    QueueSizeMismatch, // attempted to send or receive with buffer too large, too small
    QueueSendError, // message send error
    QueueReceiveError, // message receive error
    QueueInvalidPriority, // invalid priority requested
    QueueEmptyBuffer, // supplied buffer is empty
    QueueFull, // queue was full when attempting to send a message
    QueueUnknownError // Unexpected error; can't match with returns
}

pub enum QueueBlocking {
    QueueBlocking,
    QueueNonblocking
}

pub trait Queue: Sync + Send {
    fn create(&self, name: String, depth: u32, msg_size: u32) -> QueueStatus;

    fn send(&self, buffer: [u8], size: u32, priority: u32, block: QueueBlocking) -> QueueStatus;
    fn receive(&self, buffer: [u8], size: u32, priority: u32, block: QueueBlocking) -> QueueStatus;

    fn get_name(&self) -> String;

    fn get_num_msgs(&self) -> u32;
    fn get_max_msgs(&self) -> u32;
    fn get_queue_size(&self) -> u32;
    fn get_msg_size(&self) -> u32;

}

pub fn get_num_queues() -> u32 {
    *NUM_QUEUES.lock().unwrap()
}

#[cfg(feature = "queue_registration")]
pub fn set_queue_registry(reg: Box<dyn QueueRegistry + Send>) {
    *QUEUE_REGISTRY.lock().unwrap() = reg;
}

#[cfg(feature = "queue_registration")]
pub trait QueueRegistry: Send {
    fn reg_queue(&mut self, queue: Box<dyn Queue>);
}
