// Rust test file autogenerated with cargo build (build/spectests.rs).
// Please do NOT modify it by hand, as it will be reset on next build.
// Test based on spectests/start.wast
#![allow(
    warnings,
    dead_code
)]
use std::{f32, f64};
use wabt::wat2wasm;

use wasmer_clif_backend::CraneliftCompiler;
use wasmer_runtime::types::Value;
use wasmer_runtime::{module::Module, Instance};

use crate::spectests::_common::{generate_imports, NaNCheck};

// Line 2
#[test]
fn c0_l2_assert_invalid() {
    let wasm_binary = [
        0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 8, 1, 1, 10, 4, 1, 2, 0, 11,
    ];
    let module = wasmer_runtime::compile(&wasm_binary, &CraneliftCompiler::new());
    assert!(module.is_err(), "WASM should not compile as is invalid");
}

// Line 7
#[test]
fn c1_l7_assert_invalid() {
    let wasm_binary = [
        0, 97, 115, 109, 1, 0, 0, 0, 1, 5, 1, 96, 0, 1, 127, 3, 2, 1, 0, 8, 1, 0, 10, 7, 1, 5, 0,
        65, 0, 15, 11,
    ];
    let module = wasmer_runtime::compile(&wasm_binary, &CraneliftCompiler::new());
    assert!(module.is_err(), "WASM should not compile as is invalid");
}

// Line 14
#[test]
fn c2_l14_assert_invalid() {
    let wasm_binary = [
        0, 97, 115, 109, 1, 0, 0, 0, 1, 5, 1, 96, 1, 127, 0, 3, 2, 1, 0, 8, 1, 0, 10, 4, 1, 2, 0,
        11,
    ];
    let module = wasmer_runtime::compile(&wasm_binary, &CraneliftCompiler::new());
    assert!(module.is_err(), "WASM should not compile as is invalid");
}

// Line 21
fn create_module_1() -> Box<Instance> {
    let module_str = "(module
      (type (;0;) (func))
      (type (;1;) (func (result i32)))
      (func (;0;) (type 0)
        i32.const 0
        i32.const 0
        i32.load8_u
        i32.const 1
        i32.add
        i32.store8)
      (func (;1;) (type 1) (result i32)
        i32.const 0
        i32.load8_u
        return)
      (func (;2;) (type 0)
        call 0
        call 0
        call 0)
      (memory (;0;) 1 1)
      (export \"inc\" (func 0))
      (export \"get\" (func 1))
      (start 2)
      (data (;0;) (i32.const 0) \"A\"))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    let module = wasmer_runtime::compile(&wasm_binary[..], &CraneliftCompiler::new())
        .expect("WASM can't be compiled");
    module
        .instantiate(generate_imports())
        .expect("WASM can't be instantiated")
}

fn start_module_1(instance: &mut Instance) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //instance.start();
}

// Line 45
fn c4_l45_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c4_l45_action_invoke");
    let result = instance.call("get", &[]);
    assert_eq!(result, Ok(Some(Value::I32(68 as i32))));
    result.map(|_| ())
}

// Line 46
fn c5_l46_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c5_l46_action_invoke");
    let result = instance.call("inc", &[]);

    result.map(|_| ())
}

// Line 47
fn c6_l47_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c6_l47_action_invoke");
    let result = instance.call("get", &[]);
    assert_eq!(result, Ok(Some(Value::I32(69 as i32))));
    result.map(|_| ())
}

// Line 48
fn c7_l48_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c7_l48_action_invoke");
    let result = instance.call("inc", &[]);

    result.map(|_| ())
}

// Line 49
fn c8_l49_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c8_l49_action_invoke");
    let result = instance.call("get", &[]);
    assert_eq!(result, Ok(Some(Value::I32(70 as i32))));
    result.map(|_| ())
}

// Line 51

