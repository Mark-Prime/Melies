<script>
  import ClassLogo from "$lib/components/ClassLogo.svelte";
  import Toggle from "$lib/components/ToggleSelected.svelte";
  import { writeText } from '@tauri-apps/plugin-clipboard-manager';

  export let classConverter;
  export let toggleSelected;
  export let steamid64;
  export let parsedDemo;
  export let tickToTime;
  export let ksPointer;
  export let toggleBookmarkSelected;
  export let isPovDemo;

  function getLife() {
    return parsedDemo.data.player_lives[ksPointer.owner_id][
      ksPointer.life_index
    ];
  }

  function getKills() {
    if (!ksPointer?.kills) {
      return [];
    }

    let kills = [];

    ksPointer.kills?.forEach((element) => {
      kills.push(getLife().kills[element]);
    });

    return kills;
  }
</script>

<div
  class={"demo demo__killstreak " +
    ((ksPointer.selected ||
      ksPointer.selected_as_bookmark ||
      getLife().selected) &&
      "demo--selected")}
>
  <div class="player_classes">
    {#each getLife().classes as playerClass}
      <ClassLogo
        player_class={classConverter(playerClass)}
        tooltip={`Kills: ${
          getKills().filter(
            (kill) => kill.killer_class === classConverter(playerClass)
          ).length
        }`}
      />
    {/each}
  </div>
  <button
    on:click={toggleBookmarkSelected(ksPointer)}
    on:keydown={toggleBookmarkSelected(ksPointer)}
    on:keyup={() => {}}
    tabindex="-1"
    aria-disabled="true"
    class={`demo__kill-count ` +
      (getKills().length >= 3 && " killstreak ") +
      (getKills().length >= 5 && " killstreak--large ") +
      (getKills().length >= 10 && " killstreak--massive ")}
  >
    Kills: {getKills().length}
  </button>
  <div class="demo__kills">
    {#each getKills() as kill}
      <div class="demo__kill">
        <ClassLogo player_class={classConverter(kill.killer_class)} />
        killed
        <a
          href={`#player-${parsedDemo.data.users[kill.victim].name}`}
          class={parsedDemo.data.users[kill.victim]["team"] + " tooltip"}
          style="--kills: 0;"
          data-tooltip="Jump To Player"
        >
          <ClassLogo player_class={classConverter(kill.victim_class)} />
          {parsedDemo.data.users[kill.victim].name}
        </a>
        with {kill.weapon}
        {#if kill.crit_type}
          <span class={["", "killstreak", "killstreak--large"][kill.crit_type]}>
            {["", " (Mini-Crit) ", " (CRITICAL HIT!) "][kill.crit_type]}
          </span>
        {/if}
        at
        <span
          class="tooltip"
          style={`--kills: 0;`}
          data-tooltip={`Timecode: ${tickToTime(kill.tick)}`}
        >
          <button
            class="tick"
            on:click={() => writeText(`demo_gototick ${kill.tick}; wait 10; spec_player ${steamid64}`)}
          >
            {kill.tick}
          </button>
        </span>
      </div>
    {/each}
  </div>
  <div
    class="tooltip"
    style={`--kills: 0;`}
    data-tooltip={`Timecode: ${Math.floor(
      Math.round(getKills()[0].tick / 66) / 60
    )}m ${Math.round(getKills()[0].tick / 66) % 60}s`}
  >
    <button
      class="tick"
      on:click={() => writeText(`demo_gototick ${getKills()[0].tick}; wait 10; spec_player ${steamid64}`)}
    >
      First: {getKills()[0].tick}
    </button>
  </div>
  <div
    class="tooltip"
    style={`--kills: 0;`}
    data-tooltip={`Length: ${tickToTime(
      getKills()[getKills().length - 1].tick - getKills()[0].tick
    )}`}
  >
    <button
      class="tick"
      on:click={() => writeText(`demo_gototick ${getKills()[getKills().length - 1].tick}; wait 10; spec_player ${steamid64}`)}
    >
    Last: {getKills()[getKills().length - 1].tick}
    </button>
  </div>
  <div class="killstreak__buttons">
    <Toggle 
      value={parsedDemo.data.player_lives[ksPointer.owner_id][ksPointer.life_index].selected}
      on:click={toggleSelected(getLife())}
      tooltip="Entire Life"
      tooltipDirection="left"
    />
    {#if isPovDemo}
      <Toggle 
        value={ksPointer.selected}
        on:click={toggleSelected(ksPointer, false)}
        tooltip="As Killstreak"
        tooltipDirection="left"
      />
    {/if}
    <Toggle 
      value={ksPointer.selected_as_bookmark}
      on:click={toggleBookmarkSelected(ksPointer, false)}
      tooltip="As Bookmarks"
      tooltipDirection="left"
    />
  </div>
</div>

<style lang="scss">
  .tick {
    all: unset;
    position: relative;
  }

  .demo {
    background-color: var(--bg2);
    font-size: small;
    padding: 0.3rem 0.5rem;
    margin: 2px 0;
    font-family: "Source Code Pro", monospace;
    color: var(--tert-con-text);
    border: 1px solid var(--tert-con);
    border-radius: 5px;

    display: grid;
    grid-template-columns: 1fr 1fr;
    white-space: nowrap;

    transition: all 0.2s;

    &--selected {
      border: 1px solid var(--tert);
    }

    & > div {
      display: flex;
      align-items: center;
      white-space: nowrap;
      padding: 0;
      margin: 0;
    }

    &__kills {
      grid-row: 2;
      grid-column: 1 / 7;
      display: flex;
      flex-direction: column;
      width: 100%;
      transition: all 0.2s;
      border-radius: 3px;
      padding: 0.5rem;

      &:hover {
        background-color: rgba(256, 256, 256, 0.03);

        & .demo__kill {
          opacity: 1;
          display: flex;
          gap: 0.5rem;
          height: 100%;

          & button {
            width: fit-content;
            height: 100%;
            line-height: 0;
          }

          &-text {
            display: flex;
            gap: 0.5rem;
            width: 100%;
            flex-grow: 1;
          }
        }
      }
    }

    &__killstreak {
      grid-template-columns: 0.3fr 1fr 1fr 1fr min-content;
    }

    &__kill-count {
      cursor: pointer;
      transition: all 0.2s;
      background-color: transparent;
      border-radius: 3px 3px 0 0;

      &:hover {
        color: var(--sec);
        background-color: rgba(256, 256, 256, 0.03);

        & + .demo__kills {
          background-color: rgba(256, 256, 256, 0.03);

          & .demo__kill {
            opacity: 1;
            display: flex;
            gap: 0.5rem;
            height: 100%;

            & button {
              width: fit-content;
              height: 100%;
              line-height: 0;
            }

            &-text {
              display: flex;
              gap: 0.5rem;
              width: 100%;
              flex-grow: 1;
            }
          }
        }
      }
    }

    & .demo__kill-count {
      border: unset;
      text-align: left;
      color: inherit;
      filter: unset;
      padding-left: 0.5rem;
      margin-right: 0.5rem;

      &:hover {
        color: var(--sec);
      }
    }

    &__kill {
      width: 100%;
      opacity: 0;
      display: none;
      transition: all 0.2s;
      height: 0;
      padding: 0 0.5rem;

      &:first-of-type {
        margin-top: 0.5rem;
      }

      &:last-of-type {
        margin-bottom: 0.5rem;
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

      span {
        margin: 0;
      }
    }
  }

  .killstreaks {
    max-width: 800px;
    margin: auto;
  }

  .killstreak {
    font-weight: bold;

    &--large {
      font-weight: bolder;
    }

    &--massive {
      font-weight: 900;
      filter: drop-shadow(0px 0px 3px var(--tert));
    }

    &__buttons {
      margin-left: 0.5rem;
      display: flex;
      gap: 0.5rem;
      justify-content: end;
      grid-column: 6;
    }
  }

  .add_demo {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    gap: 1px;

    & > button {
      font-size: 12px;
      padding: 0.3rem 0.7rem;
      margin: 0;
      // height: 100%;
      border-radius: 5px;
      width: fit-content;
    }

    &--disabled {
      opacity: 0.75;
    }
  }
</style>
