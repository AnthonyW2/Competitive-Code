pub mod day7 {
    
    use std::collections::HashMap;
    
    struct Hand {
        cards: Vec<u32>,
        strength: u32
    }
    
    impl Hand {
        fn from(hand_str: &str) -> Hand {
            
            let card_map = HashMap::from([
                ('A', 14),
                ('K', 13),
                ('Q', 12),
                ('J', 11),
                ('T', 10),
                ('9', 9),
                ('8', 8),
                ('7', 7),
                ('6', 6),
                ('5', 5),
                ('4', 4),
                ('3', 3),
                ('2', 2),
            ]);
            
            // Kay: card; value: card count
            let mut card_counts: HashMap<u32, u32> = HashMap::new();
            
            let encoded_cards = hand_str.chars().map(|c| {
                let encoded = card_map[&c];
                card_counts.entry(encoded).and_modify(|count| *count += 1).or_insert(1);
                return encoded;
            }).collect::<Vec<_>>();
            
            let mut counts_vec = card_counts.into_values().collect::<Vec<_>>();
            counts_vec.sort();
            
            
            // Calculate the strength of the hand
            let strength: u32;
            if counts_vec.len() == 1 {
                // Five of a kind
                strength = 6;
            } else if counts_vec.len() == 2 {
                if counts_vec[0] == 1 {
                    // Four of a kind
                    strength = 5;
                } else {
                    // Full house
                    strength = 4;
                }
            } else if counts_vec.len() == 3 {
                if counts_vec[2] == 3 {
                    // Three of a kind
                    strength = 3;
                } else {
                    // Two pair
                    strength = 2;
                }
            } else if counts_vec.len() == 4 {
                // One pair
                strength = 1;
            } else {
                // High card
                strength = 0;
            }
            
            return Hand {
                cards: encoded_cards,
                strength: strength
            }
            
        }
        
        fn from2(hand_str: &str) -> Hand {
            
            let card_map = HashMap::from([
                ('A', 13),
                ('K', 12),
                ('Q', 11),
                ('T', 10),
                ('9', 9),
                ('8', 8),
                ('7', 7),
                ('6', 6),
                ('5', 5),
                ('4', 4),
                ('3', 3),
                ('2', 2),
                ('J', 1),
            ]);
            
            // Kay: card; value: card count
            let mut card_counts: HashMap<u32, u32> = HashMap::new();
            
            let encoded_cards = hand_str.chars().map(|c| {
                let encoded = card_map[&c];
                card_counts.entry(encoded).and_modify(|count| *count += 1).or_insert(1);
                return encoded;
            }).collect::<Vec<_>>();
            
            let jokers = *card_counts.entry(1).or_default();
            card_counts.remove(&1);
            
            let mut counts_vec = card_counts.into_values().collect::<Vec<_>>();
            counts_vec.sort();
            
            
            // Calculate the strength of the hand
            let strength: u32;
            if counts_vec.len() <= 1 {
                // Five of a kind
                strength = 6;
            } else if counts_vec.len() == 2 {
                if counts_vec[0] == 1 {
                    // Four of a kind
                    strength = 5;
                } else {
                    // Full house
                    strength = 4;
                }
            } else if counts_vec.len() == 3 {
                if counts_vec[2]+jokers == 3 {
                    // Three of a kind
                    strength = 3;
                } else {
                    // Two pair
                    strength = 2;
                }
            } else if counts_vec.len() == 4 {
                // One pair
                strength = 1;
            } else {
                // High card
                strength = 0;
            }
            
            return Hand {
                cards: encoded_cards,
                strength: strength
            }
            
        }
    }
    
    pub fn solution(lines: Vec<String>) {
        
        //println!("{:?}", lines);
        
        // ==== Part 1 ==== //
        
        let mut hands = Vec::new();
        
        for line in &lines {
            let mut split = line.split(' ');
            let hand = Hand::from(split.next().unwrap());
            
            hands.push((
                hand,
                split.next().unwrap().parse::<u32>().unwrap()
            ));
        }
        
        hands.sort_by(|a, b| {
            let str_cmp = a.0.strength.partial_cmp(&b.0.strength).unwrap();
            if str_cmp != std::cmp::Ordering::Equal {
                return str_cmp;
            }
            return a.0.cards.partial_cmp(&b.0.cards).unwrap();
        });
        
        let mut acc1 = 0;
        
        for (i, hand) in (&hands).iter().enumerate() {
            acc1 += hand.1 * (i as u32 + 1);
        }
        
        println!("Part 1: {}", acc1);
        
        
        // ==== Part 2 ==== //
        
        let mut hands = Vec::new();
        
        for line in &lines {
            let mut split = line.split(' ');
            let hand = Hand::from2(split.next().unwrap());
            
            hands.push((
                hand,
                split.next().unwrap().parse::<u32>().unwrap()
            ));
        }
        
        hands.sort_by(|a, b| {
            let str_cmp = a.0.strength.partial_cmp(&b.0.strength).unwrap();
            if str_cmp != std::cmp::Ordering::Equal {
                return str_cmp;
            }
            return a.0.cards.partial_cmp(&b.0.cards).unwrap();
        });
        
        let mut acc2 = 0;
        
        for (i, hand) in (&hands).iter().enumerate() {
            acc2 += hand.1 * (i as u32 + 1);
        }
        
        
        println!("Part 2: {}", acc2);
        
    }
    
}
