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
    font-size: 9rem;
    text-align: center;
    vertical-align: middle;
    padding-bottom: 1rem;
    cursor: pointer;
  }
  .interactive {
    &:hover {
      top: -10px;
    }
  }
</style>

