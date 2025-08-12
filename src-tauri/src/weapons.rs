use serde::Serialize;
use serde_json::{json, Value};

#[derive(Debug, Serialize)]
pub enum WeaponType {
    Melee,
    Hitscan,
    Projectile,
    Misc,
}

#[derive(Debug, Serialize)]
pub struct Weapon {
    pub name: String,
    pub weapon_type: WeaponType,
    pub can_airshot: bool,
}

pub fn get_weapons_as_json() -> Value {
    let mut weapons = json!({});

    let proj = json!({
      "arrow": "Arrow",
      "arrow_fire": "Flaming Arrow",
      "balloffire": "Dragon's Fury Fireball",
      "ball": "Ball",
      "detonator": "Detonator",
      "guillotine": "Flying Guillotine",
      "energy_ball": "Short Circuit Orb",
      "energy_ring": "Righteous Bison",
      "flare": "Flare Gun",
      "grapplinghook": "Grappling Hook",
      "crusaders_crossbow": "Crusader's Crossbow",
      "huntsman_flyingburn": "Flaming Arrow",
      "loose_cannon_impact": "Loose Cannon Impact",
      "loose_cannon_explosion": "Loose Cannon",
      "mechanicalarmorb": "Short Circuit Orb",
      "pipe": "Grenade",
      "pipe_remote": "Stickybomb",
      "tf_projectile_pipe_remote": "Stickybomb",
      "promode": "Grenade",
      "rocket": "Rocket",
      "quake_rl": "Original",
      "tf_projectile_rocket": "Rocket",
      "sentryrocket": "Sentry Rocket",
      "iron_bomber": "Iron Bomber",
    });

    for (weapon, name) in proj.as_object().unwrap() {
        weapons[weapon] = json!(Weapon {
            name: format!("the {}", name.as_str().unwrap()),
            weapon_type: WeaponType::Projectile,
            can_airshot: true
        });

        weapons[format!("deflect_{}", weapon)] = json!(Weapon {
            name: format!("a Reflected {}", name.as_str().unwrap()),
            weapon_type: WeaponType::Projectile,
            can_airshot: true
        });
    }

    let melee = json!({
      "bat": "Bat",
      "holymackerel": "Holy Mackerel",
      "wrap_assassin": "Wrap Assassin",
      "bonesaw": "Bonesaw",
      "fryingpan": "Frying Pan",
      "fists": "Fists",
      "nonnonviolent_protest": "Conscientious Objector",
      "claidheamohmor": "Claidheamh Mòr",
      "warfan": "Fan O'War",
      "prinny_machete": "Prinny Machete",
      "sandman": "Sandman",
      "candy_cane": "Candy Cane",
      "boston_basher": "Boston Basher",
      "skullbat": "Bat Outta Hell",
      "atomizer": "Atomizer",
      "lava_bat": "Sun-on-a-Stick",
      "pep_brawlerblaster": "Baby Face Blaster",
      "pep_pistol": "Pretty Boys Pocket Pistol",
      "disciplinary_action": "Disciplinary Action",
    });

    for (weapon, name) in melee.as_object().unwrap() {
        weapons[weapon] = json!(Weapon {
            name: format!("the {}", name.as_str().unwrap()),
            weapon_type: WeaponType::Melee,
            can_airshot: false
        });
    }

    let hitscan = json!({
      "scattergun": "Scattergun",
      "pistol_scout": "Pistol",
      "soda_popper": "Soda Popper",
      "the_winger": "Winger",
      "back_scatter": "Back Scatter",
      "force_a_nature": "Force-A-Nature",
      "shortstop": "Shortstop",
      "tomislav": "Tomislav",
    });

    for (weapon, name) in hitscan.as_object().unwrap() {
        weapons[weapon] = json!(Weapon {
            name: format!("the {}", name.as_str().unwrap()),
            weapon_type: WeaponType::Hitscan,
            can_airshot: false
        });
    }

    let misc = json!({
      "world": "World"
    });

    for (weapon, name) in misc.as_object().unwrap() {
        weapons[weapon] = json!(Weapon {
            name: format!("the {}", name.as_str().unwrap()),
            weapon_type: WeaponType::Misc,
            can_airshot: false
        });
    }

    weapons
}
