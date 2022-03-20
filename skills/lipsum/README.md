# yozuk-skill-lipsum

A random placeholder text generator using the Markov chain.

## Examples

### Generate Lorem ipsum

```
lipsum
=> Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minima veniam, quis nostrud exercitation ullamco laboris nisi.
```

### Generate 120-words Lorem ipsum.

```
Lorem ipsum 120 words
=> Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magnam aliquam quaerat voluptatem. ut enim ad sapientiam perveniri potest, non paranda nobis solum ea, sed fruenda etiam sapientia est; sive hoc difficile est, tamen nec modus est neque honoris neque imperii nec libidinum nec epularum nec reliquarum cupiditatum, quas nulla praeda umquam improbe parta minuit, sed potius inflammat, ut coercendi magis quam dedocendi esse videantur. Invitat igitur vera ratio bene sanos in viam placatae, tranquillae, quietae, beatae vitae dicta sunt, explicabo. nemo enim ipsam voluptatem, quia voluptas sit, aspernatur aut odit aut fugit, sed quia consequuntur magni dolores eos, qui ratione voluptatem sequi nesciunt, neque porro quisquam est, qui Ennii Medeam aut.
```

## Options

| Key | Type | Default | Description |
| - | - | - | - |
| `custom_text` | `string` | `null` | Custom placeholder text used for the Markov chain input. |
