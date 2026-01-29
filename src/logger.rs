use crate::rules::{gravity_rule};
use crate::Event;
use crate::stats;

use colored::Colorize;

pub fn logger(array_events : &Vec<&Event>) {
    let mut stats = stats::Stats::init_stats();
    println!("===== STAR LOG =====");
    for event in array_events {
        stats.calculate_stats(&event);
        if gravity_rule(&event.gravity_level) {
            println!("{}", r"\/\/\/ ALERT \/\/\/".red().bold());
        }
        println!(" id : {} | Type : {}", event.id, event.event_type);
    }
    println!("===== END LOG =====");
    let final_stats = stats.get_stats();

    let (total, warning, error, critical) = final_stats;
    println!("Total log : {}", total);
    println!("Total warning : {}", warning);
    println!("Total error : {}", error);
    println!("Total critical : {}", critical);
}