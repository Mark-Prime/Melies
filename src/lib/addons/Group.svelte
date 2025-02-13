<script>
  import Group from './Group.svelte';
  // @ts-nocheck
  import Setting from "./Setting.svelte";
  import addonTypeSort from "$lib/composables/addonTypeSort";
  /** @type {{group: any, defaultTitle: any, depth?: number}} */
  let { group = $bindable(), defaultTitle, depth = 1 } = $props();

  let open = $state(true);
  let addonTypeSorted = $derived(addonTypeSort(group.settings));

  let title = group.title || defaultTitle;
</script>

{#if Object.keys(group.settings).length > 0}
  <div class={`bordered group bordered--${['pri', 'sec', 'tert'][depth % 3]}`} >
    <button onclick={() => open = !open} class:open ><h4>{open ? '-' : '+'} {title}</h4></button>
    {#if group.description}
      <p>{group.description}</p>
    {/if}
    {#if open}
      <div class="setting">
        {#each addonTypeSorted as setting, i}
          {#if group.settings[setting].type === "group"}
            <Group group={group.settings[setting]} defaultTitle={setting} depth={depth + 1}/>
          {:else}
            <Setting bind:setting={group.settings[setting]} bind:defaultTitle={addonTypeSorted[i]} depth={depth}/>
          {/if}
        {/each}
      </div>
    {/if}
  </div>
{/if}

<style lang="scss">
  button {
    all: unset;
    width: fit-content;
    display: flex;

    &:not(.open) {
      color: var(--err-con-text);
    }
  }

  .group {
    grid-column: 1 / span 2;

    & .setting {
      margin-bottom: 0;
    }
  }

  .bordered {
    padding: 1rem;
  }

  h4 {
    margin: 0;
    text-align: left;
  }

  p {
    margin: 0 0 1rem 0;
    text-align: left;
  }
</style>