<script>
  import { run } from "svelte/legacy";

  // @ts-ignore
  import { invoke } from "@tauri-apps/api/core";
  import Modal from "$lib/components/Modal.svelte";
  import {
    faGear,
    faPlus,
    faMinus,
    faFolderOpen,
  } from "@fortawesome/free-solid-svg-icons";
  import Fa from "svelte-fa";
  import { createEventDispatcher } from "svelte";
  import Input from "$lib/components/Input.svelte";
  import Switch from "$lib/components/Switch.svelte";
  import Select from "$lib/components/Select.svelte";
  import Collapse from "$lib/components/Collapse.svelte";
  import Addons from "$lib/addons/Addons.svelte";
  import Theme from "$lib/Theme.svelte";
  import { inv } from "mathjs";
  const dispatch = createEventDispatcher();
  let settings = $state({});
  let outputSettings = $state({});
  let recordingSettings = $state({});
  let automationSettings = $state({});
  let demoManagerSettings = $state({});
  let featureSettings = $state({});
  let hlaeSettings = $state({});
  let addons = $state({});
  let enabled = $state(false);
  let addingCustomInstall = $state(false);
  let newFolderName = $state("");

  let toggle = () => {
    enabled = !enabled;

    if (enabled) {
      loadSettings();
    }

    if (!enabled) {
      dispatch("close");
    }
  };

  async function loadSettings() {
    settings = await invoke("load_settings");
    outputSettings = settings.output;
    recordingSettings = settings.recording;
    automationSettings = settings.automation;
    demoManagerSettings = settings.demo_manager;
    featureSettings = settings.features;
    hlaeSettings = settings.hlae;
    addons = settings.addons;
    $state.snapshot(settings);
  }

  async function saveSettings() {
    let answer = await invoke("save_settings", {
      newSettings: JSON.stringify(settings),
    });
    console.log(answer);

    toggle();
  }

  async function buildInstall() {
    let res = await invoke("build_install", {
      folderName: newFolderName,
    });

    settings.alt_installs = [...settings.alt_installs, res];

    addingCustomInstall = false;

    invoke("open_install_folder", {
      install: res.tf_folder,
    });
  }

  function addInstall() {
    settings.alt_installs = [
      ...settings.alt_installs,
      {
        name: "",
        tf_folder: "",
      },
    ];
  }

  function removeInstall(install) {
    settings.alt_installs.splice(install, 1);

    settings.alt_installs = settings.alt_installs;
  }
</script>

