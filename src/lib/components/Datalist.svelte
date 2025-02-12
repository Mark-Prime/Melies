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
    <input
      list={key + '_list'} 
      id={key} 
      name={key} 
      class={`settings__input input--${color}`}
      bind:value
      on:change={change}
      on:input={change}
    />
    <datalist id={key + '_list'}>
      <slot></slot>
    </datalist>
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
