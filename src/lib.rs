pub mod bdg_operation {

    use super::bdg_record::{self, BdgRecord};
    use super::bed_record::{self, BedLikeRecord};
    impl From<BedLikeRecord> for BdgRecord {
        fn from(item: BedLikeRecord) -> BdgRecord {
            BdgRecord {
                chrom: item.chrom,
                start: item.start,
                end: item.end,
                // default score 0
                score: item.score.unwrap(),
            }
        }
    }
}

pub mod bdg_record {
    use serde::{Deserialize, Serialize};

    pub const BDG_HEADERS: [&str; 4] = ["chrom", "start", "end", "score"];

    #[derive(Debug, Serialize, Deserialize)]
    pub struct BdgRecord {
        pub chrom: String,
        pub start: u32,
        pub end: u32,
        pub score: f64,
    }
}

pub mod bed_record {
    use serde::de::{self, Visitor};
    use serde::{self, Deserialize, Serialize, Serializer};

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
        pub chrom: String,
        pub start: u32,
        pub end: u32,

        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,

        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub score: Option<f64>,

        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub strand: Option<Strand>,
    }
}
