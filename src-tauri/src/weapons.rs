use serde::Serialize;
use serde_json::{ json, Value };

#[derive(Debug, Serialize)]
pub enum WeaponType {
  Projectile,
  DeflectedProjectile,
  Flamethrower,
  Hitscan,
  Building,
  Rifle,
  Melee,
  Knife,
  Misc,
  Status,
}

#[derive(Debug, Serialize)]
pub struct Weapon {
  pub name: String,
  pub weapon_type: WeaponType,
}

pub fn get_weapons_as_json() -> Value {
  let mut weapons = json!({});

  let proj =
    json!({
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
      "flaregun": "Flare Gun",
      "scorch_shot": "Scorch Shot",
      "rocketlauncher_directhit": "Direct Hit",
      "battleneedle": "Blutsauger",
    });

  let flamethrowers =
    json!({
      "flamethrower": "Flamethrower",
      "degreaser": "Degreaser",
      "dragons_fury_bonus": "Dragon's Fury",
    });

  let melee =
    json!({
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
      "prinny_machete": "Prinny Machete",
      "scotland_shard": "Scottish Handshake",
    });

  let knives =
    json!({
      "spy_cicle": "Spy Cicle",
      "knife": "Knife",
      "black_rose": "Black Rose",
      "eternal_reward": "Eternal Reward",
      "kunai": "Kunai",
      "big_earner": "Big Earner",
    });

  let hitscan =
    json!({
      "scattergun": "Scattergun",
      "pistol_scout": "Pistol",
      "pistol": "Pistol",
      "soda_popper": "Soda Popper",
      "the_winger": "Winger",
      "back_scatter": "Back Scatter",
      "force_a_nature": "Force-A-Nature",
      "shortstop": "Shortstop",
      "tomislav": "Tomislav",
      "shotgun_primary": "Shotgun",
      "shotgun_pyro": "Shotgun",
      "wrangler_kill": "Wrangler",
      "letranger": "L'Etranger",
      "revolver": "Revolver",
      "minigun": "Minigun",
      "smg": "SMG",
      "the_winger": "Winger",
      "frontier_kill": "Frontier Justice",
    });

  let buildings =
    json!({
      "obj_minisentry": "Minisentry",
      "obj_sentrygun": "Sentrygun",
    });

  let rifles =
    json!({
      "sniperrifle": "Sniper Rifle",
      "ambassador": "Ambassador",
      "huntsman": "Huntsman",
    });

  let misc = json!({
      "world": "World",
    });

  let status = json!({
      "bleed_kill": "Bleed",
    });

  for (weapon, name) in proj.as_object().unwrap() {
    weapons[weapon] = json!(Weapon {
      name: format!("the {}", name.as_str().unwrap()),
      weapon_type: WeaponType::Projectile,
    });

    weapons[format!("deflect_{}", weapon)] = json!(Weapon {
      name: format!("a reflected {}", name.as_str().unwrap()),
      weapon_type: WeaponType::DeflectedProjectile,
    });
  }

  for (weapon, name) in flamethrowers.as_object().unwrap() {
    weapons[weapon] = json!(Weapon {
      name: format!("the {}", name.as_str().unwrap()),
      weapon_type: WeaponType::Flamethrower,
    });
  }

  for (weapon, name) in melee.as_object().unwrap() {
    weapons[weapon] = json!(Weapon {
      name: format!("the {}", name.as_str().unwrap()),
      weapon_type: WeaponType::Melee,
    });
  }

  for (weapon, name) in knives.as_object().unwrap() {
    weapons[weapon] = json!(Weapon {
      name: format!("the {}", name.as_str().unwrap()),
      weapon_type: WeaponType::Knife,
    });
  }

  for (weapon, name) in hitscan.as_object().unwrap() {
    weapons[weapon] = json!(Weapon {
      name: format!("the {}", name.as_str().unwrap()),
      weapon_type: WeaponType::Hitscan,
    });
  }

  for (weapon, name) in buildings.as_object().unwrap() {
    weapons[weapon] = json!(Weapon {
      name: format!("a {}", name.as_str().unwrap()),
      weapon_type: WeaponType::Building,
    });
  }

  for (weapon, name) in rifles.as_object().unwrap() {
    weapons[weapon] = json!(Weapon {
      name: format!("the {}", name.as_str().unwrap()),
      weapon_type: WeaponType::Rifle,
    });
  }

  for (weapon, name) in misc.as_object().unwrap() {
    weapons[weapon] = json!(Weapon {
      name: format!("the {}", name.as_str().unwrap()),
      weapon_type: WeaponType::Misc,
    });
  }

  for (weapon, name) in status.as_object().unwrap() {
    weapons[weapon] = json!(Weapon {
      name: name.to_string(),
      weapon_type: WeaponType::Status,
    });
  }

  weapons
}
