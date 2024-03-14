use std::{cell::RefCell, rc::Rc};

// Define the Observer trait
trait Observer {
    fn update(&self, message: &str);
}

// Define the Subject trait
trait Subject {
    fn register_observer(&mut self, observer: Rc<RefCell<dyn Observer>>);
    fn remove_observer(&mut self, observer: Rc<RefCell<dyn Observer>>);
    fn notify_observers(&self);
}

