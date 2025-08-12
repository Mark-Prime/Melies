<script>
  import { run } from 'svelte/legacy';
  import { untrack } from 'svelte'

  // @ts-nocheck
  import { useMousePosition } from "@svelteuidev/composables";
  import Slider2 from "./timeline_slider.svelte";
  import ClassLogo from "$lib/components/ClassLogo.svelte";
  import { onMount } from "svelte";
  import { median } from "mathjs";

  /** @type {{parsedDemo: any, tickToTime: any, displayPlayer: any, toggleSelected: any, displayLives: any, displayAssists: any, getTeam: any}} */
  let {
    parsedDemo,
    tickToTime,
    displayPlayer,
    toggleSelected,
    displayLives,
    displayAssists,
    getTeam
  } = $props();

  let maxScale = $state(1.0);
  let timeline = $state();
  let left = $state(0);
  let right = $state(1);
  let leftPos = $derived((left / 100) * totalTicks);
  let rightPos = $derived((right / 100) * totalTicks);
  let width = $derived(rightPos - leftPos);
  let minimum = $derived(Math.max(1, (divWidth / totalTicks) * 100));

  let scalePerc = $derived((right - left) / 100);
  let scale = $derived(scalePerc * maxScale);

  let divWidth = $state();
  let divHeight = $state();
  let oldDivWidth = $state();
  let startTick = parsedDemo.data?.start_tick;
  let totalTicks = parsedDemo?.header?.ticks;

  function getLife(ksPointer) {
    return parsedDemo.data.player_lives[ksPointer.owner_id][
      ksPointer.life_index
    ];
  }

  function getKills(ksPointer, index) {
    if (!ksPointer.kills) {
      return [];
    }

    let kills = [];

    ksPointer.kills?.forEach((element) => {
      kills.push(getLife(ksPointer).kills[element]);
    });

    return kills;
  }

  function calcMaxScale() {
    maxScale = totalTicks / divWidth;
  }

  onMount(() => {
    calcMaxScale();
    right = 100;
  });

  const [global_pos, global_ref] = useMousePosition();
  let global_x = $derived($global_pos.x);

  let isDragging = $state(false);
  let dragTarget = $state("");
  let startingPos = $state(0);
  let relativePos = $state(0);

  let startingLeft = 0;
  let startingRight = 1;

  $effect(() => {
    if (isDragging) {
      let padding = startingPos - relativePos;
      let paddingPerc = relativePos / divWidth;

      let distancePerc = (global_x - padding) / divWidth;

      switch (dragTarget) {
        case "range":
          let difference = right - left;

          untrack(() => left = median(
            0,
            startingLeft + distancePerc * 100 - paddingPerc * 100,
            100 - difference
          ));
          untrack(() => right = median(
            difference,
            startingRight + distancePerc * 100 - paddingPerc * 100,
            100
          ));
          break;
        case "left":
          untrack(() => left = median(
            0,
            startingLeft + distancePerc * 100 - paddingPerc * 100,
            right - minimum
          ));
          break;
        case "right":
          untrack(() => right = median(
            left + minimum,
            startingRight + distancePerc * 100 - paddingPerc * 100,
            100
          ));
          break;
      }
    }
  });

  let is_shift_down = false;

  const [position, ref] = useMousePosition();
  let { x } = $derived($position);

  $effect(() => {
    if (divWidth != oldDivWidth) {
      oldDivWidth = divWidth;
      calcMaxScale();
    }
  });

  function pauseAdjust(tick) {
    if (!parsedDemo.data.pause_tick) {
      return tick;
    }

    if (tick < parsedDemo.data.pause_tick) {
      return tick;
    }

    return tick + parsedDemo.data.pause_length;
  }

  function calcTick(tick) {
    // return pauseAdjust(tick) - startTick
    return tick;
  }

  function isLifeVisible(life) {
    if (life.end < leftPos || life.start > rightPos) {
      return false;
    }

    return true;
  }

  function isMarkerVisible(pos) {
    if (pos < leftPos || pos > rightPos) {
      return false;
    }

    return true;
  }

  function calcTimelineStart(life) {
    let start = Math.max(life.start, leftPos);

    return (calcTick(start) - leftPos) / scale;
  }

  function calcRoundEnd(round) {
    return (calcTick(round.end_tick) - leftPos) * scalePerc;
  }

  function calcTimelineLength(life) {
    let end = Math.min(life.end, rightPos);
    let start = Math.max(life.start, leftPos);

    return (pauseAdjust(end) - pauseAdjust(start)) / scale;
  }

  // --position: ${((life.end - life.start) / scale) - 2}px;

  function calcTimelineMarker(tick, life) {
    let start = Math.max(life.start, leftPos);

    return (tick - start) / scale - 2;
  }

  function on_mouse_down(e) {
    if (!["range", "left", "right"].includes(e.target.classList[0])) {
      return;
    }

    isDragging = true;
    dragTarget = e.target.classList[0];
    startingPos = global_x;
    relativePos = e.layerX;

    startingLeft = left;
    startingRight = right;
  }

  function on_mouse_up(e) {
    isDragging = false;
  }

  function on_key_down(event) {
    if (event.repeat) return;

    switch (event.key) {
      case "Shift":
        is_shift_down = true;

        event.preventDefault();
        break;
    }
  }

  function on_key_up(event) {
    switch (event.key) {
      case "Shift":
        is_shift_down = false;

        event.preventDefault();
        break;
    }
  }

  function timelineMousewheel(event) {
    event.preventDefault();

    if (!is_shift_down) {
      let difference = right - left;
      let delta = event.deltaY / 200;

      left = median(0, left + delta, 100 - difference);
      right = median(difference, right + delta, 100);
    } else {
      let deltaS = event.deltaY / 200;

      let mouse_loc = 0.5;

      if (event.deltaY < 0) {
        mouse_loc = x / divWidth;
      }

      if (left + deltaS < 0 || right + (1 - deltaS) > 100) {
        mouse_loc = mouse_loc * 2;
      }

      left = median(0, left - deltaS * mouse_loc, right - minimum);
      right = median(left + minimum, right + deltaS * (1 - mouse_loc), 100);
    }
  }
