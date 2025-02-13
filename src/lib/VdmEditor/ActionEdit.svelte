<script>
  import { run } from 'svelte/legacy';

  import ColorSelect from "$lib/components/ColorSelect.svelte";
  import Input from "$lib/components/Input.svelte";
  import Range from "$lib/components/Range.svelte";
  import Select from "$lib/components/Select.svelte";
  import Switch from "$lib/components/Switch.svelte";


  let startTimeType = $state("tick");
  let skipToTimeType = $state("tick");
  let prevStartTimeType = $state("tick");
  let prevSkipToTimeType = $state("tick");

  function deleteAction() {
    dispatch("delete");
  }

  import { createEventDispatcher } from "svelte";
  /** @type {{action: any}} */
  let { action = $bindable() } = $props();
  const dispatch = createEventDispatcher();

  function rgbToHex(r, g, b) {
    return "#" + ((1 << 24) | (r << 16) | (g << 8) | b).toString(16).slice(1);
  }

  function hexToRgb(hex) {
    var result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
    return result
      ? {
          r: parseInt(result[1], 16),
          g: parseInt(result[2], 16),
          b: parseInt(result[3], 16),
        }
      : null;
  }

  let color1 = $state("#ffffff");
  let color2 = $state("#ffffff");



  function calcTickTypes() {
    if (action?.start_tick !== null) {
      startTimeType = "tick";
      prevSkipToTimeType = "tick";
    } else if (action?.start_time !== null) {
      startTimeType = "time";
      prevSkipToTimeType = "time";
    } else {
      startTimeType = "none";
      prevSkipToTimeType = "none";
    }

    if (action?.skip_to_tick !== null) {
      skipToTimeType = "tick";
      prevSkipToTimeType = "tick";
    } else if (action?.skip_to_time !== null) {
      skipToTimeType = "time";
      prevSkipToTimeType = "time";
    } else {
      skipToTimeType = "none";
      prevSkipToTimeType = "none";
    }
  }

  function onColorChange() {
    const rgb1 = hexToRgb(color1);
    const rgb2 = hexToRgb(color2);
    action.rgba1 = [rgb1.r, rgb1.g, rgb1.b, action.rgba1[3]];
    action.rgba2 = [rgb2.r, rgb2.g, rgb2.b, action.rgba2[3]];
    dispatch("change", action);
  }


  function onStartTimeChange() {
    switch (startTimeType) {
      case "tick":
        action.start_tick = 0;
        action.start_time = null;
        break;
      case "time":
        action.start_tick = null;
        action.start_time = 0;
        break;
      case "none":
        action.start_tick = null;
        action.start_time = null;
        break;
    }
  }

  function onSkipToTimeChange() {
    switch (skipToTimeType) {
      case "tick":
        action.skip_to_tick = 0;
        action.skip_to_time = null;
        break;
      case "time":
        action.skip_to_tick = null;
        action.skip_to_time = 0;
        break;
      case "none":
        action.skip_to_tick = null;
        action.skip_to_time = null;
        break;
    }
  }
  run(() => {
    switch (action?.factory) {
      case "ScreenFadeStart":
        color1 =
          rgbToHex(action.rgba1[0], action.rgba1[1], action.rgba1[2]) ||
          "#ffffff";
        break;
      case "TextMessageStart":
        color1 =
          rgbToHex(action.rgba1[0], action.rgba1[1], action.rgba1[2]) ||
          "#ffffff";
        color2 =
          rgbToHex(action.rgba2[0], action.rgba2[1], action.rgba2[2]) ||
          "#ffffff";
        break;
      default:
        break;
    }
  });
  run(() => {
    if (action) {
      calcTickTypes();
    }
  });
  run(() => {
    if (prevStartTimeType !== startTimeType && action) {
      onStartTimeChange();
      prevStartTimeType = startTimeType;

      dispatch("change", action);
    }

    if (prevSkipToTimeType !== skipToTimeType && action) {
      onSkipToTimeChange();
      prevSkipToTimeType = skipToTimeType;

      dispatch("change", action);
    }
  });
