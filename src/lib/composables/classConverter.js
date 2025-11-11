export function classConverter(playerClass) {
  switch (playerClass) {
    case "1":
      return "scout";
    case "3":
      return "soldier";
    case "7":
      return "pyro";
    case "4":
      return "demoman";
    case "6":
      return "heavy";
    case "9":
      return "engineer";
    case "5":
      return "medic";
    case "2":
      return "sniper";
    case "8":
      return "spy";
    default:
      return playerClass;
  }
}

export function classNumConverter(playerClass) {
  switch (playerClass) {
    case "1":
      return 1;
    case "3":
      return 2;
    case "7":
      return 3;
    case "4":
      return 4;
    case "6":
      return 5;
    case "9":
      return 6;
    case "5":
      return 7;
    case "2":
      return 8;
    case "8":
      return 9;
    default:
      return playerClass;
  }
}
