<script>
  // @ts-nocheck

  /** @type {{i: any, event: any}} */
  let { i, event } = $props();
</script>

{#if !i}
  <div class="carrot">></div>
{/if}
<div class="event" class:bookmark={!event.isKillstreak}>{event.event}</div>

<style lang="scss">
  .carrot {
    color: var(--pri);
  }

  .event {
    color: var(--tert);
  }

  .bookmark {
    color: var(--sec);
  }
</style>
