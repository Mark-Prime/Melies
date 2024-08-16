export default function isAirshot(parsed_demo, k, settings) {
  if (!parsed_demo.data.player_lives[k.owner_id][k.life_index]?.kills) {
    return false;
  }

  let kill =
    parsed_demo.data.player_lives[k.owner_id][k.life_index].kills[
      k.kill_index
    ];

  if (["pumpkin", "explosion", "golden_frying_pan"].includes(kill.weapon)) {
    return true;
  }

  let airshot_settings = settings.advanced.airshots;

  let isAirshot = null;

  switch (airshot_settings.killer[kill.killer_class]) {
    case "Never":
      return false;
    case "CriticalHit":
      isAirshot = kill.crit_type === 2;
      break;
    case "MiniCriticalHit":
      isAirshot = kill.crit_type === 1;
      break;
    case "AnyCritHit":
      isAirshot = kill.crit_type !== 0;
      break;
    case "Always":
      return true;
  }

  switch (airshot_settings.victim[kill.victim_class]) {
    case "Never":
      return false;
    case "CriticalHit":
      isAirshot = kill.crit_type === 2 || isAirshot;
      break;
    case "MiniCriticalHit":
      isAirshot = kill.crit_type === 1 || isAirshot;
      break;
    case "AnyCritHit":
      isAirshot = kill.crit_type !== 0 || isAirshot;
      break;
    case "Always":
      return true;
  }

  if (isAirshot === null) {
    return airshot_settings.default;
  }

  return isAirshot;

  // switch (kill.killer_class) {
  //   case "scout":
  //     return false;
  //   case "soldier":
  //     return true;
  //   case "pyro":
  //     return [
  //       "deflect_rocket",
  //       "detonator",
  //       "flare_gun",
  //       "gas_blast",
  //       "scorch_shot",
  //       "deflect_sticky",
  //       "deflect_arrow",
  //       "deflect_ball",
  //       "deflect_cannonballs",
  //       "deflect_flaming_arrow",
  //       "deflect_flare",
  //       "deflect_grenade",
  //       "deflect_repair_claws",
  //       "execution",
  //     ].includes(kill.weapon);
  //   case "demoman":
  //     return true;
  //   case "heavy":
  //     return false;
  //   case "engineer":
  //     return false;
  //   case "medic":
  //     return true;
  //   case "sniper":
  //     return kill.crit_type == 2;
  //   case "spy":
  //     return false;
  //   default:
  //     return false;
  // }
}