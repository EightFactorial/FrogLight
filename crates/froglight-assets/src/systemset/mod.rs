use std::sync::atomic::{AtomicBool, Ordering};

use bevy::{ecs::schedule::BoxedCondition, prelude::*};
use parking_lot::Mutex;

pub(crate) mod resourcepack;

static BUILD_ONCE: AtomicBool = AtomicBool::new(false);
static FINISH_ONCE: AtomicBool = AtomicBool::new(false);

#[doc(hidden)]
pub(crate) fn build(app: &mut App) {
    // Only run build once
    if BUILD_ONCE.load(Ordering::Relaxed) {
        return;
    }

    resourcepack::build(app);

    // Only run build once
    BUILD_ONCE.store(true, Ordering::Relaxed);
}

#[doc(hidden)]
pub(crate) fn finish(conditions: &Mutex<Vec<BoxedCondition>>, app: &mut App) {
    // Only run finish once
    if FINISH_ONCE.load(Ordering::Relaxed) {
        return;
    }

    resourcepack::finish(conditions, app);

    // Only run finish once
    FINISH_ONCE.store(true, Ordering::Relaxed);
}
