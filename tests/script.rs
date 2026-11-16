use clingo::ast::Location;
use clingo::*;
use std::sync::{Arc, Mutex};

struct TestScript {
    executed: Arc<Mutex<Vec<String>>>,
}
impl Script for TestScript {
    fn execute(&self, _location: &Location, code: &str) -> Result<(), ClingoError> {
        self.executed.lock().unwrap().push(code.to_string());
        Ok(())
    }
    fn call(
        &self,
        _location: &Location,
        name: &str,
        arguments: &[Symbol],
    ) -> Result<Vec<Symbol>, ClingoError> {
        assert_eq!(name, "f");
        let number = arguments[0].number()?;
        Ok(vec![
            Symbol::create_number(number * 42),
            Symbol::create_number(number),
        ])
    }
    fn callable(&self, name: &str) -> Result<bool, ClingoError> {
        Ok(name == "f")
    }
    fn main(&self, _ctl: Control) -> Result<(), ClingoError> {
        Ok(())
    }
}

#[test]
fn script() {
    let executed = Arc::new(Mutex::new(vec![]));
    let script = TestScript {
        executed: executed.clone(),
    };
    register_script("test", script, "1.2.3").unwrap();
    assert_eq!(script_version("test").unwrap(), Some("1.2.3"));
    assert_eq!(script_version("unknown").unwrap(), None);

    let mut ctl = control(vec![]).unwrap();
    ctl.add("base", &[], "#script (test) some code #end. a(@f(1)).")
        .unwrap();
    assert_eq!(executed.lock().unwrap().len(), 1);
    assert!(executed.lock().unwrap()[0].contains("some code"));

    ctl.ground(&[Part::new("base", vec![]).unwrap()]).unwrap();
    let mut handle = ctl.solve(SolveMode::YIELD, &[]).unwrap();
    handle.resume().unwrap();
    let model = handle.model().unwrap().expect("Expected a model.");
    let mut symbols: Vec<String> = model
        .symbols(ShowType::SHOWN)
        .unwrap()
        .iter()
        .map(|symbol| symbol.to_string())
        .collect();
    symbols.sort();
    assert_eq!(symbols, ["a(1)", "a(42)"]);
    handle.close().unwrap();
}
