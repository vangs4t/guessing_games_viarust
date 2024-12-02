use core::num;
/*Secara default rust punya beberapa library bawaan
yang bisa digunakan di setiap program */
// untuk penggunaan library kita bisa menggunakan kata kunci use
use std::io;
use std::cmp::Ordering;
/*library std::io memiliki cukup banyak fitur yang salah satunya
yaitu kemampuan untuk menerima user input pada terminal */
use rand::Rng;

fn main() { // kata kunci fn merupakan syntax untuk mendeklarasikan sebuah function atau dalam bahasa java nya method
    // sedangkan () mengindikasi tidak ada parameter
    /*Title Game terminal yang akan dijalankan */
    println!("Guess the number"); // println! merupakan makro

    let secret_number = rand::thread_rng().gen_range(0..20);
    println!("The secret number is {secret_number}");
    
    /*Setiap menggunakan loop kita harus menyiapkan titik akhir
    agar program dapat berhenti jika bertemu suatu kondisi
    cara menghentikan loop yaitu menggunakan kata kunci break, return dll */
    loop {
        /*Instruksi untuk user agar mengimput nomor
    tebakan  */
    println!("Please input your guess");


    /*Menambahkan variable untuk menyimpan input yang
    dimasukan oleh user */
    /* String:: new merupakan function yang mengembalikan
    String 
    Sedangkan syntax :: di dalam ::new mengindikasi kan kalau new adalah
    asosiasi fungsi dari String tipe
     */
    let mut guess = String::new();


    /*Setelah sebelumnya kita memanggil library menggunakan
    use std::io;
    Sekarang kita bisa memanggil fungsi stdin dari modul io yang dimana membolehkan kita untuk 
    menghandle user input 
    
    sedangkan function read_line merupakan method untuk mengatur
    apapun yang di input oleh user, mengubahnya menjadi string, dan memasukan nya ke dalam parameter
    dan method expect berfungsi untuk mengatur kemungkinan eror dan sejenisnya
    */
    io::stdin()
    .read_line(&mut guess) 
    .expect("Failed to read line");


    /*wait wait wait wait kenapa bisa ada 2 variable yang sama?
    yahh rust sebenarnya mengijinkan variable shadowing
    tapi jika terjadi variable shadowing maka
    variable yang terahir ditambahkan yang akan menjadi variable utama
    dalam variable guess terahir kita mengubah input user yang awalnya string menajadi
    number dengan methode parsing 
    oh ya kita juga harus menambahkan kondisi jika input
    user bukan number maka kita akan menolaknya dan mengharuskan user 
    input ulang sampai benar*/
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    
    /*match bisa digunakan untuk mencari kondisi
    untuk cmp method berfungsi untuk mengcompare dua value bersamaan 
    oh ya compare hanya bisa di gunakan untuk dua variable yang sama ya
    */
    println!("You guessed : {}", guess);
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("To small!"),
        Ordering::Greater => println!("To big!"),
        Ordering::Equal => {
            println!("You Win!");
            break; // hentikan loop jika user menang
        },
    }
    }
}