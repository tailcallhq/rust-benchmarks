use criterion::{black_box, criterion_group, criterion_main, Criterion};

trait DoSomething {
    fn do_it(&self, i: i32) -> i32;
}

struct ActionOne;
struct ActionTwo;

impl DoSomething for ActionOne {
    #[inline]
    fn do_it(&self, i: i32) -> i32 {
        i + i
    }
}

impl DoSomething for ActionTwo {
    #[inline]
    fn do_it(&self, i: i32) -> i32 {
        i * i
    }
}

enum Action {
    One(ActionOne),
    Two(ActionTwo),
}

fn dynamic_dispatch(actions: &Vec<Box<dyn DoSomething>>, mut output: i32) -> i32 {
    for action in actions {
        output = output + action.do_it(output);
    }
    output
}

fn pattern_matching(actions: &Vec<Action>, mut output: i32) -> i32 {
    for action in actions {
        match action {
            Action::One(a) => output = output + a.do_it(output),
            Action::Two(a) => output = output + a.do_it(output),
        }
    }
    output
}

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Dispatch vs Matching");

    let dynamic_actions: Vec<Box<dyn DoSomething>> = vec![Box::new(ActionOne), Box::new(ActionTwo)];
    let static_actions: Vec<Action> = vec![Action::One(ActionOne), Action::Two(ActionTwo)];

    group.bench_function("Dynamic Dispatch", |b| {
        b.iter(|| {
            let output = dynamic_dispatch(&dynamic_actions, 1);
            black_box(output)
        })
    });

    group.bench_function("Pattern Matching", |b| {
        b.iter(|| {
            let output = pattern_matching(&static_actions, 1);
            black_box(output)
        })
    });

    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
