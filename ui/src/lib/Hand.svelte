<script lang="ts">
  export let count = 0;
</script>

<div>
  <span class="hand">
    <span class="card">🂠</span>
    <span class="card" style="transform: rotate(-15deg);">🂠</span>
    <span class="card" style="transform: rotate(15);">🂠</span>
    <span class="card" style="transform: rotate(-30deg);">🂠</span>
    <span class="card" style="transform: rotate(30deg);">🂠</span>
  </span>
  x{count}
</div>

<style lang="scss">
    .hand{
        position: relative;
        width: 200px;
        height: 200px;
        display: flex;
        align-items: center;
        justify-content: center;

        .card {
          position: absolute;
          font-size: 9rem;
          transform-origin: bottom center;
          text-align: center;
          vertical-align: middle;
        }
    }
</style>

