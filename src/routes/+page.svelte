<script>
  // @ts-nocheck
  import { invoke } from "@tauri-apps/api/tauri"
  import Logstf from "../lib/logstf.svelte";
  import Demo from "../lib/demo.svelte";

  let demos = [];
  
  let editting = false;
  let logstfModal = false;
  let demoModal = false;

  let resp = { vdms: 0, clips: 0, events: 0, code: 0 };

  function setEvents(event_list = []) {
    demos = []

    if (event_list.code === 200) {
      event_list.events.forEach((/** @type {{ demo_name: any; }} */ event, /** @type {number} */ i) => {
        event.isKillstreak = false;
        
        if (event.value.Killstreak) {
          event.isKillstreak = true;
        }

        if (i === 0 || event_list.events[i - 1].demo_name != event.demo_name) {
          demos.push([event]);
          return;
        }

        demos[demos.length - 1].push(event);
      });

      // console.log("DEMOS", demos);

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
          Bookmark: "General"
        },
        tick: 0,
        demo_name: "new_demo",
        event: `[_] Bookmark _ (\"_\" at 0)`,
        isKillstreak: false
      }
    ])

    demos = demos;
  }

  function parseLogEvents(demo_name, events) {
    let new_demo = []
    
    for (let event of events) {
      new_demo.push(
        {
          value: {
            Bookmark: `spec ${event.steamid64}`
          },
          tick: event.time * 66,
          demo_name: demo_name,
          event: `[logs.tf_${event.label}] spec ${event.steamid64} (\"${demo_name}\" at ${event.time * 66})`,
          isKillstreak: false
        }
      )
    }

    demos.push(new_demo);

    demos = demos;
  }

  function parseDemoEvents(demo_name, events) {
    let new_demo = []
    
    for (let event of events) {
      if (event.start) {
        new_demo.push(
          {
            value: {
              Bookmark: `clip_start spec ${event.steamid64}`
            },
            tick: event.time,
            demo_name: demo_name,
            event: `[demo_${event.label}] clip_start spec ${event.steamid64} (\"${demo_name}\" at ${event.time})`,
            isKillstreak: false
          }
        )

        continue;
      }

      if (event.bookmark) {
        new_demo.push(
          {
            value: {
              Bookmark: `spec ${event.steamid64}`
            },
            tick: event.time,
            demo_name: demo_name,
            event: `[demo_${event.label}] spec ${event.steamid64} (\"${demo_name}\" at ${event.time})`,
            isKillstreak: false
          }
        )

        continue;
      }

      if (event.killstreak) {
        new_demo.push(
          {
            value: {
              Killstreak: event.kills
            },
            tick: event.time,
            demo_name: demo_name,
            event: `[demo_${event.label}] Killstreak ${event.kills} (\"${demo_name}\" at ${event.time})`,
            isKillstreak: true
          }
        )

        continue;
      }

      new_demo.push(
        {
          value: {
            Bookmark: `clip_end`
          },
          tick: event.time,
          demo_name: demo_name,
          event: `[demo_${event.label}] clip_end (\"${demo_name}\" at ${event.time})`,
          isKillstreak: false
        }
      )
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
  }

  async function disableEditing() {
    for (let demo of demos) {
      demo.sort((a, b) =>  a.tick - b.tick)
    }

    let new_demos = await invoke("save_events", { newEvents: demos });

    setEvents(new_demos);

    editting = false;
  }

  function deleteEvent(demo_i, i) {
    demos[demo_i].splice(i, 1);

    if (demos[demo_i].length === 0) {
      demos.splice(demo_i, 1)
    }

    demos = demos;
  }

  function editDemoName(demo_i, new_name) {
    for (let event of demos[demo_i]) {
      event.demo_name = new_name;
    }

    demos = demos;
    // console.log(demos)
  }

  function addEvent(demo_i) {
    demos[demo_i].push({
      value: {
        Bookmark: "General"
      },
      tick: 0,
      demo_name: demos[demo_i][0].demo_name,
      event: `[_] Bookmark _ (\"_\" at 0)`,
      isKillstreak: false
    });

    demos = demos;
  }

  function deleteDemo(demo_i) {
    demos.splice(demo_i, 1);

    demos = demos;
  }

  function toggleKillstreak(demo_i, i) {
    let event = demos[demo_i][i];

    if (event.isKillstreak) {
      event.isKillstreak = false;

      if (!event.value.Bookmark) {
        event.value.Bookmark = 'General';
      } 

      demos = demos;
      return;
    }

    event.isKillstreak = true;

    if (!event.value.Killstreak) {
      event.value.Killstreak = 3;
    } 
    
    demos = demos;
  }

  function toggleLogModal() {
    logstfModal = !logstfModal;
  }

  function toggleDemoModal() {
    demoModal = !demoModal;
  }

  loadEvents();
</script>

<div class="home-page">
  <h1 class="header">Méliès</h1>
  <div class="events">
    <Logstf enabled={logstfModal} toggle={toggleLogModal} parseLogEvents={parseLogEvents}/>
    <Demo enabled={demoModal} toggle={toggleDemoModal} parseDemoEvents={parseDemoEvents}/>
    {#if !resp.code}
      {#each demos as demo, demo_i}
        {#each demo as event, i (`${demo_i}__${i}`)}
          {#if editting}
            {#if !i}
              <div class="demo demo__header">
                <input 
                    class="demo__header-input"
                    data-tooltip="Edit Demo Name"
                    value={event.demo_name}
                    on:change={(e) => editDemoName(demo_i, e.target.value)}
                  />
                <a 
                  class="demo-delete" 
                  href="/" 
                  on:click={() => deleteDemo(demo_i)}
                >
                  Delete
                </a>
              </div>
            {/if}
            <div class="demo__event-container">
              <div class="demo__event" class:demo__event--bookmark={!event.isKillstreak}>
                {#if !event.isKillstreak}
                  <a 
                    class="demo__event-link"
                    data-tooltip="Change to Killstreak" 
                    on:click={() => toggleKillstreak(demo_i, i)}
                    href="/"
                  >
                    Bookmark
                  </a>
                  <input 
                    class="demo__event-input"
                    data-tooltip="Edit Bookmark label"
                    bind:value={event.value.Bookmark}
                  />
                {:else}
                  <a
                    class="demo__event-link"
                    data-tooltip="Change to Bookmark"
                    on:click={() => toggleKillstreak(demo_i, i)}
                    href="/"
                  >
                    Killstreak
                  </a>
                  <input 
                    class="demo__event-input"
                    data-tooltip="Edit Killstreak count"
                    bind:value={event.value.Killstreak}
                    type="number"
                  />
                {/if}
                <input 
                    class="demo__event-input"
                    data-tooltip="Edit tick"
                    bind:value={event.tick}
                    type="number"
                  />
                <a class="demo__event-delete" href="/" on:click={() => deleteEvent(demo_i, i)}>
                  Delete
                </a>
              </div>
            </div>
            {#if i === demo.length - 1}
              <div class="demo demo__new-event">
                <a on:click={() => addEvent(demo_i)} href="/">
                  Add new event to demo
                </a>
              </div>
              <div class="demo demo__bottom">{demo.length} event{#if demo.length > 1}s{/if}</div>
            {/if}
          {:else}
            {#if !i}
              <div class="carrot">></div>
            {/if}
            <div class="event" class:bookmark={!event.isKillstreak}>{event.event}</div>
          {/if}
        {/each}
      {/each}
      {#if editting}
        <div class="new-demo">
          <a href="/" class="new-demo__1" on:click={addDemo}>Add Manual Events</a>
          <a href="/" class="new-demo__2" on:click={toggleLogModal}>Logs.tf</a>
          <a href="/" class="new-demo__3" on:click={toggleDemoModal}>Scan Demo</a>
        </div>
      {/if}
    {:else if resp.code === 200}
      <p>
        {`Created ${resp.vdms} VDMs containing ${resp.clips} containing ${resp.events} events.`}
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
    <button on:click={disableEditing}>Save Events</button>
  {:else}
    <button class="input--tert" on:click={loadEvents}>Refresh</button>
    <button class="input--sec" on:click={enableEditing}>Edit Events</button>
    <button on:click={runMelies}>Run</button>

    <a href="/settings">Settings</a>
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
      padding: 1px .5rem;
      border-radius: 3px;

      transition: all .2s;
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
    font-family: 'Source Code Pro', monospace;
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

  .demo {
    width: 100%;
    text-align: center;
    color: var(--pri-con-text);
    border: var(--pri-con) 1px solid;

    border-radius: .7rem .7rem 3px 3px;

    overflow: hidden;

    transition: all .2s;

    &-delete {
      opacity: 0;
      color: var(--err);
      cursor: pointer;
      transition: all .2s;
    }

    &__header {
      display: grid;
      grid-template-columns: 1fr 0px;
      gap: 0rem;
      transition: all .2s;

      &:hover {
        grid-template-columns: 1fr 5rem;
        gap: 1rem;

        .demo-delete {
          opacity: 1;
        }
      }

      &-input {
        padding: 0 .5rem;
        border-width: 1px;
        border-style: solid;
        box-shadow: none;
        border-top: 0;
        border-left: 0;
        border-right: 0;
        border-radius: 0;
        border-color: transparent;
        margin-left: 1rem;
        width: 100%;
        text-align: center;
        transition: all .2s;

        &:hover,
        &:active,
        &:focus {
          border-color: var(--pri);
        }
      }
    }

    &__event {
      color: var(--tert-con-text);
      border: var(--tert-con) 1px solid;
      border-radius: 3px;
      margin: 1px 0;
      padding: 1px .5rem;
      font-size: 1rem;
      display: grid;
      gap: 2px;

      grid-template-columns: 1fr 1fr 1fr 0px;

      transition: all .2s;
      z-index: 1;

      &-input {
        padding: inherit;
        border-width: 1px;
        border-style: solid;
        box-shadow: none;
        border-top: 0;
        border-left: 0;
        border-right: 0;
        border-radius: 0;
        border-color: transparent;

        &:hover,
        &:active,
        &:focus {
          border-color: var(--tert);
        }
      }

      &-container {
        position: relative;
      }

      &-delete {
        color: var(--err);
        cursor: pointer;
        opacity: 0;
        transition: all .2s;

        overflow: hidden;
      }

      &:hover {
        grid-template-columns: 1fr 1fr 1fr 5rem;
        border-color: var(--tert);

        .demo__event-delete {
          color: var(--err);
          opacity: 1;
        }
      }

      .demo__event-link,
      .demo__event-input,
      .demo__event-input {
        cursor: pointer;
        position: relative;
        text-align: left;
        width: 100%;
        color: var(--tert-con-text);

        &::before {
          content: attr(data-tooltip);
          position: absolute;
          bottom: 1.9rem;
          left: -.5rem;
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
          bottom: 1.4rem;
          left: .5rem;
          height: .5rem;
          width: .8rem;
          background-color: var(--outline);
          clip-path: polygon(100% 0, 0 0, 50% 100%);
        }

        &:hover, &:active, &:focus {
          color: var(--tert);
          border-color: var(--tert);

          &::before {
            display: block;
          }

          &::after {
            display: block;
          }
        }
      }

      &--bookmark {
        color: var(--sec-con-text);
        border: var(--sec-con) 1px solid;
        border-radius: 3px;
        margin: 1px 0;
        padding: 1px .5rem;

        .demo__event-link, .demo__event-input {
          color: var(--sec-con-text);
        }

        &:hover {
          border-color: var(--sec);
        }

        .demo__event-link:hover,
        .demo__event-input:hover,
        .demo__event-input:active {
          color: var(--sec);
        }
      }
    }

    &__bottom {
      border-radius: 3px 3px .7rem .7rem;
      margin-bottom: 1rem;
      text-align: left;
      padding: 0 .5rem;
    }

    &__new-event {
      margin-bottom: 1px;
      padding: 0 .5rem;
      border-radius: 3px;

      height: .5rem;

      color: transparent;

      transition: all .2s;

      overflow: hidden;

      cursor: pointer;
    }

    &__new-event:hover {
      margin-bottom: 1px;
      padding: 0 .5rem;
      border-radius: 3px;

      height: 1.8rem;

      color: var(--pri-con-text);
    }
  }
</style>