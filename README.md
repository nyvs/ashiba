# ashiba
[![Latest Release][crates-io-badge]][crates-io-url]
[![Documentation][docs-rs-img]][docs-rs-url]
[![Dependencies][deps]][github]

tui framework, based on tui-rs
(not ready for production)

## Features
- easy to use.
- architectural structure through `View` and `Controller` traits.
- QOL components, clickable context-menus, popups, forms and more.

Components in ashiba shall not implement `View` and `Controller` traits.
They merely provide a model to cover basic functionality. 
Components can then be used in widgets through composition to fulfil the usecase.

## Quick Introduction
It's as easy as this:
```rust
fn main() {
    ashiba::app::run(AshibaApp::default()).unwrap();
}
```
where `AshibaApp` implements `View` and `Controller`.
The `AshibaApp` struct acts as a model and could look something like this:
```rust
#[derive(Default)]
pub struct AshibaApp {
    button: BasicButton,
    counter: u32,
    should_quit: bool,
}
```
A `Controller` makes it possible to define actions for your App or Widget:
```rust
impl Controller for AshibaApp {
    fn handle_mouse(&mut self, ev: MouseEvent) {
        self.button.on_click(ev, || {
            self.counter += 1;
        })
    }

    fn handle_key(&mut self, ev: KeyEvent) {
        if let KeyCode::Char('q') = ev.code {
            self.should_quit = true;
        }
    }

    fn should_quit(&self) -> bool {
        self.should_quit
    }
}
```
A `View` gives freedom to style your App or Widget:
```rust
impl View for AshibaApp {
    fn ui(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>, area: Rect) {
        let button = BasicPlacement::compute_area_relative(area, (50, 50), (20, 4));
        let block = Block::default().borders(Borders::ALL);
        let ok_block = Block::default().borders(Borders::ALL);
        let ok_button = 
        Paragraph::new(Span::from(format!("Clicked {} times", self.counter)))
            .block(ok_block)
            .alignment(Alignment::Center);
        self.button.set_area(button);
        f.render_widget(block, area);
        f.render_widget(ok_button, button);
    }
}
```
In ashiba, you should make use of a hierachical pattern, 
creating widgets of your own to manage the growing complexity of your app.

## Contributions
Feel free to open an issue/PR explaining possible improvements or changes.

## Help
Also, please do not hesitate and open an issue when needed. I am happy to help!

[deps]: https://img.shields.io/librariesio/github/nyvs/ashiba
[github]: https://github.com/nyvs/ashiba
[crates-io-badge]: https://img.shields.io/crates/v/ashiba.svg
[crates-io-url]: https://crates.io/crates/ashiba
[docs-rs-img]: https://docs.rs/ashiba/badge.svg
[docs-rs-url]: https://docs.rs/ashiba
