# Choose your parser combinator

## char

- [match_char]()
- [match_char_range]()
- [match_char_set]()

You can also expand to `or partten`, but can be very slow

## string

- [match_str]()
- [match_str_if]()
- [match_str_until]()
- [match_regex]()

You can also expand to `or partten`, but it might be a little slow

## maybe

- [match_maybe]()

## many

- [match_repeats]()
- [match_many0]()

There's no combinator for `a+`, `a+` is recommended to expand to `a a*`

## string literal

- [match_literal]()
- [match_literal_if]()