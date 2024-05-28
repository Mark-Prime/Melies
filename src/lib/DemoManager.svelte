<script>
    import { createEventDispatcher } from "svelte";
    import { faListCheck } from "@fortawesome/free-solid-svg-icons";
    import Modal from "./Modal.svelte";
    import Fa from "svelte-fa";
  
    const dispatch = createEventDispatcher();
  
    let enabled = false;
  
    function toggle() {
      enabled = !enabled;
  
      if (enabled) {
        loadEvents();
      }
    }
    
  </script>
  
  <button class="btn btn--sec" on:click={toggle} disabled>
    <Fa icon={faListCheck} color={`var(--sec)`} />
    Demo Manager
  </button>
  
  <Modal color="sec" {toggle} {enabled}>

  </Modal>
  
  <style lang="scss">
    .btn {
      display: flex;
      gap: 0.5rem;
    }
  </style>
  