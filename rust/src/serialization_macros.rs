// CardanoError can't be used by non-wasm targets so we use this macro to expose
// either a DeserializeError or a CardanoError error depending on if we're on a
// wasm or a non-wasm target where CardanoError is not available (it panics!).
// Note: wasm-bindgen doesn't support macros inside impls, so we have to wrap these
//       in their own impl and invoke the invoke the macro from global scope.
// TODO: possibly write s generic version of this for other usages (e.g. PrivateKey, etc)
#[macro_export]
macro_rules! from_bytes {
    // Custom from_bytes() code
    ($name:ident, $data: ident, $body:block) => {
        // wasm-exposed CardanoError return - CardanoError panics when used outside wasm
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        #[wasm_bindgen]
        impl $name {
            pub fn from_bytes($data: Vec<u8>) -> Result<$name, CardanoError> {
                Ok($body?)
            }
        }
        // non-wasm exposed DeserializeError return
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        impl $name {
            pub fn from_bytes($data: Vec<u8>) -> Result<$name, DeserializeError> $body
        }
    };
    // Uses Deserialize trait to auto-generate one
    ($name:ident) => {
        from_bytes!($name, bytes, {
            let mut raw = Deserializer::from(std::io::Cursor::new(bytes));
            Self::deserialize(&mut raw)
        });
    };
}

// There's no need to do wasm vs non-wasm as this call can't fail but
// this is here just to provide a default Serialize-based impl
// Note: Once again you can't use macros in impls with wasm-bindgen
//       so make sure you invoke this outside of one
#[macro_export]
macro_rules! to_bytes {
    ($name:ident) => {
        #[wasm_bindgen]
        impl $name {
            pub fn to_bytes(&self) -> Vec<u8> {
                let mut buf = Serializer::new_vec();
                self.serialize(&mut buf).unwrap();
                buf.finalize()
            }
        }
    };
}

#[macro_export]
macro_rules! from_hex {
    // Custom from_bytes() code
    ($name:ident, $data: ident, $body:block) => {
        // wasm-exposed CardanoError return - CardanoError panics when used outside wasm
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        #[wasm_bindgen]
        impl $name {
            pub fn from_hex($data: &str) -> Result<$name, CardanoError> {
                match hex::decode($data) {
                    Ok(_) => Ok($body?),
                    Err(e) => Err(CardanoError::new(&e.to_string()))
                }

            }
        }
        // non-wasm exposed DeserializeError return
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        impl $name {
            pub fn from_hex($data: &str) -> Result<$name, DeserializeError> $body
        }
    };
    // Uses Deserialize trait to auto-generate one
    ($name:ident) => {
        from_hex!($name, hex_str, {
            let mut raw = Deserializer::from(std::io::Cursor::new(hex::decode(hex_str).unwrap()));
            Self::deserialize(&mut raw)
        });
    };
}

#[macro_export]
macro_rules! to_hex {
    ($name:ident) => {
        #[wasm_bindgen]
        impl $name {
            pub fn to_hex(&self) -> String {
                let mut buf = Serializer::new_vec();
                self.serialize(&mut buf).unwrap();
                hex::encode(buf.finalize())
            }
        }
    };
}

#[macro_export]
macro_rules! to_from_bytes {
    ($name:ident) => {
        to_bytes!($name);
        from_bytes!($name);
        to_hex!($name);
        from_hex!($name);
    };
}
