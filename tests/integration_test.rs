use clingo::ast::*;
use clingo::*;

#[test]
fn version() {
    let (ma, mi, re) = clingo::version();
    assert!(ma == 5);
    assert!(mi == 8);
    assert!(re == 3);
}
#[test]
fn error_and_warning_strings() {
    assert_eq!(ErrorType::Runtime.to_string(), "runtime error");
    assert_eq!(ErrorType::BadAlloc.to_string(), "bad allocation");
    assert_eq!(Warning::AtomUndefined.to_string(), "atom undefined");
    assert_eq!(Warning::FileIncluded.to_string(), "file included");
}
#[test]
fn signature() {
    let a = Signature::new("a", 2, false).unwrap();
    let b = Signature::new("a", 2, false).unwrap();
    let c = Signature::new("a", 2, true).unwrap();
    assert_eq!(a.name(), Ok("a"));
    assert_eq!(a.arity(), 2);
    assert!(a.is_negative());
    assert!(!a.is_positive());
    assert_eq!(b, a);
    assert_ne!(c, a);
    assert!(c < a);
    assert!(c <= a);
    assert!(a <= b);
    assert!(!(a <= c));
    assert!(a > c);
    assert!(a >= c);
    assert!(a >= b);
    assert!(!(c >= a));
    // assert!(c.hash() != a.hash());
    // assert!(b.hash() == a.hash());
}
#[test]
fn symbol() {
    // numbers
    let sym = Symbol::create_number(42);
    assert!(42 == sym.number().unwrap());
    // inf
    let sym2 = Symbol::create_infimum();
    assert!(SymbolType::Infimum == sym2.symbol_type().unwrap());
    // sup
    let sym3 = Symbol::create_supremum();
    assert!(SymbolType::Supremum == sym3.symbol_type().unwrap());
    // str
    let sym4 = Symbol::create_string("x").unwrap();
    assert!("x" == sym4.string().unwrap());
    // id
    let sym5 = Symbol::create_id("x", false).unwrap();
    assert!(SymbolType::Function == sym5.symbol_type().unwrap());
    assert!(sym5.is_negative().unwrap());
    assert!(!sym5.is_positive().unwrap());
    assert!("x" == sym5.name().unwrap());
    let args = vec![sym, sym2, sym3, sym4, sym5];
    // fun
    let sym6 = Symbol::create_function("f", &args, true).unwrap();
    assert!(SymbolType::Function == sym6.symbol_type().unwrap());
    assert!(!sym6.is_negative().unwrap());
    assert!("f" == sym6.name().unwrap());
    assert!("f(42,#inf,#sup,\"x\",-x)" == sym6.to_string());
    assert!(args.len() == sym6.arguments().unwrap().len());
    assert_eq!(args, sym6.arguments().unwrap());
    if let Err(e) = sym6.number() {
        assert!(e.to_string() == "InternalError: Call to clingo_symbol_number() failed, code: Runtime, last: unexpected");
    }
    // comparison
    let a = Symbol::create_number(1);
    let b = Symbol::create_number(2);
    assert!(a < b);
    assert!(!(a < a));
    assert!(!(b < a));
    assert!(b > a);
    assert!(!(a > a));
    assert!(!(a > b));
    assert!(a <= a);
    assert!(a <= b);
    assert!(!(b <= a));
    assert!(a >= a);
    assert!(b >= a);
    assert!(!(a >= b));
    assert!(a == a);
    assert!(!(a == b));
    assert!(a != b);
    assert!(!(a != a));
    // assert!(a.hash() == a.hash());
    // assert!(a.hash() != b.hash());
}
#[test]
fn configuration() {
    let mut ctl = control(vec![]).unwrap();
    // get the configuration object and its root key
    let conf = ctl.configuration_mut().unwrap();
    let root_key = conf.root().unwrap();
    let sub_key = conf.map_at(root_key, "solve.models").unwrap();
    conf.value_set(sub_key, "0").unwrap();
    let res = conf.value_get(sub_key).unwrap();
    assert_eq!(res, "0");
    let desc = conf.description(sub_key).unwrap();
    assert_eq!(desc, "Compute at most %A models (0 for all)\n");
}
#[test]
fn backend() {
    let mut ctl = control(vec![]).unwrap();
    ctl.add("base", &[], "{a; b; c}.").unwrap();

    let part = Part::new("base", vec![]).unwrap();
    let parts = [part];
    ctl.ground(&parts).unwrap();

    let sa = ctl.symbolic_atoms().unwrap();
    let bla = sa.signatures().unwrap();
    for b in bla {
        println!("bbla: {:?}", b);
    }
}
#[test]
fn symbols() {
    let number_symbol = Symbol::create_number(42);
    let identifier_symbol = Symbol::create_id("x", true).unwrap();

    let symbols = [number_symbol, identifier_symbol];
    let function_symbol = Symbol::create_function("x", &symbols, true).unwrap();

    // retrieve argument symbols of a symbol
    let symbols2 = function_symbol.arguments().unwrap();
    assert_eq!(symbols.to_vec(), symbols2);
}
#[test]
fn theory_atoms() {
    let mut ctl = control(vec![]).unwrap();
    ctl.add(
        "base",
        &[],
        "#theory t {\
         term   { + : 1, binary, left };\
         &a/0 : term, any;\
         &b/1 : term, {=}, term, any\
         }.\
         y :- &b(a) { } = 17.",
    )
    .unwrap();

    let part = Part::new("base", vec![]).unwrap();
    let parts = vec![part];
    ctl.ground(&parts).unwrap();

    let atoms = ctl.theory_atoms().unwrap();
    for atom in atoms.iter() {
        let term = atoms.atom_term(atom).unwrap();
        let string = atoms.atom_to_string(atom).unwrap();
        assert_eq!("&b(a){}=17", string);
        let string = atoms.term_to_string(term).unwrap();
        assert_eq!("b(a)", string);
    }
}

fn test_statement(stmt: &Statement, string: &str) {
    let string2 = format!("{}", stmt);
    assert_eq!(string2, string);

    let mut ctl = control(vec![]).unwrap();

    // get the program builder
    let mut builder = ast::ProgramBuilder::from(&mut ctl).unwrap();

    builder
        .add(stmt)
        .expect("Failed to add statement to ProgramBuilder.");

    // finish building a program
    builder.end().expect("Failed to finish building a program.");
    let string2 = format!("{}", stmt);
    assert_eq!(string2, string);

    // ground the base part
    let part = Part::new("base", vec![]).unwrap();
    let parts = [part];
    ctl.ground(&parts)
        .expect("Failed to ground a logic program.");

    let atoms = ctl.symbolic_atoms().unwrap();
    for item in atoms.iter().unwrap() {
        let symbol = item.symbol().unwrap();
        let string2 = symbol.to_string();
        println!("{}", string2);
    }
}

