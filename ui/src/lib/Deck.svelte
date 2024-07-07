<script lang="ts">
  export let name = ""
  export let count = 0;
</script>

<div class="container">
  <!-- card backface, deck/player name -->
  <div>{name}</div>
  <div class="card">🂠</div>
  <!-- deck count -->
  <div>x{count}</div>
</div>

<style lang="scss">
  .container {
    @apply text-center;
  }
  .card {
    font-size: 9rem;
    line-height: 7rem;
    padding-bottom: 1.2rem;
  }
</style>


