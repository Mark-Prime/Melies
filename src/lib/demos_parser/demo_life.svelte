<script>
  import ClassLogo from "$lib/components/ClassLogo.svelte";
  import Toggle from "$lib/components/ToggleSelected.svelte";
  import { writeText } from '@tauri-apps/plugin-clipboard-manager';

  export let life;
  export let steamid64;
  export let classConverter;
  export let toggleSelected;
  export let parsedDemo;
  export let tickToTime;
  export let toggleKillsSelected;
  export let allKillsSelected;
</script>

<div class={"demo demo__life " + (life.selected && "demo--selected")}>
  <div class="player_classes">
    {#each life.classes as playerClass}
      <ClassLogo
        player_class={classConverter(playerClass)}
        tooltip={`Kills: ${
          life.kills.filter(
            (kill) => kill.killer_class === classConverter(playerClass)
          ).length
        }`}
      />
    {/each}
  </div>
  <button
    on:click={toggleSelected(life)}
    on:keydown={toggleSelected(life)}
    class={"demo__kill-count " +
      (life.kills.length >= 3 && " killstreak ") +
      (life.kills.length >= 5 && " killstreak--large ") +
      (life.kills.length >= 10 && " killstreak--massive ")}
  >
    Kills: {life.kills.length}
  </button>
  <div class="demo__kills">
    {#each life.kills as kill}
      <div class="demo__kill">
        <div class="demo__kill-text">
          <ClassLogo player_class={classConverter(kill.killer_class)} /> killed
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
            <span
              class={["", "killstreak", "killstreak--large"][kill.crit_type]}
            >
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

        <Toggle value={kill.selected} on:click={toggleKillsSelected([kill])} tooltip="As Bookmark" tooltipDirection="left"/>
      </div>
    {/each}
  </div>
  <div
    class={(life.assists.length >= 3 && "killstreak ") +
      (life.assists.length >= 5 && " killstreak--large ") +
      (life.assists.length >= 10 && " killstreak--massive ")}
  >
    Assists: {life.assists.length}
  </div>
  <div
    class="tooltip"
    style={`--kills: 0;`}
    data-tooltip={`Timecode: ${tickToTime(life.start)}`}
  >
    <button
      class="tick"
      on:click={() => writeText(`demo_gototick ${life.start}; wait 10; spec_player ${steamid64}`)}
    >
      Start: {life.start}
    </button>
  </div>
  <div
    class="tooltip"
    style={`--kills: 0;`}
    data-tooltip={`Length: ${tickToTime(life.end - life.start)}`}
  >
    <button
      class="tick"
      on:click={() => writeText(`demo_gototick ${life.end}; wait 10; spec_player ${steamid64}`)}
    >
      End: {life.end}
    </button>
  </div>
  <div class="killstreak__buttons">
    <Toggle value={life.selected} on:click={toggleSelected(life)} tooltip="Entire Life" tooltipDirection="left"/>
    <div class="add_demo">
      {#if life.kills.length == 0}
        <div
          class="add_demo add_demo--disabled tooltip tooltip--left"
          data-tooltip="Toggle Kills as Bookmarks"
          style={`--kills: 0;`}
        >
          <button disabled>+</button>
        </div>
      {:else if allKillsSelected(life)}
        <div
          class="add_demo tooltip tooltip--left"
          data-tooltip="Toggle Kills as Bookmarks"
          style={`--kills: 0;`}
        >
          <button
            class="auto-height cancel-btn"
            on:click={toggleKillsSelected(life.kills)}
          >
            -
          </button>
        </div>
      {:else}
        <div
          class="add_demo tooltip tooltip--left"
          data-tooltip="Toggle Kills as Bookmarks"
          style={`--kills: 0;`}
        >
          <button on:click={toggleKillsSelected(life.kills)}>+</button>
        </div>
      {/if}
    </div>
  </div>
</div>

<style lang="scss">
  :global(.player_classes) {
    display: flex;
    gap: 0.3rem;
  }

  .killstreak__buttons {
    display: flex;
    gap: 0.5rem;
    justify-content: end;
    grid-column: 6;
  }
  
  .tick {
    all: unset;
    position: relative;
  }

  .demo {
    font-size: small;
    padding: 0.3rem 0.5rem;
    margin: 2px 0;
    font-family: "Source Code Pro", monospace;
    color: var(--tert-con-text);
    border: 1px solid var(--tert-con);
    border-radius: 5px;
    background-color: var(--bg2);

    display: grid;
    grid-template-columns: 1fr 1fr;
    white-space: nowrap;

    transition: all 0.2s;

    &__life {
      display: grid;
      grid-template-columns: 0.5fr 1fr 1fr 1fr 1fr min-content;
      height: auto;
      max-height: 100%;
      transition: all 0.2s;

      &:hover {
        max-height: 100%;
      }
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

    &--selected {
      border: 1px solid var(--tert);
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
      padding-top: 0.3rem;
      margin-right: 0.5rem;

      &:hover {
        color: var(--sec);
      }
    }

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

    &:hover {
      border: 1px solid var(--tert-con-text);
      max-height: 100vh;
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
