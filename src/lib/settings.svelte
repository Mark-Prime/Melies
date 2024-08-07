<script>
  // @ts-ignore
  import { invoke } from "@tauri-apps/api/tauri";
  import Modal from "$lib/Modal.svelte";
  import { faGear } from "@fortawesome/free-solid-svg-icons";
  import Fa from "svelte-fa";
  import { createEventDispatcher } from "svelte";
  import Input from "./Input.svelte";
  import Switch from "./Switch.svelte";
  import Select from "./Select.svelte";
  const dispatch = createEventDispatcher();
  let settings = {};
  let output_settings = {};
  let recording_settings = {};
  let addons = {};
  let enabled = false;

  let toggle = () => (enabled = !enabled);

  async function loadSettings() {
    settings = await invoke("load_settings");
    output_settings = settings.output;
    recording_settings = settings.recording;
    addons = settings.addons;
  }

  async function saveSettings() {
    let answer = await invoke("save_settings", {
      newSettings: JSON.stringify(settings),
    });
    console.log(answer);

    toggle();
  }

  $: {
    if (enabled) {
      loadSettings();
    }

    if (!enabled) {
      dispatch("close");
    }
  }
</script>

<button class="btn btn--sec" on:click={toggle}>
  <Fa icon={faGear} color={`var(--sec)`} />
</button>
<Modal color="sec" {toggle} {enabled}>
  <h1>Settings</h1>
  <div class="setting">
    <div class="settings__input-group settings__span">
      <Input
        title="\tf Folder"
        bind:value={settings.tf_folder}
        tooltip="The full filepath to your \tf folder within the Team Fortress 2 game files."
      />
    </div>
    <div class="settings__input-group settings__span">
      <Input
        title="Output Folder"
        bind:value={output_settings.folder}
        tooltip="The full filepath to the folder you want to output to.
