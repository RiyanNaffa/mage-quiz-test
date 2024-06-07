use press_btn_continue::wait;
use std::io::{self};


pub fn game_over_message(name: &str, progress: u32){
    println!("
    ================================================================================
    =============================      GAME OVER!      =============================
    ============================ (Thanks for playing) ==============================
    ================================================================================
    
    Player:   {}
    Progress: {}/3 stages
    ", name, progress);
}
pub struct Player{
    health: i32,
    name: String,
    age: i32,
    progress: u32,
    // total_progress: u32,
    item: Items,
    trophies: Vec<Trophies>,
}
impl Player{
    // ===== CONSTRUCTOR ======
    pub fn new(name: &str, age: i32) -> Self {
        let trophy_capacity: usize = 10;
        Self { health: 1000, name: name.to_string(), age, progress: 0, item: Items::Non, trophies: Vec::with_capacity(trophy_capacity) }
    }
    // ====== DISPLAY PURPOSES =====
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_progress(&self) -> u32 {
        self.progress
    }
    // ===== STATS AT THE END CREDIT =====
    pub fn the_end_progress(&mut self){
        match self.progress {
            3 => self.add_trophy_to_collection(Trophies::CertifiedMage),
            _ => (),
        }
    }
    pub fn progress_message(&self){
        match self.progress {
            3 => {
                println!("\tYou are now a great level 2 mage.\n~Serie\n");
            },
            _ => println!("\tGak lulus.\n~Serie\n"),
        }
    }
    pub fn display_trophies(&self){
        let trophy_iter = self.trophies.iter();
        let mut ctr = 0;
        // let mut file = File::create("history.txt").unwrap();
        println!("Your trophies:\t\t<( - ~ -  )>");
        for it in trophy_iter{
            ctr += 1;
            let msg = it.get_message();
            /* // Write a .txt file out of it
            file.write(msg);
            let message = match str::from_utf8(msg) {
                Ok(ms) => ms,
                Err(e) => println!("Error while reading data: {}.", e)
            } */
            // Display on the end credit
            println!("{}. {}", ctr, msg);
        }
        println!("");
    }
    // ===== HP PLAYER =====
    pub fn affect_health(&mut self){
        let hp_dmg = self.item.hp_dmg();
        self.item.message();
        self.health += hp_dmg;
    }
    pub fn get_health(&self) -> i32 {
        return self.health;
    }
    pub fn check_health(&self)-> bool {
        if self.health < 0 {
            game_over_message(&self.name, self.progress);
            self.progress_message();
            true
        } else {
            false
        }
    }
    // ===== INCREMENT IN-GAME PROGRESS =====
    fn incr_progress(&mut self){
        self.progress += 1
    }
    // ===== AGE VERIFICATION =====
    fn match_age(&self) -> Option<i32>{
        if self.age >= 18 {
            return Some(self.age)
        } else {
            return None
        }
    }
    pub fn age_confirmation_message(&self) -> bool{
        let age = self.match_age();
        if let Some(i) = age{
            println!("You are {}, safe to play this game.", i);
            println!("Welcome, {}", self.name);
            wait("").unwrap();
            false
        } else {
            println!("
Did your parents never teach you internet?
Shame on you, you dirty little mustard.
Now help your mother take the groceries.");
            game_over_message(&self.name, self.progress);
            true
        }
    }
    // ===== SET THE ITEM HELD =====
    fn set_to_none(&mut self){
        self.item = Items::Non
    }
    fn set_item(&mut self, item: Items){
        self.item = item
    }
    fn add_trophy_to_collection(&mut self, trophy: Trophies){
        self.trophies.push(trophy)
    }
    
}

enum Items {
    // Non
    Non,
    // Waktu perang (damage)
    Mimic,
    BomPipa,
    JebakanSorganeil,
    SabetanGeisel,
    TebasanUbel,
    KlonFrieren,
    // Waktu perang (heal)
    RotiKering,
    TehCampfire,
    DagingMonster,
    SihirPenyembuhan,
    // Istirahat (heal)
    RamuanCintaFrieren,
    BirBoshaft,
    Patisserie,
    SteakALaHimmel,
    ObatKuatHimmel,
}
impl Items{
    fn message(&self) {
        let _hp_dmg = self.hp_dmg();
        match self {
            Items::Non => (),
            Items::Mimic => {
                println!("*sfx* krauk");
                println!("HP berkurang {}", _hp_dmg*-1);
                wait("").unwrap();
            },
            Items::BomPipa => {
                println!("Kau menyibak semak-semak di ujung itu.");
                println!(r#""Hei, apa ini? Bentuknya lucu.""#);
                println!("*sfx* DUAR!");
                println!("HP berkurang {}", _hp_dmg*(-1));
                wait("").unwrap();
            },
            Items::JebakanSorganeil => {
                println!("Seseorang menggunakan Sorganeil, sihir mengikat target.");
                println!("Dia menyerangmu dengan cepat.");
                println!("Untungnya kau bisa menghindar.");
                println!(r#""Syukurlah ini hanya luka kecil.""#);
                println!("HP berkurang {}", _hp_dmg*-1);
                wait("").unwrap();
            },
            Items::SabetanGeisel => {
                println!("Geisel terkenal dengan sabetan ekornya yang tajam.");
                println!("Ketika mereka mendekat, kau mencoba untuk merapal mantra serangan.");
                println!("Tapi sabetan mereka tak terhindarkan.");
                println!("Sabetan ekor Geisel mengenai tubuhmu dan kau kehilangan keseimbangan.");
                println!("*sfx* bruk!");
                println!("HP berkurang {}", _hp_dmg*-1);
                wait("").unwrap();
            },
            Items::TebasanUbel => {
                println!("Kau benar-benar sedang tidak beruntung sampai-sampai bertemu dengan Ubel.");
                println!("Tentu saja dia tak mau kalah. Tebasannya tepat mengenai dadamu.");
                println!(r#""Itu tebasan paling mengerikan yang pernah kurasakan...""#);
                println!("HP berkurang {}", _hp_dmg*-1);
                wait("").unwrap();
            },
            Items::KlonFrieren => {
                println!("Kau pikir bertemu dengan Ubel merupakan hari paling tidak beruntung?");
                println!("Kali ini, Klon Frieren sendiri mendatangimu.");
                println!("Instant death!");
                println!("HP berkurang {}", _hp_dmg*-1);
                wait("").unwrap();
            },
            Items::RotiKering => {
                println!(r#""Kita lihat apa yang ada di dalam tas kita.""#);
                println!(r#""Iuh! roti dari kapan ini?.""#);
                println!(r#""Lebih baik roti kering daripada kelaparan.""#);
                println!("HP bertambah {}", _hp_dmg);
                wait("").unwrap();
            },
            Items::TehCampfire => {
                println!("Hari sudah malam. Sudah saatnya untuk berkemah.");
                println!("Kau mendirikan tenda dan menyalakan api unggun.");
                println!("Teh yang kaubawa sejak berangkat akhirnya diminum.");
                println!(r#""Mmm... Dulu Himmel suka minum teh ini sampai beser.""#);
                println!("HP bertambah {}", _hp_dmg);
                wait("").unwrap();
            },
            Items::DagingMonster => {
                println!("Setelah pertarungan yang (tidak) begitu sengit dengan seekor monster,");
                println!("akhirnya kau dapat beristirahat dan membakar dagingnya.");
                println!(r#""Kebetulan sekali monster ini memiliki daging yang enak.""#);
                println!(r#""Semoga saja sihir barbekyu enak milikku bekerja dengan sempurna.""#);
                println!("HP bertambah {}", _hp_dmg);
                wait("").unwrap();
            },
            Items::SihirPenyembuhan => {
                // println!("Pertarungan tersebut membuatmu sangat kelelahan.");
                // println!(r#""Argh! Kenapa ujian mage level 2 sesulit ini, sih?""#);
                println!(r#""Sihir Penyembuhan!""#);
                println!("HP bertambah {}", _hp_dmg);
                wait("").unwrap();
            },
            Items::RamuanCintaFrieren => {
                println!("Kau menginap di sebuah hotel dan bertemu kembali dengan Fern.");
                println!(r#""Lihat apa yang kubawa. <(¬‿¬)>"
                "#);
                println!(r#""Dasar mesum."
                "#);
                println!(r#"*sfx* bonk "Aduh!""#);
                println!("HP bertambah {}, setidaknya kali ini kau hanya diketuk tongkatnya Fern.", _hp_dmg);
                wait("").unwrap();
            },
            Items::BirBoshaft => {
                println!("Untuk menyegarkan diri, kau dan Stark pergi ke bar di tengah kota.");
                println!("Fern menolak untuk ikut. Dia malah mengomel-omel.");
                println!(r#""Paman, bir terenak satu!"
                "#);
                println!(r#""Hei, aku juga, Stark. Bir satu."
                "#);
                println!("*beberapa saatu kemudian*");
                println!(r#""Uangku habis hanya untuk segelas Bir Boshaft." (`~ `. )"#);
                println!("Pelajaran hari ini: malu bertanya rekening sirna.");
                println!("HP bertambah {}", _hp_dmg);
                wait("").unwrap();
            },
            Items::Patisserie => {
                println!("Kalian bertiga memutuskan untuk mencicipi roti isi yang katanya terenak.");
                println!(r#""Hmm, harus kuakui roti-rotinya menarik."
                "#);
                println!(r#""Benar, Nona Frieren. Aku diberitahu bahwa ini merupakan toko roti terbaik di negeri ini."
                "#);
                println!(r#""Kuingat kemarin membeli sourdough Waal."
                "#);
                println!(r#""Toko itu sudah tutup 50 tahun yang lalu, Nona Frieren.""#);
                println!("HP bertambah {}", _hp_dmg);
                wait("").unwrap();
            },
            Items::SteakALaHimmel => {
                println!("Setelah beristirahat dengan cukup, kalian bertiga pergi ke sebuah rumah makan legendaris milik Lecker.");
                println!(r#""Aku ingat pernah ke sini bersama dengan Himmel dan yang lainnya.""#);
                println!(r#""Semoga saja rasanya tidak berubah."
                "#);
                println!(r#""Itu sudah 80 tahun lalu..."
                "#);
                println!(r#""Nah, kita sudah sampai. Aku mau pesan steak porsi Frieren!""#);
                println!("HP bertambah {}", _hp_dmg);
                wait("").unwrap();
            },
            Items::ObatKuatHimmel => {
                println!("Kau melihat-lihat kembali isi kopermu.");
                println!("Sebuah gelas ramuan berisi cairan berwarna biru menarik perhatianmu.");
                println!(r#""Ah! Apa ini? Rasanya aku pernah lihat...""#);
                println!(r#""I-Ini bukannya Obat Kuat Himmel (TM) yang legendaris itu!""#);
                println!(r#""Himmel pernah bilang kalau ramuan ini yang membuatnya kuat melawan Demon King.""#);
                println!(r#""Aku izin minum, Himmel.""#);
                println!("HP bertambah {}, rasanya kini kau tahan melawan Demon King sendirian.", _hp_dmg);
                wait("").unwrap();
            },
        }
    }
    fn hp_dmg(&self) -> i32 {
        return match self {
            Items::Non => 0,
            // Waktu perang (damage)
            Items::Mimic => -20,
            Items::BomPipa => -100,
            Items::JebakanSorganeil => -200,
            Items::SabetanGeisel => -300,
            Items::TebasanUbel => -500,
            Items::KlonFrieren => -1000,
            // Waktu perang (heal)
            Items::RotiKering => 40,
            Items::TehCampfire => 60,
            Items::DagingMonster => 150,
            Items::SihirPenyembuhan => 300,
            // Istirahat (heal)
            Items::RamuanCintaFrieren => 0,
            Items::BirBoshaft => 10,
            Items::Patisserie => 100,
            Items::SteakALaHimmel => 300,
            Items::ObatKuatHimmel => 1000,
        }
    }
    /* pub fn get_item_random() -> Self {
        
    } */
}

enum Trophies {
    Quiz1,
    Quiz2,
    Quiz3,
    CertifiedMage,
}
impl Trophies {
    fn get_message(&self) -> &str{
        match self {
            Trophies::Quiz1 => "First Test Passed.",
            Trophies::Quiz2 => "Second Test Passed.",
            Trophies::Quiz3 => "Third Test Passed.",
            Trophies::CertifiedMage => "A Certified Mage with Grand Mage Serie License.",
        }
    }
}

pub fn quiz_1(player: &mut Player)->bool{
    println!("Sebuah monster griffin menghalangi jalanmu ke puncak.");
    println!("Akan tetapi, monster itu bukanlah fokus utamamu, jadi kau memutuskan untuk berlindung.");
    println!("Ke manakah kau akan pergi berlindung?");
    println!("1. Bersembunyi di balik pohon dan menekan mana-mu.\n2. Berlindung di dalam semak-semak.\n3. Merapal sihir terbang.");
    let mut opt = String::new();
    io::stdin().read_line(&mut opt).expect("Cannot read line.");
    opt = opt.trim().to_owned();
    let opt_i = opt.parse::<i32>().unwrap();
    match opt_i {
        1 => {
            println!("Kau memilih jurus andalanmu, yakni menekan mana.");
            println!("Kini, kau tidak terdeteksi oleh griffin itu.");
            println!("Kau melanjutkan perjalanan dengan aman, setidaknya untuk saat ini.");
            println!("HP sekarang: {}", player.get_health());
            player.add_trophy_to_collection(Trophies::Quiz1);
            player.incr_progress();
            wait("").unwrap();
            false
        },
        3 => {
            println!(r#""Sihir terbang memang yang terbaik.""#);
            println!("Kau bergumam sambil memandang griffin yang sedang kebingungan di bawahmu.");
            println!("HP sekarang: {}", player.get_health());
            player.add_trophy_to_collection(Trophies::Quiz1);
            player.incr_progress();
            wait("").unwrap();
            false
        },
        _ => {
            let item = Items::BomPipa;
            player.set_item(item);
            player.affect_health();
            println!("HP sekarang: {}", player.get_health());
            if player.check_health(){
                true
            } else {
                player.add_trophy_to_collection(Trophies::Quiz1);
                player.incr_progress();
                false
            }
        }
    }
}

pub fn quiz_2(player: &mut Player)->bool{
    println!("Jalanmu terhalangi oleh jurang yang sangat dalam.");
    println!("Di belakangmu sudah mengejar, dua Geisel, monster burung yang memakan manusia.");
    println!(r#""Sudah tidak ada waktu untuk memutar balik.""#);
    println!("Apakah yang akan kau lakukan?");
    println!("1. Mencari tempat berlindung dan menekan mana-mu.\n2. Berbalik arah dan menyerang langsung Geisel.\n3. Merapal sihir terbang.");
    let mut opt = String::new();
    io::stdin().read_line(&mut opt).expect("Cannot read line.");
    opt = opt.trim().to_owned();
    let opt_i = opt.parse::<i32>().unwrap();
    match opt_i {
        1 => {
            println!(r#""Memang benar apa yang dikatakan Master Flamme.\nMenekan mana memang sangat membantu di saat-saat genting.\nGeisel-geisel itu mencari mangsa dengan mendeteksi mana mereka.""#);
            println!("HP sekarang: {}", player.get_health());
            player.add_trophy_to_collection(Trophies::Quiz2);
            player.incr_progress();
            wait("").unwrap();
            false
        },
        _ => {
            /* println!("Geisel terkenal dengan sabetan ekornya yang tajam, dan bisa terbang.");
            println!("Ketika mereka mendekat, kau mencoba untuk merapal mantra serangan.");
            println!("Tapi sabetan mereka tak terhindarkan.");
            println!("Sabetan ekor Geisel mengenai tubuhmu dan kau kehilangan keseimbangan.");
            println!("*sfx* bruk!"); */
            let item = Items::SabetanGeisel;
            player.set_item(item);
            player.affect_health();
            println!("HP sekarang: {}", player.get_health());
            wait("").unwrap();
            if player.check_health(){
                true
            } else {
                player.add_trophy_to_collection(Trophies::Quiz2);
                player.incr_progress();
                false
            }
        },
    }
}

pub fn quiz_3(player: &mut Player)->bool{
    println!("Di dalam dungeon yang anti-sihir ini, kau melihat ada tiga buah peti.");
    println!("Di bagian depan peti terdapat sebuah relief yang bertuliskan petunjuk peti mana yang memiliki kitab sihir.");
    println!("Sedangkan sisanya merupakan mimic.");
    println!(r#"Peti 1: "Salah satu dari ketiga peti mengatakan kebenaran.""#);
    println!(r#"Peti 2: "Dua peti yang lain mengatakan kebenaran.""#);
    println!(r#"Peti 3: "Kitabnya tidak ada di sini.""#);
    println!("Peti manakah yang mengatakan kebenaran?");
    println!("1. Peti 1.\n2. Peti 2.\n3. Peti 3.");
    let mut opt = String::new();
    io::stdin().read_line(&mut opt).expect("Cannot read line.");
    opt = opt.trim().to_owned();
    let opt_i = opt.parse::<i32>().unwrap();
    match opt_i {
        3 => {
            println!("Kau membuka perlahan peti 3.\nIsinya benar kitab sihir penyembuhan.\nIni pertama kalinya dalam sejarah Frieren tidak tertipu oleh seekor Mimic.");
            let item = Items::SihirPenyembuhan;
            player.set_item(item);
            player.affect_health();
            println!("HP sekarang: {}", player.get_health());
            wait("").unwrap();
            player.add_trophy_to_collection(Trophies::Quiz3);
            player.incr_progress();
            false
        },
        _ => {
            println!("Kau mendekat perlahan-lahan ke arah peti itu.\nKetika tanganmu hendak meraih gagangnya, peti tersebut terbuka lebar dengan gigi yang tajam dan lidah yang menjulur-julur.");
            let item = Items::Mimic;
            player.set_item(item);
            player.affect_health();
            println!("HP sekarang: {}", player.get_health());
            wait("").unwrap();
            if player.check_health(){
                true
            } else {
                player.add_trophy_to_collection(Trophies::Quiz3);
                player.incr_progress();
                false
            }
        }
    }
}