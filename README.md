# Sig Remover (Penghapus Latar Belakang Tanda Tangan) ✍️

Sebuah aplikasi untuk menghapus latar belakang (background) putih/terang pada foto tanda tangan di atas kertas, sehingga menghasilkan gambar tanda tangan yang transparan (PNG). 

Proyek ini mencakup dua versi: **Aplikasi Desktop** (dibangun dengan Rust & Dioxus) dan **Aplikasi Web** (HTML, CSS, JS murni).

---

## 🚀 Fitur Utama

- **Pemrosesan Instan**: Mengubah latar kertas menjadi transparan dengan cepat.
- **Kontrol Penuh**:
  - **Threshold**: Mengatur batas deteksi warna putih/terang.
  - **Toleransi**: Menghaluskan piksel di sekitar tinta.
  - **Kontras**: Memperjelas atau menebalkan tinta tanda tangan sebelum latar belakang dihapus.
- **Transparansi Sempurna**: Output disimpan dalam format PNG transparan yang siap disisipkan ke dokumen digital (Word, PDF, dll).
- **Desain Premium**: Antarmuka pengguna modern dengan tema gelap (Dark Theme) dan efek *Glassmorphism*.

---

## 💻 Versi 1: Aplikasi Desktop (Rust + Dioxus)

Versi ini berjalan sebagai aplikasi *native* di Windows. Dibangun dengan bahasa pemrograman Rust untuk performa yang sangat cepat dan Dioxus 0.6 untuk antarmuka pengguna (UI).

### Prasyarat
- [Rust & Cargo](https://www.rust-lang.org/tools/install)
- [Dioxus CLI](https://dioxuslabs.com/learn/0.6/getting_started) (`cargo install dioxus-cli`)

### Cara Menjalankan (Development)
1. Buka terminal di direktori proyek ini.
2. Jalankan perintah:
   ```bash
   cargo run
   ```

### Cara Build (Production)
Untuk membuat file `.exe` yang siap didistribusikan:
```bash
dx build --platform desktop --release
```
Hasil *build* akan berada di dalam direktori `dist`.

---

## 🌐 Versi 2: Aplikasi Web Sisi-Klien (HTML, CSS, JS)

Versi Web ini dirancang agar sangat mudah digunakan oleh siapa saja **tanpa perlu instalasi**. Semua pemrosesan (algoritma penghapusan *background* & penyesuaian kontras) dilakukan 100% di dalam browser menggunakan *Canvas API*.

**Kelebihan:**
- Tidak perlu server (Serverless).
- Privasi aman terjamin (gambar tidak pernah diunggah/di-*upload* ke internet).
- Langsung jalan di perangkat apapun yang memiliki browser modern.

### Cara Menjalankan
1. Buka folder `web/` di dalam proyek ini.
2. Klik ganda (double-click) pada file **`index.html`**.
3. File akan otomatis terbuka di browser Anda. Tersedia dukungan *Drag & Drop* untuk kemudahan penggunaan.

---

## 🛠️ Teknologi yang Digunakan

- **Rust**: Bahasa sistem untuk pemrosesan aplikasi desktop.
- **Dioxus (0.6)**: Framework UI lintas platform untuk versi desktop.
- **Crate `image` & `rfd`**: Untuk manipulasi data piksel dan dialog *open/save* file di Windows.
- **Vanilla JS & Canvas API**: Tenaga penggerak versi web ringan tanpa dependensi tambahan.
- **CSS3 Glassmorphism**: Untuk tampilan UI yang cantik dan modern.

---

*Dibuat untuk memudahkan digitalisasi tanda tangan secara instan.*
