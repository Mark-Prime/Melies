<script>
  // @ts-nocheck
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";

  export let enabled;
  export let toggle;

  let resp = {};

  async function loadBackups() {
    try {
      resp = await invoke("load_backups");
    } catch (error) {
      alert(error);
    }
  }

  async function reloadBackup(backup) {
    await invoke("reload_backup", { fileName: backup.file_name });
    toggle(true);
  }

  onMount(async () => {
    loadBackups();
  });

  $: {
    console.log("Modal Enabled:", enabled);
    loadBackups();
  }
</script>

{#if enabled}
  <div class="modal">
    <a class="modal__background" on:click={() => toggle(false)} href="/"> </a>
    <div class="modal__card">
      {#if resp.length > 0}
        <h1>Backups</h1>
        <h4 class="centered">
          Loading will completely override your current events.
        </h4>
        {#each resp.sort( (a, b) => b.file_name.localeCompare(a.file_name) ) as backup}
          <div class="demo">
            <p>{backup.file_name}</p>
            <div class="add_demo">
              <button on:click={reloadBackup(backup)}>Load</button>
            </div>
          </div>
        {/each}
      {:else}
        <h1>LOADING BACKUPS</h1>
      {/if}
    </div>
  </div>
{/if}

<style lang="scss">
  .demo {
    font-size: small;
    padding: 0.3rem 0.5rem;
    margin: 2px 0;
    font-family: "Source Code Pro", monospace;
    color: var(--sec-con-text);
    border: 1px solid var(--sec-con);
    border-radius: 5px;

    display: grid;
    grid-template-columns: 1fr 1fr;
    white-space: nowrap;

    transition: all 0.2s;

    & > div,
    & > p {
      display: flex;
      align-items: center;
      white-space: nowrap;
      padding: 0;
      margin: 0;
    }

    &:hover {
      border: 1px solid var(--sec-con-text);
      max-height: 100vh;
    }
  }

  .add_demo {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    gap: 1px;

    & > button {
      font-size: 12px;
      padding: 0.3rem 0.7rem;
      margin: 0;
      // height: 100%;
      border-radius: 5px;
      width: fit-content;
    }
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
      border: 1px solid var(--sec-con);
      padding: 1rem;
      position: relative;
      z-index: 1000;
      overflow-y: auto;
      overflow-x: hidden;
      margin: 1rem;

      /* width */
      &::-webkit-scrollbar {
        width: 12px;
      }

      /* Track */
      &::-webkit-scrollbar-track {
        background: var(--sec);
        border-radius: 0 8px 8px 0;
        overflow: hidden;
      }

      /* Handle */
      &::-webkit-scrollbar-thumb {
        background: var(--sec-con);
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
