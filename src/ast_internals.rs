use crate::ast::{BodyLiteral, Head, Location, Term, TheoryAtomElement, TheoryGuard};
use crate::{ClingoError, Symbol};
use clingo_sys::*;
use std::marker::PhantomData;
use std::os::raw::c_void;
use std::ptr::NonNull;
use crate::{set_internal_error, ErrorType};
use std::cmp::Ordering;
use std::ffi::{CStr, CString};
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub struct Body<'a>(ASTArray<'a>);
impl<'a> Iterator for Body<'a> {
    type Item = BodyLiteral<'a>;

    fn next(&mut self) -> Option<BodyLiteral<'a>> {
        let ast = self.0.next()?;
        Some(BodyLiteral { ast })
    }
}
impl<'a> Body<'a> {
    pub fn size(&self) -> Result<usize, ClingoError> {
        self.0.size()
    }
}
#[derive(Debug, Clone)]
struct ASTArray<'a> {
    ast: &'a Ast<'a>,
    index: usize,
}
impl<'a> ASTArray<'a> {
    /// Get the size of an ASTArray
    ///
    /// might set one of the following error codes:
    /// - ::clingo_error_runtime
    pub(crate) fn size(&self) -> Result<usize, ClingoError> {
        let mut size: usize = 0;
        if !unsafe {
            clingo_ast_attribute_size_ast_array(
                self.ast.ptr.as_ptr(),
                Attribute::Body as i32,
                &mut size,
            )
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_ast_attribute_size_ast_array() failed.",
            ));
        }
        Ok(size)
    }
}
impl<'a> Iterator for ASTArray<'a> {
    type Item = Ast<'a>;

    fn next(&mut self) -> Option<Ast<'a>> {
        let size = self.size().unwrap(); //Err->None

        if size == self.index {
            return None;
        }

        if let Ok(ast) = self
            .ast
            .get_attribute_ast_at(Attribute::Body, self.index)
        {
            self.index += 1;
            Some(ast)
        } else {
            None
        }
    }
}

