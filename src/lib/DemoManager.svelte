<script>
  import { createEventDispatcher } from "svelte";
  import { faFileCircleMinus, faFileCirclePlus, faListCheck, faSquare, faSquareCheck, faTrash } from "@fortawesome/free-solid-svg-icons";
  import Modal from "./Modal.svelte";
  import Fa from "svelte-fa";
  import { invoke } from "@tauri-apps/api/tauri";
  import dayjs from "dayjs";
  import tickToTime from "$lib/composables/tickToTime.js";
  import { faCheck, faXmark } from "@fortawesome/free-solid-svg-icons";
  import Input from "./Input.svelte";
  import Select from "./Select.svelte";

  let enabled = false;
  let resp = { loaded: false };
  let mapList = [];

  let search_filter = "";
  let map_filter = "";
  let vdm_filter = null;

  $: {
    let filters = {map: map_filter, search: search_filter, vdm: vdm_filter};
      
    resp = resp;
  }

  async function loadDemos() {
    try {
      resp = await invoke("load_demos");
      let collator = new Intl.Collator(undefined, {numeric: true, sensitivity: 'base'});

      mapList = [...new Set(resp.demos.map((demo) => demo.header.map))].sort((a, b) => collator.compare(a, b));
    } catch (error) {
      alert(error);
    }
  }

  function toggle() {
    enabled = !enabled;
  }

  async function delete_vdm(fileName) {
    await invoke("delete_vdm", { fileName: fileName })

    loadDemos();
  }

  async function create_vdm(fileName) {
    await invoke("create_vdm", { fileName: fileName })

    loadDemos();
  }

  async function delete_demo(fileName, hasVdm) {
    let answer = confirm(`Are you sure you want to delete this demo?${hasVdm ? `\nThis will also deleted the associated vdm.` : ``}`);

    answer.then(async (resp) => {
      if (resp) {
        await invoke("delete_demo", { fileName: fileName })

        loadDemos();
      }
    });
  }

  function filter(demo) {
    if (map_filter && demo.header.map != map_filter) {
      return false;
    }

    console.log(vdm_filter, demo.hasVdm)

    if (vdm_filter != null && vdm_filter != demo.hasVdm) {
      return false;
    }

    let search = search_filter.toLowerCase();

    let isInName = demo.name.toLowerCase().includes(search);
    let isInNick = demo.header.nick.toLowerCase().includes(search);
    let isInServer = demo.header.server.toLowerCase().includes(search);
    let isInMap = demo.header.map.toLowerCase().includes(search);

    return isInName || isInNick || isInServer || isInMap;
  }
</script>

<button class="btn btn--sec" on:click={toggle}>
  <Fa icon={faListCheck} color={`var(--sec)`} />
  Demo Manager
</button>

