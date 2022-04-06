// ANCHOR: here
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.last_stocked())
    }

    fn last_stocked(&self) -> ShirtColor {
        *self.shirts.last().expect("need to have shirts to do a giveaway")
    }
}
// ANCHOR_END: here

#[test]
fn testme() {
    let store = Inventory {
        shirts: vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Blue],
    };

    assert_eq!(store.giveaway(Some(ShirtColor::Red)), ShirtColor::Red);
    assert_eq!(store.giveaway(Some(ShirtColor::Blue)), ShirtColor::Blue);
    assert_eq!(store.giveaway(None), ShirtColor::Blue);
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum DisplayMode {
    Light,
    Dark,
}

struct User {
    ui_preference: Option<DisplayMode>,
}

enum Time {
    Day,
    Night,
}

impl User {
    fn current_time(&self) -> Time {
        Time::Night
    }

    fn display_mode(&self) -> DisplayMode {
        self.ui_preference.unwrap_or_else(|| {
            match self.current_time() {
                Time::Day => DisplayMode::Light,
                Time::Night => DisplayMode::Dark,
            }
        })
    }
}

#[test]
fn other_test() {
    let u = User { ui_preference: Some(DisplayMode::Light) };
    assert_eq!(u.display_mode(), DisplayMode::Light);

    let u = User { ui_preference: Some(DisplayMode::Dark) };
    assert_eq!(u.display_mode(), DisplayMode::Dark);

    let u = User { ui_preference: None };
    assert_eq!(u.display_mode(), DisplayMode::Dark);

}

fn main() {}