/// Enumeration of AST node types.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum NodeType {
    Id = clingo_ast_type_e_clingo_ast_type_id as isize,
    Variable = clingo_ast_type_e_clingo_ast_type_variable as isize,
    SymbolicTerm = clingo_ast_type_e_clingo_ast_type_symbolic_term as isize,
    UnaryOperation = clingo_ast_type_e_clingo_ast_type_unary_operation as isize,
    BinaryOperation = clingo_ast_type_e_clingo_ast_type_binary_operation as isize,
    Interval = clingo_ast_type_e_clingo_ast_type_interval as isize,
    Function = clingo_ast_type_e_clingo_ast_type_function as isize,
    Pool = clingo_ast_type_e_clingo_ast_type_pool as isize,
    BooleanConstant = clingo_ast_type_e_clingo_ast_type_boolean_constant as isize,
    SymbolicAtom = clingo_ast_type_e_clingo_ast_type_symbolic_atom as isize,
    Comparison = clingo_ast_type_e_clingo_ast_type_comparison as isize,
    Guard = clingo_ast_type_e_clingo_ast_type_guard as isize,
    ConditionalLiteral = clingo_ast_type_e_clingo_ast_type_conditional_literal as isize,
    Aggregate = clingo_ast_type_e_clingo_ast_type_aggregate as isize,
    BodyAggregateElement = clingo_ast_type_e_clingo_ast_type_body_aggregate_element as isize,
    BodyAggregate = clingo_ast_type_e_clingo_ast_type_body_aggregate as isize,
    HeadAggregateElement = clingo_ast_type_e_clingo_ast_type_head_aggregate_element as isize,
    HeadAggregate = clingo_ast_type_e_clingo_ast_type_head_aggregate as isize,
    Disjunction = clingo_ast_type_e_clingo_ast_type_disjunction as isize,
    TheorySequence = clingo_ast_type_e_clingo_ast_type_theory_sequence as isize,
    TheoryFunction = clingo_ast_type_e_clingo_ast_type_theory_function as isize,
    TheoryUnparsedTermElement =
        clingo_ast_type_e_clingo_ast_type_theory_unparsed_term_element as isize,
    TheoryUnparsedTerm = clingo_ast_type_e_clingo_ast_type_theory_unparsed_term as isize,
    TheoryGuard = clingo_ast_type_e_clingo_ast_type_theory_guard as isize,
    TheoryAtomElement = clingo_ast_type_e_clingo_ast_type_theory_atom_element as isize,
    TheoryAtom = clingo_ast_type_e_clingo_ast_type_theory_atom as isize,
    Literal = clingo_ast_type_e_clingo_ast_type_literal as isize,
    TheoryOperatorDefinition =
        clingo_ast_type_e_clingo_ast_type_theory_operator_definition as isize,
    TheoryTermDefinition = clingo_ast_type_e_clingo_ast_type_theory_term_definition as isize,
    TheoryGuardDefinition = clingo_ast_type_e_clingo_ast_type_theory_guard_definition as isize,
    TheoryAtomDefinition = clingo_ast_type_e_clingo_ast_type_theory_atom_definition as isize,
    Rule = clingo_ast_type_e_clingo_ast_type_rule as isize,
    Definition = clingo_ast_type_e_clingo_ast_type_definition as isize,
    ShowSignature = clingo_ast_type_e_clingo_ast_type_show_signature as isize,
    ShowTerm = clingo_ast_type_e_clingo_ast_type_show_term as isize,
    Minimize = clingo_ast_type_e_clingo_ast_type_minimize as isize,
    Script = clingo_ast_type_e_clingo_ast_type_script as isize,
    Program = clingo_ast_type_e_clingo_ast_type_program as isize,
    External = clingo_ast_type_e_clingo_ast_type_external as isize,
    Edge = clingo_ast_type_e_clingo_ast_type_edge as isize,
    Heuristic = clingo_ast_type_e_clingo_ast_type_heuristic as isize,
    ProjectAtom = clingo_ast_type_e_clingo_ast_type_project_atom as isize,
    ProjectSignature = clingo_ast_type_e_clingo_ast_type_project_signature as isize,
    Defined = clingo_ast_type_e_clingo_ast_type_defined as isize,
    TheoryDefinition = clingo_ast_type_e_clingo_ast_type_theory_definition as isize,
    Comment = clingo_ast_type_e_clingo_ast_type_comment as isize,
}
impl NodeType {
    /// Whether nodes of this type are statements.
    pub fn is_statement(self) -> bool {
        matches!(
            self,
            NodeType::Rule
                | NodeType::Definition
                | NodeType::ShowSignature
                | NodeType::Defined
                | NodeType::ShowTerm
                | NodeType::Minimize
                | NodeType::Script
                | NodeType::Program
                | NodeType::External
                | NodeType::Edge
                | NodeType::Heuristic
                | NodeType::ProjectAtom
                | NodeType::ProjectSignature
                | NodeType::TheoryDefinition
                | NodeType::Comment
        )
    }
    fn try_from(code: u32) -> Result<NodeType, ClingoError> {
        // println!("in try_from");
        match code {
            clingo_ast_type_e_clingo_ast_type_id => Ok(NodeType::Id),
            clingo_ast_type_e_clingo_ast_type_variable => Ok(NodeType::Variable),
            clingo_ast_type_e_clingo_ast_type_symbolic_term => Ok(NodeType::SymbolicTerm),
            clingo_ast_type_e_clingo_ast_type_unary_operation => Ok(NodeType::UnaryOperation),
            clingo_ast_type_e_clingo_ast_type_binary_operation => Ok(NodeType::BinaryOperation),
            clingo_ast_type_e_clingo_ast_type_interval => Ok(NodeType::Interval),
            clingo_ast_type_e_clingo_ast_type_function => Ok(NodeType::Function),
            clingo_ast_type_e_clingo_ast_type_pool => Ok(NodeType::Pool),
            clingo_ast_type_e_clingo_ast_type_boolean_constant => Ok(NodeType::BooleanConstant),
            clingo_ast_type_e_clingo_ast_type_symbolic_atom => Ok(NodeType::SymbolicAtom),
            clingo_ast_type_e_clingo_ast_type_comparison => Ok(NodeType::Comparison),
            clingo_ast_type_e_clingo_ast_type_guard => Ok(NodeType::Guard),
            clingo_ast_type_e_clingo_ast_type_conditional_literal => {
                Ok(NodeType::ConditionalLiteral)
            }
            clingo_ast_type_e_clingo_ast_type_aggregate => Ok(NodeType::Aggregate),
            clingo_ast_type_e_clingo_ast_type_body_aggregate_element => {
                Ok(NodeType::BodyAggregateElement)
            }
            clingo_ast_type_e_clingo_ast_type_body_aggregate => Ok(NodeType::BodyAggregate),
            clingo_ast_type_e_clingo_ast_type_head_aggregate_element => {
                Ok(NodeType::HeadAggregateElement)
            }
            clingo_ast_type_e_clingo_ast_type_head_aggregate => Ok(NodeType::HeadAggregate),
            clingo_ast_type_e_clingo_ast_type_disjunction => Ok(NodeType::Disjunction),
            clingo_ast_type_e_clingo_ast_type_theory_sequence => Ok(NodeType::TheorySequence),
            clingo_ast_type_e_clingo_ast_type_theory_function => Ok(NodeType::TheoryFunction),
            clingo_ast_type_e_clingo_ast_type_theory_unparsed_term_element => {
                Ok(NodeType::TheoryUnparsedTermElement)
            }
            clingo_ast_type_e_clingo_ast_type_theory_unparsed_term => {
                Ok(NodeType::TheoryUnparsedTerm)
            }
            clingo_ast_type_e_clingo_ast_type_theory_guard => Ok(NodeType::TheoryGuard),
            clingo_ast_type_e_clingo_ast_type_theory_atom_element => Ok(NodeType::TheoryAtomElement),
            clingo_ast_type_e_clingo_ast_type_theory_atom => Ok(NodeType::TheoryAtom),
            clingo_ast_type_e_clingo_ast_type_literal => Ok(NodeType::Literal),
            clingo_ast_type_e_clingo_ast_type_theory_operator_definition => {
                Ok(NodeType::TheoryOperatorDefinition)
            }
            clingo_ast_type_e_clingo_ast_type_theory_term_definition => {
                Ok(NodeType::TheoryTermDefinition)
            }
            clingo_ast_type_e_clingo_ast_type_theory_guard_definition => {
                Ok(NodeType::TheoryGuardDefinition)
            }
            clingo_ast_type_e_clingo_ast_type_theory_atom_definition => {
                Ok(NodeType::TheoryAtomDefinition)
            }
            clingo_ast_type_e_clingo_ast_type_rule => Ok(NodeType::Rule),
            clingo_ast_type_e_clingo_ast_type_definition => Ok(NodeType::Definition),
            clingo_ast_type_e_clingo_ast_type_show_signature => Ok(NodeType::ShowSignature),
            clingo_ast_type_e_clingo_ast_type_show_term => Ok(NodeType::ShowTerm),
            clingo_ast_type_e_clingo_ast_type_minimize => Ok(NodeType::Minimize),
            clingo_ast_type_e_clingo_ast_type_script => Ok(NodeType::Script),
            clingo_ast_type_e_clingo_ast_type_program => Ok(NodeType::Program),
            clingo_ast_type_e_clingo_ast_type_external => Ok(NodeType::External),
            clingo_ast_type_e_clingo_ast_type_edge => Ok(NodeType::Edge),
            clingo_ast_type_e_clingo_ast_type_heuristic => Ok(NodeType::Heuristic),
            clingo_ast_type_e_clingo_ast_type_project_atom => Ok(NodeType::ProjectAtom),
            clingo_ast_type_e_clingo_ast_type_project_signature => Ok(NodeType::ProjectSignature),
            clingo_ast_type_e_clingo_ast_type_defined => Ok(NodeType::Defined),
            clingo_ast_type_e_clingo_ast_type_theory_definition => Ok(NodeType::TheoryDefinition),
            clingo_ast_type_e_clingo_ast_type_comment => Ok(NodeType::Comment),
            x => {
                eprintln!(
                    "FFIError in {} {}, {} : Failed to match clingo_ast_type {}",
                    file!(),
                    line!(),
                    column!(),
                    x
                );
                Err(ClingoError::FFIError {
                    msg: "Failed to match clingo_ast_type.",
                })
            }
        }
    }
}

