<script>
  // @ts-ignore
  import { invoke } from "@tauri-apps/api/tauri";
  let settings = {};
  let output_settings = {};
  let recording_settings = {};
  let addons = {};

  async function loadSettings() {
    settings = await invoke("load_settings");
    output_settings = settings.output;
    recording_settings = settings.recording;
    addons = settings.addons;
    console.log(addons);
  }

  async function saveSettings() {
    let answer = await invoke("save_settings", {
      newSettings: JSON.stringify(settings),
    });
    console.log(answer);
    // @ts-ignore
    window.location = "/";
  }

  loadSettings();
</script>

<h1>Settings</h1>

<div class="setting">
  <div class="settings__input-group settings__span">
    <label
      for="tf_folder"
      class="settings__label tooltip"
      data-tooltip="The full filepath to your \tf folder within the Team Fortress 2 game files"
      style={`--kills: 0;`}
    >
      \tf Folder
    </label>
    <input
      bind:value={settings.tf_folder}
      id="tf_folder"
      class="settings__input input"
    />
  </div>
  <div class="settings__switch">
    <label class="switch">
      <input type="checkbox" bind:checked={settings.safe_mode} />
      <span class="slider round"></span>
    </label>
    <div
      class="tooltip"
      data-tooltip="If a VDM already exists, do not rewrite it."
      style={`--kills: 0;`}
    >
      Prevent overwriting previously made VDMs
    </div>
  </div>
  <div class="settings__switch">
    <label class="switch">
      <input type="checkbox" bind:checked={settings.clear_events} />
      <span class="slider round"></span>
    </label>
    <div
      class="tooltip"
      data-tooltip="Empty the _events.txt or KillStreaks.txt files once completed."
      style={`--kills: 0;`}
    >
      Clear events after running
    </div>
  </div>
  <div class="settings__switch">
    <label class="switch">
      <input type="checkbox" bind:checked={settings.save_backups} />
      <span class="slider round"></span>
    </label>
    <div
      class="tooltip"
      data-tooltip="Save a backup of the original _events.txt or Killstreaks.txt file in the Documents folder."
      style={`--kills: 0;`}
    >
      Save backups of events
    </div>
  </div>
</div>

<h2>Output</h2>

<div class="setting">
  <div class="settings__input-group">
    <label
      for="method"
      class="settings__label tooltip"
      data-tooltip={`The method you want to use to record each clip:
      h264 - QuickTime encoding to .mp4
      jpeg - generates .jpeg image sequence
      tga - generates .tga image sequence
      SVR - uses Source Video Render to record .mp4
      Lawena - uses commands for Lawena (settings will be overwritten by Lawena)
      Do Not Record - ignores recording commands but still skips
`}
      style={`--kills: 6;`}
    >
      Recording Method
    </label>
    <select
      bind:value={output_settings.method}
      id="method"
      class="settings__input input--sec"
    >
      <option value="h264">h264</option>
      <option value="jpeg">jpeg</option>
      <option value="tga">tga</option>
      <option value="svr">SVR</option>
      <option value="lawena">Lawena</option>
      <option value="none">Do Not Record</option>
    </select>
  </div>
  <div class="settings__input-group">
    <label for="framerate" class="settings__label">Framerate</label>
    <input
      bind:value={output_settings.framerate}
      id="framerate"
      class="settings__input input--sec"
      type="number"
    />
  </div>
  <div class="settings__switch">
    <label class="switch">
      <input type="checkbox" bind:checked={output_settings.text_chat} />
      <span class="slider round slider--sec"></span>
    </label>
    Enable Text Chat
  </div>
  <div class="settings__switch">
    <label class="switch">
      <input type="checkbox" bind:checked={output_settings.crosshair} />
      <span class="slider round slider--sec"></span>
    </label>
    Display Crosshair
  </div>
  <div class="settings__switch">
    <label class="switch">
      <input type="checkbox" bind:checked={output_settings.voice_chat} />
      <span class="slider round slider--sec"></span>
    </label>
    Enable Voice Chat
  </div>
  <div class="settings__switch">
    <label class="switch">
      <input type="checkbox" bind:checked={output_settings.hud} />
      <span class="slider round slider--sec"></span>
    </label>
    Display HUD
  </div>
  <div class="settings__switch">
    <label class="switch">
      <input type="checkbox" bind:checked={output_settings.snd_fix} />
      <span class="slider round slider--sec"></span>
    </label>
    <div
      class="tooltip"
      data-tooltip="Reset games audio when clip is starting to undo demo glitches."
      style={`--kills: 0;`}
    >
      Attempt to fix sound issues.
    </div>
  </div>
  <div class="settings__switch">
    <label class="switch">
      <input type="checkbox" bind:checked={output_settings.viewmodel} />
      <span class="slider round slider--sec"></span>
    </label>
    Display Viewmodel
  </div>
  <div class="settings__switch">
    <label class="switch">
      <input type="checkbox" bind:checked={output_settings.lock} />
      <span class="slider round slider--sec"></span>
    </label>
    <div
      class="tooltip"
      data-tooltip="Disables commands from certain configs that could overwrite these settings when played."
      style={`--kills: 0;`}
    >
      Prevent in-game settings from changing.
    </div>
  </div>
