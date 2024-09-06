<script>
  // @ts-nocheck
  export let event;
  export let demos;
  export let demo;
  export let demo_i;
  export let i;
  export let refresh;

  function deleteEvent(demo_i, i) {
    demos[demo_i].splice(i, 1);

    if (demos[demo_i].length === 0) {
      demos.splice(demo_i, 1);
    }

    demos = demos;
    refresh();
  }

  function editDemoName(demo_i, new_name) {
    for (let event of demos[demo_i]) {
      event.demo_name = new_name;
    }

    demos = demos;
    refresh();
  }

  function addEvent(demo_i) {
    demos[demo_i].push({
      value: {
        Bookmark: "General",
      },
      tick: 0,
      demo_name: demos[demo_i][0].demo_name,
      event: `[_] Bookmark _ (\"_\" at 0)`,
      isKillstreak: false,
    });

    demos = demos;
    refresh();
  }

  function deleteDemo(demo_i) {
    demos.splice(demo_i, 1);

    demos = demos;
    refresh();
  }

  function toggleKillstreak(demo_i, i) {
    let event = demos[demo_i][i];

    if (event.isKillstreak) {
      event.isKillstreak = false;

      if (!event.value.Bookmark) {
        event.value.Bookmark = "General";
      }

      demos = demos;
      refresh();
      return;
    }

    event.isKillstreak = true;

    if (!event.value.Killstreak) {
      event.value.Killstreak = 3;
    }

    demos = demos;
    refresh();
  }
</script>

<div>
  {#if !i}
    <div class="demo demo__header">
      <input
        class="demo__header-input"
        data-tooltip="Edit Demo Name"
        value={event.demo_name}
        on:change={(e) => editDemoName(demo_i, e.target.value)}
      />
      <a class="demo-delete" href="/" on:click={() => deleteDemo(demo_i)}>
        Remove
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
      <a
        class="demo__event-delete tooltip tooltip--left"
        data-tooltip="Delete Event"
        style={"--kills: 0"}
        href="/"
        on:click={() => deleteEvent(demo_i, i)}
      >
        -
      </a>
    </div>
  </div>
  {#if i === demo.length - 1}
    <div class="demo demo__new-event">
      <a on:click={() => addEvent(demo_i)} href="/"> Add new event to demo </a>
    </div>
    <div class="demo demo__bottom">
      {demo.length} event{#if demo.length > 1}s{/if}
    </div>
  {/if}
</div>

<style lang="scss">
  .demo {
    width: 100%;
    text-align: center;
    color: var(--pri-con-text);
    border: var(--pri-con) 1px solid;

    border-radius: 0.7rem 0.7rem 3px 3px;

    overflow-x: hidden;

    transition: all 0.2s;

    background-color: var(--bg2);

    &-delete {
      white-space: nowrap;
      color: var(--err);
      cursor: pointer;
      transition: all 0.2s;
    }

    &__header {
      display: grid;
      grid-template-columns: 1fr min-content;
      gap: 1rem;
      transition: all 0.2s;
      padding: 0 1rem;

      &:hover {
        gap: 1rem;

        .demo-delete {
          opacity: 1;
        }
      }

      &-input {
        padding: 0 0.5rem;
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
        transition: all 0.2s;

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
      background-color: var(--bg2);
      border-radius: 3px;
      margin: 1px 0;
      padding: 1px 0.5rem;
      font-size: 1rem;
      display: grid;
      gap: 1rem;

      grid-template-columns: min-content 1fr 0.4fr 26px;

      transition: all 0.2s;
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
        border: 1px solid var(--err);
        border-radius: 5px;
        width: 26px;
        cursor: pointer;
        transition: all 0.2s;
        text-align: center;
      }

      &:hover {
        border-color: var(--tert);
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
          left: -0.5rem;
          display: none;
          background-color: var(--bg);
          color: var(--bg-text);
          border: var(--outline) 1px solid;
          padding: 0.2rem 0.5rem;
          border-radius: 0.5rem;
          white-space: nowrap;
        }

        &::after {
          content: "";
          display: none;
          position: absolute;
          bottom: 1.4rem;
          left: 0.5rem;
          height: 0.5rem;
          width: 0.8rem;
          background-color: var(--outline);
          clip-path: polygon(100% 0, 0 0, 50% 100%);
        }

        &:hover,
        &:active,
        &:focus {
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
        padding: 1px 0.5rem;

        .demo__event-link,
        .demo__event-input {
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
      border-radius: 3px 3px 0.7rem 0.7rem;
      margin-bottom: 1rem;
      text-align: left;
      padding: 0 0.5rem;
    }

    &__new-event {
      margin-bottom: 1px;
      padding: 0 0.5rem;
      border-radius: 3px;

      height: 0.5rem;

      color: transparent;

      transition: all 0.2s;

      overflow: hidden;

      cursor: pointer;
    }

    &__new-event:hover {
      margin-bottom: 1px;
      padding: 0 0.5rem;
      border-radius: 3px;

      height: 1.8rem;

      color: var(--pri-con-text);
    }
  }
</style>
