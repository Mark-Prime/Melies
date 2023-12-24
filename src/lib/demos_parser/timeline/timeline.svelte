<script>
    // @ts-nocheck
    import { useMousePosition } from '@svelteuidev/composables';
    import Slider2 from "./timeline_slider.svelte";
    import ClassLogo from '$lib/classlogo.svelte';
    import { onMount } from 'svelte';
    import { median } from 'mathjs';

    export let parsed_demo;
    export let tickToTime;
    export let displayPlayer;
    export let toggleSelected;
    export let displayLives;
    export let displayAssists;

    let scale = 1.0;
    let scalePerc = 1.0;
    let maxScale = 1.0;
    let timeline;
    let left = 0;
    let right = 1;
    let leftPos = 0;
    let rightPos = 1;
    let width = 1;
    let minimum = 1;

    let divWidth;
    let startTick = parsed_demo.data?.start_tick;
    let totalTicks = parsed_demo?.header?.ticks;

    onMount(() => {
        right = Math.max(1, (divWidth / totalTicks) * 100);
        minimum = right;
        maxScale = totalTicks / divWidth;
        right = 100;
    })

    $: {
        scalePerc = (right - left) / 100;
        scale = scalePerc * maxScale;
        leftPos = (left / 100) * totalTicks;
        rightPos = (right / 100) * totalTicks;
        width = rightPos - leftPos;
    }

    const [global_pos, global_ref] = useMousePosition();
	$: (global_x = $global_pos.x);

    let isDragging = false;
    let dragTarget = '';
    let startingPos = 0;
    let relativePos = 0;

    let startingLeft = 0;
    let startingRight = 1;

    $: {
        if (isDragging) {
            let padding = startingPos - relativePos;
            let paddingPerc = relativePos / divWidth;

            let distancePerc = (global_x - padding) / divWidth;

            switch (dragTarget) {
                case "range":
                    let difference = right - left;

                    left = median(0, startingLeft + (distancePerc * 100) - (paddingPerc * 100), 100 - difference);
                    right = median(difference, startingRight + (distancePerc * 100) - (paddingPerc * 100), 100);
                    break;
                case "left":
                    left = median(0, startingLeft + (distancePerc * 100) - (paddingPerc * 100), right - minimum);
                    break;
                case "right":
                    right = median(left + minimum, startingRight + (distancePerc * 100) - (paddingPerc * 100), 100);
                    break;
            }
        }
    }
    
    let is_shift_down = false;

    const [position, ref] = useMousePosition();
	$: ({ x } = $position);

    $: if (scale > totalTicks / divWidth) {
        scale = totalTicks / divWidth
    };

    $: {
        scale = scale < 1 ? 1 : (scale > totalTicks / divWidth ? totalTicks / divWidth : scale) 
    };

    function pauseAdjust(tick) {
        if (!parsed_demo.data.pause_tick) {
            return tick
        }

        if (tick < parsed_demo.data.pause_tick) {
            return tick
        }

        return tick + parsed_demo.data.pause_length
    }

    function calcTick(tick) {
        // return pauseAdjust(tick) - startTick
        return tick;
    }

    function calcTimelineLength(life) {
        return (pauseAdjust(life.end) - pauseAdjust(life.start)) / scale
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
            let delta = event.deltaY/200;

            left = median(0, left + delta, 100 - difference);
            right = median(difference, right + delta, 100);
        } else {
            let deltaS = event.deltaY/200;

            let mouse_loc = .5;

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
    on:keydown={on_key_down}
    on:keyup={on_key_up}
    on:mousedown={on_mouse_down}
    on:mouseup={on_mouse_up}
/>

<h2 class="centered chat__title">Timeline</h2>
<div class="timeline" use:global_ref>
    <div class="timeline__labels">
        {#each ["blue", "red"] as team}
            {#each Object.keys(parsed_demo.data?.player_lives) as player}
                {#if displayPlayer(player) & parsed_demo.data?.users[player]?.team === team}
                    <a 
                        class={`timeline__label tooltip tooltip--align-right ${team}`}
                        href={`#player-${parsed_demo.data?.users[player]?.name}`} 
                        style={`--kills: 0`}
                        data-tooltip="Jump To Player"
                    >
                        {parsed_demo.data?.users[player]?.name}
                    </a>
                {/if}
            {/each}
        {/each}
    </div>
    <div class="timeline__lives-container" 
        bind:clientWidth={divWidth}
        bind:this={timeline} 
        use:ref
        on:wheel={timelineMousewheel}
    >
        {#each ["blue", "red"] as team}
            {#each Object.keys(parsed_demo.data?.player_lives) as player}
                {#if displayPlayer(player) & parsed_demo.data?.users[player]?.team === team}
                    <div class="timeline__lives">
                        {#each parsed_demo.data?.player_lives[player] as life}
                            {#if life.start != 0 && 
                                (displayLives || life.kills.length > 0 || (displayAssists && life.assists.length > 0))
                            }
                                <div class={`timeline__life timeline__life--${team} ${(life.selected ? "timeline--selected" : "")}`} on:click={toggleSelected(life)} on:keydown={toggleSelected(life)} style={`
                                        --length: ${calcTimelineLength(life)}px;
                                        --start: ${((calcTick(life.start) - leftPos) / scale)}px
                                    `}
                                >
                                    <div
                                        class={`
                                            timeline__data-tooltip tooltip 
                                            ${parsed_demo.data?.users[player]?.team == "blue" ? "tooltip__lower" : ""}
                                            ${calcTick(life.start) > leftPos + ((divWidth * scale) * .7) && "tooltip--left"}
                                            ${calcTick(life.start) < leftPos && "tooltip--custom"}
                                        ` }
                                        data-tooltip={`Length: ${tickToTime(life.end - life.start)}\n${
                                            life.kills.length ? 
                                            `Player${(life.kills.length > 1) ? "s" : ""} Killed: ` :
                                            `No Kills`
                                        }\n\r${life.kills.map((kill) => {
                                            let crit_types = ["", " Mini-Crit", " CRITICAL HIT!"]
                                            return `${parsed_demo.data?.users[kill.victim].name} (tick: ${calcTick(kill.tick)})${crit_types[kill.crit_type]}`
                                        }).join(", \n\r")}`}
                                        style={`
                                            --kills: ${life.kills.length + 1};
                                            --pos: ${((leftPos + startTick) - life.start) / scale}px;
                                        `}
                                    >
                                        <div class="timeline__data">
                                            <div class="timeline__icons">
                                                {#each life.classes as player_class}
                                                    <ClassLogo 
                                                        player_class={player_class} 
                                                    />
                                                {/each}
                                            </div>
                                            <div
                                                class={
                                                    (life.kills.length >= 3 && " killstreak ") +
                                                    (life.kills.length >= 5 && " killstreak--large ") +
                                                    (life.kills.length >= 10 && " killstreak--massive ")
                                                }
                                            >
                                                K: {life.kills.length}
                                            </div>
                                            <div
                                                class={
                                                    (life.assists.length >= 3 && "killstreak ") +
                                                    (life.assists.length >= 5 && " killstreak--large ") +
                                                    (life.assists.length >= 10 && " killstreak--massive ")
                                                }
                                            >
                                                A: {life.assists.length}
                                            </div>
                                        </div>
                                    </div>
                                    <div 
                                        class={`
                                            timeline__marker 
                                            ${parsed_demo.data?.users[player]?.team == "blue" ? "timeline__marker--lower" : ""}
                                            ${(x > divWidth * .7) && "timeline__marker--left"}
                                        `}
                                        data-tooltip={`Start: ${calcTick(life.start)}\r\nTimecode: ${tickToTime(calcTick(life.start))}`}
                                        style={`
                                            --position: -1px;
                                            --kills: 1;
                                        `}
                                    ></div>
                                    <div 
                                        class={`
                                            timeline__marker 
                                            ${parsed_demo.data?.users[player]?.team == "blue" ? "timeline__marker--lower" : ""}
                                            ${(x > divWidth * .7) && "timeline__marker--left"}
                                        `}
                                        data-tooltip={`End: ${calcTick(life.end)}\r\nTimecode: ${tickToTime(calcTick(life.end))}`}
                                        style={`
                                            --position: ${((life.end - life.start) / scale) - 2}px;
                                            --kills: 1;
                                        `}
                                    ></div>
                                    {#each life.kills as kill} 
                                        <div 
                                            class={`timeline__marker 
                                                ${parsed_demo.data?.users[player]?.team == "blue" ? "timeline__marker--lower" : ""}
                                                ${(x > divWidth * .7) && "timeline__marker--left"}
                                            `}
                                            data-tooltip={`Killed: ${parsed_demo.data?.users[kill.victim].name}\r\nTick: ${calcTick(kill.tick)}\r\nTimecode: ${tickToTime(calcTick(kill.tick))}`}
                                            style={`
                                                --position: ${((kill.tick - life.start) / scale) - 2}px;
                                                --kills: 2;
                                            `}
                                        ></div>
                                    {/each}
                                    {#each life.killstreaks as ks} 
                                        <div 
                                            class={`timeline__ks
                                                ${parsed_demo.data?.users[player]?.team == "blue" ? "timeline__ks--lower" : ""}
                                                ${(x > divWidth * .7) && "timeline__ks--left"}
                                            `}
                                            data-tooltip={`${
                                                `Players Killed in Killstreak: ` 
                                            }\n\r${ks.kills.map((kill) => {
                                                let crit_types = ["", " Mini-Crit", " CRITICAL HIT!"]
                                                return `${parsed_demo.data?.users[kill.victim].name} (tick: ${calcTick(kill.tick)})${crit_types[kill.crit_type]}`
                                            }).join(", \n\r")}`}
                                            style={`
                                                --position: ${((ks.kills[0].tick - life.start) / scale) - 2}px;
                                                --kills: ${ks.kills.length};
                                                --length: ${((ks.kills[ks.kills.length - 1].tick - life.start) / scale) - ((ks.kills[0].tick - life.start) / scale)}px;
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
    </div>
    <div></div>
    <Slider2 left={left} right={right} />
    <div></div>
    <div class="timeline__states">
        <div>
            Zoom: {Math.round(scalePerc * 100)}% 
        </div>
        <div>
            Range: {Math.round(leftPos)} - {Math.round(rightPos)}
        </div>
        <div>
            Pos: {median(
                0,
                Math.round(leftPos + (x / divWidth) * width),
                totalTicks
            )}
        </div>
    </div>
</div>

<style lang="scss">
    .timeline {
        display: grid;
        grid-template-columns: min-content 1fr;
        text-align: right;
        margin-top: 3rem;
        overflow-y: visible;
        padding: .5rem;
        border-radius: 5px;
        border: 1px solid var(--tert-con);
        transition: all .2s;

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
            margin-bottom: .2rem;
            text-align: right;
            border-right: var(--tert-con) solid 1px;
        }

        &__lives {
            user-select: none;
            height: 35px;
            position: relative;
            width: min-content;
            overflow: visible;
            margin-bottom: .2rem;
        }

        &__lives-container {
            overflow-x: hidden;
            overflow-y: visible;
            padding-top: 1rem;
            background-color: var(--bg2);
            border-radius: 5px;

            /* width */
            &::-webkit-scrollbar {
                width: 16px;
            }

            /* Track */
            &::-webkit-scrollbar-track {
                background: var(--tert);
            }

            /* Handle */
            &::-webkit-scrollbar-thumb {
                background: var(--tert-con);
            }
        }

        &__states {
            width: 100%;
            display: grid;
            grid-template-columns: 1fr 1fr 1fr;
            gap: 1rem;
            text-align: left;
        }

        &__life {
            height: 100%;
            border: 1px solid var(--tert-con);
            text-align: left;
            display: flex;
            gap: 1rem;
            align-items: center;
            padding: 0 4px;
            width: var(--length);
            position: absolute;
            left: var(--start);
            top: 0;
            white-space: nowrap;
            overflow: visible;
            cursor: pointer;
            transition: all .2s, width 0s, left 0s;

            &--red {
                background: linear-gradient(-45deg, #f3535533, transparent);
            }

            &--blue {
                background: linear-gradient(-45deg, #65b1e233, transparent);
            }
        }

        &--selected {
            border: 1px solid var(--tert);
        }

        &--selected.timeline__life--red {
                background: linear-gradient(-45deg, var(--red), transparent);
        }

        &--selected.timeline__life--blue {
                background: linear-gradient(-45deg, var(--blu), transparent);
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
            z-index: 10000;
        }

        &__icons {
            display: flex;
            align-items: center;
            justify-content: baseline;
        }

        &__marker {
            position: absolute;
            top: -1px;
            left: calc(-.4rem + var(--position));
            height: 37px;
            width: .8rem;
            background-color: transparent;
            cursor: pointer;
            overflow: visible;

            &::before {
                z-index: 1001;
                content: attr(data-tooltip);
                position: absolute;
                top: calc(-2.2rem - (1.72rem * var(--kills)));
                left: -.4rem;
                display: none;
                background-color: var(--bg);
                color: var(--bg-text);
                border: var(--outline) 1px solid;
                padding: .2rem .5rem;
                border-radius: .5rem;
                width: max-content;
                max-width: 500px;
                overflow: hidden;
                white-space: pre;
                font-size: 12px;
                pointer-events: none;
            }

            &::after {
                z-index: 999;
                content: '';
                position: absolute;
                top: 0;
                left: 0;
                height: 35px;
                width: .8rem;
                background-color: var(--tert-con);
                clip-path: polygon(40% 100%, 60% 100%, 60% 25%, 100% 0, 0 0, 40% 25%);
            }

            &--lower {
                &::before {
                    top: 34px;
                    background-color: var(--bg);
                    z-index: 1000;
                }

                &::after {
                    z-index: 999;
                    clip-path: polygon(40% 0, 60% 0, 60% 75%, 100% 100%, 0 100%, 40% 75%);
                }
            }

            &--left {
                &::before {
                    left: auto;
                    right: -.4rem;
                }

                &::after {
                    left: auto;
                    right: 0;
                }
            }

            &:hover, &:active, &:focus {
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

        &__ks {
            position: absolute;
            top: -1px;
            left: var(--position);
            height: 3px;
            width: var(--length);
            background-color: transparent;
            cursor: pointer;
            overflow: visible;

            &::before {
                z-index: 1001;
                content: attr(data-tooltip);
                position: absolute;
                top: calc(-2.2rem - (1.72rem * var(--kills)));
                left: 50%;
                display: none;
                background-color: var(--bg);
                color: var(--bg-text);
                border: var(--outline) 1px solid;
                padding: .2rem .5rem;
                border-radius: .5rem;
                width: max-content;
                max-width: 500px;
                overflow: hidden;
                white-space: pre;
                font-size: 12px;
                transform: translateX(-50%);
            }

            &::after {
                z-index: 998;
                content: '';
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
                    z-index: 1000;
                }
            }

            &--left {
                &::before {
                    left: auto;
                    right: -.4rem;
                }

                &::after {
                    left: auto;
                    right: 0;
                }
            }

            &:hover, &:active, &:focus {
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