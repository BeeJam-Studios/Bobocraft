use serde::de::{self, IntoDeserializer, Visitor};
use serde::forward_to_deserialize_any;
use serde::{Deserialize, Serialize};
use serde_variant::to_variant_name;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::hash::Hash;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Category {
    Weapon,
    Module,
    Movement,
    Armor,
    Cosmetic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "lowercase"))]
pub enum Ident {
    // Weapons
    Laser,
    Plasma,
    Rail,
    Nano,
    Tesla,
    Seeker,
    AeroFlak,
    Chaingun,
    Ion,
    Mortar,
    // Modules
    Module,
    // Movement
    Wheel,
    Tank,
    Mech,
    Sprinter,
    Insect,
    Hover,
    Rotor,
    Wing,
    Rudder,
    Thruster,
    Propeller,
    Ski,
    // Armor
    Armored,
    Medium,
    Heavy,
    Light,
    Compact,
    Cardboard,
    Neon,
    Glass,
    Retro,
    Helium,
    Rod,
    Strut,
    ElectroShield,
    //Cosmetics
    Medal,
    League,
    Badge,
    Altimeter,
    Speedometer,
    Headlamp,
    Robot,
    Name,
    Banner,
    PilotSeat,
    BubbleBlower,
    Balloon,
    Vapor,
    Trail,
    Eyeball,
    Spike,
    Present,
    Fairy,
    Exhaust,
    Rectifier,
    Jammer,
    Radar,
    Receiver,
    Holoflag,
    Cockpit,
    Sabretooth,
    Football,
    Eagle,
    Alienware,
    Scary,
    Ninja,
    Trex,
    Honeydew,
    Mech7,
    Overwolf,
    Rhino8,
    // Variants
    A,
    B,
    C,
    Cube,
    Edge,
    Corner,
    Inner,
    Round,
    Slope,
    Cone,
    Pyramid,
    Prism,
    Tetra,
    Carbon6,
    Letters,
    Logo,
    Short,
    Long,
    Arc,
    Diagonal,
    #[serde(rename = "2D")]
    TwoD,
    #[serde(rename = "3D")]
    ThreeD,
    Plus,
    Frame,
    Cross,
    Spring,
    Slice,
    Ramp,
    Left,
    Right,
    Skewed,
    Steering,
    Track,
    Leg,
    Bladed,
    Spider,
    Bat,
    Front,
    Firework,
    Egg,
    Power,
    Disc,
    Blink,
    Ghost,
    EMP,
    Windowmaker,
    Spiked,
    Cray,
    Gene,
    Mega,
    Bunny,
    Single,
    Snowflake,
    Flowers,
    Twin,
    Vigilant,
    Cat,
    Cyborg,
    Pin,
    Needle,
    Dagger,
    Claw,
    Small,
    Large,
    Lights,
    Blower,
    Stack,
    Argentina,
    Australia,
    #[serde(rename = "Chrono.gg")]
    ChronoGG,
    Belarus,
    Belgium,
    Brazil,
    Canada,
    China,
    Danish,
    France,
    Germany,
    Iceland,
    Ireland,
    Italy,
    Japan,
    Kazakhstan,
    Netherlands,
    New,
    Zealand,
    Poland,
    Russia,
    South,
    Korea,
    Spain,
    Sweden,
    Turkish,
    UK,
    USA,
    Ukraine,
    Robocraft,
    Dev,
    Supporter,
    Bronze,
    Silver,
    Golden,
    Diamond,
    Protonium,
    #[serde(rename = "1")]
    One,
    #[serde(rename = "2")]
    Two,
    #[serde(rename = "3")]
    Three,
    #[serde(rename = "4")]
    Four,
    #[serde(rename = "5")]
    Five,
    #[serde(rename = "6")]
    Six,
    #[serde(rename = "100")]
    OneHundred,
    Years,
    Stars,
    Pirate,
    Nyan,
    Able,
    Gamers,
    Charity,
    Curse,
    Yogscast,
    Humble,
    Bundle,
    #[serde(rename = "2019")]
    TwoThousandNineTeen,
    Candy,
    Cane,
    Santa,
    Robopass,
    Season,
    EasterEgg,
    Mid,
    Top,
    Ear,
    Face,
    Jaw,
    Guard,
    Helmet,
    Feathers,
    Neck,
    Head,
    Eye,
    Eyes,
    Horn,
    Bandana,
    Nose,
    Beard,
    Horns,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ParseIdentError(String);

impl Display for ParseIdentError {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
        write!(fmt, "{}", self.0)
    }
}

impl serde::de::Error for ParseIdentError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        ParseIdentError(msg.to_string())
    }
}

impl Error for ParseIdentError {}

struct IdentDeserializer<'de>(&'de str);

impl<'de> de::Deserializer<'de> for &mut IdentDeserializer<'de> {
    type Error = ParseIdentError;

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct identifier ignored_any
    }

    fn deserialize_any<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(ParseIdentError(String::from("hit deserialize_any")))
    }

    fn deserialize_enum<V>(
        self,
        _: &'static str,
        _: &'static [&'static str],
        v: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        v.visit_enum(self.0.into_deserializer())
    }
}

impl FromStr for Ident {
    type Err = ParseIdentError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = match input {
            "chrono.gg" => "Chrono.gg",
            "2d" => "2D",
            "3d" => "3D",
            _ => input,
        };
        Ident::deserialize(&mut IdentDeserializer(input))
    }
}

impl Ident {
    pub fn as_str(&self) -> &'static str {
        to_variant_name(self).unwrap()
    }
}
