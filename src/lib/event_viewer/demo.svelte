<script>
  import {
    faMinus,
    faPlus,
    faCircleExclamation,
  } from "@fortawesome/free-solid-svg-icons";
  import Fa from "svelte-fa";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  export let demo;
  export let settings;

  let demoEvents = [];

  async function loadSettings() {
    settings = await invoke("load_settings");
  }

  class Event {
    constructor(event) {
      this.color = "pri";
      this.isKillstreak = event.isKillstreak;
      this.tick = event.tick;
      this.event = event.event;

      if (!settings.recording) {
        loadSettings().then(() => {
          this.setBounds(event);
        });

        return;
      }

      this.setBounds(event);
    }

    setBounds(event) {
      if (this.isKillstreak) {
        this.color = "tert";
        (this.value = event.value.Killstreak),
          (this.start =
            event.tick -
            settings.recording.before_killstreak_per_kill *
              event.value.Killstreak);

        this.end = event.tick + settings.recording.after_killstreak;

        return;
      }

      this.color = "sec";
      this.value = event.value.Bookmark;
      this.start = event.tick - settings.recording.before_bookmark;
      this.end = event.tick + settings.recording.after_bookmark;
    }

    err(msg) {
      this.color = "err";
      this.err = msg;
    }
  }

  let open = true;

  function toggle() {
    open = !open;
  }

  function organizeEvents() {
    let events = [];

    console.log(demo);

    for (let index = 0; index < demo.length; index++) {
      const element = demo[index];

      let event = new Event(element, false);

      console.log(event);

      if (event.isKillstreak) {
        events.push(event);
        continue;
      }

      if (event.value?.includes("clip_start")) {
        event.start = element.tick;

        if (!(index + 1 < demo.length)) {
          event.err("???? Idk how you did this");
          events.push(event);
          continue;
        }

        if (!demo[index + 1].value.Bookmark?.includes("clip_end")) {
          event.err("No clip end found");
          events.push(event);
          continue;
        }

        event.end = demo[index + 1].tick;

        event.value = event.value.replace("clip_start ", "");
        index++;
      }

      if (!event.value) {
        continue;
      }

      if (event.value.includes("clip_end")) {
        event.err("No clip start found");
      }

      event.value = event.value.replace("spec", "Spectate player");

      events.push(event);
    }

    console.log(events)

    return events;
  }

  function tickToTime(ticks) {
    return `${Math.floor(Math.round(ticks / 66) / 60)}m ${
      Math.round(ticks / 66) % 60
    }s`;
  }

  onMount(() => {
    demoEvents = organizeEvents();
  })
</script>

<div class="demo-display">
  <button class="demo" on:click={toggle}>
    {#if open}
      <Fa icon={faMinus} color={`var(--err)`} />
    {:else}
      <Fa icon={faPlus} color={`var(--pri)`} />
    {/if}
    {demo[0]?.demo_name}
  </button>

  {#if open}
    <div class="events">
      {#each demoEvents as event}
        <div class={`event event--${event.color}`}>
          {#if event.color === "err"}
            <span class="tooltip" data-tooltip={event.err} style={`--kills: 0;`}>
              <Fa icon={faCircleExclamation} color={`var(--err)`} /> {event.event}
            </span>
          {:else}
            {#if typeof event.value !== "string" || !event.value?.includes("mls_rec_demo")}
              {event.isKillstreak ? `${event.value}ks` : `Bookmark "${event.value}"`}
              from
              <span
                class="tick tooltip"
                data-tooltip={`${tickToTime(event.start)}`}
                style={`--kills: 0;`}>{event.start}</span
              >
              to
              <span
                class="tick tooltip"
                data-tooltip={`${tickToTime(event.end)}`}
                style={`--kills: 0;`}>{event.end}</span
              >
            {:else}
              Record Entire Demo
            {/if}
          {/if}
        </div>
      {/each}
    </div>
  {/if}
</div>

<style lang="scss">
  .demo-display {
    padding: 0 1rem;

    &:nth-child(even) {
      background-color: var(--bg);
    }

    &:first-child {
      padding-top: 0.5rem;
    }

    &:last-child {
      padding-bottom: 0.5rem;
    }
  }

  .demo {
    all: unset;

    text-align: left;

    display: flex;
    align-items: center;
    gap: 0.5rem;

    color: var(--pri-con-text);
  }

  .events {
    text-align: left;
    padding-left: 2rem;
  }

  .event {
    &--pri {
      color: var(--pri-con-text);

      .tick {
        color: var(--pri);
      }
    }

    &--sec {
      color: var(--sec-con-text);

      .tick {
        color: var(--sec);
      }
    }

    &--tert {
      color: var(--tert-con-text);

      .tick {
        color: var(--tert);
      }
    }

    &--err {
      color: var(--err);
      margin-left: -1.3rem;
    }
  }
</style>
