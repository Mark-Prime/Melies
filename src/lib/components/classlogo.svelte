<script>
  /** @type {{player_class: any, tooltip?: boolean, click?: any, args?: any}} */
  let {
    player_class,
    tooltip = false,
    click = () => {},
    args = []
  } = $props();

  function getImgUrl(player_class) {
    switch (player_class) {
      case "scout":
        return "https://wiki.teamfortress.com/w/images/a/ad/Leaderboard_class_scout.png";
      case "soldier":
        return "https://wiki.teamfortress.com/w/images/9/96/Leaderboard_class_soldier.png";
      case "pyro":
        return "https://wiki.teamfortress.com/w/images/8/80/Leaderboard_class_pyro.png";
      case "demoman":
        return "https://wiki.teamfortress.com/w/images/4/47/Leaderboard_class_demoman.png";
      case "heavy":
        return "https://wiki.teamfortress.com/w/images/5/5a/Leaderboard_class_heavy.png";
      case "engineer":
        return "https://wiki.teamfortress.com/w/images/1/12/Leaderboard_class_engineer.png";
      case "medic":
        return "https://wiki.teamfortress.com/w/images/e/e5/Leaderboard_class_medic.png";
      case "sniper":
        return "https://wiki.teamfortress.com/w/images/f/fe/Leaderboard_class_sniper.png";
      case "spy":
        return "https://wiki.teamfortress.com/w/images/3/33/Leaderboard_class_spy.png";
      default:
        return false;
    }
  }
</script>

{#if getImgUrl(player_class)}
  <button
    class:tooltip={tooltip}
    data-tooltip={tooltip}
    style="--kills: 0"
    onclick={() => click(...args)}
    onkeydown={() => click(...args)}
  >
    <img src={getImgUrl(player_class)} alt={`${player_class} Icon`} />
  </button>
{/if}

<style lang="scss">
  button {
    all: unset;

    &:hover {
      filter: brightness(1);
    }
  }

  .tooltip {
    cursor: pointer;
  }

  img {
    height: 1.5rem;
    display: flex;
    justify-content: center;
    align-items: center;
  }
</style>
