<script>
  // @ts-ignore
  import { invoke } from "@tauri-apps/api/core";
  import Modal from "$lib/components/Modal.svelte";
  import { faGear } from "@fortawesome/free-solid-svg-icons";
  import Fa from "svelte-fa";
  import { createEventDispatcher } from "svelte";
  import Input from "$lib/components/Input.svelte";
  import Switch from "$lib/components/Switch.svelte";
  import Select from "$lib/components/Select.svelte";
  import Collapse from "$lib/components/Collapse.svelte";
  import Addons from "$lib/addons/Addons.svelte";
  import Theme from "$lib/Theme.svelte";
  const dispatch = createEventDispatcher();
  let settings = {};
  let outputSettings = {};
  let recordingSettings = {};
  let automationSettings = {};
  let demoManagerSettings = {};
  let hlaeSettings = {};
  let addons = {};
  let enabled = false;

  let toggle = () => (enabled = !enabled);

  async function loadSettings() {
    settings = await invoke("load_settings");
    outputSettings = settings.output;
    recordingSettings = settings.recording;
    automationSettings = settings.automation;
    demoManagerSettings = settings.demo_manager;
    hlaeSettings = settings.hlae;
    addons = settings.addons;
    console.log(settings)
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
        bind:value={outputSettings.folder}
        tooltip="The full filepath to the folder you want to output to.
If left blank, the output folder will default to your sparklyfx settings."
        display={outputSettings.method === "sparklyfx"}
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
    <Switch
      title="Display Automation Tools"
      bind:value={automationSettings.enabled}
      tooltip="Shows extra buttons useful for quickly grabbing clips
in the demo scanner."
    />
    <Switch
      title="Absolute File Paths"
      bind:value={settings.absolute_file_paths}
      tooltip=".VDM files made with the demo scanner will now be saved to the folder the demo is in.
Note: the order of the demos remains the same (top to bottom)

DO NOT MOVE THE DEMOS FROM THEIR FOLDERS IF THIS IS ENABLED."
    />
    <Switch
      title="Treat POV demos as STV demos"
      bind:value={settings.pov_as_stv}
      tooltip="Forces the demo scanner to show all information
on all players in POV demos."
    />
  </div>

  <h1>Demo Manager</h1>
  <div class="setting">
    <div class="settings__input-group settings__span">
      <Input
        title="Default demoname format when renaming"
        bind:value={demoManagerSettings.default_name}
        tooltip="Allows you to change the default demoname format when renaming or mass renaming."
      />
      <p class="settings__span">
        Metadata elements:
        <span
          class="tooltip"
          data-tooltip={`The player's nickname\nExample: JoseGonzales2007`}
          style="--kills: 1;"
        >
          {`{nickname}`},
        </span>
        <span
          class="tooltip"
          data-tooltip={`The number of ticks in the demo\nExample: 12345`}
          style="--kills: 1;"
        >
          {`{ticks}`},
        </span>
        <span
          class="tooltip"
          data-tooltip={`The server the demo was played on\nExample: skial.harvest.247`}
          style="--kills: 1;"
        >
          {`{server}`},
        </span>
        <span
          class="tooltip"
          data-tooltip={`The map the demo was played on\nExample: koth_harvest_final`}
          style="--kills: 1;"
        >
          {`{map}`},
        </span>
        <span
          class="tooltip"
          data-tooltip={`The date the demo was created\nExample: 2022-01-01`}
          style="--kills: 1;"
        >
          {`{date}`},
        </span>
        <span
          class="tooltip"
          data-tooltip={`The time the demo was created\nExample: 03-10-35`}
          style="--kills: 1;"
        >
          {`{time}`}
        </span>
      </p>
    </div>
    <Switch
      title="Confirm before deleting demos"
      bind:value={settings.confirm_delete}
      tooltip="Gives a confirmation prompt before deleting demos."
    />
  </div>

  {#if automationSettings.enabled}
    <Collapse defaultOpen={true} title="Automation Tools">
      <div class="bordered bordered--pri ">
        <div class="setting">
          <Switch
            title="Airshots"
            bind:value={automationSettings.airshots}
            tooltip="Bookmark anytime a player hits an airshot."
          />
          <Switch
            title="Med Picks"
            bind:value={automationSettings.med_picks}
            tooltip="Bookmark anytime a player kills a Medic."
          />
          <Switch
            title="Killstreaks"
            bind:value={automationSettings.killstreaks}
            tooltip="Bookmark anytime a player gets a killstreak."
          />
          <Switch
            title="Record Entire Life"
            bind:value={automationSettings.whole_life}
            tooltip="Record the entire life. Will use standard bookmarks if disabled."
          />
        </div>

        <h4>Classes</h4>

        <div class="setting">
          {#each Object.keys(automationSettings.classes) as class_name}
            
            <Switch
              title="Record {class_name[0].toUpperCase() + class_name.slice(1)}"
              bind:value={automationSettings.classes[class_name]}
              tooltip="Record clips of {class_name}."
            />
          {/each}
        </div>
      </div>
    </Collapse>
  {/if}

  <h2>Output</h2>

  <div class="setting">
    <div class="settings__input-group settings__span">
      <Input
        title="Clip name format"
        bind:value={outputSettings.clip_name_template}
        tooltip={`How you want clips to be titled by default.
        {demo_name} - The name of the demo
        {start_tick} - The start tick of the clip
        {end_tick} - The end tick of the clip
        {suffix} - If the clip is a bookmark or killstreak
        {bookmarks} - The bookmarks of the clip
        {recording_method} - The recording method of the clip`}
        color="sec"
      />
    </div>
    <Select
      title="Recording Method"
      bind:value={outputSettings.method}
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
      bind:value={outputSettings.framerate}
      color="sec"
      display={!["sparklyfx", "svr", "svr.mov", "svr.mp4", "lawena"].includes(outputSettings.method)}
    />
    {#if ["sparklyfx", "svr", "svr.mov", "svr.mp4", "lawena"].includes(outputSettings.method)}
      <div></div>
    {/if}
    <Switch
      title="Enable Text Chat"
      bind:value={outputSettings.text_chat}
      color="sec"
    />
    <Switch
      title="Display Crosshair"
      bind:value={outputSettings.crosshair}
      color="sec"
    />
    <Switch
      title="Enable Voice Chat"
      bind:value={outputSettings.voice_chat}
      color="sec"
    />
    <Switch title="Display HUD" bind:value={outputSettings.hud} color="sec" />
    <Switch
      title="Attempt to fix sound issues"
      bind:value={outputSettings.snd_fix}
      tooltip="Reset games audio when clip is starting to undo demo glitches."
      color="sec"
    />
    <Switch
      title="Display Viewmodel"
      bind:value={outputSettings.viewmodel}
      color="sec"
    />
    <!-- <Switch
      title="Prevent in-game settings from changing"
      bind:value={outputSettings.lock}
      tooltip="Disables commands from certain configs that could overwrite these settings when played."
      color="sec"
    /> -->
    <Switch
      title="Minmode Viewmodels"
      bind:value={outputSettings.minmode}
      color="sec"
    />
  </div>

  <h2>Recording</h2>

  <div class="setting">
    <Input
      title="Commands"
      bind:value={recordingSettings.commands}
      tooltip="Commands to run before each clip is recorded."
      color="tert"
    />
    <Input
      title="End Commands"
      bind:value={recordingSettings.end_commands}
      tooltip={"Commands to run after every clip is done recording.\nUseful for resetting configs to in game settings."}
      color="tert"
    />
    <Input
      title="Delay before initial skip"
      bind:value={recordingSettings.start_delay}
      tooltip={"How long to wait at the start of the demo before skipping ahead.\n1 second = 66 ticks"}
      color="tert"
      type="number"
    />
    <Input
      title="Minimum ticks between clips"
      bind:value={recordingSettings.minimum_ticks_between_clips}
      tooltip={"If clips are closer than this, combine them into a single clip.\n1 second = 66 ticks"}
      color="tert"
      type="number"
    />
    <Input
      title="Ticks before bookmarks"
      bind:value={recordingSettings.before_bookmark}
      tooltip={"How far before each bookmark to start recording.\n1 second = 66 ticks"}
      color="tert"
      type="number"
    />
    <Input
      title="Ticks after bookmarks"
      bind:value={recordingSettings.after_bookmark}
      tooltip={"How far after each bookmark to record.\n1 second = 66 ticks"}
      color="tert"
      type="number"
    />
    <Input
      title="Minimum Kills in Killstreak"
      bind:value={recordingSettings.minimum_kills_in_streak}
      tooltip={"Used by the demo scanner to determine what a killstreak is."}
      color="tert"
      type="number"
    />
    <Input
      title="Ticks before killstreak per kill in streak"
      bind:value={recordingSettings.before_killstreak_per_kill}
      tooltip={"The average time between kills.\nShould match ds_kill_delay times 66.\n1 second = 66 ticks"}
      color="tert"
      type="number"
    />
    <Input
      title="Ticks after killstreak"
      bind:value={recordingSettings.after_killstreak}
      tooltip={"How far after each killstreak to record.\n1 second = 66 ticks"}
      color="tert"
      type="number"
    />
    <Input
      title="Interval for rewind double tapscc"
      bind:value={recordingSettings.interval_for_rewind_double_taps}
      tooltip={"How close bookmarks need to be to be considered a double tap.\n1 second = 66 ticks"}
      color="tert"
      type="number"
    />
    <Input
      title="FOV"
      bind:value={recordingSettings.fov}
      color="tert"
      type="number"
    />
    <Input
      title="Viewmodel FOV"
      bind:value={recordingSettings.viewmodel_fov}
      color="tert"
      type="number"
    />
    <Input
      title="Rewind amount upon double tap"
      bind:value={recordingSettings.rewind_amount}
      tooltip={"How far back it records when a double tap happens.\nUseful for if you missed a clip due to being too busy in game.\n1 second = 66 ticks"}
      color="tert"
      type="number"
    />
    <div></div>
    <Switch
      title="Automatically record next demo"
      bind:value={recordingSettings.record_continuous}
      tooltip="If multiple demos are in a row, automatically record the next one."
      color="tert"
    />
    <Switch
      title="Prevent idle hours"
      bind:value={recordingSettings.auto_close}
      tooltip="Automatically close TF2 when the last clip is finished recording."
      color="tert"
    />
    <Switch
      title="Automatically label videos"
      bind:value={recordingSettings.auto_suffix}
      tooltip="Add a label to the end of each clip with information from the bookmarks."
      color="tert"
    />
    <Switch
      title="Default to Third Person"
      bind:value={recordingSettings.third_person}
      tooltip={`Changes default recording to third person.\nWhen disabled always records in First Person`}
      color="tert"
    />
    <!-- <Switch
      title="Prevent Taunt Menu Bug"
      bind:value={recordingSettings.prevent_taunt}
      tooltip="Disabled the taunt command to prevent the menu from opening."
      color="tert"
    /> -->
  </div>

  {#if !["svr", "svr.mov", "svr.mp4"].includes(outputSettings.method)}
    <h2>
      {#if outputSettings.method === "sparklyfx"}
        HLAE
      {:else}
        TF2
      {/if}
    </h2>

    <div class="setting">
      {#if outputSettings.method === "sparklyfx"}
        <Input
          title="HLAE .exe Path"
          bind:value={hlaeSettings.hlae_path}
          color="pri"
        />
        <Input
          title="SparklyFX .dll Path"
          bind:value={hlaeSettings.sparklyfx_path}
          color="pri"
        />
      {/if}
      <div class="settings__input-group settings__span">
        <Input
          title="TF2 Launch Options"
          bind:value={hlaeSettings.launch_options}
          color="pri"
        />
      </div>
      <Switch
        title="64Bit"
        bind:value={hlaeSettings.use_64bit}
        tooltip="Launches with 64Bit TF2."
        color="pri"
      />
      <Switch
        title="Automatically playdemo"
        bind:value={hlaeSettings.playdemo}
        tooltip="Plays first demo on list as soon as it launches."
        color="pri"
      />
    </div>
  {/if}

  <Addons bind:addons={addons} />

  <Theme />

  <div class="buttons" slot="footer">
    <button on:click={toggle} class="cancel-btn">Cancel</button>
    <button on:click={saveSettings} class="Save"> Save </button>
  </div>
</Modal>

<style lang="scss">
  .buttons {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 1rem;
    padding-top: 1rem;
  }

  .bordered {
    padding: 1rem;

    & > .setting {
      margin-bottom: 0;
    }
  }

  .btn {
    padding: 0.3rem 0.5rem;
    height: 100%;
    width: fit-content;

    display: flex;
    align-items: center;
    justify-content: center;
  }
</style>
