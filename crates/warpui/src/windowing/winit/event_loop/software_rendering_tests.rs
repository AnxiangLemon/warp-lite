use super::*;

#[test]
fn idle_input_draws_immediately_and_bursts_share_one_timer() {
    let mut pacer = FramePacer::default();
    pacer.configure(FrameRateLimit::Automatic, true);
    let start = Instant::now();
    assert_eq!(pacer.request(start), FrameRequest::Draw);
    pacer.frame_finished(start, start + Duration::from_millis(2));
    assert_eq!(
        pacer.request(start + Duration::from_millis(3)),
        FrameRequest::Schedule(FRAME_INTERVAL - Duration::from_millis(3))
    );
    for offset in 4..100 {
        assert_eq!(
            pacer.request(start + Duration::from_millis(offset)),
            FrameRequest::Pending
        );
    }
    pacer.timer_elapsed();
    assert_eq!(
        pacer.request(start + Duration::from_millis(100)),
        FrameRequest::Draw
    );
}

#[test]
fn early_timer_preserves_the_final_pending_frame() {
    let mut pacer = FramePacer::default();
    pacer.configure(FrameRateLimit::Automatic, true);
    let start = Instant::now();
    pacer.frame_finished(start, start);
    assert_eq!(pacer.request(start), FrameRequest::Schedule(FRAME_INTERVAL));
    pacer.timer_elapsed();
    assert_eq!(
        pacer.request(start + Duration::from_millis(10)),
        FrameRequest::Schedule(FRAME_INTERVAL - Duration::from_millis(10))
    );
    pacer.timer_elapsed();
    assert_eq!(pacer.request(start + FRAME_INTERVAL), FrameRequest::Draw);
}

#[test]
fn slow_frames_leave_time_for_input_and_bound_the_recovery_delay() {
    for elapsed in [Duration::from_millis(40), Duration::from_secs(1)] {
        let mut pacer = FramePacer::default();
        pacer.configure(FrameRateLimit::Automatic, true);
        let start = Instant::now();
        let finished = start + elapsed;
        pacer.frame_finished(start, finished);
        assert_eq!(
            pacer.request(finished),
            FrameRequest::Schedule(elapsed.min(MAX_RECOVERY_INTERVAL))
        );
        pacer.timer_elapsed();
        assert_eq!(
            pacer.request(finished + elapsed.min(MAX_RECOVERY_INTERVAL)),
            FrameRequest::Draw
        );
    }
}

#[test]
fn continuous_requests_cannot_exceed_thirty_frames_per_second() {
    let mut pacer = FramePacer::default();
    pacer.configure(FrameRateLimit::Automatic, true);
    let start = Instant::now();
    let mut frames = 0;
    for millis in 0..1000 {
        let now = start + Duration::from_millis(millis);
        // Even an over-eager native paint/timer stream must respect the deadline.
        pacer.timer_elapsed();
        match pacer.request(now) {
            FrameRequest::Draw => {
                frames += 1;
                pacer.frame_finished(now, now);
            }
            FrameRequest::Schedule(_) | FrameRequest::Pending => {}
        }
    }
    assert_eq!(frames, 30);
}

#[test]
fn explicit_limits_apply_to_both_devices_without_adaptive_recovery() {
    for software in [false, true] {
        for (limit, expected) in [
            (FrameRateLimit::Fps15, 15),
            (FrameRateLimit::Fps30, 30),
            (FrameRateLimit::Fps60, 59),
        ] {
            let mut pacer = FramePacer::default();
            pacer.configure(limit, software);
            let start = Instant::now();
            let mut frames = 0;
            for millis in 0..1000 {
                let now = start + Duration::from_millis(millis);
                pacer.timer_elapsed();
                if pacer.request(now) == FrameRequest::Draw {
                    frames += 1;
                    pacer.frame_finished(now, now + Duration::from_millis(10));
                }
            }
            assert_eq!(frames, expected, "{limit:?}, software={software}");
        }
    }
}

#[test]
fn unlimited_and_automatic_hardware_do_not_throttle() {
    for (limit, software) in [
        (FrameRateLimit::Unlimited, true),
        (FrameRateLimit::Unlimited, false),
        (FrameRateLimit::Automatic, false),
    ] {
        let mut pacer = FramePacer::default();
        pacer.configure(limit, software);
        let start = Instant::now();
        pacer.frame_finished(start, start + Duration::from_millis(50));
        assert_eq!(
            pacer.request(start + Duration::from_millis(50)),
            FrameRequest::Draw
        );
    }
}

#[test]
fn changing_a_limit_keeps_the_pending_final_redraw() {
    let mut pacer = FramePacer::default();
    pacer.configure(FrameRateLimit::Fps15, true);
    let start = Instant::now();
    pacer.frame_finished(start, start);
    assert!(matches!(pacer.request(start), FrameRequest::Schedule(_)));
    pacer.configure(FrameRateLimit::Fps60, true);
    assert_eq!(pacer.request(start), FrameRequest::Pending);
    pacer.timer_elapsed();
    assert_eq!(pacer.request(start), FrameRequest::Draw);
    pacer.frame_finished(start, start);
    assert!(matches!(pacer.request(start), FrameRequest::Schedule(_)));
    pacer.configure(FrameRateLimit::Unlimited, true);
    assert_eq!(pacer.request(start), FrameRequest::Draw);
}
