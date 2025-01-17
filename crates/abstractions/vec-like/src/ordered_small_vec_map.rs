use crate::{
    error::{FromVecEntryRepeatError, InsertEntryRepeatError},
    *,
};
use smallvec::{smallvec, Array, SmallVec};

#[derive(PartialEq, Eq, Clone, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct OrderedSmallVecMap<E, const N: usize>
where
    [E; N]: Array<Item = E>,
{
    entries: SmallVec<[E; N]>,
}

impl<K, Entry, const M: usize, const N: usize> From<[Entry; M]> for OrderedSmallVecMap<Entry, N>
where
    K: Ord + Copy + std::fmt::Debug,
    Entry: AsVecMapEntry<K = K> + std::fmt::Debug,
    [Entry; M]: Array<Item = Entry>,
    [Entry; N]: Array<Item = Entry>,
{
    fn from(value: [Entry; M]) -> Self {
        let iter: std::array::IntoIter<_, M> = value.into_iter();
        Self::from_iter(iter)
    }
}

#[test]
fn ordered_small_vec_map_from_slice_works() {
    type Entry = (i32, i32);

    // construct a map from input and assert its entries being equal to entries_expected
    fn t<const N: usize>(input: [Entry; N], entries_expected: &[Entry]) {
        let map = OrderedSmallVecMap::<_, N>::from(input);
        assert_eq!(map.entries.as_slice(), entries_expected);
    }

    t([], &[]);
    t([(1, 2)], &[(1, 2)]);
    t([(1, 2), (3, 4)], &[(1, 2), (3, 4)]);
    t([(3, 4), (1, 2)], &[(1, 2), (3, 4)]);
    t([(2, 3), (1, 2), (3, 4)], &[(1, 2), (2, 3), (3, 4)]);
}

#[test]
#[should_panic]
fn ordered_small_vec_map_from_slice_fails_on_duplication() {
    type Entry = (i32, i32);

    // construct a map from input and assert its entries being equal to entries_expected
    fn t<const N: usize>(input: [Entry; N]) {
        let _map = OrderedSmallVecMap::<_, N>::from(input);
    }

    t([(1, 2), (1, 2)]);
}

impl<K, E, const N: usize> FromIterator<E> for OrderedSmallVecMap<E, N>
where
    K: Ord + Copy + std::fmt::Debug,
    E: AsVecMapEntry<K = K> + std::fmt::Debug,
    [E; N]: Array<Item = E>,
{
    fn from_iter<T: IntoIterator<Item = E>>(iter: T) -> Self {
        let mut map = Self::default();
        for v in iter {
            map.insert_new(v).unwrap()
        }
        map
    }
}

impl<E, const N: usize> IntoIterator for OrderedSmallVecMap<E, N>
where
    [E; N]: Array<Item = E>,
{
    type Item = E;

    type IntoIter = impl Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}

#[test]
fn ordered_small_vec_map_from_iter_works() {
    type Entry = (i32, i32);

    // construct a map from input and assert its entries being equal to entries_expected
    fn t<const N: usize>(input: [Entry; N], entries_expected: &[Entry]) {
        let map: OrderedSmallVecMap<_, N> = OrderedSmallVecMap::from_iter(input);
        assert_eq!(map.entries.as_slice(), entries_expected);
    }

    t([], &[]);
    t([(1, 2)], &[(1, 2)]);
    t([(1, 2), (3, 4)], &[(1, 2), (3, 4)]);
    t([(3, 4), (1, 2)], &[(1, 2), (3, 4)]);
    t([(2, 3), (1, 2), (3, 4)], &[(1, 2), (2, 3), (3, 4)]);
}

#[test]
#[should_panic]
fn ordered_small_vec_map_from_iter_fails_on_duplication() {
    type Entry = (i32, i32);

    // construct a map from input and assert its entries being equal to entries_expected
    fn t<const N: usize>(input: [Entry; N]) {
        let _map = OrderedSmallVecMap::<_, N>::from_iter(input);
    }

    t([(1, 2), (1, 2)]);
}

