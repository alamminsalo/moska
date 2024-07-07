<script lang="ts">
  import {Card} from "./moska";
  export let card: Card;
  export let interactive: boolean = true;
  export let onclick = () => {};

  export let style = "";

  let red = ["Hearts", "Diamonds"].includes(card.suit);
</script>

<span class="card" class:interactive={interactive} style={style} on:click={onclick} class:red={red}>
  {card.unicode}
</span>

<style lang="scss">
  .card {
    @apply relative;
    cursor: pointer;
    rotate: inherit;
  }
  .interactive {
    &:hover {
      top: -1rem;
      transform: rotate(3deg);
      z-index: 999;
    }
  }
  .red {
    @apply text-red-700;
  }
</style>

