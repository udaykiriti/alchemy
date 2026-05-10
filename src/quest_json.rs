use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::output::{print_section, print_separator};

const HIGH_PRIORITY_DANGER_LEVEL: u8 = 6;

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
    artifacts: Vec<Cow<'static, str>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Quest {
    title: Cow<'static, str>,
    region: Cow<'static, str>,
    danger_level: u8,
    status: QuestStatus,
    reward: Reward,
    party: Vec<Cow<'static, str>>,
}

pub fn run() {
    print_section("Quest JSON example");

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

    for quest in restored
        .iter()
        .filter(|quest| quest.danger_level >= HIGH_PRIORITY_DANGER_LEVEL)
    {
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
            title: Cow::Borrowed("Silence the Ember Wyrm"),
            region: Cow::Borrowed("Ashen Peaks"),
            danger_level: 9,
            status: QuestStatus::InProgress,
            reward: Reward {
                gold: 1800,
                artifacts: vec![
                    Cow::Borrowed("wyrm scale shield"),
                    Cow::Borrowed("emberglass vial"),
                ],
            },
            party: vec![
                Cow::Borrowed("Mira"),
                Cow::Borrowed("Tovin"),
                Cow::Borrowed("Salt"),
            ],
        },
        Quest {
            title: Cow::Borrowed("Map the Sunken Library"),
            region: Cow::Borrowed("Drowned Quarter"),
            danger_level: 6,
            status: QuestStatus::Posted,
            reward: Reward {
                gold: 950,
                artifacts: vec![
                    Cow::Borrowed("waterproof codex"),
                    Cow::Borrowed("brass astrolabe"),
                ],
            },
            party: vec![Cow::Borrowed("Iris"), Cow::Borrowed("Fen")],
        },
        Quest {
            title: Cow::Borrowed("Negotiate with the Moss Court"),
            region: Cow::Borrowed("Verdant Hollow"),
            danger_level: 4,
            status: QuestStatus::Completed,
            reward: Reward {
                gold: 600,
                artifacts: vec![Cow::Borrowed("living treaty seal")],
            },
            party: vec![Cow::Borrowed("Cael")],
        },
    ]
}
