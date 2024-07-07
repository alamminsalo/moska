<script lang="ts">
  import Deck from './Deck.svelte';
  import {Player} from './moska/moska';

  export let players: Player[] = [];
  export let current = 0;
</script>

<!-- player decks -->
<div class="flex grow justify-end gap-4">
  {#each players as player, index}
    <Deck name={"Player " + (index+1)} count={player.cards.length} active={index === current}/>
  {/each}
</div>

