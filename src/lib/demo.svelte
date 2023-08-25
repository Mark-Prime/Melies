<script>
    // @ts-nocheck
    import { invoke } from "@tauri-apps/api/tauri"
    import { onMount } from 'svelte';

    let index = 0,
        total = 0;

    import Timeline from "./timeline.svelte";

    export let enabled;
    export let toggle;
    export let parseDemoEvents;

    let resp = {loaded: false, loading: false};
    let parsed_demo = {loaded: false, loading: false};
    let selected = [];
    let displayLives = false;
    let displayAssists = false;
    let displayPlayers = false;
    
    let current_demo = "";
    
    let settings = {};
    let recording_settings = {};

    async function loadSettings(){
        settings = await invoke("load_settings");
        recording_settings = settings.recording
    }

    async function loadDemos() {
        resp = await invoke("load_demos");
    };

    function closeModal() {
        selected = [];
        current_demo = "";
        parsed_demo = {loaded: false, loading: false};
        // displayLives = false;
        // displayPlayers = false;

        for (let demo of resp.demos) {
            demo.selected = false;
        }

        toggle();
    }

    onMount(async () => {
		loadDemos();
        loadSettings();
	});

    $: {
        console.log("Modal Enabled:", enabled);
        loadDemos()
    }

    function limitStringLength(str, len) {
        if (str.length < len) {
            return str;
        }

        return str.substring(0, len - 3) + '...'
    }

    function toggleSelected(demo, isKillstreak = null) {
        demo.selected = !demo.selected;

        if (demo.selected_as_bookmark) {
            demo.selected_as_bookmark = false;
        }

        if (isKillstreak) {
            let player = parsed_demo.data.player_lives[demo.kills[0].killer];
            for (let life of player) {
                for (let killstreak of life.killstreaks) {
                    if (JSON.stringify(killstreak.kills[0]) === JSON.stringify(demo.kills[0])) {
                        toggleSelected(killstreak);
                        break;
                    } 
                }
            }
        } else if (isKillstreak === false) {
            for (let killstreak of parsed_demo.data.killstreaks) {
                if (JSON.stringify(killstreak.kills[0]) === JSON.stringify(demo.kills[0])) {
                    toggleSelected(killstreak);
                    break;
                } 
            }
        }

        resp = resp;
        parsed_demo = parsed_demo;
    }

    function toggleBookmarkSelected(demo, isKillstreak = null) {
        demo.selected_as_bookmark = !demo.selected_as_bookmark;

        if (demo.selected) {
            demo.selected = false;
        }

        if (isKillstreak) {
            let player = parsed_demo.data.player_lives[demo.kills[0].killer];
            for (let life of player) {
                for (let killstreak of life.killstreaks) {
                    if (JSON.stringify(killstreak.kills[0]) === JSON.stringify(demo.kills[0])) {
                        toggleBookmarkSelected(killstreak);
                        break;
                    } 
                }
            }
        } else if (isKillstreak === false) {
            for (let killstreak of parsed_demo.data.killstreaks) {
                if (JSON.stringify(killstreak.kills[0]) === JSON.stringify(demo.kills[0])) {
                    toggleBookmarkSelected(killstreak);
                    break;
                } 
            }
        }

        resp = resp;
        parsed_demo = parsed_demo;
    }

    function parseDemos() {
        for (let demo of resp.demos) {
            if (demo.selected) {
                selected.push(demo.name)
            }
        }
        
        total = selected.length;
        index = 0;

        nextDemo();
    }

    async function nextDemo() {
        index += 1;

        if (current_demo != "") {
            let events = [];

            for (let i in parsed_demo.data.player_lives) {
                let player = parsed_demo.data.player_lives[i];

                for (let life of player) {
                    if (life.selected) {
                        events.push({
                            time: (life.start + 20) - parsed_demo.data.start_tick,
                            label: `${life.kills.length}k-${life.assists.length}a_start`,
                            steamid64: parsed_demo.data.users[i].steamId64,
                            kills: life.kills.length,
                            start: true
                        })

                        events.push({
                            time: (life.end + 132) - parsed_demo.data.start_tick,
                            label: `${life.kills.length}k-${life.assists.length}a_end`
                        })
                    }

                    for (let killstreak of life.killstreaks) {
                        if (killstreak.selected) {
                            events.push({
                                time: killstreak.kills[killstreak.kills.length - 1].tick - parsed_demo.data.start_tick,
                                label: `${killstreak.kills.length}ks`,
                                steamid64: parsed_demo.data.users[i].steamId64,
                                kills: killstreak.kills.length,
                                killstreak: true
                            })

                            continue;
                        }

                        if (killstreak.selected_as_bookmark) {
                            let start_time = killstreak.kills[0].tick - recording_settings.before_killstreak_per_kill;
                            let end_time = killstreak.kills[killstreak.kills.length - 1].tick + recording_settings.after_killstreak;

                            if (life.start + 20 > start_time) {
                                start_time = life.start + 20
                            }

                            if (life.end + 132 < end_time) {
                                end_time = life.end + 132
                            }

                            events.push({
                                time: start_time - parsed_demo.data.start_tick,
                                label: `${killstreak.kills.length}ks_start`,
                                steamid64: parsed_demo.data.users[i].steamId64,
                                kills: killstreak.kills.length,
                                start: true
                            })

                            events.push({
                                time: end_time - parsed_demo.data.start_tick,
                                label: `${killstreak.kills.length}ks_end`
                            })
                        }
                    }
                }
            }

            for (let i in parsed_demo.data.chat) {
                let message = parsed_demo.data.chat[i];

                if (message.selected) {
                    events.push({
                        time: message.tick - parsed_demo.data.start_tick,
                        label: `message-sent`,
                        steamid64: parsed_demo.data.users[message.from].steamId64,
                        bookmark: true,
                    })
                }
            }

            let name_split = current_demo.replace(".dem", "").split("\\");

            parseDemoEvents(name_split[name_split.length - 1], events.sort((a, b) => a.time - b.time));
        }

        if (selected.length !== 0) {
            parsed_demo = {loaded: false, loading: true};
            current_demo = selected.shift();
            parsed_demo = await invoke("parse_demo", { path: current_demo });

            verifyTicks();

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
            if (life.kills.length > 0) {
                return true;
            }

            if (displayAssists) {
                if (life.assists.length > 0) {
                    return true;
                }
            }
        }

        return false;
    }

    function recordEntireDemo(steamId) {
        let events = [
            {
                time: settings.recording.start_delay,
                label: parsed_demo.data.users[steamId].steamId64,
                steamid64: parsed_demo.data.users[steamId].steamId64,
                kills: 0,
                start: true
            },{
                time: Math.max(parsed_demo.header.ticks - 99, settings.recording.start_delay + 66),
                label: parsed_demo.data.users[steamId].steamId64,
                steamid64: parsed_demo.data.users[steamId].steamId64
            }
        ];
        
        let name_split = current_demo.replace(".dem", "").split("\\");

        parseDemoEvents(name_split[name_split.length - 1], events.sort((a, b) => a.time - b.time));
        nextDemo();
    }

    function classConverter(player_class) {
        switch (player_class) {
            case "1":
                return "scout";
            case "3":
                return "soldier"
            case "7":
                return "pyro";
            case "4":
                return "demoman"
            case "6":
                return "heavy";
            case "9":
                return "engineer"
            case "5":
                return "medic";
            case "2":
                return "sniper"
            case "8":
                return "spy";
            default:
                return player_class;
        }
    }

    function getImgUrl(player_class) {
        switch (classConverter(player_class)) {
            case "scout":
                return "https://wiki.teamfortress.com/w/images/a/ad/Leaderboard_class_scout.png";
            case "soldier":
                return "https://wiki.teamfortress.com/w/images/9/96/Leaderboard_class_soldier.png"
            case "pyro":
                return "https://wiki.teamfortress.com/w/images/8/80/Leaderboard_class_pyro.png";
            case "demoman":
                return "https://wiki.teamfortress.com/w/images/4/47/Leaderboard_class_demoman.png"
            case "heavy":
                return "https://wiki.teamfortress.com/w/images/5/5a/Leaderboard_class_heavy.png";
            case "engineer":
                return "https://wiki.teamfortress.com/w/images/1/12/Leaderboard_class_engineer.png"
            case "medic":
                return "https://wiki.teamfortress.com/w/images/e/e5/Leaderboard_class_medic.png";
            case "sniper":
                return "https://wiki.teamfortress.com/w/images/f/fe/Leaderboard_class_sniper.png"
            case "spy":
                return "https://wiki.teamfortress.com/w/images/3/33/Leaderboard_class_spy.png";
            default:
                return false;
        }
    }

    function getLifeFromKillstreak(ks) {
        for (let life of parsed_demo.data.player_lives[ks.kills[0].killer]) {
            if (life.killstreaks.length === 0) {
                continue;
            }

            for (let killstreak of life.killstreaks) {
                if (JSON.stringify(killstreak.kills[0]) === JSON.stringify(ks.kills[0])) {
                    return life;
                }
            }
        }
    }

    function toggleClass(player, player_class) {
        let class_mapping = ['scout', 'sniper', 'soldier', 'demoman', 'medic', 'heavy', 'pyro', 'spy', 'engineer'];

        if (parsed_demo.data.users[player].hide) {
            return;
        }

        for (let life of parsed_demo.data.player_lives[player]) {
            if (!life.classes.includes(class_mapping[Number(player_class) - 1])) {
                continue;
            }

            if (displayLives) {
                toggleSelected(life);
                continue;
            }

            if (life.kills.length > 0) {
                toggleSelected(life);
                continue;
            }

            if (displayAssists) {
                if (life.assists.length > 0) {
                    toggleSelected(life);
                    continue;
                }
            }
        }
    }

    function getMessageType(message_type) {
        switch (message_type) {
            case "TF_Chat_AllDead":
                return " (Dead)";
            case "TF_Chat_Team":
                return " (Team)";
            case "TF_Chat_AllSpec":
                return " (Spectator)";
            default:
                return "";
        }
    }
    
    function verifyTicks() {
        if (parsed_demo?.header?.ticks > 0) {
            return;
        }
        
        if (parsed_demo.err_text) {
            return;
        }

        parsed_demo.header.ticks = parsed_demo.data.end_tick - parsed_demo.data.start_tick;
    }

    function refreshList() {
        displayLives = displayLives;
        displayAssists = displayAssists;
        displayPlayers = displayPlayers;
        resp = resp;
        parsed_demo = parsed_demo;
    }

    function tickToTime(ticks) {
        return `${
            Math.floor(
                Math.round(
                    (ticks) / 66
                    ) / 60
                )
            }m ${
                Math.round(
                    (ticks) / 66
                ) % 60
            }s`
    }
