#[cfg(feature = "strum")]
use bevy_utils::hashbrown::HashMap;
#[cfg(feature = "strum")]
use strum::IntoEnumIterator;

/// Validates that a given hash map contains exactly one of each possible key as defined by the key type `K`.
///
/// This function checks whether the provided `hash_map` contains exactly one of each possible key.
/// It's particularly useful for ensuring that a `HashMap` is fully populated with no missing or extra elements
/// compared to a known list of keys. This might be the case in configurations or states that require a 
/// representative value for every possible key.
///
/// # Type Parameters
///
/// * `K`: The type of the keys in the `HashMap`. It must satisfy the following conditions:
///   * `Eq`: Allows comparing keys for equality.
///   * `std::hash::Hash`: Necessary for the keys to be hashed, a requirement in a `HashMap`.
///   * `Copy`: Indicates that the keys can be copied, which is used in iterating through keys.
///   * `IntoEnumIterator`: Provides an iterator over all possible values of `K`.
/// * `V`: The type of the values in the `HashMap`. There are no specific trait bounds for `V` in this function.
///
/// # Parameters
///
/// * `hash_map`: A reference to the hash map of key-value pairs to be validated.
///
/// # Returns
///
/// Returns `true` if the `hash_map` contains exactly one of each possible key (as defined by the key type `K`),
/// and no more. Otherwise, it returns `false`.
///
/// # Examples
///
/// ```rust
/// enum LevelState {
///     Level1,
///     Level2,
/// }
/// 
/// fn load_level_1(/* ... */) {
/// // ...
/// }
/// 
/// fn load_level_2(/* ... */) {
/// // ...
/// }
/// 
/// fn main() {
///     let mut hash_map = HashMap::new();
///     hash_map.insert(State::State1, load_level_1);
///     hash_map.insert(State::State2, load_level_2);
///     
///     use bevy_hectic_utils::hashmap::*;
///     assert!(validate_hash_map(hash_map)); // Returns true
/// 
///     // Code where you sure that the hash map contains exactly one of each possible key ...
/// }
/// ```
#[cfg(feature = "strum")]
pub fn validate_hash_map<K, V>(hash_map: &HashMap<K, V>) -> bool
where
    K: Eq + std::hash::Hash + Copy + IntoEnumIterator,
    K::Iterator: Iterator<Item = K>,
{
    let all_keys = K::iter().collect::<Vec<_>>();
    if hash_map.len() != all_keys.len() {
        return false;
    }

    for key in all_keys {
        if !hash_map.contains_key(&key) {
            return false;
        }
    }

    true
}

/// Asserts that a given hash map contains exactly one of each possible key.
///
/// This macro is a convenience wrapper around the [`validate_hash_map`] function, intended to be used in 
/// tests or other scenarios where you want to ensure that a `HashMap` is fully populated with no missing 
/// or extra elements and panic otherwise. It's equivalent to `assert!(validate_hash_map(hash_map));`.
///
/// # Usage
///
/// ```rust
/// use bevy_hectic_utils::validate_hash_map;
/// 
/// validate_hash_map!(hash_map);
/// ```
///
/// # Panics
///
/// Panics if the `hash_map` does not contain exactly one of each possible key
/// or if the `hash_map` contains more than one of any key.
///
/// # Examples
///
/// ```rust
/// enum LevelState {
///     Level1,
///     Level2,
/// }
/// 
/// fn load_level_1(/* ... */) {
/// // ...
/// }
/// 
/// fn load_level_2(/* ... */) {
/// // ...
/// }
/// 
/// fn main() {
///     let mut hash_map = HashMap::new();
///     hash_map.insert(State::State1, load_level_1);
///     hash_map.insert(State::State2, load_level_2);
///     
///     use bevy_hectic_utils::hashmap::*;
///     validate_hash_map!(hash_map); // Returns true
/// 
///     // Code where you sure that the hash map contains exactly one of each possible key ...
/// }
/// ```
#[macro_export]
#[cfg(feature = "strum")]
macro_rules! validate_hash_map {
    ($hash_map:expr) => {
        assert!(validate_hash_map($hash_map));
    };
}

