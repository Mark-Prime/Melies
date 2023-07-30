<script>
    // @ts-nocheck
    import { invoke } from "@tauri-apps/api/tauri"
    import { onMount } from 'svelte';
    import { useMousePosition } from '@svelteuidev/composables';
    export let enabled;
    export let toggle;
    export let parseDemoEvents;

    let resp = {loaded: false, loading: false};
    let parsed_demo = {loaded: false, loading: false};
    let selected = [];
    let displayLives = false;
    let displayAssists = false;
    let displayPlayers = false;
    let scale = 1.0;
    let timeline;
    let sleft = 0;

    let divWidth;

	const [position, ref] = useMousePosition();
	$: ({ x, y } = $position);

    $: if (scale > parsed_demo?.header?.ticks / divWidth) {
        scale = parsed_demo?.header?.ticks / divWidth
    };
    
    let current_demo = "";

    
    let settings = {};
    let recording_settings = {};

    async function loadSettings(){
        settings = await invoke("load_settings");
        recording_settings = settings.recording
    }

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
        loadSettings();
	});

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
            console.log({parsed_demo});
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
                time: 349,
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

    function getImgUrl(player_class) {
        switch (player_class) {
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
            case "1":
                return "https://wiki.teamfortress.com/w/images/a/ad/Leaderboard_class_scout.png";
            case "3":
                return "https://wiki.teamfortress.com/w/images/9/96/Leaderboard_class_soldier.png"
            case "7":
                return "https://wiki.teamfortress.com/w/images/8/80/Leaderboard_class_pyro.png";
            case "4":
                return "https://wiki.teamfortress.com/w/images/4/47/Leaderboard_class_demoman.png"
            case "6":
                return "https://wiki.teamfortress.com/w/images/5/5a/Leaderboard_class_heavy.png";
            case "9":
                return "https://wiki.teamfortress.com/w/images/1/12/Leaderboard_class_engineer.png"
            case "5":
                return "https://wiki.teamfortress.com/w/images/e/e5/Leaderboard_class_medic.png";
            case "2":
                return "https://wiki.teamfortress.com/w/images/f/fe/Leaderboard_class_sniper.png"
            case "8":
                return "https://wiki.teamfortress.com/w/images/3/33/Leaderboard_class_spy.png";
            default:
                return ""
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
                {:else} 
                    <h1>{current_demo}</h1>
                    {#if !parsed_demo.loading}
                        <h4 class="centered">{parsed_demo.header.map}</h4>
                        <div class="flex-between flex-wrap">
                            <div class="settings__switch">
                                <label class="switch">
                                    <input type="checkbox" bind:checked={displayLives}>
                                    <span class="slider round slider--tert"></span>
                                </label>
                                <p>Display all lives</p>
                            </div>
                            <div class="settings__switch">
                                <label class="switch">
                                    <input type="checkbox" bind:checked={displayAssists}>
                                    <span class="slider round slider--tert"></span>
                                </label>
                                <p>Display lives with 0 Kills if they have an Assist</p>
                            </div>
                            <div class="settings__switch">
                                <label class="switch">
                                    <input type="checkbox" bind:checked={displayPlayers}>
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
                                                    <div 
                                                        class="tooltip" 
                                                        data-tooltip={`Lives: ${parsed_demo.data.users[player]["classes"][player_class]}`} 
                                                        style="--kills: 0"
                                                        on:click={toggleClass(player, player_class)}
                                                        on:keydown={toggleClass(player, player_class)}
                                                    >
                                                        <img src={getImgUrl(player_class)} alt="icon"/>
                                                    </div>
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
                                                                        <img src={getImgUrl(player_class)} alt="icon" />
                                                                    {/each}
                                                                </div>
                                                                <div 
                                                                    data-tooltip={`${
                                                                        life.kills.length ? 
                                                                        `Player${(life.kills.length > 1) ? "s" : ""} Killed: ` :
                                                                        `No Kills`
                                                                    }\n\r${life.kills.map((kill) => {
                                                                        let crit_types = ["", " Mini-Crit", " CRITICAL HIT!"]
                                                                        return `${parsed_demo.data.users[kill.victim].name} (tick: ${kill.tick - parsed_demo.data.start_tick})${crit_types[kill.crit_type]}`
                                                                    }).join(", \n\r")}`}
                                                                    style={`--kills: ${life.kills.length};`}
                                                                    on:click={toggleSelected(life)}
                                                                    on:keydown={toggleSelected(life)}
                                                                    class={
                                                                        `tooltip ` +
                                                                        (life.kills.length >= 3 && " killstreak ") +
                                                                        (life.kills.length >= 5 && " killstreak--large ") +
                                                                        (life.kills.length >= 10 && " killstreak--massive ")
                                                                    }
                                                                >
                                                                    Kills: {life.kills.length}
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
                                                                <div>Start: {life.start - parsed_demo.data.start_tick}</div>
                                                                <div>End: {life.end - parsed_demo.data.start_tick}</div>
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
                                                            <div class={"demo demo__killstreak " + ((killstreak.selected || killstreak.selected_as_bookmark) && "demo--selected")}>
                                                                <div>
                                                                    {#each life.classes as player_class}
                                                                        <img src={getImgUrl(player_class)} alt="icon" />
                                                                    {/each}
                                                                </div>
                                                                <div 
                                                                    data-tooltip={`${
                                                                        killstreak.kills.length ? 
                                                                        `Player${(killstreak.kills.length > 1) ? "s" : ""} Killed: ` :
                                                                        `No Kills`
                                                                    }\n\r${killstreak.kills.map((kill) => {
                                                                        let crit_types = ["", " Mini-Crit", " CRITICAL HIT!"]
                                                                        return `${parsed_demo.data.users[kill.victim].name} (tick: ${kill.tick - parsed_demo.data.start_tick})${crit_types[kill.crit_type]}`
                                                                    }).join(", \n\r")}`}
                                                                    style={`--kills: ${killstreak.kills.length};`}
                                                                    on:click={toggleBookmarkSelected(killstreak)}
                                                                    on:keydown={toggleBookmarkSelected(killstreak)}
                                                                    class={
                                                                        `tooltip ` +
                                                                        (killstreak.kills.length >= 3 && " killstreak ") +
                                                                        (killstreak.kills.length >= 5 && " killstreak--large ") +
                                                                        (killstreak.kills.length >= 10 && " killstreak--massive ")
                                                                    }
                                                                >
                                                                    Kills: {killstreak.kills.length}
                                                                </div>
                                                                <div>
                                                                    First: {killstreak.kills[0].tick - parsed_demo.data.start_tick}
                                                                </div>
                                                                <div>
                                                                    Last: {killstreak.kills[killstreak.kills.length - 1].tick - parsed_demo.data.start_tick}
                                                                </div>
                                                                <div class="killstreak__buttons">
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
                        <h2 class="centered chat__title">All Killstreaks</h2>
                        <div class="killstreaks">
                            {#each parsed_demo.data.killstreaks as killstreak}
                                <div class={"demo demo__life " + ((killstreak.selected || killstreak.selected_as_bookmark) && "demo--selected")}>
                                    <div>
                                        {#each killstreak.classes as player_class}
                                            <img src={getImgUrl(player_class)} alt="icon" />
                                        {/each}
                                    </div>
                                    <a 
                                        href={`#player-${parsed_demo.data.users[killstreak.kills[0].killer].name}`} 
                                        style="width: 100%; --kills: 0;" 
                                        class={parsed_demo.data.users[killstreak.kills[0].killer]["team"] + " tooltip"}
                                        data-tooltip="Jump To Player"
                                    >
                                        {limitStringLength(parsed_demo.data.users[killstreak.kills[0].killer].name, 16)}
                                    </a>
                                    <div 
                                        data-tooltip={`${
                                            killstreak.kills.length ? 
                                            `Player${(killstreak.kills.length > 1) ? "s" : ""} Killed: ` :
                                            `No Kills`
                                        }\n\r${killstreak.kills.map((kill) => {
                                            let crit_types = ["", " Mini-Crit", " CRITICAL HIT!"]
                                            return `${parsed_demo.data.users[kill.victim].name} (tick: ${kill.tick - parsed_demo.data.start_tick})${crit_types[kill.crit_type]}`
                                        }).join(", \n\r")}`}
                                        style={`--kills: ${killstreak.kills.length};`}
                                        on:click={toggleBookmarkSelected(killstreak, true)}
                                        on:keydown={toggleBookmarkSelected(killstreak, true)}
                                        class={
                                            `tooltip ` +
                                            (killstreak.kills.length >= 3 && " killstreak ") +
                                            (killstreak.kills.length >= 5 && " killstreak--large ") +
                                            (killstreak.kills.length >= 10 && " killstreak--massive ")
                                        }
                                    >
                                        Kills: {killstreak.kills.length}
                                    </div>
                                    <div>
                                        First: {killstreak.kills[0].tick - parsed_demo.data.start_tick}
                                    </div>
                                    <div>
                                        Last: {killstreak.kills[killstreak.kills.length - 1].tick - parsed_demo.data.start_tick}
                                    </div>
                                    <div class="killstreak__buttons">
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
                        <h2 class="centered chat__title">Timeline</h2>
                        <div class="timeline">
                            <div class="timeline__labels">
                                {#each ["blue", "red"] as team}
                                    {#each Object.keys(parsed_demo.data.player_lives) as player}
                                        {#if displayPlayer(player) & parsed_demo.data?.users[player]?.team === team}
                                            <div class={`timeline__label ${team}`}>
                                                {parsed_demo.data?.users[player]?.name}
                                            </div>
                                        {/if}
                                    {/each}
                                {/each}
                            </div>
                            <div class="timeline__lives-container" bind:clientWidth={divWidth} bind:this={timeline} on:scroll={()=>sleft=timeline.scrollLeft} use:ref>
                                {#each ["blue", "red"] as team}
                                    {#each Object.keys(parsed_demo.data.player_lives) as player}
                                        {#if displayPlayer(player) & parsed_demo.data?.users[player]?.team === team}
                                            <div class="timeline__lives">
                                                {#each parsed_demo.data.player_lives[player] as life}
                                                    {#if life.start != 0 && (displayLives || life.kills.length > 0 || (displayAssists && life.assists.length > 0))}
                                                        <div class={`timeline__life timeline__life--${team} ${(life.selected ? "demo--selected" : "")}`} on:click={toggleSelected(life)} on:keydown={toggleSelected(life)} style={`
                                                                --length: ${(life.end - life.start)/scale}px;
                                                                --start: ${(life.start - parsed_demo.data.start_tick)/scale}px
                                                            `}
                                                        >
                                                            <div
                                                                class={`timeline__data-tooltip tooltip ${parsed_demo.data?.users[player]?.team == "blue" ? "tooltip__lower" : ""}` }
                                                                data-tooltip={`${
                                                                    life.kills.length ? 
                                                                    `Player${(life.kills.length > 1) ? "s" : ""} Killed: ` :
                                                                    `No Kills`
                                                                }\n\r${life.kills.map((kill) => {
                                                                    let crit_types = ["", " Mini-Crit", " CRITICAL HIT!"]
                                                                    return `${parsed_demo.data.users[kill.victim].name} (tick: ${kill.tick - parsed_demo.data.start_tick})${crit_types[kill.crit_type]}`
                                                                }).join(", \n\r")}`}
                                                                style={`
                                                                    --kills: ${life.kills.length};
                                                                `}
                                                            >
                                                                <div class="timeline__data">
                                                                    <div>
                                                                        {#each life.classes as player_class}
                                                                            <img src={getImgUrl(player_class)} alt="icon"/>
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
                                                                class={`timeline__marker ${parsed_demo.data?.users[player]?.team == "blue" ? "timeline__marker--lower" : ""}`}
                                                                data-tooltip={`Start: ${life.start - parsed_demo.data.start_tick}`}
                                                                style={`
                                                                    --position: -1px;
                                                                    --kills: 0;
                                                                `}
                                                            ></div>
                                                            <div 
                                                                class={`timeline__marker ${parsed_demo.data?.users[player]?.team == "blue" ? "timeline__marker--lower" : ""}`}
                                                                data-tooltip={`End: ${life.end - parsed_demo.data.start_tick}`}
                                                                style={`
                                                                    --position: ${((life.end - life.start) / scale) - 2}px;
                                                                    --kills: 0;
                                                                `}
                                                            ></div>
                                                            {#each life.kills as kill} 
                                                                <div 
                                                                    class={`timeline__marker ${parsed_demo.data?.users[player]?.team == "blue" ? "timeline__marker--lower" : ""}`}
                                                                    data-tooltip={`Killed: ${parsed_demo.data.users[kill.victim].name}\r\nTick: ${kill.tick - parsed_demo.data.start_tick}`}
                                                                    style={`
                                                                        --position: ${((kill.tick - life.start) / scale) - 2}px;
                                                                        --kills: 1;
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
                            <div class="settings__input-group">
                                <!-- <label for="myRange" class="settings__label">scale</label> -->
                                <input type="range" min="1" max={parsed_demo?.header?.ticks / divWidth} step=".01" bind:value={scale} class="input__slider" id="myRange">
                            </div>
                            <div class="timeline__states">
                                <div>
                                    Zoom: {Math.round((scale / (parsed_demo?.header?.ticks / divWidth)) * 100)}% 
                                </div>
                                <div>
                                    Range: {Math.round(sleft * scale)} - {Math.min(Math.round((sleft + divWidth) * scale), parsed_demo?.header?.ticks)}
                                </div>
                                <div>
                                    Pos: {Math.min(Math.round((sleft + x) * scale), parsed_demo?.header?.ticks)}
                                </div>
                            </div>
                        </div>
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

    .tooltip {
        position: relative;
        cursor: pointer;

        &::before {
            z-index: 1000;
            content: attr(data-tooltip);
            position: absolute;
            top: calc(-2.4rem - (1.72rem * var(--kills)));
            left: 0rem;
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
        }

        &::after {
            z-index: 1000;
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

        &__lower {
            &::before {
                top: 1.9rem;
                background-color: var(--bg);
                z-index: 1000;
            }

            &::after {
                top: 1.45rem;
                z-index: 1000;
                clip-path: polygon(50% 0, 0 100%, 100% 100%);
            }
        }

        &--left {
            &::before {
                left: auto;
                right: 0rem;
            }

            &::after {
                left: auto;
                right: .5rem;
            }
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

    .timeline {
        display: grid;
        grid-template-columns: min-content 1fr;
        text-align: right;
        margin-top: 3rem;
        overflow-y: visible;
        padding: .5rem;
        border-radius: 5px;
        border: 1px solid var(--tert-con);

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
            overflow-x: scroll;
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
                // top: calc(100% - .5rem + 1px);

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

        &__life {
            grid-template-columns: .5fr 1fr 1fr 1fr 1fr min-content;
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