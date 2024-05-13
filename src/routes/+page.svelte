<script>
  // @ts-nocheck
  import { invoke } from "@tauri-apps/api/tauri";
  import Logstf from "../lib/logstf.svelte";
  import Demo from "../lib/demos_parser/demos.svelte";
  import Backups from "../lib/backups.svelte";
  import DemoEdit from "$lib/home/demoEdit.svelte";
  import DemoDisplay from "$lib/home/demoDisplay.svelte";
  import { onMount } from "svelte";

  let recording_settings = {};

  async function loadSettings() {
    let settings = await invoke("load_settings");
    recording_settings = settings.recording;
    console.log(settings);
  }

  onMount(async () => {
    loadSettings();
  });

  let demos = [];

  let editting = false;
  let modified = false;
  let logstfModal = false;
  let demoModal = false;
  let backupModal = false;

  let resp = { vdms: 0, clips: 0, events: 0, code: 0 };

  function setEvents(event_list = []) {
    demos = [];

    if (event_list.code === 200) {
      event_list.events.forEach(
        (/** @type {{ demo_name: any; }} */ event, /** @type {number} */ i) => {
          event.isKillstreak = false;

          if (event.value.Killstreak) {
            event.isKillstreak = true;
          }

          if (
            i === 0 ||
            event_list.events[i - 1].demo_name != event.demo_name
          ) {
            demos.push([event]);
            return;
          }

          demos[demos.length - 1].push(event);
        }
      );

      demos = demos;

      return;
    }
  }

  async function loadEvents() {
    let event_list = await invoke("load_events");

    setEvents(event_list);
  }

  async function runMelies() {
    resp = await invoke("ryukbot");

    setEvents();
  }

  function addDemo() {
    demos.push([
      {
        value: {
          Bookmark: "General",
        },
        tick: 0,
        demo_name: "new_demo",
        event: `[_] Bookmark _ (\"_\" at 0)`,
        isKillstreak: false,
      },
    ]);

    demos = demos;
  }

  function parseLogEvents(demo_name, events) {
    let new_demo = [];

    let spec_mode = recording_settings["third_person"] ? "spec_third" : "spec";

    for (let event of events) {
      new_demo.push({
        value: {
          Bookmark: `${spec_mode} ${event.steamid64}`,
        },
        tick: event.time * 66,
        demo_name: demo_name,
        event: `[logs.tf_${event.label}] ${spec_mode}  ${
          event.steamid64
        } (\"${demo_name}\" at ${event.time * 66})`,
        isKillstreak: false,
      });
    }

    demos.push(new_demo);

    demos = demos;
  }

  function parseDemoEvents(demo_name, events) {
    let new_demo = [];

    let spec_mode = recording_settings["third_person"] ? "spec_third" : "spec";

    for (let event of events) {
      if (event.start) {
        new_demo.push({
          value: {
            Bookmark: `clip_start ${spec_mode} ${event.steamid64}`,
          },
          tick: event.time,
          demo_name: demo_name,
          event: `[demo_${event.label}] clip_start ${spec_mode} ${event.steamid64} (\"${demo_name}\" at ${event.time})`,
          isKillstreak: false,
        });

        continue;
      }

      if (event.bookmark) {
        new_demo.push({
          value: {
            Bookmark: `${spec_mode} ${event.steamid64}`,
          },
          tick: event.time,
          demo_name: demo_name,
          event: `[demo_${event.label}] ${spec_mode} ${event.steamid64} (\"${demo_name}\" at ${event.time})`,
          isKillstreak: false,
        });

        continue;
      }

      if (event.killstreak) {
        new_demo.push({
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

      new_demo.push({
        value: {
          Bookmark: `clip_end`,
        },
        tick: event.time,
        demo_name: demo_name,
        event: `[demo_${event.label}] clip_end (\"${demo_name}\" at ${event.time})`,
        isKillstreak: false,
      });
    }

    demos.push(new_demo);

    demos = demos;
  }

  function cancelEditing() {
    loadEvents();

    editting = false;
  }

  function enableEditing() {
    resp.code = 0;
    editting = true;
    modified = false;
  }

  async function disableEditing() {
    if (!modified) {
      editting = false;
      return;
    }

    for (let demo of demos) {
      demo.sort((a, b) => a.tick - b.tick);
    }

    let new_demos = await invoke("save_events", { newEvents: demos });

    setEvents(new_demos);

    editting = false;
  }

  function toggleLogModal() {
    logstfModal = !logstfModal;
  }

  function toggleDemoModal() {
    demoModal = !demoModal;
  }

  function toggleBackupModal(update = false) {
    backupModal = !backupModal;

    if (update) {
      loadEvents();
    }
  }

  function refresh() {
    demos = demos;
    modified = true;
  }

  function enable_modified() {
    modified = true;
  }
  loadEvents();
</script>

<div class="home-page">
  <h1 class="header">Méliès</h1>
  <div class="events">
    <Logstf
      enabled={logstfModal}
      toggle={toggleLogModal}
      {parseLogEvents}
      modified={enable_modified}
    />
    <Demo
      enabled={demoModal}
      toggle={toggleDemoModal}
      {parseDemoEvents}
      modified={enable_modified}
    />
    <Backups
      enabled={backupModal}
      toggle={toggleBackupModal}
    />
    {#if !resp.code}
      {#each demos as demo, demo_i}
        {#each demo as event, i (`${demo_i}__${i}`)}
          {#if editting}
            <DemoEdit {demo_i} {i} {demo} {demos} {event} {refresh} />
          {:else}
            <DemoDisplay {i} {event} />
          {/if}
        {/each}
      {/each}
      {#if editting}
        <div class="new-demo">
          <a href="/" class="new-demo__1" on:click={addDemo}>
            Add Manual Events
          </a>
          <a href="/" class="new-demo__2" on:click={toggleLogModal}>Logs.tf</a>
          <a href="/" class="new-demo__3" on:click={toggleDemoModal}>
            Scan Demo
          </a>
        </div>
      {/if}
    {:else if resp.code === 200}
      <p>
        {`Created ${resp.vdms} VDMs containing ${resp.clips} events.`}
      </p>
      {#if resp.output_path != ""}
        <p>
          Backup saved to: {resp.backup_location}
        </p>
      {/if}
    {:else}
      <p>Error {resp.code}</p>
      <p>{resp.err_text}</p>
    {/if}
  </div>

  {#if editting}
    <button class="cancel-btn" on:click={cancelEditing}>Cancel</button>
    <button class="input--sec" on:click={() => toggleBackupModal(false)}>Load Backup</button>
    <button on:click={disableEditing}>Save Events</button>
  {:else}
    <button class="input--tert" on:click={loadEvents}>Refresh</button>
    <button class="input--sec" on:click={enableEditing}>Edit Events</button>
    <button on:click={runMelies}>Run</button>

    <a href="/settings">Settings</a>
    <!-- <a href="/newhome">TEST</a> -->
  {/if}
</div>

<style lang="scss">
  .home-page {
    display: grid;
    grid-template-columns: repeat(5, 1fr) 8rem;
    grid-template-rows: repeat(24, 1fr);
    gap: 1rem;
    height: calc(100vh - 1.4rem);
    width: 100%;
    max-width: calc(1000px - 2rem);
    margin: auto;
  }

  .header {
    grid-column-start: 1;
    grid-column-end: 7;
  }

  .new-demo {
    display: grid;
    grid-template-columns: 1fr 1fr 1fr;
    gap: 2px;

    & > a {
      width: 100%;
      padding: 1px 0.5rem;
      border-radius: 3px;

      transition: all 0.2s;
    }

    &__1 {
      color: var(--pri-con-text);
      border: var(--pri-con) 1px solid;

      &:hover {
        color: var(--pri);
        border-color: var(--pri);
      }
    }

    &__2 {
      color: var(--sec-con-text);
      border: var(--sec-con) 1px solid;

      &:not(p):hover {
        color: var(--sec);
        border-color: var(--sec);
      }
    }

    &__3 {
      color: var(--tert-con-text);
      border: var(--tert-con) 1px solid;

      &:not(p):hover {
        color: var(--sec);
        border-color: var(--sec);
      }
    }
  }

  .events {
    font-family: "Source Code Pro", monospace;
    grid-column-start: 1;
    grid-column-end: 6;
    grid-row-start: 2;
    grid-row-end: 25;

    border-left: var(--pri-con) 1px solid;
    white-space: pre-line;

    text-align: left;

    padding: 1rem;
    overflow-y: auto;
    overflow-x: hidden;

    border-radius: 0.7rem;
  }
</style>
