use std::collections::HashMap;

use chrono::{Datelike, NaiveDate};

use crate::types::AllSavedTracks;
use crate::utils;

pub struct Stats {
    average_tracks_per_day: f64,
    average_tracks_per_week: f64,
}

impl Stats {
    pub fn new(tracks: AllSavedTracks) -> Self {
        let daily_average = calculate_daily_average(&tracks);
        let weekly_average = calculate_weekly_average(&tracks);

        Self {
            average_tracks_per_day: daily_average,
            average_tracks_per_week: weekly_average,
        }
    }

    pub fn print(&self) {
        println!("STATS:\n");

        println!("Average Tracks Per Day:  {}", self.average_tracks_per_day);
        println!("Average Tracks Per Week: {}", self.average_tracks_per_week);
    }
}

fn calculate_daily_average(tracks: &AllSavedTracks) -> f64 {
    let mut groups: HashMap<NaiveDate, i32> = HashMap::new();

    tracks.into_iter().for_each(|track| {
        let group = groups.entry(track.date()).or_insert(0);
        *group = *group + 1
    });

    groups.iter().map(|g| g.1).sum::<i32>() as f64 / utils::get_total_days(&tracks) as f64
}

fn calculate_weekly_average(tracks: &AllSavedTracks) -> f64 {
    let mut groups: HashMap<u32, i32> = HashMap::new();

    tracks.into_iter().for_each(|track| {
        let group = groups.entry(track.date().iso_week().week()).or_insert(0);
        *group = *group + 1;
    });

    groups.iter().map(|g| g.1).sum::<i32>() as f64 / utils::get_total_weeks(&tracks) as f64
}