combinator selection tutorial, choose your parser combinator


## character

- [match_char](https://docs.rs/pex/latest/pex/struct.ParseState.html#method.match_char)
- [match_any](https://docs.rs/pex/latest/pex/struct.ParseState.html#method.match_char)
- [match_char_range](https://docs.rs/pex/latest/pex/struct.ParseState.html#method.match_char_range)
- [match_char_if](https://docs.rs/pex/latest/pex/struct.ParseState.html#method.match_char_if)
- [match_char_set](https://docs.rs/pex/latest/pex/struct.ParseState.html#method.match_char_set)

`match_char_set` using the [TrieSetSlice](https://docs.rs/pex/latest/pex/struct.TrieSetSlice.html) to match char set, it's very fast.

You can also expand to `or partten`, but can be very slow

## string

- [match_str](https://docs.rs/pex/latest/pex/struct.ParseState.html#method.match_str)
- [match_str_if](https://docs.rs/pex/latest/pex/struct.ParseState.html#method.match_str_if)
- [match_str_until](https://docs.rs/pex/latest/pex/struct.ParseState.html#method.match_str_until)
- [match_regex]()

You can also expand to `or partten`, but it might be a little slow

## many

- [match_repeats](https://docs.rs/pex/latest/pex/struct.ParseState.html#method.match_repeats)
- [match_repeats_m_n](https://docs.rs/pex/latest/pex/struct.ParseState.html#method.match_repeat_m_n)

There's no combinator for `a+`, `a+` is recommended to expand to `a a*`


## maybe

- [match_optional](https://docs.rs/pex/latest/pex/struct.ParseState.html#method.match_optional)
- [skip](https://docs.rs/pex/latest/pex/struct.ParseState.html#method.skip)

## choice

- [begin_choice](https://docs.rs/pex/latest/pex/struct.ParseState.html#method.begin_choice)
- [try_match](https://docs.rs/pex/latest/pex/struct.ChoiceHelper.html#method.or_else)
- [end_choice](https://docs.rs/pex/latest/pex/struct.ChoiceHelper.html#method.end_choice)

See more in
[ChoiceHelper](https://docs.rs/pex/latest/pex/struct.ChoiceHelper.html)

## peek

- [match_positive](https://docs.rs/pex/latest/pex/struct.ParseState.html#method.match_positive)
- [match_negative](https://docs.rs/pex/latest/pex/struct.ParseState.html#method.match_negative)

## whitespace

- [skip_whitespace](https://docs.rs/pex/latest/pex/struct.ParseState.html#method.skip_whitespace)
- [skip_whitespace_if](https://docs.rs/pex/latest/pex/struct.ParseState.html#method.skip_whitespace_if)

## comment

- [comment_line](https://docs.rs/pex/latest/pex/helpers/fn.comment_line.html)
- [comment_block](https://docs.rs/pex/latest/pex/helpers/fn.comment_block.html)
- [comment_block_nested](https://docs.rs/pex/latest/pex/helpers/fn.comment_block_nested.html)

## string literal

- [double_quote_string](https://docs.rs/pex/latest/pex/helpers/fn.double_quote_string.html)
- [single_quote_string](https://docs.rs/pex/latest/pex/helpers/fn.single_quote_string.html)
- [surround_pair](https://docs.rs/pex/latest/pex/helpers/fn.surround_pair.html)
- [surround_pair_with_escaper](https://docs.rs/pex/latest/pex/helpers/fn.surround_pair_with_escaper.html)

## number

- [match_number]()
- [match_number_if]()