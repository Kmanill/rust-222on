#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    #[test]
    fn test1() {
        let mut scores = HashMap::new();
        scores.insert("Sunface", 98);
        scores.insert("Daniel", 95);
        scores.insert("Ashley", 69);
        scores.insert("Katie", 58);

        let score = scores.get("Sunface");
        assert_eq!(score, Some(&98));

        if scores.contains_key("Daniel") {
            // Доступ к значению по индексу возвращает V
            let score = scores["Daniel"];
            assert_eq!(score, 95);
            scores.remove("Daniel");
        }

        assert_eq!(scores.len(), 3);

        for (name, score) in &scores {
            println!("The score of {} is {}", name, score);
        }
    }
    #[test]
    fn test2() {
        let teams = [
            ("Chinese Team", 100),
            ("American Team", 10),
            ("France Team", 50),
        ];

        let mut teams_map1 = HashMap::new();
        for team in &teams {
            teams_map1.insert(team.0, team.1);
        }

        let teams_map2: HashMap<_, _> = teams.iter().cloned().collect();

        assert_eq!(teams_map1, teams_map2);

        println!("Success!");
    }
    #[test]
    fn test3() {
        let mut player_stats = HashMap::new();

        player_stats.entry("health").or_insert(100);
        assert_eq!(player_stats["health"], 100);

        player_stats.entry("health").or_insert_with(|| 42);
        assert_eq!(player_stats["health"], 100);

        let health = player_stats.entry("health").or_insert(50);
        assert_eq!(health, &100);
        *health -= 50;
        assert_eq!(*health, 50);

        println!("Success!");
    }

    #[derive(Hash, Eq, PartialEq, Debug)]
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
        let vikings = HashMap::from([
            (Viking::new("Einar", "Norway"), 25),
            (Viking::new("Olaf", "Denmark"), 24),
            (Viking::new("Harald", "Iceland"), 12),
        ]);

        for (viking, health) in &vikings {
            println!("{:?} has {} hp", viking, health);
        }
    }






}