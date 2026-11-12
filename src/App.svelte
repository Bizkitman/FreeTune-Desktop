<script lang="ts">
  let query = '';
</script>

<main>
  <h1>FreeTune</h1>
  <p>Lightweight desktop music client.</p>
  <input placeholder="Search music..." bind:value={query} />
</main>

<style>
  main {
    display: grid;
    place-items: center;
    min-height: 100vh;
    font-family: system-ui, sans-serif;
    background: #0f1115;
    color: #e6e6e6;
  }
  input {
    margin-top: 1rem;
    padding: 0.75rem 1rem;
    border-radius: 10px;
    border: 1px solid #333;
    background: #171a21;
    color: inherit;
    width: min(420px, 80vw);
  }
</style>
