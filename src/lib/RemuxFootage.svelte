<script>
    import { createEventDispatcher } from "svelte";
    import { faTimeline } from "@fortawesome/free-solid-svg-icons";
    import Modal from "./Modal.svelte";
    import Fa from "svelte-fa";
  
    const dispatch = createEventDispatcher();
  
    let enabled = false;
  
    function toggle() {
      enabled = !enabled;
    }
    
  </script>
  
  <button class="btn btn--tert" on:click={toggle} disabled>
    <Fa icon={faTimeline} color={`var(--tert)`} />
    Remux Footage
  </button>
  
  <Modal color="tert" {toggle} {enabled}>

  </Modal>
  
  <style lang="scss">
    .btn {
      display: flex;
      gap: 0.5rem;
    }
  </style>
  