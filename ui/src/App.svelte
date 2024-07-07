<script lang="ts">
  import Card from './lib/Card.svelte'
  import Deck from './lib/Deck.svelte';
  import Divider from './lib/Divider.svelte';
  import Players from './lib/Players.svelte';
  import init, {Moska} from "./lib/moska";

  let game: Moska | null = null;
 
  // human player index
  let humanPlayer = 0;

  let currentPlayer = 0;
  let previousPlayer = 0;
  let state = 0;

  init().then(() => {
    game = new Moska(2);
    game.new_round();
    game = game;

    console.log(game)
  })

  function action(action: number, index: number){
    if (game) {
      previousPlayer = currentPlayer;

      game.player_action(action, index);
      game = game;

      console.log(game)
    }
  }

  function reset() {
    if (game) {
      currentPlayer = 0;
      previousPlayer = 0;
      game.new_round();
      game = game;
    }
  }

  let reverseTable = false;

  // update state on game object changes
  $: game, (() => {
    if (game) {
      currentPlayer = game.table.player_index;
      state = game.state;

      //reverseTable = (state == 1 && currentPlayer == humanPlayer) || (state == 2 && previousPlayer == humanPlayer);

      console.log('updated')
    }
  })()
</script>

<main class="h-full w-full p-6 flex justify-center">
  {#if game}
  <!-- action buttons -->

  <div class="h-full w-full flex flex-col h-full lg:w-5/6 divide-y divide-dotted">

    <!-- deck and players -->
    <section class="flex justify-center items-center border-black pb-2 gap-6">
      <!-- drawing deck -->
      <Deck count={game.table.deck.count}/>

      <!-- trump card -->
      <Card card={game.trump_card} interactive={false} style="transform: rotate(90deg); z-index: -1; left: -6rem; top: -0.6rem;"/>

      <!-- player decks -->
      <Players players={game.table.players} current={game.table.player_index} />
    </section>

    <!-- table cards -->
    <section class="relative grow flex flex-col border-black" class:flex-col-reverse={reverseTable}>

      <!-- ok/take button -->
      <div class="absolute h-full w-full flex justify-center items-center">
        <button class="bg-green-700 px-4 z-50" on:click={() => action(3, 0)} class:invisible={!game.valid} disabled={!game.valid}>
          { game.state === 2 && game.defender_cards.length === 0 ? 'Take' : 'Ok'}
        </button>
      </div>

      <!-- attacker cards -->
      <div class="flex h-1/2 justify-center items-center border-black gap-3">
        {#each game.attacker_cards as card, index}
          <Card card={card} onclick={() => action(2, index)} />
        {/each}
      </div>

      <Divider/>

      <!-- defender cards -->
      <div class="flex h-1/2 justify-center items-center border-black gap-3">
        {#each game.defender_cards as card, index}
          <Card card={card} onclick={() => action(2, index)} />
        {/each}
      </div>
    </section>

    <section class="border-black pt-6 flex flex-col">
      <!-- player cards -->
      <div class="grow flex flex-wrap justify-center items-center border-black gap-3">
        {#each game.table.players[currentPlayer].cards as card, index}
          <Card card={card} onclick={() => action(1, index)}/>
        {/each}
      </div>
    </section>

  </div>
  {/if}
</main>
