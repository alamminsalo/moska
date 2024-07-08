<script lang="ts">
  export let name = ""
  export let count = 0;
  export let active = false;
</script>

<span class="deck text-center px-4" class:active={active} class:empty={count < 1}>
  <!-- card backface, deck/player name -->
  <div class="name mb-1">{name}</div>
  <div class="card text-blue-700">🂠</div>
  <!-- deck count -->
  <div class="count">x{count}</div>
</span>

<style lang="scss">
  .deck > .card {
    box-shadow: 0.15rem 0.15rem #3333;
  }
  .deck.active > .name, .deck.active > .count  {
    @apply font-bold;
  }
  .empty {
    .card {
      @apply text-slate-400 border-slate-400;
      box-shadow: 0 0;
    }
  }
</style>


