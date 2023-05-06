<script>
    // @ts-nocheck
    import { invoke } from "@tauri-apps/api/tauri"
    import { onMount } from 'svelte';
    export let enabled;
    export let toggle;
    export let parseDemoEvents;

    let resp = {loaded: false, loading: false};
    let parsed_demo = {loaded: false, loading: false};
    let selected = [];
    let displayLives = false;
    let displayPlayers = false;
    
    let current_demo = "";

    function closeModal() {
        selected = [];
        current_demo = "";
        parsed_demo = {loaded: false, loading: false};
        displayLives = false;
        displayPlayers = false;

        for (let demo of resp.demos) {
            demo.selected = false;
        }

        toggle();
    }

    onMount(async () => {
		resp = await invoke("load_demos");
	});

    function toggleSelected(demo) {
        demo.selected = !demo.selected;
        resp = resp;
        parsed_demo = parsed_demo;
    }

    function parseDemos() {
        for (let demo of resp.demos) {
            if (demo.selected) {
                selected.push(demo.name)
            }
        }
        
        nextDemo();
    }

    async function nextDemo() {
        if (current_demo != "") {
            let events = [];

            for (let i in parsed_demo.data.player_lives) {
                let player = parsed_demo.data.player_lives[i];

                for (let life of player) {
                    if (life.selected) {
                        events.push({
                            time: (life.start + 20) - parsed_demo.data.start_tick,
                            label: `${life.kills}k-${life.assists}a_start`,
                            steamid64: parsed_demo.data.users[i].steamId64,
                            kills: life.kills,
                            start: true
                        })

                        events.push({
                            time: (life.end + 132) - parsed_demo.data.start_tick,
                            label: `${life.kills}k-${life.assists}a_end`,
                            steamid64: parsed_demo.data.users[i].steamId64
                        })
                    }
                }
            }

            let name_split = current_demo.replace(".dem", "").split("\\");
            console.log(name_split)

            parseDemoEvents(name_split[name_split.length - 1], events.sort((a, b) => a.time - b.time));
        }

        if (selected.length !== 0) {
            parsed_demo = {loaded: false, loading: true};
            current_demo = selected.shift();
            parsed_demo = await invoke("parse_demo", { path: current_demo });
            console.log(parsed_demo);
        } else {
            closeModal();
        }
    }

    function displayPlayer(player) {
        if (player == 0) {
            return false;
        }

        if (displayPlayers) {
            return true;
        }

        if (parsed_demo.data.player_lives[player].length == 0) {
            return false;
        }

        if (displayLives) {
            return true;
        }

        for (let life of parsed_demo.data.player_lives[player]) {
            if (life.kills > 0) {
                return true;
            }
        }

        return false;
    }

    function recordEntireDemo(steamId) {
        let events = [
            {
                time: 66,
                label: parsed_demo.data.users[steamId].steamId64,
                steamid64: parsed_demo.data.users[steamId].steamId64,
                kills: 0,
                start: true
            },{
                time: parsed_demo.header.ticks - 99,
                label: parsed_demo.data.users[steamId].steamId64,
                steamid64: parsed_demo.data.users[steamId].steamId64
            }
        ];
        
        let name_split = current_demo.replace(".dem", "").split("\\");
        console.log(name_split)

        parseDemoEvents(name_split[name_split.length - 1], events.sort((a, b) => a.time - b.time));
        nextDemo();
    }
</script>

