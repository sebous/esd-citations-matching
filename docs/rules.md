# Rules

> all texts are lowercased and unicode normalized (all diacritics are removed)
  
> `start` = start of match / first character of the matched string

> `end` = end of match / last character of the matched string

> `(start - 10)` = 10 characters before start of match

## ECLI

- matches regex `ecli:eu:c:\d{4}:\d{1,3}`

## full code

- matches regex `([CTF]\s*[\u2010\u2011\u2012\u2013\u2014\u2212\u002D]\s*\d{1,4}[/\u2010\u2011\u2012\u2013\u2014\u2212\u002D]\d{2})([ \u202F\u00A0,.)]|$)`

## num code (with additional params)

- matches regex `[\s\u202F\u00A0](\d{1,4}[/\u2010\u2011\u2012\u2013\u2014\u2212\u002D]\d{2})([\s\u202F\u00A0,.)]|$)`
- text from `(start - 150)` to `(end + 150)` must contain any of `["dvůr", "dvora", "dvoře", "dvorem", "dvoru", "SDEU", "ESD"]`

- every rule below must be false:
  - text from `end` to `(end + 10)` must start with `"sbnu"`
  - text from `(start - 10)` to `start` must end with `"us"`
  - text from `(start - 15)` to `start` must contain any of `["smernic", "stiznost"]`
  - text from `(start - 100)` to `start` must contain any of `["spolkovy", "spolkoveho", "narizeni"]`
