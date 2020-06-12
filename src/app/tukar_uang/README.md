# Uang Kembalian

Problem from https://tlx.toki.id/courses/competitive/chapters/07/problems/A

### Deskripsi

Koin kembalian adalah persoalan yang umum ditemukan dimana-mana. Saat Anda membayar belanjaan, jika uang Anda tidak pas, tentu Anda mengharapkan kembalian bukan? Dan secara umum, biasanya Anda mengharapkan kembalian dengan koin sesedikit mungkin supaya kantong Anda tidak menjadi berat.

Secara spesifik, saat ini Anda diminta untuk menuliskan sebuah program yang dapat mencari kemungkinan kembalian dengan banyak koin sesedikit mungkin.

### Format Masukan
Field `pecahan_koin_tersedia` berisi array bilangan yang merupakan nilai dari masing-masing jenis koin tersebut. Field `uang` berisi sebuah bilangan yang menyatakan besar uang kembalian. 

### Format Keluaran
Field `total_koin_minimal` yang merupakan banyak koin minimal yang dapat digunakan untuk menyatakan kembalian yang diinginkan. Jika ternyata, dengan jenis koin yang tersedia, Anda tidak dapat menyatakan kembalian dengan tepat, keluarkan -1 (misalnya kembalian seharusnya 5 namun hanya ada satu jenis koin yakni 2).

Contoh body payload:
```
{
	"pecahan_koin_tersedia": [1, 2, 5, 6],
	"uang": 12
}
```

Contoh Keluaran
```
{
    "total_koin_minimal": 2
}
```