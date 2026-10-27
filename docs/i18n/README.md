# Translations

The launcher speaks English, Turkish, German, Spanish, French and Brazilian Portuguese (`Language::ALL`).
English is the source. Each of the three newest languages has a glossary so the same thing is called the same
word on every screen, now and in strings added later.

| File | Language | Code | Console language number | Address |
|---|---|---|---|---|
| [es.md](es.md) | Spanish (neutral) | `es` | 3 (Spain), 20 (Latin America, picked by the system locale) | tú |
| [fr.md](fr.md) | French | `fr` | 2 | tu |
| [pt-BR.md](pt-BR.md) | Brazilian Portuguese | `pt` | 17 | você |

Spanish, French and Portuguese were written by an AI model and have **not** been reviewed by a native speaker
yet. Native speakers: pull requests that improve a text are very welcome (see the Texts section of
[CONTRIBUTING.md](../../CONTRIBUTING.md#texts)). When you change a term, change the glossary in the same pull
request so the two stay in step.

## How a text is added

A string lives next to its English in the strings file of its screen
(`crates/anyps5-launcher/src/screens/*_strings.rs`, or `ui/i18n_table.rs` for the shared ones). Every text enum has
`english()`, `turkish()`, `german()`, `spanish()`, `french()` and `portuguese()`. The last four return `Option`
and end in `_ => return None`; the test `every_text_has_an_arm_in_every_language`
(`screens/language_coverage.rs`) fails when a variant lacks an arm, so nothing reaches a player in English by
accident. The core table keeps its translations in `ui/i18n_german.rs`, `ui/i18n_spanish.rs`, `ui/i18n_french.rs`
and `ui/i18n_portuguese.rs`.

## Rules for all three

- Friendly, plain, console tone. Short sentences, the imperative for instructions, no exclamation marks, no
  jokes, no slang that will not travel.
- Sentence case, not English Title Case: capitalise the first word and proper names only. A button reads
  "Iniciar sesión", never "Iniciar Sesión".
- Names stay as they are: AnyPlay, AnyPlay Launcher, AnyPS5, LudusCtrl, Play Together, Discord, Steam, Toolkit,
  Party, GitHub, Flatpak.
- Keep the placeholders `%@` and `%d` and their order; the program fills them in. `%%` is a percent sign. If the
  language needs another word order, rephrase the sentence so the placeholders still come in the same order.
- Never build a sentence by joining pieces around a variable. Plural forms are separate texts.
- Numbers, dates and file sizes are formatted by the program (`screens/format.rs`, `ui/i18n.rs::byte_text`);
  texts only hold words.
- Never name the console or its maker in a text (the same rule as English).
