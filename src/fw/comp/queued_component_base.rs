use crate::fw::comp::passive_component_base::PassiveComponentBase;
use crate::os::queue::Queue;
use crate::os::queue::QueueStatus;

pub enum MsgDispatchStatus {
    MsgDispatchOk,
    MsgDispatchEmpty,
    MsgDispatchError,
    MsgDispatchExit
}

pub trait QueuedComponent {
    // Dispatch a single method in the queue
    fn do_dispatch(&self) -> MsgDispatchStatus;
}

pub struct QueuedComponentBase<Q>
where Q: Queue
{
    pub base: PassiveComponentBase,
    pub queue: Box<Q>,
    msgs_dropped: u32
}

impl<Q: Queue> QueuedComponentBase<Q> {

    pub fn create_queue(&self, depth: u32, msg_size: u32) -> QueueStatus {
        #[cfg(feature = "object_names")]
        let queue_name: String = self.base.object.name.clone();
        #[cfg(not(feature = "object_names"))]
        let queue_name: String = format!("CompQ_{}", Queue::get_num_queues());

        self.queue.create(queue_name, depth, msg_size)
    }

    pub fn get_num_msgs_dropped(&self) -> u32 {
        self.msgs_dropped
    }

    pub fn inc_num_msgs_dropped(&mut self) {
        self.msgs_dropped += 1
    }

    #[cfg(feature = "object_names")]
    pub fn to_string(&self) -> String {
        format!("QueueComp: {}", self.base.object.name)
    }

    #[cfg(not(feature = "object_names"))]
    pub fn to_string(&self) -> String {
        format!("QueuedComponentBase")
    }
    
    pub fn new(#[cfg(feature = "object_names")] name: String, queue: Box<Q>) -> Self {
        QueuedComponentBase {
            base: PassiveComponentBase::new(#[cfg(feature = "object_names")] name),
            queue: queue,
            msgs_dropped: 0
        }
    }
}
