use clingo::*;
use std::cell::RefCell;
use std::env;
use std::rc::Rc;

struct App {
    // the program part to ground, set by a command line option
    program: Rc<RefCell<String>>,
}

impl Application for App {
    fn program_name(&self) -> &str {
        // the name of the program printed in its help output
        "example"
    }

    fn version(&self) -> &str {
        // the version of the program printed in its help output
        "1.0.0"
    }

    fn main(&self, mut ctl: Control, files: &[&str]) -> Result<(), ClingoError> {
        // load files into the control object
        for file in files {
            ctl.load(file)?;
        }
        // if no files are given read from stdin
        if files.is_empty() {
            ctl.load("-")?;
        }

        // ground
        let part = Part::new(&self.program.borrow(), vec![])?;
        ctl.ground(&[part])?;

        // solve, clingo prints the models
        let mut handle = ctl.solve(SolveMode::YIELD, &[])?;
        loop {
            handle.resume()?;
            if handle.model()?.is_none() {
                break;
            }
        }
        handle.get()?;
        handle.close()?;
        Ok(())
    }

    fn register_options(&mut self, options: &mut ClingoOptions) -> Result<(), ClingoError> {
        // register an option to overwrite which program part to ground
        let program = self.program.clone();
        options.add(
            "Example",
            "program",
            "Override the default program part to ground",
            move |value| {
                *program.borrow_mut() = value.to_string();
                true
            },
            false,
            Some("<prog>"),
        )
    }
}

fn main() {
    let arguments: Vec<String> = env::args().skip(1).collect();
    let arguments: Vec<&str> = arguments.iter().map(String::as_str).collect();
    let mut app = App {
        program: Rc::new(RefCell::new("base".to_string())),
    };
    let code = clingo_main(&mut app, &arguments).expect("Failed to run clingo_main.");
    std::process::exit(code);
}
