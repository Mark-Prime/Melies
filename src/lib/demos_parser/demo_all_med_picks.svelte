<script>
  import ClassLogo from "../classlogo.svelte";

  export let label;
  export let kills;
  export let toggleSelected;
  export let classConverter;
  export let parsed_demo;
  export let tickToTime;
  export let toggleKillsSelected;

  function getKill(pointer) {
    if (parsed_demo.data.player_lives[pointer.owner_id][pointer.life_index]) {
      return parsed_demo.data.player_lives[pointer.owner_id][pointer.life_index]
        .kills[pointer.kill_index];
    }

    return {
      tick: 0,
    };
  }

  function getLife(pointer) {
    return parsed_demo.data.player_lives[pointer.owner_id][pointer.life_index];
  }
</script>

{#if kills.length > 0}
  <div class="kill_container">
    <h2 class="centered chat__title">All {label}</h2>
    <div class="killstreaks card demo__kills">
      {#each kills.sort((a, b) => getKill(a).tick - getKill(b).tick) as pointer}
        {#if getKill(pointer).tick !== 0}
          <div class="demo__kill">
            <div class="demo__kill-text">
              <a
                href={`#player-${parsed_demo.data.users[getKill(pointer).killer].name}`}
                class={parsed_demo.data.users[getKill(pointer).killer]["team"] +
                  " tooltip"}
                style="--kills: 0;"
                data-tooltip="Jump To Player"
              >
                <ClassLogo
                  player_class={classConverter(getKill(pointer).killer_class)}
                />
                {parsed_demo.data.users[getKill(pointer).killer].name}
              </a>
              killed
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
        {/if}
      {/each}
    </div>
  </div>
{/if}

<style lang="scss">
  .buttons {
    margin-left: 0.5rem;
    display: flex;
    gap: 0.5rem;
  }

  @media (min-width: 1500px) {
    .kill_container:nth-child(odd):last-child {
      grid-column: span 2;
    }

    .killstreaks {
      max-width: 800px;
      margin: auto;
    }
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
    background-color: var(--bg2);
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
