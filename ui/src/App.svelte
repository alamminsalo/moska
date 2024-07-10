<script lang="ts">
  import Card from './lib/Card.svelte'
  import Deck from './lib/Deck.svelte';
  import Divider from './lib/Divider.svelte';
  import Menu from './lib/Menu.svelte';
  import Players from './lib/Players.svelte';
  import init, {Moska} from "./lib/moska";
  import type {Player} from './lib/moska/moska';
  import {State} from './lib/moska/moska';

  let game: Moska | null = null;
  let numberOfPlayers = 2;

  // player pointers
  let players: Player[] = []
  let humanPlayer: Player | null = null;
  let currentPlayer: Player | null = null;
  let statusText = getStatusText()

  function newGame() {
    if (game) {
      game.free();
      game = null;
    }

    // Triggers animations
    setTimeout(() => {
      game = new Moska(numberOfPlayers);
      game.new_round();

      humanPlayer = game.table.players[0];
      game = game;
    }, 0)
  }

  function action(action: number, index: number){
    if (game) {
      game.player_action(action, index);
      game = game;

      console.log(game)
    }
  }

  function getStatusText() {
    if (game?.state == State.GameOver) {
      return "Game over";
    }

    if (game?.state == State.PlayerAttacking) {
      return `Player ${currentPlayer?.id + 1} attacking`;
    }

    if (game?.state == State.PlayerDefending) {
      return `Player ${currentPlayer?.id + 1} defending`;
    }

    return "";
  }

  // WASM initialize
  init().then(newGame);

  // update state on game object changes
  $: game, (() => {
    if (game) {
      players = game.table.players;
      currentPlayer = game.table.players[game.table.player_index];
      statusText = getStatusText()

      console.log('Game updated:', game)
      console.log('Current player:', currentPlayer)
    }
  })()
</script>

<main class="h-full w-full p-4 flex flex-col items-center">
  {#if game}
  <!-- top menu -->
  <Menu>
    <b>Moska</b>
    <a class="dice text-2xl" title="New Game" on:click={newGame}><i class="ph-bold ph-dice-five"/></a>
    <span class="grow text-end font-bold italic">
      {statusText}
      <a class="text-xl"><i class="ph-bold ph-question"/></a>
    </span>
  </Menu>

  <div class="h-full w-full flex flex-col divide-y py-2">

    <!-- deck and players -->
    <section class="w-full flex justify-center items-center pb-2">
        <!-- drawing deck -->
        <Deck count={game.table.deck.count}/>

        <!-- show trump card below deck -->
        {#if game.table.deck.count > 0}
          <Card addClass="trump-card" card={game.trump_card} onclick={() => action(4, 0)}/>
        {/if}

        <!-- player decks -->
        <Players players={players} current={currentPlayer} />
    </section>

    <!-- table cards -->
    <section class="relative grow flex flex-col border-black">

      <!-- attacker cards -->
      <div class="flex h-1/2 justify-center items-center gap-2">
        {#each game.attacker_cards as card, index}
          <Card card={card} interactive={game.state == State.PlayerAttacking} onclick={() => action(2, index)} />
        {/each}
      </div>

      <Divider/>

      <!-- defender cards -->
      <div class="flex h-1/2 justify-center items-center gap-2">
        {#each game.defender_cards as card, index}
          <Card card={card} interactive={game.state == State.PlayerDefending} onclick={() => action(2, index)} />
        {/each}
      </div>

      <!-- ok/take button -->
      <span class="absolute self-center top-1/2 -translate-y-1/2">
        {#if game.valid}
          {#if game.state === State.PlayerDefending && game.defender_cards.length === 0}
            <!-- Take button -->
            <button class="danger" on:click={() => action(3, 0)} title="Withdraw cards">
              <i class="ph-bold ph-hand"/> 
            </button>
            {:else}
              <!-- Ok button -->
              <button class="success" on:click={() => action(3, 0)} title="Submit cards">
                <i class="ph-bold ph-check"/> 
              </button>
            {/if}
          {:else}
            <!-- Invalid sign -->
            <button class="" on:click={() => action(3, 0)} disabled title="Invalid cards">
              <i class="ph-bold ph-prohibit"/> 
            </button>
        {/if}
      </span>

    </section>

    <section class="pt-6 flex flex-col">
      {#if currentPlayer}
        <!-- player cards -->
        <div class="player-hand grow flex justify-center items-center border-black gap-2">
          {#each currentPlayer.cards as card, index}
            <Card card={card} onclick={() => action(1, index)}/>
          {/each}
        </div>
      {/if}
    </section>

  </div>
  {/if}
</main>

<style lang="scss">
  .dice {
    @apply transition-all;
    transform: rotate(30deg);

    &:hover {
      transform: rotate(360deg);
    }
  }
  
  .player-hand {
    animation: 0.4s playerHandSlide;
  }

  section {
    @apply border-slate-400;
  }
</style>
