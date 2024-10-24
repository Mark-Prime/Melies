<script>
  // @ts-nocheck
  import Input from "$lib/components/Input.svelte";
  import Switch from "$lib/components/Switch.svelte";
  import Collapse from "$lib/components/Collapse.svelte";
  import Addon from "./Addon.svelte";
  import { invoke } from "@tauri-apps/api/core";
  export let addons;

</script>

<div class="addons">
  <Collapse defaultOpen={false} title="Addons">
    {#if Object.keys(addons).length === 0}
      <div class="setting">
        <div class="settings__span">No Addons installed</div>
      </div>
    {/if}

    {#each Object.keys(addons) as addon}
      <Addon bind:addon={addon} bind:addons={addons} />
    {/each}
    <div>
      <button on:click={() => window.open("https://github.com/Mark-Prime/awesome-Melies-addons")}>
        Get Addons
      </button>
      <button on:click={() => invoke("open_addons_folder")}>
        Open Addons Folder
      </button>
    </div>
  </Collapse>
</div>

<style lang="scss">
  .addons {
    margin-bottom: 1rem;
  }
</style>