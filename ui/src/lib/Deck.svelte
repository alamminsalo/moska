<script lang="ts">
  export let name = ""
  export let count = 0;
  export let active = false;
</script>

<span class="text-center" class:active={active}>
  <!-- card backface, deck/player name -->
  <div>{name}</div>
  <div class="card">🂠</div>
  <!-- deck count -->
  <div>x{count}</div>
</span>

<style lang="scss">
  .card {
    font-size: 9rem;
    line-height: 7rem;
    padding-bottom: 1.2rem;
  }

  .active {
    top: -10px;
  }
</style>


