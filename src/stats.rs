use std::collections::HashMap;

use chrono::{Datelike, NaiveDate};

use crate::models::SavedTrack;
use crate::types::AllSavedTracks;
use crate::utils;

pub struct Stats {
    average_tracks_per_day: f64,
    average_tracks_per_week: f64,
    average_tracks_per_month: f64,
    average_tracks_per_year: f64,

    best_month: (String, i32),
}

impl Stats {
    pub fn new(tracks: AllSavedTracks) -> Self {
        Self {
            average_tracks_per_day: calculate_daily_average(&tracks),
            average_tracks_per_week: calculate_weekly_average(&tracks),
            average_tracks_per_month: calculate_monthly_average(&tracks),
            average_tracks_per_year: calculate_yearly_average(&tracks),

            best_month: calculate_best_month(&tracks),
        }
    }

    pub fn print(&self) {
        println!("STATS:\n");

        println!("Average Tracks Per Day:   {}", self.average_tracks_per_day);
        println!("Average Tracks Per Week:  {}", self.average_tracks_per_week);
        println!(
            "Average Tracks Per Month: {}",
            self.average_tracks_per_month
        );
        println!("Average Tracks Per Year:  {}", self.average_tracks_per_year);

        println!(
            "Best Month: {} ({} scrobbles)",
            self.best_month.0, self.best_month.1
        );
    }
}

fn calculate_daily_average(tracks: &[SavedTrack]) -> f64 {
    let mut groups: HashMap<NaiveDate, i32> = HashMap::new();

    tracks.into_iter().for_each(|track| {
        let group = groups.entry(track.date()).or_insert(0);
        *group = *group + 1
    });

    groups.iter().map(|g| g.1).sum::<i32>() as f64 / utils::get_total_days(&tracks) as f64
}

fn calculate_weekly_average(tracks: &[SavedTrack]) -> f64 {
    let mut groups: HashMap<u32, i32> = HashMap::new();

    tracks.into_iter().for_each(|track| {
        let group = groups.entry(track.date().iso_week().week()).or_insert(0);
        *group = *group + 1;
    });

    groups.iter().map(|g| g.1).sum::<i32>() as f64 / utils::get_total_weeks(&tracks) as f64
}

fn calculate_monthly_average(tracks: &[SavedTrack]) -> f64 {
    let mut groups: HashMap<u32, i32> = HashMap::new();

    tracks.iter().for_each(|track| {
        let group = groups.entry(track.date().month()).or_insert(0);
        *group += 1;
    });

    groups.iter().map(|g| g.1).sum::<i32>() as f64 / utils::get_total_months(&tracks) as f64
}

fn calculate_yearly_average(tracks: &[SavedTrack]) -> f64 {
    let mut groups: HashMap<i32, i32> = HashMap::new();

    tracks.iter().for_each(|track| {
        let group = groups.entry(track.date().year()).or_insert(0);
        *group += 1
    });

    groups.iter().map(|g| g.1).sum::<i32>() as f64 / utils::get_total_years(&tracks) as f64
}

fn calculate_best_month(tracks: &[SavedTrack]) -> (String, i32) {
    let mut groups: HashMap<String, i32> = HashMap::new();

    tracks.into_iter().for_each(|track| {
        let group = groups.entry(track.month_year()).or_insert(0);
        *group += 1
    });

    let mut group_vec = groups.iter().collect::<Vec<(&String, &i32)>>();
    group_vec.sort_by(|a, b| a.1.cmp(b.1));
    group_vec.reverse();

    let best_month = group_vec.get(0).expect("Error retrieving best month");

    (best_month.0.to_string(), *best_month.1)
}