/// Enumeration of attributes types used by the AST.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AttributeType {
    /// For an attribute of type `int`.
    Number = clingo_ast_attribute_type_e_clingo_ast_attribute_type_number as isize,
    /// For an attribute of type `clingo_ast_symbol_t`.
    Symbol = clingo_ast_attribute_type_e_clingo_ast_attribute_type_symbol as isize,
    /// For an attribute of type `clingo_location_t`.
    Location = clingo_ast_attribute_type_e_clingo_ast_attribute_type_location as isize,
    /// For an attribute of type `char const *`.
    String = clingo_ast_attribute_type_e_clingo_ast_attribute_type_string as isize,
    /// For an attribute of type `clingo_ast_t *`.
    Ast = clingo_ast_attribute_type_e_clingo_ast_attribute_type_ast as isize,
    /// For an attribute of type `clingo_ast_t *` that can be NULL.
    OptionalAst = clingo_ast_attribute_type_e_clingo_ast_attribute_type_optional_ast as isize,
    /// For an attribute of type `char const **`.
    StringArray = clingo_ast_attribute_type_e_clingo_ast_attribute_type_string_array as isize,
    /// For an attribute of type `clingo_ast_t **`.
    AstArray = clingo_ast_attribute_type_e_clingo_ast_attribute_type_ast_array as isize,
}

