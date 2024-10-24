<script>
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  import Demo from "./demo.svelte";

  let settings = {};
  let demos = [];
  let resp = "";
  let loading = true;

  async function loadSettings() {
    settings = await invoke("load_settings");
  }

  function setEvents(eventList = {}) {
    demos = [];

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

      demos = demos;
    }

    loading = false;
  }

  async function loadEvents() {
    loading = true;
    let eventList = await invoke("load_events");

    setEvents(eventList);
  }

  onMount(async () => {
    loadSettings();
    loadEvents();
  });
</script>

<div class="event-viewer">
  {#if loading}
    <h1>Loading...</h1>
  {/if}
  {#each demos as demo}
    <Demo {demo} {settings} />
  {/each}
</div>

<style lang="scss">
  .event-viewer {
    background-color: var(--bg2);
    width: 100%;
    height: 100%;
    max-height: calc(100vh - 50px - 4rem);
    border: 1px solid var(--pri-con);
    padding: 0;
    border-radius: 8px;

    overflow-y: auto;

    
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
</style>
