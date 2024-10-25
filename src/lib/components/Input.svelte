<script>
  import { createEventDispatcher } from "svelte";

  export let value;
  export let title = null;
  export let key = title?.toLowerCase().replace(/ /g, "_") || "input";
  export let color = "pri";
  export let display = true;
  export let tooltip = "";
  export let type = "text";
  export let disabled = false;

  const dispatch = createEventDispatcher();

  const change = () => dispatch("change", value);
</script>

{#if display}
  <div class="input-group">
    {#if title}
      <label
        for={key}
        class:tooltip
        data-tooltip={tooltip.trim()}
        style={tooltip
          ? `--kills: ${tooltip.split(/\r\n|\r|\n/).length - 1};`
          : ""}
      >
        {title}
      </label>
    {/if}
    {#if type === "number"}
      <input
        bind:value
        on:change={change}
        on:input={change}
        type="number"
        id={key}
        class={`input input--${color}`}
        disabled={disabled}
      />
    {:else}
      <input
        bind:value
        on:change={change}
        on:input={change}
        id={key}
        class={`input input--${color}`}
        disabled={disabled}
      />
    {/if}
  </div>
{/if}
