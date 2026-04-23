// game concept
// a player can be a warrior, mage, rogue, or healer. qualities like health, strength, agility, and intelligence determine their abilities.
//     weapon and armor determine their physical capabilities and are optional.
// rules:
// Class    |	Base Strength |	Base Agility |	Base Intelligence |	Base Health
// Warrior  |	15            |	8            |	3                 |	120
// Mage     |	3             |	8            |	15                |	80
// Rogue    |	10            |	14           |	6                 |	100
// Healer   |	5             |	10           |	14                |	90

// Weapon rules
// Class   |	Allowed Weapons |	Notes
// Warrior |	Sword, Axe      |	Should have a weapon
// Mage    |	Staff           |	Optional
// Rogue   |	Bow, Dagger     |	Optional
// Healer  |	None            |	Cannot equip weapon
//
// Armor rules
// Class   |	Allowed Armor  |	Notes
// Warrior |	Light, Heavy   |	Should have armor
// Mage    |	❌ None allowed |    Cannot equip armor
// Rogue   |	Light, Medium  |	Optional
// Healer  |	Light, Medium  |	Should have armor

// Strength    |	Default from class, can override
// Agility     |	Default from class, can override
// Intelligence|	Default from class, can override
// Health      |	Always from class (no override)

// Name        |	Required
// Class       |	Required
// Weapon      |	Optional (validated if present)
// Armor       |	Optional (validated if present)
// Stats       |	Optional overrides
// Health      |	Derived only

#[derive(Debug, PartialEq)]
struct Player {
    name: String,
    health: u32,
    class: PlayerClass,
    strength: u32,
    agility: u32,
    intelligence: u32,
    weapon: Option<Weapon>,
    armor: Option<Armor>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum PlayerClass {
    Warrior,
    Mage,
    Rogue,
    Healer,
}

#[derive(Debug, PartialEq)]
enum Weapon {
    Sword,
    Bow,
    Staff,
    Axe,
    Dagger,
}

#[derive(Debug, PartialEq)]
enum Armor {
    Light,
    Medium,
    Heavy,
}

struct PlayerBuilder {
    name: Option<String>,
    class: Option<PlayerClass>,
    strength: Option<u32>,
    agility: Option<u32>,
    intelligence: Option<u32>,
    weapon: Option<Weapon>,
    armor: Option<Armor>,
}

impl PlayerBuilder {
    fn new() -> Self {
        Self {
            name: None,
            class: None,
            strength: None,
            agility: None,
            intelligence: None,
            weapon: None,
            armor: None,
        }
    }
    fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_owned());
        self
    }

    fn class(mut self, class: PlayerClass) -> Self {
        self.class = Some(class);
        self
    }

    fn weapon(mut self, weapon: Weapon) -> Self {
        self.weapon = Some(weapon);
        self
    }
    fn armor(mut self, armor: Armor) -> Self {
        self.armor = Some(armor);
        self
    }
    fn build(self) -> Result<Player, String> {
        let name = self.name.ok_or("Name is required")?;
        let class = self.class.ok_or("Class is required")?;

        let (base_str, base_agi, base_int, base_health) = match class {
            PlayerClass::Warrior => (15, 8, 3, 120),
            PlayerClass::Mage => (3, 8, 15, 80),
            PlayerClass::Rogue => (10, 14, 6, 100),
            PlayerClass::Healer => (5, 10, 14, 90),
        };
        let strength = self.strength.unwrap_or(base_str);
        let agility = self.agility.unwrap_or(base_agi);
        let intelligence = self.intelligence.unwrap_or(base_int);
        let health = base_health;

        if class == PlayerClass::Warrior && (self.weapon.is_none() || self.armor.is_none()) {
            return Err("Warrior should have both weapon and armor".into());
        }

        if class == PlayerClass::Healer && self.armor.is_none() {
            return Err("Healer should have armor".into());
        }

        if let Some(ref weapon) = self.weapon {
            match (class, weapon) {
                (PlayerClass::Warrior, Weapon::Sword | Weapon::Axe) => {}
                (PlayerClass::Mage, Weapon::Staff) => {}
                (PlayerClass::Rogue, Weapon::Bow | Weapon::Dagger) => {}
                (PlayerClass::Healer, _) => {
                    return Err("Healer cannot have a weapon".into());
                }
                _ => return Err("Invalid weapon for class".into()),
            }
        }

        if let Some(ref armor) = self.armor {
            match (class, armor) {
                (PlayerClass::Warrior, Armor::Light | Armor::Heavy) => {}
                (PlayerClass::Mage, _) => {
                    return Err("Mage cannot have an armor".into());
                }
                (PlayerClass::Rogue, Armor::Light | Armor::Medium) => {}
                (PlayerClass::Healer, Armor::Light | Armor::Medium) => {}
                _ => return Err("Invalid armor for class".into()),
            }
        }

        Ok(Player {
            name,
            class,
            health,
            strength,
            agility,
            intelligence,
            weapon: self.weapon,
            armor: self.armor,
        })
    }
}

use std::io;

fn main() {
    println!("=== Welcome to Player Builder Game ===");
    println!("Create your character by choosing class, weapon, and armor.\n");

    // --- Name ---
    let mut input = String::new();
    println!("Enter player name:");
    io::stdin().read_line(&mut input).unwrap();
    let name = input.trim().to_string();

    // --- Class ---
    input.clear();
    println!("Choose class: warrior | mage | rogue | healer");
    io::stdin().read_line(&mut input).unwrap();
    let class = match input.trim().to_lowercase().as_str() {
        "warrior" => PlayerClass::Warrior,
        "mage" => PlayerClass::Mage,
        "rogue" => PlayerClass::Rogue,
        "healer" => PlayerClass::Healer,
        _ => {
            println!("Invalid class");
            return;
        }
    };

    // --- Weapon ---
    input.clear();
    println!("Choose weapon (or press Enter to skip): sword | axe | bow | dagger | staff");
    io::stdin().read_line(&mut input).unwrap();
    let weapon = match input.trim().to_lowercase().as_str() {
        "sword" => Some(Weapon::Sword),
        "axe" => Some(Weapon::Axe),
        "bow" => Some(Weapon::Bow),
        "dagger" => Some(Weapon::Dagger),
        "staff" => Some(Weapon::Staff),
        "" => None,
        _ => {
            println!("Invalid weapon");
            return;
        }
    };

    // --- Armor ---
    input.clear();
    println!("Choose armor (or press Enter to skip): light | medium | heavy");
    io::stdin().read_line(&mut input).unwrap();
    let armor = match input.trim().to_lowercase().as_str() {
        "light" => Some(Armor::Light),
        "medium" => Some(Armor::Medium),
        "heavy" => Some(Armor::Heavy),
        "" => None,
        _ => {
            println!("Invalid armor");
            return;
        }
    };

    // --- Build player ---
    let mut builder = PlayerBuilder::new().name(&name).class(class);

    if let Some(w) = weapon {
        builder = builder.weapon(w);
    }

    if let Some(a) = armor {
        builder = builder.armor(a);
    }

    let result = builder.build();

    // --- Output ---
    match result {
        Ok(player) => {
            println!("\n=== Player Created Successfully ===");
            println!("{:?}", player);
        }
        Err(e) => {
            println!("\n=== Failed to create player ===");
            println!("Error: {}", e);
        }
    }
}
