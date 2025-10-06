<script>
  import { createEventDispatcher } from "svelte";
  import { open } from '@tauri-apps/plugin-dialog';
  import {
    faFileImport,
  } from "@fortawesome/free-solid-svg-icons";
  import Fa from "svelte-fa";

  /** @type {{value: any, title?: any, key?: any, color?: string, display?: boolean, tooltip?: string, type?: string, disabled?: boolean}} */
  let {
    value = $bindable(),
    title = null,
    key = title?.toLowerCase().replace(/ /g, "_") || "input",
    color = "pri",
    display = true,
    tooltip = "",
    type = "text",
    disabled = false,
    filepath = false,
    filetype = null,
    directory = false
  } = $props();

  const dispatch = createEventDispatcher();

  const change = () => dispatch("change", value);

  async function openFilePicker() {
    const file = await open({
      multiple: false,
      directory: directory,
    });
    console.log(file);

    if (filetype) {
      if (!file.endsWith(filetype)) {
        await confirm(
          'This must be a "' + filetype + '" file.',
          { title: 'Tauri', kind: 'warning' }
        );
        return;
      }
    }

    value = file;
  }
</script>

{#if display}
  <div class="input-group">
    {#if title}
      <label
        for={key}
        class:tooltip
        data-tooltip={tooltip.trim()}
        style={tooltip
          ? `--kills: ${tooltip.split(/\r\n|\r|\n/).length - 1};`
          : ""}
      >
        {title}
      </label>
    {/if}
    {#if type === "number"}
      <input
        bind:value
        onchange={change}
        oninput={change}
        type="number"
        id={key}
        class={`input input--${color}`}
        disabled={disabled}
      />
    {:else}
      <input
        bind:value
        onchange={change}
        oninput={change}
        id={key}
        class={`input input--${color}`}
        disabled={disabled}
      />
    {/if}
    {#if filepath}
      <div class="file-picker tooltip"  data-tooltip={`Select ${filetype ? filetype + " " : ""}${directory ? "Folder" : "File"}`}>
        <button onclick={openFilePicker} >
          <Fa icon={faFileImport} color={`var(--${color})`}/>
        </button>
      </div>
    {/if}
  </div>
{/if}

<style lang="scss">
  .input-group {
    position: relative;
  }

  .file-picker {
    position: absolute;
    bottom: 1px;
    right: 0;
    cursor: pointer;
    z-index: 9999;

    button {
      background: transparent;
      border: none;
    }
  }
</style>
