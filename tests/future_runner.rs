use future_runner::run_future;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[test]
fn test_basic_future() {
    let completed = Arc::new(AtomicBool::new(false));
    let completed_clone = completed.clone();

    run_future(async move {
        completed_clone.store(true, Ordering::SeqCst);
    });

    assert!(completed.load(Ordering::SeqCst));
}

#[test]
#[cfg(feature = "threaded")]
fn test_threaded_future() {
    use tokio::time::Duration;

    let completed = Arc::new(AtomicBool::new(false));
    let completed_clone = completed.clone();

    run_future(async move {
        tokio::time::sleep(Duration::from_millis(100)).await;
        completed_clone.store(true, Ordering::SeqCst);
    });

    assert!(completed.load(Ordering::SeqCst));
}

#[test]
#[cfg(feature = "block")]
fn test_blocking_future() {
    use std::sync::atomic::AtomicI32;
    let value = Arc::new(AtomicI32::new(0));
    let value_clone = value.clone();

    run_future(async move {
        value_clone.store(42, Ordering::SeqCst);
    });

    assert_eq!(value.load(Ordering::SeqCst), 42);
}

#[test]
fn test_multiple_futures() {
    let counter = Arc::new(AtomicBool::new(false));
    let c1 = counter.clone();
    let c2 = counter.clone();

    run_future(async move {
        c1.store(true, Ordering::SeqCst);
    });

    run_future(async move {
        assert!(c2.load(Ordering::SeqCst));
    });
}

// For threaded runtime
#[test]
#[cfg(feature = "threaded")]
#[should_panic(expected = "Thread panicked")]
fn test_future_panic() {
    run_future(async {
        panic!("This async task will panic");
    });
}

// For blocking runtime
#[test]
#[cfg(all(feature = "block", not(feature = "threaded")))]
#[should_panic(expected = "This async task will panic")]
fn test_future_panic() {
    run_future(async {
        panic!("This async task will panic");
    });
}

// For non-blocking default runtime (no features)
#[test]
#[cfg(all(not(feature = "block"), not(feature = "threaded")))]
#[should_panic(expected = "This async task will panic")]
fn test_future_panic() {
    run_future(async {
        panic!("This async task will panic");
    });
}
