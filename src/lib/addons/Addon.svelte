<script>
  // @ts-nocheck
  import Input from "$lib/components/Input.svelte";
  import Switch from "$lib/components/Switch.svelte";
  import Setting from "./Setting.svelte";
  import Group from "./Group.svelte";
  import addonTypeSort from "$lib/composables/addonTypeSort";
  /** @type {{addon: any, addons: any}} */
  let { addon, addons = $bindable() } = $props();
</script>

<div class="bordered bordered--pri addon">
  <h3>{addon}</h3>
  <div class="setting">
    {#each addonTypeSort(addons[addon]) as addonSetting}
      {#if addons[addon][addonSetting].type === "group"}
        <Group
          bind:group={addons[addon][addonSetting]}
          defaultTitle={addonSetting}
        />
      {:else}
        <Setting
          bind:setting={addons[addon][addonSetting]}
          defaultTitle={addonSetting}
        />
      {/if}
    {/each}
  </div>
</div>

<style lang="scss">
  .bordered {
    padding: 1rem;

    & > .setting {
      margin-bottom: 0;
    }
  }

  .addon {
    margin-bottom: 1rem;
  }

  h3 {
    margin-top: 0;
  }
</style>
