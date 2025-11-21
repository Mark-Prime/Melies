<script>
  import { invoke } from "@tauri-apps/api/core";
  import { createEventDispatcher } from "svelte";
  import { faFilePen } from "@fortawesome/free-solid-svg-icons";
  import DemoEdit from "$lib/editEvents/demoEdit.svelte";
  import Modal from "$lib/components/Modal.svelte";
  import Fa from "svelte-fa";

  let settings = $state({});
  let recordingSettings = $state({});

  const dispatch = createEventDispatcher();

  let enabled = $state(false);
  let demos = $state([]);
  let toggle = () => {
    enabled = !enabled
  
    if (enabled) {
      loadSettings();
    }
  };

  async function loadSettings() {
    settings = await invoke("load_settings");
    recordingSettings = settings.recording;
  }

  function cancel() {
    enabled = false;
    demos = [];
  }

  async function saveEvents() {
    for (let demo of demos) {
      demo.sort((a, b) => a.tick - b.tick);
    }

    console.log({ newEvents: $state.snapshot(demos) });

    await invoke("save_events", { newEvents: demos });
    dispatch("reload");
    toggle();
  }

  function refresh() {
    demos = demos;
  }

  function setEvents(eventList = {}) {
    demos = [];

    if (eventList.code === 200) {
      eventList.events.forEach(
        (event, i) => {
          event.isKillstreak = false;

          if (event.value.Killstreak) {
            event.isKillstreak = true;
            event.isClip = false;
          }

          if (event.value.Bookmark) {
            if (
              event.value.Bookmark.includes("clip_end") ||
              event.value.Bookmark.includes("clip_start")
            ) {
              event.isClip = true;
            }
          }

          if (i === 0 || eventList.events[i - 1].demo_name != event.demo_name) {
            demos.push([event]);
            return;
          }

          demos[demos.length - 1].push(event);
        },
      );

      demos = demos;
    }

    console.log($state.snapshot(demos))
  }

  async function loadEvents() {
    let eventList = await invoke("load_events");

    setEvents(eventList);
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
        notes: ""
      },
    ]);

    demos = demos;
  }
</script>

<button class="btn" onclick={toggle}>
  <Fa icon={faFilePen} color={`var(--pri)`} />
  Edit Events
</button>

<Modal color="pri" {toggle} {enabled} on:open={loadEvents} wide min_width="900px">
  {#each demos as demo, demoIndex}
    {#each demo as event, i (`${demoIndex}__${i}`)}
      <DemoEdit {demoIndex} {i} {demo} {demos} bind:event={demos[demoIndex][i]} {refresh} {recordingSettings} />
    {/each}
  {/each}
  {#snippet footer()}
    <div>
      <div class="new-demo">
        <a href="/" class="new-demo__1" onclick={addDemo}>
          Add Event Manually
        </a>
      </div>
      <div class="new-demo">
        <a href="/" class="new-demo__2" onclick={cancel}> Cancel </a>
        <a href="/" class="new-demo__3" onclick={saveEvents}> Save Events </a>
      </div>
    </div>
  {/snippet}
</Modal>

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
</style>