</script>

<svelte:window
  onkeydown={on_key_down}
  onkeyup={on_key_up}
  onmousedown={on_mouse_down}
  onmouseup={on_mouse_up}
/>

<h2 class="centered chat__title">Timeline</h2>
<div class="timeline" use:global_ref>
  <div class="timeline__labels">
    {#each ["blue", "red"] as team}
      {#each getTeam(team) as player}
        {#if displayPlayer(player, team)}
          <a
            class={`timeline__label tooltip tooltip--align-right ${team}`}
            href={`#player-${parsedDemo.data?.users[player]?.name}`}
            style={`--kills: 0`}
            data-tooltip="Jump To Player"
          >
            {parsedDemo.data?.users[player]?.name}
          </a>
        {/if}
      {/each}
    {/each}
  </div>
  <div
    class="timeline__lives-container"
    bind:clientWidth={divWidth}
    bind:clientHeight={divHeight}
    bind:this={timeline}
    use:ref
    onwheel={timelineMousewheel}
  >
    {#each ["blue", "red"] as team}
      {#each getTeam(team) as player}
        {#if displayPlayer(player, team)}
          <div class="timeline__lives">
            {#each parsedDemo.data?.player_lives[player] as life, index}
              {#if life.start != 0 && (displayLives || life.kills.length > 0 || (displayAssists && life.assists.length > 0)) && isLifeVisible(life, leftPos, rightPos)}
                <div
                  class={`timeline__life timeline__life--${team} ${
                    life.selected ? "timeline--selected" : ""
                  }`}
                  role="button"
                  aria-pressed="false"
                  tabindex={index}
                  onclick={() => toggleSelected(life)}
                  onkeydown={() => toggleSelected(life)}
                  style={`
                      --length: ${calcTimelineLength(
                        life,
                        leftPos,
                        rightPos
                      )}px;
                      --start: ${calcTimelineStart(life)}px
                  `}
                >
                  <div
                    class={`
                        timeline__data-tooltip tooltip 
                        ${
                          parsedDemo.data?.users[player]?.team == "blue"
                            ? "tooltip__lower"
                            : ""
                        }
                        ${
                          calcTick(life.start) >
                            leftPos + divWidth * scale * 0.7 && "tooltip--left"
                        }
                        ${calcTick(life.start) < leftPos && "tooltip--custom"}
                    `}
                    data-tooltip={`Length: ${tickToTime(
                      life.end - life.start
                    )}\n${
                      life.kills.length
                        ? `Player${life.kills.length > 1 ? "s" : ""} Killed: `
                        : `No Kills`
                    }\n${life.kills
                      .map((kill) => {
                        let crit_types = ["", " Mini-Crit", " CRITICAL HIT!"];
                        return `${
                          parsedDemo.data?.users[kill.victim].name
                        } (tick: ${calcTick(kill.tick)})${
                          crit_types[kill.crit_type]
                        }`;
                      })
                      .join(", \n")}`}
                    style={`
                      --kills: ${life.kills.length + 1};
                      --pos: ${(leftPos + startTick - life.start) / scale}px;
                  `}
                  >
                    <div class="timeline__life--container">
                      <div class="timeline__data">
                        <div class="timeline__icons">
                          {#each life.classes as player_class}
                            <ClassLogo {player_class} />
                          {/each}
                        </div>
                        <div
                          class={(life.kills.length >= 3 && " killstreak ") +
                            (life.kills.length >= 5 && " killstreak--large ") +
                            (life.kills.length >= 10 &&
                              " killstreak--massive ")}
                        >
                          K: {life.kills.length}
                        </div>
                        <div
                          class={(life.assists.length >= 3 && "killstreak ") +
                            (life.assists.length >= 5 &&
                              " killstreak--large ") +
                            (life.assists.length >= 10 &&
                              " killstreak--massive ")}
                        >
                          A: {life.assists.length}
                        </div>
                      </div>
                    </div>
                  </div>
                  {#if isMarkerVisible(life.start, leftPos, rightPos)}
                    <div
                      class={`
                          timeline__marker 
                          ${
                            parsedDemo.data?.users[player]?.team == "blue"
                              ? "timeline__marker--lower"
                              : ""
                          }
                          ${x > divWidth * 0.7 && "timeline__marker--left"}
                      `}
                      data-tooltip={`Start: ${calcTick(
                        life.start
                      )}\nTimecode: ${tickToTime(calcTick(life.start))}`}
                      style={`
                          --position: -1px;
                          --kills: 1;
                      `}
                    >
                      <div class="timeline__marker__text" style="--lines: 2">
                        Start: {calcTick(life.start)} <br />
                        Timecode: {tickToTime(calcTick(life.start))}
                      </div>
                    </div>
                  {/if}
                  {#if isMarkerVisible(life.end, leftPos, rightPos)}
                    <div
                      class={`timeline__marker 
                        ${
                          parsedDemo.data?.users[player]?.team == "blue"
                            ? "timeline__marker--lower"
                            : ""
                        }
                        ${x > divWidth * 0.7 && "timeline__marker--left"}
                      `}
                      data-tooltip={`End: ${calcTick(
                        life.end
                      )}\nTimecode: ${tickToTime(calcTick(life.end))}`}
                      style={`
                        --position: ${calcTimelineMarker(
                          life.end,
                          life,
                          leftPos,
                          rightPos
                        )}px;
                    `}
                    >
                      <div class="timeline__marker__text" style="--lines: 2">
                        End: {calcTick(life.end)} <br />
                        Timecode: {tickToTime(calcTick(life.end))}
                      </div>
                    </div>
                  {/if}
                  {#each life.kills as kill}
                    {#if isMarkerVisible(kill.tick, leftPos, rightPos)}
                      <div
                        class={`timeline__marker 
                            ${
                              parsedDemo.data?.users[player]?.team == "blue"
                                ? "timeline__marker--lower"
                                : ""
                            }
                            ${x > divWidth * 0.7 && "timeline__marker--left"}
                        `}
                        data-tooltip={`Killed: ${
                          parsedDemo.data?.users[kill.victim].name
                        }\nTick: ${calcTick(
                          kill.tick
                        )}\nTimecode: ${tickToTime(calcTick(kill.tick))}`}
                        style={`
                            --position: ${calcTimelineMarker(
                              kill.tick,
                              life,
                              leftPos,
                              rightPos
                            )}px;
                        `}
                      >
                        <div class="timeline__marker__text" style="--lines: 4">
                          <div>
                            Killed:
                            <ClassLogo
                              player_class={parsedDemo.data?.users[
                                kill.victim_class
                              ]}
                            />
                            {parsedDemo.data?.users[kill.victim].name} ({kill.victim_class})
                          </div>
                          Weapon: {kill.weapon} <br />
                          Tick: {calcTick(kill.tick)} <br />
                          Timecode: {tickToTime(calcTick(kill.tick))}
                        </div>
                      </div>
                    {/if}
                  {/each}
                  {#each life.killstreak_pointers as ksPointer, index}
                    <div
                      class={`timeline__ks
                          ${
                            parsedDemo.data?.users[player]?.team == "blue"
                              ? "timeline__ks--lower"
                              : ""
                          }
                          ${x > divWidth * 0.7 && "timeline__ks--left"}
                      `}
                      data-tooltip={`${`Players Killed in Killstreak: `}\n${getKills(
                        ksPointer,
                        index
                      )
                        .map((kill) => {
                          let crit_types = ["", " Mini-Crit", " CRITICAL HIT!"];
                          return `${
                            parsedDemo.data?.users[kill.victim].name
                          } (tick: ${calcTick(kill.tick)})${
                            crit_types[kill.crit_type]
                          }`;
                        })
                        .join(", \n")}`}
                      style={`
                        --position: ${Math.max(
                          calcTimelineMarker(
                            getKills(ksPointer, index)[0].tick,
                            life,
                            leftPos,
                            rightPos
                          ),
                          0
                        )}px;
                        --kills: ${getKills(ksPointer, index).length};
                        --length: ${
                          (getKills(ksPointer, index)[
                            getKills(ksPointer, index).length - 1
                          ].tick -
                            life.start) /
                            scale -
                          (getKills(ksPointer, index)[0].tick - life.start) /
                            scale +
                          (calcTimelineMarker(
                            getKills(ksPointer, index)[0].tick,
                            life,
                            leftPos,
                            rightPos
                          ) < 0
                            ? calcTimelineMarker(
                                getKills(ksPointer, index)[0].tick,
                                life,
                                leftPos,
                                rightPos
                              )
                            : 0)
                        }px;
                    `}
                    ></div>
                  {/each}
                </div>
              {/if}
            {/each}
          </div>
        {/if}
      {/each}
    {/each}
    <div
      class="timeline__rounds"
      style={`width: ${divWidth + "px"}; height: ${divHeight - 3 + "px"}`}
    >
      {#each parsedDemo.data?.rounds as round, index}
        <div
          class="timeline__round"
          style={`
          --start: ${(Math.max(parsedDemo.data?.rounds[index - 1]?.end_tick || 0, leftPos) - leftPos) / scale}px;
          --end: ${(Math.max(round.end_tick, leftPos) - leftPos) / scale}px;
          height: ${divHeight - 3}px;
        `}
        >
          <p class="timeline__round--text">
            { round.label }
          </p>
        </div>
      {/each}
    </div>
  </div>
  <div></div>
  <Slider2 bind:left bind:right />
  <div></div>
  <div class="timeline__states">
    <div>
      Zoom: {Math.round(scalePerc * 100)}%
    </div>
    <div>
      Range: {Math.round(leftPos)} - {Math.round(rightPos)}
    </div>
    <div>
      Mouse Pos: {median(0, Math.round(leftPos + (x / divWidth) * width), totalTicks)}
    </div>
  </div>
</div>

<style lang="scss">
  .timeline__rounds {
    position: absolute;
    top: 0;
    left: 0;
    z-index: -10;
  }

  .timeline__round {
    position: absolute;
    top: 0;
    left: var(--start);
    width: calc(var(--end) - var(--start));
    border: color-mix(in srgb, var(--tert-con) 40%, transparent) 1px dashed;
    border-left: 0;
    border-top: 0;
    border-bottom: 0;
    opacity: 1;
    text-align: center;
    z-index: -10;

    &:nth-of-type(2n) {
      background: color-mix(in srgb, var(--bg) 60%, transparent);
    }

    &--text {
      font-size: smaller;
      margin-top: -0.4rem;
      color: var(--tert-con-text);
      opacity: 0;
      transition: all 0.2s;
    }

    &:hover &--text {
      opacity: 1;
    }
  }

  .chat__title {
    margin-top: 3rem;
  }

  .timeline {
    display: grid;
    grid-template-columns: min-content 1fr;
    text-align: right;
    margin-top: 1rem;
    overflow-y: visible;
    padding: 0.5rem;
    border-radius: 5px;
    border: 1px solid var(--tert-con);
    transition: all 0.2s;

    &__labels {
      padding-top: 1rem;
    }

    &__label {
      white-space: nowrap;
      padding-right: 1rem;
      height: 35px;
      display: flex;
      align-items: center;
      justify-content: end;
      margin-bottom: 0.2rem;
      text-align: right;
      border-right: var(--tert-con) solid 1px;
    }

    &__lives {
      user-select: none;
      height: 35px;
      position: relative;
      width: min-content;
      overflow: visible;
      margin-bottom: 0.2rem;
    }

    &__lives-container {
      // overflow-x: hidden;
      // overflow-y: visible;
      padding-top: 1rem;
      background-color: var(--bg2);
      border-radius: 5px;
      z-index: 1005;
      position: relative;
    }

    &__states {
      width: 100%;
      display: grid;
      grid-template-columns: 1fr 1fr 1fr;
      gap: 2rem;
      text-align: center;
      justify-content: center;
      align-items: center;
    }

    &__life {
      all: unset;
      height: 100%;
      border: 1px solid var(--tert-con);
      text-align: left;
      display: flex;
      gap: 1rem;
      align-items: center;
      width: var(--length);
      position: absolute;
      left: var(--start);
      top: 0;
      white-space: nowrap;
      overflow: visible;
      cursor: pointer;
      transition:
        all 0.2s,
        width 0s,
        left 0s;

      &--red {
        background: linear-gradient(-45deg, #f3535533, transparent);
      }

      &--blue {
        background: linear-gradient(-45deg, #65b1e233, transparent);
      }

      &--container {
        padding: 0 4px;
        overflow: hidden;
      }
    }

    &--selected {
      border: 1px solid var(--tert);

      &.timeline__life--red {
        background: linear-gradient(-45deg, var(--red), transparent);
      }

      &.timeline__life--blue {
        background: linear-gradient(-45deg, var(--blu), transparent);
      }
    }

    &__data-tooltip {
      width: 100%;
      position: relative;
    }

    &__data {
      width: 100%;
      display: grid;
      gap: 1rem;
      grid-template-columns: min-content min-content min-content;
      overflow: hidden;
      z-index: -10;
    }

    &__icons {
      display: flex;
      align-items: center;
      justify-content: baseline;
    }

    &__marker {
      position: absolute;
      top: 0px;
      left: calc(-0.4rem + var(--position));
      height: 37px;
      width: 0.8rem;
      background-color: transparent;
      cursor: pointer;
      overflow: visible;
      transform: scale(1);
      z-index: 999;

      &:nth-child(3) {
        left: calc(-0.2rem + var(--position));
      }

      &__text {
        z-index: 1;
        position: absolute;
        top: calc(-2.2rem - (1.72rem * calc(var(--lines) - 1)));
        left: -0.4rem;
        display: none;
        background-color: var(--bg);
        color: var(--bg-text);
        border: var(--outline) 1px solid;
        padding: 0.2rem 0.5rem;
        border-radius: 0.5rem;
        width: max-content;
        max-width: 500px;
        overflow: hidden;
        // white-space: pre;
        font-size: 12px;
        pointer-events: none;
      }

      &::before {
        z-index: 1;
        content: attr(data-tooltip);
        position: absolute;
        top: calc(-2.2rem - (1.72rem * var(--kills)));
        left: -0.4rem;
        display: none;
        background-color: var(--bg);
        color: var(--bg-text);
        border: var(--outline) 1px solid;
        padding: 0.2rem 0.5rem;
        border-radius: 0.5rem;
        width: max-content;
        max-width: 500px;
        overflow: hidden;
        white-space: pre;
        font-size: 12px;
        pointer-events: none;
      }

      &::after {
        z-index: 999;
        content: "";
        position: absolute;
        top: -1px;
        left: 0;
        height: 37px;
        width: 0.8rem;
        background-color: var(--tert-con);
        clip-path: polygon(40% 100%, 60% 100%, 60% 25%, 100% 0, 0 0, 40% 25%);
      }

      &--lower {
        &::before {
          top: 34px;
          background-color: var(--bg);
        }

        &::after {
          // z-index: 999;
          clip-path: polygon(40% 0, 60% 0, 60% 75%, 100% 100%, 0 100%, 40% 75%);
        }

        & > .timeline__marker__text {
          top: 34px;
          background-color: var(--bg);
        }
      }

      &--left {
        &::before {
          left: auto;
          right: -0.4rem;
        }

        &::after {
          left: auto;
          right: 0;
        }

        & > .timeline__marker__text {
          left: auto;
          right: -0.4rem;
        }
      }

      &:hover,
      &:active,
      &:focus {
        color: var(--sec);
        z-index: 1003;

        &::after {
          display: block;
          background-color: var(--outline);
        }

        & .timeline__marker__text {
          display: block;
          z-index: 1003;
        }
      }
    }

    &__ks {
      position: absolute;
      top: 0;
      left: var(--position);
      height: 3px;
      width: var(--length);
      background-color: transparent;
      cursor: pointer;
      overflow: visible;

      &::before {
        z-index: 1005;
        content: attr(data-tooltip);
        position: absolute;
        top: calc(-2.2rem - (1.72rem * var(--kills)));
        left: 50%;
        display: none;
        background-color: var(--bg);
        color: var(--bg-text);
        border: var(--outline) 1px solid;
        padding: 0.2rem 0.5rem;
        border-radius: 0.5rem;
        width: max-content;
        max-width: 500px;
        overflow: hidden;
        white-space: pre;
        font-size: 12px;
        transform: translateX(-50%);
      }

      &::after {
        z-index: 998;
        content: "";
        position: absolute;
        top: 0;
        left: 0;
        height: 3px;
        width: var(--length);
        background-color: var(--tert-con);
      }

      &--lower {
        top: 30px;

        &::before {
          top: 3px;
          background-color: var(--bg);
          z-index: 1005;
        }
      }

      &--left {
        &::before {
          left: auto;
          right: -0.4rem;
        }

        &::after {
          left: auto;
          right: 0;
        }
      }

      &:hover,
      &:active,
      &:focus {
        color: var(--sec);

        &::before {
          display: block;
        }

        &::after {
          display: block;
          background-color: var(--outline);
        }
      }
    }
  }
</style>