impl<V, const N: usize> std::convert::AsRef<[V]> for OrderedSmallVecMap<V, N>
where
    [V; N]: Array<Item = V>,
{
    fn as_ref(&self) -> &[V] {
        &self.entries
    }
}

impl<'a, E, const N: usize> IntoIterator for &'a OrderedSmallVecMap<E, N>
where
    [E; N]: Array<Item = E>,
{
    type Item = &'a E;

    type IntoIter = impl Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        (&self.entries).into_iter()
    }
}

impl<E, const N: usize> std::fmt::Debug for OrderedSmallVecMap<E, N>
where
    E: std::fmt::Debug,
    [E; N]: Array<Item = E>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.entries.fmt(f)
    }
}

impl<K, E, const N: usize> OrderedSmallVecMap<E, N>
where
    E: AsVecMapEntry<K = K>,
    [E; N]: Array<Item = E>,
{
    pub fn clear(&mut self) {
        self.entries.clear()
    }

    pub fn new_one_element_map(entry: E) -> Self {
        Self {
            entries: smallvec![entry],
        }
    }

    pub fn take_data(self) -> SmallVec<[E; N]> {
        self.entries
    }
    pub fn data(&self) -> &[E] {
        &self.entries
    }
    pub unsafe fn data_mut(&mut self) -> &mut [E] {
        &mut self.entries
    }

    pub fn from_smallvec(data: SmallVec<[E; N]>) -> Result<Self, FromVecEntryRepeatError>
    where
        K: Copy + Eq,
    {
        for i in 0..data.len() {
            for j in (i + 1)..data.len() {
                if data[i].key() == data[j].key() {
                    return Err(FromVecEntryRepeatError { i, j });
                }
            }
        }
        Ok(Self { entries: data })
    }

    pub unsafe fn from_smallvec_unchecked(entries: SmallVec<[E; N]>) -> Self {
        Self { entries }
    }

    pub unsafe fn from_iter_unchecked(into_iter: impl IntoIterator<Item = E>) -> Self {
        Self {
            entries: into_iter.into_iter().collect(),
        }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn get_entry(&self, key: K) -> Option<&E>
    where
        K: Copy + Eq,
    {
        self.entries.iter().find(|entry| entry.key() == key)
    }

    pub unsafe fn get_entry_mut(&mut self, key: K) -> Option<&mut E>
    where
        K: Copy + Eq,
    {
        self.entries.iter_mut().find(|entry| entry.key() == key)
    }

    pub fn iget_entry(&self, key: K) -> Option<(usize, &E)>
    where
        K: Copy + Eq,
    {
        self.entries
            .iter()
            .enumerate()
            .find(|(_, entry)| entry.key() == key)
    }

    pub fn has(&self, key: K) -> bool
    where
        K: Copy + Eq,
    {
        self.entries
            .iter()
            .find(|entry| entry.key() == key)
            .is_some()
    }

    pub fn keys<'a>(&'a self) -> impl Iterator<Item = K> + 'a
    where
        K: Copy,
    {
        self.entries.iter().map(|entry| entry.key())
    }

    pub unsafe fn get_mut(&mut self, key: K) -> Option<&mut E>
    where
        K: Copy + Eq,
    {
        self.entries.iter_mut().find(|entry| entry.key() == key)
    }

    pub fn insert_new(&mut self, new: E) -> Result<(), InsertEntryRepeatError<E>>
    where
        K: Ord + Copy,
    {
        let key = new.key();
        match self.entries.binary_search_by(|e| e.key().cmp(&key)) {
            Ok(old) => Err(InsertEntryRepeatError { old, new }),
            Err(index) => Ok(self.entries.insert(index, new)),
        }
    }

    pub fn insert(&mut self, new: E)
    where
        K: Ord + Copy,
    {
        let key = new.key();
        match self.entries.binary_search_by(|e| e.key().cmp(&key)) {
            Ok(old) => (),
            Err(index) => self.entries.insert(index, new),
        }
    }

    pub fn insert_with_or_update(
        &mut self,
        key: K,
        new_entry: impl FnOnce() -> E,
        update: impl FnOnce(&mut E),
    ) where
        K: Ord + Copy + std::fmt::Debug,
    {
        match self.entries.binary_search_by(|e| e.key().cmp(&key)) {
            Ok(old) => {
                update(&mut self.entries[old]);
                debug_assert_eq!(self.entries[old].key(), key);
            }
            Err(index) => self.entries.insert(index, new_entry()),
        }
    }

    pub fn insert_from_ref(&mut self, new_entry: &E)
    where
        E: Clone,
        K: Ord + Copy,
    {
        let key = new_entry.key();
        match self.entries.binary_search_by(|e| e.key().cmp(&key)) {
            Ok(old) => (),
            Err(index) => self.entries.insert(index, new_entry.clone()),
        }
    }

    pub fn position(&self, key: K) -> Option<usize>
    where
        K: Copy + Eq,
    {
        self.entries.iter().position(|entry| entry.key() == key)
    }

    pub fn extend(&mut self, iter: impl Iterator<Item = E>) -> Result<(), InsertEntryRepeatError<E>>
    where
        K: Ord + Copy,
    {
        for v in iter {
            self.insert_new(v)?
        }
        Ok(())
    }

    pub fn extend_from_other(&mut self, other: Self) -> Result<(), InsertEntryRepeatError<E>>
    where
        K: Ord + Copy,
    {
        for v in other.entries {
            self.insert_new(v)?
        }
        Ok(())
    }

    pub fn extend_from_ref(&mut self, other: &Self)
    where
        E: Clone,
        K: Ord + Copy,
    {
        for entry in &other.entries {
            self.insert_from_ref(entry)
        }
    }

    pub fn toggle(&mut self, key: K)
    where
        E: DefaultVecMapEntry<K>,
        K: Ord + Copy,
    {
        if let Some(position) = self.entries.iter().position(|entry| entry.key() == key) {
            self.entries.remove(position);
        } else {
            self.insert(E::default_from_key(key))
        }
    }

    pub fn remove(&mut self, key: K) -> Option<E>
    where
        K: Ord + Copy,
    {
        if let Some(position) = self.entries.iter().position(|entry| entry.key() == key) {
            Some(self.entries.remove(position))
        } else {
            None
        }
    }

    /// `f` should preserve the key
    #[inline(always)]
    pub fn map_collect_on_entries<E2>(&self, f: impl Fn(&E) -> E2) -> OrderedSmallVecMap<E2, N>
    where
        K: Copy + Eq + std::fmt::Debug,
        E2: AsVecMapEntry<K = K>,
        [E2; N]: Array<Item = E2>,
    {
        OrderedSmallVecMap {
            entries: self
                .entries
                .iter()
                .map(|entry| {
                    let mapped_entry = f(entry);
                    debug_assert_eq!(entry.key(), mapped_entry.key());
                    mapped_entry
                })
                .collect(),
        }
    }

    /// `f` should preserve the key
    #[inline(always)]
    pub fn map_into_collect_on_entries<E2>(self, f: impl Fn(E) -> E2) -> OrderedSmallVecMap<E2, N>
    where
        K: Copy + Eq + std::fmt::Debug,
        E2: AsVecMapEntry<K = K>,
        [E2; N]: Array<Item = E2>,
    {
        OrderedSmallVecMap {
            entries: self
                .entries
                .into_iter()
                .map(|entry| {
                    let prev_key = entry.key();
                    let mapped_entry = f(entry);
                    debug_assert_eq!(prev_key, mapped_entry.key());
                    mapped_entry
                })
                .collect(),
        }
    }

    pub unsafe fn entries_mut(&mut self) -> &mut SmallVec<[E; N]> {
        &mut self.entries
    }
}