#[test]
fn ast_term() {
    let loc = Location::default();
    let sym1 = Symbol::create_number(42);
    let sym2 = Symbol::create_string("test").unwrap();

    let symbols = [sym1, sym2];
    let sym3 = Symbol::create_function("fun1", &symbols, true).unwrap();
    let sym4 = Symbol::create_supremum();
    let sym5 = Symbol::create_infimum();

    let term1 = symbolic_term(&loc, &sym1).unwrap();
    let term1 = Term::from(term1);
    assert_eq!(format!("{}", term1), "42");

    let term2 = symbolic_term(&loc, &sym2).unwrap();
    let term2 = Term::from(term2);
    assert_eq!(format!("{}", term2), "\"test\"");

    let term = symbolic_term(&loc, &sym3).unwrap();
    let term = Term::from(term);
    assert_eq!(format!("{}", term), "fun1(42,\"test\")");

    let term = symbolic_term(&loc, &sym4).unwrap();
    let term = Term::from(term);
    assert_eq!(format!("{}", term), "#sup");

    let term = symbolic_term(&loc, &sym5).unwrap();
    let term = Term::from(term);
    assert_eq!(format!("{}", term), "#inf");

    let term = variable(&loc, "Var").unwrap();
    assert_eq!(format!("{}", term), "Var");

    let negation = UnaryOperator::Negation;
    let uop = unary_operation(&loc, negation, term1.clone()).unwrap();
    let uop = Term::from(uop);
    assert_eq!(format!("{}", uop), "~42");

    let xor = BinaryOperator::Xor;
    let bop = binary_operation(&loc, xor, term1.clone(), term2.clone()).unwrap();
    let bop = Term::from(bop);
    assert_eq!(format!("{}", bop), "(42^\"test\")");

    let interval = interval(&loc, term1.clone(), term2.clone()).unwrap();
    let interval = Term::from(interval);
    assert_eq!(format!("{}", interval), "(42..\"test\")");

    let args = [term1, term2];
    let fun = function(&loc, "fun2", &args, false).unwrap();
    let fun = Term::from(fun);
    assert_eq!(format!("{}", fun), "fun2(42,\"test\")");
    let external_function = function(&loc, "fun2", &args, true).unwrap();
    let external_function = Term::from(external_function);
    assert_eq!(format!("{}", external_function), "@fun2(42,\"test\")");
    let pool = pool(&loc, &args).unwrap();
    let pool = Term::from(pool);
    assert_eq!(format!("{}", pool), "(42;\"test\")");
}

#[test]
fn ast_literal() {
    let loc = Location::default();
    let sym1 = Symbol::create_number(42);
    let sym2 = Symbol::create_string("test").unwrap();

    let symbols = [sym1, sym2];
    let sym3 = Symbol::create_function("fun1", &symbols, true).unwrap();

    let term1 = symbolic_term(&loc, &sym1).unwrap();
    let term2 = symbolic_term(&loc, &sym2).unwrap();
    let term3 = symbolic_term(&loc, &sym3).unwrap();

    let lit = ast::basic_literal_from_boolean_constant(&loc, Sign::NoSign, true).unwrap();
    assert_eq!(format!("{}", lit), "#true");
    let sterm1 = ast::symbolic_atom(term1.clone()).unwrap();
    let lit = ast::basic_literal_from_symbolic_atom(&loc, Sign::NoSign, sterm1).unwrap();
    assert_eq!(format!("{}", lit), "42");

    let lt = ComparisonOperator::LessThan;
    let guard = guard(lt, term2).unwrap();
    assert_eq!(guard.to_string().unwrap(), " < \"test\"");

    let guards = [guard];
    let comp = comparison(term3, &guards).unwrap();
    assert_eq!(comp.to_string().unwrap(), "fun1(42,\"test\") < \"test\"");
    let lit = ast::basic_literal_from_comparison(&loc, Sign::NoSign, comp).unwrap();
    assert_eq!(format!("{}", lit), "fun1(42,\"test\") < \"test\"");
}

