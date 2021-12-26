use std::any::Any;
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref NUM_TASKS: Mutex<u32> = Mutex::new(0);
}

lazy_static! {
    static ref TASK_REGISTRY: Arc<Mutex<Box<dyn TaskRegistry + Send>>> = Arc::new(Mutex::new(Box::new(SimpleTaskRegistry::new())));
}

pub type TaskRoutine = dyn Fn(dyn Any);

pub enum TaskStatus {
    TaskOk, // message sent/received okay
    TaskInvalidParams, // started task with invalid parameters
    TaskInvalidStack, // started with invalid stack size
    TaskUnknownError, // unexpected error return value
    TaskInvalidAffinity, // unable to set the task affinity
    TaskDelayError, // error trying to delay the task
    TaskJoinError, // error trying to join the task
    TaskErrorResources, // unable to allocate more tasks
    TaskErrorPermission, // permissions error setting-up tasks
}

pub type NativeUIntType = u32;
pub const TASK_DEFAULT: NativeUIntType = 0;
pub type TaskUInt = Option<NativeUIntType>;

pub trait Task: Sync + Send {
    fn start(&self, name: &str, routine: TaskRoutine, priority: TaskUInt, stack_size: TaskUInt, cpu_affinity: TaskUInt, identifer: TaskUInt) -> TaskStatus;

    fn get_identifier(&self) -> NativeUIntType;
    fn join(&self) -> (TaskStatus, i32); // Wait for task to finish. Returns status and return value of task routine
    fn suspend(&self, on_purpose: bool); // suspend task
    fn resume(&self); // resume execution of task
    fn was_suspended(&self) -> bool; // returns whether or not task was suspended on purpose
    fn is_suspended(&self) -> bool; // check with OS to see if it is suspended already
    fn is_started(&self) -> bool; // check to see if task is started
    fn set_started(&self, started: bool); // set task to started when thread is fully up. Avoids a vx_works race condition.
}

pub fn get_num_tasks() -> u32 {
    *NUM_TASKS.lock().unwrap()
}

pub fn set_task_registry(reg: Box<dyn TaskRegistry + Send>) {
    *TASK_REGISTRY.lock().unwrap() = reg;
}

pub trait TaskRegistry: Send {
    fn reg_task(&mut self, task: Box<dyn Task>);
}

pub struct SimpleTaskRegistry {
    pub tasks: Vec<Box<dyn Task>>
}

impl SimpleTaskRegistry {
    pub fn new() -> Self {
        SimpleTaskRegistry {
            tasks: Vec::new()
        }
    }
}

impl TaskRegistry for SimpleTaskRegistry {
    fn reg_task(&mut self, task: Box<dyn Task>) {
        self.tasks.push(task);
    }
}
