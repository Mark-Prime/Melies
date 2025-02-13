<script>
  import "../app.css";
  /** @type {{children?: import('svelte').Snippet}} */
  let { children } = $props();
</script>

<main class="container">
  {@render children?.()}
</main>
