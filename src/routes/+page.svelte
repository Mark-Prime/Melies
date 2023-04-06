<script>
  import { invoke } from "@tauri-apps/api/tauri"

  let events = "";
  let resp = { vdms: 0, clips: 0, events: 0, code: 0 };

  async function loadEvents(){
    let event_list = await invoke("load_events");

    if (event_list.code === 200) {
      events = event_list.events;
      return;
    }

    events = event_list.err_text;
  }

  async function runMelies(){
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    resp = await invoke("ryukbot")
  }

  loadEvents();
</script>

<div class="home-page">
  <h1 class="header">Méliès</h1>
  <div class="events">
    { 
      resp.code === 0 ? (
        events || "_events.txt or KillStreaks.txt is empty"
      ) : (
        `Created ${resp.vdms} VDMs containing ${resp.clips} containing ${resp.events} events.`
      )
    }
  </div>

  <a href="/settings">Settings</a>

  <button class="input--tert" disabled>Scan Demo</button>
  <button class="input--sec" disabled>Edit Events</button>

  <button on:click={runMelies}>
    Run
  </button>
</div>

<style lang="scss">
  .home-page {
    display: grid;
    grid-template-columns: repeat(5, 1fr) 8rem;
    grid-template-rows: repeat(24, 1fr);
    gap: 1rem;
    height: calc(100vh - 1.4rem);
    overflow: hidden;
  }

  .header {
    grid-column-start: 1;
    grid-column-end: 7;
  }

  .events {
    grid-column-start: 1;
    grid-column-end: 6;
    grid-row-start: 2;
    grid-row-end: 25;

    border-left: var(--outline) 1px solid;
    white-space: pre-line;

    text-align: left;

    padding: 1rem;
    overflow-y: auto;

    border-radius: .7rem;
  }
</style>