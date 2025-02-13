<script>
  import AutoComplete from "simple-svelte-autocomplete"
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
    items = []
  } = $props();

  const dispatch = createEventDispatcher();

  const change = () => dispatch("change", value);
</script>

{#if display}
  <div>
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
    <div class={`autocomplete--${color}`}>
      <AutoComplete 
        {items} 
        bind:selectedItem={value} 
        onChange={change}
      />
    </div>
  </div>
{/if}

<style lang="scss">
  label {
    text-align: left;
  }

  div {
    width: 100%;
    display: flex;
    flex-direction: column;
  }
</style>
