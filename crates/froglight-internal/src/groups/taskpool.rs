use bevy::core::{TaskPoolOptions, TaskPoolThreadAssignmentPolicy};

/// The default [`TaskPoolOptions`] used by `FrogLight`.
///
/// Assigns CPU cores as follows:
/// - 20% for `IO`, at least 1, no more than 4
/// - 40% for `async compute`, at least 1, no limit
/// - 40% for `compute`, at least 1, no limit
///
/// At lower core counts, `async compute` and `compute` are balanced.
/// As core counts increase, `compute` is favored over `async compute`.
///
/// | CPU Cores/Threads | # IO | # Async Compute | # Compute |
/// |-------------------|------|-----------------|-----------|
/// | 1-3               | 1    | 1               | 1         |
/// | 4                 | 1    | 2               | 1         |
/// | 5                 | 1    | 2               | 2         |
/// | 6                 | 1    | 2               | 3         |
/// | 7                 | 1    | 3               | 3         |
/// | 8                 | 2    | 3               | 3         |
/// | 9                 | 2    | 4               | 3         |
/// | 10                | 2    | 4               | 4         |
/// | 11                | 2    | 4               | 5         |
/// | 12                | 2    | 5               | 5         |
/// | 13                | 3    | 5               | 5         |
/// | 14                | 3    | 6               | 5         |
/// | 15                | 3    | 6               | 6         |
/// | 16                | 3    | 6               | 7         |
/// | 24                | 4    | 10              | 10        |
/// | 32                | 4    | 13              | 15        |
/// | 48                | 4    | 19              | 25        |
/// | 64                | 4    | 26              | 34        |
/// | 128               | 4    | 51              | 73        |
pub const TASKPOOL_SETTINGS: TaskPoolOptions = TaskPoolOptions {
    // By default, use however many cores are available on the system
    min_total_threads: 1,
    max_total_threads: usize::MAX,

    // Use 20% of cores for IO, at least 1, no more than 4
    io: TaskPoolThreadAssignmentPolicy { min_threads: 1, max_threads: 4, percent: 0.2 },

    // Use 40% of cores for async compute, at least 1, no limit
    async_compute: TaskPoolThreadAssignmentPolicy {
        min_threads: 1,
        max_threads: usize::MAX,
        percent: 0.4,
    },

    // Use all (40%) remaining cores for compute, at least 1, no limit
    compute: TaskPoolThreadAssignmentPolicy {
        min_threads: 1,
        max_threads: usize::MAX,
        percent: 1.0,
    },
};

#[cfg(test)]
mod tests {
    use bevy::core::{TaskPoolOptions, TaskPoolThreadAssignmentPolicy};

    /// The expected distribution of threads based on the number of cores.
    const EXPECTED_DISTRIBUTION: [(usize, usize, usize); 16] = [
        (1, 1, 1),
        (1, 1, 1),
        (1, 1, 1),
        (1, 2, 1),
        (1, 2, 2),
        (1, 2, 3),
        (1, 3, 3),
        (2, 3, 3),
        (2, 4, 3),
        (2, 4, 4),
        (2, 4, 5),
        (2, 5, 5),
        (3, 5, 5),
        (3, 6, 5),
        (3, 6, 6),
        (3, 6, 7),
    ];

    /// Test the distribution of threads based on the number of cores.
    #[test]
    fn taskpool_threads() {
        // Test core counts 1 through 16
        for (index, distribution) in EXPECTED_DISTRIBUTION.iter().enumerate() {
            assert_eq!(*distribution, calculate_threads(index + 1, &super::TASKPOOL_SETTINGS));
        }

        // Test 24, 32, 48, 64, and 128 cores, just for fun
        assert_eq!((4, 10, 10), calculate_threads(24, &super::TASKPOOL_SETTINGS));
        assert_eq!((4, 13, 15), calculate_threads(32, &super::TASKPOOL_SETTINGS));
        assert_eq!((4, 19, 25), calculate_threads(48, &super::TASKPOOL_SETTINGS));
        assert_eq!((4, 26, 34), calculate_threads(64, &super::TASKPOOL_SETTINGS));
        assert_eq!((4, 51, 73), calculate_threads(128, &super::TASKPOOL_SETTINGS));
    }

    /// Calculate the number of threads to use based on the taskpool options and
    /// the number of cores.
    fn calculate_threads(cores: usize, options: &TaskPoolOptions) -> (usize, usize, usize) {
        let mut remaining = cores;

        // Calculate the number of threads for the IO pool
        let io = get_number_of_threads(&options.io, remaining, cores);
        remaining = remaining.saturating_sub(io);

        // Calculate the number of threads for the async compute pool
        let async_compute = get_number_of_threads(&options.async_compute, remaining, cores);
        remaining = remaining.saturating_sub(async_compute);

        // Calculate the number of threads for the compute pool
        let compute = get_number_of_threads(&options.compute, remaining, cores);

        (io, async_compute, compute)
    }

    /// Calculate the number of threads to use based on policy and remaining
    /// cores.
    #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss, clippy::cast_sign_loss)]
    fn get_number_of_threads(
        policy: &TaskPoolThreadAssignmentPolicy,
        remaining: usize,
        total: usize,
    ) -> usize {
        let mut desired = (total as f32 * policy.percent).round() as usize;
        // Limit ourselves to the number of cores available
        desired = desired.min(remaining);
        // Clamp by min_threads, max_threads.
        desired.clamp(policy.min_threads, policy.max_threads)
    }
}
