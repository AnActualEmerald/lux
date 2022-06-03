use riven::consts::PlatformRoute;
use std::str::FromStr;

pub struct WrappedRoute(pub PlatformRoute);

impl FromStr for WrappedRoute {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NA" => Ok(WrappedRoute(PlatformRoute::NA1)),
            "EUNE" => Ok(WrappedRoute(PlatformRoute::EUN1)),
            "EUW" => Ok(WrappedRoute(PlatformRoute::EUW1)),
            "JP" => Ok(WrappedRoute(PlatformRoute::JP1)),
            "LAN" => Ok(WrappedRoute(PlatformRoute::LA1)),
            "LAS" => Ok(WrappedRoute(PlatformRoute::LA2)),
            "OCE" => Ok(WrappedRoute(PlatformRoute::OC1)),
            "PBE" => Ok(WrappedRoute(PlatformRoute::PBE1)),
            "RU" => Ok(WrappedRoute(PlatformRoute::RU)),
            "TR" => Ok(WrappedRoute(PlatformRoute::TR1)),
            "KR" => Ok(WrappedRoute(PlatformRoute::KR)),
            _ => Err("Unrecognized region string"),
        }
    }
}
