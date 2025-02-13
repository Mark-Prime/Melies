<script>
  /** @type {{resp: any}} */
  let { resp } = $props();
</script>

<div class="event-viewer">
  <p>Error {resp.code}</p>
  <p>{resp.err_text}</p>
</div>

<style lang="scss">
  .event-viewer {
    background-color: var(--bg2);
    width: 100%;
    height: 100%;
    max-height: calc(100vh - 50px - 4rem);
    border: 1px solid var(--err-con);
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
      background: var(--err);
      border-radius: 0 8px 8px 0;
      overflow: hidden;
    }

    /* Handle */
    &::-webkit-scrollbar-thumb {
      background: var(--err-con);
      border-radius: 0 8px 8px 0;
    }
  }
</style>
