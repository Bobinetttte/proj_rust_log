use crate::rules::{gravity_rule};
use crate::Event;
use crate::stats;

use colored::Colorize;

pub fn logger(array_events : &Vec<&Event>) {
    let mut stats = stats::Stats::init_stats();
    println!("===== START LOG =====");
    for event in array_events {
        stats.calculate_stats(&event);
        if gravity_rule(&event.gravity_level) {
            println!("\n{}", r"<<<< ALERT >>>>".red().bold());
        }
        println!(" id : {} | Type : {}", event.id, event.event_type);
        if gravity_rule(&event.gravity_level) {
            println!("{}\n", r"<<<< ALERT >>>>".red().bold());
        }
    }
    println!("===== END LOG =====");
    let final_stats = stats.get_stats();

    let (nb_events_total, nb_events_warning, nb_events_error, nb_events_critical, worse_gravity) = final_stats;
    println!("Total log : {}", nb_events_total);
    println!("Total warning : {}", nb_events_warning);
    println!("Total error : {}", nb_events_error);
    println!("Total critical : {}", nb_events_critical);
    println!("Worse gravity level : {:?}", worse_gravity);
}
