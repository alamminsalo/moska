<div class="max-w-4xl mx-auto bg-white p-8 rounded-lg shadow-md">
      <slot/>
      <h1 class="text-3xl font-bold mb-4">Moska Rules</h1>
      
      <p class="mb-2"><span class="font-bold text-blue-600">Moska</span> is a card game played with a standard 52-card deck. The aim is to get rid of all your cards.</p>

      <h2 class="text-2xl font-semibold mt-6 mb-2">Setup</h2>
      <ul class="list-disc list-inside ml-4 mb-4">
          <li class="hidden"><span class="font-bold text-blue-600">Players:</span> 2-6</li>
          <li><span class="font-bold text-blue-600">Deck:</span> 52 cards</li>
          <li><span class="font-bold text-blue-600">Initial Hand:</span> Each player is dealt 6 cards</li>
          <li><span class="font-bold text-blue-600">Trump:</span> The bottom card of the remaining deck is revealed and placed under the deck, its suit becoming the trump suit.</li>
      </ul>

      <h2 class="text-2xl font-semibold mt-6 mb-2">Gameplay</h2>
      <p class="mb-2">Players take turns in a clockwise direction. During a turn, a player may either attack or defend.</p>

      <h3 class="text-xl font-semibold mt-4 mb-2">Attacking</h3>
      <ul class="list-disc list-inside ml-4 mb-4">
          <li>The attacker plays one or more cards of the same rank.</li>
          <li>These cards are placed in front of the defender.</li>
          <li>The attacker may add more cards of the same rank if they have them.</li>
      </ul>

      <h3 class="text-xl font-semibold mt-4 mb-2">Defending</h3>
      <ul class="list-disc list-inside ml-4 mb-4">
          <li>The defender must play a higher card of the same suit or any card of the trump suit to beat the attacking cards.</li>
          <li>If the defender cannot or does not wish to beat the cards, they must pick up all the attacking cards.</li>
      </ul>

      <h3 class="text-xl font-semibold mt-4 mb-2">End of Turn</h3>
      <p class="mb-2">The turn ends when the defender beats all attacking cards or picks them up.</p>
      <ul class="list-disc list-inside ml-4 mb-4">
          <li>If the defender successfully beats all the attacking cards, these cards are discarded from the game.</li>
          <li>If the defender picks up the cards, the turn passes to the next player.</li>
      </ul>

      <h2 class="text-2xl font-semibold mt-6 mb-2">Winning</h2>
      <p class="mb-2">The game continues until one player gets rid of all their cards. This player is the <span class="font-bold text-blue-600">winner</span>.</p>
      <p class="mb-2">If the deck runs out of cards, the game continues without drawing new cards.</p>

      <h2 class="text-2xl font-semibold mt-6 mb-2">Special Rules</h2>
      <ul class="list-disc list-inside ml-4 mb-4">
          <li><span class="font-bold text-blue-600">Drawing Cards:</span> After each turn, players draw cards from the deck to maintain a hand of 6 cards until the deck is empty.</li>
          <li><span class="font-bold text-blue-600">Trump Card Swap:</span> If player holds a trump card with rank 2, they can swap it with the trump card placed under the deck.</li>
          <li class="hidden"><span class="font-bold text-blue-600">Reinforcements:</span> After the initial attack, other players may also add cards of the same rank to the attack.</li>
      </ul>
  </div>
