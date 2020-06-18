use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum GemType {
    //Gold,
    White,
    Blue,
    Green,
    Red,
    Brown,
}

type GemPool = HashMap<GemType, usize>;

#[allow(dead_code)]
struct Improvement {
    name: String,
    flavor: GemType,
    bonus: usize,
    cost: GemPool,
}

#[allow(dead_code)]
fn purchase(hand: &GemPool, cost: &GemPool) -> Option<GemPool> {
    if can_aford(hand, cost) {
        let mut result = GemPool::new();
        for (kk, vv) in hand.iter(){
            let tmp = match cost.get(kk){
                Some(&cnt) => vv - cnt,
                _ => *vv
            };
            result.insert(*kk, tmp);
        }
        Some(result)
    } else {
        None
    }
}

#[allow(dead_code)]
fn can_aford(hand: &GemPool, cost: &GemPool) -> bool {
    let mut have_enough = true;
    for (kk, need) in cost.iter(){
        let tmp = match hand.get(kk){
            Some(&have) => if have >= *need { true } else { false },
            _ => false
        };
        have_enough &= tmp;
    }
    have_enough
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn afordable() {
        let mut cost = GemPool::new();
        let mut hand = GemPool::new();
        cost.insert(GemType::White, 1);
        hand.insert(GemType::White, 2);
        assert_eq!(can_aford(&hand, &cost), true);
        assert_eq!(can_aford(&cost, &hand), false);

        hand.insert(GemType::White, 1);
        assert_eq!(can_aford(&hand, &cost), true);
        assert_eq!(can_aford(&cost, &hand), true);

        hand.insert(GemType::Red, 1);
        assert_eq!(can_aford(&hand, &cost), true);
        assert_eq!(can_aford(&cost, &hand), false);

        cost.insert(GemType::Blue, 1);
        assert_eq!(can_aford(&hand, &cost), false);
        assert_eq!(can_aford(&cost, &hand), false);
    }

    #[test]
    fn can_purchase_simple() {
        let mut cost = GemPool::new();
        let mut hand = GemPool::new();
        cost.insert(GemType::White, 1);
        hand.insert(GemType::White, 2);

        let res = purchase(&hand, &cost);
        let valid = match res {
            Some(rem) => {
                assert_eq!(rem.len(), 1);
                assert_eq!(rem.contains_key(&GemType::White), true);
                true
            },
            _ => false
        };
        assert!(valid, "expected a Some<GemPool>")
        
    }

    #[test]
    fn cannot_purchase() {
        let mut cost = GemPool::new();
        let mut hand = GemPool::new();
        hand.insert(GemType::White, 1);
        cost.insert(GemType::White, 2);

        let res = purchase(&hand, &cost);
        let cant_afford = match res {
            None => true,
            _ => false
        };
        assert!(cant_afford, "shouldn't be able to buy something that we can't afford")
    }
}