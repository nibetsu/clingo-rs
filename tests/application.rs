use clingo::*;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

struct Ctx {
    non: defaults::Non,
}
impl ControlCtx for Ctx {
    type L = defaults::Non;
    type P = defaults::Non;
    type O = defaults::Non;
    type F = defaults::Non;
    fn logger(&mut self) -> (&mut Self::L, u32) {
        (&mut self.non, 0)
    }
    fn propagator(&mut self) -> (&mut Self::P, bool) {
        (&mut self.non, false)
    }
    fn observer(&mut self) -> (&mut Self::O, bool) {
        (&mut self.non, false)
    }
    fn function_handler(&mut self) -> &mut Self::F {
        &mut self.non
    }
}

#[derive(Default)]
struct TestApp {
    flag: Flag,
    number: Rc<Cell<i32>>,
    validated: bool,
    fail: bool,
    files: RefCell<Vec<String>>,
    models: RefCell<Vec<Vec<String>>>,
    messages: RefCell<Vec<(Warning, String)>>,
}
impl Application for TestApp {
    fn program_name(&self) -> &str {
        "test-app"
    }
    fn version(&self) -> &str {
        "1.2.3"
    }
    fn main(&self, ctl: Control, files: &[&str]) -> Result<(), ClingoError> {
        // the control object still belongs to clingo after registering a context
        let mut ctl = ctl.register_control_context(Ctx { non: defaults::Non });
        self.files
            .borrow_mut()
            .extend(files.iter().map(|file| file.to_string()));
        for file in files {
            ctl.load(file)?;
        }
        ctl.add("base", &[], &format!("n({}).", self.number.get()))?;
        if self.fail {
            ctl.add("base", &[], "syntax error")?;
        }
        ctl.ground(&[Part::new("base", vec![])?])?;
        let mut handle = ctl.solve(SolveMode::empty(), &[])?;
        handle.get()?;
        handle.close()?;
        Ok(())
    }
    fn log(&self, code: Warning, message: &str) {
        self.messages.borrow_mut().push((code, message.to_string()));
    }
    fn print_model(
        &self,
        model: &Model,
        printer: &mut DefaultModelPrinter,
    ) -> Result<(), ClingoError> {
        let mut symbols: Vec<String> = model
            .symbols(ShowType::SHOWN)?
            .iter()
            .map(|symbol| symbol.to_string())
            .collect();
        symbols.sort();
        self.models.borrow_mut().push(symbols);
        printer.print()
    }
    fn register_options(&mut self, options: &mut ClingoOptions) -> Result<(), ClingoError> {
        options.add_flag("Test", "test-flag", "A flag", &self.flag)?;
        let number = self.number.clone();
        options.add(
            "Test",
            "test-number",
            "A number",
            move |value| match value.parse() {
                Ok(value) => {
                    number.set(value);
                    true
                }
                Err(_) => false,
            },
            false,
            Some("<n>"),
        )?;
        Ok(())
    }
    fn validate_options(&mut self) -> Result<(), ClingoError> {
        self.validated = true;
        Ok(())
    }
}

#[test]
fn application() {
    let path = std::env::temp_dir().join(format!("clingo-rs-{}-app.lp", std::process::id()));
    std::fs::write(&path, "a. {b}. c(1/0).").unwrap();
    let file = path.to_str().unwrap();

    let mut app = TestApp::default();
    let code = clingo_main(&mut app, &["0", "--test-flag", "--test-number=42", file]).unwrap();
    // satisfiable and the search space was exhausted
    assert_eq!(code, 30);
    assert!(app.flag.get());
    assert_eq!(app.number.get(), 42);
    assert!(app.validated);
    assert_eq!(*app.files.borrow(), [file]);
    let mut models = app.models.borrow().clone();
    models.sort();
    assert_eq!(models, [vec!["a", "b", "n(42)"], vec!["a", "n(42)"]]);
    let messages = app.messages.borrow();
    assert_eq!(messages.len(), 1);
    assert!(matches!(messages[0].0, Warning::OperationUndefined));

    // invalid option values are rejected before main is called
    let mut app = TestApp::default();
    let code = clingo_main(&mut app, &["--test-number=x", file]).unwrap();
    assert_ne!(code, 0);
    assert!(app.files.borrow().is_empty());

    // errors in main are reported by clingo
    let mut app = TestApp {
        fail: true,
        ..TestApp::default()
    };
    let code = clingo_main(&mut app, &[file]).unwrap();
    assert_ne!(code, 0);
    assert_eq!(*app.files.borrow(), [file]);
    assert!(app.models.borrow().is_empty());

    std::fs::remove_file(&path).unwrap();
}
