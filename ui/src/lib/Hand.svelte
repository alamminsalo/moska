<script lang="ts">
  export let count = 0;

  let maxCardsShown = 8;
  count = Math.min(count, maxCardsShown)

  let cardAngles: number[] = [];
  let handRotation = 0;

  function refresh(maxAngle=30) {
    cardAngles = [...Array(count).keys()].map(index => maxAngle / count * index)
    handRotation = -(cardAngles.at(-1) ?? 0) / 2
  }

  $: count, refresh()
</script>

<span>
  <span class="hand p-0 m-0" style={`transform: rotate(${handRotation}deg);`}>
    {#each cardAngles as angle}
      <span class="card back" style={`transform: rotate(${angle}deg);`}>🂠</span>
    {/each}
  </span>
</span>

<style lang="scss">
    .hand{
      @apply relative flex items-center justify-center transition-all;
      width: 9rem;
      height: 9rem;

      .card {
        @apply absolute transition-all;
        font-size: 9rem;
        transform-origin: bottom center;
        text-align: center;
        vertical-align: middle;
      }
    }
</style>

