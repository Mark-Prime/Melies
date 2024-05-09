<script>
  // @ts-nocheck
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";

  let index = 0,
    total = 0;

  import Timeline from "./timeline/timeline.svelte";
  import ClassLogo from "../classlogo.svelte";
  import Life from "./demo_life.svelte";
  import KillstreakPointer from "./demo_ks_pointer.svelte";
  import AllKillstreaksPointer from "./demo_all_ks_pointer.svelte";
  import KillPointerList from "./demo_med_picks.svelte";
  import AllKillPointers from "./demo_all_med_picks.svelte";

  export let enabled;
  export let toggle;
  export let parseDemoEvents;
  export let modified;

  let resp = { loaded: false, loading: false };
  let parsed_demo = { loaded: false, loading: false };
  let selected = [];
  let bluTeam = [];
  let redTeam = [];
  let displayLives = false;
  let displayAssists = false;
  let displayPlayers = false;
  let isShiftDown = false;
  let isPovDemo = false;
  let displayChat = false;
  let povId = 0;
  let last_selected = 0;

  let current_demo = "";

  let settings = {};
  let recording_settings = {};

  async function loadSettings() {
    settings = await invoke("load_settings");
    recording_settings = settings.recording;
  }

  async function loadDemos() {
    try {
      resp = await invoke("load_demos");
    } catch (error) {
      alert(error);
    }
  }

  // function calcTick(tick) {
  // if (!parsed_demo.data.pause_tick) {
  //     return tick - parsed_demo.data.start_tick
  // }

  // if (tick < parsed_demo.data.pause_tick) {
  //     return tick - parsed_demo.data.start_tick
  // }

  // return tick - parsed_demo.data.start_tick + parsed_demo.data.pause_length

  //     return tick
  // }

  function closeModal() {
    selected = [];
    current_demo = "";
    parsed_demo = { loaded: false, loading: false };
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
    loadDemos();
  }

  function limitStringLength(str, len) {
    if (str.length < len) {
      return str;
    }

    return str.substring(0, len - 3) + "...";
  }

  function getMessageName(chat) {
    if (chat.from === 1 || chat.from === 0) {
      return "Spectator";
    }

    if (chat.name) {
      return chat.name;
    }

    return parsed_demo.data?.users[chat.from]?.name;
  }

  function toggleSelected(demo, isKillstreak = null, i = null) {
    demo.selected = !demo.selected;

    if (i !== null) {
      if (isShiftDown) {
        for (let index in resp.demos) {
          if (
            index > Math.min(i, last_selected) &&
            index < Math.max(i, last_selected)
          ) {
            resp.demos[index].selected = !resp.demos[index].selected;
          }
        }
      }

      last_selected = i;
    }

    if (demo.selected_as_bookmark) {
      demo.selected_as_bookmark = false;
    }

    if (isKillstreak) {
      let player = parsed_demo.data.player_lives[demo.kills[0].killer];
      for (let life of player) {
        for (let killstreak of life.killstreak_pointers) {
          if (
            JSON.stringify(killstreak.kills[0]) ===
            JSON.stringify(demo.kills[0])
          ) {
            toggleSelected(killstreak);
            break;
          }
        }
      }
    } else if (isKillstreak === false) {
      for (let killstreak of parsed_demo.data.killstreak_pointers) {
        if (
          JSON.stringify(killstreak.kills[0]) === JSON.stringify(demo.kills[0])
        ) {
          toggleSelected(killstreak);
          break;
        }
      }
    }

    resp = resp;
    parsed_demo = parsed_demo;
  }

  function toggleKillsSelected(kills, isKillstreak = null, i = null) {
    for (let kill of kills) {
      if (kill.selected) {
        kill.selected = false;
        continue;
      }

      kill.selected = true;
    }

    resp = resp;
    parsed_demo = parsed_demo;
  }

  function toggleBookmarkSelected(demo, isKillstreak = null) {
    demo.selected_as_bookmark = !demo.selected_as_bookmark;

    if (demo.selected) {
      demo.selected = false;
    }

    // if (isKillstreak) {
    //   let player = parsed_demo.data.player_lives[demo.kills[0].killer];
    //   for (let life of player) {
    //     for (let killstreak of life.killstreaks) {
    //       if (
    //         JSON.stringify(killstreak.kills[0]) ===
    //         JSON.stringify(demo.kills[0])
    //       ) {
    //         toggleBookmarkSelected(killstreak);
    //         break;
    //       }
    //     }
    //   }
    // } else if (isKillstreak === false) {
    //   for (let killstreak of parsed_demo.data.killstreaks) {
    //     if (
    //       JSON.stringify(killstreak.kills[0]) === JSON.stringify(demo.kills[0])
    //     ) {
    //       toggleBookmarkSelected(killstreak);
    //       break;
    //     }
    //   }
    // }

    resp = resp;
    parsed_demo = parsed_demo;
  }

  function parseDemos() {
    for (let demo of resp.demos) {
      if (demo.selected) {
        selected.push(demo.name);
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
              time: life.start + 20,
              label: `${life.kills.length}k-${life.assists.length}a_start`,
              steamid64: parsed_demo.data.users[i].steamId64,
              kills: life.kills.length,
              start: true,
            });

            events.push({
              time: life.end + 132,
              label: `${life.kills.length}k-${life.assists.length}a_end`,
            });
          }

          for (let kill of life.kills) {
            if (kill.selected) {
              events.push({
                time: kill.tick + 20,
                label: `k-${kill.killer_class}_v-${kill.victim_class}`,
                steamid64: parsed_demo.data.users[i].steamId64,
                bookmark: true,
              });
            }
          }

          for (let ks_pointer of life.killstreak_pointers) {
            if (ks_pointer.selected) {
              events.push({
                time: life.kills[ks_pointer.kills[ks_pointer.kills.length - 1]]
                  .tick,
                label: `${ks_pointer.kills.length}ks`,
                steamid64: parsed_demo.data.users[i].steamId64,
                kills: ks_pointer.kills.length,
                killstreak: true,
              });

              continue;
            }

            if (ks_pointer.selected_as_bookmark) {
              let start_time =
                life.kills[ks_pointer.kills[0]].tick -
                recording_settings.before_killstreak_per_kill;
              let end_time =
                life.kills[ks_pointer.kills[ks_pointer.kills.length - 1]].tick +
                recording_settings.after_killstreak;

              if (life.start + 20 > start_time) {
                start_time = life.start + 20;
              }

              if (life.end + 132 < end_time) {
                end_time = life.end + 132;
              }

              events.push({
                time: start_time,
                label: `${ks_pointer.kills.length}ks_start`,
                steamid64: parsed_demo.data.users[i].steamId64,
                kills: ks_pointer.kills.length,
                start: true,
              });

              events.push({
                time: end_time,
                label: `${ks_pointer.kills.length}ks_end`,
              });
            }
          }
        }
      }

      for (let i in parsed_demo.data.chat) {
        let message = parsed_demo.data.chat[i];

        if (message.selected) {
          events.push({
            time: message.tick,
            label: `message-sent`,
            steamid64: parsed_demo.data.users[message.from].steamId64,
            bookmark: true,
          });
        }
      }

      let name_split = current_demo.replace(".dem", "").split("\\");

      parseDemoEvents(
        name_split[name_split.length - 1],
        events.sort((a, b) => a.time - b.time)
      );
    }

    modified();

    if (selected.length !== 0) {
      parsed_demo = { loaded: false, loading: true };
      current_demo = selected.shift();
      parsed_demo = await invoke("parse_demo", { path: current_demo });

      verifyTicks();
      isPovDemo = isDemoPov();

      console.log("isPovDemo", isPovDemo);
      console.log("povId", povId);

      // Sort the team order by class
      bluTeam.sort(sortByClass);
      redTeam.sort(sortByClass);

      console.log(parsed_demo);
    } else {
      closeModal();
    }
  }

  function sortByClass(a, b) {
    let a1 = classNumConverter(getClasses(a)[0]);
    let b1 = classNumConverter(getClasses(b)[0]);

    return a1 - b1;
  }

  function isDemoPov() {
    let nick = parsed_demo.header.nick;

    bluTeam = [];
    redTeam = [];
    povId = 0;

    for (let user in parsed_demo.data.users) {
      let username = parsed_demo.data.users[user].name;

      if (username === nick) {
        povId = user;
        return true;
      }

      let team = parsed_demo.data.users[user].team;

      if (team === "other") {
        continue;
      }

      if (team === "blue") {
        bluTeam.push(user);
        continue;
      }

      redTeam.push(user);
    }

    return false;
  }

  function displayPlayer(player, team) {
    if (player == 0) {
      return false;
    }

    if (displayPlayers) {
      return true;
    }

    if (parsed_demo.data?.users[player]?.team !== team) {
      return false;
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

  function recordEntireDemo(userId) {
    let events = [
      {
        time: settings.recording.start_delay,
        label: parsed_demo.data?.users[userId].steamId64,
        steamid64: parsed_demo.data?.users[userId].steamId64,
        kills: 0,
        start: true,
      },
      {
        time: Math.max(
          parsed_demo.header?.ticks - 99,
          settings.recording.start_delay + 66
        ),
        label: parsed_demo.data?.users[userId].steamId64,
        steamid64: parsed_demo.data?.users[userId].steamId64,
      },
    ];

    let name_split = current_demo.replace(".dem", "").split("\\");

    parseDemoEvents(
      name_split[name_split.length - 1],
      events.sort((a, b) => a.time - b.time)
    );
    nextDemo();
  }

  function getClasses(playerId) {
    let player = parsed_demo.data.users[playerId];
    let playerClasses;
    try {
      playerClasses = Object.keys(player.classes);
    } catch (TypeError) {
      console.log(player);
    }

    playerClasses.sort((a, b) => {
      let a1 = player.classes[a];
      let a2 = classNumConverter(player.classes[a]);

      let b1 = player.classes[b];
      let b2 = classNumConverter(player.classes[b]);

      return b1 - a1 || b2 - a2;
    });

    return playerClasses;
  }

  function getTeam(team) {
    if (isPovDemo) {
      return [povId];
    }

    if (team === "blue") {
      return bluTeam;
    }

    return redTeam;
  }

  function getKillstreaks() {
    if (!isPovDemo) {
      return parsed_demo.data.killstreak_pointers;
    }

    return [];
  }

  function classConverter(player_class) {
    switch (player_class) {
      case "1":
        return "scout";
      case "3":
        return "soldier";
      case "7":
        return "pyro";
      case "4":
        return "demoman";
      case "6":
        return "heavy";
      case "9":
        return "engineer";
      case "5":
        return "medic";
      case "2":
        return "sniper";
      case "8":
        return "spy";
      default:
        return player_class;
    }
  }

  function classNumConverter(player_class) {
    switch (player_class) {
      case "1":
        return 1;
      case "3":
        return 2;
      case "7":
        return 3;
      case "4":
        return 4;
      case "6":
        return 5;
      case "9":
        return 6;
      case "5":
        return 7;
      case "2":
        return 8;
      case "8":
        return 9;
      default:
        return player_class;
    }
  }

  function getLifeFromKillstreak(ks) {
    for (let life of parsed_demo.data.player_lives[ks.kills[0].killer]) {
      if (life.killstreak_pointers.length === 0) {
        continue;
      }

      for (let killstreak of life.killstreak_pointers) {
        if (
          JSON.stringify(killstreak.kills[0]) === JSON.stringify(ks.kills[0])
        ) {
          return life;
        }
      }
    }
  }

  function toggleClass(player, player_class) {
    let class_mapping = [
      "scout",
      "sniper",
      "soldier",
      "demoman",
      "medic",
      "heavy",
      "pyro",
      "spy",
      "engineer",
    ];

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

    // parsed_demo.header.ticks = parsed_demo.data.end_tick - parsed_demo.data.start_tick;
    parsed_demo.header.ticks = parsed_demo.data.start_tick;
  }

  function refreshList() {
    displayLives = displayLives;
    displayAssists = displayAssists;
    displayPlayers = displayPlayers;
    resp = resp;
    parsed_demo = parsed_demo;
  }

  function tickToTime(ticks) {
    return `${Math.floor(Math.round(ticks / 66) / 60)}m ${
      Math.round(ticks / 66) % 60
    }s`;
  }

  function on_key_down(event) {
    if (event.repeat) return;

    switch (event.key) {
      case "Shift":
        isShiftDown = true;

        event.preventDefault();
        break;
    }
  }

  function on_key_up(event) {
    switch (event.key) {
      case "Shift":
        isShiftDown = false;

        event.preventDefault();
        break;
    }
  }

  function allKillsSelected(life) {
    for (let kill of life.kills) {
      if (!kill.selected) {
        return false;
      }
    }

    return true;
  }
</script>

<svelte:window on:keydown={on_key_down} on:keyup={on_key_up} />

{#if enabled}
  <div class="modal">
    <a class="modal__background" on:click={closeModal} href="/"> </a>
    <div class="modal__card" class:modal__card--large={current_demo}>
      {#if resp.loaded}
        {#if current_demo === ""}
          <h1>Demos</h1>
          {#each resp.demos as demo, i}
            <div class={"demo " + (demo.selected && "demo--selected")}>
              <p>{demo.name}</p>
              <div class="add_demo">
                {#if demo.selected}
                  <button
                    class="cancel-btn"
                    on:click={toggleSelected(demo, null, i)}>-</button
                  >
                {:else}
                  <button on:click={toggleSelected(demo, null, i)}>+</button>
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
                  <input
                    type="checkbox"
                    bind:checked={displayLives}
                    on:changed={refreshList}
                  />
                  <span class="slider round slider--tert"></span>
                </label>
                <p>Display all lives</p>
              </div>
              <div class="settings__switch">
                <label class="switch">
                  <input
                    type="checkbox"
                    bind:checked={displayAssists}
                    on:changed={refreshList}
                  />
                  <span class="slider round slider--tert"></span>
                </label>
                <p>Display lives with 0 Kills if they have an Assist</p>
              </div>
              <div class="settings__switch">
                <label class="switch">
                  <input
                    type="checkbox"
                    bind:checked={displayPlayers}
                    on:changed={refreshList}
                  />
                  <span class="slider round slider--tert"></span>
                </label>
                <p>Display players with 0 displayed lives</p>
              </div>
            </div>
            <div class="teams">
              {#each ["blue", "red"] as team}
                <div class="team">
                  <h2 class={"team__label " + team}>
                    {team[0].toUpperCase() + team.slice(1)}
                  </h2>
                  {#each getTeam(team) as player}
                    {#if displayPlayer(player, team)}
                      <div class="flex-start align-center">
                        {#if parsed_demo.data.users[player].hide}
                          <button
                            on:click={() =>
                              (parsed_demo.data.users[player].hide = false)}
                            class="hide-toggle"
                          >
                            +
                          </button>
                        {:else}
                          <button
                            on:click={() =>
                              (parsed_demo.data.users[player].hide = true)}
                            class="cancel-btn hide-toggle"
                          >
                            -
                          </button>
                        {/if}
                        <h3 class="player__header">
                          <a
                            href={`https://logs.tf/profile/${parsed_demo.data.users[player]["steamId64"]}`}
                            class={parsed_demo.data.users[player]["team"] +
                              " player"}
                            data-tooltip="Open logs.tf profile"
                            target="_blank"
                            rel="noopener noreferrer"
                            id={`player-${parsed_demo.data.users[player].name}`}
                          >
                            {parsed_demo.data.users[player].name}
                          </a>
                          {#each getClasses(player) as player_class}
                            <ClassLogo
                              player_class={classConverter(player_class)}
                              tooltip={`Lives: ${parsed_demo.data.users[player]["classes"][player_class]}`}
                              click={toggleClass}
                              args={[player, player_class]}
                            />
                          {/each}
                        </h3>
                      </div>
                      {#if !parsed_demo.data.users[player].hide}
                        {#each parsed_demo.data.player_lives[player] as life}
                          {#if life.start != 0}
                            {#if displayLives || life.kills.length > 0 || (displayAssists && life.assists.length > 0)}
                              <Life
                                {life}
                                {classConverter}
                                {toggleSelected}
                                {parsed_demo}
                                {tickToTime}
                                {toggleKillsSelected}
                                {allKillsSelected}
                              />
                            {/if}
                          {/if}
                        {/each}
                        <button
                          class="full_demo"
                          on:click={() => recordEntireDemo(player)}
                        >
                          Record entire demo
                        </button>

                        <KillPointerList
                          label="Med Picks"
                          valKey="med_picks"
                          {player}
                          {classConverter}
                          {parsed_demo}
                          {tickToTime}
                          {toggleSelected}
                          {toggleKillsSelected}
                          lives={parsed_demo.data.player_lives[player].filter(
                            (life) => life.med_picks.length > 0
                          )}
                        />
                        <!-- <KillPointerList
                          label="Air Shots"
                          valKey="airshots"
                          {player}
                          {classConverter}
                          {parsed_demo}
                          {tickToTime}
                          {toggleKillsSelected}
                          lives={parsed_demo.data.player_lives[player].filter(
                            (life) => life.airshots.length > 0
                          )}
                        /> -->
                        {#if parsed_demo.data.player_lives[player].filter((life) => life.killstreak_pointers.length > 0).length > 0}
                          <h4 class="centered">Killstreaks</h4>
                          {#each parsed_demo.data.player_lives[player].filter((life) => life.killstreak_pointers.length > 0) as life}
                            {#each life.killstreak_pointers as ks_pointer}
                              <KillstreakPointer
                                {classConverter}
                                {toggleSelected}
                                {parsed_demo}
                                {tickToTime}
                                {ks_pointer}
                                {toggleBookmarkSelected}
                                {isPovDemo}
                              />
                            {/each}
                          {/each}
                        {/if}
                      {/if}
                    {/if}
                  {/each}
                </div>
              {/each}
            </div>
            {#if !isPovDemo}
              <div class="kill_pointers">
                <AllKillPointers
                  label="Med Picks"
                  {classConverter}
                  {parsed_demo}
                  {tickToTime}
                  {toggleKillsSelected}
                  {toggleSelected}
                  {isPovDemo}
                  {povId}
                  kills={parsed_demo.data.med_picks}
                />
                <AllKillstreaksPointer
                  killstreaks={parsed_demo.data.killstreak_pointers}
                  {parsed_demo}
                  {limitStringLength}
                  {classConverter}
                  {toggleBookmarkSelected}
                  {tickToTime}
                  {toggleSelected}
                  {isPovDemo}
                />
                <!-- <AllKillPointers
                  label="Air Shots"
                  {classConverter}
                  {parsed_demo}
                  {tickToTime}
                  {toggleKillsSelected}
                  kills={parsed_demo.data.airshots}
                /> -->
              </div>
            {/if}
            {#if parsed_demo.data.chat.length > 0}
              <div class="section-title-toggle chat__title">
                {#if displayChat}
                  <button
                    on:click={() => (displayChat = false)}
                    class="cancel-btn hide-toggle"
                  >
                    -
                  </button>
                {:else}
                  <button
                    on:click={() => (displayChat = true)}
                    class="hide-toggle"
                  >
                    +
                  </button>
                {/if}
                <h2 class="centered">Chat</h2>
              </div>
              {#if displayChat}
                <div class="chat">
                  {#each parsed_demo.data.chat as chat}
                    {#if chat.selected}
                      <button class="cancel-btn" on:click={toggleSelected(chat)}
                        >-</button
                      >
                    {:else}
                      <button on:click={toggleSelected(chat)}>+</button>
                    {/if}
                    <div class="chat__tick">
                      {chat.tick}
                    </div>
                    <div class="chat__text">
                      <a
                        href={`#player-${chat.name}`}
                        class={`chat__name ${
                          parsed_demo.data?.users[chat.from]?.team
                        }`}
                      >
                        {getMessageName(chat)}{getMessageType(chat.kind)}:
                      </a>
                      {chat.text}
                    </div>
                  {/each}
                </div>
              {/if}
            {/if}
            <Timeline
              {parsed_demo}
              {tickToTime}
              {displayPlayer}
              {toggleSelected}
              {displayLives}
              {displayAssists}
              {getTeam}
            />
            <div class="buttons">
              <button class="cancel-btn" on:click={closeModal}>Cancel</button>
              <button on:click={nextDemo}>Save</button>
            </div>
          {:else}
            <div class="loading">
              <div class="loadingio-spinner-dual-ball-gstkvx2ybq5">
                <div class="ldio-h6cxzkuee3g">
                  <div></div>
                  <div></div>
                  <div></div>
                </div>
              </div>
              <h4>Loading {index}/{total}...</h4>
            </div>
          {/if}
        {:else if parsed_demo.err_text}
          <h1>Error: {parsed_demo.code}</h1>
          <h2 class="centered">{parsed_demo.err_text}</h2>

          <div class="buttons">
            <button class="cancel-btn" on:click={closeModal}>Cancel</button>
            <button on:click={nextDemo}>Skip</button>
          </div>
        {/if}
      {:else}
        <h1>LOADING DEMOS</h1>
      {/if}
    </div>
  </div>
{/if}

<style lang="scss">
  .kill_pointers {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(700px, 1fr));
    width: 100%;
    gap: 1rem;
  }

  .section-title-toggle {
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .killstreak__buttons {
    display: flex;
    gap: 1rem;
    justify-content: end;
    grid-column: 6;
  }

  .input__slider {
    -webkit-appearance: none; /* Override default CSS styles */
    appearance: none;
    width: 95%; /* Full-width */
    height: 0px; /* Specified height */
    background: var(--tert); /* Grey background */
    outline: none; /* Remove outline */
    opacity: 0.7; /* Set transparency (for mouse-over effects on hover) */
    -webkit-transition: 0.2s; /* 0.2 seconds transition on hover */
    transition: opacity 0.2s;
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
    gap: 0.2rem;
    border: 1px solid var(--tert-con);
    border-radius: 5px;
    padding: 1rem;

    &__tick {
      text-align: right;
      margin: 0 0.3rem;
      padding-right: 0.5rem;
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
    padding: 0.3rem 0.5rem;
    margin: 2px 0;
    font-family: "Source Code Pro", monospace;
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

    transition: all 0.2s;

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
      padding: 0.2rem 0.5rem;
      border-radius: 0.5rem;
      white-space: nowrap;
      font-size: 12px;
    }

    &::after {
      content: "";
      display: none;
      position: absolute;
      top: -0.2rem;
      left: 0.5rem;
      height: 0.5rem;
      width: 0.8rem;
      background-color: var(--outline);
      clip-path: polygon(100% 0, 0 0, 50% 100%);
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
      }
    }
  }

  .auto-height {
    height: 25.38px;
    line-height: 0;
  }

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

    &__icon {
      display: flex;
      justify-content: center;
      align-items: center;
    }

    &__life {
      grid-template-columns: 0.5fr 1fr 1fr 1fr 1fr min-content;
      height: auto;
      max-height: 35.75px;
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
    }

    &__kill-count {
      cursor: pointer;
      transition: all 0.2s;
      background-color: transparent;
      border-radius: 3px 3px 0 0;
    }

    &__killstreak {
      grid-template-columns: 0.5fr 1fr 1fr 1fr min-content;
    }

    &--selected {
      border: 1px solid var(--tert);
    }

    & > div,
    & > p {
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
    overflow: hidden;

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

      /* width */
      &::-webkit-scrollbar {
        width: 12px;
      }

      /* Track */
      &::-webkit-scrollbar-track {
        background: var(--tert);
        border-radius: 0 8px 8px 0;
        overflow: hidden;
      }

      /* Handle */
      &::-webkit-scrollbar-thumb {
        background: var(--tert-con);
        border-radius: 0 8px 8px 0;
      }
    }

    &__background {
      position: fixed;
      background-color: rgba(0, 0, 0, 0.6);
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
    0% {
      opacity: 1;
      transform: translate(0, 0);
    }
    49.99% {
      opacity: 1;
      transform: translate(100px, 0);
    }
    50% {
      opacity: 0;
      transform: translate(100px, 0);
    }
    100% {
      opacity: 0;
      transform: translate(0, 0);
    }
  }
  @keyframes ldio-h6cxzkuee3g {
    0% {
      transform: translate(0, 0);
    }
    50% {
      transform: translate(100px, 0);
    }
    100% {
      transform: translate(0, 0);
    }
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
  .ldio-h6cxzkuee3g div {
    box-sizing: content-box;
  }
</style>
