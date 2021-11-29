#[cfg(feature = "queue_registration")]
use crate::os::queue::Queue;

#[cfg(feature = "queue_registration")]
use crate::os::queue::QueueRegistry;

#[cfg(feature = "queue_registration")]
pub struct SimpleQueueRegistry {
    pub queues: Vec<Box<dyn Queue>>
}

impl SimpleQueueRegistry {
    pub fn new() -> Self {
        SimpleQueueRegistry {
            queues: Vec::new()
        }
    }
}

#[cfg(feature = "queue_registration")]
impl QueueRegistry for SimpleQueueRegistry {
    fn reg_queue(&mut self, queue: Box<dyn Queue>) {
        self.queues.push(queue);
    }
}