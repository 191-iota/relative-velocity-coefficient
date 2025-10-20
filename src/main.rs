use core::f64;
use std::io;

struct WeekRecord {
    hyperfocus_hours_wd: u8,
    hyperfocus_hours_we: u8,
    book_pages: u8,
    side_skills_days: u8,
    constraints_days: u8,
    load: f64,
}

impl WeekRecord {
    fn u_hf(&self) -> f64 {
        let capped = f64::min(
            (self.hyperfocus_hours_wd + self.hyperfocus_hours_we) as f64,
            20.0,
        );
        (capped / 20.0).sqrt()
    }

    fn q(&self, weekday_q: f64, weekend_q: f64) -> f64 {
        let h_total = (self.hyperfocus_hours_wd + self.hyperfocus_hours_we) as f64;
        if h_total == 0.0 {
            return (weekday_q + weekend_q) / 2.0;
        }
        (weekday_q * self.hyperfocus_hours_wd as f64 + weekend_q * self.hyperfocus_hours_we as f64)
            / h_total
    }

    fn r(&self, optimal_pages: u8) -> f64 {
        let r = self.book_pages as f64 / optimal_pages as f64;
        r / (1.0 + r)
    }

    fn b(&self) -> f64 {
        let h = (self.hyperfocus_hours_wd + self.hyperfocus_hours_we) as f64;
        let o = f64::max(0.0, h / 20.0 - 1.0);
        f64::exp(-4.0 * o * o)
    }

    fn m(&self) -> f64 {
        (1.0 + 0.06 * (self.side_skills_days as f64 / 7.0))
            * (1.0 + 0.05 * (self.constraints_days as f64 / 7.0))
    }

    fn c(&self) -> f64 {
        let l = self.load.clamp(0.0, 1.0);
        if l > 0.5 {
            1.0 + 1.6 * (l - 0.5).powf(2.4) * 10.0 // nonlinear, sharper rise near 1.0
        } else {
            1.0 - 0.1 * (0.5 - l)
        }
    }

    fn calculate_velocity(&self) -> f64 {
        let base_velocity = 2.0 * self.q(0.55, 0.675) * self.u_hf() * self.r(125);
        (base_velocity * self.b() * self.m() * self.c()).clamp(0.0, 1.0)
    }
}

fn read_f64(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<f64>().unwrap_or(0.0)
}

fn read_u8(prompt: &str) -> u8 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<u8>().unwrap_or(0)
}

fn main() {
    let record = WeekRecord {
        hyperfocus_hours_wd: read_u8("Hyperfocus hours (weekday):"),
        hyperfocus_hours_we: read_u8("Hyperfocus hours (weekend):"),
        book_pages: read_u8("Book pages read:"),
        side_skills_days: read_u8("Side skill days met (0–7):"),
        constraints_days: read_u8("Constraint days met (0–7):"),
        load: read_f64("Work/school load (0–1):"),
    };

    let velocity = record.calculate_velocity();
    println!("\n=== Weekly Velocity: {:.3} ===", velocity);
}
