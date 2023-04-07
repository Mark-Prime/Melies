<script>
  import { invoke } from "@tauri-apps/api/tauri"

  /**
   * @type {any[]}
   */
  let demos = [];

  // @ts-ignore
  let editting = true;

  let resp = { vdms: 0, clips: 0, events: 0, code: 0 };

  // @ts-ignore
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

  // @ts-ignore
  async function runMelies() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    resp = await invoke("ryukbot")
  }

  // @ts-ignore
  function enableEditing() {
    editting = true;
  }

  // @ts-ignore
  function disableEditing() {
    editting = false;
  }

  // @ts-ignore
  function deleteEvent(demo_i, i) {
    demos[demo_i].splice(i, 1);

    demos = demos;
  }

  // @ts-ignore
  function addEvent(demo_i) {
    demos[demo_i].push({
      value: {
        Bookmark: "General"
      },
      tick: 0,
      demo_name: demos[demo_i][0].demo_name,
      // @ts-ignore
      event: `[${
        (new Date().toLocaleString('en-GB', { hour12: false })).replace(",", "")
      }] Bookmark General (\"${demos[demo_i][0].demo_name}\" at 0)`,
    });

    demos = demos;
  }

  // @ts-ignore
  function deleteDemo(demo_i) {
    demos.splice(demo_i, 1);

    demos = demos;
  }

  loadEvents();
</script>

<div class="home-page">
  <h1 class="header">Méliès</h1>
  <div class="events">
    {#if !resp.code}
      {#each demos as demo, demo_i}
        {#each demo as event, i (event.event)}
          {#if editting}
            {#if !i}
              <div class="demo demo__header">
                {event.demo_name}
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
              <div class="demo__event" class:demo__event--bookmark={event.value.Bookmark}>
                {#if event.value.Bookmark}
                  <div data-tooltip="Change to Killstreak">Bookmark</div>
                  <div data-tooltip="Edit Bookmark label">{event.value.Bookmark}</div>
                {:else}
                  <div data-tooltip="Change to Bookmark">Killstreak</div>
                  <div data-tooltip="Edit Killstreak count">{event.value.Killstreak}</div>
                {/if}
                <div data-tooltip="Edit tick">{event.tick}</div>
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
            <div class="event" class:bookmark={event.value.Bookmark}>{event.event}</div>
          {/if}
        {/each}
      {/each}
    {:else}
      {`Created ${resp.vdms} VDMs containing ${resp.clips} containing ${resp.events} events.`}
    {/if}
  </div>

  <a href="/settings">Settings</a>

  <button class="input--tert" disabled>Scan Demo</button>

  {#if editting}
    <button class="input--sec" on:click={disableEditing}>Save Events</button>
  {:else}
    <button class="input--sec" on:click={enableEditing}>Edit Events</button>
  {/if}

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
    width: 100%;
    max-width: calc(800px - 2rem);
    margin: auto;
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

      &:hover {
        grid-template-columns: 1fr 5rem;

        .demo-delete {
          opacity: 1;
        }
      }
    }

    &__event {
      color: var(--tert-con-text);
      border: var(--tert-con) 1px solid;
      border-radius: 3px;
      margin: 1px 0;
      padding: 0 .5rem;
      font-size: 1rem;
      display: grid;

      grid-template-columns: 1fr 1fr 1fr 0px;

      transition: all .2s;
      z-index: 1;

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

      & > div {
        cursor: pointer;
        position: relative;

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

        &:hover {
          color: var(--tert);

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
        padding: 0 .5rem;

        &:hover {
          border-color: var(--sec);
        }

        div:hover {
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