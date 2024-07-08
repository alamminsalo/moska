<script lang="ts">
  export let count = 0;

  let maxCardsShown = 8;
  count = Math.min(count, maxCardsShown)

  let cardTransforms: any[] = []
  let deckOffset = 0;

  function refresh(maxOffset=18) {
    let cards = [...Array(count).keys()]
    
    cardTransforms = cards.map(index => ({
      offset: maxOffset / count * index,
    }))

    deckOffset = -(cardTransforms.at(-1)?.offset ?? 0) / 2
  }

  $: count, refresh()
</script>

<span class="text-center">
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


