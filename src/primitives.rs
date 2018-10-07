extern crate cgmath;
extern crate rand;
extern crate euclid;
extern crate conrod;

use piston_window::*;
use cgmath::*;
use rand::distributions::{IndependentSample, Range};
use rand::Rng;
//use rosrust::PublisherStream;
use rosrust::api::raii::Publisher;
use euclid::*;

pub type Point2f64 = Point2<f64>;
pub type Vec2f64 = Vector2<f64>;
pub type Size2f64 = Size2D<f64>;

pub fn zero_vec2f64() -> Vec2f64 {
    Vec2f64{x: 0.0, y: 0.0}
}