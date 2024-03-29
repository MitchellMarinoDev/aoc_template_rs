use crate::solutions::SOLUTIONS;
use crate::{days, CURRENT_DAY};
use std::path::Path;

/// Tests the current day. This will fail if you haven't entered the solution yet.
/// This test can be helpful if you want to refactor your solution after you solve the puzzle.
#[test]
fn test_current() {
    let result = days::DAYS[CURRENT_DAY - 1].solve(Path::new("./input/"));

    assert_eq!(
        result.p1,
        SOLUTIONS[result.day - 1].0,
        "solution to day {} part 1 was incorrect",
        result.day
    );
    assert_eq!(
        result.p2,
        SOLUTIONS[result.day - 1].1,
        "solution to day {} part 2 was incorrect",
        result.day
    );
}

/// Tests all implemented days other than the current day.
#[test]
#[ignore]
fn test_other() {
    let results: Vec<_> = days::DAYS
        .iter()
        .take(CURRENT_DAY)
        .map(|d| d.solve(Path::new("./input/")))
        .collect();

    for result in results {
        assert_eq!(
            result.p1,
            SOLUTIONS[result.day - 1].0,
            "solution to day {} part 1 was incorrect",
            result.day
        );
        assert_eq!(
            result.p2,
            SOLUTIONS[result.day - 1].1,
            "solution to day {} part 2 was incorrect",
            result.day
        );
    }
}