{#snippet general()}
  <div class="setting">
    <div class="settings__input-group settings__span tf-folder">
      <Input
        title="\tf Folder"
        bind:value={settings.tf_folder}
        tooltip="The full filepath to your \tf folder within the Team Fortress 2 game files."
        filepath={true}
        directory={true}
      />
      <button
        class="btn--sec"
        onclick={() =>
          invoke("open_install_folder", {
            install: settings.tf_folder,
          })}
      >
        <Fa icon={faFolderOpen} color={`var(--sec)`} />
      </button>
    </div>
    <div class="settings__input-group settings__span">
      {#each settings.alt_installs || [] as install, index}
        <div class="settings__input-group settings__span install-group">
          <Input
            title={index === 0 ? "Nickname" : ""}
            bind:value={install.name}
            tooltip="What you want to call this custom install."
          />
          <Input
            title={index === 0 ? "Custom \\tf Folder Path" : ""}
            bind:value={install.tf_folder}
            tooltip="The full filepath to your \tf folder within the Team Fortress 2 game files."
            filepath={true}
            directory={true}
          />
          <button
            class="btn--sec"
            onclick={() =>
              invoke("open_install_folder", {
                install: install.tf_folder,
              })}
          >
            <Fa icon={faFolderOpen} color={`var(--sec)`} />
          </button>
          <button class="cancel-btn" onclick={() => removeInstall(index)}>
            <Fa icon={faMinus} color={`var(--err-con-text)`} />
          </button>
        </div>
      {/each}
    </div>
    {#if addingCustomInstall}
      <div class="settings__input-group settings__span new-install">
        <Input title="New Folder Name" bind:value={newFolderName} color="pri" />
        <button onclick={buildInstall}> Build Install </button>
        <button class="cancel-btn" onclick={(addingCustomInstall = false)}>
          Cancel
        </button>
      </div>
    {/if}
    <div class="btn-container settings__span">
      <button class="btn-custom-install" onclick={addInstall}>
        <Fa icon={faPlus} color={`var(--pri)`} />
        Existing Custom Install
      </button>
      <button
        class="btn-custom-install btn--sec"
        onclick={(addingCustomInstall = true)}
        disabled={addingCustomInstall}
      >
        <Fa icon={faPlus} color={`var(--sec)`} />
        New Custom Install
      </button>
      <a href="https://www.youtube.com/watch?v=lH4scK3uB_s" target="_blank">
        What's this?
      </a>
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

  <br />
  {#if automationSettings.enabled}
    <h2>Automation</h2>
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
  {/if}
{/snippet}

{#snippet demoManager()}
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
    <Switch
      title="Automatically rename events and vdm commands"
      bind:value={demoManagerSettings.auto_update}
      tooltip="When a demo is renamed, automatically update the
events and vdm commands that reference the demo."
    />
  </div>
{/snippet}

{#snippet output()}
  <div class="setting">
    <div class="settings__input-group settings__span">
      <Input
        title="Output Folder"
        bind:value={outputSettings.folder}
        tooltip="The full filepath to the folder you want to output to.
If left blank, the output folder will default to your sparklyfx settings."
        display={outputSettings.method === "sparklyfx"}
        filepath={true}
        directory={true}
        color="sec"
      />
    </div>
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
      display={!["sparklyfx", "svr", "svr.mov", "svr.mp4", "lawena"].includes(
        outputSettings.method
      )}
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
    <Switch
      title="Minmode Viewmodels"
      bind:value={outputSettings.minmode}
      color="sec"
    />
  </div>
{/snippet}

{#snippet recording()}
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
    <Switch
      title="Prefer Casting Essentials spec commands"
      bind:value={recordingSettings.use_ce_spec}
      tooltip="Using Casting Essentials commands instead of the default ones.
Useful in STVs when the player could be dead."
      color="tert"
    />
    <!-- <Switch
      title="Prevent Taunt Menu Bug"
      bind:value={recordingSettings.prevent_taunt}
      tooltip="Disabled the taunt command to prevent the menu from opening."
      color="tert"
    /> -->
  </div>
{/snippet}

{#snippet hlae()}
  {#if !["svr", "svr.mov", "svr.mp4"].includes(outputSettings.method)}
    <div class="setting">
      {#if outputSettings.method === "sparklyfx"}
        <Input
          title="HLAE .exe Path"
          bind:value={hlaeSettings.hlae_path}
          color="sec"
          filepath={true}
          filetype=".exe"
        />
        <Input
          title="SparklyFX .dll Path"
          bind:value={hlaeSettings.sparklyfx_path}
          color="sec"
          filepath={true}
          filetype=".dll"
        />
      {/if}
      <div class="settings__input-group settings__span">
        <Input
          title="TF2 Launch Options"
          bind:value={hlaeSettings.launch_options}
          color="sec"
        />
      </div>
      <!-- <Switch
        title="64Bit"
        bind:value={hlaeSettings.use_64bit}
        tooltip="Launches with 64Bit TF2."
        color="sec"
      />
      <Switch
        title="Automatically playdemo"
        bind:value={hlaeSettings.playdemo}
        tooltip="Plays first demo on list as soon as it launches."
        color="sec"
      /> -->
      <Select
        title="Before Batch Recording"
        bind:value={hlaeSettings.before_batch}
        tooltip={`What do to when before starting batch recording.`}
        color="sec"
        disabled
      >
        <option value="nothing">Nothing</option>
        <option value="open">Open Output Folder</option>
        <option value="run">Run Program</option>
      </Select>
      <Select
        title="After Batch Recording"
        bind:value={hlaeSettings.after_batch}
        tooltip={`What do to when batch recording is complete.`}
        color="sec"
        disabled
      >
        <option value="nothing">Nothing</option>
        <option value="open">Open Output Folder</option>
        <option value="shutdown">Shutdown PC</option>
        <option value="run">Run Program</option>
      </Select>
      
      <Input
        title="Program to Run BEFORE Batch Recording"
        bind:value={hlaeSettings.before_batch_path}
        disabled={hlaeSettings.before_batch !== "run"}
        tooltip={hlaeSettings.before_batch !== "run" ? "Before Batch Recording must be set to 'Run Program'." : ""}
        color="sec"
      />
      
      <Input
        title="Program to Run AFTER Batch Recording"
        bind:value={hlaeSettings.after_batch_path}
        disabled={hlaeSettings.after_batch !== "run"}
        tooltip={hlaeSettings.after_batch !== "run" ? "After Batch Recording must be set to 'Run Program'." : ""}
        color="sec"
      />
      <Switch
        title="Skip Intro Video"
        bind:value={hlaeSettings.novid}
        tooltip="Uses -novid launch option."
        color="sec"
      />
    </div>
  {:else}
    <h1>SVR must be launched through the SVR client</h1>
  {/if}
{/snippet}

{#snippet addonsTab()}
  <Addons bind:addons />
{/snippet}

{#snippet theme()}
  <Theme />
{/snippet}

{#snippet features()}
  <div class="setting">
    <Switch
      title="Edit Events"
      bind:value={featureSettings.basic.editEvents}
      color="sec"
    />
    <Switch
      title="Load Backup"
      bind:value={featureSettings.basic.backups}
      color="sec"
    />
    <Switch
      title="Scan Demos"
      bind:value={featureSettings.basic.scanDemos}
      color="sec"
    />
  </div>
  <h3>Advanced Features</h3>
  <div class="setting">
    <Switch
      title="VDM Editor"
      bind:value={featureSettings.advanced.vdmeditor}
      color="sec"
    />
    <Switch
      title="Demo Manager"
      bind:value={featureSettings.advanced.demoManager}
      color="sec"
    />
    <Switch
      title="Sort Footage"
      bind:value={featureSettings.advanced.sortFootage}
      color="sec"
    />
  </div>
  <h3>Demo Scanner</h3>
  <div class="setting">
    <Switch
      title="Display Med Picks"
      bind:value={featureSettings.demo_scanner.med_picks}
      color="sec"
    />
    <Switch
      title="Display Player breakdowns"
      bind:value={featureSettings.demo_scanner.breakdowns}
      color="sec"
    />
    <Switch
      title="Display Killstreaks"
      bind:value={featureSettings.demo_scanner.killstreaks}
      color="sec"
    />
    <Switch
      title="Display Chat"
      bind:value={featureSettings.demo_scanner.chat}
      color="sec"
    />
    <Switch
      title="Display Air Shots"
      bind:value={featureSettings.demo_scanner.airshots}
      color="sec"
    />
    <Switch
      title="Display Timeline View"
      bind:value={featureSettings.demo_scanner.timeline}
      color="sec"
    />
    <Switch
      title="Load RGL.gg player information"
      tooltip="Replaces player name with RGL.gg profile name if available."
      bind:value={featureSettings.demo_scanner.rgl}
      color="sec"
    />
    <Switch
      title="Link to Logs.tf profiles"
      bind:value={featureSettings.demo_scanner.logstf}
      color="sec"
    />
  </div>
  <h3>Deprecated Features</h3>
  <p>Note: These features are no longer supported or updated.</p>
  <div class="setting">
    <Switch
      title="Load from Logs.tf"
      bind:value={featureSettings.deprecated.logstf}
      tooltip="Deprecated due to being less accurate and less applicable compared to scanning demos."
      color="sec"
    />
  </div>
{/snippet}
<button class="btn btn--sec" onclick={toggle}>
  <Fa icon={faGear} color={`var(--sec)`} />
</button>
<Modal
  color="sec"
  height="100vw"
  {toggle}
  {enabled}
  tabHeaders={[
    "General",
    "Output",
    "Recording",
    "Demo Manager",
    {
      title: outputSettings.method === "sparklyfx" ? "HLAE" : "TF2",
      enabled: !["svr", "svr.mov", "svr.mp4"].includes(outputSettings.method),
    },
    "Addons",
    "Theme",
    "Features"
  ]}
  tabs={[general, output, recording, demoManager, hlae, addonsTab, theme, features]}
>
  {#snippet footer()}
    <div class="buttons">
      <button onclick={toggle} class="cancel-btn">Cancel</button>
      <button onclick={saveSettings} class="Save"> Save </button>
    </div>
  {/snippet}
</Modal>

<style lang="scss">
  .buttons {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 1rem;
    padding-top: 1rem;
  }

  .btn {
    padding: 0.3rem 0.5rem;
    height: 100%;
    width: fit-content;

    display: flex;
    align-items: center;
    justify-content: center;

    &-custom-install {
      padding: 0.3rem 0.5rem;
      height: 100%;
      width: fit-content;

      gap: 0.5rem;

      display: flex;
      align-items: center;
      justify-content: center;
    }

    &-container {
      width: fit-content;

      gap: 0.5rem;

      display: flex;
      align-items: center;
      justify-content: center;
    }
  }

  .install-group {
    display: grid;
    grid-template-columns: 0.5fr 1fr max-content max-content;
    gap: 0.5rem;
    margin-bottom: 3px;

    align-items: end;
  }

  .new-install {
    display: grid;
    grid-template-columns: 1fr max-content max-content;
    gap: 1rem;

    align-items: end;
  }

  .tf-folder {
    display: grid;
    grid-template-columns: 1fr max-content;
    gap: 0.5rem;
    align-items: end;
  }
</style>
