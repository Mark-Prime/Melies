<script>
  import { createEventDispatcher } from "svelte";

  export let value;
  export let title = null;
  export let key = title.toLowerCase().replace(/ /g, "_") || "switch";
  export let color = "pri";
  export let display = true;
  export let tooltip = "";

  const dispatch = createEventDispatcher();

  const change = () => dispatch("change", value);
</script>

{#if display}
  <div class="input__switch" id="{key}">
    <label class="switch">
      <input type="checkbox" bind:checked={value} on:change={change} />
      <span class={`slider round slider--${color}`}></span>
    </label>
    <div
      class:tooltip
      data-tooltip={tooltip.trim()}
      style={tooltip
        ? `--kills: ${tooltip.split(/\r\n|\r|\n/).length - 1};`
        : ""}
    >
      {title}
    </div>
  </div>
{/if}
