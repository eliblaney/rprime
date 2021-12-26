use crate::fw::comp::queued_component_base::QueuedComponentBase;
use crate::fw::comp::queued_component_base::QueuedComponent;
use crate::os::queue::Queue;
use crate::os::task::Task;

pub enum ActiveComponentExit {
    ActiveComponentExit
}

pub trait ActiveComponent: QueuedComponent {
    fn preamble(&self);
    fn loop_dispatch(&self);
    fn finalizer(&self);
}

pub struct ActiveComponentBase<Q, T>
where Q: Queue, T: Task
{
    pub base: QueuedComponentBase<Q>,
    pub task: Box<T>
}

impl<Q: Queue, T: Task> ActiveComponentBase<Q, T> {

    #[cfg(feature = "object_names")]
    pub fn to_string(&self) -> String {
        format!("ActiveComp: {}", self.base.base.object.name)
    }

    #[cfg(not(feature = "object_names"))]
    pub fn to_string(&self) -> String {
        format!("ActiveComponentBase")
    }
    
    pub fn new(#[cfg(feature = "object_names")] name: String, queue: Box<Q>, task: Box<T>) -> Self {
        ActiveComponentBase {
            base: QueuedComponentBase::new(#[cfg(feature = "object_names")] name, queue),
            task: task
        }
    }
}
