<script>
  import { invoke } from "@tauri-apps/api/tauri";
  import { createEventDispatcher } from "svelte";
  import { faFilePen } from "@fortawesome/free-solid-svg-icons";

  import { onMount } from "svelte";
  import DemoEdit from "$lib/home/demoEdit.svelte";
  import Fa from "svelte-fa";

  const dispatch = createEventDispatcher();

  let enabled = false;
  let demos = [];

  function toggle() {
    enabled = !enabled;
    
    if (enabled) {
      loadEvents();
    }
  }

  function cancel() {
    enabled = false;
    demos = [];
  }

  async function saveEvents() {
    for (let demo of demos) {
      demo.sort((a, b) => a.tick - b.tick);
    }

    await invoke("save_events", { newEvents: demos });
    dispatch("reload");
    toggle();
  }

  function refresh() {
    demos = demos;
  }

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
    }
  }

  async function loadEvents() {
    let event_list = await invoke("load_events");

    setEvents(event_list);
  }

  onMount(async () => {
    loadEvents();
  });

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
</script>

<button class="btn" on:click={toggle}>
  <Fa icon={faFilePen} color={`var(--pri)`} />
  Edit Events
</button>

{#if enabled}
  <div class="modal">
    <a class="modal__background" on:click={() => cancel()} href="/"> </a>
    <div class="modal__card">
      {#each demos as demo, demo_i}
        {#each demo as event, i (`${demo_i}__${i}`)}
          <DemoEdit {demo_i} {i} {demo} {demos} {event} {refresh} />
        {/each}
      {/each}
      <div class="new-demo">
        <a href="/" class="new-demo__1" on:click={addDemo}>
          Add Event Manually
        </a>
      </div>
      <div class="new-demo">
        <a href="/" class="new-demo__2" on:click={cancel}> Cancel </a>
        <a href="/" class="new-demo__3" on:click={saveEvents}> Save Events </a>
      </div>
    </div>
  </div>
{/if}

<style lang="scss">
  .new-demo {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 0.5rem;

    &:last-of-type {
      margin-top: 0.5rem;
    }

    & > a {
      width: 100%;
      padding: 1px 0.5rem;
      border-radius: 3px;

      transition: all 0.2s;
    }

    &__1 {
      color: var(--sec-con-text);
      border: var(--sec-con) 1px solid;
      grid-column: span 2;

      &:hover {
        color: var(--sec);
        border-color: var(--sec);
      }
    }

    &__2 {
      color: var(--err-con-text);
      border: var(--err-con) 1px solid;

      &:not(p):hover {
        color: var(--err);
        border-color: var(--err);
      }
    }

    &__3 {
      color: var(--pri-con-text);
      border: var(--pri-con) 1px solid;

      &:not(p):hover {
        color: var(--pri);
        border-color: var(--pri);
      }
    }
  }

  .btn {
    display: flex;
    gap: 0.5rem;
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
      border: 1px solid var(--pri-con);
      padding: 1rem;
      position: relative;
      z-index: 1000;
      overflow-y: auto;
      overflow-x: hidden;
      margin: 1rem;

      /* width */
      &::-webkit-scrollbar {
        width: 8px;
      }

      /* Track */
      &::-webkit-scrollbar-track {
        background: var(--pri);
        border-radius: 0 8px 8px 0;
        overflow: hidden;
      }

      /* Handle */
      &::-webkit-scrollbar-thumb {
        background: var(--pri-con);
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
</style>
