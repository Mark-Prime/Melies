<script>
  export let resp;
</script>

<div class="event-viewer">
  <p>
    {`Created ${resp.vdms} VDMs containing ${resp.clips} events.`}
  </p>
  {#if resp.output_path != ""}
    <p>
      Backup saved to: {resp.backup_location}
    </p>
  {/if}
</div>

<style lang="scss">
  .event-viewer {
    background-color: var(--bg2);
    width: 100%;
    height: 100%;
    max-height: calc(100vh - 50px - 4rem);
    border: 1px solid var(--sec-con);
    padding: 1rem;
    border-radius: 8px;
    text-align: left;

    overflow-y: auto;

    /* width */
    &::-webkit-scrollbar {
      width: 8px;
    }

    /* Track */
    &::-webkit-scrollbar-track {
      background: var(--pri);
      border-radius: 0 8px 8px 0;
      overflow: hidden;
    }

    /* Handle */
    &::-webkit-scrollbar-thumb {
      background: var(--pri-con);
      border-radius: 0 8px 8px 0;
    }
  }
</style>
