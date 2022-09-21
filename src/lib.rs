pub mod session;
pub mod ui;
pub use session::*;
pub use ui::render::*;
pub use ui::update::*;
pub use render::html;
pub use render::raw as raw_html;
pub extern crate shiny_rs_derive;
pub mod types;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
