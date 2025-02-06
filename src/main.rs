const ADJECTIVES: [&str; 27] = [
    "beautiful",
    "sophisticated",
    "poetic",
    "noble",
    "talented",
    "brilliant",
    "powerful",
    "perfect",
    "cunning",
    "chestnut-haired",
    "thoughtful",
    "kind",
    "rule-breaking",
    "glowing",
    "sweet",
    "innocent",
    "pretty",
    "amazing",
    "opalescent",
    "rainbow infused",
    "naive",
    "beautiful",
    "cunning",
    "tricky",
    "sassy",
    "smooth coated",
    "majestic",
];

const NOUNS: [&str; 14] = [
    "newborn baby",
    "land mermaid",
    "musk ox",
    "sunflower",
    "unicorn nurse",
    "spinster",
    "sunfish",
    "sun goddess",
    "tropical fish",
    "tree shark",
    "space unicorn",
    "mannequin",
    "sea otter",
    "flamingo",
];

fn main() {
    let random_adjective = ADJECTIVES[rand::random_range(0..ADJECTIVES.len())];
    let random_noun = NOUNS[rand::random_range(0..NOUNS.len())];

    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg("whoami")
        .output()
        .expect("Failed to execute process");
    let name = String::from_utf8_lossy(&output.stdout);
    println!(
        "Have a wonderful day {}, you {random_adjective} {random_noun}!",
        name.trim()
    );
}
