pub use input::input_lines;
pub use input::parse_input;
pub use input::split_input;
pub use run::run;

mod debug;
mod input;
mod instant;
mod run;

#[cfg(test)]
mod tests {
    #[test]
    fn test_debug() {
        let a = "hola";
        debug!(a, "hola2");
    }
}
