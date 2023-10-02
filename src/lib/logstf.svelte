<script>
// @ts-nocheck
export let enabled;
export let toggle;
export let parseLogEvents;
export let modified;

import { invoke } from "@tauri-apps/api/tauri"

let url = "";
let resp = { loading: false };

let ubers = [],
    med_deaths = [],
    killstreaks = [],
    logs = [];

let index = 0,
    total = 0;

function parseUrls() {
    logs = url.split(',')

    total = logs.length;

    parseLog();
}

async function parseLog(){
    resp.loading = true;

    url = logs.pop();

    if (!url) {
        return false;
    }

    ubers = [];
    med_deaths = [];
    killstreaks = [];
    index += 1;

    try {
        let parsed_url = new URL(url);
        resp = await invoke("parse_log", {url: parsed_url.pathname.replace("/", "")})
        url = parsed_url.pathname.replace("/", "")
    } catch {
        resp = await invoke("parse_log", {url: url.split("#")[0]})
        url = url.split("#")[0]
    }

    for (let [i, round] of resp["rounds"].entries()) {
        ubers.push([]);
        med_deaths.push([]);

        for (let [index, event] of round["events"].entries()) {
            switch (event.type) {
                case 'charge': ubers[i].push(event); continue;
                case 'medic_death': {
                    if (round["events"][index - 1]?.type === 'drop') {
                        event.isDrop = true;
                    }

                    med_deaths[i].push(event); 
                    continue;
                }
                default: continue;
            }
        }
    }

    killstreaks = resp["killstreaks"]

    resp.loaded = true
    resp = resp

    return true;
}

function getPlayerStats(steamid) {
    return resp.players[steamid];
}

function getPlayerName(steamid) {
    return resp.names[steamid];
}

function isPlayer(steamid) {
    if (Object.hasOwn(resp.names, steamid)) {
        return true;
    }

    return false;
}

function parseClasses(class_stats) {
    let classes = [];
    for (let player_class of class_stats) {
        classes.push(player_class.type[0].toUpperCase() + player_class.type.slice(1));
    }

    return classes.join(", ")
}

function toggleSelected(event) {
    event.selected = !event.selected;
    killstreaks = killstreaks;
    ubers = ubers;
    med_deaths = med_deaths;
}

function saveSelected() {
    let events = [];
    let demo_name = "log_" + url;
    for (let killstreak of killstreaks) {
        if (killstreak.selected) {
            events.push({
                time: killstreak.time,
                label: "killstreak",
                steamid64: getPlayerStats(killstreak.steamid).steamid64
            })
        }
    }

    for (let round of med_deaths) {
        for (let event of round) {
            if (event.selected) {
                events.push({
                    time: event.time,
                    label: "medic-death",
                    steamid64: getPlayerStats(event.killer).steamid64
                })
            }
        }
    }

    for (let round of ubers) {
        for (let event of round) {
            if (event.selected) {
                events.push({
                    time: event.time,
                    label: "ubercharge",
                    steamid64: getPlayerStats(event.steamid).steamid64
                })
            }
        }
    }

    modified();

    parseLogEvents(demo_name, events.sort((a, b) => a.time - b.time));

    ubers = [];
    med_deaths = [];
    killstreaks = [];

    if (logs.length === 0) {
        closeModal();
        return;
    }

    parseLog();
}

function closeModal() {
    resp = {}
    resp.loading = false;
    url = "";
    toggle();
}
</script>

