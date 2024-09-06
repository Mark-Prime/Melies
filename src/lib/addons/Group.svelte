<script>
  // @ts-nocheck
  import Setting from "./Setting.svelte";
  import addonTypeSort from "../composables/addonTypeSort";
  export let group;
  export let defaultTitle;
  export let depth = 1;

  let open = true;

  let title = group.title || defaultTitle;
</script>

{#if Object.keys(group.settings).length > 0}
  <div class={`bordered group bordered--${['pri', 'sec', 'tert'][depth % 3]}`} >
    <button on:click={() => open = !open} class:open ><h4>{open ? '-' : '+'} {title}</h4></button>
    {#if group.description}
      <p>{group.description}</p>
    {/if}
    {#if open}
      <div class="setting">
        {#each addonTypeSort(group.settings) as setting}
          {#if group.settings[setting].type === "group"}
            <svelte:self group={group.settings[setting]} defaultTitle={setting} depth={depth + 1}/>
          {:else}
            <Setting bind:setting={group.settings[setting]} bind:defaultTitle={setting} depth={depth}/>
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