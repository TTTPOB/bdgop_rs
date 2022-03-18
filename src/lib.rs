pub mod bdg_record {
    use serde::{Deserialize, Serialize};
    use serde_json::Number;

    pub const BDG_HEADERS: [&str; 4] = ["chrom", "start", "end", "score"];

    #[derive(Debug, Serialize, Deserialize)]
    pub struct BdgRecord {
        chrom: String,
        start: u32,
        end: u32,
        score: Number,
    }
}

pub mod bed_record {
    use serde::de::{self, Visitor};
    use serde::{self, Deserialize, Serialize, Serializer};
    use serde_json::Number;

    #[derive(Debug)]
    pub enum Strand {
        Plus,
        Minus,
        Unknown,
    }

    struct StrandVisitor;
    impl Serialize for Strand {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Strand::Plus => serializer.serialize_char('+'),
                Strand::Minus => serializer.serialize_char('-'),
                Strand::Unknown => serializer.serialize_char('.'),
            }
        }
    }
    // implementing deserialize for Strand
    impl<'de> Visitor<'de> for StrandVisitor {
        type Value = Strand;
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a char, one of '+', '-' or '.'")
        }
        fn visit_char<E>(self, ch: char) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            match ch {
                '+' => Ok(Strand::Plus),
                '-' => Ok(Strand::Minus),
                '.' => Ok(Strand::Unknown),
                _ => Err(de::Error::invalid_value(de::Unexpected::Char(ch), &self)),
            }
        }
    }
    impl<'de> Deserialize<'de> for Strand {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            deserializer.deserialize_char(StrandVisitor)
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct BedLikeRecord {
        chrom: String,
        start: u32,
        end: u32,

        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,

        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        score: Option<Number>,

        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        strand: Option<Strand>,
    }
}
