use criterion::{criterion_group, criterion_main, Criterion};
use std::time::Duration;

trait DoSomething {
    fn do_it(&self);
}

struct ActionOne;
struct ActionTwo;

impl DoSomething for ActionOne {
    #[inline]
    fn do_it(&self) {
        // Simulate some work
        let _ = (1..10).fold(0, |acc, x| acc + x);
    }
}

impl DoSomething for ActionTwo {
    #[inline]
    fn do_it(&self) {
        // Simulate some different work
        let _ = (1..10).map(|x| x * x).sum::<i32>();
    }
}

enum Action {
    One(ActionOne),
    Two(ActionTwo),
}

fn dynamic_dispatch() {
    let actions: Vec<Box<dyn DoSomething>> = vec![Box::new(ActionOne), Box::new(ActionTwo)];
    for action in actions {
        action.do_it();
    }
}

fn pattern_matching() {
    let actions = vec![Action::One(ActionOne), Action::Two(ActionTwo)];
    for action in actions {
        match action {
            Action::One(a) => a.do_it(),
            Action::Two(a) => a.do_it(),
        }
    }
}

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Dispatch vs Matching");
    group.measurement_time(Duration::new(10, 0)); // Adjust the measurement time as needed

    group.bench_function("Dynamic Dispatch", |b| b.iter(|| dynamic_dispatch()));
    group.bench_function("Pattern Matching", |b| b.iter(|| pattern_matching()));

    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