#[test]
fn ast_head_literal() {
    let loc = Location::default();
    let sym = Symbol::create_id("test", true).unwrap();
    let sym2 = Symbol::create_id("toast", true).unwrap();
    let sym3 = Symbol::create_id("tset", true).unwrap();
    let term1 = symbolic_term(&loc, &sym).unwrap();
    let term2 = symbolic_term(&loc, &sym2).unwrap();
    let term3 = symbolic_term(&loc, &sym3).unwrap();
    let atom1 = symbolic_atom(term1.clone()).unwrap();
    let atom2 = symbolic_atom(term2).unwrap();
    let lit = basic_literal_from_symbolic_atom(&loc, Sign::NoSign, atom1).unwrap();
    let lit2 = basic_literal_from_symbolic_atom(&loc, Sign::NoSign, atom2).unwrap();
    let condition: Vec<ast::Literal> = vec![lit2.into()];
    let cond = conditional_literal(&loc, lit.clone(), &condition).unwrap();
    let elements = [cond.clone()];
    let dis = disjunction(&loc, &elements).unwrap();

    let tuple = [term1.clone().into()];
    let helem = head_aggregate_element(&tuple, cond).unwrap();
    let elements = [helem];
    let hagg = head_aggregate(&loc, None, ast::AggregateFunction::Count, &elements, None).unwrap();

    let tuple = [term1.clone().into()];
    let element = theory_atom_element(&tuple, &condition).unwrap();
    let elements = [element];
    let guard = theory_guard("theory_operator", term1.clone()).unwrap();
    let tatom = theory_atom(&loc, term3, &elements, Some(guard)).unwrap();

    let hlit: ast::Head = lit.into();
    assert_eq!(format!("{}", hlit), "test");

    let hlit: ast::Head = dis.into();
    assert_eq!(format!("{}", hlit), "test: toast");

    let hlit: Head = hagg.into();
    assert_eq!(format!("{}", hlit), "#count { test: test: toast }");

    let hlit: Head = tatom.clone().into();
    assert_eq!(
        format!("{}", hlit),
        "&tset { test: toast } theory_operator test"
    );

    let ta_term: Term = tatom.term();
    assert_eq!(format!("{}", ta_term), "tset");
}
#[test]
fn ast_body_literal() {
    let loc = Location::default();
    let sym = Symbol::create_id("test", true).unwrap();
    let term1 = symbolic_term(&loc, &sym).unwrap();
    let atom1 = symbolic_atom(term1.clone()).unwrap();
    let term2 = symbolic_term(&loc, &sym).unwrap();
    let atom2 = symbolic_atom(term2.clone()).unwrap();
    let lit = basic_literal_from_symbolic_atom(&loc, Sign::NoSign, atom1).unwrap();
    let lit2 = basic_literal_from_symbolic_atom(&loc, Sign::NoSign, atom2).unwrap();
    let condition = [lit2.into()];
    let cond = conditional_literal(&loc, lit.clone(), &condition).unwrap();
    let elements = [cond.clone()];

    let agg = aggregate(&loc, None, &elements, None).unwrap();

    let tuple = [term1.clone().into()];
    let element = body_aggregate_element(&tuple, &condition).unwrap();
    let elements = [element];
    let bagg = body_aggregate(&loc, None, AggregateFunction::Count, &elements, None).unwrap();

    let th_term = symbolic_term(&loc, &sym).unwrap();
    let tuple = [th_term.clone().into()];
    let element = theory_atom_element(&tuple, &condition).unwrap();
    let elements = [element];
    let guard = theory_guard("theory_operator", th_term).unwrap();
    let tatom = theory_atom(&loc, term1.clone(), &elements, Some(guard)).unwrap();
    let blit: BodyLiteral = lit.into();
    assert_eq!(format!("{}", blit), "test");

    let blit: BodyLiteral = cond.into();
    assert_eq!(format!("{}", blit), "test: test");

    let blit = atomic_literal_from_body_atom(&loc, Sign::NoSign, agg).unwrap();
    let blit: BodyLiteral = blit.into();
    assert_eq!(format!("{}", blit), "{ test: test }");

    let blit = atomic_literal_from_body_atom(&loc, Sign::NoSign, bagg).unwrap();
    let blit: BodyLiteral = blit.into();
    assert_eq!(format!("{}", blit), "#count { test: test }");

    let blit: BodyLiteral = tatom.into();
    assert_eq!(
        format!("{}", blit),
        "&test { test: test } theory_operator test"
    );
}
#[test]
fn ast_theory_term() {
    let loc = Location::default();
    let sym = Symbol::create_id("test", true).unwrap();

    let th_term1 = symbolic_term(&loc, &sym).unwrap();
    let th_term1: TheoryTerm = th_term1.into();
    assert_eq!(format!("{}", th_term1), "test");

    let arr = [th_term1];
    let th_fun = theory_function(&loc, "fun1", &arr).unwrap();

    let th_term = theory_sequence(&loc, TheoryTermSequenceType::Tuple, &arr).unwrap();
    assert_eq!(format!("{}", th_term), "(test,)");

    let th_term = theory_sequence(&loc, TheoryTermSequenceType::List, &arr).unwrap();
    assert_eq!(format!("{}", th_term), "[test]");

    let th_term = theory_sequence(&loc, TheoryTermSequenceType::Set, &arr).unwrap();
    assert_eq!(format!("{}", th_term), "{test}");

    let th_term: TheoryTerm = th_fun.into();
    assert_eq!(format!("{}", th_term), "fun1(test)");

    let th_term: TheoryTerm = variable(&loc, "Var").unwrap().into();
    assert_eq!(format!("{}", th_term), "Var");
}
#[test]
fn ast_edge() {
    let loc = Location::default();
    let sym1 = Symbol::create_id("test1", true).unwrap();
    let term1 = symbolic_term(&loc, &sym1).unwrap();
    let sym2 = Symbol::create_id("test2", true).unwrap();
    let term2 = symbolic_term(&loc, &sym2).unwrap();
    let atom2 = symbolic_atom(term2.clone()).unwrap();
    let lit = ast::basic_literal_from_symbolic_atom(&loc, Sign::NoSign, atom2).unwrap();
    let body = [lit.into()];
    let edge = edge(&loc, term1, term2, &body).unwrap();
    let stm: Statement = edge.into();
    test_statement(&stm, "#edge (test1,test2) : test2.");
}
#[test]
fn ast_minimize() {
    let loc = Location::default();
    let sym1 = Symbol::create_id("test1", true).unwrap();
    let weight = symbolic_term(&loc, &sym1).unwrap();
    let weight_atom = symbolic_atom(weight.clone()).unwrap();
    let sym2 = Symbol::create_id("test2", true).unwrap();
    let priority = symbolic_term(&loc, &sym2).unwrap();
    let tuple = [weight.clone().into(), priority.clone().into()];
    let lit = basic_literal_from_symbolic_atom(&loc, Sign::NoSign, weight_atom).unwrap();
    let body = [lit.into()];
    let mini = minimize(&loc, weight, priority, &tuple, &body).unwrap();
    let stm: Statement = mini.into();
    test_statement(&stm, ":~ test1. [test1@test2,test1,test2]");
}
#[test]
fn ast_show_term() {
    let loc = Location::default();
    let sym1 = Symbol::create_id("test1", true).unwrap();
    let term1 = symbolic_term(&loc, &sym1).unwrap();
    let sym2 = Symbol::create_id("test2", true).unwrap();
    let term2 = symbolic_term(&loc, &sym2).unwrap();
    let atom2 = symbolic_atom(term2).unwrap();
    let lit = basic_literal_from_symbolic_atom(&loc, Sign::NoSign, atom2).unwrap();
    let body = [lit.into()];
    let term = show_term(&loc, term1, &body, true).unwrap();
    let stm: Statement = term.into();
    test_statement(&stm, "#show test1 : test2.");
}
#[test]
fn ast_show_signature() {
    let loc = Location::default();
    let ssig = show_signature(&loc, "signame", 3, true, false).unwrap();
    let stm: Statement = ssig.into();
    test_statement(&stm, "#show signame/3.");
}
#[test]
fn ast_project_signature() {
    let loc = Location::default();
    let sig = project_signature(&loc, "signame", 3, false).unwrap();
    let stm: Statement = sig.into();
    test_statement(&stm, "#project -signame/3.");
}
#[test]
fn ast_defined() {
    let loc = Location::default();
    let def = defined(&loc, "signame", 3, false).unwrap();
    let stm: Statement = def.into();
    test_statement(&stm, "#defined -signame/3.");
}
#[test]
fn ast_const_definition() {
    let loc = Location::default();
    let sym1 = Symbol::create_id("test1", true).unwrap();
    let value = symbolic_term(&loc, &sym1).unwrap();
    let def = definition(&loc, "constname", value, true).unwrap();
    let stm: Statement = def.into();
    test_statement(&stm, "#const constname = test1.");
}
#[test]
fn ast_theory_definition() {
    let loc = Location::default();
    let op_def =
        theory_operator_definition(&loc, "operator_name", 2, TheoryOperatorType::Unary).unwrap();
    let operators = [op_def];
    let termdef = theory_term_definition(&loc, "def_name", &operators).unwrap();
    let terms = [termdef];
    let op1 = "operator1";
    let op2 = "operator2";
    let operators = [op1, op2];
    let guard = theory_guard_definition(&operators, "guard_term").unwrap();
    let atom_def = theory_atom_definition(
        &loc,
        TheoryAtomType::Head,
        "atom_def_name",
        2,
        "bla",
        Some(guard),
    )
    .unwrap();
    let atoms = [atom_def];
    let def = theory_definition(&loc, "theory_name", &terms, &atoms).unwrap();
    let stm: Statement = def.into();
    test_statement(
            &stm,
            "#theory theory_name {\n  def_name {\n    operator_name : 2, unary\n  };\n  &atom_def_name/2: bla, { operator1, operator2 }, guard_term, head\n}.",
        );
}
#[test]
fn ast_external() {
    let loc = Location::default();
    let sym = Symbol::create_id("test", true).unwrap();
    let term = symbolic_term(&loc, &sym).unwrap();
    let atom = symbolic_atom(term).unwrap();
    let ext = external(&loc, atom, &[], ExternalType::False).unwrap();
    let stm = ext.into();
    test_statement(&stm, "#external test. [false]");
}
#[test]
fn ast_rule_head_literal() {
    let loc = Location::default();
    let sym = Symbol::create_id("test", true).unwrap();
    let term = symbolic_term(&loc, &sym).unwrap();
    let atom = symbolic_atom(term).unwrap();
    let lit = basic_literal_from_symbolic_atom(&loc, Sign::NoSign, atom).unwrap();
    let rule = rule(&loc, lit, &[]).unwrap();
    let stm = rule.into();
    test_statement(&stm, "test.");
}
#[test]
fn ast_rule_head_aggregate() {
    let loc = Location::default();
    let sym = Symbol::create_id("test", true).unwrap();
    let term = symbolic_term(&loc, &sym).unwrap();
    let atom = symbolic_atom(term.clone()).unwrap();
    let lit = basic_literal_from_symbolic_atom(&loc, Sign::NoSign, atom).unwrap();
    let condition = [lit.clone().into()];
    let cond = conditional_literal(&loc, lit, &condition).unwrap();
    let elements = [cond];
    let agg = aggregate(&loc, None, &elements, None).unwrap();
    let rule = rule(&loc, agg, &[]).unwrap();
    let stm = rule.into();
    test_statement(&stm, "{ test: test }.");
}
#[test]
fn ast_rule_head_head_aggregate() {
    let loc = Location::default();
    let sym = Symbol::create_id("test", true).unwrap();
    let term = symbolic_term(&loc, &sym).unwrap();
    let atom = symbolic_atom(term.clone()).unwrap();
    let lit = basic_literal_from_symbolic_atom(&loc, Sign::NoSign, atom).unwrap();
    let condition = [lit.clone().into()];
    let cond = conditional_literal(&loc, lit, &condition).unwrap();
    let tuple = [term.clone().into()];
    let helem = head_aggregate_element(&tuple, cond).unwrap();
    let elements = [helem];
    let hagg = head_aggregate(&loc, None, AggregateFunction::Count, &elements, None).unwrap();
    let rule = rule(&loc, hagg, &[]).unwrap();
    let stm = rule.into();
    test_statement(&stm, "#count { test: test: test }.");
}
#[test]
fn ast_rule() {
    let loc = Location::default();
    let id1 = String::from("test1");
    let id2 = String::from("test2");
    let id3 = String::from("test3");
    let id8 = String::from("test8");
    let id9 = String::from("test9");
    let id10 = String::from("test10");

    let sym1 = Symbol::create_id(&id1, true).unwrap();
    let sym2 = Symbol::create_id(&id2, true).unwrap();
    let sym3 = Symbol::create_id(&id3, true).unwrap();
    let sym8 = Symbol::create_id(&id8, true).unwrap();
    let sym9 = Symbol::create_id(&id9, true).unwrap();
    let sym10 = Symbol::create_id(&id10, true).unwrap();

    let term1 = symbolic_term(&loc, &sym1).unwrap();
    let atom1 = symbolic_atom(term1).unwrap();
    let term2 = symbolic_term(&loc, &sym2).unwrap();
    let term3 = symbolic_term(&loc, &sym3).unwrap();
    let term8 = symbolic_term(&loc, &sym8).unwrap();
    let term9 = symbolic_term(&loc, &sym9).unwrap();
    let term10 = symbolic_term(&loc, &sym10).unwrap();

    let minus = UnaryOperator::Minus;
    let uop1 = unary_operation(&loc, minus, term8).unwrap();

    let xor = BinaryOperator::Xor;
    let bop1 = binary_operation(&loc, xor, term9, term10).unwrap();

    let args = [bop1.into()];
    let fun1 = function(&loc, "fun1", &args, false).unwrap();

    let gt = ComparisonOperator::GreaterThan;
    let guard = guard(gt, term2).unwrap();
    let guards = [guard];
    let comp = comparison(term3, &guards).unwrap();

    let lit1 = basic_literal_from_boolean_constant(&loc, Sign::NoSign, true).unwrap();
    let lit2 = basic_literal_from_symbolic_atom(&loc, Sign::NoSign, atom1).unwrap();
    let lit3 = basic_literal_from_comparison(&loc, Sign::NoSign, comp).unwrap();

    let lit5 = symbolic_atom(uop1).unwrap();
    let lit5: ast::Literal = basic_literal_from_symbolic_atom(&loc, Sign::NoSign, lit5)
        .unwrap()
        .into();

    let lit6 = symbolic_atom(fun1).unwrap();
    let lit6: ast::Literal = basic_literal_from_symbolic_atom(&loc, Sign::NoSign, lit6)
        .unwrap()
        .into();

    let rule1 = rule(&loc, lit1, &[]).unwrap();
    let rule2 = rule(&loc, lit2, &[]).unwrap();
    let rule3 = rule(&loc, lit3, &[]).unwrap();
    let rule5 = rule(&loc, lit5, &[]).unwrap();
    let rule6 = rule(&loc, lit6, &[]).unwrap();

    let stm = rule1.clone().into();
    test_statement(&stm, "#true.");
    let head = rule1.head();
    assert_eq!(format!("{}", head), "#true");

    let stm = rule2.into();
    test_statement(&stm, "test1.");
    let stm = rule3.into();
    test_statement(&stm, "test3 > test2.");
    let stm = rule5.into();
    test_statement(&stm, "-test8.");
    let stm = rule6.into();
    test_statement(&stm, "fun1((test9^test10)).");
}
#[test]
fn ast_rule_body() {
    let loc = Location::default();
    let sym = Symbol::create_id("test", true).unwrap();
    let term = symbolic_term(&loc, &sym).unwrap();
    let atom = symbolic_atom(term).unwrap();
    let lit = basic_literal_from_symbolic_atom(&loc, Sign::NoSign, atom).unwrap();
    let body = [lit.clone().into()];
    let rule = rule(&loc, lit, &body).unwrap();
    let h = rule.head();
    assert_eq!(format!("{}", h), "test");
    drop(h);
    let stm = rule.into();
    test_statement(&stm, "test :- test.");
}
#[test]
fn ast_program() {
    let loc = Location::default();
    let prg = program(&loc, "base", &[]).unwrap();
    let stm = prg.into();
    test_statement(&stm, "#program base.");
}
#[test]
fn ast_comment() {
    let loc = Location::default();
    let line = comment(&loc, "% line comment", CommentType::Line).unwrap();
    assert_eq!(line.value().unwrap(), "% line comment");
    assert_eq!(line.comment_type().unwrap(), CommentType::Line);
    let stm = line.into();
    test_statement(&stm, "% line comment");

    let block = comment(&loc, "%* block comment *%", CommentType::Block).unwrap();
    assert_eq!(block.comment_type().unwrap(), CommentType::Block);
    match Statement::from(block).is_a().unwrap() {
        StatementIsA::Comment(c) => assert_eq!(c.value().unwrap(), "%* block comment *%"),
        x => panic!("unexpected statement: {:?}", x),
    }
}

