<script lang="ts">
  import {Card} from "./moska";
  export let card: Card;
  export let interactive: boolean = true;
  export let onclick = () => {};

  export let style = "";

  let red = false;

  $: card, (() => {
    red = ["Hearts", "Diamonds"].includes(card.suit);
  })()
</script>

<span class="card" class:interactive={interactive} style={style} on:click={onclick} class:red={red}>
  {card.unicode}
</span>

<style lang="scss">
  .card {
    @apply relative cursor-pointer;
  }
  .interactive {
    &:hover {
      top: -1rem;
      transform: rotate(3deg);
      z-index: 999;
      box-shadow: 0.4rem 0.8rem #3333;
    }
  }
  .red {
    @apply text-red-700;
  }
</style>

