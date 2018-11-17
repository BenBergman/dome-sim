extern crate cgmath;
extern crate mint;
extern crate three;

use std::collections::HashMap;
use three::Object;

const LAT: f32 = 26.57;
const LONG: f32 = 36.0;

fn get_x(lat: f32, long: f32) -> f32 {
    long.to_radians().cos() * (lat + 90.0).to_radians().sin()
}

fn get_y(lat: f32, long: f32) -> f32 {
    long.to_radians().sin() * (lat + 90.0).to_radians().sin()
}

fn get_z(lat: f32, _long: f32) -> f32 {
    (lat + 90.0).to_radians().cos()
}

fn find_mid(p1: mint::Point3<f32>, p2: mint::Point3<f32>, div: f32) -> mint::Point3<f32> {
    [
        (p1.x + p2.x * (div - 1.0)) / div,
        (p1.y + p2.y * (div - 1.0)) / div,
        (p1.z + p2.z * (div - 1.0)) / div,
    ]
    .into()
}

fn find_center(
    p1: mint::Point3<f32>,
    p2: mint::Point3<f32>,
    p3: mint::Point3<f32>,
) -> mint::Point3<f32> {
    [
        (p1.x + p2.x + p3.x) / 3.0,
        (p1.y + p2.y + p3.y) / 3.0,
        (p1.z + p2.z + p3.z) / 3.0,
    ]
    .into()
}

fn normalize(p: mint::Point3<f32>) -> mint::Point3<f32> {
    let mag = (p.x * p.x + p.y * p.y + p.z * p.z).sqrt();
    [p.x / mag, p.y / mag, p.z / mag].into()
}

fn main() {
    let mut win = three::Window::new("Three-rs Mesh Update Example");
    let cam = win.factory.perspective_camera(60.0, 1.0..10.0);
    let mut controls = three::controls::Orbit::builder(&cam)
        .position([0.0, 3.0, 1.0])
        .target([0.0, 0.0, 0.0])
        .build();

    let light = win.factory.point_light(0xffffff, 0.5);
    let pos = [0.0, 5.0, 5.0];
    light.set_position(pos);
    win.scene.add(&light);

    let geometry = three::Geometry::plane(2.0, 2.0);
    let material = three::material::Phong {
        color: 0xBF0000,
        glossiness: 80.0,
    };
    let mesh = win.factory.mesh(geometry, material);
    mesh.set_position([0.0, 0.0, -1.2]);
    win.scene.add(&mesh);

    let geometry = three::Geometry::cuboid(0.05, 0.05, 0.05);
    let material = three::material::Phong {
        color: 0xBF0000,
        glossiness: 80.0,
    };
    let mesh = win.factory.mesh(geometry.clone(), material);
    win.scene.add(&mesh);

    let mut all_points = HashMap::new();

    let icosohedron_points: [mint::Point3<f32>; 12] = [
        [get_x(90.0, 0.0), get_y(90.0, 0.0), get_z(90.0, 0.0)].into(),
        [
            get_x(LAT, 0.0 * LONG),
            get_y(LAT, 0.0 * LONG),
            get_z(LAT, 0.0 * LONG),
        ]
        .into(),
        [
            get_x(LAT, 2.0 * LONG),
            get_y(LAT, 2.0 * LONG),
            get_z(LAT, 2.0 * LONG),
        ]
        .into(),
        [
            get_x(LAT, 4.0 * LONG),
            get_y(LAT, 4.0 * LONG),
            get_z(LAT, 4.0 * LONG),
        ]
        .into(),
        [
            get_x(LAT, 6.0 * LONG),
            get_y(LAT, 6.0 * LONG),
            get_z(LAT, 6.0 * LONG),
        ]
        .into(),
        [
            get_x(LAT, 8.0 * LONG),
            get_y(LAT, 8.0 * LONG),
            get_z(LAT, 8.0 * LONG),
        ]
        .into(),
        [
            get_x(-LAT, 1.0 * LONG),
            get_y(-LAT, 1.0 * LONG),
            get_z(-LAT, 1.0 * LONG),
        ]
        .into(),
        [
            get_x(-LAT, 3.0 * LONG),
            get_y(-LAT, 3.0 * LONG),
            get_z(-LAT, 3.0 * LONG),
        ]
        .into(),
        [
            get_x(-LAT, 5.0 * LONG),
            get_y(-LAT, 5.0 * LONG),
            get_z(-LAT, 5.0 * LONG),
        ]
        .into(),
        [
            get_x(-LAT, 7.0 * LONG),
            get_y(-LAT, 7.0 * LONG),
            get_z(-LAT, 7.0 * LONG),
        ]
        .into(),
        [
            get_x(-LAT, 9.0 * LONG),
            get_y(-LAT, 9.0 * LONG),
            get_z(-LAT, 9.0 * LONG),
        ]
        .into(),
        [get_x(-90.0, 0.0), get_y(-90.0, 0.0), get_z(-90.0, 0.0)].into(),
    ];

    for (i, &point) in icosohedron_points.iter().enumerate() {
        all_points.insert(vec!(i), point);
    }

    let faces = [
        [0, 1, 2],
        [0, 2, 3],
        [0, 3, 4],
        [0, 4, 5],
        [0, 5, 1],
        [1, 6, 2],
        [6, 2, 7],
        [2, 7, 3],
        [7, 3, 8],
        [3, 8, 4],
        [8, 4, 9],
        [4, 9, 5],
        [9, 5, 10],
        [5, 10, 1],
        [10, 1, 6],
        [11, 6, 7],
        [11, 7, 8],
        [11, 8, 9],
        [11, 9, 10],
        [11, 10, 6],
    ];

    for face in faces.iter() {
        let p1 = face[0];
        let p2 = face[1];
        let p3 = face[2];

        for set in [vec!(p1, p2), vec!(p2, p1), vec!(p2, p3), vec!(p3, p2), vec!(p3, p1), vec!(p1, p3)].iter() {
            let point = find_mid(icosohedron_points[set[0]], icosohedron_points[set[1]], 3.0);
            all_points.insert(set.to_vec(), point);
        }

        let point = find_center(icosohedron_points[p1], icosohedron_points[p2], icosohedron_points[p3]);
        all_points.insert(vec!(p1, p2, p3), point);
    }

    println!("{}", all_points.iter().filter(|(_, p)| p.z > -0.4).count());

    for (_, &point) in all_points.iter().filter(|(_, p)| p.z > -0.4) {
        let colour = 0x1010FF;
        let material = three::material::Phong {
            color: colour,
            glossiness: 80.0,
        };
        let mesh = win.factory.mesh(geometry.clone(), material.clone());
        mesh.set_position(normalize(point));
        win.scene.add(&mesh);
    }

    let timer = three::Timer::new();
    while win.update() && !win.input.hit(three::KEY_ESCAPE) {
        let _elapsed_time = timer.elapsed();
        controls.update(&win.input);
        win.render(&cam);
    }
}
