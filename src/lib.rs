//! Permute is a framework and a DSL to describe data transformations.
//! It is designed to be used in a data processing pipeline,
//! where data is transformed from one form to another.
//! Configuration files are used to describe the data, formats, transformations, validation
//! and other aspects of the data processing.
//! 
//! # File Kinds
//! There are several kinds of files that can be used in the framework:
//! 
//! ## Main File
//! The YAML main file is the entry point of the project. It describes the project, its name,
//! and defines the purpose of the program by creating a pipeline of data transformations.
//! Pipeline is complete when there is a source and a sink defined in the main file.
//! There can be several sources and sinks defined with complex flows of data.
//! 
//! ## Source & Sink Files
//! YAML source and sink file definition. These are two kinds of files that have similar
//! structure.
//! Source file describes the data source, and sink file describes the destination. 
//! In both cases it can be a file, a database, a service, or any other
//! source/sink for data. It describes the format of the data, and allows to define the schema
//! of the data, and also to validate the data. Source itself then is implemented externally
//! to the framework, and the framework only uses this definition as a contract (interface).
//! It does validate the data against the schema on runtime as to detect any errors in the
//! implementation.
//! 
//! ## Logics File
//! Logic files use Rust-like language to describe traits and functions that can be used
//! in the project. It allows to define custom functions and types that can be used
//! in the data transformations. It allows to define a function that will calculate
//! a value, or transform the data in any way. It also allows to define types that
//! will be used in the data processing.
//! 
//! ### Loops Nonexistent!
//! The language of function code is similar to Rust. However, it is not possible to
//! use any kind of loops directly. Though loop-like functionality can be defined
//! in external code. It was decided to avoid loops as to make the code more predictable
//! and linear by time complexity. Thus it is not possible to make an infinite loop
//! or a loop that will take a long time to execute in the language itself. Though it also
//! assumes that backend implementation of external functions is sane as not to allow to
//! do any harmful or infinite operations.
//! 
//! Recursion is also forbidden in the language as compiler will statically analyze the
//! execution paths and would reject the code if it detects a loop of function calls,
//! either direct from within the same function, or indirect through the chain
//! of function calls. It would not be able to detect a loop if it is defined in the
//! external code, so it is up to the developer to ensure that the code is safe on the
//! backend side.
//! 
//! ### External Code
//! Functions and types can be declared in the YAML file, but the actual implementation
//! can be delegated to the external code. This allows to use any kind of code, including
//! loops, recursion, and other constructs that are not allowed in the YAML file.
//! It also allows to use any library. It allows to write backend code that is
//! not possible to implement in YAML or otherwise is unwanted to be exposed to the
//! user.
//! 
//! ### Borrow & Copy Semantics
//! All values carried by the functions as arguments are borrowed Copy-on-Write. This means that
//! the value is copied only when modification on it is attempted.
//! By default, it is a reference,
//! though it is not directly specifiable in the code as there is no way to
//! define Rust-y references (or pointers). If the argument is declared as mutable,
//! then the value is modified in place and the caller would see the modified value after
//! the function's execution (aka mutable reference). All values passed to types (structures)
//! are copied and then the structure is the owner of the value with no connection to 
//! the original source value. It is not possible to make a structure to borrow the value.
//! 
//! If the function code creates a new binding to the argument of the function, then the
//! value is copied. It is copied in both cases, whether the argument is mutable or not.
//! Then the original value is still accessible (unless the name is shadowed). Unlike in Rust.
//! 
//! It was decided to use this approach to avoid the complexity of the ownership system as it
//! seemed unnecessary for the framework. We won't need to describe complex inter-object
//! references for the data processing and transformations.

