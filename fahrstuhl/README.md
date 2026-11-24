# TODO

- Cam follow one

- STEP 1
	- Cabins Store targets
	- Drive up and down, visit all pressed in between
	- Timeout Doors
	- Visited turn off

- STEP 2
	- Player Movement and Entering Elevators ets.
	- Max 2 per elevator
	- Doors collision

# Requirements

| ID | Anforderung | Erfüllt |
|----|-------------|---------|
| F1 | Das Fahrstuhlsystem besteht aus 3 Fahrkabinen, die zwischen Ebenen 0 und 3 fahren und beliebig vielen Passagieren | X |
| F2 | Fahrkabinen fahren unabhängig voneinander (parallel) | X |
| F3 | Fahraufträge werden durch Knöpfe in der Ebene oder in der Fahrkabine erteilt | X |
| F3.1 | In den Ebenen können Anforderungen für AUF und AB gewählt werden | X |
| F3.2 | In den Kabinen können Zielstockwerke gewählt werden | X |
| F4 | Fahrkabinen zeigen Ihre Position in der Kabine an | X |
| | | |
| P1 | Passagiere werden auf den Ebenen erzeugt und wählen zufällig AUF oder AB als Fahrauftrag in der Ebene | X |
| P2 | Passagiere betreten Fahrkabinen wenn sie in der Ebene halten und die Türen in den Zuständen offen, öffnend und schließend sind. | |
| P3 | Passagiere wählen zufällig Zielebenen in der Kabine | X |
| P4 | Passagiere können Fahrkabinen nur bei geöffneter Tür verlassen | |
| | | |
| Z1 | Zustände der Fahrkabinen: Kabine steht in Ebene x, Kabine fährt von Ebene x zu Ebene y, Kabine hält in Ebene x, | X |
| Z2 | Zustände der Türen: Türen geschlossen, Türen öffnend (in Bewegung von geschlossen zu offen), Türen offen, Türen schließend (in Bewegung von offen zu geschlossen) | X |
| Z3 | Zustände eines Passagiers: idle auf Ebene x, betritt Fahrkabine, wählt Zielebene, ist in Fahrkabine, verlässt Fahrkabine | X |
| | | |
| C1 | Eine zentrale Steuerung gibt Fahraufträge aus der Ebene an Fahrkabinen und überwacht die Bewegungen der Fahrkabinen und Fahraufträge | |
| C2 | Die zentrale Steuerung steuert die Fahrkabinen zu Ebenen entsprechend der vorliegenden Aufträge | X |
| C3 | Die zentrale Steuerung steuert das Öffnen und Schließen der Türen | X |
| C4 | Die zentrale Steuerung überwacht die Anzahl der Passagiere pro Kabine | |
| | | |
| S1 | Türen von Kabinen sind während der Fahrt nicht offen | X |
| S2 | Kabinen die auf Ebene 0 stehen können nicht abwärts fahren | X |
| S3 | Kabinen, die auf Ebene 3 stehen, können nicht aufwärts fahren | X |
| S4 | Türen von Kabinen, die in Ebene x stehen, schließen sich nach Timeout | |
| S5 | Es dürfen nicht mehr als 2 Passagiere gleichzeitig transportiert werden. | |
| S6 | Wenn zuviele Passagiere in einer Fahrkabine sind, wird das geeignet angezeigt und die Türen schließen erst wenn die maximale Anzahl an Passagieren nicht überschritten ist | |
| S7 | Wenn beim Schließen der Türen Passagiere oder andere Hindernisse in der Tür entdeckt werden, dann öffnen sich die Türen wieder |  |
| | | |
| N1 | Bei der Implementierung sollen die Hauptkomponenten in separaten Threads realisiert werden und weitgehend unabhängig von einander agieren Bsp: wenn sich die Türen in Kabine 1 schließen kann Kabine 2 trotzdem fahren | X |
