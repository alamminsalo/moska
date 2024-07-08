<script lang="ts">
  export let count = 0;

  let cardTransforms: any[] = []
  let deckOffset = 0;

  function refresh(maxOffset=14) {
    let _count = Math.min(Math.max(count-1,0), 6)
    let cards = [...Array(_count).keys()]
    
    cardTransforms = cards.map(index => ({
      offset: maxOffset / _count * index,
    }))

    deckOffset = -(cardTransforms.at(-1)?.offset ?? 0) / 2
  }

  $: count, refresh()
</script>

<span class="text-center"}>
  <span class="deck text-center px-4" style={`transform: translate(${deckOffset}px);`}>
    {#each cardTransforms as tr}
      <span class="card back" style={`transform: translate(${tr.offset}px);`}>🂠</span>
    {/each}
  </span>
  <div class="count">x{count}</div>
</span>

<style lang="scss">
  .deck {
    @apply relative flex items-center justify-center;
    width: 9rem;
    height: 9rem;
    .card {
      @apply absolute transition-all;
    }
  }
</style>


