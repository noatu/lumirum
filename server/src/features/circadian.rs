use chrono::{
    DateTime,
    Duration,
    NaiveDate,
    NaiveTime,
    Offset,
    TimeZone,
    Timelike,
    Utc,
};
use chrono_tz::Tz;
use serde::{
    Deserialize,
    Serialize,
};
use sunrise::{
    Coordinates,
    SolarDay,
    SolarEvent,
};
use utoipa::ToSchema;

use crate::{
    errors::Error,
    features::profiles::Profile,
};

/// Lighting schedule data sent to device
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LightingSchedule {
    pub profile_id: i64,

    /// Seconds since midnight UTC when sleep starts
    pub sleep_start_utc_seconds: u32,
    /// Seconds since midnight UTC when sleep ends
    pub sleep_end_utc_seconds: u32,

    /// In Kelvin
    pub min_color_temp: i32,
    /// In Kelvin
    pub max_color_temp: i32,

    pub night_mode_enabled: bool,
    pub motion_timeout_seconds: i32,

    pub generated_at: DateTime<Utc>,
    pub valid_until: DateTime<Utc>,

    /// Lookup table of lighting data points
    pub schedule: Vec<LightingPoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LightingPoint {
    /// Absolute UTC timestamp
    #[serde(rename = "utc")]
    pub timestamp: DateTime<Utc>,
    /// Color temperature in Kelvin
    #[serde(rename = "temp")]
    pub color_temp: i32,
}

/// Convert local time to seconds since midnight UTC
fn to_utc_seconds_from_midnight(local_time: NaiveTime, timezone: Tz) -> u32 {
    let now_in_tz = Utc::now().with_timezone(&timezone);
    let today_naive = now_in_tz.date_naive();

    let local_naive_dt = today_naive.and_time(local_time);

    match timezone.from_local_datetime(&local_naive_dt) {
        chrono::LocalResult::Single(dt) | chrono::LocalResult::Ambiguous(dt, _) => {
            dt.with_timezone(&Utc).num_seconds_from_midnight()
        }
        chrono::LocalResult::None => {
            let offset_seconds = now_in_tz.offset().fix().local_minus_utc();
            let forced_utc = local_naive_dt - chrono::Duration::seconds(offset_seconds.into());
            forced_utc.num_seconds_from_midnight()
        }
    }
}

impl Profile {
    /// Compute a lighting schedule for a profile
    pub fn calculate(&self, points: u16, offset: Duration) -> Result<LightingSchedule, Error> {
        let now = Utc::now();
        let timezone = self.timezone.parse()?;

        let mut schedule = Vec::with_capacity(points.into());

        for timestamp in (0..points.into()).map(|i| now + offset * i) {
            schedule.push(LightingPoint {
                timestamp,
                color_temp: self.calculate_at_time(&timestamp.with_timezone(&timezone))?,
            });
        }

        Ok(LightingSchedule {
            profile_id: self.id,
            sleep_start_utc_seconds: to_utc_seconds_from_midnight(self.sleep_start, timezone),
            sleep_end_utc_seconds: to_utc_seconds_from_midnight(self.sleep_end, timezone),
            min_color_temp: self.min_color_temp,
            max_color_temp: self.max_color_temp,
            night_mode_enabled: self.night_mode_enabled,
            motion_timeout_seconds: self.motion_timeout_seconds,
            generated_at: now,
            valid_until: now + offset * points.into(),
            schedule,
        })
    }

    /// Calculate color temperature for a specific local time
    fn calculate_at_time<T: TimeZone>(&self, local_time: &DateTime<T>) -> Result<i32, Error> {
        let (_, sunset_time) = if let (Some(lat), Some(lon)) = (self.latitude, self.longitude) {
            Self::calculate_solar_times(local_time.date_naive(), lat, lon)?
        } else {
            self.estimate_solar_times()
        };

        // Convert all times to minutes from midnight for easier comparison
        let current_minutes = local_time.time().num_seconds_from_midnight() / 60;

        let sleep_start_minutes = self.sleep_start.num_seconds_from_midnight() / 60;
        let sleep_end_minutes = self.sleep_end.num_seconds_from_midnight() / 60;

        let sunset_minutes = sunset_time.num_seconds_from_midnight() / 60;

        // Determine if we're in sleep period
        let in_sleep_period = if sleep_start_minutes < sleep_end_minutes {
            // Sleep doesn't cross midnight (e.g. 2:00-10:00)
            current_minutes >= sleep_start_minutes && current_minutes < sleep_end_minutes
        } else {
            // Sleep crosses midnight (e.g. 22:00-06:00)
            current_minutes >= sleep_start_minutes || current_minutes < sleep_end_minutes
        };

        if in_sleep_period {
            return Ok(self.min_color_temp);
        }

        // We start winding down 60 mins before bed
        let pre_sleep_minutes = if sleep_start_minutes >= 60 {
            sleep_start_minutes - 60 // 1 hour before sleep
        } else {
            23 * 60 + sleep_start_minutes // Handle midnight crossing
        };

        // Calculate color temperature using piecewise function
        Ok(self.interpolate_circadian_curve(
            current_minutes,
            sleep_end_minutes,   // Wake time
            sleep_start_minutes, // Sleep time
            pre_sleep_minutes,   // Wind down time
            sunset_minutes,
        ))
    }

    /// Calculate sunrise and sunset times using astronomical algorithms
    fn calculate_solar_times(
        date: NaiveDate,
        latitude: f64,
        longitude: f64,
    ) -> Result<(NaiveTime, NaiveTime), Error> {
        let coord = Coordinates::new(latitude, longitude).ok_or(Error::DataCorruption(format!(
            "Invalid coordinates: {latitude}, {longitude}"
        )))?;
        let solar_day = SolarDay::new(coord, date);

        let sunrise = solar_day.event_time(SolarEvent::Sunrise).time();
        let sunset = solar_day.event_time(SolarEvent::Sunset).time();

        Ok((sunrise, sunset))
    }

    /// Estimate solar times based on sleep schedule when location is missing:
    /// - Sunrise aligns with Wake Up time (user needs light to wake up).
    /// - Sunset is approximated 2 hours before Sleep Start to allow for an evening relaxation phase.
    fn estimate_solar_times(&self) -> (NaiveTime, NaiveTime) {
        let sleep_start_seconds = self.sleep_start.num_seconds_from_midnight();
        let offset_seconds = 2 * 3600; // 2 hours

        // Handle wrapping around midnight
        let sunset_seconds = if sleep_start_seconds >= offset_seconds {
            sleep_start_seconds - offset_seconds
        } else {
            // e.g. sleep at 01:00, 3600s - 2h = 23:00 prev day
            24 * 3600 - (offset_seconds - sleep_start_seconds)
        };

        #[allow(clippy::expect_used)]
        let sunset = NaiveTime::from_num_seconds_from_midnight_opt(sunset_seconds, 0)
            .unwrap_or(NaiveTime::from_hms_opt(20, 0, 0).expect("valid time"));

        (self.sleep_end, sunset)
    }

    /// Calculates curve with 4 phases:
    /// 1. Wake -> Morning Boost (Min -> Max)
    /// 2. Day -> Sunset (Hold Max)
    /// 3. Sunset -> Pre-Sleep (Max -> Relaxation Temp)
    /// 4. Pre-Sleep -> Sleep (Relaxation Temp -> Min)
    fn interpolate_circadian_curve(
        &self,
        current_minutes: u32,
        wake_minutes: u32,
        sleep_minutes: u32,
        pre_sleep_minutes: u32,
        sunset_minutes: u32,
    ) -> i32 {
        // Define an "Evening/Relaxation" temperature
        // This is warmer than daylight but brighter than nightlight
        // e.g. if Max=6500, Min=2000, Relax = 3500
        let relax_temp = self.min_color_temp + (self.max_color_temp - self.min_color_temp) / 3;

        // Helper to check time range which may wrap midnight
        let in_range = |curr: u32, start: u32, end: u32| -> bool {
            if start <= end {
                curr >= start && curr < end
            } else {
                curr >= start || curr < end
            }
        };

        // Helper for linear interpolation 0.0 -> 1.0
        let get_t = |curr: u32, start: u32, end: u32| -> f64 {
            if start <= end {
                if curr < start {
                    return 0.0;
                }
                if curr > end {
                    return 1.0;
                }
                f64::from(curr - start) / f64::from(end - start)
            } else {
                let total = f64::from(24 * 60 - start + end);
                let val = if curr >= start {
                    f64::from(curr - start)
                } else {
                    f64::from(24 * 60 - start + curr)
                };
                val / total
            }
        };

        // Morning ramp-up duration, 60 mins after waking
        let morning_end_minutes = (wake_minutes + 60) % 1440;

        // PHASE 1: Morning Boost (Wake -> Wake+1h)
        if in_range(current_minutes, wake_minutes, morning_end_minutes) {
            let t = get_t(current_minutes, wake_minutes, morning_end_minutes);
            // Ease-out (fast rise, slow finish)
            let t_eased = (1.0 - t).mul_add(-(1.0 - t), 1.0);
            #[allow(clippy::cast_possible_truncation)]
            return self.min_color_temp
                + (f64::from(self.max_color_temp - self.min_color_temp) * t_eased) as i32;
        }

        // PHASE 4: Final Wind Down (Pre-Sleep -> Sleep)
        // We check this before Phase 2/3 to ensure user sleep schedule overrides solar sunset
        if in_range(current_minutes, pre_sleep_minutes, sleep_minutes) {
            let t = get_t(current_minutes, pre_sleep_minutes, sleep_minutes);
            // Ease-in (slow drop start, fast finish)
            let t_eased = t.powi(2);
            // Drop from Relax Temp to Min Temp
            #[allow(clippy::cast_possible_truncation)]
            return relax_temp - (f64::from(relax_temp - self.min_color_temp) * t_eased) as i32;
        }

        // PHASE 3: Evening Relaxation (Sunset -> Pre-Sleep)
        // We need to check if Sunset happens before Pre-Sleep
        // If Sunset is super late (summer) or after pre_sleep, we skip this logic
        // NOTE: We use sunset_minutes as start, pre_sleep_minutes as end.
        if in_range(current_minutes, sunset_minutes, pre_sleep_minutes) {
            // Determine effective start/end for interpolation
            // If we are here, we are between sunset and pre-sleep
            let t = get_t(current_minutes, sunset_minutes, pre_sleep_minutes);
            // Linear drop from Max to Relax
            #[allow(clippy::cast_possible_truncation)]
            return self.max_color_temp - (f64::from(self.max_color_temp - relax_temp) * t) as i32;
        }

        // PHASE 2: Daylight (Morning End -> Sunset OR Pre-Sleep)
        // If we haven't hit sunset yet, or if sunset is weirdly placed, keep it bright.
        // Also captures the "gap" if sunrise is way before wake time (handled by default max).

        // Simply returning max_temp covers the remaining productive hours.
        self.max_color_temp
    }
}
