<script>
  // @ts-nocheck
  import Input from "$lib/components/Input.svelte";
  import Switch from "$lib/components/Switch.svelte";
  import Collapse from "$lib/components/Collapse.svelte";
  import Addon from "./Addon.svelte";
  import { invoke } from "@tauri-apps/api/core";
  /** @type {{addons: any}} */
  let { addons = $bindable() } = $props();

  let keys = $derived(Object.keys(addons));
</script>

<div class="addons">
  {#if Object.keys(addons).length === 0}
    <div class="setting">
      <div class="settings__span">No Addons installed</div>
    </div>
  {/if}

  {#each keys as addon, i}
    <Addon bind:addon={keys[i]} bind:addons />
  {/each}
  <div>
    <button
      onclick={() =>
        window.open("https://github.com/Mark-Prime/awesome-Melies-addons")}
    >
      Get Addons
    </button>
    <button onclick={() => invoke("open_addons_folder")}>
      Open Addons Folder
    </button>
  </div>
</div>

<style lang="scss">
  .addons {
    margin-bottom: 1rem;
  }
</style>
