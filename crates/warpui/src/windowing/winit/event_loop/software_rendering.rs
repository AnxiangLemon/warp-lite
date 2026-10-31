use instant::{Duration, Instant};

use crate::rendering::FrameRateLimit;

const FRAME_INTERVAL: Duration = Duration::from_micros(33_334);
const MAX_RECOVERY_INTERVAL: Duration = Duration::from_millis(100);

#[derive(Default)]
pub(super) struct FramePacer {
    next_frame: Option<Instant>,
    timer_pending: bool,
    interval: Option<Duration>,
    adaptive: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum FrameRequest {
    Draw,
    Schedule(Duration),
    Pending,
}

impl FramePacer {
    pub fn configure(&mut self, limit: FrameRateLimit, software: bool) {
        let interval = match limit {
            FrameRateLimit::Automatic if software => Some(FRAME_INTERVAL),
            FrameRateLimit::Automatic | FrameRateLimit::Unlimited => None,
            FrameRateLimit::Fps15 => Some(Duration::from_micros(66_667)),
            FrameRateLimit::Fps30 => Some(FRAME_INTERVAL),
            FrameRateLimit::Fps60 => Some(Duration::from_micros(16_667)),
        };
        let adaptive = limit == FrameRateLimit::Automatic && software;
        if self.interval != interval || self.adaptive != adaptive {
            self.next_frame = None;
        }
        self.interval = interval;
        self.adaptive = adaptive;
    }

    pub fn request(&mut self, now: Instant) -> FrameRequest {
        if self.interval.is_none() {
            return FrameRequest::Draw;
        }
        if self.timer_pending {
            return FrameRequest::Pending;
        }
        if let Some(deadline) = self.next_frame
            && deadline > now
        {
            self.timer_pending = true;
            return FrameRequest::Schedule(deadline - now);
        }
        FrameRequest::Draw
    }

    pub fn timer_elapsed(&mut self) {
        self.timer_pending = false;
    }

    pub fn frame_finished(&mut self, started: Instant, finished: Instant) {
        // Slow software frames need recovery time as well as a frame-rate ceiling, so rendering
        // cannot continuously occupy the UI thread on a small server.
        let elapsed = finished - started;
        let recovery = if self.adaptive {
            elapsed.min(MAX_RECOVERY_INTERVAL)
        } else {
            Duration::ZERO
        };
        self.next_frame = self
            .interval
            .map(|interval| (started + interval).max(finished + recovery));
    }
}

#[cfg(test)]
#[path = "software_rendering_tests.rs"]
mod tests;
