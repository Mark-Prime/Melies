<script>
  import { invoke } from "@tauri-apps/api/core";
  import { createEventDispatcher } from "svelte";
  import { faFilePen } from "@fortawesome/free-solid-svg-icons";
  import DemoEdit from "$lib/home/demoEdit.svelte";
  import Modal from "$lib/components/Modal.svelte";
  import Fa from "svelte-fa";

  const dispatch = createEventDispatcher();

  let enabled = false;
  let demos = [];
  let toggle = () => (enabled = !enabled);

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

<Modal color="pri" {toggle} {enabled} on:open={loadEvents}>
  {#each demos as demo, demo_i}
    {#each demo as event, i (`${demo_i}__${i}`)}
      <DemoEdit {demo_i} {i} {demo} {demos} {event} {refresh} />
    {/each}
  {/each}
  <div slot="footer">
    <div class="new-demo">
      <a href="/" class="new-demo__1" on:click={addDemo}> Add Event Manually </a>
    </div>
    <div class="new-demo">
      <a href="/" class="new-demo__2" on:click={cancel}> Cancel </a>
      <a href="/" class="new-demo__3" on:click={saveEvents}> Save Events </a>
    </div>
  </div>
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
