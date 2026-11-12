<script lang="ts">
  let query = '';
  let playing = false;
  let progress = 0;
  let volume = 70;

  const results = Array.from({ length: 12 }, (_, i) => ({
    id: i + 1,
    title: `Track ${i + 1}`,
    artist: `Artist ${i + 1}`,
    duration: '3:24'
  }));

  function togglePlay() {
    playing = !playing;
  }
</script>

<div class="app">
  <aside class="sidebar">
    <div class="brand">FreeTune</div>
    <nav>
      <button class="active">Home</button>
      <button>Search</button>
      <button>Library</button>
      <button>Settings</button>
    </nav>
  </aside>

  <main>
    <header>
      <input
        type="text"
        placeholder="Search songs, artists or albums..."
        bind:value={query}
      />
    </header>

    <section class="content">
      <h2>Discover</h2>
      <div class="grid">
        {#each results as track (track.id)}
          <article class="card">
            <div class="cover"></div>
            <h3>{track.title}</h3>
            <p>{track.artist}</p>
            <span>{track.duration}</span>
          </article>
        {/each}
      </div>
    </section>
  </main>

  <footer class="player">
    <div class="track">
      <div class="mini-cover"></div>
      <div>
        <strong>Track 1</strong>
        <p>Artist 1</p>
      </div>
    </div>

    <div class="controls">
      <button aria-label="Previous">⏮</button>
      <button class="play" on:click={togglePlay}>
        {playing ? '⏸' : '▶'}
      </button>
      <button aria-label="Next">⏭</button>
    </div>

    <div class="progress">
      <input type="range" min="0" max="100" bind:value={progress} />
    </div>

    <div class="volume">
      <span>🔊</span>
      <input type="range" min="0" max="100" bind:value={volume} />
    </div>
  </footer>
</div>

<style>
  :global(html, body) {
    margin: 0;
    height: 100%;
    background: #0d1017;
    color: #e8eaed;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
  }

  .app {
    display: grid;
    grid-template-columns: 240px 1fr;
    grid-template-rows: 1fr 84px;
    height: 100vh;
    overflow: hidden;
  }

  .sidebar {
    grid-row: 1 / 3;
    background: #12151d;
    border-right: 1px solid #232833;
    padding: 24px 16px;
  }

  .brand {
    font-size: 22px;
    font-weight: 700;
    color: #4c9dff;
    margin-bottom: 32px;
  }

  nav {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  nav button {
    text-align: left;
    padding: 12px 14px;
    border: 0;
    border-radius: 10px;
    background: transparent;
    color: #aab2c0;
    font-size: 15px;
    cursor: pointer;
  }

  nav button:hover {
    background: #1b202b;
    color: #fff;
  }

  nav button.active {
    background: #1d2735;
    color: #fff;
  }

  main {
    overflow-y: auto;
    padding: 24px 32px;
  }

  header input {
    width: 100%;
    padding: 14px 18px;
    border-radius: 12px;
    border: 1px solid #2b3240;
    background: #161a23;
    color: #e8eaed;
    font-size: 15px;
    outline: none;
  }

  h2 {
    margin: 28px 0 18px;
    font-size: 22px;
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(170px, 1fr));
    gap: 20px;
  }

  .card {
    background: #161a23;
    border: 1px solid #232833;
    border-radius: 14px;
    padding: 14px;
  }

  .cover {
    aspect-ratio: 1;
    border-radius: 10px;
    background: linear-gradient(135deg, #294b73, #101623);
    margin-bottom: 12px;
  }

  .card h3 {
    margin: 0;
    font-size: 15px;
  }

  .card p {
    margin: 4px 0 8px;
    color: #9aa4b2;
    font-size: 13px;
  }

  .card span {
    color: #7d8794;
    font-size: 12px;
  }

  .player {
    grid-column: 2;
    display: grid;
    grid-template-columns: 220px 1fr 180px;
    align-items: center;
    gap: 20px;
    padding: 0 24px;
    background: #12151d;
    border-top: 1px solid #232833;
  }

  .track {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .mini-cover {
    width: 48px;
    height: 48px;
    border-radius: 8px;
    background: linear-gradient(135deg, #294b73, #101623);
  }

  .track p {
    margin: 2px 0 0;
    color: #9aa4b2;
    font-size: 12px;
  }

  .controls {
    display: flex;
    justify-content: center;
    gap: 14px;
  }

  .controls button {
    width: 40px;
    height: 40px;
    border: 0;
    border-radius: 50%;
    background: #1d2735;
    color: #e8eaed;
    font-size: 15px;
    cursor: pointer;
  }

  .controls .play {
    background: #4c9dff;
    color: #06121f;
  }

  .progress input,
  .volume input {
    width: 100%;
    accent-color: #4c9dff;
  }

  .volume {
    display: flex;
    align-items: center;
    gap: 10px;
  }
</style>
