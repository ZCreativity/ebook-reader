# Definizione del problema

Un ebook reader è un programma basato su GUI che permette ad un utente di aprirefile conte-nenti libri da visualizzare pagina per pagine, con la possibilità di tenere traccia del punto in cui si è arrivati, cambiare la grandezza del font, visualizzare a pagina singola o doppia, ecc.

# Obiettivi

L’obiettivo primario è quello di sviluppare un programma per la gestione di libri in formato EPUB, lavorando con la libreria Rust DRUID (vedere link nelle referenze). Si cerchi di personalizzare al massimo l’interazione con l’utente, eventualmente sfruttando eventualmente altri programmi come riferimento (sia per funzionalità di base, sia per difetti da migliorare). La strutturazione del codice faccia ampio uso di test per sviluppare e verificare le singole funzionalità. Inoltre, si permetta all’utente di attivare una modalità “correttore di bozza” che consenta di modificare il contenuto in presenza di errori (esempio errori di battitura), andando a generare un nuovo file ad ogni salvataggio. Il secondo obiettivo è quello di integrare il sistema di fotocamera con OCR(vedere il link in refe-rence) per mettere insieme il possesso del libro in formato cartaceo con quello in digitale in uno dei seguenti modi:
 1. Dal libro che si stava leggendo su carta, saltare al punto a cui si è arrivati facendo la foto alla pagina
 2. Operare l’inverso, cioè da due pagine riconosciute indicare a che pagina si trova il testo a cui si è arrivati sulla versione digitale.
 
# Contatti
Prof. Alessandro Savino

# Referenze:
- DRUID: https://github.com/linebender/druid
- Libri privi di copyright (legali): https://www.gutenberg.org 
- Metodi di OCR con RUST: https://www.linkedin.com/pulse/ocr-rustleptess-tesseract-ha%C3%AFkel-ouaghrem/
