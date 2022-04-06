// ANCHOR: here
#[derive(Debug, PartialEq)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        match user_preference {
            Some(pref) => pref,
            None => self.most_stocked(),
        }
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}
// ANCHOR_END: here

#[test]
fn testme() {
    let store = Inventory { shirts: vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Blue]};

    assert_eq!(store.giveaway(Some(ShirtColor::Red)), ShirtColor::Red);
    assert_eq!(store.giveaway(Some(ShirtColor::Blue)), ShirtColor::Blue);
    assert_eq!(store.giveaway(None), ShirtColor::Blue);

}

fn main() {}
