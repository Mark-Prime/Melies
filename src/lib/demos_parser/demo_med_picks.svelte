<script>
  import ClassLogo from "$lib/components/ClassLogo.svelte";
  import Toggle from "$lib/components/ToggleSelected.svelte";
  import { writeText } from '@tauri-apps/plugin-clipboard-manager';
  import { classConverter } from "$lib/composables/classConverter";

  /** @type {{lives: any, steamid64: any, label: any, valKey: any, parsedDemo: any, tickToTime: any, toggleKillsSelected: any, toggleSelected: any}} */
  let {
    lives,
    steamid64,
    label,
    valKey,
    parsedDemo,
    tickToTime,
    toggleKillsSelected,
    toggleSelected
  } = $props();

  function getKill(pointer) {
    return parsedDemo.data.player_lives[pointer.owner_id][pointer.life_index]
      .kills[pointer.kill_index];
  }

  function getLife(pointer) {
    return parsedDemo.data.player_lives[pointer.owner_id][pointer.life_index];
  }
</script>

{#if lives.length > 0}
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
              href={`#player-${parsedDemo.data.users[getKill(pointer).victim].name}`}
              class={parsedDemo.data.users[getKill(pointer).victim]["team"] +
                " tooltip"}
              style="--kills: 0;"
              data-tooltip="Jump To Player"
            >
              <ClassLogo
                player_class={classConverter(getKill(pointer).victim_class)}
              />
              {parsedDemo.data.users[getKill(pointer).victim].name}
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
              <button
                class="tick"
                onclick={() => writeText(`demo_gototick ${getKill(pointer).tick}; wait 10; spec_player ${steamid64}`)}
              >
                {getKill(pointer).tick}
              </button>
            </span>
          </div>

          <div class="buttons">
            <div
              class="add_demo tooltip tooltip--left"
              data-tooltip="Entire Life"
              style={`--kills: 0;`}
            >
              <Toggle value={getLife(pointer).selected} on:click={() => toggleSelected(getLife(pointer))} />
            </div>

            <div
              class="add_demo tooltip tooltip--left"
              data-tooltip="As Bookmark"
              style={`--kills: 0;`}
            >
              <Toggle value={getKill(pointer).selected} on:click={() => toggleKillsSelected([getKill(pointer)])} />
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
  
  .tick {
    all: unset;
    position: relative;
  }

  .buttons {
    margin-left: 0.5rem;
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