fn ground_base(arguments: Vec<String>, program: &str) -> Control {
    let mut ctl = control(arguments).unwrap();
    ctl.add("base", &[], program)
        .expect("Failed to add a logic program.");
    let part = Part::new("base", vec![]).unwrap();
    ctl.ground(&[part])
        .expect("Failed to ground a logic program.");
    ctl
}
fn atom_literal(ctl: &Control, name: &str) -> SolverLiteral {
    ctl.symbolic_atoms()
        .unwrap()
        .iter()
        .unwrap()
        .find(|atom| atom.symbol().unwrap().to_string() == name)
        .unwrap()
        .literal()
        .unwrap()
}
#[test]
fn model_is_consequence() {
    // during normal enumeration consequences correspond to the truth values in the model
    let ctl = ground_base(vec!["0".into()], "a. {b}.");
    let a = atom_literal(&ctl, "a");
    let b = atom_literal(&ctl, "b");
    let mut handle = ctl.solve(SolveMode::YIELD, &[]).unwrap();
    let mut models = 0;
    loop {
        handle.resume().unwrap();
        match handle.model().unwrap() {
            Some(model) => {
                models += 1;
                assert_eq!(model.is_consequence(a).unwrap(), Consequence::True);
                let expected = if model.is_true(b).unwrap() {
                    Consequence::True
                } else {
                    Consequence::False
                };
                assert_eq!(model.is_consequence(b).unwrap(), expected);
            }
            None => break,
        }
    }
    assert_eq!(models, 2);
    handle.close().unwrap();

    // the last model of the cautious consequences only contains a
    let ctl = ground_base(vec!["--enum-mode=cautious".into(), "0".into()], "a. {b}.");
    let a = atom_literal(&ctl, "a");
    let b = atom_literal(&ctl, "b");
    let mut handle = ctl.solve(SolveMode::YIELD, &[]).unwrap();
    let mut last = None;
    loop {
        handle.resume().unwrap();
        match handle.model().unwrap() {
            Some(model) => {
                last = Some((
                    model.is_consequence(a).unwrap(),
                    model.is_consequence(b).unwrap(),
                ))
            }
            None => break,
        }
    }
    assert_eq!(last, Some((Consequence::True, Consequence::False)));
    handle.close().unwrap();
}
#[test]
fn model_priority() {
    let ctl = ground_base(
        vec!["0".into()],
        "{a; b}. :- not a, not b. #minimize { 1@2: a }. #minimize { 1@1: b }.",
    );
    let mut handle = ctl.solve(SolveMode::YIELD, &[]).unwrap();
    let mut models = 0;
    loop {
        handle.resume().unwrap();
        match handle.model().unwrap() {
            Some(model) => {
                models += 1;
                assert_eq!(model.priority().unwrap(), [2, 1]);
                assert_eq!(model.cost().unwrap().len(), 2);
            }
            None => break,
        }
    }
    assert!(models > 0);
    handle.close().unwrap();
}
#[test]
fn solve_handle_last() {
    let ctl = ground_base(vec![], "1 {a; b; c}. #minimize { 1,a: a; 2,b: b; 3,c: c }.");
    let mut handle = ctl.solve(SolveMode::empty(), &[]).unwrap();
    assert!(handle.get().unwrap().contains(SolveResult::SATISFIABLE));
    let model = handle.last().unwrap().expect("Expected a last model.");
    assert_eq!(model.cost().unwrap(), [1]);
    assert!(model.optimality_proven().unwrap());
    assert!(model
        .contains(Symbol::create_id("a", true).unwrap())
        .unwrap());
    handle.close().unwrap();

    let ctl = ground_base(vec![], "a. :- a.");
    let mut handle = ctl.solve(SolveMode::empty(), &[]).unwrap();
    assert!(handle.get().unwrap().contains(SolveResult::UNSATISFIABLE));
    assert!(handle.last().unwrap().is_none());
    handle.close().unwrap();
}
#[test]
fn model_context() {
    let ctl = ground_base(vec!["0".into()], "{a; b}.");
    let a = atom_literal(&ctl, "a");
    let mut handle = ctl.solve(SolveMode::YIELD, &[]).unwrap();
    let mut models = vec![];
    loop {
        handle.resume().unwrap();
        match handle.model_mut().unwrap() {
            Some(model) => {
                models.push(model.is_true(a).unwrap());
                let context = model.context().unwrap();
                assert_eq!(context.symbolic_atoms().unwrap().size().unwrap(), 2);
                // all following models have to contain a
                context.add_clause(&[a]).unwrap();
            }
            None => break,
        }
    }
    handle.close().unwrap();
    assert!(models.len() < 4);
    assert!(models[1..].iter().all(|contains_a| *contains_a));
}

