function typeValue(val) {
  switch (val) {
    case "bool" || "toggle":
      return 0;
    case "string" || "int":
      return 1;
    default:
      return 2;
  }
}

export default function addonTypeSort(addons) {
  let addonKeys = Object.keys(addons);

  return addonKeys.sort((a, b) => {
    return typeValue(addons[a].type) - typeValue(addons[b].type);
  });
}
