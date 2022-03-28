# yozuk-skill-dice

A simple dice roller using the [dice notation](https://en.wikipedia.org/wiki/Dice_notation).

## Examples

### Roll two six-sided dice

The result will be returned as `<sum> (<1st> <2nd> ..)`.

```
2d6
=> 10 (4 6)
```

### Roll three 1000-sided dice, add them together and multiply the result by 2.

```
3d1000 * 2
=> 2238
```

## Options

| Key | Type | Default | Description |
| - | - | - | - |
| `secure` | `boolean` | `false` | Use a cryptographically-secure PRNG for dice rolls. |
