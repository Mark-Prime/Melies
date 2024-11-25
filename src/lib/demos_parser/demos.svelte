<script>
  import { invoke } from "@tauri-apps/api/core";
  import {
    faWandMagicSparkles,
    faCheck,
    faXmark,
  } from "@fortawesome/free-solid-svg-icons";
  import Fa from "svelte-fa";
  import { createEventDispatcher } from "svelte";
  import dayjs from "dayjs";

  import tickToTime from "$lib/composables/tickToTime.js";
  import isAirshot from "$lib/composables/isAirshot.js";
  import Modal from "$lib/components/Modal.svelte";
  import ClassLogo from "$lib/components/ClassLogo.svelte";
  import Toggle from "$lib/components/ToggleSelected.svelte";

  import Timeline from "./timeline/timeline.svelte";

  const dispatch = createEventDispatcher();

  let index = 0,
    total = 0;

  import Life from "./demo_life.svelte";
  import KillstreakPointer from "./demo_ks_pointer.svelte";
  import AllKillstreaksPointer from "./demo_all_ks_pointer.svelte";
  import KillPointerList from "./demo_med_picks.svelte";
  import AllKillPointers from "./demo_all_med_picks.svelte";

  let enabled = false;
  let toggle = () => (enabled = !enabled);

  let resp = { loaded: false, loading: false };
  let parsedDemo = { loaded: false, loading: false };
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
  let lastSelected = 0;

  let currentDemo = "";

  let settings = {};
  let recordingSettings = {};

  let filterAirshots = (k) => isAirshot(parsedDemo, k, settings);

  function filterLife(lives, valKey) {
    let validLives = lives.filter((life) => life[valKey].length > 0);

    for (let life of validLives) {
      let validKills = [];

      for (let kill of life[valKey]) {
        if (filterAirshots(kill)) {
          validKills.push(kill);
        }
      }

      life[valKey] = validKills;
    }

    return validLives.filter((life) => life[valKey].length > 0);
  }

  async function loadEvents() {
    let eventList = await invoke("load_events");
    let demos = [];

    if (eventList.code === 200) {
      eventList.events.forEach(
        (/** @type {{ demo_name: any; }} */ event, /** @type {number} */ i) => {
          event.isKillstreak = false;

          if (event.value.Killstreak) {
            event.isKillstreak = true;
          }

          if (
            i === 0 ||
            eventList.events[i - 1].demo_name != event.demo_name
          ) {
            demos.push([event]);
            return;
          }

          demos[demos.length - 1].push(event);
        }
      );

      return demos;
    }

    return [];
  }

  async function parseDemoEvents(demo_name, events) {
    console.log("demo_name", demo_name);
    console.log("events", events);

    let demos = await loadEvents();
    let newDemo = [];

    let settings = await invoke("load_settings");
    let recordingSettings = settings.recording;

    let specMode = recordingSettings["third_person"] ? "spec_third" : "spec";

    for (let event of events) {
      let spectate = `${specMode} ${event.steamid64}`;

      if (event.start) {
        newDemo.push({
          value: {
            Bookmark: `clip_start ${event.steamid64 !== undefined ? spectate : ""}`,
          },
          tick: event.time,
          demo_name: demo_name,
          event: `[demo_${event.label}] clip_start ${event.steamid64 !== undefined ? spectate : ""} (\"${demo_name}\" at ${event.time})`,
          isKillstreak: false,
        });

        continue;
      }

      if (event.bookmark) {
        newDemo.push({
          value: {
            Bookmark: event.steamid64 !== undefined ? spectate : "General",
          },
          tick: event.time,
          demo_name: demo_name,
          event: `[demo_${event.label}] ${event.steamid64 !== undefined ? spectate : "General"} (\"${demo_name}\" at ${event.time})`,
          isKillstreak: false,
        });

        continue;
      }

      if (event.killstreak) {
        newDemo.push({
          value: {
            Killstreak: event.kills,
          },
          tick: event.time,
          demo_name: demo_name,
          event: `[demo_${event.label}] Killstreak ${event.kills} (\"${demo_name}\" at ${event.time})`,
          isKillstreak: true,
        });

        continue;
      }

      newDemo.push({
        value: {
          Bookmark: `clip_end`,
        },
        tick: event.time,
        demo_name: demo_name,
        event: `[demo_${event.label}] clip_end (\"${demo_name}\" at ${event.time})`,
        isKillstreak: false,
      });
    }

    demos.push(newDemo);

    for (let demo of demos) {
      demo.sort((a, b) => a.tick - b.tick);
    }

    await invoke("save_events", { newEvents: demos });
    dispatch("reload");
  }

  async function loadSettings() {
    settings = await invoke("load_settings");
    recordingSettings = settings.recording;
  }

  async function loadDemos() {
    try {
      resp = await invoke("load_demos");
    } catch (error) {
      alert(error);
    }
  }

  function closeModal() {
    selected = [];
    currentDemo = "";
    parsedDemo = { loaded: false, loading: false };

    for (let demo of resp.demos) {
      demo.selected = false;
    }

    dispatch("reload");
    toggle();
  }

  $: {
    console.log("Modal Enabled:", enabled);
    loadDemos();
    loadSettings();
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

    return parsedDemo.data?.users[chat.from]?.name;
  }

  function toggleSelected(demo, isKillstreak = null, i = null) {
    demo.selected = !demo.selected;

    if (i !== null) {
      if (isShiftDown) {
        for (let index in resp.demos) {
          if (
            index > Math.min(i, lastSelected) &&
            index < Math.max(i, lastSelected)
          ) {
            resp.demos[index].selected = !resp.demos[index].selected;
          }
        }
      }

      lastSelected = i;
    }

    if (demo.selected_as_bookmark) {
      demo.selected_as_bookmark = false;
    }

    if (isKillstreak) {
      let player = parsedDemo.data.player_lives[demo.kills[0].killer];
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
      for (let killstreak of parsedDemo.data.killstreak_pointers) {
        if (
          JSON.stringify(killstreak.kills[0]) === JSON.stringify(demo.kills[0])
        ) {
          toggleSelected(killstreak);
          break;
        }
      }
    }

    resp = resp;
    parsedDemo = parsedDemo;
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
    parsedDemo = parsedDemo;
  }

  function toggleBookmarkSelected(demo, isKillstreak = null) {
    demo.selected_as_bookmark = !demo.selected_as_bookmark;

    if (demo.selected) {
      demo.selected = false;
    }

    resp = resp;
    parsedDemo = parsedDemo;
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

  async function nextDemo(skipScan = false) {
    index += 1;

    if (currentDemo != "" && skipScan !== true) {
      let events = [];

      for (let i in parsedDemo.data.player_lives) {
        let player = parsedDemo.data.player_lives[i];

        for (let life of player) {
          if (life.selected) {
            events.push({
              time: life.start + 20,
              label: `${life.kills.length}k-${life.assists.length}a_start`,
              steamid64: parsedDemo.data.users[i].steamId64,
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
                steamid64: parsedDemo.data.users[i].steamId64,
                bookmark: true,
              });
            }
          }

          for (let ksPointer of life.killstreak_pointers) {
            if (ksPointer.selected) {
              events.push({
                time: life.kills[ksPointer.kills[ksPointer.kills.length - 1]]
                  .tick,
                label: `${ksPointer.kills.length}ks`,
                steamid64: parsedDemo.data.users[i].steamId64,
                kills: ksPointer.kills.length,
                killstreak: true,
              });

              continue;
            }

            if (ksPointer.selected_as_bookmark) {
              let start_time =
                life.kills[ksPointer.kills[0]].tick -
                recordingSettings.before_killstreak_per_kill;
              let end_time =
                life.kills[ksPointer.kills[ksPointer.kills.length - 1]].tick +
                recordingSettings.after_killstreak;

              if (life.start + 20 > start_time) {
                start_time = life.start + 20;
              }

              if (life.end + 132 < end_time) {
                end_time = life.end + 132;
              }

              events.push({
                time: start_time,
                label: `${ksPointer.kills.length}ks_start`,
                steamid64: parsedDemo.data.users[i].steamId64,
                kills: ksPointer.kills.length,
                start: true,
              });

              events.push({
                time: end_time,
                label: `${ksPointer.kills.length}ks_end`,
              });
            }
          }
        }
      }

      for (let i in parsedDemo.data.chat) {
        let message = parsedDemo.data.chat[i];

        if (message.selected) {
          let steamid64 = parsedDemo.data.users[message.from]?.steamId64;

          events.push({
            time: message.tick,
            label: `message-sent`,
            bookmark: true,
            steamid64,
          });
        }
      }

      let demo_name = currentDemo.replace(".dem", "").substring(1);

      if (!settings.absolute_file_paths) {
        demo_name = demo_name.replace("demos\\", "");
      }

      if (events.length !== 0) {
        parseDemoEvents(
          demo_name,
          events.sort((a, b) => a.time - b.time)
        );
      }
    }

    if (selected.length !== 0) {
      parsedDemo = { loaded: false, loading: true };
      currentDemo = selected.shift();
      parsedDemo = await invoke("parse_demo", { path: currentDemo });

      verifyTicks();
      labelRounds();
      isPovDemo = isDemoPov();

      console.log(settings);

      // Sort the team order by class
      bluTeam.sort(sortByClass);
      redTeam.sort(sortByClass);

      console.log(parsedDemo);
    } else {
      closeModal();
    }
  }

  function labelRounds() {
    let roundNum = 1;

    for (let i = 0; i < parsedDemo.data.rounds.length; i++) {
      const round = parsedDemo.data.rounds[i];

      if (round.is_pregame) {
        if (i === 0) {
          round.label = `Pregame`;
          continue;
        }

        if (i === parsedDemo.data.rounds.length - 1) {
          round.label = `Postgame`;
          continue;
        }

        if (round.end_tick - round.start_tick < 1000) {
          round.label = `Humiliation`;
          continue;
        }

        round.label = `Halftime`;
        continue;
      }

      round.label = `Round ${roundNum++}`;
    }
  }

  function sortByClass(a, b) {
    let a1 = classNumConverter(getClasses(a)[0]);
    let b1 = classNumConverter(getClasses(b)[0]);

    return a1 - b1;
  }

  function isDemoPov() {
    let nick = parsedDemo.header.nick;

    bluTeam = [];
    redTeam = [];
    povId = 0;

    for (let user in parsedDemo.data.users) {
      let username = parsedDemo.data.users[user].name;

      if (username === nick) {
        if (!settings.pov_as_stv) {
          povId = user;
          return true;
        }
      }

      let team = parsedDemo.data.users[user].team;

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

    if (parsedDemo.data?.users[player]?.team !== team) {
      return false;
    }

    if (parsedDemo.data.player_lives[player].length == 0) {
      return false;
    }

    if (displayLives) {
      return true;
    }

    for (let life of parsedDemo.data.player_lives[player]) {
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
        label: parsedDemo.data?.users[userId].steamId64,
        steamid64: parsedDemo.data?.users[userId].steamId64,
        kills: 0,
        start: true,
      },
      {
        time: Math.max(
          parsedDemo.header?.ticks - 99,
          settings.recording.start_delay + 66
        ),
        label: parsedDemo.data?.users[userId].steamId64,
        steamid64: parsedDemo.data?.users[userId].steamId64,
      },
    ];

    let demo_name = currentDemo.replace(".dem", "").substring(1);

    if (!settings.absolute_file_paths) {
      demo_name = demo_name.replace("demos\\", "");
    }

    parseDemoEvents(demo_name, events);

    nextDemo();
  }

  function bookmarkHighlight(
    spectate,
    userId,
    demo_name,
    label,
    tick,
    demo_count,
    isPovDemo
  ) {
    let name = demo_name;

    if (!isPovDemo) {
      name = name + "_" + demo_count;
    }

    return {
      value: {
        Bookmark: `${label}${isPovDemo ? "" : " " + spectate}`,
      },
      tick: tick,
      demo_name: name,
      event: `[demo_${parsedDemo.data?.users[userId].steamId64}] ${label} ${isPovDemo ? "" : spectate + " "}(\"${name}\" at ${tick})`,
      isKillstreak: false,
    };
  }

  function bookmarkLife(
    spectate,
    demo_name,
    userId,
    life,
    demo_count,
    isPovDemo
  ) {
    let newDemo = [];

    let name = demo_name;

    if (!isPovDemo) {
      name = name + "_" + demo_count;
    }

    newDemo.push({
      value: {
        Bookmark: `clip_start${isPovDemo ? "" : " " + spectate}`,
      },
      tick: life.start + 20,
      demo_name: name,
      event: `[demo_${parsedDemo.data?.users[userId].steamId64}] clip_start ${isPovDemo ? "" : spectate + " "}(\"${name}\" at ${life.start + 20})`,
      isKillstreak: false,
    });

    newDemo.push({
      value: {
        Bookmark: `clip_end`,
      },
      tick: life.end + 132,
      demo_name: name,
      event: `[demo_${parsedDemo.data?.users[userId].steamId64}] clip_end (\"${name}\" at ${life.end + 132})`,
      isKillstreak: false,
    });

    return newDemo;
  }

  function shouldRecordLife(life) {
    for (const class_name of life.classes) {
      if (settings.automation.classes[class_name]) {
        return true;
      }
    }

    return false;
  }

  function shouldRecordKill(killer_class) {
    if (settings.automation.classes[killer_class]) {
      return true;
    }

    return false;
  }

  async function recordAllHighlights() {
    let demos = await loadEvents();
    let settings = await invoke("load_settings");
    let recordingSettings = settings.recording;

    let specMode = recordingSettings["third_person"] ? "spec_third" : "spec";

    let demo_count = 1;

    for (let userId in parsedDemo.data.users) {
      if (isPovDemo) {
        if (povId != userId) {
          continue;
        }
      }

      let newDemo = [];

      let name_split = currentDemo.replace(".dem", "").split("\\");

      let demo_name = name_split[name_split.length - 1];

      let spectate = `${specMode} ${parsedDemo.data?.users[userId].steamId64}`;

      if (
        !parsedDemo.data.player_lives[userId] ||
        parsedDemo.data.player_lives[userId].length == 0
      ) {
        continue;
      }

      for (let i in parsedDemo.data.player_lives[userId]) {
        let player = parsedDemo.data.player_lives[userId][i];

        if (player.kills.length == 0) {
          continue;
        }

        if (!shouldRecordLife(player)) {
          continue;
        }

        if (settings.automation.killstreaks) {
          for (let ks of player.killstreak_pointers) {
            if (settings.automation.whole_life) {
              let life = player;

              if (life.selected) {
                continue;
              }

              life.selected = true;

              newDemo = [
                ...newDemo,
                ...bookmarkLife(
                  spectate,
                  demo_name,
                  userId,
                  life,
                  demo_count,
                  isPovDemo
                ),
              ];

              continue;
            }

            for (let k of ks.kills) {
              let kill = player.kills[k];

              if (!shouldRecordKill(kill.killer_class)) {
                continue;
              }

              let label = `KS`;

              if (kill.is_airborne) {
                label = `${label} AS`;
              }

              if (kill.victim_class == "medic") {
                label = `${label} MP`;
              }

              newDemo.push(
                bookmarkHighlight(
                  spectate,
                  userId,
                  demo_name,
                  label,
                  kill.tick,
                  demo_count,
                  isPovDemo
                )
              );

              kill.selected = true;
            }
          }
        }

        if (settings.automation.med_picks) {
          for (let mp of player.med_picks) {
            if (settings.automation.whole_life) {
              let life = player;

              if (life.selected) {
                continue;
              }

              life.selected = true;

              newDemo = [
                ...newDemo,
                ...bookmarkLife(
                  spectate,
                  demo_name,
                  userId,
                  life,
                  demo_count,
                  isPovDemo
                ),
              ];

              continue;
            }

            let kill = player.kills[mp.kill_index];

            if (!shouldRecordKill(kill.killer_class)) {
              continue;
            }

            if (kill.selected) {
              continue;
            }

            kill.selected = true;

            let label = `MP`;

            if (kill.is_airborne) {
              label = `${label} AS`;
            }

            newDemo.push(
              bookmarkHighlight(
                spectate,
                userId,
                demo_name,
                "MP",
                kill.tick,
                demo_count,
                isPovDemo
              )
            );
          }
        }

        if (settings.automation.airshots) {
          for (let as of player.airshots) {
            if (settings.automation.whole_life) {
              let life = player;

              if (life.selected) {
                continue;
              }

              life.selected = true;

              newDemo = [
                ...newDemo,
                ...bookmarkLife(
                  spectate,
                  demo_name,
                  userId,
                  life,
                  demo_count,
                  isPovDemo
                ),
              ];

              continue;
            }

            if (!filterAirshots(as)) {
              continue;
            }

            let kill = player.kills[as.kill_index];

            if (!shouldRecordKill(kill.killer_class)) {
              continue;
            }

            if (kill.selected) {
              continue;
            }

            kill.selected = true;

            newDemo.push(
              bookmarkHighlight(
                spectate,
                userId,
                demo_name,
                "AS",
                kill.tick,
                demo_count,
                isPovDemo
              )
            );
          }
        }
      }

      if (newDemo.length == 0) {
        continue;
      }

      demo_count += 1;

      demos.push(newDemo);
    }

    await invoke("save_events", { newEvents: demos });
    dispatch("reload");

    demos = await loadEvents();

    console.log(demos);

    nextDemo(true);
  }

  async function recordAll() {
    let demos = await loadEvents();
    let settings = await invoke("load_settings");
    let recordingSettings = settings.recording;

    let specMode = recordingSettings["third_person"] ? "spec_third" : "spec";

    for (let userId in parsedDemo.data.users) {
      let newDemo = [];

      let name_split = currentDemo.replace(".dem", "").split("\\");

      let demo_name = name_split[name_split.length - 1];

      let spectate = `${specMode} ${parsedDemo.data?.users[userId].steamId64}`;

      console.log("spec", spectate);

      if (
        !parsedDemo.data.player_lives[userId] ||
        parsedDemo.data.player_lives[userId].length == 0
      ) {
        continue;
      }

      newDemo.push({
        value: {
          Bookmark: `clip_start ${spectate}`,
        },
        tick: parsedDemo.data.player_lives[userId][0].start + 20,
        demo_name: demo_name + "_" + userId,
        event: `[demo_${parsedDemo.data?.users[userId].steamId64}] clip_start  ${spectate} (\"${demo_name}\" at ${settings.recording.start_delay})`,
        isKillstreak: false,
      });

      newDemo.push({
        value: {
          Bookmark: `clip_end`,
        },
        tick: Math.max(
          parsedDemo.header?.ticks - 99,
          settings.recording.start_delay + 66
        ),
        demo_name: demo_name + "_" + userId,
        event: `[demo_${parsedDemo.data?.users[userId].steamId64}] clip_end (\"${demo_name}\" at ${Math.max(
          parsedDemo.header?.ticks - 99,
          settings.recording.start_delay + 66
        )})`,
        isKillstreak: false,
      });

      demos.push(newDemo);
    }

    await invoke("save_events", { newEvents: demos });
    dispatch("reload");

    console.log(demos);

    nextDemo(true);
  }

  function getClasses(playerId) {
    let player = parsedDemo.data.users[playerId];
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
      return parsedDemo.data.killstreak_pointers;
    }

    return [];
  }

  function classConverter(playerClass) {
    switch (playerClass) {
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
        return playerClass;
    }
  }

  function classNumConverter(playerClass) {
    switch (playerClass) {
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
        return playerClass;
    }
  }

  function getLifeFromKillstreak(ks) {
    for (let life of parsedDemo.data.player_lives[ks.kills[0].killer]) {
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

  function toggleClass(player, playerClass) {
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

    if (parsedDemo.data.users[player].hide) {
      return;
    }

    for (let life of parsedDemo.data.player_lives[player]) {
      if (!life.classes.includes(class_mapping[Number(playerClass) - 1])) {
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
    if (parsedDemo?.header?.ticks > 0) {
      return;
    }

    if (parsedDemo.err_text) {
      return;
    }

    // parsedDemo.header.ticks = parsedDemo.data.end_tick - parsedDemo.data.start_tick;
    parsedDemo.header.ticks = parsedDemo.data.start_tick;
  }

  function refreshList() {
    displayLives = displayLives;
    displayAssists = displayAssists;
    displayPlayers = displayPlayers;
    resp = resp;
    parsedDemo = parsedDemo;
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

<button class="btn btn--tert" on:click={toggle}>
  <Fa icon={faWandMagicSparkles} color={`var(--tert)`} />
  Scan Demos
</button>
<Modal
  color="tert"
  {toggle}
  {enabled}
  grow
  wide={resp.loaded && currentDemo !== ""}
>
  {#if resp.loaded}
    {#if currentDemo === ""}
      <h1>Demos</h1>
      <table>
        <thead>
          <tr>
            <th> Name </th>
            <th>Length</th>
            <th>Map</th>
            <th
              class="tooltip tooltip--left"
              data-tooltip={`Does the demo have a vdm?`}
              style="--kills: 0;"
            >
              VDM
            </th>
            <th></th>
          </tr>
        </thead>
        <tbody>
          {#each resp.demos as demo, i}
            <tr
              class={"table_row " +
                (demo.hasVdm && "demo--hasvdm") +
                " " +
                (demo.selected && "demo--selected")}
            >
              <td
                class="tooltip"
                data-tooltip={`nickname: ${demo.header.nick}
created: ${dayjs.unix(demo.metadata.created.secs_since_epoch).format("MMM DD, YYYY")}`}
                style="--kills: 1;"
              >
                {demo.name}
              </td>
              <td>{tickToTime(demo.header.ticks)}</td>
              <td>{demo.header.map}</td>
              <td class="table__has-vdm">
                {#if demo.hasVdm}
                  <span
                    class="tooltip tooltip--left"
                    data-tooltip={`This demo has a VDM.`}
                    style="--kills: 0;"
                  >
                    <Fa icon={faCheck} color={`var(--sec)`} />
                  </span>
                {:else}
                  <span
                    class="tooltip tooltip--left"
                    data-tooltip={`This demo does not have a VDM.`}
                    style="--kills: 0;"
                  >
                    <Fa icon={faXmark} color={`var(--tert)`} />
                  </span>
                {/if}
              </td>
              <td class="add_demo">
                <Toggle value={demo.selected} on:click={toggleSelected(demo, null, i)} />
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    {:else if !parsedDemo.err_text}
      <h1>{currentDemo}</h1>
      {#if !parsedDemo.loading}
        <h4 class="centered full-width">{parsedDemo.header.map}</h4>
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
        {#if settings.automation.enabled}
          <div class="buttons">
            <button class="btn" on:click={recordAll}> Record All Lives </button>
            <button class="btn" on:click={recordAllHighlights}>
              Record All Highlights
            </button>
          </div>
        {/if}
        <div class="teams">
          {#each ["blue", "red"] as team}
            <div class="team">
              <h2 class={"team__label " + team}>
                {team[0].toUpperCase() + team.slice(1)}
              </h2>
              {#each getTeam(team) as player}
                {#if displayPlayer(player, team)}
                  <div class="flex-start align-center">
                    {#if parsedDemo.data.users[player].hide}
                      <button
                        on:click={() =>
                          (parsedDemo.data.users[player].hide = false)}
                        class="hide-toggle"
                      >
                        +
                      </button>
                    {:else}
                      <button
                        on:click={() =>
                          (parsedDemo.data.users[player].hide = true)}
                        class="cancel-btn hide-toggle"
                      >
                        -
                      </button>
                    {/if}
                    <h3 class="player__header">
                      <a
                        href={`https://logs.tf/profile/${parsedDemo.data.users[player]["steamId64"]}`}
                        class={parsedDemo.data.users[player]["team"] +
                          " player"}
                        data-tooltip="Open logs.tf profile"
                        target="_blank"
                        rel="noopener noreferrer"
                        id={`player-${parsedDemo.data.users[player].name}`}
                      >
                        {parsedDemo.data.users[player].name}
                      </a>
                      {#each getClasses(player) as playerClass}
                        <ClassLogo
                          player_class={classConverter(playerClass)}
                          tooltip={`Lives: ${parsedDemo.data.users[player]["classes"][playerClass]}`}
                          click={toggleClass}
                          args={[player, playerClass]}
                        />
                      {/each}
                    </h3>
                  </div>
                  {#if !parsedDemo.data.users[player].hide}
                    {#each parsedDemo.data.player_lives[player] as life}
                      {#if life.start != 0}
                        {#if displayLives || life.kills.length > 0 || (displayAssists && life.assists.length > 0)}
                          <Life
                            {life}
                            {classConverter}
                            {toggleSelected}
                            {parsedDemo}
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
                      {classConverter}
                      {parsedDemo}
                      {tickToTime}
                      {toggleSelected}
                      {toggleKillsSelected}
                      lives={parsedDemo.data.player_lives[player].filter(
                        (life) => life.med_picks.length > 0
                      )}
                    />
                    <KillPointerList
                      label="Air Shots"
                      valKey="airshots"
                      {classConverter}
                      {parsedDemo}
                      {tickToTime}
                      {toggleKillsSelected}
                      {toggleSelected}
                      lives={filterLife(
                        parsedDemo.data.player_lives[player],
                        "airshots"
                      )}
                    />
                    {#if parsedDemo.data.player_lives[player].filter((life) => life.killstreak_pointers.length > 0).length > 0}
                      <h4 class="centered">Killstreaks</h4>
                      {#each parsedDemo.data.player_lives[player].filter((life) => life.killstreak_pointers.length > 0) as life}
                        {#each life.killstreak_pointers as ksPointer}
                          <KillstreakPointer
                            {classConverter}
                            {toggleSelected}
                            {parsedDemo}
                            {tickToTime}
                            {ksPointer}
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
              {parsedDemo}
              {tickToTime}
              {toggleKillsSelected}
              {toggleSelected}
              kills={parsedDemo.data.med_picks}
            />
            <AllKillstreaksPointer
              killstreaks={parsedDemo.data.killstreak_pointers}
              {parsedDemo}
              {limitStringLength}
              {classConverter}
              {toggleBookmarkSelected}
              {tickToTime}
              {toggleSelected}
            />
            <AllKillPointers
              label="Air Shots"
              {classConverter}
              {parsedDemo}
              {tickToTime}
              {toggleKillsSelected}
              {toggleSelected}
              kills={parsedDemo.data.airshots.filter((k) => filterAirshots(k))}
            />
          </div>
        {/if}
        {#if parsedDemo.data.chat.length > 0}
          <div class="section-title-toggle chat__title">
            {#if displayChat}
              <button
                on:click={() => (displayChat = false)}
                class="cancel-btn hide-toggle"
              >
                -
              </button>
            {:else}
              <button on:click={() => (displayChat = true)} class="hide-toggle">
                +
              </button>
            {/if}
            <h2 class="centered">Chat</h2>
          </div>
          {#if displayChat}
            <div class="chat">
              {#each parsedDemo.data.chat as chat}
                <Toggle value={chat.selected} on:click={toggleSelected(chat)} />
                <div class="chat__tick">
                  {chat.tick}
                </div>
                <div class="chat__text">
                  <a
                    href={`#player-${chat.name}`}
                    class={`chat__name ${
                      parsedDemo.data?.users[chat.from]?.team
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
          {parsedDemo}
          {tickToTime}
          {displayPlayer}
          {toggleSelected}
          {displayLives}
          {displayAssists}
          {getTeam}
        />
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
    {:else if parsedDemo.err_text}
      <h1>Error: {parsedDemo.code}</h1>
      <h2 class="centered">{parsedDemo.err_text}</h2>
    {/if}
  {:else}
    <h1>LOADING DEMOS</h1>
  {/if}

  <div class="buttons" slot="footer">
    {#if resp.loaded}
      {#if currentDemo === ""}
        <button class="cancel-btn" on:click={closeModal}>Cancel</button>
        <button on:click={parseDemos}>Parse</button>
      {:else if !parsedDemo.err_text}
        {#if !parsedDemo.loading}
          <button class="cancel-btn" on:click={closeModal}>Cancel</button>
          <button on:click={nextDemo}>Save</button>
        {/if}
      {:else if parsedDemo.err_text}
        <button class="cancel-btn" on:click={closeModal}>Cancel</button>
        <button on:click={nextDemo}>Skip</button>
      {/if}
    {/if}
  </div>
</Modal>

<style lang="scss">
  .demo-labels {
    display: grid;
    grid-template-columns: 2fr 1fr 1fr 1fr 3rem;
    justify-items: center;
    align-items: center;
    gap: 1rem;
    margin-bottom: 1rem;
    padding: 0 1rem;
    margin: 0;
  }

  .btn {
    display: flex;
    gap: 0.5rem;
  }

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

  table {
    border-collapse: separate;
    border-spacing: 0 2px;
    width: 100%;
  }

  th {
    text-align: left;
    white-space: nowrap;
  }

  .table_row {
    font-size: small;
    font-family: "Source Code Pro", monospace;
    color: var(--tert-con-text);
    padding: 0 1rem;
    margin: 2px 0;
    border-radius: 5px;

    transition: all 0.2s;

    background-color: var(--bg2);

    &.demo--hasvdm .tooltip:hover {
      color: var(--sec);
    }

    &:not(.demo--hasvdm) .tooltip:hover {
      color: var(--tert);
    }

    &.demo--hasvdm > td {
      color: var(--sec-con-text);
      border-color: var(--sec-con);
    }

    &:hover {
      &:not(.demo--hasvdm) {
        filter: drop-shadow(0px 0px 3px var(--tert));
      }

      &.demo--hasvdm {
        filter: drop-shadow(0px 0px 3px var(--sec));
      }

      & > td {
        border-color: var(--tert-con-text);
      }

      &.demo--hasvdm > td {
        border-color: var(--sec-con-text);
      }
    }

    & > td {
      text-align: left;
      white-space: nowrap;
      border: 1px solid var(--tert-con);
      border-left-width: 0px;
      border-right-width: 0px;
      padding: 0.3rem 0.25rem;

      &:first-of-type {
        border-left-width: 1px;
        border-radius: 5px 0 0 5px;
        padding-left: 0.5rem;
      }

      &:last-of-type {
        border-right-width: 1px;
        border-radius: 0 5px 5px 0;
        padding-right: 0.5rem;

        overflow: hidden;
      }
    }
  }

  .demo {
    font-size: small;
    padding: 0.3rem 0.5rem;
    margin: 2px 0;
    font-family: "Source Code Pro", monospace;
    color: var(--tert-con-text);
    border: 1px solid var(--tert-con);
    border-radius: 5px;
    background: var(--bg2);
    max-height: 100vh;

    display: grid;
    grid-template-columns: 2fr 1fr 1fr 1fr 3rem;
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
      max-height: 100%;
      transition: all 0.2s;
      background: var(--bg2);

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

    &--disabled {
      opacity: 0.75;
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
