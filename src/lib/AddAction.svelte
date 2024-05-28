<script>
  import { createEventDispatcher } from "svelte";
  import Modal from "./Modal.svelte";
  import { invoke } from "@tauri-apps/api";

  const dispatch = createEventDispatcher();

  let enabled = false;

  function toggle() {
    enabled = !enabled;
  }

  async function add(factory) {
    switch (factory) {
      case "SkipAhead":
        dispatch("add", [
          {
            factory: "SkipAhead",
            name: "Unnamed",
            skip_to_tick: 0,
            skip_to_time: null,
            start_tick: 0,
            start_time: null,
          },
        ]);
        break;
      case "StopPlayback":
        dispatch("add", [
          {
            factory: "StopPlayback",
            name: "Unnamed",
            start_tick: 0,
            start_time: null,
          },
        ]);
        break;
      case "PlayCommands":
        dispatch("add", [
          {
            commands: "",
            factory: "PlayCommands",
            name: "Unnamed",
            start_tick: 0,
            start_time: null,
          },
        ]);
        break;
      case "ScreenFadeStart":
        dispatch("add", [
          {
            duration: 0,
            factory: "ScreenFadeStart",
            fade_in_enabled: false,
            fade_out_enabled: false,
            hold_time: 0,
            modulate_enabled: false,
            name: "Unnamed",
            purge_enabled: false,
            rgba1: [0, 0, 0, 0],
            start_tick: null,
            start_time: null,
            stay_out_enabled: false,
          },
        ]);
        break;
      case "TextMessageStart":
        dispatch("add", [
          {
            effect: "FadeInOut",
            factory: "TextMessageStart",
            fade_in: 0,
            fade_out: 0,
            font: "",
            fx_time: 0,
            hold_time: 0,
            message: "",
            name: "Unnamed",
            rgba1: [0, 0, 0, 0],
            rgba2: [0, 0, 0, 0],
            start_tick: 0,
            start_time: null,
            xy: [0, 0],
          },
        ]);
        break;
      case "PlayCDTrackStart":
        dispatch("add", [
          {
            factory: "PlayCDTrackStart",
            name: "Unnamed",
            start_tick: 0,
            start_time: null,
            track: 0,
          },
        ]);
        break;
      case "PlaySoundStart":
        dispatch("add", [
          {
            factory: "PlaySoundStart",
            name: "Unnamed",
            sound: "",
            start_tick: 0,
            start_time: null,
          },
        ]);
        break;
      case "Pause":
        dispatch("add", [
          {
            duration: 0,
            factory: "Pause",
            name: "Unnamed",
            start_tick: 0,
            start_time: null,
            stop_tick: 0,
            stop_time: null,
          },
        ]);
        break;
      case "ChangePlaybackRate":
        dispatch("add", [
          {
            factory: "ChangePlaybackRate",
            name: "Unnamed",
            playback_rate: 1,
            start_tick: 0,
            start_time: null,
            stop_tick: 0,
            stop_time: null,
          },
        ]);
        break;
      case "ZoomFov":
        dispatch("add", [
          {
            factory: "ZoomFov",
            final_fov: 90,
            fov_hold: 0,
            fov_rate_in: 0,
            fov_rate_out: 0,
            name: "Unnamed",
            spline: false,
            start_tick: 0,
            start_time: null,
            stayout: false,
          },
        ]);
        break;
      case "Record Clip":
        let [commands, end_commands] = await buildCommands();

        dispatch("add", [
          {
            commands: "exec melies;",
            factory: "PlayCommands",
            name: "Exec Melies",
            start_tick: 17,
            start_time: null,
          },
          {
            commands: commands,
            factory: "PlayCommands",
            name: "Start Recording",
            start_tick: 66,
            start_time: null,
          },
          {
            commands: end_commands,
            factory: "PlayCommands",
            name: "Stop Recording",
            start_tick: 132,
            start_time: null,
          },
        ]);
        break;
      case "Play Demo":
        dispatch("add", [
          {
            commands: "playdemo demoname;",
            factory: "PlayCommands",
            name: "Start the next demo",
            start_tick: 0,
            start_time: null,
          },
        ]);
        break;
      default:
        break;
    }

    toggle();
  }

  async function buildCommands() {
    let settings = await invoke("load_settings");

    let commands = "";
    let end_commands = "";

    switch (settings.output.method) {
      case "sparklyfx":
        commands = `sf_recorder_start ${settings.output.folder}\\clip_name;`;
        end_commands = "sf_recorder_stop;";
        break;
      case "h264" || "jpeg":
        commands = `host_framerate ${settings.output.framerate}; startmovie clip_name ${settings.output.method}; clear`;
        end_commands = `${settings.recording.end_commands}; endmovie; host_framerate 0;`;
        break;
      case "lawena":
        commands = `startrecording;`;
        end_commands = `${settings.recording.end_commands}; stoprecording;`;
        break;
      default:
        commands = `host_framerate ${settings.output.framerate}; startmovie clip_name; clear`;
        end_commands = `${settings.recording.end_commands}; endmovie; host_framerate 0;`;
        break;
    }

    return [commands, end_commands];
  }
</script>

<button on:click={toggle}> Add Action </button>
<Modal color="sec" {toggle} {enabled} small>
  <h4>Basic</h4>
  <div class="buttons">
    {#each ["SkipAhead", "StopPlayback", "PlayCommands", "ScreenFadeStart", "TextMessageStart", "PlayCDTrackStart", "PlaySoundStart", "Pause", "ChangePlaybackRate", "ZoomFov"] as factory}
      <button class="btn" on:click={() => add(factory)}>{factory}</button>
    {/each}
  </div>
  <h4>Presets</h4>
  <div class="buttons">
    {#each ["Record Clip", "Play Demo"] as factory}
      <button class="btn" on:click={() => add(factory)}>{factory}</button>
    {/each}
  </div>
</Modal>

<style lang="scss">
  .buttons {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1rem;
  }

  h4 {
    text-align: left;
  }
</style>
