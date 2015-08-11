pub trait BlockCipherOperation {
	fn encrypt(&mut self) -> Vec<u8>;
	fn decrypt(&mut self) -> Vec<u8>;
}