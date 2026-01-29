mod event;
mod rules;
mod logger;
use crate::event::Event;
use crate::event::Gravity;
use crate::logger::{logger};

fn main () {


    /*
        Events with id, type and gravity level
    */

    // Debug
    let event_debug = Event {
        id : String::from("1001"),
        event_type : String::from("ConfigLoaded") ,
        gravity_level : Gravity::Debug,
    };

    // Information
    let event_info = Event {
        id : String::from("2001"),
        event_type : String::from("UserLogin"),
        gravity_level : Gravity::Information,
    };
    
    // Warning
    let event_warning = Event {
        id : String::from("3001"),
        event_type : String::from("MultipleAttempts"),
        gravity_level : Gravity::Warning,
    };
    
    // Error
    let event_error = Event {
        id : String::from("4001"),
        event_type : String::from("AccessDenied"),
        gravity_level : Gravity::Error,
    };
    
    // Critical
    let event_critical = Event {
        id : String::from("5001"),
        event_type : String::from("SystemFailure"),
        gravity_level : Gravity::Critical,
    };

    let array_of_events = vec![&event_debug, &event_info, &event_warning, &event_error, &event_critical];

    logger(&array_of_events);

}