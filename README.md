# WIP Symulator układów logicznych
Jednym z pobocznych projektów KN Breadboard Computing jest własny symulator układów logicznych.
Na razie jest w bardzo wczesnym stadium prototypowania, ale w przyszłości może będzie pomagać w pracach nad naszych sprzętem i przy działalności dydaktycznej.

## Cele 
- Tworzenie układów - zarówno na "niskim" poziomie, tj. przy pomocy podstawowych bramek logicznych, jak i na "wysokim" poziomie, łącząc ze sobą złożone komponenty (MEM, ALU, CU, itp.)
- Symulowanie układów - uruchamianie i testowanie działania, wpuszczanie sygnału ręcznie albo przy pomocy zegara
- Debugowanie - możliwość sprawdzenia i modyfikowania stanu komponentów takich jak pamięć oraz wgrywanie i testowanie napisanych programów
- Biblioteka - działanie jest zaimplementowane w postaci biblioteki, której można użyć z poziomu dowolnie przygotowanego frontendu, czy to graficzny, tekstowy, czy do automatycznego testowania
- Graficzny frontend - najłatwiejsza forma tworzenia układu, działa na komputerze użytkownika, który może tworzyć i uruchamiać, zapisywać i wczytywać, itd.
- Łatwe dodawanie nowych komponentów - jak najmniej boilerplate'u.

## Uruchamianie 
Zbudowany projekt można zobaczyć i przetestować pod https://kn-breadboard-computing.github.io/simulator/
W celu uruchomienia lokalnie będziesz potrzebował zainstalowanego języka [rust](https://www.rust-lang.org/tools/install).
Polecam zapoznać się z nim choć trochę zapoznać, na przykład przez przeczytanie pierwszych rozdziałów [The Book](https://doc.rust-lang.org/stable/book/).
Razem z nim zainstaluje się narzędzie *cargo*, które potrafi pobrać potrzebne biblioteki i uruchomić dowolny projekt przy użyciu polecenia `cargo run`.
(Jeżeli chcesz zbudować frontend, to musisz wywołać polecenie z poziomu katalogu *./emulator_web*)
