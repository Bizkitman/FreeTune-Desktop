<script lang="ts">
  type Track = {id: string; title: string; subtitle: string; thumbnail: string};
  let query = '';
  let loading = false;
  let error = '';
  let tracks: Track[] = [];
  let selected: Track | null = null;
  async function search() {
    if (loading || !query.trim()) return;
    loading = true; error = ''; tracks = []; selected = null;
    try {
      const api = (window as any).__TAURI__;
      if (!api) throw new Error('Apri FreeTune come app desktop, non nel browser.');
      tracks = await api.core.invoke('search_tracks', {query: query.trim()});
    } catch (e) { error = e instanceof Error ? e.message : String(e); }
    finally { loading = false; }
  }
</script>

<div class="app">
  <aside><h1>FreeTune</h1><p>Cerca musica</p><small>YouTube Music<br />Integrazione sperimentale</small></aside>
  <main>
    <form on:submit|preventDefault={search}>
      <input aria-label="Cerca musica" placeholder="Cerca un brano o un artista…" bind:value={query} maxlength="200" />
      <button disabled={loading || !query.trim()}>{loading ? 'Ricerca…' : 'Cerca'}</button>
    </form>
    <h2>Ricerca YouTube Music</h2>
    <p class="muted">Risultati reali, senza account. Riproduzione audio non ancora collegata.</p>
    {#if loading}<p role="status">Connessione a YouTube Music…</p>{/if}
    {#if error}<p class="error" role="alert">{error}</p>{/if}
    <div class="grid">
      {#each tracks as track (track.id)}
        <button class="card" on:click={() => selected = track}>
          {#if track.thumbnail}<img src={track.thumbnail} alt="" loading="lazy" />{:else}<div class="cover"></div>{/if}
          <strong>{track.title}</strong><span>{track.subtitle}</span>
        </button>
      {/each}
    </div>
  </main>
  <footer><strong>{selected ? selected.title : 'Nessun brano selezionato'}</strong><span>{selected ? selected.subtitle : 'Cerca un artista e premi Invio.'}</span><small>Player non ancora disponibile</small></footer>
</div>

<style>
  :global(*){box-sizing:border-box}
  :global(body){margin:0;background:#0d1017;color:#e8eaed;font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',sans-serif}
  .app{display:grid;grid-template-columns:210px minmax(0,1fr);grid-template-rows:minmax(0,1fr) 100px;height:100vh}
  aside{padding:28px 20px;background:#12151d;border-right:1px solid #293140}h1{color:#65acff;margin:0 0 35px;font-size:25px}
  small,.muted,.card span,footer span{color:#aab2c0}small{line-height:1.6}
  main{padding:28px;overflow:auto;min-width:0}form{display:flex;gap:12px}
  input{flex:1;min-width:0;background:#161a23;border:1px solid #354055;border-radius:12px;color:white;padding:14px;font:inherit}
  button{font:inherit;cursor:pointer}form button{background:#65acff;border:0;border-radius:12px;padding:12px 24px;color:#071321}button:disabled{opacity:.5;cursor:wait}
  h2{margin-top:30px}.grid{display:grid;grid-template-columns:repeat(auto-fill,minmax(170px,1fr));gap:18px}
  .card{text-align:left;padding:12px;background:#161a23;color:white;border:1px solid #293140;border-radius:14px;display:flex;flex-direction:column;gap:9px;min-width:0}.card:hover{border-color:#65acff}
  .card img,.cover{width:100%;aspect-ratio:1;object-fit:cover;border-radius:9px;background:#25374a}.card span{font-size:13px}
  .error{padding:16px;background:#382026;border:1px solid #995563;border-radius:10px;overflow-wrap:anywhere}
  footer{grid-column:1/-1;padding:14px 28px;background:#12151d;border-top:1px solid #293140;display:flex;flex-direction:column;gap:5px;overflow:auto}footer span{font-size:13px}
  @media(max-width:700px){.app{grid-template-columns:1fr}aside{display:none}main{padding:18px}}
</style>
