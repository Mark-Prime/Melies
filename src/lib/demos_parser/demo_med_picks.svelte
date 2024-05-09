<script>
  import ClassLogo from "../classlogo.svelte";

  export let lives;
  export let classConverter;
  export let parsed_demo;
  export let tickToTime;
  export let toggleKillsSelected;

  function getKill(med_pick) {
    return parsed_demo.data.player_lives[med_pick.owner_id][med_pick.life_index]
      .kills[med_pick.kill_index];
  }
</script>

<h4 class="centered">Med Picks</h4>
<div class="killstreaks card demo__kills">
  {#each lives as life}
    {#each life.med_picks as med_pick}
      <div class="demo__kill">
        <div class="demo__kill-text">
          <ClassLogo
            player_class={classConverter(getKill(med_pick).killer_class)}
          /> killed
          <a
            href={`#player-${parsed_demo.data.users[getKill(med_pick).victim].name}`}
            class={parsed_demo.data.users[getKill(med_pick).victim]["team"] +
              " tooltip"}
            style="--kills: 0;"
            data-tooltip="Jump To Player"
          >
            <ClassLogo
              player_class={classConverter(getKill(med_pick).victim_class)}
            />
            {parsed_demo.data.users[getKill(med_pick).victim].name}
          </a>
          with {getKill(med_pick).weapon}
          {#if getKill(med_pick).crit_type}
            <span
              class={["", "killstreak", "killstreak--large"][
                getKill(med_pick).crit_type
              ]}
            >
              {["", " (Mini-Crit) ", " (CRITICAL HIT!) "][
                getKill(med_pick).crit_type
              ]}
            </span>
          {/if}
          at
          <span
            class="tooltip"
            style={`--kills: 0;`}
            data-tooltip={`Timecode: ${tickToTime(getKill(med_pick).tick)}`}
          >
            {getKill(med_pick).tick}
          </span>
        </div>

        <div class="add_demo">
          {#if getKill(med_pick).selected}
            <button
              class="add_demo cancel-btn"
              on:click={toggleKillsSelected([getKill(med_pick)])}>-</button
            >
          {:else}
            <div class="add_demo">
              <button on:click={toggleKillsSelected([getKill(med_pick)])}>
                +
              </button>
            </div>
          {/if}
        </div>
      </div>
    {/each}
  {/each}
</div>

<style lang="scss">
  .killstreaks {
    max-width: 800px;
    margin: auto;
  }

  .add_demo {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    gap: 1px;
    width: fit-content;

    & > button {
      font-size: 12px;
      padding: 0.3rem 0.7rem;
    }
  }

  .card {
    font-size: small;
    font-family: "Source Code Pro", monospace;
    color: var(--tert-con-text);
    border: 1px solid var(--tert-con);
    border-radius: 5px;
  }

  .demo__kill {
    display: flex;

    &-text {
      display: flex;
      gap: 0.5rem;
      width: 100%;
      flex-grow: 1;
      white-space: nowrap;
      align-items: center;
    }

    a {
      display: flex;
      justify-content: start;
      align-items: center;
      gap: 0.5rem;
      margin: 0;

      &:first-of-type {
        margin-left: 0;
      }
    }
  }

  .demo__kills {
    grid-row: 2;
    grid-column: 1 / 7;
    display: flex;
    flex-direction: column;
    width: 100%;
    transition: all 0.2s;
    border-radius: 3px;
    padding: 0.3rem 0.5rem;
    gap: 0.1rem;
  }
</style>
