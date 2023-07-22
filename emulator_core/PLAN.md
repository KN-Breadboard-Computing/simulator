# Zamysł
- Graf, gdzie węzły to poszczególne komponenty, bramki, itd.
- Krawędzie to połączenia kabelkowe między komponentami
- Kiedy nastąpi jakaś akcja w układzie, zmiany są propagowane przez graf BFSem 
- Każdy komponent ma wejścia i wyjścia, może ich być wiele i każdy może być połączony do czegoś innego
- W ramach propagacji komponent przyjmuje wszystkie wejścia i zgodnie z ustaloną logiką ustawia wyjścia, które zostają przekazane na wejścia kolejnych komponentów

# Plan 
## Faza 1
- Cel : Prosty symulator układów boolowskich
- Przygotowanie frontendu (Javascript + lekkie wasmowe API)
- API : dodawanie i usuwanie komponentów i połączeń, odczytywanie i zmienianie stanu komponentów, (opcjonalnie: odczytywanie stanów na połączeniach)