{#if enabled}
    <div class="modal">
        <a class="modal__background" on:click={closeModal} href="/"> </a>
        <div class="modal__card">
            {#if resp.loading}
                <h1>Logs.tf Parser</h1>
                <div class="loading">
                    <div class="loadingio-spinner-dual-ball-gstkvx2ybq5"><div class="ldio-h6cxzkuee3g">
                        <div></div><div></div><div></div>
                        </div>
                    </div>
                    <h4>Loading {index}/{total}...</h4>
                </div>
            {:else if resp.loaded}
                <div>
                    <h1>{resp.info.title}</h1>
                    <h4 class="centered">{resp.info.map}</h4>
                    <div class="buttons">
                        <a href={`https://logs.tf/${url}`} class={resp.demo_url && "align-left"} target="_blank" rel="noopener noreferrer">Logs.tf</a>
                        {#if resp.demo_url}
                            <a href={resp.demo_url} target="_blank" rel="noopener noreferrer" class="centered">Demos.tf</a>
                            <a href={resp.demo_download} rel="noopener noreferrer" class="align-right">Download Demo</a>
                        {/if}
                    </div>
                    <h2 class="section_header">Killstreaks</h2>
                    {#each killstreaks as killstreak}
                        {#if isPlayer(killstreak.steamid)}
                            <div class={"event streak " + (killstreak.selected && "event--selected")}>
                                <p>
                                    <a
                                        href={`https://logs.tf/profile/${getPlayerStats(killstreak.steamid)?.steamid64}`}
                                        class={getPlayerStats(killstreak.steamid)?.team + " player"}
                                        data-tooltip={parseClasses(getPlayerStats(killstreak.steamid).class_stats)}
                                        target="_blank" rel="noopener noreferrer"
                                    >
                                        {getPlayerName(killstreak.steamid)}
                                    </a>got a {killstreak.streak}ks at {killstreak.time * 66}
                                </p>
                                <div class="add_event">
                                    {#if killstreak.selected}
                                        <button class="cancel-btn" on:click={toggleSelected(killstreak)}>Remove</button>
                                    {:else}
                                        <button on:click={toggleSelected(killstreak)}>Add</button>
                                    {/if}
                                </div>
                            </div>
                        {/if}
                    {/each}

                    <h2 class="section_header">Medic Kills</h2>
                    {#each med_deaths as round, index}
                        {#if round.length > 1}
                            <h4 class="round">Round {index + 1}</h4>
                            {#each round as event}
                                {#if isPlayer(event.killer) && isPlayer(event.steamid)}
                                    <div class={"event death" + (event.selected && " event--selected ") + (event.isDrop && " drop")}>
                                        <p>
                                            <a
                                                href={`https://logs.tf/profile/${getPlayerStats(event.killer)?.steamid64}`}
                                                class={getPlayerStats(event.killer)?.team + " player"}
                                                data-tooltip={parseClasses(getPlayerStats(event.killer).class_stats)}
                                                target="_blank" rel="noopener noreferrer"
                                            >
                                                {getPlayerName(event.killer)}
                                            </a>{#if event.isDrop}
                                                <strong>dropped </strong>{:else}
                                                killed {/if}<a
                                                href={`https://logs.tf/profile/${getPlayerStats(event.steamid)?.steamid64}`}
                                                class={getPlayerStats(event.steamid)?.team + " player"}
                                                data-tooltip={parseClasses(getPlayerStats(event.steamid).class_stats)}
                                                target="_blank" rel="noopener noreferrer"
                                            >
                                                {getPlayerName(event.steamid)}
                                            </a>at {event.time * 66}
                                        </p>
                                        <div class="add_event">
                                            {#if event.selected}
                                                <button class="cancel-btn" on:click={toggleSelected(event)}>Remove</button>
                                            {:else}
                                                <button on:click={toggleSelected(event)}>Add</button>
                                            {/if}
                                        </div>
                                    </div>
                                {/if}
                            {/each}
                        {/if}
                    {/each}

                    <h2 class="section_header">Übers</h2>
                    {#each ubers as round, index}
                        {#if round.length > 1}
                            <h4 class="round">Round {index + 1}</h4>
                            {#each round as event}
                                {#if isPlayer(event.steamid)}
                                    <div class={"event uber " + (event.selected && "event--selected")}>
                                        <p>
                                            <a
                                                href={`https://logs.tf/profile/${getPlayerStats(event.steamid)?.steamid64}`}
                                                class={getPlayerStats(event.steamid)?.team + " player"}
                                                data-tooltip={parseClasses(getPlayerStats(event.steamid).class_stats)}
                                                target="_blank" rel="noopener noreferrer"
                                            >
                                                {resp["names"][event.steamid]}
                                            </a>used Übercharge ({event.medigun}) at {event.time * 66}
                                        </p>
                                        <div class="add_event">
                                            {#if event.selected}
                                                <button class="cancel-btn" on:click={toggleSelected(event)}>Remove</button>
                                            {:else}
                                                <button on:click={toggleSelected(event)}>Add</button>
                                            {/if}
                                        </div>
                                    </div>
                                {/if}
                            {/each}
                        {/if}
                    {/each}
                </div>
                <h3 class="centered padding-0">Ticks may be inaccurate</h3>
                <p class="centered padding-0">(logs.tf doesn't store them well)</p>
                <div class="buttons">
                    <button class="cancel-btn" on:click={closeModal}>Cancel</button>
                    <button on:click={saveSelected}>Save</button>
                </div>
            {:else}
                <h1>Logs.tf Parser</h1>
                <div class="input-group">
                    <label for="tf_folder" class="input-group__label">Logs.tf URL or ID</label>
                    <input bind:value={url} id="tf_folder" class="input-group__input input--sec"/>
                </div>
                <div class="buttons">
                    <button class="cancel-btn" on:click={closeModal}>Cancel</button>
                    <button on:click={parseUrls}>Parse</button>
                </div>
            {/if}
        </div>
    </div>
{/if}

<style lang="scss">
    .section_header {
        margin-top: 2.5rem;
        margin-bottom: 0;
        color: var(--sec-con-text);
    }

    .event {
        font-size: small;
        padding: .3rem .5rem;
        margin: 2px 0;
        font-family: 'Source Code Pro', monospace;
        color: var(--sec-con-text);
        border: 1px solid var(--sec-con);
        border-radius: 5px;

        display: grid;
        grid-template-columns: 1fr 1fr;

        &--selected {
            border: 1px solid var(--sec);
        }

        & > p {
            padding: 0;
            margin: 0;
            white-space: nowrap;
        }
    }

    .add_event {
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

    .Red {
        color: var(--red);
    }

    .Blue {
        color: var(--blu);
    }

    .round {
        margin: 0;
        color: var(--sec);
    }

    .drop {
        color: var(--tert);
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
            border: 1px solid var(--sec-con);
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

    .buttons {
        width: 100%;
        display: flex;
        gap: 1rem;
        margin-top: 1rem;

        & > * {
            width: 100%;
        }

        & > a {
            padding: 0 1rem;
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
        0%    { opacity: 1; transform: translate(0, 0) }
        49.99% { opacity: 1; transform: translate(100px, 0) }
        50%    { opacity: 0; transform: translate(100px, 0) }
        100%    { opacity: 0; transform: translate(0, 0) }
    }
    @keyframes ldio-h6cxzkuee3g {
        0% { transform: translate(0, 0) }
        50% { transform: translate(100px, 0) }
        100% { transform: translate(0, 0) }
    }
    .ldio-h6cxzkuee3g div {
        position: absolute;
        width: 60px;
        height: 60px;
        border-radius: 50%;
        top: 40px;
        left: 20px;
    }
    .ldio-h6cxzkuee3g div:nth-child(1) {
        background: var(--pri-con);
        animation: ldio-h6cxzkuee3g 0.6097560975609756s linear infinite;
        animation-delay: -0.3048780487804878s;
    }
    .ldio-h6cxzkuee3g div:nth-child(2) {
        background: var(--tert-con);
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