<script>
  import {
    faMinus,
    faPlus,
    faCircleExclamation,
  } from "@fortawesome/free-solid-svg-icons";
  import Fa from "svelte-fa";
  import { invoke } from "@tauri-apps/api/core";

  export let demo;
  export let settings;

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

    err() {
      this.color = "err";
    }
  }

  let open = true;

  function toggle() {
    open = !open;
  }

  function organizeEvents() {
    let events = [];

    for (let index = 0; index < demo.length; index++) {
      const element = demo[index];

      let event = new Event(element, false);

      if (event.isKillstreak) {
        events.push(event);
        continue;
      }

      if (event.value.includes("clip_start")) {
        event.start = element.tick;

        if (!(index + 1 < demo.length)) {
          event.err();
          events.push(event);
          continue;
        }

        if (!demo[index + 1].value.Bookmark?.includes("clip_end")) {
          event.err();
          events.push(event);
          continue;
        }

        event.end = demo[index + 1].tick;

        event.value = event.value.replace("clip_start ", "");
        index++;
      }

      if (event.value.includes("clip_end")) {
        event.err();
      }

      event.value = event.value.replace("spec", "Spectate player");

      events.push(event);
    }

    return events;
  }

  function tickToTime(ticks) {
    return `${Math.floor(Math.round(ticks / 66) / 60)}m ${
      Math.round(ticks / 66) % 60
    }s`;
  }
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
      {#each organizeEvents() as event}
        <div class={`event event--${event.color}`}>
          {#if event.color === "err"}
            <Fa icon={faCircleExclamation} color={`var(--err)`} /> {event.event}
          {:else}
            {#if !event.value.includes("mls_rec_demo")}
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