fn models(ctl: Control) -> Vec<Vec<String>> {
    let mut models = vec![];
    let mut handle = ctl.solve(SolveMode::YIELD, &[]).unwrap();
    loop {
        handle.resume().unwrap();
        match handle.model().unwrap() {
            Some(model) => {
                let mut symbols: Vec<String> = model
                    .symbols(ShowType::SHOWN)
                    .unwrap()
                    .iter()
                    .map(|symbol| symbol.to_string())
                    .collect();
                symbols.sort();
                models.push(symbols)
            }
            None => break,
        }
    }
    handle.close().unwrap();
    models.sort();
    models
}
fn temp_file(name: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(format!("clingo-rs-{}-{}", std::process::id(), name))
}
#[test]
fn control_load() {
    let path = temp_file("program.lp");
    std::fs::write(&path, "a. {b}.").unwrap();
    let mut ctl = control(vec!["0".into()]).unwrap();
    ctl.load(path.to_str().unwrap()).unwrap();
    std::fs::remove_file(&path).unwrap();
    ctl.ground(&[Part::new("base", vec![]).unwrap()]).unwrap();
    assert_eq!(models(ctl), [vec!["a"], vec!["a", "b"]]);

    let mut ctl = control(vec![]).unwrap();
    assert!(ctl.load(path.to_str().unwrap()).is_err());
}
#[test]
fn control_aspif_backend() {
    let path = temp_file("program.aspif");
    let file = path.to_str().unwrap();

    let mut ctl = control(vec![]).unwrap();
    ctl.register_backend(BackendType::Aspif, file, true)
        .unwrap();
    ctl.add("base", &[], "a. {b}.").unwrap();
    ctl.ground(&[Part::new("base", vec![]).unwrap()]).unwrap();
    // the step is only completed when solving
    let mut handle = ctl.solve(SolveMode::empty(), &[]).unwrap();
    handle.get().unwrap();
    drop(handle.close().unwrap());
    let aspif = std::fs::read_to_string(&path).unwrap();
    assert!(aspif.starts_with("asp "));
    assert!(aspif.ends_with("\n0\n"));

    let mut ctl = control(vec!["0".into()]).unwrap();
    ctl.load_aspif(&[file]).unwrap();
    ctl.ground(&[Part::new("base", vec![]).unwrap()]).unwrap();
    assert_eq!(models(ctl), [vec!["a"], vec!["a", "b"]]);
    std::fs::remove_file(&path).unwrap();
}
#[test]
fn control_reify_backend() {
    let path = temp_file("program.reify");
    let file = path.to_str().unwrap();

    let mut ctl = control(vec![]).unwrap();
    let reify = BackendType::Reify {
        sccs: false,
        steps: false,
    };
    ctl.register_backend(reify, file, true).unwrap();
    ctl.add("base", &[], "{a}.").unwrap();
    ctl.ground(&[Part::new("base", vec![]).unwrap()]).unwrap();
    drop(ctl);
    let reified = std::fs::read_to_string(&path).unwrap();
    assert!(reified.contains("rule(choice("));
    std::fs::remove_file(&path).unwrap();
}
struct NonCtx(defaults::Non);
impl ControlCtx for NonCtx {
    type L = defaults::Non;
    type P = defaults::Non;
    type O = defaults::Non;
    type F = defaults::Non;
    fn logger(&mut self) -> (&mut Self::L, u32) {
        (&mut self.0, 0)
    }
    fn propagator(&mut self) -> (&mut Self::P, bool) {
        (&mut self.0, false)
    }
    fn observer(&mut self) -> (&mut Self::O, bool) {
        (&mut self.0, false)
    }
    fn function_handler(&mut self) -> &mut Self::F {
        &mut self.0
    }
}
#[test]
fn control_with_context_drop() {
    // the reified program is written when the control object is freed
    let path = temp_file("context.reify");
    let file = path.to_str().unwrap();

    let mut ctl = control_with_context(vec![], NonCtx(defaults::Non)).unwrap();
    let reify = BackendType::Reify {
        sccs: false,
        steps: false,
    };
    ctl.register_backend(reify, file, true).unwrap();
    ctl.add("base", &[], "{a}.").unwrap();
    ctl.ground(&[Part::new("base", vec![]).unwrap()]).unwrap();
    drop(ctl);
    let reified = std::fs::read_to_string(&path).unwrap();
    std::fs::remove_file(&path).unwrap();
    assert!(reified.contains("rule(choice("));
}
struct FinishHandler(std::rc::Rc<std::cell::Cell<bool>>);
impl SolveEventHandler for FinishHandler {
    fn on_solve_event(&mut self, event: SolveEvent, _goon: &mut bool) -> bool {
        if let SolveEvent::Finish(_) = event {
            self.0.set(true);
        }
        true
    }
}
#[test]
fn solve_handle_drop() {
    // dropping a solve handle closes it, which finishes the search
    let finished = std::rc::Rc::new(std::cell::Cell::new(false));
    let ctl = ground_base(vec!["0".into()], "{a;b;c}.");
    let mut handle = ctl
        .solve_with_event_handler(SolveMode::YIELD, &[], FinishHandler(finished.clone()))
        .unwrap();
    handle.resume().unwrap();
    assert!(handle.model().unwrap().is_some());
    assert!(!finished.get());
    drop(handle);
    assert!(finished.get());
}
#[test]
fn control_remove_minimize() {
    let mut ctl = ground_base(vec!["0".into()], "{a}. #minimize { 1: a }.");
    ctl.remove_minimize().unwrap();
    let mut handle = ctl.solve(SolveMode::YIELD, &[]).unwrap();
    let mut count = 0;
    loop {
        handle.resume().unwrap();
        match handle.model().unwrap() {
            Some(model) => {
                count += 1;
                assert!(model.cost().unwrap().is_empty());
            }
            None => break,
        }
    }
    handle.close().unwrap();
    assert_eq!(count, 2);
}
#[test]
fn control_update_project() {
    // projective enumeration has to be enabled explicitly
    let args = || vec!["--project".into(), "0".into()];
    let mut ctl = ground_base(args(), "{a; b}.");
    let a = Atom::from(atom_literal(&ctl, "a"));
    let b = Atom::from(atom_literal(&ctl, "b"));
    ctl.update_project(&[b], false).unwrap();
    ctl.update_project(&[a], false).unwrap();
    assert_eq!(models(ctl).len(), 2);

    let mut ctl = ground_base(args(), "{a; b; c}.");
    let a = Atom::from(atom_literal(&ctl, "a"));
    let b = Atom::from(atom_literal(&ctl, "b"));
    ctl.update_project(&[a], false).unwrap();
    ctl.update_project(&[b], true).unwrap();
    assert_eq!(models(ctl).len(), 4);
}