If left blank, the output folder will default to your sparklyfx settings."
        display={output_settings.method === "sparklyfx"}
      />
    </div>
    <Switch
      title="Prevent overwriting previously made VDMs"
      bind:value={settings.safe_mode}
      tooltip="If a VDM already exists, do not rewrite it."
    />
    <Switch
      title="Clear events after running"
      bind:value={settings.clear_events}
      tooltip="Empty the _events.txt or KillStreaks.txt files once completed."
    />
    <Switch
      title="Save backups of events"
      bind:value={settings.save_backups}
      tooltip="Save a backup of the original _events.txt or Killstreaks.txt file in the Documents folder."
    />
  </div>

  <h2>Output</h2>

  <div class="setting">
    <Select
      title="Recording Method"
      bind:value={output_settings.method}
      tooltip={`The method you want to use to record each clip:
        h264 - QuickTime encoding to .mp4 (requires you to launch the 32bit tf.exe)
        tga - generates .tga image sequence
        jpeg - generates .jpeg image sequence
        SparklyFX - Automates recording using SparklyFX
        SVR - uses Source Video Render to record .mkv
        SVR.mov - uses Source Video Render to record .mov
        SVR.mp4 - uses Source Video Render to record .mp4 (Requires SVR encoder to be updated)
        Lawena - uses commands for Lawena (settings will be overwritten by Lawena)
        Do Not Record - ignores recording commands but still skips`}
      color="sec"
    >
      <option value="h264">h264</option>
      <option value="tga">tga</option>
      <option value="jpeg">jpeg</option>
      <option value="sparklyfx">SparklyFX</option>
      <option value="svr">SVR</option>
      <option value="svr.mov">SVR.mov</option>
      <option value="svr.mp4">SVR.mp4</option>
      <option value="lawena">Lawena</option>
      <option value="none">Do Not Record</option>
    </Select>
    <Input
      title="Framerate"
      bind:value={output_settings.framerate}
      color="sec"
    />
    <Switch
      title="Enable Text Chat"
      bind:value={output_settings.text_chat}
      color="sec"
    />
    <Switch
      title="Display Crosshair"
      bind:value={output_settings.crosshair}
      color="sec"
    />
    <Switch
      title="Enable Voice Chat"
      bind:value={output_settings.voice_chat}
      color="sec"
    />
    <Switch title="Display HUD" bind:value={output_settings.hud} color="sec" />
    <Switch
      title="Attempt to fix sound issues"
      bind:value={output_settings.snd_fix}
      tooltip="Reset games audio when clip is starting to undo demo glitches."
      color="sec"
    />
    <Switch
      title="Display Viewmodel"
      bind:value={output_settings.viewmodel}
      color="sec"
    />
    <Switch
      title="Prevent in-game settings from changing"
      bind:value={output_settings.lock}
      tooltip="Disables commands from certain configs that could overwrite these settings when played."
      color="sec"
    />
    <Switch
      title="Minmode Viewmodels"
      bind:value={output_settings.minmode}
      color="sec"
    />
  </div>

  <h2>Recording</h2>

  <div class="setting">
    <Input
      title="Commands"
      bind:value={recording_settings.commands}
      tooltip="Commands to run before each clip is recorded."
      color="tert"
    />
    <Input
      title="End Commands"
      bind:value={recording_settings.end_commands}
      tooltip={"Commands to run after every clip is done recording.\nUseful for resetting configs to in game settings."}
      color="tert"
    />
    <Input
      title="Delay before initial skip"
      bind:value={recording_settings.start_delay}
      tooltip={"How long to wait at the start of the demo before skipping ahead.\n1 second = 66 ticks"}
      color="tert"
      type="number"
    />
    <Input
      title="Minimum ticks between clips"
      bind:value={recording_settings.minimum_ticks_between_clips}
      tooltip={"If clips are closer than this, combine them into a single clip.\n1 second = 66 ticks"}
      color="tert"
      type="number"
    />
    <Input
      title="Ticks before bookmarks"
      bind:value={recording_settings.before_bookmark}
      tooltip={"How far before each bookmark to start recording.\n1 second = 66 ticks"}
      color="tert"
      type="number"
    />
    <Input
      title="Ticks after bookmarks"
      bind:value={recording_settings.after_bookmark}
      tooltip={"How far after each bookmark to record.\n1 second = 66 ticks"}
      color="tert"
      type="number"
    />
    <Input
      title="Ticks before killstreak per kill in streak"
      bind:value={recording_settings.before_killstreak_per_kill}
      tooltip={"The average time between kills.\nShould match ds_kill_delay times 66.\n1 second = 66 ticks"}
      color="tert"
      type="number"
    />
    <Input
      title="Ticks after killstreak"
      bind:value={recording_settings.after_killstreak}
      tooltip={"How far after each killstreak to record.\n1 second = 66 ticks"}
      color="tert"
      type="number"
    />
    <Input
      title="Interval for rewind double tapscc"
      bind:value={recording_settings.interval_for_rewind_double_taps}
      tooltip={"How close bookmarks need to be to be considered a double tap.\n1 second = 66 ticks"}
      color="tert"
      type="number"
    />
    <Input
      title="Rewind amount upon double tap"
      bind:value={recording_settings.rewind_amount}
      tooltip={"How far back it records when a double tap happens.\nUseful for if you missed a clip due to being too busy in game.\n1 second = 66 ticks"}
      color="tert"
      type="number"
    />
    <Input
      title="FOV"
      bind:value={recording_settings.fov}
      color="tert"
      type="number"
    />
    <Input
      title="Viewmodel FOV"
      bind:value={recording_settings.viewmodel_fov}
      color="tert"
      type="number"
    />
    <Switch
      title="Automatically record next demo"
      bind:value={recording_settings.record_continuous}
      tooltip="If multiple demos are in a row, automatically record the next one."
      color="tert"
    />
    <Switch
      title="Prevent idle hours"
      bind:value={recording_settings.auto_close}
      tooltip="Automatically close TF2 when the last clip is finished recording."
      color="tert"
    />
    <Switch
      title="Automatically label videos"
      bind:value={recording_settings.auto_suffix}
      tooltip="Add a label to the end of each clip with information from the bookmarks."
      color="tert"
    />
    <Switch
      title="Default to Third Person"
      bind:value={recording_settings.third_person}
      tooltip={`Changes default recording to third person.\nWhen disabled always records in First Person`}
      color="tert"
    />
    <Switch
      title="Prevent Taunt Menu Bug"
      bind:value={recording_settings.prevent_taunt}
      tooltip="Disabled the taunt command to prevent the menu from opening."
      color="tert"
    />
  </div>

  <h2>Addons</h2>

  {#if Object.keys(addons).length === 0}
    <div class="setting">
      <div class="settings__span">No Addons installed</div>
    </div>
  {/if}

  {#each Object.keys(addons) as addon}
    <h3>{addon}</h3>
    <div class="setting">
      {#each Object.keys(addons[addon]) as addonSetting}
        {#if addons[addon][addonSetting].type === "string"}
          <Input
            title={addons[addon][addonSetting].title || addonSetting}
            bind:value={addons[addon][addonSetting].value}
            tooltip={addons[addon][addonSetting].tooltip}
          />
        {:else if addons[addon][addonSetting].type === "int"}
          <Input
            title={addons[addon][addonSetting].title || addonSetting}
            bind:value={addons[addon][addonSetting].value}
            tooltip={addons[addon][addonSetting].tooltip}
            type="number"
          />
        {:else if addons[addon][addonSetting].type === "bool" || addons[addon][addonSetting].type === "toggle"}
          <Switch
            title={addons[addon][addonSetting].title || addonSetting}
            bind:value={addons[addon][addonSetting].value}
            tooltip={addons[addon][addonSetting].tooltip}
          />
        {/if}
      {/each}
    </div>
  {/each}

  <div class="setting">
    <button on:click={toggle} class="cancel-btn">Cancel</button>
    <button on:click={saveSettings} class="Save"> Save </button>
  </div>
</Modal>

<style lang="scss">
  .btn {
    padding: 0.3rem 0.5rem;
    height: 100%;
    width: fit-content;

    display: flex;
    align-items: center;
    justify-content: center;
  }

  .setting {
    display: grid;
    grid-template-columns: 1fr 1fr;
    justify-content: space-between;
    gap: 1rem;
    margin: auto;
    margin-bottom: 2rem;
    width: 100%;
    max-width: 1000px;
    padding: 0rem 1rem;
  }

  .settings {
    &__input-group {
      display: flex;
      flex-direction: column;
      align-items: flex-start;
      width: 100%;
    }

    &__span {
      grid-column-start: 1;
      grid-column-end: span 2;
    }
  }
</style>
