use crate::*;

use serde;
use serde::ser::SerializeStruct;

struct U8Visitor(u8);

impl<'de> serde::de::Visitor<'de> for U8Visitor {
    type Value = u8;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a u8")
    }

    fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(value)
    }

    fn visit_u16<E>(self, value: u16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if value > u8::MAX.into() {
            return Err(serde::de::Error::custom(format!("u16 value {} is too large", value)));
        }
        Ok(value as u8)
    }

    fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if value > u8::MAX.into() {
            return Err(serde::de::Error::custom(format!("u32 value {} is too large", value)));
        }
        Ok(value as u8)
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if value > u8::MAX.into() {
            return Err(serde::de::Error::custom(format!("u64 value {} is too large", value)));
        }
        Ok(value as u8)
    }

    fn visit_u128<E>(self, value: u128) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if value > u8::MAX.into() {
            return Err(serde::de::Error::custom(format!("u128 value {} is too large", value)));
        }
        Ok(value as u8)
    }
}

impl serde::Serialize for Verbosity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // 2 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("Verbosity", 2)?;
        state.serialize_field("verbose", &self.verbose)?;
        state.serialize_field("quiet", &self.quiet)?;
        state.end()
    }
}

impl <'de> serde::Deserialize<'de> for Verbosity {

    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        enum Field { Verbose, Quiet }

        impl<'de> serde::de::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: serde::de::Deserializer<'de>,
            {
                struct FieldVisitor;

                impl <'de> serde::de::Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`verbose` or `quiet`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "verbose" => Ok(Field::Verbose),
                            "quiet" => Ok(Field::Quiet),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct VerbosityVisitor;

        impl<'de> serde::de::Visitor<'de> for VerbosityVisitor {
            type Value = Verbosity;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("`verbose` or `quiet`")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Verbosity, V::Error>
            where
                V: serde::de::SeqAccess<'de>,
            {
                let verbose = seq.next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                let quiet = seq.next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                Ok(Verbosity::new(verbose, quiet))
            }

            fn visit_map<V>(self, mut map: V) -> Result<Verbosity, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut verbose = None;
                let mut quiet = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Verbose => {
                            if verbose.is_some() {
                                return Err(serde::de::Error::duplicate_field("verbose"));
                            }
                            verbose = Some(map.next_value()?);
                        }
                        Field::Quiet => {
                            if quiet.is_some() {
                                return Err(serde::de::Error::duplicate_field("quiet"));
                            }
                            quiet = Some(map.next_value()?);
                        }
                    }
                }
                let verbose = verbose.ok_or_else(|| serde::de::Error::missing_field("verbose"))?;
                let quiet = quiet.ok_or_else(|| serde::de::Error::missing_field("quiet"))?;
                Ok(Verbosity::new(verbose, quiet))
            }
        }

        const FIELDS: &'static [&'static str] = &["verbose", "quiet"];
        deserializer.deserialize_struct("Verbosity", FIELDS, VerbosityVisitor)
    }
}
