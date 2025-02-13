<script>

  import { createEventDispatcher } from "svelte";
  /** @type {{action: any, index: any, selected: any}} */
  let { action = $bindable(), index, selected } = $props();
  const dispatch = createEventDispatcher();

  function update(value) {
    action.factory = value;

    dispatch("update", action);
  }
</script>

<button class="action" class:selected={selected} onclick={() => dispatch("select")}>
  <p>{index + 1}: {action.name}</p>

  <p class="tick">{action.start_tick || action.start_time || ""}</p>

  <select value={action.factory} class="select" onchange={(e) => update(e.target.value)}>
    <option value="SkipAhead">SkipAhead</option>
    <option value="StopPlayback">StopPlayback</option>
    <option value="PlayCommands">PlayCommands</option>
    <option value="ScreenFadeStart">ScreenFadeStart</option>
    <option value="TextMessageStart">TextMessageStart</option>
    <option value="PlayCDTrackStart">PlayCDTrackStart</option>
    <option value="PlaySoundStart">PlaySoundStart</option>
    <option value="Pause">Pause</option>
    <option value="ChangePlaybackRate">ChangePlaybackRate</option>
    <option value="ZoomFov">ZoomFov</option>
  </select>
</button>

<style lang="scss">
  .action {
    color: var(--pri);
    text-align: left;
    width: 100%;
    padding: .2rem .3rem ;

    display: grid;
    grid-template-columns: 1.5fr 1fr 1fr;
    gap: 1rem;
    justify-content: space-between;

    border: 1px solid var(--pri-con);
    border-radius: 5px;

    & > p {
      margin: 0;
      padding: 0;
    }
  }

  .selected {
    border: 1px solid var(--tert-con);
  }

  .select {
    padding: 0;
    margin: 0;

    border-radius: 3px;

    border: 1px solid var(--sec-con);
  }
</style>
