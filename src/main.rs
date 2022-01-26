extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    let anjay: u32= "21".parse().expect("anjay");
    println!("{}",anjay);
    println!("-------------------------");
    let c = fungsi_dengan_return(3);
    println!("{}", c);
    println!("{}", if_else_expression(0));
    perulangan_dengan_while();
    keluarin_array_forloop();
}

fn fungsi_dengan_return(x: i32) -> i32 {
    x * x
    }

fn keluarin_array_forloop(){
    let arr = [1, 2, 4, 51,21, 22, 52, 32, 89];
    
    for ar in arr.iter() {
        println!("isi array = {}",ar);
    }
    for number in (1..100).rev() {
        println!("{}!", number);
    }
}

fn perulangan_dengan_while(){
    let mut x = 10;
    while x > 0{
        println!("mengulang masa lalu {} kali lagi", x);
        x = x - 1;
    }
    println!("selesai mengulang Masa Lalu");
}

fn if_else_expression(x: i32) -> i32{
    if x <= 10{
        if x / 2 == 0{
            x + 10
        } else if x - 5 == 0{
            x + 100
        } else {
            x
        }
    }else {
        x
    }
}

fn tipe_data(){
    //----- Float -----
    // ini f64
    let a = 2.6;
    // ini f32
    let b = 3.4;
    //---- Bool ------
    let c = true;
    let d: bool= false;
    //---- char -----
    let e = 'a';
    let f = 'd';
    //--- string ---
    let g = "selamat pagi";
    //--- tuple ---
    let h = (2, 3.2, "selamat siang");
    let (h1, h2, h3) = h;
    //--- array ----
    let i = [1, 2, 3, 4, 5, 6];

    println!("
            ini Float64 {} --- ini Float32 {}\n
            ini Bool {} ---- ini juga {}\n
            ini Char {} ---- ini juga {}\n
            ini String {}\n
            ini tuple {}\n
            ini array {}"
             , a,b,c,d,e,f, g, h3, i[3] );
}

fn fungsi_dengan_param(a: i32, b: i32){
    let c = a+b;
    println!("{}",c );
    println!("{}", a * b);
    println!("{}", a - b);
    println!("{}", a / b);
}

fn mencari_jumlah_huruf(){
    let mut jml_angka;
    println!("Masukan Angka");
    
    jml_angka = String::new();

    io::stdin().read_line(&mut jml_angka).expect("error :(");
    
    println!("ini hasilnya ea");
    println!("{}", jml_angka.len() - 1);
}

fn muta_imuta(){
    let mut angka_mut;
    println!("Masukan Angka");
    
    angka_mut = String::new();

    io::stdin().read_line(&mut angka_mut).expect("Ada Err");

    println!("ini angka yang tadi {}", angka_mut);
}

