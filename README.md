# Obsidian-512

Obsidian-512 is a high-performance, experimental 512-bit hashing algorithm developed in Rust. The architecture focuses on high-entropy bit diffusion and non-linear transformation logic to ensure maximum data integrity and uniqueness.

## SECURITY WARNING

DO NOT USE THIS ALGORITHM FOR CRYPTOCURRENCY, PASSWORD HASHING, OR SENSITIVE DATA. This project is currently in an experimental and developmental stage. It has not been peer-reviewed by cryptographic experts or subjected to formal security audits. Use it only for educational purposes, file integrity checks, or non-critical applications.

## Performance and Statistical Stress Test Results

Obsidian-512 has been subjected to rigorous statistical stress tests, including large-scale data processing and SMHasher-inspired avalanche analysis, to verify its bit distribution characteristics.

### Avalanche Effect and Large-Scale Verification
- **1 GB Data Test:** Verified using 1,073,741,824 bytes of data. A single bit flip (1/8,589,934,592 ratio) resulted in a **49.41%** deviation, demonstrating near-perfect diffusion across massive datasets.
- **Small String Test:** Changing 1 bit in a 30-character string resulted in an average of **259 of 512 bits** flipped (**50.59%**), meeting the ideal cryptographic threshold.

### Differential Resistance
- **Test:** Comparing hashes of consecutive integers (e.g., 1000 vs 1001).
- **Result:** Output bits remained **51.1%** different despite near-identical inputs, proving high resistance to linear analysis (262 bits difference).

### Sparse Bits and Low Entropy Integrity
- **Test:** Processing a 512-bit null block versus a 512-bit block containing a single '1' bit.
- **Result:** The algorithm maintains chaotic integrity (**253 bits** flipped) even when processing data with extremely low information density.

### Large-Scale Stability
- **Test:** Successfully processed a 208,989-character Fibonacci sequence with perfect diffusion and zero collisions.

## Features

Obsidian-512 provides four versatile methods to handle different data types:

### 1. File Hashing
`obsidian_512::hash_file("path/to/file.ext");`
Computes the hash of an entire file by processing the raw byte stream directly from the disk.

### 2. String Hashing
`obsidian_512::hash_str("your_text_here");`
Generates a 512-bit digest from any UTF-8 string input.

### 3. Number Hashing
`obsidian_512::hash_number(123456789);`
Directly hashes numeric values (supports up to u128) by processing their raw big-endian byte representation.

### 4. Raw Byte Hashing
`obsidian_512::hash_bytes(&[u8]);`
The core engine that directly processes raw byte arrays for maximum flexibility and performance.

## Security & Architecture

- **Nothing-Up-My-Sleeve Constants:** Uses mathematically derived, irrational constants (Euler’s number e, π, and square roots of prime numbers) to ensure a provably neutral foundation.
- **128-Round Transformation:** Each input block undergoes 128 rounds of aggressive bit-mixing, providing significantly more diffusion cycles than many industrial standards.
- **Data-Dependent Rotation (DDR):** Employs `count_ones` logic for non-linear bit shifting. The rotation amount is tied to the data itself, making traditional linear cryptanalysis difficult.
- **512-Bit Output:** Produces a massive search space of 2^512 combinations, represented as a 128-character hexadecimal string.

## License

This project is open-source. Please refer to the LICENSE file for more details.

---
*Developer: twitchozel@gmail.com*