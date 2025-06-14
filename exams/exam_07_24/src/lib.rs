//Si considerino le seguenti strutture dati e rispettive porzioni di codice. Per ciascuna di esse si indichi la dimensione
// di memoria allocata nello stack e nello heap.
//Struttura 1
///```
///use std::rc::Rc;
///
///let rc: Rc<u64> = Rc::new(42);  
///let rc2 = rc.clone();  
///let wk = Rc::downgrade(&rc);
///```

//Struttura 2
///```
///let mut vector = Vec::<u64>::with_capacity(8);  
///for i in 0..5 {  
///    vector.push(i);  
///}  
///let vslice = &vector[1..3];

pub mod ex_1{
	// Considero architettura di 64 bit.
	
	//Struttura 1
	//La prima struttura è formata da tre smart pointer, un Rc e un Weak. Il primo smart pointer è un Rc che punta a una portzione di memeoria dell'heap di tipo
	// u64, quindi nello stack verranno istanziati 8 byte per salvare l'indirizo di memoria all'heap, mentre nell'heap verranno istanziati 8byte  per il contatore
	// storng, 8 byte contatore weak e 8 byte per il dato.
	// Il secondo puntatore essendo una clone rc viene istanziato nello stack altri 8 byte in cui viene salvata un copia del puntatore, nell'heap non cambia  
	// niente se non che viene aumentato il counter strong. 
	// L'ultimo puntatore è un waek che punta a rc quindi viengono istanziati 8 byte per salvare il puntatore che punta a rc, anche in questo caso non
	// viene istaziato nulla nell'heap ma viene incrementato il puntatore weak.
	// -stack: 8(rc) + 8(rc2) + 8(wk) = 24 byte
	// -heap: 8(strong) + 8(weak) + 8(u64) = 24 byte

	// Struttura 2
	// vector è una variabile di tipo Vec, cioè una struttura dati che nello stack istanzia un puntatore al'heap (8byte), capacity di (8byte) e len 8(byte), 
	// nell'heap in questo caso ha istazionziato un buffer di 8x8 byte. Daro che nel anche inserendo i dati dentro il buffer la capacity resta maggiore
	// di len non verrà fatto il move del buffer ma resta uguale. Vslice è invece un fat pointer che quindi istanzia nel	
}
