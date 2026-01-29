mod event;
use crate::event::Event;

fn main () {

    /*
        Events with id, type and gravity level
    */

    // Debug
    let event_debug = Event {
        id : 00,
        event_type : String::from("Debug"),
        gravity_level : 0,
    };

    // Information
    let event_info = Event {
        id : 01,
        event_type : String::from("Information"),
        gravity_level : 0,
    };
    
    // Warning
    let event_warning = Event {
        id : 02,
        event_type : String::from("Warning"),
        gravity_level : 1,
    };
    
    // Error
    let event_error = Event {
        id : 03,
        event_type : String::from("Error"),
        gravity_level : 2,
    };
    
    // Critical
    let event_critical = Event {
        id : 04,
        event_type : String::from("Critical"),
        gravity_level : 3,
    };

    

}