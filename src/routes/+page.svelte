<script>
  import EventViewer from "$lib/event_viewer/EventViewer.svelte";
  import Hero from "$lib/hero.svelte";
  import Backups from "$lib/backups.svelte";
  import Logstf from "$lib/logstf.svelte";
  import Demos from "$lib/demos_parser/demos.svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import ErrViewer from "$lib/ErrViewer.svelte";
  import LogViewer from "$lib/LogViewer.svelte";
  import EditEvents from "$lib/EditEvents.svelte";
  import Settings from "$lib/settings.svelte";

  let resp = { vdms: 0, clips: 0, events: 0, code: 0 };
  let reload = false;
  let forceReload = (full) => {
    reload = !reload;
    if (full) {
      resp = { vdms: 0, clips: 0, events: 0, code: 0 };
    }
  };

  async function runMelies() {
    resp = await invoke("ryukbot");
    forceReload();
  }
</script>

<link
  href="https://fonts.googleapis.com/css2?family=Briem+Hand:wght@100..900&display=swap"
  rel="stylesheet"
/>
<div class="homepage">
  <div class="homepage__main">
    <Hero />
    <div class="homepage__container">
      <div class="homepage__left">
        {#key reload}
          {#if !resp.code}
            <EventViewer />
          {:else if resp.code === 200}
            <LogViewer {resp} />
          {:else}
            <ErrViewer {resp} />
          {/if}
        {/key}
      </div>
      <div class="homepage__right">
        <div class="homepage__tools">
          <EditEvents on:reload={() => forceReload(true)} />
          <Demos on:reload={() => forceReload(true)} />
          <Logstf on:reload={() => forceReload(true)} />
          <Backups on:reload={() => forceReload(true)} />
        </div>

        <div class="homepage__run">
          <button class="btn" on:click={runMelies}> Run </button>
          <Settings on:close={() => forceReload(true)} />
        </div>
      </div>
    </div>
  </div>
</div>

<style lang="scss">
  .homepage {
    width: 100vw;
    height: 100vh;

    background: linear-gradient(0, var(--bg2) 0%, var(--bg) 60%);
    z-index: 1;

    &:before {
      content: "";
      position: absolute;
      top: 0;
      left: 0;
      width: 100%;
      height: 100%;

      background: url("/hero-dark.png") no-repeat;
      background-position: bottom right;
      background-size: contain;

      opacity: 0.1;

      z-index: -1;
    }

    &__main {
      max-width: 1200px;
      margin: auto;
      padding: 1rem;

      width: 100vw;
      height: 100vh;

      display: flex;
      flex-direction: column;
    }

    &__container {
      margin: 1rem auto;

      width: 100%;

      flex-grow: 1;

      display: flex;
      flex-wrap: nowrap;
      gap: 1rem;
    }

    &__left {
      flex-grow: 1;
      display: flex;
      flex-direction: column;
    }

    &__right {
      flex-shrink: 0;

      display: flex;
      flex-direction: column;

      justify-content: space-between;
    }

    &__tools {
      display: flex;
      gap: 0.5rem;
      flex-direction: column;
    }

    &__run {
      display: flex;
      gap: 0.5rem;
      justify-content: space-between;
      width: 100%;

      & .btn {
        flex-grow: 1;
      }
    }
  }
</style>
