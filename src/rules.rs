use crate::event::Gravity;

pub fn gravity_rule(gravity_level : &Gravity) -> bool {

    let mut is_alert : bool = false; 
    if *gravity_level == Gravity::Error || *gravity_level == Gravity::Critical {
        is_alert = true;
    }

    return is_alert;
}