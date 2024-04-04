use crate::prelude::*;
use galileo::layer::feature_layer::Feature;
use galileo_types::cartesian::{CartesianPoint2d, Point2d, Rect};
use galileo_types::geo::Projection;
use galileo_types::geometry::{CartesianGeometry2d, Geom, Geometry};
use polite::Polite;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tracing::info;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Address {
    #[serde(rename(deserialize = "FULLADDRES"))]
    pub label: String,
    #[serde(rename(deserialize = "STATUS"))]
    pub status: String,
    #[serde(rename(deserialize = "wgs84_y"))]
    pub lat: f64,
    #[serde(rename(deserialize = "wgs84_x"))]
    pub lon: f64,
    #[serde(rename(deserialize = "espg3857_x"))]
    pub x: f64,
    #[serde(rename(deserialize = "espg3857_y"))]
    pub y: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Addresses {
    pub records: Vec<Address>,
}

impl Addresses {
    pub fn from_csv<P: AsRef<Path>>(path: P) -> Polite<Self> {
        let records = from_csv(path)?;
        Ok(Addresses { records })
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Polite<Self> {
        info!("Deserializing from binary.");
        let vec: Vec<u8> = std::fs::read(path)?;
        let values: Addresses = bincode::deserialize(&vec[..])?;
        Ok(values)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AddressPoint {
    pub address: Address,
    pub geometry: Point2d,
    pub bounds: Rect,
    pub selected: bool,
}

impl From<Address> for AddressPoint {
    fn from(address: Address) -> Self {
        let geometry = Point2d::new(address.x, address.y);
        let bounds = point_bounds(&geometry, 0.05);
        let selected = false;
        Self {
            address,
            geometry,
            bounds,
            selected,
        }
    }
}

impl Geometry for AddressPoint {
    type Point = Point2d;

    fn project<P: Projection<InPoint = Self::Point> + ?Sized>(
        &self,
        projection: &P,
    ) -> Option<Geom<P::OutPoint>> {
        self.geometry.project(projection)
    }
}

impl CartesianGeometry2d<Point2d> for AddressPoint {
    fn is_point_inside<Other: CartesianPoint2d<Num = f64>>(
        &self,
        point: &Other,
        tolerance: f64,
    ) -> bool {
        if !self.bounds.contains(point) {
            return false;
        }

        self.geometry.is_point_inside(point, tolerance)
    }

    fn bounding_rectangle(&self) -> Option<Rect> {
        Some(self.bounds)
    }
}

impl Feature for AddressPoint {
    type Geom = Self;

    fn geometry(&self) -> &Self::Geom {
        self
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AddressPoints {
    pub records: Vec<AddressPoint>,
}

impl From<Addresses> for AddressPoints {
    fn from(addresses: Addresses) -> Self {
        let records = addresses
            .records
            .iter()
            .map(|v| AddressPoint::from(v.clone()))
            .collect::<Vec<AddressPoint>>();
        Self { records }
    }
}

impl AddressPoints {
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Polite<()> {
        info!("Serializing to binary.");
        let encode = bincode::serialize(self)?;
        info!("Writing to file.");
        std::fs::write(path, encode)?;
        Ok(())
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Polite<Self> {
        info!("Deserializing from binary.");
        let vec: Vec<u8> = std::fs::read(path)?;
        let addresses: AddressPoints = bincode::deserialize(&vec[..])?;
        Ok(addresses)
    }
}
