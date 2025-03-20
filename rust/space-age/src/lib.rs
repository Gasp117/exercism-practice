/// # Implement planet_years for planets
macro_rules! planet_years {
    ($t:ident, $e:expr) => {
        pub struct $t;
        impl Planet for $t {
            fn years_during(d: &Duration) -> f64 {
                d.years / $e
            }
        }
    };
}

/// # Duration
/// 
/// Store duration in Earth years
#[derive(Debug)]
pub struct Duration {
    pub years: f64,
}

/// Convet from seconds to Earth years.
impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration {
            years: (s as f64)/ 31_557_600.0,
        }
    }
}

/// # Boilerplate trait to be implemented through macros.
pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

// Trait implementation through macros
planet_years!(Mercury, 0.2408467);
planet_years!(Venus, 0.61519726);
planet_years!(Earth, 1.0);
planet_years!(Mars, 1.8808158);
planet_years!(Jupiter, 11.862615);
planet_years!(Saturn, 29.447498);
planet_years!(Uranus, 84.016846);
planet_years!(Neptune, 164.79132);
