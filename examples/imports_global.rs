//! A Wasm module can import entities, like functions, memories,
//! globals and tables.
//!
//! This example illustrates how to use imported globals. They come
//! in 2 flavors:
//!
//!   1. Immutable globals (const),
//!   2. Mutable globals.
//!
//! You can run the example directly by executing in Wasmer root:
//!
//! ```shell
//! cargo run --example imported-global --release --features "cranelift"
//! ```
//!
//! Ready?

use wasmer::{imports, wat2wasm, Context, Global, Instance, Module, Store, TypedFunction, Value};
use wasmer_compiler::Universal;
use wasmer_compiler_cranelift::Cranelift;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Let's declare the Wasm module with the text representation.
    let wasm_bytes = wat2wasm(
        br#"
(module
  (global $some (import "env" "some") f32)
  (global $other (import "env" "other") (mut f32))

  (func (export "get_some") (result f32) (global.get $some))
  (func (export "get_other") (result f32) (global.get $other))

  (func (export "set_other") (param f32) (global.set $other (local.get 0))))
"#,
    )?;

    // Create a Store.
    // Note that we don't need to specify the engine/compiler if we want to use
    // the default provided by Wasmer.
    // You can use `Store::default()` for that.
    let store = Store::new_with_engine(&Universal::new(Cranelift::default()).engine());
    let mut ctx = Context::new(&store, (), ());

    println!("Compiling module...");
    // Let's compile the Wasm module.
    let module = Module::new(&store, wasm_bytes)?;

    // Create the globals
    let some = Global::new(&mut ctx, Value::F32(1.0));
    let other = Global::new_mut(&mut ctx, Value::F32(2.0));

    // Create an import object.
    // We add the two required globals in the `env` namespace.
    let import_object = imports! {
        "env" => {
            "some" => some.clone(),
            "other" => other.clone(),
        }
    };

    println!("Instantiating module...");
    // Let's instantiate the Wasm module.
    let instance = Instance::new(&mut ctx, &module, &import_object)?;

    // Here we go.
    //
    // The Wasm module only imports some globals. We'll have to interact
    // with them either using the Global API or exported functions.
    let get_some: TypedFunction<(), f32> = instance
        .exports
        .get_function("get_some")?
        .native(&mut ctx)?;
    let get_other: TypedFunction<(), f32> = instance
        .exports
        .get_function("get_other")?
        .native(&mut ctx)?;

    let some_result = get_some.call(&mut ctx)?;
    let other_result = get_other.call(&mut ctx)?;

    println!("some value (via `get_some`): {:?}", some_result);
    println!("some value (via Global API): {:?}", some.get(&mut ctx));
    println!("other value (via `get_other`): {:?}", other_result);
    println!("other value (via Global API): {:?}", other.get(&mut ctx));

    assert_eq!(some_result, some.get(&mut ctx).f32().unwrap());
    assert_eq!(other_result, other.get(&mut ctx).f32().unwrap());

    println!("Setting global values...");
    // Trying to set the value of a immutable global (`const`)
    // will result in a `RuntimeError`.
    let result = some.set(&mut ctx, Value::F32(42.0));
    assert_eq!(
        result.expect_err("Expected an error").message(),
        "Attempted to set an immutable global"
    );

    other.set(&mut ctx, Value::F32(21.0))?;
    let other_result = other.get(&mut ctx);
    println!("other value after `set`: {:?}", other_result);
    assert_eq!(other_result, Value::F32(21.0));

    println!("Altering global values through exported functions...");
    // Changes made to global through exported functions will
    // be reflected on the host side.
    let set_other: TypedFunction<f32, ()> = instance
        .exports
        .get_function("set_other")?
        .native(&mut ctx)?;
    set_other.call(&mut ctx, 42.0)?;

    println!("other value (via Global API): {:?}", other.get(&mut ctx));
    assert_eq!(other.get(&mut ctx), Value::F32(42.0));

    Ok(())
}

#[test]
fn test_imported_global() -> Result<(), Box<dyn std::error::Error>> {
    main()
}