{#if enabled}
    <div class="modal">
        <a class="modal__background" on:click={closeModal} href="/"> </a>
        <div class="modal__card">
            {#if resp.loaded}
                {#if current_demo === ""}
                    <h1>Demos</h1>
                    {#each resp.demos as demo}
                        <div  class={"demo " + (demo.selected && "demo--selected")}>
                            <p>{demo.name}</p>
                            <div class="add_demo">
                                {#if demo.selected}
                                    <button class="cancel-btn" on:click={toggleSelected(demo)}>Remove</button>
                                {:else}
                                    <button on:click={toggleSelected(demo)}>Add</button>
                                {/if}
                            </div>
                        </div>
                    {/each}
                    <div class="buttons">
                        <button class="cancel-btn" on:click={closeModal}>Cancel</button>
                        <button on:click={parseDemos}>Parse</button>
                    </div>
                {:else} 
                    <h1>{current_demo}</h1>
                    {#if !parsed_demo.loading}
                        <h4 class="centered">{parsed_demo.header.map}</h4>
                        <div class="buttons">
                            <div class="settings__switch">
                                <label class="switch">
                                    <input type="checkbox" bind:checked={displayLives}>
                                    <span class="slider round slider--tert"></span>
                                </label>
                                <p>Display lives with 0 Kills</p>
                            </div>
                            <div class="settings__switch">
                                <label class="switch">
                                    <input type="checkbox" bind:checked={displayPlayers}>
                                    <span class="slider round slider--tert"></span>
                                </label>
                                <p>Display players with 0 displayed lives</p>
                            </div>
                        </div>
                        <div>
                            {#each Object.keys(parsed_demo.data.player_lives) as player}
                                {#if displayPlayer(player)}
                                    <h2>
                                        <a
                                            href={`https://logs.tf/profile/${parsed_demo.data.users[player]["steamId64"]}`}
                                            class={parsed_demo.data.users[player]["team"] + " player"}
                                            data-tooltip="Open logs.tf profile"
                                            target="_blank" rel="noopener noreferrer"
                                        >{parsed_demo.data.users[player].name}</a>
                                    </h2>
                                    {#each parsed_demo.data.player_lives[player] as life}
                                        {#if life.start != 0}
                                            {#if displayLives || life.kills > 0}
                                                <div class={"demo demo__life " + (life.selected && "demo--selected")}>
                                                    <p class={
                                                        (life.kills >= 3 && "killstreak ") +
                                                        (life.kills >= 5 && " killstreak--large ") +
                                                        (life.kills >= 10 && " killstreak--massive ")
                                                    }>Kills: {life.kills}</p>
                                                    <p class={
                                                        (life.assists >= 3 && "killstreak ") +
                                                        (life.assists >= 5 && " killstreak--large ") +
                                                        (life.assists >= 10 && " killstreak--massive ")
                                                    }>Assists: {life.assists}</p>
                                                    <p>Start: {life.start - parsed_demo.data.start_tick}</p>
                                                    <p>End: {life.end - parsed_demo.data.start_tick}</p>
                                                    <div class="add_demo">
                                                        {#if life.selected}
                                                            <button class="cancel-btn" on:click={toggleSelected(life)}>Remove</button>
                                                        {:else}
                                                            <button on:click={toggleSelected(life)}>Add</button>
                                                        {/if}
                                                    </div>
                                                </div>
                                            {/if}
                                        {/if}
                                    {/each}
                                    <button class="full_demo" on:click={recordEntireDemo(player)}>Record entire demo</button>
                                {/if}
                            {/each}
                        </div>
                        <div class="buttons">
                            <button class="cancel-btn" on:click={closeModal}>Cancel</button>
                            <button on:click={nextDemo}>Save</button>
                        </div>
                    {:else}
                        <div class="loading">
                            <div class="loadingio-spinner-dual-ball-gstkvx2ybq5"><div class="ldio-h6cxzkuee3g">
                                <div></div><div></div><div></div>
                                </div>
                            </div>
                            <h4>Loading...</h4>
                        </div>
                    {/if}
                {/if}
            {:else}
                <h1>LOADING DEMOS</h1>
            {/if}
        </div>
    </div>
{/if}

<style lang="scss">
    .buttons {
        width: 100%;
        display: flex;
        gap: 1rem;
        margin-top: 1rem;

        & > * {
            width: 100%;
        }
    }

    .red {
        color: var(--red);
    }

    .blue {
        color: var(--blu);
    }

    .settings__switch {
        display: flex;
        align-items: center;
        gap: 1rem;
    }

    .full_demo {
        font-size: small;
        width: 100%;
        padding: .3rem .5rem;
        margin: 2px 0;
        font-family: 'Source Code Pro', monospace;
        color: transparent;
        border: 1px solid var(--tert-con);
        border-radius: 5px;

        display: grid;
        grid-template-columns: 1fr;
        text-align: center;
        white-space: nowrap;
        height: 0px;
        cursor: pointer;

        overflow: hidden;

        transition: all .2s;

        &:hover {
            border: 1px solid var(--tert-con-text);
        color: var(--tert-con-text);
            height: 28px;
        }
    }

    .player {
        position: relative;

        &::before {
            content: attr(data-tooltip);
            position: absolute;
            top: -2.8rem;
            left: 0rem;
            display: none;
            background-color: var(--bg);
            color: var(--bg-text);
            border: var(--outline) 1px solid;
            padding: .2rem .5rem;
            border-radius: .5rem;
            white-space: nowrap;
        }

        &::after {
            content: '';
            display: none;
            position: absolute;
            top: -.6rem;
            left: .5rem;
            height: .5rem;
            width: .8rem;
            background-color: var(--outline);
            clip-path: polygon(100% 0, 0 0, 50% 100%);
        }

        &:hover, &:active, &:focus {
            color: var(--sec);

            &::before {
                display: block;
            }

            &::after {
                display: block;
            }
        }
    }

    .demo {
        font-size: small;
        padding: .3rem .5rem;
        margin: 2px 0;
        font-family: 'Source Code Pro', monospace;
        color: var(--tert-con-text);
        border: 1px solid var(--tert-con);
        border-radius: 5px;

        display: grid;
        grid-template-columns: 1fr 1fr;
        white-space: nowrap;

        transition: all .2s;

        &__life {
            grid-template-columns: 1fr 1fr 1fr 1fr 1fr;
        }

        &--selected {
            border: 1px solid var(--tert);
        }

        & > p {
            padding: 0;
            margin: 0;
            white-space: nowrap;
        }

        &:hover {
            border: 1px solid var(--tert-con-text);
        }
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
    }

    .add_demo {
        display: flex;
        align-items: end;
        justify-content: end;
        gap: 1px;

        & > button {
            padding: 0 .7rem;
            margin: 0;
            height: 100%;
            border-radius: 5px;
            width: fit-content;
        }
    }

    .modal {
        position: fixed;
        width: 100%;
        height: 100%;
        left: 0;
        top: 0;
        z-index: 1000;
        display: flex;
        justify-content: center;
        align-items: center;

        &__card {
            height: fit-content;
            width: fit-content;
            width: 100%;
            max-width: min(calc(100vw - 2rem), 800px);
            max-height: min(calc(100vh - 2rem), 800px);
            background-color: var(--bg);
            border-radius: 8px;
            border: 1px solid var(--tert-con);
            padding: 1rem;
            position: relative;
            z-index: 1000;
            overflow-y: auto;
            overflow-x: hidden;
            margin: 1rem;
        }

        &__background {
            position: fixed;
            background-color: rgba(0, 0, 0, .6);
            width: 100%;
            height: 100%;
            left: 0;
            top: 0;
            z-index: 999;
            backdrop-filter: blur(5px);
        }
    }

    .loading {
        display: flex;
        justify-content: center;
        align-items: center;
        flex-direction: column;
        position: relative;

        & h4 {
            position: absolute;
            bottom: 1rem;
        }
    }

    @keyframes ldio-h6cxzkuee3g-o {
        0%    { opacity: 1; transform: translate(0 0) }
        49.99% { opacity: 1; transform: translate(80px,0) }
        50%    { opacity: 0; transform: translate(80px,0) }
        100%    { opacity: 0; transform: translate(0,0) }
    }
    @keyframes ldio-h6cxzkuee3g {
        0% { transform: translate(0,0) }
        50% { transform: translate(80px,0) }
        100% { transform: translate(0,0) }
    }
    .ldio-h6cxzkuee3g div {
        position: absolute;
        width: 50px;
        height: 50px;
        border-radius: 50%;
        top: 60px;
        left: 20px;
    }
    .ldio-h6cxzkuee3g div:nth-child(1) {
        background: var(--pri-con);
        animation: ldio-h6cxzkuee3g 0.6097560975609756s linear infinite;
        animation-delay: -0.3048780487804878s;
    }
    .ldio-h6cxzkuee3g div:nth-child(2) {
        background: var(--sec-con);
        animation: ldio-h6cxzkuee3g 0.6097560975609756s linear infinite;
        animation-delay: 0s;
    }
    .ldio-h6cxzkuee3g div:nth-child(3) {
        background: var(--pri-con);
        animation: ldio-h6cxzkuee3g-o 0.6097560975609756s linear infinite;
        animation-delay: -0.3048780487804878s;
    }
    .loadingio-spinner-dual-ball-gstkvx2ybq5 {
        width: 200px;
        height: 200px;
        display: inline-block;
        overflow: hidden;
        background: transparent;
    }
    .ldio-h6cxzkuee3g {
        width: 100%;
        height: 100%;
        position: relative;
        transform: translateZ(0) scale(1);
        backface-visibility: hidden;
        transform-origin: 0 0; /* see note above */
    }
    .ldio-h6cxzkuee3g div { box-sizing: content-box; }
</style>