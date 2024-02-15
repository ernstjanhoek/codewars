macro_rules! update_time {
    ($time_data:expr, $time_parts:expr) => {
        let mut local_time: TimeParts = TimeParts::new(0, 0);
        if $time_parts.remainder >= Year::LIMIT {
            local_time = Year::calculate_time(&$time_parts);
            $time_data.year = Year::update_time_data(&local_time);
            $time_parts = local_time;
        }
        if $time_parts.remainder < Year::LIMIT && $time_parts.remainder >= Day::LIMIT { 
            local_time = Day::calculate_time(&$time_parts);
            $time_data.day = Day::update_time_data(&local_time);
            $time_parts = local_time;
        }

        if $time_parts.remainder < Day::LIMIT && $time_parts.remainder >= Hour::LIMIT {
            local_time = Hour::calculate_time(&$time_parts);
            $time_data.hour = Hour::update_time_data(&local_time);
            $time_parts = local_time;
        }

        if $time_parts.remainder < Hour::LIMIT && $time_parts.remainder >= Minute::LIMIT {
            local_time = Minute::calculate_time(&$time_parts);
            $time_data.minute = Minute::update_time_data(&local_time);
            $time_parts = local_time;
        }

        if $time_parts.remainder < Minute::LIMIT && $time_parts.remainder >= 1 {
            local_time = Second::calculate_time(&$time_parts);
            $time_data.second = Second::update_time_data(&local_time);
            $time_parts = local_time;
        }
        local_time
    }
}

#[derive(Debug)]
struct TimeData {
    second: Option<Second>,
    minute: Option<Minute>,
    hour: Option<Hour>,
    day: Option<Day>,
    year: Option<Year>,
}

impl TimeData {
    fn new() -> Self {
        Self {
            second: None,
            minute: None,
            hour: None,
            day: None,
            year: None,
        }
    }

    fn derive_from_secs(mut self, seconds: u64) -> Self { 
        let mut time_parts = TimeParts::new(seconds, 0);
        while time_parts.time_left() {
            update_time!(self, time_parts);
        }
        self
    }

    fn build_string(&self) -> String {
        let mut out: Vec<String> = vec![];
        
        if self.day == None && self.hour == None && self.minute == None && self.second == None {
            out.push("now".to_string());
        }

        if let Some(time) = &self.year {
            out.push(time.to_string());
        }

        if let Some(time) = &self.day {
            out.push(time.to_string());
        }
        
        if let Some(time) = &self.hour {
            out.push(time.to_string());
        }
        
        if let Some(time) = &self.minute {
            out.push(time.to_string());
        }
        
        if let Some(time) = &self.second {
            out.push(time.to_string());
        }

        if out.len() > 1 {
            let index = out.len() - 2;
            out[index].push_str(" and ");
            if out.len() > 2 {
                for i in 0..out.len()-2 {
                    out[i].push_str(", "); 
                }
            }
        }

        out.join("").trim().to_string()
    }
}

#[derive(Debug, PartialEq)]
struct Second {
    amount: u64
}

impl Second {
    fn to_string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.amount.to_string());
        out.push_str(&" second");
        if self.amount > 1 { out.push_str(&"s"); }
        out
    }
}

impl TimeCalculation for Second {
    const LIMIT: u64 = 1;

    fn update_time_data(time_parts: &TimeParts) -> Option<Self> {
        Some(Self { 
            amount: time_parts.quotient,
        })
    }
}

#[derive(Debug, PartialEq)]
struct Minute {
    amount: u64
}

impl Minute {
    fn to_string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.amount.to_string());
        out.push_str(&" minute");
        if self.amount > 1 { out.push_str(&"s"); }
        out
    }
}

impl TimeCalculation for Minute {
    const LIMIT: u64 = 60;

    fn update_time_data(time_parts: &TimeParts) -> Option<Self> {
        Some(Self { 
            amount: time_parts.quotient,
        })
    }
}

#[derive(Debug, PartialEq)]
struct Hour {
    amount: u64
}

impl Hour {
    fn to_string(&self) -> String{
        let mut out = String::new();
        out.push_str(&self.amount.to_string());
        out.push_str(&" hour");
        if self.amount > 1 { out.push_str(&"s"); }
        out
    }
}

impl TimeCalculation for Hour {
    const LIMIT: u64 = 3600;

    fn update_time_data(time_parts: &TimeParts) -> Option<Self> {
        Some(Self { 
            amount: time_parts.quotient,
        })
    }
}

#[derive(Debug, PartialEq)]
struct Day {
    amount: u64
}

impl Day {
    fn to_string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.amount.to_string());
        out.push_str(&" day");
        if self.amount > 1 { out.push_str(&"s"); }
        out
    }
}

impl TimeCalculation for Day {
    const LIMIT: u64 = 86400;

    fn update_time_data(time_parts: &TimeParts) -> Option<Self> {
        Some(Self { 
            amount: time_parts.quotient,
        })
    }
}

#[derive(Debug)]
struct Year {
    amount: u64
}

impl Year {
    fn to_string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.amount.to_string());
        out.push_str(&" year");
        if self.amount > 1 { out.push_str(&"s"); }
        out
    }
}

impl TimeCalculation for Year {
    const LIMIT: u64 = 31536000;

    fn update_time_data(time_parts: &TimeParts) -> Option<Self> {
        Some(Self { 
            amount: time_parts.quotient,
        })
    }
}

#[derive(Copy, Clone, Debug)]
struct TimeParts {
    remainder: u64,
    quotient: u64
}

impl TimeParts {
    fn new(remainder: u64, quotient: u64) -> Self {
        Self {
            remainder,
            quotient,
        }
    }

    fn time_left(&self) -> bool {
        self.remainder > 0 
    }
}

trait TimeCalculation {
    const LIMIT: u64;

    fn calculate_time(time_parts: &TimeParts) -> TimeParts where Self: Sized {
        TimeParts::new(time_parts.remainder % Self::LIMIT, time_parts.remainder / Self::LIMIT)
    }

    fn update_time_data(time_parts: &TimeParts) -> Option<Self> where Self: Sized;
}


fn format_duration(seconds: u64) -> String {
    TimeData::new().derive_from_secs(seconds).build_string()
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::format_duration;

    #[test]
    fn test_basic() {
        assert_eq!(format_duration(1), "1 second");
        assert_eq!(format_duration(62), "1 minute and 2 seconds");
        assert_eq!(format_duration(120), "2 minutes");
        assert_eq!(format_duration(3600), "1 hour");
        assert_eq!(format_duration(3662), "1 hour, 1 minute and 2 seconds");
        assert_eq!(format_duration(333662662), "10 years, 211 days, 20 hours, 4 minutes and 22 seconds");
    }
}
