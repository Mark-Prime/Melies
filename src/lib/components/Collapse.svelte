<script>
  import { createEventDispatcher } from "svelte";
  const dispatch = createEventDispatcher();

  export let defaultOpen = false;
  export let title;

  let enabled = defaultOpen;

  $: {
    if (enabled) {
      dispatch("open");
    }

    if (!enabled) {
      dispatch("close");
    }
  }
</script>

<div class="toggle">
  {#if enabled}
    <button
      on:click={() => (enabled = false)}
      class="cancel-btn hide-toggle"
    >
      -
    </button>
  {:else}
    <button on:click={() => (enabled = true)} class="hide-toggle">
      +
    </button>
  {/if}
  <h2 class="centered">{title}</h2>
</div>
{#if enabled}
  <slot />
{/if}

<style>
.toggle {
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 1rem;
}

button {
  width: 2.5rem;
}

h2 {
  white-space: nowrap;
}
</style>