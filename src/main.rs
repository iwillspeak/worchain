use markov::Chain;

const ALICE: &str = include_str!("../assets/11-0.txt");
const FRANK: &str = include_str!("../assets/84-0.txt");
const JANE: &str = include_str!("../assets/1342-0.txt");
const ARTHUR: &str = include_str!("../assets/1661-0.txt");
const REED: &str = include_str!("../assets/pg68706.txt");

fn train_chain_on(chain: &mut Chain<char>, text: &str) {
    let words = text.split_ascii_whitespace().filter_map(|word| {
        match word.trim_matches(|ch: char| !ch.is_alphabetic()) {
            "" => None,
            trimmed => {
                if trimmed.contains(|ch: char| !ch.is_alphanumeric()) {
                    if !trimmed.ends_with("'s") || !trimmed.ends_with("'nt") {
                        return None;
                    }
                    println!("Non alpha word? {}, ignoring", trimmed);
                }
                Some(trimmed)
            }
        }
    });
    words.for_each(|word| {
        let chars = word.to_lowercase().chars().collect::<Vec<_>>();
        chain.feed(chars);
    });
}

fn main() {
    let mut chain = markov::Chain::of_order(4);
    train_chain_on(&mut chain, ALICE);
    train_chain_on(&mut chain, FRANK);
    train_chain_on(&mut chain, JANE);
    train_chain_on(&mut chain, ARTHUR);
    train_chain_on(&mut chain, REED);

    println!(
        "{}",
        r#"{
    "metadata" : {
          "name" : "worchain-word",
          "size" : 5000,
          "packagedAt" : "2022-06-12T10:51:35Z",
          "version" : 1
        },
        "words" : ["#
    );
    for _ in 1..5000 {
        print!(
            r#""{}", "#,
            chain.generate().into_iter().collect::<String>()
        );
    }
    print!(r#""{}""#, chain.generate().into_iter().collect::<String>());
    println!("{}", "]}");
}
