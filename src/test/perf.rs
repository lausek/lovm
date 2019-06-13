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
    static mut TRACK: Option<Instant> = None;
    static mut TRACKS: Vec<Duration> = Vec::new();
    const ITERATIONS: usize = 1000;

    vm.interrupts_mut()
        .set(vm::Interrupt::Debug as usize, &|_| {
            unsafe {
                TRACK = match TRACK.take() {
                    Some(time) => {
                        let delta = Instant::now() - time;
                        println!("runtime: {}", delta.as_nanos());
                        TRACKS.push(delta);
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
        let sigma = TRACKS.iter().sum::<Duration>().as_nanos();
        let avg = sigma as f64 / TRACKS.len() as f64;
        println!("average ({} runs): {}", ITERATIONS, avg);

        if !TRACKS.is_empty() {
            // we want to be 10% faster
            assert!(avg < 630_000f64 * 0.9);
            // if we have results, show them
            assert!(false, "runtime was faster now");
        }
    }
}
