<link
  href="https://fonts.googleapis.com/css2?family=Briem+Hand:wght@100..900&display=swap"
  rel="stylesheet"
/>
<div class="hero">
  <img src="/MeliesColor.svg" alt="Melies Logo" />
  <div>
    <h1>Méliès</h1>
    <h2>TF2 Recording in the <span>Space Age</span></h2>
  </div>
</div>

<style lang="scss">
  .hero {
    display: flex;
    align-items: center;
    justify-content: flex-start;

    grid-column: span 3;

    flex-grow: 0;

    & img {
      height: 100%;
      max-height: 50px;
      margin-right: 1rem;
    }

    & h1,
    h2 {
      text-align: left;
      line-height: 1;
      padding: 0;
      margin: 0;
    }

    & h1 {
      font-family: "Briem Hand", cursive;
      margin-bottom: 0.2rem;
      font-size: 2.5rem;

      color: var(--pri);
    }

    & h2 {
      color: var(--sec);
      font-size: 1rem;
    }

    & span {
      color: var(--tert);
    }
  }
</style>