use std::convert::TryFrom;
impl TryFrom<u32> for AttributeType {
    type Error = ClingoError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            clingo_ast_attribute_type_e_clingo_ast_attribute_type_number => {
                Ok(AttributeType::Number)
            }
            clingo_ast_attribute_type_e_clingo_ast_attribute_type_symbol => {
                Ok(AttributeType::Symbol)
            }
            clingo_ast_attribute_type_e_clingo_ast_attribute_type_location => {
                Ok(AttributeType::Location)
            }
            clingo_ast_attribute_type_e_clingo_ast_attribute_type_string => {
                Ok(AttributeType::String)
            }
            clingo_ast_attribute_type_e_clingo_ast_attribute_type_ast => Ok(AttributeType::Ast),
            clingo_ast_attribute_type_e_clingo_ast_attribute_type_optional_ast => {
                Ok(AttributeType::OptionalAst)
            }
            clingo_ast_attribute_type_e_clingo_ast_attribute_type_string_array => {
                Ok(AttributeType::StringArray)
            }
            clingo_ast_attribute_type_e_clingo_ast_attribute_type_ast_array => {
                Ok(AttributeType::AstArray)
            }
            x => {
                eprintln!(
                    "FFIError in {} {}, {} : Failed to match clingo_ast_type {}",
                    file!(),
                    line!(),
                    column!(),
                    x
                );
                Err(ClingoError::FFIError {
                    msg: "Failed to match clingo_ast_type.",
                })
            }
        }
    }
}
/// Enumeration of attributes used by the AST.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Attribute {
    Argument = clingo_ast_attribute_e_clingo_ast_attribute_argument as isize,
    Arguments = clingo_ast_attribute_e_clingo_ast_attribute_arguments as isize,
    Arity = clingo_ast_attribute_e_clingo_ast_attribute_arity as isize,
    Atom = clingo_ast_attribute_e_clingo_ast_attribute_atom as isize,
    Atoms = clingo_ast_attribute_e_clingo_ast_attribute_atoms as isize,
    AtomType = clingo_ast_attribute_e_clingo_ast_attribute_atom_type as isize,
    Bias = clingo_ast_attribute_e_clingo_ast_attribute_bias as isize,
    Body = clingo_ast_attribute_e_clingo_ast_attribute_body as isize,
    Code = clingo_ast_attribute_e_clingo_ast_attribute_code as isize,
    Coefficient = clingo_ast_attribute_e_clingo_ast_attribute_coefficient as isize,
    Comparison = clingo_ast_attribute_e_clingo_ast_attribute_comparison as isize,
    Condition = clingo_ast_attribute_e_clingo_ast_attribute_condition as isize,
    Elements = clingo_ast_attribute_e_clingo_ast_attribute_elements as isize,
    External = clingo_ast_attribute_e_clingo_ast_attribute_external as isize,
    ExternalType = clingo_ast_attribute_e_clingo_ast_attribute_external_type as isize,
    Function = clingo_ast_attribute_e_clingo_ast_attribute_function as isize,
    Guard = clingo_ast_attribute_e_clingo_ast_attribute_guard as isize,
    Guards = clingo_ast_attribute_e_clingo_ast_attribute_guards as isize,
    Head = clingo_ast_attribute_e_clingo_ast_attribute_head as isize,
    IsDefault = clingo_ast_attribute_e_clingo_ast_attribute_is_default as isize,
    Left = clingo_ast_attribute_e_clingo_ast_attribute_left as isize,
    LeftGuard = clingo_ast_attribute_e_clingo_ast_attribute_left_guard as isize,
    Literal = clingo_ast_attribute_e_clingo_ast_attribute_literal as isize,
    Location = clingo_ast_attribute_e_clingo_ast_attribute_location as isize,
    Modifier = clingo_ast_attribute_e_clingo_ast_attribute_modifier as isize,
    Name = clingo_ast_attribute_e_clingo_ast_attribute_name as isize,
    NodeU = clingo_ast_attribute_e_clingo_ast_attribute_node_u as isize,
    NodeV = clingo_ast_attribute_e_clingo_ast_attribute_node_v as isize,
    OperatorName = clingo_ast_attribute_e_clingo_ast_attribute_operator_name as isize,
    OperatorType = clingo_ast_attribute_e_clingo_ast_attribute_operator_type as isize,
    Operators = clingo_ast_attribute_e_clingo_ast_attribute_operators as isize,
    Parameters = clingo_ast_attribute_e_clingo_ast_attribute_parameters as isize,
    Positive = clingo_ast_attribute_e_clingo_ast_attribute_positive as isize,
    Priority = clingo_ast_attribute_e_clingo_ast_attribute_priority as isize,
    Right = clingo_ast_attribute_e_clingo_ast_attribute_right as isize,
    RightGuard = clingo_ast_attribute_e_clingo_ast_attribute_right_guard as isize,
    SequenceType = clingo_ast_attribute_e_clingo_ast_attribute_sequence_type as isize,
    Sign = clingo_ast_attribute_e_clingo_ast_attribute_sign as isize,
    Symbol = clingo_ast_attribute_e_clingo_ast_attribute_symbol as isize,
    Term = clingo_ast_attribute_e_clingo_ast_attribute_term as isize,
    Terms = clingo_ast_attribute_e_clingo_ast_attribute_terms as isize,
    Value = clingo_ast_attribute_e_clingo_ast_attribute_value as isize,
    Variable = clingo_ast_attribute_e_clingo_ast_attribute_variable as isize,
    Weight = clingo_ast_attribute_e_clingo_ast_attribute_weight as isize,
    CommentType = clingo_ast_attribute_e_clingo_ast_attribute_comment_type as isize,
}

/// This struct provides a view to nodes in the AST.
#[derive(Debug)]
pub(crate) struct Ast<'a> {
    pub(crate) ptr: NonNull<clingo_ast_t>,
    pub(crate) _lifetime: PhantomData<&'a ()>,
}
impl<'a> Clone for Ast<'a> {
    fn clone(&self) -> Ast<'a> {
        self.deep_copy().unwrap()
    }
}
use std::fmt;
impl<'a> fmt::Display for Ast<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.to_string() {
            Ok(string) => write!(f, "{}", string),
            Err(_) => Err(fmt::Error),
        }
    }
}

impl<'a> PartialEq for Ast<'a> {
    /// Equality compare two AST nodes ignoring their locations.
    fn eq(&self, other: &Ast) -> bool {
        unsafe { clingo_ast_equal(self.ptr.as_ptr(), other.ptr.as_ptr()) }
    }
}
impl<'a> Eq for Ast<'a> {}
impl<'a> PartialOrd for Ast<'a> {
    fn partial_cmp(&self, other: &Ast<'a>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<'a> Ord for Ast<'a> {
    /// Compare two AST nodes ignoring their locations.
    fn cmp(&self, other: &Ast<'a>) -> Ordering {
        if unsafe { clingo_ast_less_than(self.ptr.as_ptr(), other.ptr.as_ptr()) } {
            return Ordering::Less;
        }
        if unsafe { clingo_ast_less_than(other.ptr.as_ptr(), self.ptr.as_ptr()) } {
            return Ordering::Greater;
        }
        Ordering::Equal
    }
}
impl<'a> Hash for Ast<'a> {
    /// Calculate a hash code of an AST node ignoring its location.
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe { clingo_ast_hash(self.ptr.as_ptr()) }.hash(state);
    }
}

