<script lang="ts">
  import Card from './lib/Card.svelte'
  import Deck from './lib/Deck.svelte';
  import init, {Moska} from "./lib/moska";

  let game: Moska | null = null;

  init().then(() => {
    game = new Moska(2);
    game.new_round();

    // game.player_action(1,0);
    // game.player_action(3,0);

    // game.player_action(1,0);
    // game.player_action(1,0);

    game = game;
  })

  function action(action: number, index: number){
    if (game) {
      game.player_action(action, index);
      game = game;
    }
  }
</script>

<main class="w-full p-6 flex justify-center">
  <div class="w-full lg:w-5/6">
    {#if game}

    <!-- deck and players -->
    <section class="flex">
      <!-- deck -->
      <Deck count={game.table.deck.count}/>
      <!-- opponent decks -->
      <div class="flex grow justify-end gap-2">
        {#each game.table.players as player, index}
          <Deck name={"Player " + (index+1)} count={player.cards.length}/>
        {/each}
      </div>
    </section>

    <!-- table cards -->
    <section class="divide-y divide-dashed">
      <!-- attacker cards -->
      <div class="flex grow justify-center border-black">
        {#each game.attacker_cards as card, index}
          <Card card={card} onclick={() => action(2, index)} />
        {/each}
      </div>

      <!-- defender cards -->
      <div class="flex grow justify-center border-black">
        {#each game.defender_cards as card, index}
          <Card card={card} onclick={() => action(2, index)} />
        {/each}
      </div>
    </section>

    <!-- player cards -->
    <section>
      <div class="flex flex-wrap justify-center border-black">
        {#each game.table.players[0].cards as card, index}
          <Card card={card} onclick={() => action(1, index)}/>
        {/each}
      </div>
    </section>
    {/if}
  </div>
</main>


<style global lang="postcss">
  @tailwind utilities;
  @tailwind components;
  @tailwind base;
</style>
