<script>
  import ClassLogo from "$lib/components/ClassLogo.svelte";
  import Toggle from "$lib/components/ToggleSelected.svelte";
  import { writeText } from '@tauri-apps/plugin-clipboard-manager';

  export let killstreaks;
  export let parsedDemo;
  export let limitStringLength;
  export let classConverter;
  export let toggleBookmarkSelected;
  export let tickToTime;
  export let toggleSelected;

  function getLife(index) {
    let ksPointer = killstreaks[index];

    return parsedDemo.data.player_lives[ksPointer.owner_id][
      ksPointer.life_index
    ];
  }

  function getKills(index) {
    let ksPointer = killstreaks[index];

    if (!ksPointer.kills) {
      return [];
    }

    let kills = [];

    ksPointer.kills?.forEach((element) => {
      kills.push(getLife(index).kills[element]);
    });

    return kills;
  }

  function getKs(index) {
    let ksPointer = killstreaks[index];

    return getLife(index).killstreak_pointers[ksPointer.index];
  }
</script>

{#if killstreaks.length != 0}
  <div class="kill_container">
    <h2 class="centered chat__title">All Killstreaks</h2>
    <div class="killstreaks">
      {#each killstreaks as ksPointer, index}
        {#if parsedDemo.data.users[ksPointer.owner_id]}
          <div
            class={"demo demo__killstreaks " +
              ((parsedDemo.data.player_lives[ksPointer.owner_id][
                ksPointer.life_index
              ].killstreak_pointers[ksPointer.index].selected ||
                parsedDemo.data.player_lives[ksPointer.owner_id][
                  ksPointer.life_index
                ].killstreak_pointers[ksPointer.index].selected_as_bookmark ||
                parsedDemo.data.player_lives[ksPointer.owner_id][
                  ksPointer.life_index
                ].selected) &&
                "demo--selected")}
          >
            <div class="player_classes">
              {#each getLife(index).classes as player_class}
                <ClassLogo
                  player_class={classConverter(player_class)}
                  tooltip={`Kills: ${
                    getKills(index).filter(
                      (kill) =>
                        kill.killer_class === classConverter(player_class)
                    ).length
                  }`}
                />
              {/each}
            </div>
            <a
              href={`#player-${
                parsedDemo.data.users[ksPointer.owner_id]?.name
              }`}
              data-tooltip="Jump To Player"
              style="width: 100%;"
              class={parsedDemo.data.users[ksPointer.owner_id]["team"] +
                " tooltip"}
            >
              {limitStringLength(
                parsedDemo.data.users[ksPointer.owner_id]?.name ||
                  "Name Error",
                16
              )}
            </a>
            <button
              on:click={toggleBookmarkSelected(getKs(index), true)}
              on:keydown={toggleBookmarkSelected(getKs(index), true)}
              on:keyup={() => {}}
              class={`demo__kill-count ` +
                (ksPointer.kills.length >= 3 && " killstreak ") +
                (ksPointer.kills.length >= 5 && " killstreak--large ") +
                (ksPointer.kills.length >= 10 && " killstreak--massive ")}
            >
              Kills: {ksPointer.kills.length}
            </button>
            <div class="demo__kills">
              {#each getKills(index) as kill, i}
                <div class="demo__kill">
                  <ClassLogo player_class={classConverter(kill.killer_class)} />
                  killed
                  <a
                    href={`#player-${parsedDemo.data.users[kill.victim].name}`}
                    class={parsedDemo.data.users[kill.victim]["team"] +
                      " tooltip"}
                    style="--kills: 0;"
                    data-tooltip="Jump To Player"
                  >
                    <ClassLogo
                      player_class={classConverter(kill.victim_class)}
                    />
                    {parsedDemo.data.users[kill.victim].name}
                  </a>
                  with {kill.weapon}
                  {#if kill.crit_type}
                    <span
                      class={["", "killstreak", "killstreak--large"][
                        kill.crit_type
                      ]}
                    >
                      {["", " (Mini-Crit) ", " (CRITICAL HIT!) "][
                        kill.crit_type
                      ]}
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
                      on:click={() => writeText(`demo_gototick ${kill.tick}; wait 10; spec_player ${parsedDemo.data.users[ksPointer.owner_id].steamId64}`)}
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
                Math.round(getKills(index)[0].tick / 66) / 60
              )}m ${Math.round(getKills(index)[0].tick / 66) % 60}s`}
            >
              <button
                class="tick"
                on:click={() => writeText(`demo_gototick ${getKills(index)[0].tick}; wait 10; spec_player ${parsedDemo.data.users[getKills(index)[0].killer].steamId64}`)}
              >
                First: {getKills(index)[0].tick}
              </button>
            </div>
            <div
              class="tooltip"
              style={`--kills: 0;`}
              data-tooltip={`Length: ${tickToTime(
                getKills(index)[getKills(index).length - 1].tick -
                  getKills(index)[0].tick
              )}`}
            >
              <button
                class="tick"
                on:click={() => writeText(`demo_gototick ${getKills(index)[getKills(index).length - 1].tick}; wait 10; spec_player ${parsedDemo.data.users[getKills(index)[getKills(index).length - 1].killer].steamId64}`)}
              >
                Last: {getKills(index)[getKills(index).length - 1].tick}
              </button>
            </div>
            <div class="killstreak__buttons">
              <Toggle 
                value={parsedDemo.data.player_lives[ksPointer.owner_id][ksPointer.life_index].selected} 
                on:click={toggleSelected(getLife(index))} 
                tooltip="Entire Life" 
                tooltipDirection="left"
              />
              <Toggle 
                value={parsedDemo.data.player_lives[ksPointer.owner_id][ksPointer.life_index].killstreak_pointers[ksPointer.index].selected_as_bookmark} 
                on:click={toggleBookmarkSelected(getKs(index), true)} 
                tooltip="As Bookmarks"
                tooltipDirection="left"
              />
            </div>
          </div>
        {/if}
      {/each}
    </div>
  </div>
{/if}

<style lang="scss">
  .tick {
    all: unset;
    position: relative;
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

  .demo {
    background-color: var(--bg2);
    font-size: small;
    padding: 0rem 0.5rem;
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
