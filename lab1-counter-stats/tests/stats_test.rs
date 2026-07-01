use lab1_counter_stats::{IdleLevel, Stats};

// ---------------------------------------------------------------------------
// Milestone 1 — IdleLevel enum, Stats struct, and empty constructor
// ---------------------------------------------------------------------------

#[test]
fn m1_new_is_empty() {
    let stats = Stats::new();

    assert_eq!(stats.count, 0);
    assert_eq!(stats.sum, 0);
    assert_eq!(stats.min, u32::MAX);
    assert_eq!(stats.max, 0);
    assert_eq!(stats.low_count, 0);
    assert_eq!(stats.medium_count, 0);
    assert_eq!(stats.high_count, 0);
}

// ---------------------------------------------------------------------------
// Milestone 2 — IdleLevel classifier (match on sample ranges)
// ---------------------------------------------------------------------------

#[test]
fn m2_classify_low_boundary() {
    assert_eq!(IdleLevel::from_sample(0), IdleLevel::Low);
    assert_eq!(IdleLevel::from_sample(33), IdleLevel::Low);
}

#[test]
fn m2_classify_medium_boundary() {
    assert_eq!(IdleLevel::from_sample(34), IdleLevel::Medium);
    assert_eq!(IdleLevel::from_sample(66), IdleLevel::Medium);
}

#[test]
fn m2_classify_high_boundary() {
    assert_eq!(IdleLevel::from_sample(67), IdleLevel::High);
    assert_eq!(IdleLevel::from_sample(100), IdleLevel::High);
}

// ---------------------------------------------------------------------------
// Milestone 3 — record updates aggregates and tier counters
// ---------------------------------------------------------------------------

#[test]
fn m3_record_single_sample() {
    let mut stats = Stats::new();
    stats.record(50);

    assert_eq!(stats.count, 1);
    assert_eq!(stats.sum, 50);
    assert_eq!(stats.min, 50);
    assert_eq!(stats.max, 50);
    assert_eq!(stats.low_count, 0);
    assert_eq!(stats.medium_count, 1);
    assert_eq!(stats.high_count, 0);
}

#[test]
fn m3_record_multiple_extrema_and_tiers() {
    let mut stats = Stats::new();
    stats.record(10);
    stats.record(80);
    stats.record(50);
    stats.record(5);

    assert_eq!(stats.count, 4);
    assert_eq!(stats.sum, 145);
    assert_eq!(stats.min, 5);
    assert_eq!(stats.max, 80);
    assert_eq!(stats.low_count, 2);
    assert_eq!(stats.medium_count, 1);
    assert_eq!(stats.high_count, 1);
}

// ---------------------------------------------------------------------------
// Milestone 4 — from_samples builds stats from a fixed-size array
// ---------------------------------------------------------------------------

#[test]
fn m4_from_samples_eight_elements() {
    let stats = Stats::from_samples([42, 15, 88, 15, 60, 42, 100, 7]);

    assert_eq!(stats.count, 8);
    assert_eq!(stats.sum, 369);
    assert_eq!(stats.min, 7);
    assert_eq!(stats.max, 100);
    assert_eq!(stats.low_count, 3);
    assert_eq!(stats.medium_count, 3);
    assert_eq!(stats.high_count, 2);
}