impl<'a> Drop for Ast<'a> {
    fn drop(&mut self) {
        self.release()
    }
}
impl<'a> Ast<'a> {
    pub(crate) fn body(&self) -> Body<'_> {
        Body(ASTArray {
            ast: self,
            index: 0,
        })
    }
    pub(crate) fn head(&self) -> Head<'_> {
        let ast = self.get_attribute_ast(Attribute::Head).unwrap();
        Head { ast }
    }
    pub(crate) fn term(&self) -> Term<'_> {
        let ast = self.get_attribute_ast(Attribute::Term).unwrap();
        Term { ast }
    }
    pub(crate) fn set_term(&mut self, term: Term) {
        self.set_attribute_ast(Attribute::Term, &term.ast).unwrap();
    }
    pub(crate) fn guard(&self) -> TheoryGuard<'_> {
        // TODO: check
        let ast = self.get_attribute_ast(Attribute::Guard).unwrap();
        TheoryGuard { ast }
    }
    pub(crate) fn elements(&self) -> &[TheoryAtomElement<'_>] {
        unimplemented!()
        // let ast = self.get_attribute_ast(Attribute::Elements).unwrap();
        // &[]
    }
    pub(crate) fn left(&self) -> Term<'a> {
        let ast = self.get_attribute_ast(Attribute::Left).unwrap();
        Term { ast }
    }
    pub(crate) fn right(&self) -> Term<'a> {
        let ast = self.get_attribute_ast(Attribute::Right).unwrap();
        Term { ast }
    }

    /// Wrap an AST node whose reference is owned by the caller.
    pub(crate) fn from_ptr(ast: *mut clingo_ast_t) -> Result<Ast<'a>, ClingoError> {
        match NonNull::new(ast) {
            Some(ptr) => Ok(Ast {
                ptr,
                _lifetime: PhantomData,
            }),
            None => Err(ClingoError::FFIError {
                msg: "Tried creating NonNull from a null pointer.",
            }),
        }
    }

    /// Increment the reference count of an AST node.
    ///
    /// *Note:* All functions that return AST nodes already increment the reference count.
    /// The reference count of callback arguments is not incremented.
    pub(crate) fn acquire(&self) {
        unsafe { clingo_ast_acquire(self.ptr.as_ptr()) }
    }
    /// Decrement the reference count of an AST node.
    /// *Note:* The node is deleted if the reference count reaches zero.
    fn release(&self) {
        // println!("release");
        // println!("ast: {:?}", self);
        // println!("ast: {}", self.to_string().unwrap());
        unsafe { clingo_ast_release(self.ptr.as_ptr()) }
    }
    /// Create a shallow copy of an AST node.
    /// might set one of the following error codes:
    /// - ::clingo_error_bad_alloc
    pub(crate) fn copy(&self) -> Result<Ast<'a>, ClingoError> {
        let mut cpy = std::ptr::null_mut();
        if !unsafe { clingo_ast_copy(self.ptr.as_ptr(), &mut cpy) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_ast_copy() failed.",
            ));
        }
        Ast::from_ptr(cpy)
    }
    /// Create a deep copy of an AST node.
    /// might set one of the following error codes:
    /// - ::clingo_error_bad_alloc
    fn deep_copy(&self) -> Result<Ast<'a>, ClingoError> {
        let mut cpy = std::ptr::null_mut();
        if !unsafe { clingo_ast_deep_copy(self.ptr.as_ptr(), &mut cpy) } {
            eprintln!("Call to clingo_ast_deep_copy() failed");
            return Err(ClingoError::new_internal(
                "Call to clingo_ast_deep_copy() failed.",
            ));
        }
        match NonNull::new(cpy) {
            Some(cpy) => Ok(Ast {
                ptr: cpy,
                _lifetime: self._lifetime,
            }),
            None => Err(ClingoError::FFIError {
                msg: "Tried creating NonNull from a null pointer.",
            })?,
        }
    }

    /// Get the string representation of an AST node.
    /// might set one of the following error codes:
    /// - ::clingo_error_runtime
    pub(crate) fn to_string(&self) -> Result<String, ClingoError> {
        let mut size: usize = 0;
        if !unsafe { clingo_ast_to_string_size(self.ptr.as_ptr(), &mut size) } {
            eprintln!("Call to clingo_ast_to_string_size() failed");
            return Err(ClingoError::new_internal(
                "Call to clingo_ast_to_string_size() failed.",
            ));
        }
        let mut string = Vec::with_capacity(size);
        let string_ptr = string.as_mut_ptr();
        if !unsafe { clingo_ast_to_string(self.ptr.as_ptr(), string_ptr, size) } {
            eprintln!("Call to clingo_ast_to_string() failed");
            return Err(ClingoError::new_internal(
                "Call to clingo_ast_to_string() failed.",
            ));
        }
        let c_str: &CStr = unsafe { CStr::from_ptr(string_ptr) };
        let str_slice: &str = match c_str.to_str() {
            Ok(slice) => slice,
            Err(e) => {
                eprintln!("{:?}", e);
                return Err(ClingoError::new_internal("Call to c_str.to_str() failed."));
            }
        };
        Ok(str_slice.to_string())
    }

    /// Get the type of an AST node.
    /// might set one of the following error codes:
    /// - ::clingo_error_runtime
    pub(crate) fn get_type(&self) -> Result<NodeType, ClingoError> {
        let mut ast_type = 0;
        if !unsafe { clingo_ast_get_type(self.ptr.as_ptr(), &mut ast_type) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_ast_get_type() failed.",
            ));
        }
        NodeType::try_from(ast_type as u32)
    }

    /// Check if an AST node has the given attribute.
    ///
    /// might set one of the following error codes:
    /// - ::clingo_error_runtime
    pub(crate) fn has_attribute(&self, attribute: Attribute) -> Result<bool, ClingoError> {
        let mut has_attribute = false;
        if !unsafe {
            clingo_ast_has_attribute(self.ptr.as_ptr(), attribute as i32, &mut has_attribute)
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_ast_has_attribute() failed.",
            ));
        }
        Ok(has_attribute)
    }

    /// Get the type of the given attribute of an AST node.
    ///
    /// might set one of the following error codes:
    /// - ::clingo_error_runtime
    pub(crate) fn get_attribute_type(&self, attribute: Attribute) -> Result<AttributeType, ClingoError> {
        let mut attribute_type = 0;
        if !unsafe {
            clingo_ast_attribute_type(self.ptr.as_ptr(), attribute as i32, &mut attribute_type)
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_ast_attribute_type() failed.",
            ));
        }
        AttributeType::try_from(attribute_type as u32)
    }

    /// Get the value of an attribute of type AttributeType::Number
    ///
    /// might set one of the following error codes:
    /// - ::clingo_error_runtime
    pub(crate) fn get_attribute_number(&self, attribute: Attribute) -> Result<i32, ClingoError> {
        let mut value = 0;
        if !unsafe {
            clingo_ast_attribute_get_number(self.ptr.as_ptr(), attribute as i32, &mut value)
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_ast_attribute_get_number() failed.",
            ));
        }
        Ok(value)
    }

    /// Set the value of an attribute of type AttributeType::Number
    ///
    /// might set one of the following error codes:
    /// - ::clingo_error_runtime
    pub(crate) fn set_attribute_number(
        &self,
        attribute: Attribute,
        value: i32,
    ) -> Result<(), ClingoError> {
        if !unsafe { clingo_ast_attribute_set_number(self.ptr.as_ptr(), attribute as i32, value) }
        {
            return Err(ClingoError::new_internal(
                "Call to clingo_ast_attribute_set_number() failed.",
            ));
        }
        Ok(())
    }

    /// Get the value of an attribute of type AttributeType::String
    ///
    /// might set one of the following error codes:
    /// - ::clingo_error_runtime
    pub(crate) fn get_attribute_string(
        &self,
        attribute: Attribute,
    ) -> Result<&'static str, ClingoError> {
        let mut value = std::ptr::null();
        if !unsafe {
            clingo_ast_attribute_get_string(self.ptr.as_ptr(), attribute as i32, &mut value)
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_ast_attribute_get_string() failed.",
            ));
        }
        // strings in the AST are internalized and valid for the duration of the process
        Ok(unsafe { CStr::from_ptr(value) }.to_str()?)
    }

    pub(crate) fn value(&self) -> Result<&'static str, ClingoError> {
        self.get_attribute_string(Attribute::Value)
    }
    pub(crate) fn comment_type(&self) -> Result<i32, ClingoError> {
        self.get_attribute_number(Attribute::CommentType)
    }

    /// Get the value of an attribute of type AttributeType::Symbol
    ///
    /// might set one of the following error codes:
    /// - ::clingo_error_runtime
    pub(crate) fn get_attribute_symbol(&self, attribute: Attribute) -> Result<Symbol, ClingoError> {
        let mut sym = 0;
        if !unsafe {
            clingo_ast_attribute_get_symbol(self.ptr.as_ptr(), attribute as i32, &mut sym)
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_ast_attribute_get_symbol() failed.",
            ));
        }
        Ok(Symbol(sym))
    }

    /// Set the value of an attribute of type AttributeType::Symbol
    ///
    /// might set one of the following error codes:
    /// - ::clingo_error_runtime
    pub(crate) fn set_attribute_symbol(
        &self,
        attribute: Attribute,
        value: Symbol,
    ) -> Result<(), ClingoError> {
        if !unsafe {
            clingo_ast_attribute_set_symbol(self.ptr.as_ptr(), attribute as i32, value.0)
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_ast_attribute_set_symbol() failed.",
            ));
        }
        Ok(())
    }

    pub(crate) fn location(&self) -> Result<Location, ClingoError> {
        self.get_attribute_location(Attribute::Location)
    }

    /// Get the value of an attribute of type AttributeType::Location
    ///
    /// might set one of the following error codes:
    /// - ::clingo_error_runtime
    pub(crate) fn get_attribute_location(
        &self,
        attribute: Attribute,
    ) -> Result<Location, ClingoError> {
        let mut loc: clingo_location = Location::default().0;
        if !unsafe {
            clingo_ast_attribute_get_location(self.ptr.as_ptr(), attribute as i32, &mut loc)
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_ast_attribute_get_location() failed.",
            ));
        }
        Ok(Location(loc))
    }

    /// Set the value of an attribute of type AttributeType::Location
    ///
    /// might set one of the following error codes:
    /// - ::clingo_error_runtime
    pub(crate) fn set_attribute_location(
        &self,
        attribute: Attribute,
        value: &Location,
    ) -> Result<(), ClingoError> {
        if !unsafe {
            clingo_ast_attribute_set_location(self.ptr.as_ptr(), attribute as i32, &value.0)
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_ast_attribute_set_location() failed.",
            ));
        }
        Ok(())
    }

    /// Set the value of an attribute of type AttributeType::String
    ///
    /// might set one of the following error codes:
    /// - ::clingo_error_runtime
    pub(crate) fn set_attribute_string(
        &self,
        attribute: Attribute,
        value: &str,
    ) -> Result<(), ClingoError> {
        let value = CString::new(value)?;
        if !unsafe {
            clingo_ast_attribute_set_string(self.ptr.as_ptr(), attribute as i32, value.as_ptr())
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_ast_attribute_set_string() failed.",
            ));
        }
        Ok(())
    }

    pub(crate) fn name(&self) -> Result<String, ClingoError> {
        let attribute = Attribute::Name;
        let ast_attribute_type = self.get_attribute_type(attribute).unwrap();
        dbg!(ast_attribute_type);
        self.to_string()
    }

    /// Get the value of an attribute of type AttributeType::Ast
    ///
    /// might set one of the following error codes:
    /// - ::clingo_error_runtime
    pub(crate) fn get_attribute_ast(&self, attribute: Attribute) -> Result<Ast<'a>, ClingoError> {
        let mut ast = std::ptr::null_mut();
        if !unsafe { clingo_ast_attribute_get_ast(self.ptr.as_ptr(), attribute as i32, &mut ast) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_ast_attribute_get_ast() failed.",
            ));
        }
        Ast::from_ptr(ast)
    }

    /// Set the value of an attribute of type AttributeType::Ast
    ///
    /// might set one of the following error codes:
    /// - ::clingo_error_runtime
    pub(crate) fn set_attribute_ast(&self, attribute: Attribute, value: &Ast) -> Result<(), ClingoError> {
        if !unsafe {
            clingo_ast_attribute_set_ast(self.ptr.as_ptr(), attribute as i32, value.ptr.as_ptr())
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_ast_attribute_set_ast() failed.",
            ));
        }
        Ok(())
    }

    /// Get the value of an attribute of type AttributeType::OptionalAst
    ///
    /// might set one of the following error codes:
    /// - ::clingo_error_runtime
    pub(crate) fn get_attribute_optional_ast(
        &self,
        attribute: Attribute,
    ) -> Result<Option<Ast<'a>>, ClingoError> {
        let mut ast = std::ptr::null_mut();
        if !unsafe {
            clingo_ast_attribute_get_optional_ast(self.ptr.as_ptr(), attribute as i32, &mut ast)
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_ast_attribute_get_optional_ast() failed.",
            ));
        }
        if ast.is_null() {
            return Ok(None);
        }
        Ast::from_ptr(ast).map(Some)
    }

    /// Set the value of an attribute of type AttributeType::OptionalAst
    ///
    /// might set one of the following error codes:
    /// - ::clingo_error_runtime
    pub(crate) fn set_attribute_optional_ast(
        &self,
        attribute: Attribute,
        value: Option<&Ast>,
    ) -> Result<(), ClingoError> {
        let value = match value {
            Some(ast) => ast.ptr.as_ptr(),
            None => std::ptr::null_mut(),
        };
        if !unsafe {
            clingo_ast_attribute_set_optional_ast(self.ptr.as_ptr(), attribute as i32, value)
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_ast_attribute_set_optional_ast() failed.",
            ));
        }
        Ok(())
    }

    /// Get the size of an attribute of type AttributeType::StringArray
    ///
    /// might set one of the following error codes:
    /// - ::clingo_error_runtime
    pub(crate) fn size_attribute_string_array(
        &self,
        attribute: Attribute,
    ) -> Result<usize, ClingoError> {
        let mut size = 0;
        if !unsafe {
            clingo_ast_attribute_size_string_array(self.ptr.as_ptr(), attribute as i32, &mut size)
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_ast_attribute_size_string_array() failed.",
            ));
        }
        Ok(size)
    }

    /// Get the value of an attribute of type AttributeType::StringArray at the given index.
    ///
    /// might set one of the following error codes:
    /// - ::clingo_error_runtime
    pub(crate) fn get_attribute_string_at(
        &self,
        attribute: Attribute,
        index: usize,
    ) -> Result<&'static str, ClingoError> {
        let mut value = std::ptr::null();
        if !unsafe {
            clingo_ast_attribute_get_string_at(
                self.ptr.as_ptr(),
                attribute as i32,
                index,
                &mut value,
            )
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_ast_attribute_get_string_at() failed.",
            ));
        }
        // strings in the AST are internalized and valid for the duration of the process
        Ok(unsafe { CStr::from_ptr(value) }.to_str()?)
    }

    /// Set the value of an attribute of type AttributeType::StringArray at the given index.
    ///
    /// might set one of the following error codes:
    /// - ::clingo_error_runtime
    /// - ::clingo_error_bad_alloc
    pub(crate) fn set_attribute_string_at(
        &self,
        attribute: Attribute,
        index: usize,
        value: &str,
    ) -> Result<(), ClingoError> {
        // clingo does not check the index
        check_index(index, self.size_attribute_string_array(attribute)?)?;
        let value = CString::new(value)?;
        if !unsafe {
            clingo_ast_attribute_set_string_at(
                self.ptr.as_ptr(),
                attribute as i32,
                index,
                value.as_ptr(),
            )
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_ast_attribute_set_string_at() failed.",
            ));
        }
        Ok(())
    }

    /// Insert a value into an attribute of type AttributeType::StringArray at the given index.
    ///
    /// might set one of the following error codes:
    /// - ::clingo_error_runtime
    /// - ::clingo_error_bad_alloc
    pub(crate) fn insert_attribute_string_at(
        &self,
        attribute: Attribute,
        index: usize,
        value: &str,
    ) -> Result<(), ClingoError> {
        // clingo does not check the index
        check_index(index, self.size_attribute_string_array(attribute)? + 1)?;
        let value = CString::new(value)?;
        if !unsafe {
            clingo_ast_attribute_insert_string_at(
                self.ptr.as_ptr(),
                attribute as i32,
                index,
                value.as_ptr(),
            )
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_ast_attribute_insert_string_at() failed.",
            ));
        }
        Ok(())
    }

    /// Remove an element from an attribute of type AttributeType::StringArray at the given index.
    ///
    /// might set one of the following error codes:
    /// - ::clingo_error_runtime
    pub(crate) fn delete_attribute_string_at(
        &self,
        attribute: Attribute,
        index: usize,
    ) -> Result<(), ClingoError> {
        // clingo does not check the index
        check_index(index, self.size_attribute_string_array(attribute)?)?;
        if !unsafe {
            clingo_ast_attribute_delete_string_at(self.ptr.as_ptr(), attribute as i32, index)
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_ast_attribute_delete_string_at() failed.",
            ));
        }
        Ok(())
    }

    /// Get the size of an attribute of type AttributeType::AstArray
    ///
    /// might set one of the following error codes:
    /// - ::clingo_error_runtime
    pub(crate) fn size_attribute_ast_array(&self, attribute: Attribute) -> Result<usize, ClingoError> {
        let mut size = 0;
        if !unsafe {
            clingo_ast_attribute_size_ast_array(self.ptr.as_ptr(), attribute as i32, &mut size)
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_ast_attribute_size_ast_array() failed.",
            ));
        }
        Ok(size)
    }

    /// Get the value of an attribute of type AttributeType::AstArray at the given index.
    ///
    /// might set one of the following error codes:
    /// - ::clingo_error_runtime
    pub(crate) fn get_attribute_ast_at(
        &self,
        attribute: Attribute,
        index: usize,
    ) -> Result<Ast<'a>, ClingoError> {
        let mut ast = std::ptr::null_mut();
        if !unsafe {
            clingo_ast_attribute_get_ast_at(self.ptr.as_ptr(), attribute as i32, index, &mut ast)
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_ast_attribute_get_ast_at() failed.",
            ));
        }
        Ast::from_ptr(ast)
    }

    /// Set the value of an attribute of type AttributeType::AstArray at the given index.
    ///
    /// might set one of the following error codes:
    /// - ::clingo_error_runtime
    /// - ::clingo_error_bad_alloc
    pub(crate) fn set_attribute_ast_at(
        &self,
        attribute: Attribute,
        index: usize,
        value: &Ast,
    ) -> Result<(), ClingoError> {
        // clingo does not check the index
        check_index(index, self.size_attribute_ast_array(attribute)?)?;
        if !unsafe {
            clingo_ast_attribute_set_ast_at(
                self.ptr.as_ptr(),
                attribute as i32,
                index,
                value.ptr.as_ptr(),
            )
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_ast_attribute_set_ast_at() failed.",
            ));
        }
        Ok(())
    }

    /// Insert a value into an attribute of type AttributeType::AstArray at the given index.
    ///
    /// might set one of the following error codes:
    /// - ::clingo_error_runtime
    /// - ::clingo_error_bad_alloc
    pub(crate) fn insert_attribute_ast_at(
        &self,
        attribute: Attribute,
        index: usize,
        value: &Ast,
    ) -> Result<(), ClingoError> {
        // clingo does not check the index
        check_index(index, self.size_attribute_ast_array(attribute)? + 1)?;
        if !unsafe {
            clingo_ast_attribute_insert_ast_at(
                self.ptr.as_ptr(),
                attribute as i32,
                index,
                value.ptr.as_ptr(),
            )
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_ast_attribute_insert_ast_at() failed.",
            ));
        }
        Ok(())
    }

    /// Remove an element from an attribute of type AttributeType::AstArray at the given index.
    ///
    /// might set one of the following error codes:
    /// - ::clingo_error_runtime
    pub(crate) fn delete_attribute_ast_at(
        &self,
        attribute: Attribute,
        index: usize,
    ) -> Result<(), ClingoError> {
        // clingo does not check the index
        check_index(index, self.size_attribute_ast_array(attribute)?)?;
        if !unsafe {
            clingo_ast_attribute_delete_ast_at(self.ptr.as_ptr(), attribute as i32, index)
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_ast_attribute_delete_ast_at() failed.",
            ));
        }
        Ok(())
    }
}

