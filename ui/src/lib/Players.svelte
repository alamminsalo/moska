<script lang="ts">
  import Hand from './Hand.svelte';
  import {Player} from './moska/moska';

  export let players: Player[];
  export let current: Player | null;
</script>

<!-- player hands -->
<div class="flex grow justify-end gap-2">
  {#each players as player}
    <div class="text-center pr-4">
      <div class:active={current?.id == player.id}>{"Player " + (player.id+1)}</div>
      <Hand count={player.cards.length} open={current?.id == player.id}/>
      <div>x{player.cards.length}</div>
    </div>
  {/each}
</div>

<style lang="scss">
  .active {
    @apply font-bold;
  }
</style>

