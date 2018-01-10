//! A thing that consumes Updates, or something
//! The plan is to have a Dispatcher, that gets Updates from somewhere, maybe
//! does stuff to them, and then calls Command callbacks however it wants

use objects::UpdateKind;

pub trait Dispatcher {
    fn new() -> Self;
    fn dispatch_update(&mut self, update: UpdateKind) -> Result<(), String>;
}

pub struct SimpleDispatcher {}

impl SimpleDispatcher {}

impl Dispatcher for SimpleDispatcher {
    fn new() -> SimpleDispatcher {
        SimpleDispatcher {}
    }

    fn dispatch_update(&mut self, update: UpdateKind) -> Result<(), String> {
        unimplemented!();
    }
}
