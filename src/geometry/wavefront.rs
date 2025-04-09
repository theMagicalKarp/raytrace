use crate::geometry::BvhNode;
use crate::geometry::Geometry;
use crate::geometry::HitRecord;
use crate::geometry::Hittable;
use crate::geometry::aabb::Aabb;
use crate::geometry::triangle::Triangle;
use crate::geometry::triangle::Vertex;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use obj::raw::object::Polygon;
use obj::raw::object::RawObj;
use rand::rngs::ThreadRng;

#[derive(Debug, Clone)]
pub struct Wavefront {
    children: Box<BvhNode>,
}

impl Wavefront {
    pub fn new(object: &RawObj, material: Material) -> Self {
        let primitives: Vec<Geometry> = object
            .polygons
            .iter()
            .flat_map(|polygon| match polygon {
                Polygon::P(polygon) => (1..polygon.len() - 1)
                    .map(|i| {
                        let a = Vertex::from_poligon(object.positions[polygon[0]], None);
                        let b = Vertex::from_poligon(object.positions[polygon[i]], None);
                        let c = Vertex::from_poligon(object.positions[polygon[i + 1]], None);

                        Triangle::geometry(a, b, c, material.clone())
                    })
                    .collect::<Vec<Geometry>>(),
                Polygon::PT(polygon) => (1..polygon.len() - 1)
                    .map(|i| {
                        let a = Vertex::from_poligon(object.positions[polygon[0].0], None);
                        let b = Vertex::from_poligon(object.positions[polygon[i].0], None);
                        let c = Vertex::from_poligon(object.positions[polygon[i + 1].0], None);

                        Triangle::geometry(a, b, c, material.clone())
                    })
                    .collect::<Vec<Geometry>>(),
                Polygon::PN(polygon) => (1..polygon.len() - 1)
                    .map(|i| {
                        let a = Vertex::from_poligon(
                            object.positions[polygon[0].0],
                            Some(object.normals[polygon[0].1]),
                        );
                        let b = Vertex::from_poligon(
                            object.positions[polygon[i].0],
                            Some(object.normals[polygon[i].1]),
                        );
                        let c = Vertex::from_poligon(
                            object.positions[polygon[i + 1].0],
                            Some(object.normals[polygon[i + 1].1]),
                        );

                        Triangle::geometry(a, b, c, material.clone())
                    })
                    .collect::<Vec<Geometry>>(),
                Polygon::PTN(polygon) => (1..polygon.len() - 1)
                    .map(|i| {
                        let a = Vertex::from_poligon(
                            object.positions[polygon[0].0],
                            Some(object.normals[polygon[0].2]),
                        );
                        let b = Vertex::from_poligon(
                            object.positions[polygon[i].0],
                            Some(object.normals[polygon[i].2]),
                        );
                        let c = Vertex::from_poligon(
                            object.positions[polygon[i + 1].0],
                            Some(object.normals[polygon[i + 1].2]),
                        );

                        Triangle::geometry(a, b, c, material.clone())
                    })
                    .collect::<Vec<Geometry>>(),
            })
            .collect::<Vec<Geometry>>();

        let children = Box::new(BvhNode::new(primitives.to_vec()));

        Wavefront { children }
    }
    pub fn geometry(object: &RawObj, material: Material) -> Geometry {
        Geometry::Wavefront(Wavefront::new(object, material))
    }
}

impl Hittable for Wavefront {
    fn hit(
        &self,
        r: &Ray,
        interval: &Interval,
        record: &mut HitRecord,
        rng: &mut ThreadRng,
    ) -> bool {
        self.children.hit(r, interval, record, rng)
    }

    fn bounding_box(&self) -> Aabb {
        self.children.bounding_box()
    }
}
