/// A value in the virtual machine.
/// By using an enum, it's simple to define the different primitive types
/// supported by the language.
///
/// A Value cannot implement the `Copy` trait, as it can contain
/// heap-allocated data, such as strings, functions, or objects.
#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
}
