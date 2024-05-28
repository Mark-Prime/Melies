<script>
  import { createEventDispatcher } from "svelte";

  export let value;
  export let title = "Color";

  const dispatch = createEventDispatcher();
</script>

<div>
  <input type="color" id="color" on:change={e => dispatch("change", e.target.value)} bind:value={value}/>
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