/// Creates a [`HashMap`](bevy_utils::HashMap) using Bevy's hash maps for increased speed with less security.
///
/// This macro initializes a [`HashMap`](bevy_utils::HashMap) with the specified key-value pairs. It is called "no secure" (ns)
/// because it prioritizes performance potentially at the cost of certain security measures found
/// in other hash maps. It's a convenient way to quickly create a populated [`HashMap`](bevy_utils::HashMap).
///
/// # Examples
///
/// ```rust
/// use bevy_hectic_utils::ns_hashmap;
/// use bevy_utils::HashMap;
/// 
/// let fruits = ns_hashmap!{
///     "apple" => 1,
///     "banana" => 2
/// };
/// 
/// // `fruits` is now a HashMap containing {"apple": 1, "banana": 2}
/// ```
///
/// # Notes
///
/// - This macro is a simple and performant way to instantiate a hash map, but it should not be used
///   in security-sensitive contexts due to its lack of secure hashing.
///
#[macro_export]
macro_rules! ns_hashmap {
    ($( $key: expr => $val: expr ),*) => {{
        let mut map = HashMap::new();
        $(
            map.insert($key, $val);
        )*
        map
    }};
}

/// Creates a [`HashMap`](`std::collections::HashMap`) from a list of key-value pairs.
///
/// This macro simplifies the creation of a [`HashMap`](`std::collections::HashMap`) by allowing inline definition of key-value pairs.
/// It initializes a [`HashMap`](`std::collections::HashMap`) using Rust's standard [`std::collections::HashMap`] and inserts the specified
/// pairs into the map. It's a convenient way to quickly create and populate a [`HashMap`](`std::collections::HashMap`).
///
/// # Examples
///
/// ```rust
/// use bevy_hectic_utils::hashmap;
/// use bevy_utils::HashMap;
/// 
/// let capitals = hashmap!{
///     "France" => "Paris",
///     "Spain" => "Madrid"
/// };
/// 
/// // `capitals` is now a HashMap containing {"France": "Paris", "Spain": "Madrid"}
/// ```
///
/// # Notes
///
/// - This macro creates a mutable `HashMap` and fills it with the provided key-value pairs.
/// - It is a shorthand for manually creating a `HashMap` and inserting each key and value.
///
#[macro_export]
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
        let mut map = std::hashmap::HashMap::new();
        $(
            map.insert($key, $val);
        )*
        map
    }};
}


#[cfg(test)]
pub mod test {
    use std::{time::{Duration, Instant}, ops::{Deref, DerefMut}};

    use log::Level;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Times(u64);

    impl Deref for Times {
        type Target = u64;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    
    impl DerefMut for Times {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl Into<u64> for Times {
        fn into(self) -> u64 {
            self.0
        }
    }

    impl Into<usize> for Times {
        fn into(self) -> usize {
            self.0 as usize
        }
    }

    impl Into<u32> for Times {
        fn into(self) -> u32 {
            self.0 as u32
        }
    }

    impl Into<i32> for Times {
        fn into(self) -> i32 {
            self.0 as i32
        }
    }

    impl From<u64> for Times {
        fn from(times: u64) -> Self {
            Self(times)
        }
    }

    impl From<usize> for Times {
        fn from(times: usize) -> Self {
            Self(times as u64)
        }
    }

    impl From<u32> for Times {
        fn from(times: u32) -> Self {
            Self(times as u64)
        }
    }

    impl From<i32> for Times {
        fn from(times: i32) -> Self {
            Self(times as u64)
        }
    }

    impl Default for Times {
        /// Value that may be enough for most cases
        fn default() -> Self {
            Self(100000)
        }
    }

    /// Enable logging for debug
    pub fn enable_loggings() {
        use std::env;
        use std::io::Write;

        let _ = env::set_var("RUST_LOG", "debug");
        // FIXME: colorize logs
        // TODO: colorize thorwed args
        let _ = env_logger::builder()
            .is_test(true)
            .format(|buf, record| {
                let mut style = buf.style();
                let level = record.level();
                match level {
                    Level::Trace => style.set_color(env_logger::fmt::Color::Magenta),
                    Level::Debug => style.set_color(env_logger::fmt::Color::Blue),
                    Level::Info => style.set_color(env_logger::fmt::Color::Green),
                    Level::Warn => style.set_color(env_logger::fmt::Color::Yellow),
                    Level::Error => style.set_color(env_logger::fmt::Color::Red),
                };

                writeln!(buf, "{}: {}", style.value(level), record.args())
            })
            .try_init();
    }

    /// Measure time of predicate
    pub fn measure_time<F: Copy>(predicate: F, times: Times) -> Duration
    where
        F: FnOnce() -> (),
    {
        let start = Instant::now();
        for _ in 0..times.clone().into() {
            predicate();
        }
        let global_duration = start.elapsed();
        global_duration / times.into()
    }
}