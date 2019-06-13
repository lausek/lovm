#![cfg(test)]
use super::*;

#[test]
fn fib_recursion() {
    use std::time::*;

    let unit = unit! {
        fib => func!([n] => {
            cmp_eq().var("n").op(0) => {
                ret().op(0
            },
            cmp_eq().var("n").op(1) => {
                ret().op(1)
            },
            add()
                .op(call("fib").op(sub().var("n").op(1).end()).end())
                .op(call("fib").op(sub().var("n").op(2).end()).end())
        }),
        main => func!({
            debug(),
            call("fib").op(13),
            debug(),
        }),
    };

    let mut vm = vm::Vm::new();
    const ITERATIONS: usize = 1000;
    static mut TRACK: Option<Instant> = None;
    static mut AVG: f64 = 0.;

    vm.interrupts_mut()
        .set(vm::Interrupt::Debug as usize, &|_| {
            unsafe {
                TRACK = match TRACK.take() {
                    Some(time) => {
                        let delta = Instant::now() - time;
                        AVG += delta.as_nanos() as f64 / ITERATIONS as f64;
                        None
                    }
                    _ => Some(Instant::now()),
                };
            }
            Ok(())
        });

    for _ in 1..=ITERATIONS {
        vm.run(&unit).expect("error in code");
    }

    unsafe {
        println!("average ({} runs): {}", ITERATIONS, AVG);

        if 0. < AVG {
            // we want to be 10% faster
            assert!(AVG < 630_000f64 * 0.9);
            // if we have results, show them
            assert!(false, "runtime was faster now");
        }
    }
}
