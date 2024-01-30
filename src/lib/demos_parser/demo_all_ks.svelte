<script>
  // @ts-nocheck
  import ClassLogo from "../classlogo.svelte";

  export let killstreaks;
  export let parsed_demo;
  export let limitStringLength;
  export let classConverter;
  export let toggleBookmarkSelected;
  export let getLifeFromKillstreak;
  export let tickToTime;
  export let toggleSelected;
  export let isDemoPov;
</script>

{#if killstreaks.length != 0}
  <h2 class="centered chat__title">All Killstreaks</h2>
  <div class="killstreaks">
    {#each killstreaks as killstreak}
      {#if parsed_demo.data.users[killstreak.kills[0].killer]}
        <div
          class={"demo demo__killstreaks " +
            ((killstreak.selected ||
              killstreak.selected_as_bookmark ||
              getLifeFromKillstreak(killstreak)?.selected) &&
              "demo--selected")}
        >
          <div class="player_classes">
            {#each killstreak.classes as player_class}
              <ClassLogo
                player_class={classConverter(player_class)}
                tooltip={`Kills: ${
                  killstreak.kills.filter(
                    (kill) => kill.killer_class === classConverter(player_class)
                  ).length
                }`}
              />
            {/each}
          </div>
          <a
            href={`#player-${
              parsed_demo.data.users[killstreak.kills[0].killer]?.name
            }`}
            data-tooltip="Jump To Player"
            style="width: 100%; --kills: 0;"
            class={parsed_demo.data.users[killstreak.kills[0].killer]["team"] +
              " tooltip"}
          >
            {limitStringLength(
              parsed_demo.data.users[killstreak.kills[0].killer]?.name ||
                "Name Error",
              16
            )}
          </a>
          <div
            on:click={toggleBookmarkSelected(killstreak, true)}
            on:keydown={toggleBookmarkSelected(killstreak, true)}
            class={`demo__kill-count ` +
              (killstreak.kills.length >= 3 && " killstreak ") +
              (killstreak.kills.length >= 5 && " killstreak--large ") +
              (killstreak.kills.length >= 10 && " killstreak--massive ")}
          >
            Kills: {killstreak.kills.length}
          </div>
          <div class="demo__kills">
            {#each killstreak.kills as kill}
              <div class="demo__kill">
                <ClassLogo player_class={classConverter(kill.killer_class)} />
                killed
                <a
                  href={`#player-${parsed_demo.data.users[kill.victim].name}`}
                  class={parsed_demo.data.users[kill.victim]["team"] +
                    " tooltip"}
                  style="--kills: 0;"
                  data-tooltip="Jump To Player"
                >
                  <ClassLogo player_class={classConverter(kill.victim_class)} />
                  {parsed_demo.data.users[kill.victim].name}
                </a>
                with {kill.weapon}
                {#if kill.crit_type}
                  <span
                    class={["", "killstreak", "killstreak--large"][
                      kill.crit_type
                    ]}
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
                  {kill.tick}
                </span>
              </div>
            {/each}
          </div>
          <div
            class="tooltip"
            style={`--kills: 0;`}
            data-tooltip={`Timecode: ${Math.floor(
              Math.round(killstreak.kills[0].tick / 66) / 60
            )}m ${Math.round(killstreak.kills[0].tick / 66) % 60}s`}
          >
            First: {killstreak.kills[0].tick}
          </div>
          <div
            class="tooltip"
            style={`--kills: 0;`}
            data-tooltip={`Length: ${tickToTime(
              killstreak.kills[killstreak.kills.length - 1].tick -
                killstreak.kills[0].tick
            )}`}
          >
            Last: {killstreak.kills[killstreak.kills.length - 1].tick}
          </div>
          <div class="killstreak__buttons">
            <div
              class="add_demo tooltip tooltip--left"
              data-tooltip="Entire Life"
              style={`--kills: 0;`}
            >
              {#if getLifeFromKillstreak(killstreak).selected}
                <button
                  class="cancel-btn"
                  on:click={toggleSelected(getLifeFromKillstreak(killstreak))}
                  >-</button
                >
              {:else}
                <button
                  on:click={toggleSelected(getLifeFromKillstreak(killstreak))}
                  >+</button
                >
              {/if}
            </div>
            {#if isDemoPov}
              <div
                class="add_demo tooltip tooltip--left"
                data-tooltip="As Killstreak"
                style={`--kills: 0;`}
              >
                {#if killstreak.selected}
                  <button
                    class="cancel-btn"
                    on:click={toggleSelected(killstreak, true)}>-</button
                  >
                {:else}
                  <button on:click={toggleSelected(killstreak, true)}>+</button>
                {/if}
              </div>
            {/if}
            <div
              class="add_demo tooltip tooltip--left"
              data-tooltip="As Bookmarks"
              style={`--kills: 0;`}
            >
              {#if killstreak.selected_as_bookmark}
                <button
                  class="cancel-btn"
                  on:click={toggleBookmarkSelected(killstreak, true)}>-</button
                >
              {:else}
                <button on:click={toggleBookmarkSelected(killstreak, true)}
                  >+</button
                >
              {/if}
            </div>
          </div>
        </div>
      {/if}
    {/each}
  </div>
{/if}

<style lang="scss">
  .demo {
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
      grid-template-columns: 0.5fr 1fr 1fr 1fr min-content;
    }

    &__killstreaks {
      grid-template-columns: 0.3fr 1fr 1fr 1fr 1fr min-content;
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
      padding-left: 0.5rem;
      margin-right: 0.5rem;
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
      display: flex;
      gap: 1rem;
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
