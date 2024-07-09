<script lang="ts">
  export let count = 0;
  export let open = false;

  let maxCardsShown = 6;
  count = Math.min(count, maxCardsShown)

  let cardTransforms: any[] = []

  let handAngle = 0;
  let handOffset = 0;

  function refresh(maxAngle=40, maxOffset=6) {
    let cards = [...Array(count).keys()]
    
    cardTransforms = cards.map(index => ({
      angle: (open ? maxAngle : 6) / count * index,
      offset: maxOffset / count * index,
    }))

    handAngle = -(cardTransforms.at(-1)?.angle ?? 0) / 2
    handOffset = -(cardTransforms.at(-1)?.offset ?? 0) / 2
  }

  $: count, open, refresh()
</script>

<span class="hand" class:open={open} style={`transform: rotate(${handAngle}deg) translate(${handOffset}px);`}>
  {#each cardTransforms as tr}
    <span class="card back" style={`transform: rotate(${tr.angle}deg) translate(${tr.offset}px);`}>🂠</span>
  {/each}
</span>

<style lang="scss">
    .hand{
      @apply flex items-center justify-center transition-all;
      width: 8rem;
      height: 8rem;

      &.open {
        width: 9rem;
      }

      .card {
        @apply absolute transition-all;
        transform-origin: bottom center;
      }
    }
</style>

