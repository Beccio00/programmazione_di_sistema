# Cache in Rust

## Definizione

Una cache è una struttura dati, generica, thread safe che consente di memorizzare coppie chiave/valore per un periodo non superiore ad una durata stabilita per ciascuna coppia.

Nell'intervallo di validità associato alla coppia, richieste di lettura basate sulla chiave restituiscono il valore corrispondente, se presente. Trascorso tale periodo, eventuali richieste relative alla stessa chiave non restituiscono più il valore.

Poiché il numero di richieste in lettura può essere molto maggiore delle richieste in scrittura, è necessario fare in modo che le prime possano sovrapporsi temporalmente tra loro, mentre le seconde dovranno necessariamente essere con accesso esclusivo. Per evitare la saturazione della struttura, quando si eseguono operazioni di scrittura, si provveda ad eseguire un ciclo di pulizia, eliminando le eventuali copie scadute.

## Implementazione

Si implementi in Rust la struct `Cache<K: Eq+Hash, V>` dotata dei seguenti metodi:

### Metodi Pubblici

- `pub fn new() -> Self` // Crea una nuova istanza
- `pub fn size(&self) -> usize` // Restituisce il numero di coppie presenti nella mappa
- `pub fn put(&self, k: K, v: V, d: Duration) -> ()` // Inserisce la coppia k/v con durata pari a d
- `pub fn renew(&self, k: &K, d: Duration) -> Bool` // Rinnova la durata dell'elemento rappresentato dalla chiave k; restituisce true se la chiave esiste e non è scaduta, altrimenti restituisce false
- `pub fn get(&self, k: &K) -> Option<Arc<V>>` // Restituisce None se la chiave k è scaduta o non è presente nella cache; altrimenti restituisce Some(a), dove a è di tipo Arc<V>

## Note Tecniche

Si ricordi che Duration è una struttura contenuta in std::time, che rappresenta una durata non negativa. Può essere sommato ad un valore di tipo std::time::Instant (che rappresenta un momento specifico nel tempo) per dare origine ad un nuovo Instant, collocato più avanti nel tempo.
