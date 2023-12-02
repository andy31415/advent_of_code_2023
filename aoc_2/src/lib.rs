#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Reveal {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Bag {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl Bag {
    pub fn reveal_possible(&self, reveal: &Reveal) -> bool {
        (self.red >= reveal.red) && (self.green >= reveal.green) && (self.blue >= reveal.blue)
    }

    pub fn power(&self) -> u32 {
        self.red * self.blue * self.green
    }

    pub fn increase_to_allow_reveal(&mut self, r: &Reveal) {
        if self.red < r.red {
            self.red = r.red
        }
        if self.green < r.green {
            self.green = r.green
        }
        if self.blue < r.blue {
            self.blue = r.blue
        }
    }
}

impl From<&str> for Reveal {
    fn from(value: &str) -> Self {
        let mut result = Self::default();
        // Format: comma separated values
        for entry in value.split(',') {
            match entry.trim().split_once(' ') {
                Some((number, color)) => match color {
                    "red" => result.red = number.parse().unwrap(),
                    "green" => result.green = number.parse().unwrap(),
                    "blue" => result.blue = number.parse().unwrap(),
                    _ => panic!(
                        "Bad color {:?}: {:?} extracted from {:?}",
                        color, entry, value
                    ),
                },
                None => panic!("Bad data: {:?} extracted from {:?}", entry, value),
            }
        }

        result
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Game {
    pub id: u32,
    pub reveals: Vec<Reveal>,
}

impl Game {
    pub fn parse(s: impl AsRef<str>) -> Option<Game> {
        let s = s.as_ref();

        // FORMAT: "Game ...: semicolon-separated values
        let (game, reveals) = s.split_once(':')?;

        if !game.starts_with("Game ") {
            return None;
        }

        Some(Game {
            id: game[5..].parse().ok()?,
            reveals: reveals
                .split(';')
                .filter_map(|r| if r.is_empty() { None } else { Some(r.into()) })
                .collect(),
        })
    }

    pub fn possible(&self, bag: &Bag) -> bool {
        self.reveals.iter().all(|r| bag.reveal_possible(r))
    }

    pub fn min_bag(&self) -> Bag {
        let mut bag = Bag::default();
        for reveal in self.reveals.iter() {
            bag.increase_to_allow_reveal(reveal)
        }
        bag
    }
}

#[cfg(test)]
mod tests {
    use crate::{Bag, Game, Reveal};

    #[test]
    fn test_into() {
        assert_eq!(
            Into::<Reveal>::into("1 red, 2 green, 3 blue"),
            Reveal {
                red: 1,
                green: 2,
                blue: 3
            }
        );

        assert_eq!(
            Into::<Reveal>::into("1 red"),
            Reveal {
                red: 1,
                green: 0,
                blue: 0
            }
        );

        assert_eq!(
            Into::<Reveal>::into("100 green"),
            Reveal {
                red: 0,
                green: 100,
                blue: 0
            }
        );
    }

    #[test]
    fn test_parsing() {
        assert_eq!(Game::parse("Invalid"), None);
        assert_eq!(Game::parse("NotAGame 123: this is a test"), None);
        assert_eq!(
            Game::parse("Game 1:"),
            Some(Game {
                id: 1,
                reveals: vec![]
            })
        );
        assert_eq!(
            Game::parse("Game 1: 1 red"),
            Some(Game {
                id: 1,
                reveals: vec![Reveal {
                    red: 1,
                    green: 0,
                    blue: 0
                }]
            })
        );
        assert_eq!(
            Game::parse("Game 1: 1 red; 1 red"),
            Some(Game {
                id: 1,
                reveals: vec![
                    Reveal {
                        red: 1,
                        green: 0,
                        blue: 0
                    },
                    Reveal {
                        red: 1,
                        green: 0,
                        blue: 0
                    }
                ]
            })
        );
    }

    #[test]
    fn test_increase_game1() {
        let mut bag = Bag::default();

        bag.increase_to_allow_reveal(&Reveal::from("3 blue, 4 red"));
        bag.increase_to_allow_reveal(&Reveal::from("1 red, 2 green, 6 blue"));
        bag.increase_to_allow_reveal(&Reveal::from("2 green"));

        assert_eq!(
            bag,
            Bag {
                red: 4,
                green: 2,
                blue: 6
            }
        );
        assert_eq!(bag.power(), 48);
    }

    #[test]
    fn test_increase_game4() {
        let game =
            Game::parse("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red")
                .expect("Valid");
        let bag = game.min_bag();
        assert_eq!(
            bag,
            Bag {
                red: 14,
                green: 3,
                blue: 15
            }
        );
        assert_eq!(bag.power(), 630);
    }

    #[test]
    fn test_reveal() {
        let bag = crate::Bag {
            red: 12,
            green: 13,
            blue: 14,
        };
        assert!(
            Game::parse("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")
                .expect("Valid")
                .possible(&bag)
        );
        assert!(
            Game::parse("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue")
                .expect("Valid")
                .possible(&bag)
        );
        assert!(!Game::parse(
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
        )
        .expect("Valid")
        .possible(&bag));
        assert!(!Game::parse(
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
        )
        .expect("Valid")
        .possible(&bag));
        assert!(
            Game::parse("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green")
                .expect("Valid")
                .possible(&bag)
        );
    }
}
