#![cfg(feature = "static-linking")]
use clingo::*;

fn solve(ctl: Control) -> Control {
    let mut handle = ctl
        .solve(SolveMode::empty(), &[])
        .expect("Failed to retrieve solve handle.");
    handle.get().expect("Failed to get the solve result.");
    handle.close().expect("Failed to close solve handle.")
}

#[test]
fn clasp_facade() {
    let mut ctl = control(vec![]).unwrap();
    let conf = ctl.configuration_mut().unwrap();
    let root_key = conf.root().unwrap();
    let sub_key = conf.map_at(root_key, "solve.parallel_mode").unwrap();
    conf.value_set(sub_key, "2")
        .expect("Failed to set solve.parallel_mode to 2.");
    ctl.add("base", &[], "{a;b;c}. :- a, b.")
        .expect("Failed to add a logic program.");
    let parts = vec![Part::new("base", vec![]).unwrap()];
    ctl.ground(&parts)
        .expect("Failed to ground a logic program.");

    let ctl = solve(ctl);
    let clasp = ctl.clasp_facade().unwrap().expect("clingo uses clasp");
    assert!(!clasp.as_ptr().is_null());
    assert_eq!(clasp.concurrency().unwrap(), 2);
    assert!(clasp.num_vars().unwrap() > 0);
    assert!(clasp.num_constraints().unwrap() > 0);
    assert!(clasp.num_eliminated_vars().unwrap() <= clasp.num_vars().unwrap());
    assert!(!clasp.solving().unwrap());
    assert!(!clasp.interrupted().unwrap());
    let step = clasp.step().unwrap();

    let ctl = solve(ctl);
    let clasp = ctl.clasp_facade().unwrap().expect("clingo uses clasp");
    assert_eq!(clasp.step().unwrap(), step + 1);
}
