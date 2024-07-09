<script lang="ts">
  import {Card} from "./moska";
  export let card: Card;
  export let interactive: boolean = true;
  export let onclick = () => {};
  export let style = "";
  export let addClass = ""

  let red = false;

  $: card, (() => {
    red = ["Hearts", "Diamonds"].includes(card.suit);
  })()
</script>

<span class={"card " + addClass} class:interactive={interactive} class:red={red} style={style}
      on:click={() => interactive && onclick()}>
  {card.unicode}
</span>

<style lang="scss">
  .card {
    @apply relative cursor-pointer transition-all;

    &:not(.interactive) {
      @apply cursor-not-allowed;
    }
  }

  .interactive:not(.trump-card) {
    &:hover {
      transform: rotate(3deg) translate(0, -1rem);
      box-shadow: 0.4rem 0.8rem #3333;
    }
  }

  .interactive.trump-card {
    &:hover {
      transform: translate(-15%, 60%);
    }
  }

  .red {
    @apply text-red-700;
  }
</style>

