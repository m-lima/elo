pub trait Config: 'static + Copy + Send + Sync + std::fmt::Debug {
    const DEFAULT_VALUE: f64;

    fn updater(one: f64, two: f64, won: bool, challenge: bool) -> f64;
    fn decayer(last: crate::types::Millis, curr: crate::types::Millis, rating: f64) -> f64;
}

#[derive(Copy, Clone, Debug)]
pub struct Elo;

impl Elo {
    pub const DECAY_PER_SEC: f64 = 1.0 / (60.0 * 60.0 * 24.0);
}

impl Config for Elo {
    const DEFAULT_VALUE: f64 = skillratings::elo::EloRating::new().rating;

    fn updater(one: f64, two: f64, won: bool, challenge: bool) -> f64 {
        let ratings = skillratings::elo::elo(
            &skillratings::elo::EloRating { rating: one },
            &skillratings::elo::EloRating { rating: two },
            if won {
                &skillratings::Outcomes::WIN
            } else {
                &skillratings::Outcomes::LOSS
            },
            &skillratings::elo::EloConfig::new(),
        );
        let delta = ratings.0.rating - one;
        if challenge { delta * 3.0 } else { delta }
    }

    fn decayer(last: crate::types::Millis, curr: crate::types::Millis, rating: f64) -> f64 {
        let Ok(decay) = i32::try_from(i64::from(curr) / 1000 - i64::from(last) / 1000)
            .map(f64::from)
            .map(|diff| diff * Self::DECAY_PER_SEC)
        else {
            return Self::DEFAULT_VALUE;
        };

        if rating < Self::DEFAULT_VALUE {
            Self::DEFAULT_VALUE.min(rating + decay)
        } else if rating > Self::DEFAULT_VALUE {
            Self::DEFAULT_VALUE.max(rating - decay)
        } else {
            Self::DEFAULT_VALUE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macros::f64;

    #[test]
    fn no_time_no_change() {
        assert!(f64!(eq Elo::decayer(0.into(), 0.into(), 27.0), 27.0));
    }

    #[test]
    fn one_day_shift() {
        assert!(f64!(eq
            Elo::decayer(
                0.into(),
                (1000 * 60 * 60 * 24).into(),
                Elo::DEFAULT_VALUE + 27.0
            ),
            Elo::DEFAULT_VALUE + 26.0
        ));

        assert!(f64!(eq
            Elo::decayer(
                0.into(),
                (1000 * 60 * 60 * 24).into(),
                Elo::DEFAULT_VALUE - 27.0
            ),
            Elo::DEFAULT_VALUE - 26.0
        ));

        assert!(f64!(eq
            Elo::decayer(0.into(), (1000 * 60 * 60 * 24).into(), Elo::DEFAULT_VALUE),
            Elo::DEFAULT_VALUE
        ));
    }

    #[test]
    fn half_day_shift() {
        assert!(f64!(eq
            Elo::decayer(
                0.into(),
                (1000 * 60 * 60 * 12).into(),
                Elo::DEFAULT_VALUE + 27.0
            ),
            Elo::DEFAULT_VALUE + 26.5
        ));

        assert!(f64!(eq
            Elo::decayer(
                0.into(),
                (1000 * 60 * 60 * 12).into(),
                Elo::DEFAULT_VALUE - 27.0
            ),
            Elo::DEFAULT_VALUE - 26.5
        ));

        assert!(f64!(eq
            Elo::decayer(0.into(), (1000 * 60 * 60 * 12).into(), Elo::DEFAULT_VALUE),
            Elo::DEFAULT_VALUE
        ));
    }

    #[test]
    fn many_days_shift() {
        assert!(f64!(eq
            Elo::decayer(0.into(), (1000 * 60 * 60 * 100).into(), Elo::DEFAULT_VALUE),
            Elo::DEFAULT_VALUE
        ));

        assert!(f64!(eq
            Elo::decayer(0.into(), (1000 * 60 * 60 * 100).into(), Elo::DEFAULT_VALUE),
            Elo::DEFAULT_VALUE
        ));

        assert!(f64!(eq
            Elo::decayer(0.into(), (1000 * 60 * 60 * 48).into(), Elo::DEFAULT_VALUE),
            Elo::DEFAULT_VALUE
        ));
    }
}