impl<K, V, const N: usize> OrderedSmallVecPairMap<K, V, N>
where
    [(K, V); N]: Array<Item = (K, V)>,
{
    pub fn get_value<Borrowed: ?Sized>(&self, key: K) -> Option<&Borrowed>
    where
        K: Eq + Copy,
        V: std::borrow::Borrow<Borrowed>,
    {
        self.get_entry(key).map(|(_, v)| v.borrow())
    }

    #[inline(always)]
    pub fn get_value_mut_or_insert_default(&mut self, key: K) -> &mut V
    where
        K: Copy + PartialEq,
        V: Default,
    {
        match self.entries.iter_mut().find(|(key1, _)| *key1 == key) {
            Some(entry) => unsafe { husky_wild_utils::arb_mut(&mut entry.1) },
            None => {
                self.entries.push((key, V::default()));
                &mut unsafe { self.entries.last_mut().unwrap_unchecked() }.1
            }
        }
    }

    #[inline(always)]
    pub fn get_value_mut_or_insert_with(&mut self, key: K, f: impl FnOnce() -> V) -> &mut V
    where
        K: Copy + PartialEq,
    {
        match self.entries.iter_mut().find(|(key1, _)| *key1 == key) {
            Some(entry) => unsafe { husky_wild_utils::arb_mut(&mut entry.1) },
            None => {
                self.entries.push((key, f()));
                &mut unsafe { self.entries.last_mut().unwrap_unchecked() }.1
            }
        }
    }

    #[inline(always)]
    pub fn update_value_or_insert(&mut self, key: K, update: impl FnOnce(&mut V), v: V)
    where
        K: Copy + PartialEq,
    {
        match self.entries.iter_mut().find(|(key1, _)| *key1 == key) {
            Some(entry) => update(&mut entry.1),
            None => self.entries.push((key, v)),
        }
    }

    #[inline(always)]
    pub fn update_value_or_insert_with(
        &mut self,
        key: K,
        update: impl FnOnce(&mut V),
        f: impl FnOnce() -> V,
    ) where
        K: Copy + PartialEq,
    {
        match self.entries.iter_mut().find(|(key1, _)| *key1 == key) {
            Some(entry) => update(&mut entry.1),
            None => self.entries.push((key, f())),
        }
    }

    #[inline(always)]
    pub fn map_collect<U>(&self, f: impl Fn(&V) -> U) -> OrderedSmallVecPairMap<K, U, N>
    where
        K: Copy,
        [(K, U); N]: Array<Item = (K, U)>,
    {
        OrderedSmallVecPairMap {
            entries: self.entries.iter().map(|(k, v)| (*k, f(v))).collect(),
        }
    }

    #[inline(always)]
    pub fn map_collect2<U>(&self, f: impl Fn(K, &V) -> U) -> OrderedSmallVecPairMap<K, U, N>
    where
        K: Copy,
        [(K, U); N]: Array<Item = (K, U)>,
    {
        OrderedSmallVecPairMap {
            entries: self
                .entries
                .iter()
                .map(|&(k, ref v)| (k, f(k, v)))
                .collect(),
        }
    }
}

