# Cheating at Wordle

This script computes heuristics for the best [Wordle][wordle] words to play,
both singly and in pairs. The heuristics are rather simple: they bias 
towards words that contain the most common English letters, and seek to 
maximize information as early as possible in the game. 

This also computes the best _pairs_ of words to play early in the game, 
which aren't necessarily related to each other in any way. In fact, information 
transfer is better if the words don't share any letters, and so the heuristics 
favor words which do not overlap in letter content.

Running this requires a dictionary of words, which on my system is at
`/usr/share/dict/american-english`.

If you're just trying to cheat at Wordle, the current output of this script is [here][output].

## Future work
- [ ] make an interactive mode which tells you the best _next_ word to play
- [ ] fully script a Wordle AI

[output]: https://gist.github.com/wittrock/e6f26850290e01ffd0f09d1c1f0fb353
[wordle]: https://www.powerlanguage.co.uk/wordle/