</script>

<div class="action">
  {#if action}
    <h2>{action.factory}</h2>
    <Input
      title="Name"
      bind:value={action.name}
      on:change={() => dispatch("change", action)}
    />
    <div class="switches">
      {#if startTimeType === "tick"}
        <Input
          title="Start Time"
          type="number"
          bind:value={action.start_tick}
          on:change={() => dispatch("change", action)}
        />
      {:else if startTimeType === "time"}
        <Input
          title="Start Time"
          type="number"
          bind:value={action.start_time}
          on:change={() => dispatch("change", action)}
        />
      {:else}
        <Input
          title="Start Time"
          value="0"
          on:change={() => dispatch("change", action)}
          disabled
        />
      {/if}
      <Select title="Unit" bind:value={startTimeType}>
        <option value="none">None</option>
        <option value="tick">Tick</option>
        <option value="time">Seconds</option>
      </Select>
    </div>
    {#if action.factory == "SkipAhead"}
      <div class="switches">
        {#if skipToTimeType === "tick"}
          <Input
            title="Skip To"
            type="number"
            bind:value={action.skip_to_tick}
            on:change={() => dispatch("change", action)}
          />
        {:else if skipToTimeType === "time"}
          <Input
            title="Skip To"
            type="number"
            bind:value={action.skip_to_time}
            on:change={() => dispatch("change", action)}
          />
        {:else}
          <Input
            title="Skip To"
            value="0"
            on:change={() => dispatch("change", action)}
            disabled
          />
        {/if}
        <Select title="Unit" bind:value={skipToTimeType}>
          <option value="none">None</option>
          <option value="tick">Tick</option>
          <option value="time">Seconds</option>
        </Select>
      </div>
    {/if}
    {#if action.factory == "PlayCommands"}
      <Input
        title="Commands"
        bind:value={action.commands}
        on:change={() => dispatch("change", action)}
      />
    {/if}
    {#if action.factory == "ScreenFadeStart"}
      <div class="switches">
        <Switch
          title="Fade In"
          bind:value={action.fade_in_enabled}
          on:change={() => dispatch("change", action)}
        />
        <Switch
          title="Fade Out"
          bind:value={action.fade_out_enabled}
          on:change={() => dispatch("change", action)}
        />
        <Switch
          title="Modulate"
          bind:value={action.modulate_enabled}
          on:change={() => dispatch("change", action)}
        />
        <Switch
          title="Purge"
          bind:value={action.purge_enabled}
          on:change={() => dispatch("change", action)}
        />
        <Switch
          title="Stay Out"
          bind:value={action.stay_out_enabled}
          on:change={() => dispatch("change", action)}
        />
        <ColorSelect bind:value={color1} on:change={onColorChange} />
      </div>
      <Range
        title="Alpha"
        bind:value={action.rgba1[3]}
        on:change={() => dispatch("change", action)}
      />
    {/if}
    {#if action.factory == "TextMessageStart"}
      <Input
        title="Message"
        bind:value={action.message}
        on:change={() => dispatch("change", action)}
      />
      <div class="switches">
        <Input
          title="Font"
          bind:value={action.font}
          on:change={() => dispatch("change", action)}
          tooltip={"Name of the font to use\nMust be defined in your HUD."}
        />
        <Select
          title="Effect"
          bind:value={action.effect}
          on:change={() => dispatch("change", action)}
        >
          <option value="FadeInOut">FadeInOut</option>
          <option value="Flicker">Flicker</option>
          <option value="WriteOut">WriteOut</option>
        </Select>
        <Input
          title="Fade In"
          bind:value={action.fade_in}
          on:change={() => dispatch("change", action)}
          tooltip="In seconds"
        />
        <Input
          title="Fade Out"
          bind:value={action.fade_out}
          on:change={() => dispatch("change", action)}
          tooltip="In seconds"
        />
        <Input
          title="Hold Time"
          bind:value={action.hold_time}
          on:change={() => dispatch("change", action)}
          tooltip="In seconds"
        />
        <Input
          title="Fx Time"
          bind:value={action.fx_time}
          on:change={() => dispatch("change", action)}
          tooltip="In seconds"
        />
        <ColorSelect
          title="Color1"
          bind:value={color1}
          on:change={onColorChange}
        />
        <Range
          title="Color1 Alpha"
          bind:value={action.rgba1[3]}
          on:change={() => dispatch("change", action)}
        />
        <ColorSelect
          title="Color2"
          bind:value={color2}
          on:change={onColorChange}
        />
        <Range
          title="Color2 Alpha"
          bind:value={action.rgba2[3]}
          on:change={() => dispatch("change", action)}
        />
        <Input
          title="X Pos"
          bind:value={action.xy[0]}
          on:change={() => dispatch("change", action)}
          tooltip="Distance from the top of the screen"
          type="number"
        />
        <Input
          title="Y Pos"
          bind:value={action.xy[1]}
          on:change={() => dispatch("change", action)}
          tooltip="Distance from the left of the screen"
          type="number"
        />
      </div>
    {/if}
    {#if action.factory == "PlayCDTrackStart"}
      <Input
        title="Track"
        bind:value={action.track}
        on:change={() => dispatch("change", action)}
      />
    {/if}
    {#if action.factory == "PlaySoundStart"}
      <Input
        title="Sound"
        bind:value={action.sound}
        on:change={() => dispatch("change", action)}
      />
    {/if}
    {#if action.factory == "Pause" || action.factory == "ChangePlaybackRate"}
      <Input
        title="Stop Tick"
        bind:value={action.stop_tick}
        on:change={() => dispatch("change", action)}
      />
    {/if}
    {#if action.factory == "Pause"}
      <Input
        title="Duration"
        bind:value={action.duration}
        on:change={() => dispatch("change", action)}
        tooltip="In seconds"
      />
    {/if}
    {#if action.factory == "ChangePlaybackRate"}
      <Input
        title="Playback Rate"
        bind:value={action.playback_rate}
        on:change={() => dispatch("change", action)}
      />
    {/if}
    {#if action.factory == "ZoomFov"}
      <div class="switches">
        <Input
          title="Final FOV"
          bind:value={action.final_fov}
          on:change={() => dispatch("change", action)}
        />
        <Input
          title="Hold FOV"
          bind:value={action.fov_hold}
          on:change={() => dispatch("change", action)}
        />
        <Input
          title="FOV Rate In"
          bind:value={action.fov_fade_in}
          on:change={() => dispatch("change", action)}
        />
        <Input
          title="FOV Rate Out"
          bind:value={action.fov_fade_out}
          on:change={() => dispatch("change", action)}
        />
        <Switch
          title="Spline"
          bind:value={action.spline}
          on:change={() => dispatch("change", action)}
        />
        <Switch
          title="Stay Out"
          bind:value={action.stayout}
          on:change={() => dispatch("change", action)}
        />
      </div>
    {/if}
    <button class="cancel-btn" onclick={deleteAction}>Delete</button>
  {:else}
    <p>Nothing Selected</p>
  {/if}
</div>

<style lang="scss">
  h2 {
    margin: 0.5rem;
  }

  .cancel-btn {
    margin-top: 0.8rem;
  }

  .action {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;

    width: 100%;

    border: 1px solid var(--sec-con);
    border-radius: 5px;
    padding: 1rem;

    max-height: min(calc(100vh - 11rem), calc(800px - 9rem));

    overflow: auto;

    /* width */
    &::-webkit-scrollbar {
      width: 8px;
    }

    /* Track */
    &::-webkit-scrollbar-track {
      background: var(--sec);
      border-radius: 8px;
      overflow: hidden;
    }

    /* Handle */
    &::-webkit-scrollbar-thumb {
      background: var(--sec-con);
      border-radius: 8px;
    }
  }

  .switches {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 0.5rem;
  }
</style>