impl<'a> Ast<'a> {
    /// Unpool the AST node.
    ///
    /// might set one of the following error codes:
    /// - ::clingo_error_bad_alloc
    pub(crate) fn unpool(&self, unpool_type: UnpoolType) -> Result<Vec<Ast<'a>>, ClingoError> {
        let mut asts: Vec<Ast<'a>> = vec![];
        if !unsafe {
            clingo_ast_unpool(
                self.ptr.as_ptr(),
                unpool_type as clingo_ast_unpool_type_bitset_t,
                Some(unsafe_unpool_callback),
                &mut asts as *mut Vec<Ast<'a>> as *mut c_void,
            )
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_ast_unpool() failed.",
            ));
        }
        Ok(asts)
    }
}
unsafe extern "C" fn unsafe_unpool_callback(ast: *mut clingo_ast_t, data: *mut c_void) -> bool {
    // check for null pointers
    if ast.is_null() | data.is_null() {
        set_internal_error(
            ErrorType::Runtime,
            "unsafe_unpool_callback() got a null pointer.",
        );
        return false;
    }
    let asts = &mut *(data as *mut Vec<Ast>);
    // the reference count of callback arguments is not incremented
    clingo_ast_acquire(ast);
    match Ast::from_ptr(ast) {
        Ok(ast) => {
            asts.push(ast);
            true
        }
        Err(_) => false,
    }
}

/// Report an index that is out of range like clingo does for the attribute getters.
fn check_index(index: usize, size: usize) -> Result<(), ClingoError> {
    if index < size {
        return Ok(());
    }
    set_internal_error(ErrorType::Runtime, "index out of range");
    Err(ClingoError::new_internal("AST attribute index out of range."))
}

/// Enum to configure unpooling.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum UnpoolType {
    /// To only unpool conditions of conditional literals.
    Condition = clingo_ast_unpool_type_e_clingo_ast_unpool_type_condition as isize,
    /// To unpool everything except conditions of conditional literals.
    Other = clingo_ast_unpool_type_e_clingo_ast_unpool_type_other as isize,
    /// To unpool everything.
    All = clingo_ast_unpool_type_e_clingo_ast_unpool_type_all as isize,
}

