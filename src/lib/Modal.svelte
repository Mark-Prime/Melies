<script>
  import { createEventDispatcher } from "svelte";
  const dispatch = createEventDispatcher();

  export let color = "pri";
  export let enabled;
  export let toggle;
  export let large = false;
  export let tall = false;
  export let small = false;
  export let grow = false;

  $: {
    if (enabled) {
      dispatch("open");
    }

    if (!enabled) {
      dispatch("close");
    }
  }
</script>

{#if enabled}
  <div class="modal">
    <a class="modal__background" on:click={() => toggle()} href="/"> </a>
    <div
      class="modal__card"
      class:modal__card--large={large}
      class:modal__card--small={small}
      class:modal__card--tall={tall}
      class:modal__card--grow={grow}
      style={`--color: var(--${color}); --color-con: var(--${color}-con);`}
    >
      <slot />
    </div>
  </div>
{/if}

<style lang="scss">
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
      border: 1px solid var(--color-con);
      padding: 1rem;
      position: relative;
      z-index: 1000;
      overflow-y: auto;
      overflow-x: hidden;
      margin: 1rem;

      &--large {
        max-width: min(calc(100vw - 2rem), 1680px);
        max-height: min(calc(100vh - 2rem), 900px);
      }
      
      &--grow {
        max-width: min(calc(100vw - 2rem), 1680px);
        max-height: min(calc(100vh - 2rem), 900px);

        width: fit-content;
      }

      &--small {
        max-width: min(calc(100vw - 2rem), 500px);
        max-height: min(calc(100vh - 2rem), 500px);
      }

      &--tall {
        height: 100%;
      }

      /* width */
      &::-webkit-scrollbar {
        width: 8px;
      }

      /* Track */
      &::-webkit-scrollbar-track {
        background: var(--color);
        border-radius: 0 8px 8px 0;
        overflow: hidden;
      }

      /* Handle */
      &::-webkit-scrollbar-thumb {
        background: var(--color-con);
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
