# Changelog

All notable changes to this project will be documented in this file.

## v0.9.0

- Update to clingo-sys-0.8.0 (clingo 5.8.3)
- Fix abort when parsing programs with comments or theory definitions
  - Since clingo 5.7 comments are reported as statements
  - Unexpected statement types are reported as errors instead of panicking
- Fix undefined behavior for empty arrays passed by clingo
- Fix PropagateInit::get_check_mode() for PropagatorCheckMode::Both
- Add AST Comment node
  - Add StatementIsA::Comment (breaking change for exhaustive matches)
  - Add CommentType
  - Add ast::comment()
- Add Model::is_consequence() and Consequence
- Add Model::priority()
- Add SolveHandle::last()
- Add PropagateInit::set_undo_mode(), PropagateInit::get_undo_mode() and PropagatorUndoMode
- Add functions to Control
  - Control::load_aspif()
  - Control::register_backend() and BackendType
  - Control::remove_minimize()
  - Control::update_project()
- Add SymbolicAtoms::find()
- Add functions to Backend
  - Backend::theory_term_number()
  - Backend::theory_term_string()
  - Backend::theory_term_sequence() and TheorySequenceType
  - Backend::theory_term_function()
  - Backend::theory_term_symbol()
  - Backend::theory_element()
  - Backend::theory_atom() and TheoryAtomTarget
  - Backend::theory_atom_with_guard()
- Implement Display for ErrorType and Warning

## v0.8.0

- Update to clingo-sys-0.7.2 (clingo 5.6.2)
- Remove functions to create csp terms from the ast module
- Introduce GenericControl with parameter ControlCtx

## v0.7.3

- Fix issue with too lax dedendencies

## v0.7.1 and v0.7.2

- Fix issues with documentaion on docs.rs

## v0.7.0

- Add Theory trait
- Control::solve() now consumes the control object and returns a SolveHandle
- SolveHandle::close() consumes the solve handle and returns the corresponding Control object
- Properly use solve event data
  - Remove SolveEventType
  - Add SolveEvent
  - Change SolveEventHandler::on_solve_event(...)
- Return &'static str for internalized strings in
  - Location::begin_file()
  - Location::end_file()
  - Signature::name()
  - Symbol::name()
  - Symbol::string()
- Add FromSymbol trait
- Update clingo-sys to 0.5.2
- Add functions to PropagateInit
  - PropagateInit::add_literal()
  - PropagateInit::add_weight_constraint()
  - PropagateInit::add_minimize()
  - PropagateInit::propagate()
- Remove Control::use_enumeration_assumption() from Control
- Add Control::get_enable_enumeration_assumption()
- Add Control::set_enable_enumeration_assumption()
- Add Control::get_enable_cleanup()
- Add Control::set_enable_cleanup()

## v0.6.0 - Jan 3, 2020

**Changed:**

- Refactor ClingoError
- Update to clingo-sys 0.5.1 which includes clingo via git submodule

## v0.5.0 - Nov 4, 2019

**Changed:**

- Use clingo-sys 0.5.0
- Add functions for the AST
- Fix lifetime of objects

## v0.4.3 - Jul 10, 2019

**Fixed:**

- Remove optimality check from AllModels Iterator

## v0.4.2 - Jun 28, 2019

**Added:**

- Add the possiblibity to create HeadLiterals ast::HeadLiteral::new()
- Add clingo::ToSymbol trait
- Add clingo::FactBase
- Add method clingo::Control.add_facts(fb:FactBase)
- Add Iterators for AllModels and AllOptimalModels
- Add dynamic linking of a shared clingo library

## v0.4.0 - Feb 2, 2019

**Fixed:**

- Removed copy derive for opaque types

**Changed:**

- Use libclingo 5.3.0
- Use rust stable

**Added:**

- Add tests
- Add this CHANGELOG file

## Undocumented versions

- 0.3.1 Apr 11, 2018
- 0.3.0 Apr 11, 2018
- 0.1.0 May 4, 2017
