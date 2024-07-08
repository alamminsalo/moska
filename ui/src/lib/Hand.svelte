<script lang="ts">
  export let count = 0;
  export let open = false;

  let maxCardsShown = 8;
  count = Math.min(count, maxCardsShown)

  let cardTransforms: any[] = []

  let handAngle = 0;
  let handOffset = 0;

  function refresh(maxAngle=25, maxOffset=14) {
    let cards = [...Array(count).keys()]
    
    cardTransforms = cards.map(index => ({
      angle: open ? maxAngle / count * index : 0,
      offset: maxOffset / count * index,
    }))

    handAngle = -(cardTransforms.at(-1)?.angle ?? 0) / 2
    handOffset = -(cardTransforms.at(-1)?.offset ?? 0) / 2
  }

  $: count, open, refresh()
</script>

<span>
  <span class="hand p-0 m-0" class:open={open} style={`transform: rotate(${handAngle}deg) translate(${handOffset}px);`}>
    {#each cardTransforms as tr}
      <span class="card back" style={`transform: rotate(${tr.angle}deg) translate(${tr.offset}px);`}>🂠</span>
    {/each}
  </span>
</span>

<style lang="scss">
    .hand{
      @apply relative flex items-center justify-center transition-all;
      width: 9rem;
      height: 9rem;

      &.open {
        width: 10rem;
      }

      .card {
        @apply absolute transition-all;
        transform-origin: bottom center;
      }
    }
</style>

