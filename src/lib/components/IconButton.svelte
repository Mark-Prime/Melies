<script>
  import Fa from "svelte-fa";
  /** @type {{icon: any, label: any, color: any}} */
  let { icon, label, color } = $props();
</script>

<button class="card" style="{`--color: var(${color}-con);`}">
    <span><Fa {icon} size="4x" color="{`var(${color})`}" /></span>
    <div class="label" style="{`--color: var(${color});`}">{label}</div>
</button>

<style lang="scss">
  .card {
    height: calc(40vh - 2rem);
    max-height: 300px;
    // width: calc(40vh - 2rem);
    // max-width: 300px;
    width: 100%;
    background-color: var(--bg2);
    border-radius: 0px;
    border: 1px solid var(--color);
    padding: 2rem;
    position: relative;
    z-index: 1000;
    overflow-y: hidden;
    overflow-x: hidden;

    font-size: 3vmin;
  }

  .label {
      color: var(--color);
  }
</style>
