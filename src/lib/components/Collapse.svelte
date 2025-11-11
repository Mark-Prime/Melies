<script>
  import { run } from "svelte/legacy";

  import { createEventDispatcher } from "svelte";
  const dispatch = createEventDispatcher();

  /** @type {{defaultOpen?: boolean, title: any, children?: import('svelte').Snippet}} */
  let { defaultOpen = false, title, children } = $props();

  let enabled = $state(defaultOpen);

  run(() => {
    if (enabled) {
      dispatch("open");
    }

    if (!enabled) {
      dispatch("close");
    }
  });
</script>

<div class="toggle">
  {#if enabled}
    <button onclick={() => (enabled = false)} class="cancel-btn hide-toggle">
      -
    </button>
  {:else}
    <button onclick={() => (enabled = true)} class="hide-toggle"> + </button>
  {/if}
  <h2 class="centered">{title}</h2>
</div>
{#if enabled}
  {@render children?.()}
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
