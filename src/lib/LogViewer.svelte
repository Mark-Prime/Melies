<script>
  import { faCopy } from '@fortawesome/free-solid-svg-icons';
  import Fa from "svelte-fa";
  import { writeText } from '@tauri-apps/plugin-clipboard-manager';

  export let resp;

  let copyText = "";

  function copy() {
    writeText(`playdemo ${resp.first_demo};`)
    copyText = "Copied to clipboard";
    setTimeout(() => {
      copyText = "";
    }, 1000);
  }
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
  {#if resp.first_demo}
    <p>
      To start the automation, load the first demo by opening tf2 and using the command:
      <br />
      <button
        class="tick"
        on:click={() => copy()}
      >
        {#if copyText}
          <code>Copied to clipboard</code>
        {:else}
          <code>
            playdemo {resp.first_demo};
          </code>
          <Fa icon={faCopy} />
        {/if}
      </button>
    </p>
  {/if}
</div>

<style lang="scss">
  .tick {
    all: unset;
    position: relative;
    cursor: pointer;
    background-color: var(--bg);
    width: 100%;
    padding: 1rem 0;
  }

  code {
    font-family: "Source Code Pro", monospace;
    padding: 0 1rem;
  }

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
