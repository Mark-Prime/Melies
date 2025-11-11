<script>
  import { createEventDispatcher } from "svelte";
  /** @type {{value?: boolean, tooltip?: string, style?: string, tooltipDirection?: string}} */
  let {
    value = false,
    tooltip = "",
    style = "",
    tooltipDirection = "right",
  } = $props();

  const dispatch = createEventDispatcher();

  const click = () => dispatch("click", !value);
</script>

<div
  class:tooltip
  class:tooltip--left={tooltipDirection === "left"}
  data-tooltip={tooltip}
  {style}
>
  {#if value}
    <button class="cancel-btn" onclick={click}> - </button>
  {:else}
    <button onclick={click}> + </button>
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