#[test]
fn backend_theory_atoms() {
    let mut ctl = control(vec![]).unwrap();
    {
        let mut backend = ctl.backend().unwrap();
        let one = backend.theory_term_number(1).unwrap();
        let x = backend.theory_term_string("x").unwrap();
        let c = backend
            .theory_term_symbol(Symbol::create_id("c", true).unwrap())
            .unwrap();
        let tuple = backend
            .theory_term_sequence(TheorySequenceType::Tuple, &[one, x])
            .unwrap();
        let set = backend
            .theory_term_sequence(TheorySequenceType::Set, &[one])
            .unwrap();
        let list = backend
            .theory_term_sequence(TheorySequenceType::List, &[one])
            .unwrap();
        let f = backend.theory_term_function("f", &[one, c]).unwrap();
        let condition = backend.add_atom(None).unwrap();
        let element = backend
            .theory_element(&[tuple, set, list, f], &[condition.into()])
            .unwrap();

        let sum = backend.theory_term_string("sum").unwrap();
        let atom = backend
            .theory_atom_with_guard(TheoryAtomTarget::Fresh, sum, &[element], "<=", one)
            .unwrap();
        assert_ne!(SolverLiteral::from(atom).get_integer(), 0);
        let directive = backend.theory_term_string("directive").unwrap();
        backend
            .theory_atom(TheoryAtomTarget::Directive, directive, &[])
            .unwrap();
    }
    let atoms = ctl.theory_atoms().unwrap();
    let strings: Vec<String> = atoms
        .iter()
        .map(|atom| atoms.atom_to_string(atom).unwrap())
        .collect();
    // the sequence types also check the order of the set and list values changed in clingo 5.8
    assert_eq!(
        strings,
        ["&sum{(1,x),{1},[1],f(1,c): #aux(1)}<=1", "&directive{}"]
    );
}
#[test]
fn symbolic_atoms_find() {
    let ctl = ground_base(vec![], "a. {b}. c :- d.");
    let atoms = ctl.symbolic_atoms().unwrap();
    let id = |name| Symbol::create_id(name, true).unwrap();

    let a = atoms.find(id("a")).unwrap().expect("a should be found");
    assert_eq!(a.symbol().unwrap(), id("a"));
    assert!(a.is_fact().unwrap());
    let b = atoms.find(id("b")).unwrap().expect("b should be found");
    assert!(!b.is_fact().unwrap());
    assert!(atoms.find(id("c")).unwrap().is_none());
    assert!(atoms.find(id("x")).unwrap().is_none());
}

#[derive(Default)]
struct StatementCollector {
    statements: Vec<String>,
}
impl StatementHandler for StatementCollector {
    fn on_statement(&mut self, stm: &Statement) -> bool {
        self.statements.push(stm.to_string().unwrap());
        true
    }
}
#[test]
fn ast_parse_comments() {
    let mut collector = StatementCollector::default();
    parse_string_with_statement_handler(
        "% line comment\na :- b. %* block comment *%\n",
        &mut collector,
    )
    .unwrap();
    assert_eq!(
        collector.statements,
        [
            "#program base.",
            "% line comment",
            "a :- b.",
            "%* block comment *%"
        ]
    );
}
#[test]
fn ast_parse_theory_definition() {
    let mut collector = StatementCollector::default();
    parse_string_with_statement_handler(
        "#theory t { term { }; &a/0: term, head }.",
        &mut collector,
    )
    .unwrap();
    assert_eq!(collector.statements.len(), 2);
    assert!(collector.statements[1].starts_with("#theory t {"));

    struct TheoryDefinitionCheck;
    impl StatementHandler for TheoryDefinitionCheck {
        fn on_statement(&mut self, stm: &Statement) -> bool {
            stm.clone().is_a().is_ok()
        }
    }
    parse_string_with_statement_handler(
        "#theory t { term { }; &a/0: term, head }.",
        &mut TheoryDefinitionCheck,
    )
    .unwrap();
}
// #[test]
// fn ui() {
//     let t = trybuild::TestCases::new();
//     t.compile_fail("tests/ui/ast_term_from_unary_operation.rs");
//     t.compile_fail("tests/ui/ast_term_from_binary_operation.rs");
//     t.compile_fail("tests/ui/ast_term_from_function.rs");
//     t.compile_fail("tests/ui/ast_term_from_pool.rs");
//     t.compile_fail("tests/ui/ast_comparison.rs");
//     t.compile_fail("tests/ui/ast_unary_operation.rs");
//     t.compile_fail("tests/ui/ast_binary_operation.rs");
//     t.compile_fail("tests/ui/ast_guard.rs");
//     t.compile_fail("tests/ui/ast_function.rs");
//     t.compile_fail("tests/ui/ast_interval.rs");
//     t.compile_fail("tests/ui/ast_pool.rs");
//     t.compile_fail("tests/ui/ast_literal_from_term.rs");
//     t.compile_fail("tests/ui/ast_literal_from_comparison.rs");
//     t.compile_fail("tests/ui/ast_aggregate.rs");
//     t.compile_fail("tests/ui/ast_conditional_literal.rs");
//     t.compile_fail("tests/ui/ast_head_aggregate.rs");
//     t.compile_fail("tests/ui/ast_head_aggregate_element.rs");
//     t.compile_fail("tests/ui/ast_disjunction.rs");
//     t.compile_fail("tests/ui/ast_head_literal.rs");
//     t.compile_fail("tests/ui/ast_body_literal_from_term.rs");
//     t.compile_fail("tests/ui/ast_rule.rs");
//     t.compile_fail("tests/ui/ast_external.rs");
//     t.compile_fail("tests/ui/ast_statement_from_external.rs");
//     t.compile_fail("tests/ui/ast_statement_from_rule.rs");
//     //check builder.add(stmt)
// }
#[test]
fn ast_location() {
    let location = Location::new("a.lp", "b.lp", 2, 3, 4, 5).unwrap();
    // the file names must still be valid after other allocations
    let _other = vec![String::from("x"); 100];
    assert_eq!(location.begin_file().unwrap(), "a.lp");
    assert_eq!(location.end_file().unwrap(), "b.lp");
    assert_eq!(
        (
            location.begin_line(),
            location.end_line(),
            location.begin_column(),
            location.end_column()
        ),
        (2, 3, 4, 5)
    );
    assert!(Location::new("a\0", "b", 0, 0, 0, 0).is_err());
    let location = Location::default();
    assert_eq!(location.begin_file().unwrap(), "");
    assert_eq!(location.end_file().unwrap(), "");
}

