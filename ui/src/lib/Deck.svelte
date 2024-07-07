<script lang="ts">
  export let name = ""
  export let count = 0;
  export let active = false;
</script>

<span class="deck text-center px-4" class:active={active}>
  <!-- card backface, deck/player name -->
  <div class="name pb-1">{name}</div>
  <div class="card text-blue-700">🂠</div>
  <!-- deck count -->
  <div class="count">x{count}</div>
</span>

<style lang="scss">
  .deck.active > .name {
    @apply text-slate-200;
  }
</style>


