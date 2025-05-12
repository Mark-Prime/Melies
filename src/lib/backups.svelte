<script>
  import { invoke } from "@tauri-apps/api/core";
  import { faFloppyDisk } from "@fortawesome/free-solid-svg-icons";
  import Fa from "svelte-fa";
  import { createEventDispatcher } from "svelte";
  import Modal from "$lib/components/Modal.svelte";
  import dayjs from "dayjs";

  const dispatch = createEventDispatcher();

  let enabled = $state(false);
  let resp = $state({});
  let toggle = () => (enabled = !enabled);

  async function loadBackups() {
    try {
      resp = await invoke("load_backups");
      resp = resp.sort( (a, b) => b.file_name.localeCompare(a.file_name) )
    } catch (error) {
      alert(error);
    }
  }

  async function reloadBackup(backup) {
    await invoke("reload_backup", { fileName: backup.file_name });
    dispatch("reload");
    toggle();
  }
</script>

<button class="btn btn--sec" onclick={toggle}>
  <Fa icon={faFloppyDisk} color={`var(--sec)`} />
  Load Backup
</button>
<Modal color="sec" {toggle} {enabled} on:open={loadBackups} min_width="600px">
  {#if resp.length > 0}
    <h1>Backups</h1>
    <h4 class="centered">
      Loading will completely override your current events.
    </h4>
    {#each resp as backup}
      <div class="demo">
        <p>{backup.file_name}</p>
        <p>{dayjs.unix(backup.created.secs_since_epoch).format('MMM DD, YYYY')}</p>
        <div class="add_demo">
          <button onclick={() => reloadBackup(backup)}>Load</button>
        </div>
      </div>
    {/each}
  {:else}
    <h1>LOADING BACKUPS</h1>
  {/if}
</Modal>

<style lang="scss">
  .btn {
    display: flex;
    gap: 0.5rem;
  }

  .demo {
    font-size: small;
    padding: 0.3rem 0.5rem;
    margin: 2px 0;
    font-family: "Source Code Pro", monospace;
    color: var(--sec-con-text);
    border: 1px solid var(--sec-con);
    border-radius: 5px;

    display: grid;
    grid-template-columns: 1fr 1fr 3rem;
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
</style>
