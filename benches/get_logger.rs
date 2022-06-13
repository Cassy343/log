#![feature(test, bench_black_box)]

extern crate criterion;
extern crate log;

use criterion::{criterion_group, criterion_main, Criterion, Bencher};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::hint::black_box;
use log::{Level, Log, Metadata, Record};

// const UNINITIALIZED: usize = 0;
const INITIALIZED: usize = 2;
static mut LOGGER: &dyn Log = &NopLogger;

#[inline]
fn logger(state: &AtomicUsize, ordering: Ordering) -> &'static dyn Log {
    if state.load(ordering) == INITIALIZED {
        unsafe { LOGGER }
    } else {
        &NopLogger
    }
}

struct NopLogger;

impl Log for NopLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, _record: &Record) {}

    fn flush(&self) {}
}

fn make_record() -> Record<'static> {
    Record::builder()
        .args(format_args!("Hello world"))
        .level(Level::Info)
        .target(module_path!())
        .module_path_static(Some(module_path!()))
        .file_static(Some(file!()))
        .line(Some(line!()))
        .build()
}

fn get_logger(c: &mut Criterion) {
    let mut group = c.benchmark_group("get_logger");
    group.bench_function("Present, SeqCst", get_logger_present_seq_cst);
    group.bench_function("Present, Acquire", get_logger_present_acquire);
}

fn log(c: &mut Criterion) {
    let mut group = c.benchmark_group("log");
    group.bench_function("Present, SeqCst", log_present_seq_cst);
    group.bench_function("Present, Acquire", log_present_acquire);
}

fn get_logger_present_seq_cst(b: &mut Bencher) {
    let mut state = AtomicUsize::new(INITIALIZED);

    b.iter(|| {
        for _ in 0..100 {
            unsafe {
                LOGGER = black_box(LOGGER);
            }
            black_box(logger(black_box(&mut state), Ordering::SeqCst));
        }
    });
}

fn get_logger_present_acquire(b: &mut Bencher) {
    let mut state = AtomicUsize::new(INITIALIZED);

    b.iter(|| {
        for _ in 0..100 {
            unsafe {
                LOGGER = black_box(LOGGER);
            }
            black_box(logger(black_box(&mut state), Ordering::SeqCst));
        }
    });
}

fn log_present_seq_cst(b: &mut Bencher) {
    let mut state = AtomicUsize::new(INITIALIZED);
    let record = make_record();

    b.iter(|| {
        for _ in 0..100 {
            unsafe {
                LOGGER = black_box(LOGGER);
            }
            logger(black_box(&mut state), Ordering::SeqCst).log(&record);
        }
    });
}

fn log_present_acquire(b: &mut Bencher) {
    let mut state = AtomicUsize::new(INITIALIZED);
    let record = make_record();

    b.iter(|| {
        for _ in 0..100 {
            unsafe {
                LOGGER = black_box(LOGGER);
            }
            logger(black_box(&mut state), Ordering::Acquire).log(&record);
        }
    });
}

criterion_group!(
    benches,
    get_logger,
    log
);
criterion_main!(benches);