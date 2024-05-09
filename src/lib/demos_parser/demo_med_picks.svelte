<script>
  import ClassLogo from "../classlogo.svelte";

  export let lives;
  export let label;
  export let player;
  export let valKey;
  export let classConverter;
  export let parsed_demo;
  export let tickToTime;
  export let toggleKillsSelected;
  export let toggleSelected;

  function getKill(pointer) {
    return parsed_demo.data.player_lives[pointer.owner_id][pointer.life_index]
      .kills[pointer.kill_index];
  }

  function getLife(pointer) {
    return parsed_demo.data.player_lives[pointer.owner_id][pointer.life_index];
  }
</script>

{#if parsed_demo.data.player_lives[player].filter((life) => life[valKey].length > 0).length > 0}
  <h4 class="centered pointer__label">{label}</h4>
  <div class="killstreaks card demo__kills">
    {#each lives as life}
      {#each life[valKey] as pointer}
        <div class="demo__kill">
          <div class="demo__kill-text">
            <ClassLogo
              player_class={classConverter(getKill(pointer).killer_class)}
            /> killed
            <a
              href={`#player-${parsed_demo.data.users[getKill(pointer).victim].name}`}
              class={parsed_demo.data.users[getKill(pointer).victim]["team"] +
                " tooltip"}
              style="--kills: 0;"
              data-tooltip="Jump To Player"
            >
              <ClassLogo
                player_class={classConverter(getKill(pointer).victim_class)}
              />
              {parsed_demo.data.users[getKill(pointer).victim].name}
            </a>
            with {getKill(pointer).weapon}
            {#if getKill(pointer).crit_type}
              <span
                class={["", "killstreak", "killstreak--large"][
                  getKill(pointer).crit_type
                ]}
              >
                {["", " (Mini-Crit) ", " (CRITICAL HIT!) "][
                  getKill(pointer).crit_type
                ]}
              </span>
            {/if}
            at
            <span
              class="tooltip"
              style={`--kills: 0;`}
              data-tooltip={`Timecode: ${tickToTime(getKill(pointer).tick)}`}
            >
              {getKill(pointer).tick}
            </span>
          </div>

          <div class="buttons">
            <div
              class="add_demo tooltip tooltip--left"
              data-tooltip="Entire Life"
              style={`--kills: 0;`}
            >
              {#if getLife(pointer).selected}
                <button
                  class="add_demo cancel-btn"
                  on:click={toggleSelected(getLife(pointer))}>-</button
                >
              {:else}
                <div class="add_demo">
                  <button on:click={toggleSelected(getLife(pointer))}>
                    +
                  </button>
                </div>
              {/if}
            </div>

            <div
              class="add_demo tooltip tooltip--left"
              data-tooltip="As Bookmark"
              style={`--kills: 0;`}
            >
              {#if getKill(pointer).selected}
                <button
                  class="add_demo cancel-btn"
                  on:click={toggleKillsSelected([getKill(pointer)])}>-</button
                >
              {:else}
                <div class="add_demo">
                  <button on:click={toggleKillsSelected([getKill(pointer)])}>
                    +
                  </button>
                </div>
              {/if}
            </div>
          </div>
        </div>
      {/each}
    {/each}
  </div>
{/if}

<style lang="scss">
  .pointer__label {
    margin-top: 0.7rem;
    margin-bottom: 0.5rem;
  }

  .buttons {
    display: flex;
    gap: 0.5rem;
  }

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
      border-radius: 5px;
    }
  }

  .card {
    font-size: small;
    font-family: "Source Code Pro", monospace;
    color: var(--tert-con-text);
    border: 1px solid var(--tert-con);
    border-radius: 8px;
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
