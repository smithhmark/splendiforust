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
fn subtract(hand: GemPool, cost: GemPool) -> GemPool {
    let mut result = GemPool::new();
    for (kk, vv) in hand.iter(){
        let tmp = match cost.get(kk){
            Some(&cnt) => vv - cnt,
            _ => *vv
        };
        result.insert(*kk, tmp);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_subtract_error_free() {
        let mut cost = GemPool::new();
        let mut hand = GemPool::new();
        
        assert_eq!(cost, hand);
        cost.insert(GemType::White, 1);
        hand.insert(GemType::White, 2);

        let rem = subtract(hand, cost);
        assert_eq!(rem.len(), 1);
        assert_eq!(rem.contains_key(&GemType::White), true);
    }
}