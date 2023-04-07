<script>
  import { invoke } from "@tauri-apps/api/tauri"

  /**
   * @type {any[]}
   */
  let demos = [];

  let resp = { vdms: 0, clips: 0, events: 0, code: 0 };

  async function loadEvents(){
    let event_list = await invoke("load_events");
    
    if (event_list.code === 200) {
      event_list.events.forEach((/** @type {{ demo_name: any; }} */ event, /** @type {number} */ i) => {
        if (i === 0 || event_list.events[i - 1].demo_name != event.demo_name) {
          demos.push([event]);
          return;
        }

        demos[demos.length - 1].push(event);
      });

      console.log("DEMOS", demos);

      demos = demos;

      return;
    }
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
    {#if !resp.code}
      {#each demos as demo}
        {#each demo as event, i (event.event)}
          {#if !i}
            <div class="carrot">></div>
          {/if}
          <div class="event" class:bookmark={event.value.Bookmark}>{event.event}</div>
        {/each}
      {/each}
    {:else}
      {`Created ${resp.vdms} VDMs containing ${resp.clips} containing ${resp.events} events.`}
    {/if}
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

    border-left: var(--pri-con) 1px solid;
    white-space: pre-line;

    text-align: left;

    padding: 1rem;
    overflow-y: auto;

    border-radius: .7rem;
  }

  .carrot {
    color: var(--pri);
  }

  .event {
    color: var(--tert);
  }

  .bookmark {
    color: var(--sec)
  }
</style>