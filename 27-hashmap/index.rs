use std::collections::HashMap;
use std::hash::BuildHasherDefault;
// third party hash function
use twox_hash::XxHash64;

#[derive(Debug, Hash, PartialEq, Eq)]
struct Viking {
    name: String,
    country: String,
}

impl Viking {
    fn new(name: &str, country: &str) -> Viking {
        Viking {
            name: name.to_string(),
            country: country.to_string(),
        }
    }
}

fn main() {
    // HashMaps
    let mut scores: HashMap<&str, i32> = HashMap::new();
    scores.insert("Sunface", 98);
    scores.insert("Dipesh", 99);
    scores.insert("Ram", 78);
    scores.insert("Ganesh", 67);
    let score: Option<&i32> = scores.get("Dipesh");
    assert_eq!(score, Some(&99));
    if scores.contains_key("Ram") {
        let score = scores["Ram"];
        assert_eq!(score, 78);
        scores.remove("Ram");
    }
    assert_eq!(scores.len(), 3);
    for (name, score) in &scores {
        println!("The score of {} is {}", name, score);
    }

    let teams: [(&str, i32); 3] = [
        ("Chinese Team", 100),
        ("American Team", 98),
        ("France Team", 50),
    ];
    let mut team_map1: HashMap<&str, i32> = HashMap::new();
    for team in &teams {
        team_map1.insert(team.0, team.1);
    }
    // let team_map2: HashMap<&str, i32> = HashMap::from(teams);
    let team_map2: HashMap<&str, i32> = teams.into_iter().collect();
    assert_eq!(team_map1, team_map2);

    let mut player_stats: HashMap<&str, u8> = HashMap::new();
    player_stats.entry("health").or_insert(100);
    assert_eq!(player_stats["health"], 100);
    player_stats
        .entry("health")
        .or_insert_with(random_stats_buff);
    // 'health' already exists, so it remains 100
    assert_eq!(player_stats["health"], 100);
    let health: &mut u8 = player_stats.entry("health").or_insert(50);
    assert_eq!(health, &100);
    *health -= 50;
    assert_eq!(*health, 50);

    let vikings: HashMap<Viking, i32> = HashMap::from([
        (Viking::new("New Delhi", "India"), 25),
        (Viking::new("Islamabad", "Pakistan"), 27),
        (Viking::new("Kathmandu", "Nepal"), 28),
    ]);
    for (viking, health) in &vikings {
        println!("{:?} has {} HP.", viking, health);
    }

    let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
    map.insert(1, 2);
    map.insert(3, 4);
    assert!(map.capacity() >= 100);
    map.shrink_to(50);
    assert!(map.capacity() >= 50);
    map.shrink_to_fit();
    assert!(map.capacity() >= 2);

    let v1: i32 = 10;
    let mut m1: HashMap<i32, i32> = HashMap::new();
    m1.insert(v1, v1);
    println!("v1 is still usable {}", v1);
    let v2: String = "hello".to_string();
    let mut m2: HashMap<&str, i32> = HashMap::new();
    m2.insert(&v2, v1);
    assert_eq!(v2, "hello");

    let mut hash: HashMap<i32, &str, BuildHasherDefault<XxHash64>> = Default::default();
    hash.insert(42, "the answer");
    assert_eq!(hash.get(&42), Some(&"the answer"));

    println!("Success");
}

fn random_stats_buff() -> u8 {
    48
}
