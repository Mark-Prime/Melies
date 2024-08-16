use serde::Serialize;
use serde_json::{json, Value};

#[derive(Debug, Serialize)]
pub enum WeaponType {
    Melee,
    Ranged,
    Misc
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
    "ball_ornament": "Wrap Assassin Ball",
    "energy_ball": "Short Circuit Orb",
    "flare": "Flare",
    "flare_detonator": "Detonator Flare",
    "grapplinghook": "Grappling Hook",
    "healing_bolt": "Crusader's Crossbow Bolt",
    "huntsman_flyingburn": "Flaming Arrow",
    "mechanicalarmorb": "Short Circuit Orb",
    "pipe": "Grenade",
    "pipe_remote": "Stickybomb",
    "promode": "Grenade",
    "rocket": "Rocket",
    "sentryrocket": "Sentry Rocket",
    "sticky": "Stickybomb"
  });

  for (weapon, name) in proj.as_object().unwrap() {
    weapons[format!("tf_projectile_{}", weapon)] = json!(
      Weapon {
        name: name.to_string(),
        weapon_type: WeaponType::Ranged,
        can_airshot: true
      }
    );

    weapons[format!("deflect_{}", weapon)] = json!(
      Weapon {
        name: format!("Reflected {}", name.to_string()),
        weapon_type: WeaponType::Ranged,
        can_airshot: true
      }
    );
  }

  let melee = json!({
    "tf_weapon_bat": "Bat",
    "tf_weapon_bat_wood": "The Sandman",
    "tf_weapon_bat_fish": "The Holy Mackerel",
  });

  weapons
}