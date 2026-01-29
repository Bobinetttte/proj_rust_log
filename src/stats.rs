use crate::event::Gravity;
use crate::Event;

pub struct Stats {
    nb_events_total : u128,
    nb_events_warning : u128,
    nb_events_error : u128,
    nb_events_critical : u128,
    worse_gravity : Gravity,
}

impl Stats {

    pub fn init_stats() -> Self {
        Stats {
            nb_events_total: 0,
            nb_events_warning: 0,
            nb_events_error: 0,
            nb_events_critical: 0,
            worse_gravity: Gravity::Debug,
        }
    }


    pub fn calculate_stats(&mut self, event : &Event) {

        self.nb_events_total += 1;

        match event.gravity_level {
            Gravity::Debug => {},
            Gravity::Information => {},
            Gravity::Warning => {self.nb_events_warning += 1;},
            Gravity::Error => {self.nb_events_error += 1;},
            Gravity::Critical => {self.nb_events_critical += 1;},
        };

        if event.gravity_level != Gravity::Debug {
            if event.gravity_level != Gravity::Information {
                if event.gravity_level != Gravity::Warning {
                    if event.gravity_level != Gravity::Error {
                        self.worse_gravity = event.gravity_level.clone();
                    }
                    else {
                        self.worse_gravity = event.gravity_level.clone();
                    }
                }
                else {
                    if self.worse_gravity != Gravity::Error && self.worse_gravity != Gravity::Critical {
                        self.worse_gravity = event.gravity_level.clone();
                    }
                }
            }
            else {
                if self.worse_gravity != Gravity::Warning && self.worse_gravity != Gravity::Error && self.worse_gravity != Gravity::Critical {
                    self.worse_gravity = event.gravity_level.clone();
                }
            }
        }
    }

    pub fn get_stats(&self) -> (u128, u128, u128, u128, Gravity) {
        return (self.nb_events_total, self.nb_events_warning, self.nb_events_error, self.nb_events_critical, self.worse_gravity.clone());
    }
}
