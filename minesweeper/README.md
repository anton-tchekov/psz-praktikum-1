# Code Walkthrough

## Welche Frameworks wurden verwendet?

- Macroquad (https://macroquad.rs/)

Simple and easy to use game library for Rust programming language.

## Was ist nativer Rust-Code?

Rust ist eine kompilierte Sprache, die direkt für die
CPU-Architektur und Platform kompiliert wird, und
ohne Runtime auskommt (wie es etwa bei Java der Fall ist)

## In welcher Entwicklungsumgebung wurde gearbeitet?

- Visual Studio Code

## Wie kann dass Ergebnis verwendet werden?

Dass Spiel kann mit dem folgenden Befehl gestartet werden:

```
cargo run
```

Im Menü kann die gewünschte Schwierigkeitsstufe
mit Pfeiltasten und Enter ausgewählt werden.

![Menu](menu.png)

Mit Linksklick könnnen Felder aufgedeckt werden.
Mit Rechtsklick können Felder markiert werden.
Die Zahlen zeigen an, wie viele Minen sich im Umfeld befinden.
Man kann das Spiel jederzeit mit Escape verlassen, um zum
Menü zurückzukehren.

![Game](first.png)

Wenn man ein Feld mit einer Mine aufdeckt, hat man
verloren.

![Lose](lose.png)

Hat man alle Felder (außer die Minen) aufgedeckt,
hat man gewonnnen.

## Beobachtungen aus der Implementierung

### Unterschiede zu anderen Programmiersprachen

- match statt switch
- Keine do-while Schleife (Kann mit while Expression umgesetzt werden)
- Statt Klassen struct und impl

### Vorteile von Rust (Meiner Meinung)

- Memory Safety
	- Borrow Checker
	- Array Bounds Checking

- Option und Result Typen
	- Statt Exceptions
	- Kein null

- Kontrollstrukturen können in Expressions verwendet werden

### Nachteile von Rust (Meiner Meinung)

Eher kleinere nervige Punkte:

- Variablen müssen mit `mut` deklariert werden, ansonsten sind es Konstanten by Default
	- Verursacht häufig Kompilierfehler, da man es oft vergisst

- Typumwandlungen müssen immer gecastet werden, auch wenn die Konvertierung nicht verlustbehaftet ist (z.B. u8 nach u32)
	- Sehe ich als "Clutter" im Code

- Im Vergleich zu C, Referenzen werden automatisch (auch über mehrere Ebenen) dereferenziert
	- Macht den Code schlechter verständlich, da man nicht explizit sieht was passiert

- Variablentypen werden vom Compiler automatisch erkannt
	- Sehe ich als Nachteil beim Lesen vom Code, da man nicht direkt sehen kann welcher Typ eine Variable hat
	- Besonders bei Rückgabewerten von importierten Funktionen schwierig
