<script lang="ts">
  import {Card} from "./moska";
  export let card: Card;
  export let interactive: boolean = true;
  export let onclick = () => {};
</script>

<span class="card" class:interactive={interactive} on:click={onclick}>
  {card.unicode}
</span>

<style lang="scss">
  .card {
    @apply relative;
    cursor: pointer;
    font-size: 9rem;
    line-height: 7rem;
    padding-bottom: 1.2rem;
  }
  .interactive {
    &:hover {
      top: -10px;
    }
  }
</style>

