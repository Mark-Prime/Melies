<script>
  import EventViewer from "$lib/event_viewer/EventViewer.svelte";
  import Hero from "$lib/home/hero.svelte";
  import Backups from "$lib/backups.svelte";
  import Logstf from "$lib/LogsTf.svelte";
  import Demos from "$lib/demos_parser/demos.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import ErrViewer from "$lib/home/ErrViewer.svelte";
  import LogViewer from "$lib/LogViewer.svelte";
  import EditEvents from "$lib/EditEvents.svelte";
  import Settings from "$lib/Settings.svelte";
  import VdmEditor from "$lib/VdmEditor/VdmEditor.svelte";
  import DemoManager from "$lib/DemoManager.svelte";
  import SortFootage from "$lib/SortFootage.svelte";
  import { onMount } from "svelte";
  import Tf2 from "$lib/TF2.svelte";

  let resp = { vdms: 0, clips: 0, events: 0, code: 0 };
  let theme = { has_theme: false };
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

  onMount(async () => {
    theme = await invoke("load_theme");
  })
</script>

<link
  href="https://fonts.googleapis.com/css2?family=Briem+Hand:wght@100..900&display=swap"
  rel="stylesheet"
/>
<div class="homepage" style="{theme.has_theme ? 
` --red: ${theme.red || '#f35355'};
  --blu: ${theme.blu || '#65b1e2'};

  --bg: ${theme.bg || '#1a1c1e'};
  --bg2: ${theme.bg2 || '#131516'};
  --bg-text: ${theme.bg_text || '#e3e2e6'};

  --pri: ${theme.pri || '#a7c8ff'};
  --pri-text: ${theme.pri_text || '#003061'};
  --pri-con: ${theme.pri_con || '#004689'};
  --pri-con-text: ${theme.pri_con_text || '#d5e3ff'};

  --sec: ${theme.sec || '#d3bbff'};
  --sec-text: ${theme.sec_text || '#3c1d70'};
  --sec-con: ${theme.sec_con || '#533688'};
  --sec-con-text: ${theme.sec_con_text || '#ebddff'};

  --tert: ${theme.tert || '#ffb0cd'};
  --tert-text: ${theme.tert_text || '#5d1137'};
  --tert-con: ${theme.tert_con || '#7a294e'};
  --tert-con-text: ${theme.tert_con_text || '#ffd9e4'};

  --err: ${theme.err || '#ffb4ab'};
  --err-text: ${theme.err_text || '#690005'};
  --err-con: ${theme.err_con || '#93000a'};
  --err-con-text: ${theme.err_con_text || '#ffdad6'};
` : ''}">
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
          <br />Advanced Tools
          <VdmEditor on:reload={() => forceReload(true)} />
          <DemoManager on:reload={() => forceReload(true)} />
          <!-- <SortFootage on:reload={() => forceReload(true)} /> -->
        </div>

        <div class="homepage__bottom">
          {#key reload}
            <Tf2 on:reload={() => forceReload(true)} />
          {/key}
          <div class="homepage__run">
            <button class="btn" on:click={runMelies}> Run </button>
            <Settings on:close={() => forceReload(true)} />
          </div>
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

    &__bottom {
      display: flex;
      flex-direction: column;
      gap: 0.5rem;
      justify-content: space-between;
      width: 100%;
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