</div>

<h2>Recording</h2>

<div class="setting">
  <div class="settings__input-group">
    <label
      for="commands"
      class="settings__label tooltip"
      data-tooltip="Commands to run before every clip is recorded."
      style={`--kills: 0;`}
    >
      Commands
    </label>
    <input
      bind:value={recording_settings.commands}
      id="commands"
      class="settings__input input--tert"
    />
  </div>
  <div class="settings__input-group">
    <label
      for="end_commands"
      class="settings__label tooltip"
      data-tooltip={"Commands to run after every clip is done recording.\n\rUseful for resetting configs to in game settings."}
      style={`--kills: 1;`}
    >
      End Commands
    </label>
    <input
      bind:value={recording_settings.end_commands}
      id="end_commands"
      class="settings__input input--tert"
    />
  </div>
  <div class="settings__input-group">
    <label
      for="start_delay"
      class="settings__label tooltip"
      data-tooltip={"How long to wait at the start of the demo before skipping ahead.\n\r1 second = 66 ticks"}
      style={`--kills: 1;`}
    >
      Delay before initial skip
    </label>
    <input
      bind:value={recording_settings.start_delay}
      id="start_delay"
      class="settings__input input--tert"
      type="number"
    />
  </div>
  <div class="settings__input-group">
    <label
      for="minimum_ticks_between_clips"
      class="settings__label tooltip"
      data-tooltip={"If clips are closer than this, combine them into a single clip.\n\r1 second = 66 ticks"}
      style={`--kills: 1;`}
    >
      Minimum ticks between clips
    </label>
    <input
      bind:value={recording_settings.minimum_ticks_between_clips}
      id="minimum_ticks_between_clips"
      class="settings__input input--tert"
      type="number"
    />
  </div>
  <div class="settings__input-group">
    <label
      for="before_bookmark"
      class="settings__label tooltip"
      data-tooltip={"How far before each bookmark to start recording.\n\r1 second = 66 ticks"}
      style={`--kills: 1;`}
    >
      Ticks before bookmarks
    </label>
    <input
      bind:value={recording_settings.before_bookmark}
      id="before_bookmark"
      class="settings__input input--tert"
      type="number"
    />
  </div>
  <div class="settings__input-group">
    <label
      for="after_bookmark"
      class="settings__label tooltip"
      data-tooltip={"How far after each bookmark to record.\n\r1 second = 66 ticks"}
      style={`--kills: 1;`}
    >
      Ticks after bookmarks
    </label>
    <input
      bind:value={recording_settings.after_bookmark}
      id="after_bookmark"
      class="settings__input input--tert"
      type="number"
    />
  </div>
  <div class="settings__input-group">
    <label
      for="before_killstreak_per_kill"
      class="settings__label tooltip"
      data-tooltip={"The average time between kills.\n\rShould match ds_kill_delay times 66.\n\r1 second = 66 ticks"}
      style={`--kills: 2;`}
    >
      Ticks before killstreak per kill in streak
    </label>
    <input
      bind:value={recording_settings.before_killstreak_per_kill}
      id="before_killstreak_per_kill"
      class="settings__input input--tert"
      type="number"
    />
  </div>
  <div class="settings__input-group">
    <label
      for="after_killstreak"
      class="settings__label tooltip"
      data-tooltip={"How far after each killstreak to record.\n\r1 second = 66 ticks"}
      style={`--kills: 1;`}
    >
      Ticks after killstreak
    </label>
    <input
      bind:value={recording_settings.after_killstreak}
      id="after_killstreak"
      class="settings__input input--tert"
      type="number"
    />
  </div>
  <div class="settings__input-group">
    <label
      for="interval_for_rewind_double_taps"
      class="settings__label tooltip"
      data-tooltip={"How close bookmarks need to be to be considered a double tap.\n\r1 second = 66 ticks"}
      style={`--kills: 1;`}
    >
      Interval for rewind double taps
    </label>
    <input
      bind:value={recording_settings.interval_for_rewind_double_taps}
      id="interval_for_rewind_double_taps"
      class="settings__input input--tert"
      type="number"
    />
  </div>
  <div class="settings__input-group">
    <label
      for="rewind_amount"
      class="settings__label tooltip"
      data-tooltip={"How far back it records when a double tap happens.\n\rUseful for if you missed a clip due to being too busy in game.\n\r1 second = 66 ticks"}
      style={`--kills: 2;`}
    >
      Rewind amount upon double tap
    </label>
    <input
      bind:value={recording_settings.rewind_amount}
      id="rewind_amount"
      class="settings__input input--tert"
      type="number"
    />
  </div>
  <div class="settings__input-group">
    <label for="FOV" class="settings__label">FOV</label>
    <input
      bind:value={recording_settings.fov}
      id="FOV"
      class="settings__input input--tert"
      type="number"
    />
  </div>
  <div class="settings__input-group">
    <label for="viewodel_fov" class="settings__label">Viewmodel FOV</label>
    <input
      bind:value={recording_settings.viewmodel_fov}
      id="viewodel_fov"
      class="settings__input input--tert"
      type="number"
    />
  </div>
  <div class="settings__switch">
    <label class="switch">
      <input
        type="checkbox"
        bind:checked={recording_settings.record_continuous}
      />
      <span class="slider round slider--tert"></span>
    </label>
    Automatically record next demo when current demo completes
  </div>
  <div class="settings__switch">
    <label class="switch">
      <input type="checkbox" bind:checked={recording_settings.auto_close} />
      <span class="slider round slider--tert"></span>
    </label>
    <div
      class="tooltip"
      data-tooltip="Automatically close TF2 when the last clip is finished recording."
      style={`--kills: 0;`}
    >
      Prevent idle hours
    </div>
  </div>
  <div class="settings__switch">
    <label class="switch">
      <input type="checkbox" bind:checked={recording_settings.auto_suffix} />
      <span class="slider round slider--tert"></span>
    </label>
    <div
      class="tooltip"
      data-tooltip="Add a label to the end of each clip with information from the bookmarks."
      style={`--kills: 0;`}
    >
      Automatically label videos
    </div>
  </div>
  <div class="settings__switch">
    <label class="switch">
      <input type="checkbox" bind:checked={recording_settings.third_person} />
      <span class="slider round slider--tert"></span>
    </label>
    <div
      class="tooltip"
      data-tooltip={`Changes default recording to third person.\n\rWhen disabled always records in First Person`}
      style={`--kills: 1;`}
    >
      Default to Third Person
    </div>
  </div>
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
        <div class="settings__input-group">
          <label
            for={`${addon}-${addonSetting}`}
            class={`settings__label ${
              addons[addon][addonSetting].tooltip ? "tooltip" : ""
            }`}
            data-tooltip={addons[addon][addonSetting].tooltip}
            style={`--kills: 0;`}
          >
            {addons[addon][addonSetting].title || addonSetting}
          </label>
          <input
            bind:value={addons[addon][addonSetting].value}
            id={`${addon}-${addonSetting}`}
            class="settings__input input"
          />
        </div>
      {:else if addons[addon][addonSetting].type === "int"}
        <div class="settings__input-group">
          <label
            for={`${addon}-${addonSetting}`}
            class={`settings__label ${
              addons[addon][addonSetting].tooltip ? "tooltip" : ""
            }`}
            data-tooltip={addons[addon][addonSetting].tooltip}
            style={`--kills: 0;`}
          >
            {addons[addon][addonSetting].title || addonSetting}
          </label>
          <input
            bind:value={addons[addon][addonSetting].value}
            id={`${addon}-${addonSetting}`}
            class="settings__input input"
            type="number"
          />
        </div>
      {:else if addons[addon][addonSetting].type === "bool" || addons[addon][addonSetting].type === "toggle"}
        <div class="settings__switch">
          <label class="switch">
            <input
              type="checkbox"
              bind:checked={addons[addon][addonSetting].value}
            />
            <span class="slider round"></span>
          </label>
          <div
            class={`${addons[addon][addonSetting].tooltip ? "tooltip" : ""}`}
            data-tooltip={addons[addon][addonSetting].tooltip}
            style={`--kills: 0;`}
          >
            {addons[addon][addonSetting].title || addonSetting}
          </div>
        </div>
      {/if}
    {/each}
  </div>
{/each}

<div class="setting">
  <a href="/" class="cancel-btn">Cancel</a>
  <button on:click={saveSettings} class="Save"> Save </button>
</div>

<style lang="scss">
  .setting {
    display: grid;
    grid-template-columns: 1fr 1fr;
    justify-content: space-between;
    gap: 1rem;
    margin: auto;
    margin-bottom: 2rem;
    width: 100%;
    max-width: 1000px;
    padding: 0rem 2rem;
  }

  .settings {
    &__input-group {
      display: flex;
      flex-direction: column;
      align-items: flex-start;
      width: 100%;

      & > input,
      & > select {
        width: 100%;
      }
    }

    &__switch {
      display: flex;
      align-items: center;
      gap: 1rem;
      width: 100%;
    }

    &__span {
      grid-column-start: 1;
      grid-column-end: span 2;
    }
  }

  .tooltip:hover::before,
  .tooltip:active::before,
  .tooltip:focus::before {
    text-align: left;
  }
</style>
