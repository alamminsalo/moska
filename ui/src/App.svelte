<script lang="ts">
  import Card from './lib/Card.svelte'
  import Deck from './lib/Deck.svelte';
  import Players from './lib/Players.svelte';
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

<main class="h-full w-full p-6 flex justify-center">
  <div class="h-full w-full flex flex-col h-full lg:w-5/6 divide-y divide-dotted">
    {#if game}

    <!-- deck and players -->
    <section class="flex items-end border-black">
      <!-- drawing deck -->
      <Deck count={game.table.deck.count}/>

      <!-- player decks -->
      <Players players={game.table.players} current={game.table.player_index} />
    </section>

    <!-- table cards -->
    <section class="grow flex flex-col divide-y divide-dashed border-black">
      <!-- attacker cards -->
      <div class="flex h-1/2 justify-center items-center border-black">
        {#each game.attacker_cards as card, index}
          <Card card={card} onclick={() => action(2, index)} />
        {/each}
      </div>

      <!-- defender cards -->
      <div class="flex h-1/2 justify-center items-center border-black">
        {#each game.defender_cards as card, index}
          <Card card={card} onclick={() => action(2, index)} />
        {/each}
      </div>
    </section>

    <!-- player cards -->
    <section class="border-black">
      <div class="flex flex-wrap justify-center border-black">
        {#each game.table.players[0].cards as card, index}
          <Card card={card} onclick={() => action(1, index)}/>
        {/each}
      </div>
    </section>
    {/if}
  </div>
</main>
