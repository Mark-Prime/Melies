<script>
  import { createEventDispatcher } from "svelte";

  /** @type {{value: any, title?: any, key?: any, color?: string, display?: boolean, tooltip?: string, type?: string, disabled?: boolean}} */
  let {
    value = $bindable(),
    title = null,
    key = title?.toLowerCase().replace(/ /g, "_") || "input",
    color = "pri",
    display = true,
    tooltip = "",
    type = "text",
    disabled = false
  } = $props();

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
          ? `--kills: ${tooltip.split(/\n||\n/).length - 1};`
          : ""}
      >
        {title}
      </label>
    {/if}
    {#if type === "number"}
      <input
        bind:value
        onchange={change}
        oninput={change}
        type="number"
        id={key}
        class={`input input--${color}`}
        disabled={disabled}
      />
    {:else}
      <input
        bind:value
        onchange={change}
        oninput={change}
        id={key}
        class={`input input--${color}`}
        disabled={disabled}
      />
    {/if}
  </div>
{/if}
