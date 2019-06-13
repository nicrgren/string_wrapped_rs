use std::{cmp, fmt, str::FromStr};

pub struct StringWrapped<T>(pub T);

impl<T> AsRef<T> for StringWrapped<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> std::ops::Deref for StringWrapped<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> fmt::Debug for StringWrapped<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StringWrapped({:?})", self.0)
    }
}

impl<T> fmt::Display for StringWrapped<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StringWrapped({})", self.0)
    }
}

impl<T> Clone for StringWrapped<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        StringWrapped(self.0.clone())
    }
}

// impl<T, Rhs> cmp::PartialEq<Rhs> for StringWrapped<T>
// where
//     T: cmp::PartialEq<Rhs>,
// {
//     fn eq(&self, other: &Rhs) -> bool {
//         &self.0 == other
//     }
// }

impl<T> cmp::PartialEq for StringWrapped<T>
where
    T: cmp::PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> cmp::Eq for StringWrapped<T> where T: cmp::Eq {}

impl<T> PartialOrd for StringWrapped<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T> cmp::Ord for StringWrapped<T>
where
    T: cmp::Ord,
{
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.0.cmp(other)
    }
}

impl<'de, T> serde::Deserialize<'de> for StringWrapped<T>
where
    T: FromStr,
    <T as FromStr>::Err: fmt::Display,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer
            .deserialize_str(Visitor {
                _marker: std::marker::PhantomData::<T>,
            })
            .map(StringWrapped)
    }
}

struct Visitor<T>
where
    T: FromStr,
    T::Err: fmt::Display,
{
    _marker: std::marker::PhantomData<T>,
}

impl<'de, T> serde::de::Visitor<'de> for Visitor<T>
where
    T: FromStr,
    <T as FromStr>::Err: fmt::Display,
{
    type Value = T;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string that can be parsed as the given type")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        T::from_str(value).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_compare() {
        StringWrapped(23) == StringWrapped(23);
    }

    #[test]
    fn test_ord() {
        StringWrapped(23) < StringWrapped(23);
    }

    #[test]
    fn deserialize_number() {
        assert_eq!(
            StringWrapped(23),
            serde_json::from_str::<StringWrapped<i32>>(r#""23""#).expect("Deserializing")
        );
    }
}
