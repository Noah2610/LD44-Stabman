use super::component_prelude::*;

pub enum TimerType {
    Level,
    Global,
}

pub struct TimerUi {
    pub timer_type:  TimerType,
    pub text_prefix: String,
}

impl Component for TimerUi {
    type Storage = HashMapStorage<Self>;
}
