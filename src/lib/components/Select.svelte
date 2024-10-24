<script>
  export let value;
  export let title = null;
  export let key = title.toLowerCase().replace(/ /g, "_") || "select";
  export let color = "pri";
  export let display = true;
  export let tooltip = "";

  import { createEventDispatcher } from "svelte";

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
    <select
      bind:value
      id="method"
      class={`settings__input input--${color}`}
      on:change={change}
      on:input={change}
    >
      <slot></slot>
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
