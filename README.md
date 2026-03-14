# Program CLI Mencari Teks Dalam File

Disini aku menggunakan file txt yang bernama GuideToTheImperialCity. isi file itu adalah catatan yang berasal dari buku 'Guide To The Imperial City' yang berada di dalam game The Elder Scrolls IV: Oblivion.

Cara menjalankan program:  
1. masuk ke terminal  
2. ketik "CASE_INSENSITIVE=1 cargo run (kata bebas) GuideToTheImperialCity.txt"
3. lalu program akan mengeluarkan output kalimat dari GuideToTheImperialCity.txt yang ada isi kata (kata bebas). Jika tidak ada kalimat yang memiliki kata (kata bebas), maka kalimat tersebut tidak akan muncul.

Keterangan:  
(kata bebas) adalah kata yang ingin dicari.  
GuideToTheImperialCity.txt adalah file yang akan dibaca.

Misalkan ketik "CASE_INSENSITIVE=1 cargo run Imperial GuideToTheImperialCity.txt", maka program akan mengeluarkan output kalimat dari GuideToTheImperialCity.txt yang ada isi kata Imperial/imperial/IMPERIAL.  
Karena menggunakan 'CASE_INSENSITIVE=1' Kata imperial(huruf kecil)/IMPERIAL(huruf besar) sama saja atau dianggap sama, karena inti dari program ini adalah mengeluarkan kalimat yang ada kata dari kata yang dicari.  