#[test]
fn test_module_1() {
    let mut instance = create_module_1();
    // We group the calls together
    start_module_1(&mut instance);
    c4_l45_action_invoke(&mut instance);
    c5_l46_action_invoke(&mut instance);
    c6_l47_action_invoke(&mut instance);
    c7_l48_action_invoke(&mut instance);
    c8_l49_action_invoke(&mut instance);
}
fn create_module_2() -> Box<Instance> {
    let module_str = "(module
      (type (;0;) (func))
      (type (;1;) (func (result i32)))
      (func (;0;) (type 0)
        i32.const 0
        i32.const 0
        i32.load8_u
        i32.const 1
        i32.add
        i32.store8)
      (func (;1;) (type 1) (result i32)
        i32.const 0
        i32.load8_u
        return)
      (func (;2;) (type 0)
        call 0
        call 0
        call 0)
      (memory (;0;) 1 1)
      (export \"inc\" (func 0))
      (export \"get\" (func 1))
      (start 2)
      (data (;0;) (i32.const 0) \"A\"))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    let module = wasmer_runtime::compile(&wasm_binary[..], &CraneliftCompiler::new())
        .expect("WASM can't be compiled");
    module
        .instantiate(generate_imports())
        .expect("WASM can't be instantiated")
}

fn start_module_2(instance: &mut Instance) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //instance.start();
}

// Line 74
fn c10_l74_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c10_l74_action_invoke");
    let result = instance.call("get", &[]);
    assert_eq!(result, Ok(Some(Value::I32(68 as i32))));
    result.map(|_| ())
}

// Line 75
fn c11_l75_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c11_l75_action_invoke");
    let result = instance.call("inc", &[]);

    result.map(|_| ())
}

// Line 76
fn c12_l76_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c12_l76_action_invoke");
    let result = instance.call("get", &[]);
    assert_eq!(result, Ok(Some(Value::I32(69 as i32))));
    result.map(|_| ())
}

// Line 77
fn c13_l77_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c13_l77_action_invoke");
    let result = instance.call("inc", &[]);

    result.map(|_| ())
}

// Line 78
fn c14_l78_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c14_l78_action_invoke");
    let result = instance.call("get", &[]);
    assert_eq!(result, Ok(Some(Value::I32(70 as i32))));
    result.map(|_| ())
}

// Line 80

#[test]
fn test_module_2() {
    let mut instance = create_module_2();
    // We group the calls together
    start_module_2(&mut instance);
    c10_l74_action_invoke(&mut instance);
    c11_l75_action_invoke(&mut instance);
    c12_l76_action_invoke(&mut instance);
    c13_l77_action_invoke(&mut instance);
    c14_l78_action_invoke(&mut instance);
}
fn create_module_3() -> Box<Instance> {
    let module_str = "(module
      (type (;0;) (func (param i32)))
      (type (;1;) (func))
      (import \"spectest\" \"print_i32\" (func (;0;) (type 0)))
      (func (;1;) (type 1)
        i32.const 1
        call 0)
      (start 1))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    let module = wasmer_runtime::compile(&wasm_binary[..], &CraneliftCompiler::new())
        .expect("WASM can't be compiled");
    module
        .instantiate(generate_imports())
        .expect("WASM can't be instantiated")
}

fn start_module_3(instance: &mut Instance) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //instance.start();
}

// Line 86

#[test]
fn test_module_3() {
    let mut instance = create_module_3();
    // We group the calls together
    start_module_3(&mut instance);
}
fn create_module_4() -> Box<Instance> {
    let module_str = "(module
      (type (;0;) (func (param i32)))
      (type (;1;) (func))
      (import \"spectest\" \"print_i32\" (func (;0;) (type 0)))
      (func (;1;) (type 1)
        i32.const 2
        call 0)
      (start 1))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    let module = wasmer_runtime::compile(&wasm_binary[..], &CraneliftCompiler::new())
        .expect("WASM can't be compiled");
    module
        .instantiate(generate_imports())
        .expect("WASM can't be instantiated")
}

fn start_module_4(instance: &mut Instance) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //instance.start();
}

// Line 92

#[test]
fn test_module_4() {
    let mut instance = create_module_4();
    // We group the calls together
    start_module_4(&mut instance);
}
fn create_module_5() -> Box<Instance> {
    let module_str = "(module
      (type (;0;) (func))
      (import \"spectest\" \"print\" (func (;0;) (type 0)))
      (start 0))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    let module = wasmer_runtime::compile(&wasm_binary[..], &CraneliftCompiler::new())
        .expect("WASM can't be compiled");
    module
        .instantiate(generate_imports())
        .expect("WASM can't be instantiated")
}

fn start_module_5(instance: &mut Instance) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //instance.start();
}

// Line 98

#[test]
fn test_module_5() {
    let mut instance = create_module_5();
    // We group the calls together
    start_module_5(&mut instance);
}
