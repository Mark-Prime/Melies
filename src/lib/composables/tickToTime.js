export default function tickToTime(ticks) {
  return `${Math.floor(Math.round(ticks / 66) / 60)}m ${
    Math.round(ticks / 66) % 60
  }s`;
}