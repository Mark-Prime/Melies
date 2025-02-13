<script>
  import { createEventDispatcher } from "svelte";

  /** @type {{value: any, title?: string}} */
  let { value = $bindable(), title = "Color" } = $props();

  const dispatch = createEventDispatcher();
</script>

<div>
  <input type="color" id="color" onchange={e => dispatch("change", e.target.value)} bind:value={value}/>
  <label for="color">{title}</label>
</div>

<style lang="scss">
  div {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  input {
    width: 35px;
    padding: 0;
    border: 0;
  }
</style>