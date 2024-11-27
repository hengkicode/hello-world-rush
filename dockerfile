# Menggunakan base image Rust official
FROM rust:latest

# Set working directory di dalam container
WORKDIR /usr/src/app

# Salin semua file dari direktori lokal ke dalam container
COPY . .

# Install dependencies dan build aplikasi dalam mode release
RUN cargo build --release

# Jalankan aplikasi setelah build dalam mode release
CMD ["./target/release/hello-world"]