impl<E, const N: usize> Deref for OrderedSmallVecMap<E, N>
where
    E: AsVecMapEntry,
    [E; N]: Array<Item = E>,
{
    type Target = [E];

    fn deref(&self) -> &Self::Target {
        &self.entries
    }
}

impl<E, const N: usize> Default for OrderedSmallVecMap<E, N>
where
    [E; N]: Array<Item = E>,
{
    fn default() -> Self {
        Self {
            entries: smallvec![],
        }
    }
}

impl<K, E, const N: usize> std::ops::Index<K> for OrderedSmallVecMap<E, N>
where
    K: Eq + Copy,
    E: AsVecMapEntry<K = K>,
    [E; N]: Array<Item = E>,
{
    type Output = E;

    fn index(&self, index: K) -> &Self::Output {
        self.get_entry(index).unwrap()
    }
}

// impl<K, E, const N: usize> std::ops::IndexMut<K> for OrderedSmallVecMap<E, N>
// where
//     K: PartialEq + Eq + Copy + std::fmt::Debug,
//     E: AsVecMapEntry<K = K>,
//     [E; N]: Array<Item = E>,
// {
//     fn index_mut(&mut self, index: K) -> &mut Self::Output {
//         self.get_mut(index).unwrap()
//     }
// }

pub type OrderedSmallVecPairMap<K, V, const N: usize> = OrderedSmallVecMap<(K, V), N>;
