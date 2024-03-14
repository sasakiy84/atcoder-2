use std::{
    cmp::max,
    collections::{HashMap, HashSet},
    fmt::Binary,
};

fn main() {
    proconio::input! {
        n: u32,
    }

    // 問題への考察
    // 不親切な人は、必ずウソをつくわけではないので、正直者の特定に貢献しない、無視してよい
    // 正直者の最大値を答えればよいので、不親切な人を確定させるだけでよい
    // ただし、正直者と仮定した人が、正直者と仮定した人を不親切な人と言っている場合は、矛盾なので、そのcaseを捨てる
    // また、正直者と仮定した人数が、それらの証言から導ける正直者の最大数を超えていた場合、矛盾なので、そのcaseを捨てる

    // 方針
    // bit 演算で全探索する
    // 2 ^ n 通りを調べ上げる必要がある
    // 0 ~ (2 ^ n - 1) の値の bit 表現で、該当する桁の bit が立っている場合、正直者とする
    // n =< 15 であり、2^15 はたかだか30000程度なので、全探索で問題ない

    // まずは不親切な人の証言を集める
    let mut unkindness_testinomies = HashMap::<u32, Vec<u32>>::new();
    let mut honest_testinomies = HashMap::<u32, Vec<u32>>::new();
    for i in 0..n {
        proconio::input! {
            a: u32,
            person_testinomies: [(u32, u32); a]
        }
        unkindness_testinomies.insert(
            i,
            person_testinomies
                .clone()
                .into_iter()
                .filter(|(_, y)| y == &0)
                .map(|(x, _)| x)
                .collect(),
        );
        honest_testinomies.insert(
            i,
            person_testinomies
                .into_iter()
                .filter(|(_, y)| y == &1)
                .map(|(x, _)| x)
                .collect(),
        );
    }

    let mut ans: u32 = 0;
    for bit_mapping in 0..(2_u64.pow(n)) {
        let mut unkindnesses = HashSet::new();
        let mut honests = HashSet::new();
        let mut given_honests = HashSet::new();
        let mut given_unkindnesses = HashSet::new();

        // 正直者ならば、その証言を信用する
        for i in 0..n {
            if is_honest(bit_mapping, i) {
                given_honests.insert(i + 1);
                for unkindness in unkindness_testinomies.get(&i).unwrap() {
                    unkindnesses.insert(*unkindness);
                }
                for honest in honest_testinomies.get(&i).unwrap() {
                    honests.insert(*honest);
                }
            } else {
                given_unkindnesses.insert(i + 1);
            }
        }

        // 正直者と証言された人と不親切と証言された人で矛盾する場合は、このcaseを除外する
        if honests.intersection(&unkindnesses).count() > 0 {
            continue;
        }

        // 正直者と証言された人の中に、不親切と仮定した人が含まれる場合は、このcaseを除外する
        if given_honests.intersection(&unkindnesses).count() > 0 {
            continue;
        }

        // 不親切と証言された人の中に、正直者と仮定した人が含まれる場合は、このcaseを除外する
        if given_unkindnesses.intersection(&honests).count() > 0 {
            continue;
        }

        // 正直者の人数が、証言から導ける最大値より大きい場合は、このcaseを除外する
        let given_honest_people = count_one_bit(bit_mapping) as u32;
        let max_honest_people = n - unkindnesses.len() as u32;
        if given_honest_people > max_honest_people {
            // println!("4");
            continue;
        }

        ans = max(ans, given_honest_people);
    }

    println!("{}", ans)
}

fn is_honest(bit_mapping: u64, num: u32) -> bool {
    (1 << num & bit_mapping) != 0
}

fn count_one_bit<T: Binary>(bit: T) -> usize {
    format!("{:b}", bit).chars().filter(|x| x == &'1').count()
}
