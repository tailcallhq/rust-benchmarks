use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::Duration;

trait DoSomething {
    fn do_it(&self) -> i32;
}

struct ActionOne;
struct ActionTwo;

impl DoSomething for ActionOne {
    #[inline]
    fn do_it(&self) -> i32 {
        (1..10).fold(0, |acc, x| acc + x)
    }
}

impl DoSomething for ActionTwo {
    #[inline]
    fn do_it(&self) -> i32 {
        (1..10).map(|x| x * x).sum::<i32>()
    }
}

enum Action {
    One(ActionOne),
    Two(ActionTwo),
}

fn dynamic_dispatch() -> i32 {
    let actions: Vec<Box<dyn DoSomething>> = vec![Box::new(ActionOne), Box::new(ActionTwo)];
    let mut output = 0;
    for action in actions {
        output = output + action.do_it();
    }
    output
}

fn pattern_matching() -> i32 {
    let actions = vec![Action::One(ActionOne), Action::Two(ActionTwo)];
    let mut output = 0;
    for action in actions {
        match action {
            Action::One(a) => output = output + a.do_it(),
            Action::Two(a) => output = output + a.do_it(),
        }
    }
    output
}

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Dispatch vs Matching");
    group.measurement_time(Duration::new(10, 0)); // Adjust the measurement time as needed

    group.bench_function("Dynamic Dispatch", |b| {
        b.iter(|| {
            let output = dynamic_dispatch();
            black_box(output)
        })
    });
    
    group.bench_function("Pattern Matching", |b| {
        b.iter(|| {
            let output = pattern_matching();
            black_box(output)
        })
    });

    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
