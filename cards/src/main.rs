use std::fmt;
use std::iter::zip; // 0.10.5
use std::vec::Vec;
use std::io;

fn main() -> io::Result<()> {
    let mut game: Game = Default::default();
    let mut hand_a_list: Vec<u8> = vec![];
    
    let mut n_1 = String::new();
    io::stdin().read_line(&mut n_1)?;
    // println!("{}", n_1.trim().parse::<u8>().expect("could not parse"));
    let n_1_int = n_1.trim().parse::<u8>().expect("could not parse");

    for i in 0..n_1_int {
        let mut card_type = String::new();
        io::stdin().read_line(&mut card_type);
        let card_type_ = card_type.trim();
        let split = card_type_.split(" ");
        let card_type_vec = split.collect::<Vec<&str>>();
        let suit: &str = card_type_vec[0];
        let value: &str = card_type_vec[1];
        if suit == "Joker" {
            game.add_joker(value).expect("invalid color");
        } else {
            game.add_card(suit, value).expect("invalid params");
        }
        hand_a_list.push(i);
    }

    // TODO compare hands together

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO
    #[test]
    fn test_hand_beats_hand() -> () {
        assert_eq!(1, 0);
    }
}

mod Card {
    use super::*; 
    
    #[derive(Debug)]
    pub enum Suit {
      Heart,
      Diamond,
      Club,
      Spade
    }
    
    #[derive(Debug)]
    pub enum Color {
        Red,
        Black
    }

    #[derive(PartialEq,Eq,PartialOrd,Ord,Debug)]
    pub enum CardValue {
      Value(u8),
      J, Q, K, A, JOK
    }
    
    impl fmt::Display for CardValue {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                CardValue::Value(val) => write!(f, "{}", val),
                CardValue::J => write!(f, "Joker"),
                _ => write!(f, "nope")
            }
        }
    }

    pub trait CompareCards {
        fn value(&self) -> &CardValue;
    
        // use of Self not object safe when doing dynamic dispatch 
        // fn lt(&self, other_card: &Self) -> bool {
        //     self.value() < other_card.value()
        // }
    }

    // TODO
    // impl<T> fmt::Debug for T 
    // where
    //     T: CompareCards 
    // {
    //     fn fmt(self: &T, f: &mut fmt::Formatter) -> fmt::Result {
    //         write!(f, "{:?}", self)
    //     }
    // }
}

#[derive(Default)]
struct Game<'a> {
    cards: std::vec::Vec<Box<dyn Card::CompareCards>>,
    hands: std::vec::Vec<Hand<'a>>
}

impl<'a> Game<'a> {
    fn add_card(&mut self, suit: &str, value: &str) -> Result<(), &'static str> {
        let suit_ = match suit {
            "Heart" => Card::Suit::Heart,
            "Diamond" => Card::Suit::Diamond,
            "Club" => Card::Suit::Club,
            "Spade" => Card::Suit::Spade,
            _ => return Err("Invalid suit"),
        };

        let value_ = match value {
            "11" => Card::CardValue::J,
            "12" => Card::CardValue::Q,
            "13" => Card::CardValue::K,
            "1" => Card::CardValue::A,
            _ => match value.parse::<u8>() {
                Ok(val) => if val >= 14 {
                    return Err("invalid val")
                } else {
                    Card::CardValue::Value(val)
                },
                Err(_) => return Err("invalid val")
            }, 
        };

        self.cards.push(Box::new(NormalCard{suit: suit_, value: value_}));
        Ok(())
    }

    fn add_joker(&mut self, color: &str) -> Result<(), &'static str> {
        let color_ = match color {
            "Red" => Card::Color::Red,
            "Black" => Card::Color::Black,
            _ => return Err("Invalid color"),
        };
        self.cards.push(Box::new(JokerCard{color: color_}));
        Ok(())
    }

    fn add_hand(&'a mut self, card_indices: Vec<u8>) -> () {
        self.hands.push(Hand{
            cards: card_indices.into_iter().map(|idx| &self.cards[idx as usize]).collect()
        })
    }

    // TODO
    // fn hand_string(&self, hand: u8) -> String {
    //     self.hands[hand as usize].to_string()
    // }
}

// TODO derive clone/copy instead?
// TODO #[derive(Debug)]
struct Hand<'a> {
    cards: Vec<&'a Box<dyn Card::CompareCards>>
}

impl Hand<'_> {
    fn compare_hands(&mut self, other_hand: &mut Self) -> bool {
        self.cards.sort_by(|a, b| a.value().partial_cmp(b.value()).unwrap());
        other_hand.cards.sort_by(|a, b| a.value().partial_cmp(b.value()).unwrap());
        for (a, b) in zip(&self.cards, &other_hand.cards) {
            if a.value() == b.value() {
                continue;
            } else {
                return a.value() < b.value();
            }
        }
        return false;
    }
}

#[derive(Debug)]
struct JokerCard {
    color: Card::Color,
} 

impl Card::CompareCards for JokerCard {
    fn value(&self) -> &Card::CardValue {
        &Card::CardValue::JOK
    } 
}

impl fmt::Display for JokerCard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} Joker", self.color)
    }
}

#[derive(Debug)]
struct NormalCard {
  value: Card::CardValue,
  suit: Card::Suit 
}

impl Card::CompareCards for NormalCard {
    fn value(&self) -> &Card::CardValue {
        &self.value
    }   
}

impl fmt::Display for NormalCard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} of {:?}", self.value, self.suit)
    }
}


