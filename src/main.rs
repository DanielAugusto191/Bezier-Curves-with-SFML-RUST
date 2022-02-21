#![allow(non_snake_case)]
use sfml::{graphics::*, system::*, window::*, graphics::PrimitiveType};
use std::collections::LinkedList;

fn nPoint(p1: &CircleShape, p2: &CircleShape, t: f32) -> Vector2f{
    let nP3pX = (1.-t)*p1.position().x + t*p2.position().x;
    let nP3pY = (1.-t)*p1.position().y + t*p2.position().y;
    Vector2f::new(nP3pX, nP3pY)
}

fn main() {
    let show = false;
    let windows_size = (1600, 900);
    let desktop = VideoMode::desktop_mode();
    let mut window = RenderWindow::new(
        VideoMode::new(windows_size.0, windows_size.1, desktop.bits_per_pixel),
        "New",
        Style::CLOSE,
        &Default::default(),
    );
    let dot_size = 10.;
    let offset_dot = dot_size;
    let mut p1 = CircleShape::new(dot_size, 30);
    p1.set_fill_color(Color::YELLOW);
    p1.set_position((450.- offset_dot, 800.- offset_dot));
    let mut p2 = CircleShape::new(dot_size, 30);
    p2.set_fill_color(Color::YELLOW);
    p2.set_position((1400.- offset_dot, 170.- offset_dot));
    let mut p3 = CircleShape::new(dot_size, 30);
    p3.set_fill_color(Color::YELLOW);
    p3.set_position((20.- offset_dot, 170.- offset_dot));
    let mut p4 = CircleShape::new(dot_size, 30);
    p4.set_fill_color(Color::YELLOW);
    p4.set_position((1200.- offset_dot, 800.- offset_dot)); // 600 450

    // Parametrized 1
    let mut point1 = CircleShape::new(dot_size, 30);
    point1.set_fill_color(Color::BLUE);
    point1.set_position((120.- offset_dot, 120.- offset_dot));
    let mut point2 = CircleShape::new(dot_size, 30);
    point2.set_fill_color(Color::BLUE);
    point2.set_position((p2.position().x, p2.position().y));
    let mut point3 = CircleShape::new(dot_size, 30);
    point3.set_fill_color(Color::BLUE);
    point3.set_position((p3.position().x, p3.position().y));
    
    // Parametrized 2
    let mut point4 = CircleShape::new(dot_size, 30);
    point4.set_fill_color(Color::RED);
    point4.set_position((point1.position().x, point1.position().y));
    let mut point5 = CircleShape::new(dot_size, 30);
    point5.set_fill_color(Color::RED);
    point5.set_position((point2.position().x, point2.position().y));

    // Parametrized 3

    let mut point = CircleShape::new(dot_size, 30);
    point.set_fill_color(Color::GREEN);
    point.set_position((p1.position().x, p1.position().y));

    let mut bPoints: LinkedList<Vec<Vertex>> = LinkedList::new();

    let points = vec![p1, p2, p3, p4];

    let mut lines : Vec<Vec<Vertex>> = Vec::new(); 

    for i in 1..points.len(){
        lines.push(vec![
            Vertex::new(Vector2f::new(points[i-1].position().x + offset_dot, points[i-1].position().y + offset_dot), Color::WHITE, Vector2f::new(0., 0.)),
            Vertex::new(Vector2f::new(points[i].position().x + offset_dot, points[i].position().y + offset_dot), Color::WHITE, Vector2f::new(0., 0.)),
        ])
    }

    let mut t = 0.08;
    let mut side = true;
    let mut clock = Clock::start();

    while window.is_open() {
        while let Some(event) = window.poll_event(){
            match event {
                Event::Closed => window.close(),
                _ => {}
            }
        }
        let delta_time = clock.restart().as_seconds();
        let nP1 = nPoint(&points[0], &points[1], t);
        let nP2 = nPoint(&points[1], &points[2], t);
        let nP3 = nPoint(&points[2], &points[3], t);
        point1.set_position(nP1);
        point2.set_position(nP2);
        point3.set_position(nP3);
        let nP4 = nPoint(&point1, &point2, t);
        let nP5 = nPoint(&point2, &point3, t);
        point4.set_position(nP4); 
        point5.set_position(nP5);

        let oP = Vector2f::new(point.position().x, point.position().y);
        let nP = nPoint(&point4, &point5, t);
        point.set_position(nP);

        if side {
            bPoints.push_back(vec![
                Vertex::new(Vector2f::new(oP.x + offset_dot, oP.y + offset_dot), Color::GREEN, Vector2f::new(0., 0.)),
                Vertex::new(Vector2f::new(nP.x + offset_dot, nP.y + offset_dot), Color::GREEN, Vector2f::new(0., 0.)),
            ]);
        } else {
            bPoints.pop_back();
        }
        
        t += 0.5 * (if side {1.} else {-1.}) * delta_time;

        if t >= 1. { side = false};
        if t <= 0. { side = true};


        let nLerp = vec![
            Vertex::new(Vector2f::new(point1.position().x + offset_dot, point1.position().y + offset_dot), Color::BLUE, Vector2f::new(0., 0.)),
            Vertex::new(Vector2f::new(point2.position().x + offset_dot, point2.position().y + offset_dot), Color::BLUE, Vector2f::new(0., 0.)),
        ];
        let nLerp2 = vec![
            Vertex::new(Vector2f::new(point2.position().x + offset_dot, point2.position().y + offset_dot), Color::BLUE, Vector2f::new(0., 0.)),
            Vertex::new(Vector2f::new(point3.position().x + offset_dot, point3.position().y + offset_dot), Color::BLUE, Vector2f::new(0., 0.)),
        ];
        let nLerp3 = vec![
            Vertex::new(Vector2f::new(point4.position().x + offset_dot, point4.position().y + offset_dot), Color::RED, Vector2f::new(0., 0.)),
            Vertex::new(Vector2f::new(point5.position().x + offset_dot, point5.position().y + offset_dot), Color::RED, Vector2f::new(0., 0.)),
        ];

        window.clear(Color::BLACK);
        window.draw(&point);
        if show{
            window.draw(&point1);
            window.draw(&point2);
            window.draw(&point3);
            window.draw(&point4);
            window.draw(&point5);
            for e in &points {
                window.draw(e);
            }
            for e in &lines {
                window.draw_primitives(&e, PrimitiveType::LINES, &RenderStates::default());
            }
            window.draw_primitives(&nLerp, PrimitiveType::LINES, &RenderStates::default());
            window.draw_primitives(&nLerp2, PrimitiveType::LINES, &RenderStates::default());
            window.draw_primitives(&nLerp3, PrimitiveType::LINES, &RenderStates::default());
        }
        for e in &bPoints {
            window.draw_primitives(&e, PrimitiveType::LINES, &RenderStates::default());
        }
        window.display();
    }
}
