<script>
  export let value = false;
  export let tooltip = "";
  export let style = "";
  export let tooltipDirection = "right";

  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher();

  const click = () => dispatch("click", !value);
</script>

<div
  class:tooltip={tooltip}
  class:tooltip--left={tooltipDirection === "left"}
  data-tooltip={tooltip}
  style={style}
>
  {#if value}
    <button
      class="cancel-btn"
      on:click={click}
    >
      -
    </button>
  {:else}
    <button
      on:click={click}
    >
      +
    </button>
  {/if}
</div>

<style lang="scss">
  button {
    font-size: 12px;
    padding: 0.3rem 0.7rem;
    margin: 0;
    border-radius: 5px;
    width: fit-content;
  }
</style>