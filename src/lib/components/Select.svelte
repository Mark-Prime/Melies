<script>

  import { createEventDispatcher } from "svelte";
  /** @type {{value: any, title?: any, key?: any, color?: string, display?: boolean, tooltip?: string, children?: import('svelte').Snippet}} */
  let {
    value = $bindable(),
    title = null,
    key = title.toLowerCase().replace(/ /g, "_") || "select",
    color = "pri",
    display = true,
    tooltip = "",
    children,
    style
  } = $props();

  const dispatch = createEventDispatcher();

  const change = () => dispatch("change", value);
</script>

{#if display}
  <div style={style}>
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
    <select
      bind:value
      id={key}
      class={`settings__input input--${color}`}
      onchange={change}
      oninput={change}
    >
      {@render children?.()}
    </select>
  </div>
{/if}

<style lang="scss">
  select {
    width: 100%;
    padding-top: calc(.5rem - 2px);
  }

  label {
    text-align: left;
  }

  div {
    width: 100%;
    display: flex;
    flex-direction: column;
  }
</style>