<Modal color="sec" {toggle} {enabled} large tall on:open={loadDemos}>
  <h1>Demo Manager</h1>
  {#if resp.loaded}
    <h4>Filters</h4>
    <div class="filters">
      <Input title="Search" color="sec" bind:value={search_filter}/>
      <Select title="Map" color="sec" bind:value={map_filter}>
        <option value=""></option>
        {#each mapList as map}
          <option value="{map}">{map}</option>
        {/each}
      </Select>
      <Select title="VDM Status" color="sec" bind:value={vdm_filter}>
        <option value={null}></option>
        <option value={true}>Has VDM</option>
        <option value={false}>Doesn't have VDM</option>
      </Select>
    </div>
    <table>
      <thead>
        <tr>
          <th>Name</th>
          <th>Player Nickname</th>
          <th>Length</th>
          <th>Server</th>
          <th>Map</th>
          <th>Created Date</th>
          <th
            class="tooltip tooltip--left"
            data-tooltip={`Does the demo have a vdm?`}
            style="--kills: 0;"
          >
            VDM
          </th>
          <th>Actions</th>
        </tr>
      </thead>
      <tbody>
        {#each resp.demos.filter(filter) as demo}
          <tr class={"table_row " + (demo.hasVdm && "demo--hasvdm")}>
            <td id="{demo.name}">
              {demo.name}
            </td>
            <td>{demo.header.nick}</td>
            <td>{tickToTime(demo.header.ticks)}</td>
            <td>{demo.header.server}</td>
            <td>{demo.header.map}</td>
            <td>{dayjs.unix(demo.metadata.created.secs_since_epoch).format('MMM DD, YYYY')}</td>
            <td class="table__has-vdm">
              {#if demo.hasVdm}
              <span
                class="tooltip tooltip--left"
                data-tooltip={`This demo has a VDM.`}
                style="--kills: 0;"
              >
                <Fa 
                  icon={faCheck}
                  color={`var(--sec)`}
                />
              </span>
              {:else}
                <span
                  class="tooltip tooltip--left"
                  data-tooltip={`This demo does not have a VDM.`}
                  style="--kills: 0;"
                >
                  <Fa icon={faXmark} color={`var(--tert)`} />
                </span>
              {/if}
            </td>
            <td>
              <!-- <a
                name="#{demo.name}-select"
                class="icon"
                on:click={() => demo.selected = !demo.selected}
                on:keydown={() => demo.selected = !demo.selected}
                tabindex="-1"
                role="button"
              >
                {#if demo.selected}
                  <Fa icon={faSquareCheck} color={`var(--pri)`} />
                {:else}
                  <Fa icon={faSquare} color={demo.hasVdm ? `var(--sec)` : `var(--tert)`} />
                {/if}
              </a> -->
              {#if demo.hasVdm}
                <a
                  name="#{demo.name}-select"
                  class="icon checkbox tooltip tooltip--left"
                  data-tooltip={`Delete VDM.`}
                  style="--kills: 0;"
                  on:click={async () => await delete_vdm(demo.name)}
                  on:keydown={async () => await delete_vdm(demo.name)}
                  tabindex="-1"
                  role="button"
                >
                  <Fa icon={faFileCircleMinus} color={`var(--err)`} />
                </a>
              {:else}
                <a
                  name="#{demo.name}-select"
                  class="icon checkbox tooltip tooltip--left"
                  data-tooltip={`Create blank VDM.`}
                  style="--kills: 0;"
                  on:click={async () => await create_vdm(demo.name)}
                  on:keydown={async () => await create_vdm(demo.name)}
                  tabindex="-1"
                  role="button"
                >
                  <Fa icon={faFileCirclePlus} color="var(--pri)" />
                </a>
              {/if}
              <a
                name="#{demo.name}-select"
                class="icon checkbox tooltip tooltip--left"
                data-tooltip={`Delete this demo.`}
                style="--kills: 0;"
                on:click={async () => await delete_demo(demo.name, demo.hasVdm)}
                on:keydown={async () => await delete_demo(demo.namel, demo.hasVdm)}
                tabindex="-1"
                role="button"
              >
                <Fa icon={faTrash} color={`var(--err)`} />
              </a>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  {/if}
</Modal>
  
<style lang="scss">
  h4 {
    margin: 0;
  }

  .filters {
    display: flex;
    gap: 1rem;
    padding: 0;
    margin: 0 0 1rem 0;
  }

  .icon {
    font-size: large;
    cursor: pointer;
  }

  .tooltip--left::before {
    left: auto;
    right: calc(100% - 2rem);
  }
  
  .tooltip--left::after {
    left: auto;
    right: calc(100% - .8rem);
  }

  .btn {
    display: flex;
    gap: 0.5rem;
  }

  table {
    border-collapse: separate;
    border-spacing: 0 2px;
    width: 100%;
  }

  th {
    text-align: left;
    white-space: nowrap;
  }

  .table_row {
    font-size: small;
    font-family: "Source Code Pro", monospace;
    color: var(--tert-con-text);
    padding: 0 1rem;
    margin: 2px 0;
    border-radius: 5px;

    transition: all 0.2s;

    background-color: var(--bg2);

    &.demo--hasvdm .tooltip:hover {
      color: var(--sec);
    }

    &:not(.demo--hasvdm) .tooltip:hover {
      color: var(--tert);
    }

    &.demo--hasvdm > td {
      color: var(--sec-con-text);
      border-color: var(--sec-con);
    }

    &:hover {
      &:not(.demo--hasvdm) {
        filter: drop-shadow(0px 0px 3px var(--tert));
      }

      &.demo--hasvdm {
        filter: drop-shadow(0px 0px 3px var(--sec));
      }

      & > td {
        border-color: var(--tert-con-text);
      }

      &.demo--hasvdm > td {
        border-color: var(--sec-con-text);
      }
    }

    & > td {
      text-align: left;
      white-space: nowrap;
      border: 1px solid var(--tert-con);
      border-left-width: 0px;
      border-right-width: 0px;
      padding: 0.3rem 0.25rem;

      &:first-of-type {
        border-left-width: 1px;
        border-radius: 5px 0 0 5px;
        padding-left: 0.5rem;
      }

      &:last-of-type {
        border-right-width: 1px;
        border-radius: 0 5px 5px 0;
        padding-right: 0.5rem;
      }
    }
  }
</style>
