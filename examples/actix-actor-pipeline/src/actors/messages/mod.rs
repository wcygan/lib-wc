use actix::prelude::*;

#[derive(Debug, Clone, Message)]
#[rtype(result = "()")]
pub struct Tick;

#[derive(Debug, Clone, Message)]
#[rtype(result = "()")]
pub struct NewValue {
    pub(crate) value: u32,
}

#[derive(Debug, Clone, Message)]
#[rtype(result = "()")]
pub struct AddedValue {
    pub(crate) value: u32,
}

#[derive(Debug, Clone, Message)]
#[rtype(result = "()")]
pub struct MultipliedValue {
    pub(crate) value: u32,
}
