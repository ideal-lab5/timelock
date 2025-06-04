/*
 * Copyright 2025 by Ideal Labs, LLC
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use ark_ec::PrimeGroup;
use ark_ff::UniformRand;
use ark_std::rand::rngs::OsRng;
use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use timelock::{
	block_ciphers::AESGCMBlockCipherProvider,
	engines::{EngineBLS, drand::TinyBLS381},
	ibe::fullident::*,
	tlock::*,
};

/// Encrypts a message for the identity
fn tlock_encrypt<E: EngineBLS>(
	msk: [u8; 32],
	p_pub: E::PublicKeyGroup,
	message: &[u8],
	id: Identity,
) -> TLECiphertext<E> {
	tle::<E, AESGCMBlockCipherProvider, OsRng>(p_pub, msk, message, id, OsRng).unwrap()
}

/// Decrypts a ciphertext using the provided signature
fn tlock_decrypt<E: EngineBLS>(ct: TLECiphertext<E>, sig: IBESecret<E>) -> Vec<u8> {
	tld::<E, AESGCMBlockCipherProvider>(ct, sig.0).unwrap()
}

/// Benchmarks encryption and decryption separately
fn tlock_split(c: &mut Criterion) {
	static KB: usize = 1024;
	let s = <TinyBLS381 as EngineBLS>::Scalar::rand(&mut OsRng);
	let p_pub = <TinyBLS381 as EngineBLS>::PublicKeyGroup::generator() * s;
	let id = Identity::new(b"", vec![b"test".to_vec()]);

	// Benchmark encryption
	let mut encrypt_group = c.benchmark_group("tlock_encrypt");
	for size in [KB, 2 * KB, 4 * KB, 8 * KB, 16 * KB, 128 * KB, 256 * KB].iter() {
		let mut dummy_data = Vec::with_capacity(*size);
		(0..*size).for_each(|i| dummy_data.push(i as u8));

		encrypt_group.throughput(Throughput::Bytes(*size as u64));
		encrypt_group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &_size| {
			b.iter(|| {
				tlock_encrypt::<TinyBLS381>(
					black_box([2; 32]),
					black_box(p_pub),
					black_box(&dummy_data),
					black_box(id.clone()),
				);
			});
		});
	}
	encrypt_group.finish();

	// Benchmark decryption
	let mut decrypt_group = c.benchmark_group("tlock_decrypt");
	for size in [KB, 2 * KB, 4 * KB, 8 * KB, 16 * KB, 128 * KB, 256 * KB].iter() {
		let mut dummy_data = Vec::with_capacity(*size);
		(0..*size).for_each(|i| dummy_data.push(i as u8));

		decrypt_group.throughput(Throughput::Bytes(*size as u64));
		decrypt_group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &_size| {
			b.iter_batched(
				|| 	// Pre-encrypt the data for decryption benchmark
						tlock_encrypt::<TinyBLS381>(
							[2; 32],
							p_pub,
							&dummy_data,
							id.clone(),
						),
				|ct| {
					tlock_decrypt::<TinyBLS381>(black_box(ct), black_box(id.extract(s)));
				},
				criterion::BatchSize::SmallInput,
			);
		});
	}
	decrypt_group.finish();
}

criterion_group!(benches, tlock_split);
criterion_main!(benches);