</script>

{#if enabled}
    <div class="modal">
        <a class="modal__background" on:click={closeModal} href="/"> </a>
        <div class="modal__card" class:modal__card--large={current_demo}>
            {#if resp.loaded}
                {#if current_demo === ""}
                    <h1>Demos</h1>
                    {#each resp.demos as demo}
                        <div  class={"demo " + (demo.selected && "demo--selected")}>
                            <p>{demo.name}</p>
                            <div class="add_demo">
                                {#if demo.selected}
                                    <button class="cancel-btn" on:click={toggleSelected(demo)}>-</button>
                                {:else}
                                    <button on:click={toggleSelected(demo)}>+</button>
                                {/if}
                            </div>
                        </div>
                    {/each}
                    <div class="buttons">
                        <button class="cancel-btn" on:click={closeModal}>Cancel</button>
                        <button on:click={parseDemos}>Parse</button>
                    </div>
                {:else if !parsed_demo.err_text} 
                    <h1>{current_demo}</h1>
                    {#if !parsed_demo.loading}
                        <h4 class="centered">{parsed_demo.header.map}</h4>
                        <div class="flex-between flex-wrap">
                            <div class="settings__switch">
                                <label class="switch">
                                    <input type="checkbox" bind:checked={displayLives} on:changed={refreshList}>
                                    <span class="slider round slider--tert"></span>
                                </label>
                                <p>Display all lives</p>
                            </div>
                            <div class="settings__switch">
                                <label class="switch">
                                    <input type="checkbox" bind:checked={displayAssists} on:changed={refreshList}>
                                    <span class="slider round slider--tert"></span>
                                </label>
                                <p>Display lives with 0 Kills if they have an Assist</p>
                            </div>
                            <div class="settings__switch">
                                <label class="switch">
                                    <input type="checkbox" bind:checked={displayPlayers} on:changed={refreshList}>
                                    <span class="slider round slider--tert"></span>
                                </label>
                                <p>Display players with 0 displayed lives</p>
                            </div>
                        </div>
                        <div class="teams">
                            {#each ["blue", "red"] as team}
                                <div class="team">
                                    <h2 class={"team__label " + team}>{team[0].toUpperCase() + team.slice(1)}</h2>
                                    {#each Object.keys(parsed_demo.data.player_lives) as player}
                                        {#if displayPlayer(player) & parsed_demo.data?.users[player]?.team === team}
                                            <div class="flex-start align-center">
                                                {#if parsed_demo.data.users[player].hide}
                                                    <button on:click={() => parsed_demo.data.users[player].hide = false} class="hide-toggle">
                                                        +
                                                    </button>
                                                {:else}
                                                    <button on:click={() => parsed_demo.data.users[player].hide = true} class="cancel-btn hide-toggle">
                                                        -
                                                    </button>
                                                {/if}
                                                <h3 class="player__header">
                                                    <a
                                                        href={`https://logs.tf/profile/${parsed_demo.data.users[player]["steamId64"]}`}
                                                        class={parsed_demo.data.users[player]["team"] + " player"}
                                                        data-tooltip="Open logs.tf profile"
                                                        target="_blank" rel="noopener noreferrer"
                                                        id={`player-${parsed_demo.data.users[player].name}`}
                                                    >{parsed_demo.data.users[player].name}</a>
                                                    {#each Object.keys(parsed_demo.data.users[player]["classes"]) as player_class}
                                                        {#if getImgUrl(player_class)}
                                                            <div 
                                                                class="tooltip" 
                                                                data-tooltip={`Lives: ${parsed_demo.data.users[player]["classes"][player_class]}`} 
                                                                style="--kills: 0"
                                                                on:click={toggleClass(player, player_class)}
                                                                on:keydown={toggleClass(player, player_class)}
                                                            >
                                                                <img src={getImgUrl(player_class)} alt="icon"/>
                                                            </div>
                                                        {/if}
                                                    {/each}
                                                </h3>
                                            </div>
                                            {#if !parsed_demo.data.users[player].hide}
                                                {#each parsed_demo.data.player_lives[player] as life}
                                                    {#if life.start != 0}
                                                        {#if displayLives || life.kills.length > 0 || (displayAssists && life.assists.length > 0)}
                                                            <div class={"demo demo__life " + (life.selected && "demo--selected")}>
                                                                <div>
                                                                    {#each life.classes as player_class}
                                                                        {#if getImgUrl(player_class)}
                                                                            <div 
                                                                                class="tooltip demo__icon" 
                                                                                data-tooltip={`Kills: ${life.kills.filter((kill) => kill.killer_class === classConverter(player_class)).length}`} 
                                                                                style={`--kills: 0;`}
                                                                            >
                                                                                <img src={getImgUrl(player_class)} alt={`${classConverter(player_class)} icon`} />
                                                                            </div>
                                                                        {/if}
                                                                    {/each}
                                                                </div>
                                                                <div 
                                                                    on:click={toggleSelected(life)}
                                                                    on:keydown={toggleSelected(life)}
                                                                    class={
                                                                        "demo__kill-count " +
                                                                        (life.kills.length >= 3 && " killstreak ") +
                                                                        (life.kills.length >= 5 && " killstreak--large ") +
                                                                        (life.kills.length >= 10 && " killstreak--massive ")
                                                                    }
                                                                >
                                                                    Kills: {life.kills.length}
                                                                </div>
                                                                <div class="demo__kills">
                                                                    {#each life.kills as kill}
                                                                        <div class="demo__kill">
                                                                            {#if getImgUrl(kill.killer_class)}
                                                                                <div 
                                                                                    class="demo__icon"
                                                                                >
                                                                                    <img src={getImgUrl(kill.killer_class)} alt={`${classConverter(kill.killer_class)} icon`} />
                                                                                </div>
                                                                            {/if} killed
                                                                            <a 
                                                                                href={`#player-${parsed_demo.data.users[kill.victim].name}`} 
                                                                                class={parsed_demo.data.users[kill.victim]["team"] + " tooltip"} 
                                                                                style="--kills: 0;"
                                                                                data-tooltip="Jump To Player"
                                                                            >
                                                                                {#if getImgUrl(kill.victim_class)}
                                                                                    <div 
                                                                                        class="demo__icon"
                                                                                    >
                                                                                        <img src={getImgUrl(kill.victim_class)} alt={`${classConverter(kill.victim_class)} icon`} />
                                                                                    </div>
                                                                                {/if}
                                                                                {parsed_demo.data.users[kill.victim].name}
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
                                                                                data-tooltip={`Timecode: ${tickToTime(kill.tick - parsed_demo.data.start_tick)}`}
                                                                            >
                                                                                {kill.tick - parsed_demo.data.start_tick}
                                                                            </span>
                                                                        </div>
                                                                    {/each}
                                                                </div>
                                                                <div 
                                                                    class={
                                                                        (life.assists.length >= 3 && "killstreak ") +
                                                                        (life.assists.length >= 5 && " killstreak--large ") +
                                                                        (life.assists.length >= 10 && " killstreak--massive ")
                                                                    }
                                                                >
                                                                    Assists: {life.assists.length}
                                                                </div>
                                                                <div
                                                                    class="tooltip"
                                                                    style={`--kills: 0;`}
                                                                    data-tooltip={`Timecode: ${tickToTime(life.start - parsed_demo.data.start_tick)}`}
                                                                >
                                                                    Start: {life.start - parsed_demo.data.start_tick}
                                                                </div>
                                                                <div
                                                                    class="tooltip"
                                                                    style={`--kills: 0;`}
                                                                    data-tooltip={`Length: ${tickToTime(life.end - life.start)}`}
                                                                >
                                                                    End: {life.end - parsed_demo.data.start_tick}
                                                                </div>
                                                                <div class="add_demo">
                                                                    {#if life.selected}
                                                                        <button class="cancel-btn" on:click={toggleSelected(life)}>-</button>
                                                                    {:else}
                                                                        <button on:click={toggleSelected(life)}>+</button>
                                                                    {/if}
                                                                </div>
                                                            </div>
                                                        {/if}
                                                    {/if}
                                                {/each}
                                                <button class="full_demo" on:click={recordEntireDemo(player)}>Record entire demo</button>

                                                {#if parsed_demo.data.player_lives[player].filter(life => life.killstreaks.length > 0).length > 0}
                                                    <h4 class="centered">Killstreaks</h4>
                                                    {#each parsed_demo.data.player_lives[player].filter(life => life.killstreaks.length > 0) as life}
                                                        {#each life.killstreaks as killstreak}
                                                            <div class={"demo demo__killstreak " + ((killstreak.selected || killstreak.selected_as_bookmark || life.selected) && "demo--selected")}>
                                                                <div>
                                                                    {#each life.classes as player_class}
                                                                        {#if getImgUrl(player_class)}
                                                                            <div 
                                                                                class="tooltip demo__icon" 
                                                                                data-tooltip={`Kills: ${killstreak.kills.filter((kill) => kill.killer_class === classConverter(player_class)).length}`}
                                                                                style={`--kills: 0;`}
                                                                            >
                                                                                <img src={getImgUrl(player_class)} alt={`${classConverter(player_class)} icon`} />
                                                                            </div>
                                                                        {/if}
                                                                    {/each}
                                                                </div>
                                                                <div
                                                                    on:click={toggleBookmarkSelected(killstreak)}
                                                                    on:keydown={toggleBookmarkSelected(killstreak)}
                                                                    class={
                                                                        `demo__kill-count ` +
                                                                        (killstreak.kills.length >= 3 && " killstreak ") +
                                                                        (killstreak.kills.length >= 5 && " killstreak--large ") +
                                                                        (killstreak.kills.length >= 10 && " killstreak--massive ")
                                                                    }
                                                                >
                                                                    Kills: {killstreak.kills.length}
                                                                </div>
                                                                <div class="demo__kills">
                                                                    {#each killstreak.kills as kill}
                                                                        <div class="demo__kill">
                                                                            {#if getImgUrl(kill.killer_class)}
                                                                                <div 
                                                                                    class="demo__icon"
                                                                                >
                                                                                    <img src={getImgUrl(kill.killer_class)} alt={`${classConverter(kill.killer_class)} icon`} />
                                                                                </div>
                                                                            {/if}
                                                                            killed
                                                                            <a 
                                                                                href={`#player-${parsed_demo.data.users[kill.victim].name}`} 
                                                                                class={parsed_demo.data.users[kill.victim]["team"] + " tooltip"} 
                                                                                style="--kills: 0;"
                                                                                data-tooltip="Jump To Player"
                                                                            >
                                                                                {#if getImgUrl(kill.victim_class)}
                                                                                    <div 
                                                                                        class="demo__icon"
                                                                                    >
                                                                                        <img src={getImgUrl(kill.victim_class)} alt={`${classConverter(kill.victim_class)} icon`} />
                                                                                    </div>
                                                                                {/if}
                                                                                {parsed_demo.data.users[kill.victim].name}
                                                                            </a> with {kill.weapon}
                                                                            {#if kill.crit_type}
                                                                                <span class={["", "killstreak", "killstreak--large"][kill.crit_type]}>
                                                                                    {["", " (Mini-Crit) ", " (CRITICAL HIT!) "][kill.crit_type]}
                                                                                </span>
                                                                            {/if}
                                                                            at 
                                                                            <span 
                                                                                class="tooltip"
                                                                                style={`--kills: 0;`}
                                                                                data-tooltip={`Timecode: ${tickToTime(kill.tick - parsed_demo.data.start_tick)}`}
                                                                            >
                                                                                {kill.tick - parsed_demo.data.start_tick}
                                                                            </span>
                                                                        </div>
                                                                    {/each}
                                                                </div>
                                                                <div
                                                                    class="tooltip"
                                                                    style={`--kills: 0;`}
                                                                    data-tooltip={
                                                                    `Timecode: ${
                                                                        Math.floor(
                                                                            Math.round(
                                                                                (killstreak.kills[0].tick - parsed_demo.data.start_tick) / 66) / 60
                                                                            )
                                                                        }m ${
                                                                            Math.round(
                                                                                (killstreak.kills[0].tick - parsed_demo.data.start_tick) / 66
                                                                            ) % 60
                                                                        }s`
                                                                    }
                                                                >
                                                                    First: {killstreak.kills[0].tick - parsed_demo.data.start_tick}
                                                                </div>
                                                                <div
                                                                    class="tooltip"
                                                                    style={`--kills: 0;`}
                                                                    data-tooltip={`Length: ${tickToTime(killstreak.kills[killstreak.kills.length - 1].tick - killstreak.kills[0].tick)}`}
                                                                >
                                                                    Last: {killstreak.kills[killstreak.kills.length - 1].tick - parsed_demo.data.start_tick}
                                                                </div>
                                                                <div class="killstreak__buttons">
                                                                    <div class="add_demo tooltip tooltip--left" data-tooltip="Entire Life" style={`--kills: 0;`}>
                                                                        {#if life.selected}
                                                                            <button class="cancel-btn" on:click={toggleSelected(life)}>-</button>
                                                                        {:else}
                                                                            <button on:click={toggleSelected(life)}>+</button>
                                                                        {/if}
                                                                    </div>
                                                                    <div class="add_demo tooltip tooltip--left" data-tooltip="As Killstreak" style={`--kills: 0;`}>
                                                                        {#if killstreak.selected}
                                                                            <button class="cancel-btn" on:click={toggleSelected(killstreak, false)}>-</button>
                                                                        {:else}
                                                                            <button on:click={toggleSelected(killstreak, false)}>+</button>
                                                                        {/if}
                                                                    </div>
                                                                    <div class="add_demo tooltip tooltip--left" data-tooltip="As Bookmarks" style={`--kills: 0;`}>
                                                                        {#if killstreak.selected_as_bookmark}
                                                                            <button class="cancel-btn" on:click={toggleBookmarkSelected(killstreak, false)}>-</button>
                                                                        {:else}
                                                                            <button on:click={toggleBookmarkSelected(killstreak, false)}>+</button>
                                                                        {/if}
                                                                    </div>
                                                                </div>
                                                            </div>
                                                        {/each}
                                                    {/each}
                                                {/if}
                                            {/if}
                                        {/if}
                                    {/each}
                                </div>
                            {/each}
                        </div>
                        {#if parsed_demo.data.killstreaks.length != 0}
                            <h2 class="centered chat__title">All Killstreaks</h2>
                            <div class="killstreaks">
                                {#each parsed_demo.data.killstreaks as killstreak}
                                    <div class={"demo demo__life " + ((killstreak.selected || killstreak.selected_as_bookmark || getLifeFromKillstreak(killstreak)?.selected) && "demo--selected")}>
                                        <div>
                                            {#each killstreak.classes as player_class}
                                                {#if getImgUrl(player_class)}
                                                    <div 
                                                        class="tooltip demo__icon" 
                                                        data-tooltip={`Kills: ${killstreak.kills.filter((kill) => kill.killer_class === classConverter(player_class)).length}`} 
                                                        style={`--kills: 0;`}
                                                    >
                                                        <img src={getImgUrl(player_class)} alt={`${classConverter(player_class)} icon`} />
                                                    </div>
                                                {/if}
                                            {/each}
                                        </div>
                                        <a 
                                            href={`#player-${parsed_demo.data.users[killstreak.kills[0].killer].name}`} 
                                            data-tooltip="Jump To Player"
                                            style="width: 100%; --kills: 0;" 
                                            class={parsed_demo.data.users[killstreak.kills[0].killer]["team"] + " tooltip"}
                                        >
                                            {limitStringLength(parsed_demo.data.users[killstreak.kills[0].killer].name, 16)}
                                        </a>
                                        <div 
                                            on:click={toggleBookmarkSelected(killstreak, true)}
                                            on:keydown={toggleBookmarkSelected(killstreak, true)}
                                            class={
                                                `demo__kill-count ` +
                                                (killstreak.kills.length >= 3 && " killstreak ") +
                                                (killstreak.kills.length >= 5 && " killstreak--large ") +
                                                (killstreak.kills.length >= 10 && " killstreak--massive ")
                                            }
                                        >
                                            Kills: {killstreak.kills.length}
                                        </div>
                                        <div class="demo__kills">
                                            {#each killstreak.kills as kill}
                                                <div class="demo__kill">
                                                    {#if getImgUrl(kill.killer_class)}
                                                        <div 
                                                            class="demo__icon"
                                                        >
                                                            <img src={getImgUrl(kill.killer_class)} alt={`${classConverter(kill.killer_class)} icon`} />
                                                        </div>
                                                    {/if}
                                                    killed
                                                    <a 
                                                        href={`#player-${parsed_demo.data.users[kill.victim].name}`} 
                                                        class={parsed_demo.data.users[kill.victim]["team"] + " tooltip"} 
                                                        style="--kills: 0;"
                                                        data-tooltip="Jump To Player"
                                                    >
                                                        {#if getImgUrl(kill.victim_class)}
                                                            <div 
                                                                class="demo__icon"
                                                            >
                                                                <img src={getImgUrl(kill.victim_class)} alt={`${classConverter(kill.victim_class)} icon`} />
                                                            </div>
                                                        {/if}
                                                        {parsed_demo.data.users[kill.victim].name}
                                                    </a> with {kill.weapon}
                                                    {#if kill.crit_type}
                                                        <span class={["", "killstreak", "killstreak--large"][kill.crit_type]}>
                                                            {["", " (Mini-Crit) ", " (CRITICAL HIT!) "][kill.crit_type]}
                                                        </span>
                                                    {/if}
                                                    at 
                                                    <span 
                                                        class="tooltip"
                                                        style={`--kills: 0;`}
                                                        data-tooltip={`Timecode: ${tickToTime(kill.tick - parsed_demo.data.start_tick)}`}
                                                    >
                                                        {kill.tick - parsed_demo.data.start_tick}
                                                    </span>
                                                </div>
                                            {/each}
                                        </div>
                                        <div
                                            class="tooltip"
                                            style={`--kills: 0;`}
                                            data-tooltip={
                                            `Timecode: ${
                                                Math.floor(
                                                    Math.round(
                                                        (killstreak.kills[0].tick - parsed_demo.data.start_tick) / 66) / 60
                                                    )
                                                }m ${
                                                    Math.round(
                                                        (killstreak.kills[0].tick - parsed_demo.data.start_tick) / 66
                                                    ) % 60
                                                }s`
                                            }
                                        >
                                            First: {killstreak.kills[0].tick - parsed_demo.data.start_tick}
                                        </div>
                                        <div
                                            class="tooltip"
                                            style={`--kills: 0;`}
                                            data-tooltip={`Length: ${tickToTime(killstreak.kills[killstreak.kills.length - 1].tick - killstreak.kills[0].tick)}`}
                                        >
                                            Last: {killstreak.kills[killstreak.kills.length - 1].tick - parsed_demo.data.start_tick}
                                        </div>
                                        <div class="killstreak__buttons">
                                            <div class="add_demo tooltip tooltip--left" data-tooltip="Entire Life" style={`--kills: 0;`}>
                                                {#if getLifeFromKillstreak(killstreak).selected}
                                                    <button class="cancel-btn" on:click={toggleSelected(getLifeFromKillstreak(killstreak))}>-</button>
                                                {:else}
                                                    <button on:click={toggleSelected(getLifeFromKillstreak(killstreak))}>+</button>
                                                {/if}
                                            </div>
                                            <div class="add_demo tooltip tooltip--left" data-tooltip="As Killstreak" style={`--kills: 0;`}>
                                                {#if killstreak.selected}
                                                    <button class="cancel-btn" on:click={toggleSelected(killstreak, true)}>-</button>
                                                {:else}
                                                    <button on:click={toggleSelected(killstreak, true)}>+</button>
                                                {/if}
                                            </div>
                                            <div class="add_demo tooltip tooltip--left" data-tooltip="As Bookmarks" style={`--kills: 0;`}>
                                                {#if killstreak.selected_as_bookmark}
                                                    <button class="cancel-btn" on:click={toggleBookmarkSelected(killstreak, true)}>-</button>
                                                {:else}
                                                    <button on:click={toggleBookmarkSelected(killstreak, true)}>+</button>
                                                {/if}
                                            </div>
                                        </div>
                                    </div>
                                {/each}
                            </div>
                        {/if}
                        <Timeline 
                            parsed_demo={parsed_demo} 
                            tickToTime={tickToTime} 
                            displayPlayer={displayPlayer} 
                            toggleSelected={toggleSelected}
                            displayLives={displayLives}
                            displayAssists={displayAssists}
                            getImgUrl={getImgUrl}
                        />
                        {#if parsed_demo.data.chat.length > 0}
                            <h2 class="centered chat__title">Chat</h2>
                            <div class="chat">
                                {#each parsed_demo.data.chat as chat}
                                    {#if chat.selected}
                                        <button class="cancel-btn" on:click={toggleSelected(chat)}>-</button>
                                    {:else}
                                        <button on:click={toggleSelected(chat)}>+</button>
                                    {/if}
                                    <div class="chat__tick">
                                        {chat.tick - parsed_demo.data.start_tick}
                                    </div>
                                    <div class="chat__text">
                                        <span class={`chat__name ${parsed_demo.data?.users[chat.from]?.team}`}>{chat.name}{getMessageType(chat.message.kind)}:</span>
                                        {chat.text}
                                    </div>
                                {/each}
                            </div>
                        {/if}
                        <!-- <div class="settings__input-group">
                            <label for="tf_folder" class="settings__label">scale</label>
                            <input bind:value={scale} id="tf_folder" class="settings__input input--tert" type="number"/>
                        </div> -->
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
                            <h4>Loading {index}/{total}...</h4>
                        </div>
                    {/if}
                    {:else if parsed_demo.err_text}
                    <h1>Error: {parsed_demo.code}</h1>
                    <h2 class='centered'>{parsed_demo.err_text}</h2>
                {/if}
            {:else}
                <h1>LOADING DEMOS</h1>
            {/if}
        </div>
    </div>
{/if}

<style lang="scss">
    .killstreak__buttons {
        display: flex;
        gap: 1rem;
        justify-content: end;
        grid-column: 6;
    }

    .input__slider {
        -webkit-appearance: none;  /* Override default CSS styles */
        appearance: none;
        width: 95%; /* Full-width */
        height: 0px; /* Specified height */
        background: var(--tert); /* Grey background */
        outline: none; /* Remove outline */
        opacity: 0.7; /* Set transparency (for mouse-over effects on hover) */
        -webkit-transition: .2s; /* 0.2 seconds transition on hover */
        transition: opacity .2s;
        position: relative;
        top: -1.3rem;

        &:hover {
            opacity: 1;
        }

        &::-webkit-slider-thumb {
            -webkit-appearance: none; /* Override default look */
            appearance: none;
            width: 10px; /* Set a specific slider handle width */
            height: 10px; /* Slider handle height */
            background: var(--tert-con); /* Green background */
            cursor: pointer; /* Cursor on hover */
            border-radius: 100%;
        }

        &::-moz-range-thumb {
            width: 25px; /* Set a specific slider handle width */
            height: 25px; /* Slider handle height */
            background: var(--tert-con); /* Green background */
            cursor: pointer; /* Cursor on hover */
        }
    }

    .killstreaks {
        max-width: 800px;
        margin: auto;
    }

    .chat {
        width: 100%;
        display: grid;
        grid-template-columns: min-content min-content 1fr;
        width: 100%;
        max-width: 750px;
        margin: auto;
        gap: .2rem;
        border: 1px solid var(--tert-con);
        border-radius: 5px;
        padding: 1rem;

        &__tick {
            text-align: right;
            margin: 0 .3rem;
            padding-right: .5rem;
            border-right: var(--tert-con) solid 1px;
        }

        &__title {
            margin-top: 3rem;
        }

        & > button {
            height: 25px;
            width: 25px;
            padding: 0 0 0 1px;
            display: table-cell;
            vertical-align: middle;
        }
    }

    .flex-wrap {
        flex-wrap: wrap;
    }
    
    img {
        height: 1.5rem;
    }

    .flex-start {
        display: flex;
        justify-content: flex-start;
    }

    .player__header {
        display: flex;
        justify-content: left;
        align-items: center;
        gap: 1rem;

        & > div {
            display: flex;
            justify-content: center;
            align-items: center;
        }
    }

    .team {
        min-width: 550px;
        flex-grow: 1;
        flex-shrink: 0;

        &__label {
            text-align: center;
        }
    }

    .teams {
        width: 100%;
        display: grid;
        gap: 1rem;
        grid-template-columns: repeat(auto-fit, minmax(550px, 1fr));
    }

    .buttons {
        width: 100%;
        display: flex;
        gap: 1rem;
        margin-top: 1rem;

        & > * {
            width: 100%;
        }
    }

    .align-center {
        align-items: center;
    }

    .hide-toggle {
        height: 2.4rem;
        margin-right: 1rem;
        font-weight: bold;
        width: 2.4rem;
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
            top: -2.4rem;
            left: 0rem;
            display: none;
            background-color: var(--bg);
            color: var(--bg-text);
            border: var(--outline) 1px solid;
            padding: .2rem .5rem;
            border-radius: .5rem;
            white-space: nowrap;
            font-size: 12px;
        }

        &::after {
            content: '';
            display: none;
            position: absolute;
            top: -.2rem;
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

        &__icon {
            display: flex;
            justify-content: center;
            align-items: center;
        }

        &__kills {
            grid-row: 2;
            grid-column: 1 / 7;
            display: flex;
            flex-direction: column;
            width: 100%;
            transition: all 0.2s;
            border-radius: 3px;
            padding: .5rem;

            &:hover {
                background-color: rgba(256, 256, 256, 0.03);

                & .demo__kill {
                    opacity: 1;
                    display: flex;
                    gap: .5rem;
                    height: 100%;
                }
            }
        }

        &__kill {
            width: 100%;
            opacity: 0;
            display: none;
            transition: all 0.2s;
            height: 0;
            padding: 0 .5rem;

            &:first-of-type {
                margin-top: .5rem;
            }

            &:last-of-type {
                margin-bottom: .5rem;
            }

            a {
                display: flex;
                justify-content: start;
                align-items: center;
                gap: .5rem;
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
                        gap: .5rem;
                        height: 100%;
                    }
                }
            }
        }

        & .demo__kill-count {
            padding-left: .5rem;
            margin-right: .5rem;
        }

        &__life {
            grid-template-columns: .5fr 1fr 1fr 1fr 1fr min-content;
            height: auto;
            max-height: 35.75px;
            transition: all 0.2s;

            &:hover {
                max-height: 100%;
            }
        }

        &__killstreak {
            grid-template-columns: .5fr 1fr 1fr 1fr min-content;
        }

        &--selected {
            border: 1px solid var(--tert);
        }

        & > div, & > p {
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
        align-items: center;
        justify-content: flex-end;
        gap: 1px;

        & > button {
            font-size: 12px;
            padding: .3rem .7rem;
            margin: 0;
            // height: 100%;
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

            &--large {
                max-width: min(calc(100vw - 2rem), 1680px);
                max-height: min(calc(100vh - 2rem), 900px);
            }
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