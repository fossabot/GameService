mod slot_machine;
pub use self::slot_machine::slot_machine;

#[cfg(any(test, bench))]
mod tests;
