use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum QuestStatus {
    Posted,
    InProgress,
    Completed,
}

#[derive(Debug, Serialize, Deserialize)]
struct Reward {
    gold: u32,
    artifacts: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Quest {
    title: String,
    region: String,
    danger_level: u8,
    status: QuestStatus,
    reward: Reward,
    party: Vec<String>,
}

pub fn run() {
    println!("Quest JSON example");

    let quests = sample_quests();
    let json = serde_json::to_string_pretty(&quests).expect("quests should serialize");
    println!("JSON:\n{json}");

    let restored: Vec<Quest> = serde_json::from_str(&json).expect("json should deserialize");
    let total_gold: u32 = restored.iter().map(|quest| quest.reward.gold).sum();
    let highest_danger = restored
        .iter()
        .map(|quest| quest.danger_level)
        .max()
        .unwrap_or(0);

    println!(
        "Loaded {} quests, total reward {}, highest danger {}.",
        restored.len(),
        total_gold,
        highest_danger
    );

    for quest in restored.iter().filter(|quest| quest.danger_level >= 6) {
        println!(
            "High-priority quest: {} in {} with {} party members.",
            quest.title,
            quest.region,
            quest.party.len()
        );
    }

    print_separator();
}

fn sample_quests() -> Vec<Quest> {
    vec![
        Quest {
            title: String::from("Silence the Ember Wyrm"),
            region: String::from("Ashen Peaks"),
            danger_level: 9,
            status: QuestStatus::InProgress,
            reward: Reward {
                gold: 1800,
                artifacts: vec![
                    String::from("wyrm scale shield"),
                    String::from("emberglass vial"),
                ],
            },
            party: vec![
                String::from("Mira"),
                String::from("Tovin"),
                String::from("Salt"),
            ],
        },
        Quest {
            title: String::from("Map the Sunken Library"),
            region: String::from("Drowned Quarter"),
            danger_level: 6,
            status: QuestStatus::Posted,
            reward: Reward {
                gold: 950,
                artifacts: vec![
                    String::from("waterproof codex"),
                    String::from("brass astrolabe"),
                ],
            },
            party: vec![String::from("Iris"), String::from("Fen")],
        },
        Quest {
            title: String::from("Negotiate with the Moss Court"),
            region: String::from("Verdant Hollow"),
            danger_level: 4,
            status: QuestStatus::Completed,
            reward: Reward {
                gold: 600,
                artifacts: vec![String::from("living treaty seal")],
            },
            party: vec![String::from("Cael")],
        },
    ]
}

fn print_separator() {
    println!("--------------------------------------------------");
}
