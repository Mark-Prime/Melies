<script>
  import { createEventDispatcher } from "svelte";

  /** @type {{value: any, title?: any, key?: any, color?: string, display?: boolean, tooltip?: string}} */
  let {
    value = $bindable(),
    title = null,
    key = title.toLowerCase().replace(/ /g, "_") || "switch",
    color = "pri",
    display = true,
    tooltip = "",
    disabled = false
  } = $props();

  const dispatch = createEventDispatcher();

  const change = () => dispatch("change", value);
</script>

{#if display}
  <div class={`input__switch ${disabled ? "disabled" : ""}`} id="{key}">
    <label class="switch">
      <input type="checkbox" bind:checked={value} onchange={change} disabled={disabled}/>
      <span class={`slider round slider--${color}`}></span>
    </label>
    <div
      class="text"
      class:tooltip
      data-tooltip={tooltip?.trim() || ""}
      style={tooltip
        ? `--kills: ${tooltip.split(/\r\n|\r|\n/).length - 1};`
        : ""}
    >
      {title}
    </div>
  </div>
{/if}

<style>
  .disabled {
    opacity: 0.5;
  }

  .text {
    text-align: left;
  }
</style>