/// Call `f` for each statement of `program` except the initial `#program base.`
///
/// Panics in `f` are forwarded after parsing because they must not unwind through clingo.
fn for_each_statement(program: &str, f: impl FnMut(&Statement)) {
    struct Handler<F> {
        f: F,
        panic: Option<Box<dyn std::any::Any + Send>>,
    }
    impl<F: FnMut(&Statement)> StatementHandler for Handler<F> {
        fn on_statement(&mut self, stm: &Statement) -> bool {
            if Node::from(stm.clone()).node_type().unwrap() == NodeType::Program {
                return true;
            }
            let f = &mut self.f;
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(stm))) {
                Ok(()) => true,
                Err(panic) => {
                    self.panic = Some(panic);
                    false
                }
            }
        }
    }
    let mut handler = Handler { f, panic: None };
    let result = parse_string_with_statement_handler(program, &mut handler);
    if let Some(panic) = handler.panic {
        std::panic::resume_unwind(panic);
    }
    result.unwrap();
}
#[test]
fn ast_node_number_and_types() {
    let mut count = 0;
    for_each_statement("a :- b.", |stm| {
        count += 1;
        let mut rule = Node::from(stm.clone());
        assert_eq!(rule.node_type().unwrap(), NodeType::Rule);
        assert!(rule.has_attribute(Attribute::Head).unwrap());
        assert!(!rule.has_attribute(Attribute::Name).unwrap());
        assert_eq!(
            rule.attribute_type(Attribute::Head).unwrap(),
            AttributeType::Ast
        );
        assert_eq!(
            rule.attribute_type(Attribute::Body).unwrap(),
            AttributeType::AstArray
        );
        assert!(rule.attribute_type(Attribute::Name).is_err());

        let mut literal = rule.get_ast_at(Attribute::Body, 0).unwrap();
        assert_eq!(literal.node_type().unwrap(), NodeType::Literal);
        assert_eq!(
            literal.get_number(Attribute::Sign).unwrap(),
            Sign::NoSign as i32
        );
        literal
            .set_number(Attribute::Sign, Sign::Negation as i32)
            .unwrap();
        // the literal is shared with the rule
        assert_eq!(rule.to_string().unwrap(), "a :- not b.");
        assert!(rule.set_number(Attribute::Sign, 0).is_err());
    });
    assert_eq!(count, 1);
}
#[test]
fn ast_node_symbol_string_location() {
    for_each_statement("p(1).", |stm| {
        let rule = Node::from(stm.clone());
        let literal = rule.get_ast(Attribute::Head).unwrap();
        let atom = literal.get_ast(Attribute::Atom).unwrap();
        let mut function = atom.get_ast(Attribute::Symbol).unwrap();
        assert_eq!(function.node_type().unwrap(), NodeType::Function);
        assert_eq!(function.get_string(Attribute::Name).unwrap(), "p");
        function.set_string(Attribute::Name, "q").unwrap();

        let mut term = function.get_ast_at(Attribute::Arguments, 0).unwrap();
        assert_eq!(term.node_type().unwrap(), NodeType::SymbolicTerm);
        assert_eq!(
            term.get_symbol(Attribute::Symbol).unwrap(),
            Symbol::create_number(1)
        );
        term.set_symbol(Attribute::Symbol, Symbol::create_number(2))
            .unwrap();
        assert_eq!(rule.to_string().unwrap(), "q(2).");

        let mut rule = rule;
        let location = rule.get_location(Attribute::Location).unwrap();
        assert_eq!(location.begin_line(), 1);
        let new_location = Location::new("a.lp", "b.lp", 2, 3, 4, 5).unwrap();
        rule.set_location(Attribute::Location, &new_location)
            .unwrap();
        let location = rule.get_location(Attribute::Location).unwrap();
        assert_eq!(location.begin_file().unwrap(), "a.lp");
        assert_eq!(location.end_file().unwrap(), "b.lp");
        assert_eq!(
            (
                location.begin_line(),
                location.end_line(),
                location.begin_column(),
                location.end_column()
            ),
            (2, 3, 4, 5)
        );
    });
}
#[test]
fn ast_node_ast_array() {
    for_each_statement("a :- b; c.", |stm| {
        let mut rule = Node::from(stm.clone());
        assert_eq!(rule.ast_array_size(Attribute::Body).unwrap(), 2);
        let c = rule.get_ast_at(Attribute::Body, 1).unwrap();
        rule.delete_ast_at(Attribute::Body, 0).unwrap();
        assert_eq!(rule.to_string().unwrap(), "a :- c.");
        rule.insert_ast_at(Attribute::Body, 1, &c).unwrap();
        assert_eq!(rule.to_string().unwrap(), "a :- c; c.");
        let head = rule.get_ast(Attribute::Head).unwrap();
        rule.set_ast_at(Attribute::Body, 0, &head).unwrap();
        assert_eq!(rule.to_string().unwrap(), "a :- a; c.");
        rule.set_ast(Attribute::Head, &c).unwrap();
        assert_eq!(rule.to_string().unwrap(), "c :- a; c.");
        assert_eq!(rule.ast_array_size(Attribute::Body).unwrap(), 2);

        // clingo does not check indices, the wrapper has to
        assert!(rule.get_ast_at(Attribute::Body, 2).is_err());
        assert!(rule.set_ast_at(Attribute::Body, 2, &c).is_err());
        assert!(rule.delete_ast_at(Attribute::Body, 2).is_err());
        assert!(rule.insert_ast_at(Attribute::Body, 3, &c).is_err());
        rule.insert_ast_at(Attribute::Body, 2, &c).unwrap();
        assert_eq!(rule.ast_array_size(Attribute::Body).unwrap(), 3);
    });
}
#[test]
fn ast_node_optional_ast() {
    for_each_statement("{ a }.", |stm| {
        let rule = Node::from(stm.clone());
        let mut aggregate = rule.get_ast(Attribute::Head).unwrap();
        assert_eq!(aggregate.node_type().unwrap(), NodeType::Aggregate);
        assert!(aggregate
            .get_optional_ast(Attribute::LeftGuard)
            .unwrap()
            .is_none());

        let loc = Location::default();
        let one = symbolic_term(&loc, &Symbol::create_number(1)).unwrap();
        let guard = Node::from(guard(ComparisonOperator::LessEqual, one).unwrap());
        aggregate
            .set_optional_ast(Attribute::LeftGuard, Some(&guard))
            .unwrap();
        assert_eq!(rule.to_string().unwrap(), "1 <= { a }.");
        let left_guard = aggregate
            .get_optional_ast(Attribute::LeftGuard)
            .unwrap()
            .unwrap();
        assert_eq!(left_guard.node_type().unwrap(), NodeType::Guard);

        aggregate
            .set_optional_ast(Attribute::LeftGuard, None)
            .unwrap();
        assert_eq!(rule.to_string().unwrap(), "{ a }.");
    });
}
#[test]
fn ast_node_string_array() {
    for_each_statement("&a { - x }.", |stm| {
        let rule = Node::from(stm.clone());
        let atom = rule.get_ast(Attribute::Head).unwrap();
        let element = atom.get_ast_at(Attribute::Elements, 0).unwrap();
        let term = element.get_ast_at(Attribute::Terms, 0).unwrap();
        assert_eq!(term.node_type().unwrap(), NodeType::TheoryUnparsedTerm);
        let mut unparsed = term.get_ast_at(Attribute::Elements, 0).unwrap();
        assert_eq!(
            unparsed.node_type().unwrap(),
            NodeType::TheoryUnparsedTermElement
        );
        assert_eq!(
            unparsed.attribute_type(Attribute::Operators).unwrap(),
            AttributeType::StringArray
        );
        assert_eq!(unparsed.string_array_size(Attribute::Operators).unwrap(), 1);
        assert_eq!(
            unparsed.get_string_at(Attribute::Operators, 0).unwrap(),
            "-"
        );

        unparsed
            .set_string_at(Attribute::Operators, 0, "~")
            .unwrap();
        unparsed
            .insert_string_at(Attribute::Operators, 1, "+")
            .unwrap();
        assert_eq!(unparsed.string_array_size(Attribute::Operators).unwrap(), 2);
        assert_eq!(
            unparsed.get_string_at(Attribute::Operators, 1).unwrap(),
            "+"
        );
        assert_eq!(rule.to_string().unwrap(), "&a { (~ + x) }.");
        unparsed.delete_string_at(Attribute::Operators, 0).unwrap();
        assert_eq!(rule.to_string().unwrap(), "&a { (+ x) }.");

        // clingo does not check indices, the wrapper has to
        assert!(unparsed.get_string_at(Attribute::Operators, 1).is_err());
        assert!(unparsed
            .set_string_at(Attribute::Operators, 1, "-")
            .is_err());
        assert!(unparsed.delete_string_at(Attribute::Operators, 1).is_err());
        assert!(unparsed
            .insert_string_at(Attribute::Operators, 2, "-")
            .is_err());
    });
}
#[test]
fn ast_node_copy_and_statement() {
    for_each_statement("a :- b.", |stm| {
        let rule = Node::from(stm.clone());
        let deep = rule.clone();
        let shallow = rule.copy().unwrap();
        let mut literal = shallow.get_ast_at(Attribute::Body, 0).unwrap();
        literal
            .set_number(Attribute::Sign, Sign::Negation as i32)
            .unwrap();
        // a shallow copy shares its children
        assert_eq!(rule.to_string().unwrap(), "a :- not b.");
        assert_eq!(deep.to_string().unwrap(), "a :- b.");

        let statement = Statement::try_from(deep).unwrap();
        assert_eq!(statement.to_string().unwrap(), "a :- b.");
        assert!(Statement::try_from(literal).is_err());
    });
}
#[test]
fn ast_node_compare_and_hash() {
    use std::collections::{BTreeSet, HashSet};
    let loc1 = Location::new("a.lp", "a.lp", 1, 1, 1, 2).unwrap();
    let loc2 = Location::new("b.lp", "b.lp", 3, 3, 1, 2).unwrap();
    let one_a = symbolic_term(&loc1, &Symbol::create_number(1)).unwrap();
    let one_b = symbolic_term(&loc2, &Symbol::create_number(1)).unwrap();
    let two = symbolic_term(&loc1, &Symbol::create_number(2)).unwrap();
    // locations are ignored
    assert_eq!(one_a, one_b);
    assert_ne!(one_a, two);
    assert!(one_a < two);
    assert!(two > one_b);

    let set: HashSet<Node> = [one_a.clone(), one_b.clone(), two.clone()]
        .into_iter()
        .map(Node::from)
        .collect();
    assert_eq!(set.len(), 2);
    let sorted: Vec<String> = [two, one_a, one_b]
        .into_iter()
        .map(Node::from)
        .collect::<BTreeSet<Node>>()
        .iter()
        .map(|node| node.to_string().unwrap())
        .collect();
    assert_eq!(sorted, ["1", "2"]);

    for_each_statement("a :- b.", |stm| {
        let rule = Node::from(stm.clone());
        let mut other = rule.clone();
        other
            .set_location(Attribute::Location, &Location::default())
            .unwrap();
        assert_eq!(rule, other);
        assert_eq!(stm.clone(), Statement::try_from(other.clone()).unwrap());
        other
            .get_ast_at(Attribute::Body, 0)
            .unwrap()
            .set_number(Attribute::Sign, Sign::Negation as i32)
            .unwrap();
        assert_ne!(rule, other);
        assert!((rule < other) != (other < rule));
    });
}
#[test]
fn ast_unpool() {
    let strings = |nodes: Vec<Node>| -> Vec<String> {
        nodes.iter().map(|node| node.to_string().unwrap()).collect()
    };
    for_each_statement("a(1;2) :- b(X;Y).", |stm| {
        let unpooled = [
            "a(1) :- b(X).",
            "a(2) :- b(X).",
            "a(1) :- b(Y).",
            "a(2) :- b(Y).",
        ];
        for unpool_type in [UnpoolType::Other, UnpoolType::All] {
            let statements = stm.unpool(unpool_type).unwrap();
            let statements: Vec<String> = statements
                .iter()
                .map(|stm| stm.to_string().unwrap())
                .collect();
            assert_eq!(statements, unpooled);
        }
        // conditions are only unpooled in conditional literals
        let statements = stm.unpool(UnpoolType::Condition).unwrap();
        assert_eq!(statements, std::slice::from_ref(stm));
    });
    for_each_statement("a :- b(1;2) : c(X;Y).", |stm| {
        let rule = Node::from(stm.clone());
        let literal = rule.get_ast_at(Attribute::Body, 0).unwrap();
        assert_eq!(literal.node_type().unwrap(), NodeType::ConditionalLiteral);
        assert_eq!(
            strings(literal.unpool(UnpoolType::Condition).unwrap()),
            ["b(1;2): c(X)", "b(1;2): c(Y)"]
        );
        assert_eq!(
            strings(literal.unpool(UnpoolType::Other).unwrap()),
            ["b(1): c(X;Y)", "b(2): c(X;Y)"]
        );
        assert_eq!(literal.unpool(UnpoolType::All).unwrap().len(), 4);
    });
    for_each_statement("a.", |stm| {
        // nothing to unpool
        let nodes = Node::from(stm.clone()).unpool(UnpoolType::All).unwrap();
        assert_eq!(strings(nodes), ["a."]);
    });
}
#[test]
fn ast_parse_files() {
    let path1 = temp_file("parse1.lp");
    let path2 = temp_file("parse2.lp");
    std::fs::write(&path1, "a :- b.").unwrap();
    std::fs::write(&path2, "% comment\nb.").unwrap();
    let mut collector = StatementCollector::default();
    parse_files_with_statement_handler(
        &[path1.to_str().unwrap(), path2.to_str().unwrap()],
        &mut collector,
    )
    .unwrap();
    std::fs::remove_file(&path1).unwrap();
    std::fs::remove_file(&path2).unwrap();
    // each file starts with its own base program
    let mut statements = collector.statements;
    statements.sort();
    assert_eq!(
        statements,
        [
            "#program base.",
            "#program base.",
            "% comment",
            "a :- b.",
            "b."
        ]
    );

    let mut collector = StatementCollector::default();
    assert!(
        parse_files_with_statement_handler(&[path1.to_str().unwrap()], &mut collector).is_err()
    );
    assert!(parse_files_with_statement_handler(&["a\0"], &mut collector).is_err());
}
#[test]
fn ast_function_name_and_theory_atom_parts() {
    let loc = Location::default();
    let one = symbolic_term(&loc, &Symbol::create_number(1)).unwrap();
    let args = [one.clone().into()];
    let fun = function(&loc, "f", &args, false).unwrap();
    assert_eq!(fun.name().unwrap(), "f");

    let name = symbolic_term(&loc, &Symbol::create_id("a", true).unwrap()).unwrap();
    let tuple = [one.clone().into()];
    let elements = [
        theory_atom_element(&tuple, &[]).unwrap(),
        theory_atom_element(&tuple, &[]).unwrap(),
    ];
    let atom = theory_atom(&loc, name.clone(), &elements, None).unwrap();
    assert_eq!(atom.elements().unwrap(), elements);
    assert!(atom.guard().unwrap().is_none());

    let guard = theory_guard(">", one).unwrap();
    let atom = theory_atom(&loc, name, &[], Some(guard.clone())).unwrap();
    assert!(atom.elements().unwrap().is_empty());
    assert_eq!(atom.guard().unwrap(), Some(guard));
}
