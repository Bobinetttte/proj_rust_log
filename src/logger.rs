use crate::rules::{gravity_rule};
use crate::Event;

use colored::Colorize;

pub fn logger(array_events : &Vec<&Event>) {
    println!("===== STAR LOG =====");
    for event in array_events {
        if gravity_rule(&event.gravity_level) {
            println!("{}", r"\/\/\/ ALERT \/\/\/".red().bold());
        }
        println!(" id : {} | Type : {}", event.id, event.event_type);
    }
    println!("===== END LOG =====");
}