use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (minutes, overflow_hours) = Clock::calculate_minutes(minutes);
        let hours = Clock::calculate_hours(hours + overflow_hours);
        Clock { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let (new_minutes, overflow_hours) = Clock::calculate_minutes(self.minutes + minutes);
        let new_hours = Clock::calculate_hours(self.hours + overflow_hours);
        Clock { hours:new_hours, minutes:new_minutes }
    }

    fn calculate_minutes(minutes: i32) -> (i32, i32) {
        let mut overflow_hours = (minutes / 60) % 24;
        let new_minutes = minutes % 60;
        if new_minutes < 0 {
            overflow_hours -= 1;
            return (60 + new_minutes, overflow_hours);
        }
        (new_minutes, overflow_hours)
    }

    fn calculate_hours(current_hours: i32) -> i32 {
        let total_hours = current_hours % 24;
        if total_hours < 0 {
            return 24 + total_hours;
        }
        total_hours
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
