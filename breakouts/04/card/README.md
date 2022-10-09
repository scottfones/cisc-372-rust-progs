# Breakout 04: Card 

## `card.c`

Write a program `card.c` that defines a structured type for a playing
`Card`. A `Card` has a "value" (1..13) and a "suit" (diamonds, clubs,
hearts, or spades). The suit should be defined as an `enum`.

### `print_card()`

Write a function `print_card` which prints a card like this: "Ace of
spades", "Two of hearts", "King of diamonds", etc. Note that Ace=1,
Jack=11, Queen=12, King=13.

### `make_deck()`

Write a function `make_deck` which creates an array of cards of length
52, and fills it in with the 52 different possible cards.

### `print_deck()`

Write a function `print_deck` which prints the deck, in order.

### `shuffle()`

Write a function `shuffle` which shuffles the deck randomly.

### `destroy_deck()`

Write a function `destroy_deck` which destroys a deck.

### `main()`

Write a `main` function which creates a deck, prints it, shuffles it,
prints it again, and destroys it.

Write a Makefile to compile, test, and clean.

