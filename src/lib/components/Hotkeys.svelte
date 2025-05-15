<script>

  let {
    keyStates = $bindable({})
  } = $props();

  function on_key_event(event) {
    if (event.repeat) return;

    keyStates[event.key] = event.type === "keydown";

    switch (event.key) {
      case "Shift":
        event.preventDefault();
        break;
    }
  }
</script>

<svelte:window onkeydown={on_key_event} onkeyup={on_key_event} />