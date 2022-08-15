use macroquad::prelude::*;

#[derive(Clone, PartialEq, Eq)]
enum ParticleVariant {
    Sand,
    Brick
}

#[derive(Clone)]
struct Particle {
    id: i32,
    variant: ParticleVariant,
    active: bool
}

impl Particle {
    fn new(id: i32, variant: ParticleVariant, active: bool) -> Particle {
        Particle { id, variant, active }
    }
}

#[macroquad::main("Rusty Sandbox")]
async fn main() {
    let mut world: Vec<Vec<Particle>> = Vec::new();
    let mut last_id: i32 = 0;
    let mut paint_radius: i16 = 1;
    loop {
        clear_background(BLACK);

        // For every screen-height-pixel missing in world-space:
        for y in world.len()..screen_width() as usize {

            // Push the Y-axis particle vector
            let yvec: Vec<Particle> = Vec::new();
            world.push(yvec);

            // For every screen-width-pixel missing in world-space:
            for _x in world[y].len()..screen_height() as usize {

                // Generate a non-interactive placeholder particle
                last_id += 1;
                let air = Particle::new(
                    last_id,
                    ParticleVariant::Sand,
                    false
                );

                // Push the air particle
                world[y].push(air);
            }
        }

        // Control: left click for Sand
        if is_mouse_button_down(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            let mouse_x = mouse_x as i16;
            let mouse_y = mouse_y as i16;

            // Fill an X/Y radius from the cursor with Sand particles
            for y in mouse_y..(mouse_y + paint_radius) {
                for x in mouse_x - paint_radius..(mouse_x + paint_radius) {
                    // Note: macroquad doesn't like the mouse leaving the window when dragging.
                    // ... so make sure no crazy out-of-bounds happen!
                    if x > 0 && x < screen_width() as i16 && y > 0 && y < screen_height() as i16 {
                        let ptr = &mut world[x as usize][y as usize];
                        // If not occupied: assign Sand as the Variant and activate
                        if !ptr.active {
                            ptr.variant = ParticleVariant::Sand;
                            ptr.active = true;
                        }
                    }
                }
            }
        }

        // Control: right click for Brick
        if is_mouse_button_down(MouseButton::Right) {
            let (mouse_x, mouse_y) = mouse_position();
            let ptr = &mut world[mouse_x as usize][mouse_y as usize];
            // If not occupied: assign Brick as the Variant and activate
            if !ptr.active {
                ptr.variant = ParticleVariant::Brick;
                ptr.active = true;
            }
        }

        // Control: increase paint radius
        if is_key_pressed(KeyCode::KpAdd) {
            paint_radius += 1;
        }

        // Control: decrease paint radius
        if is_key_pressed(KeyCode::KpSubtract) && paint_radius > 1 {
            paint_radius -= 1;
        }

        // UI: Top-right
        draw_text("Left click for Sand, Right click for Brick!", 25.0, 25.0, 20.0, BLUE);

        // UI: Bottom-left
        draw_text(format!("Paint Size: {}px", paint_radius).as_str(), 25.0, screen_height() - 50.0, 50.0, BLUE);
        draw_text("Use the Numpad (+ and -) to increase/decrease size!", 25.0, screen_height() - 25.0, 20.0, BLUE);

        // Keep track of particle IDs that were modified within this frame.
        // ... this is to avoid 'infinite simulation' since gravity pulls them down the Y-axis progressively.
        let mut ids: Vec<i32> = Vec::new();
        
        // Update the state of all particles + render
        for px in 0..world.len() {
            // A couple pre-use-casts to make macroquad float calculations easier and faster
            let px32 = px as f32;

            for py in 0..world[px].len() {
                let py32 = py as f32;

                // Only process active elements (inactive is essentially thin air / invisible)
                if !world[px][py].active {
                    continue;
                }
                // Don't re-simulate particles that have already been simulated this frame
                if ids.contains(&world[px][py].id) {
                    continue;
                }
                ids.push(world[px][py].id);

                // Only process Sand (and other future interactive particles) here
                if world[px][py].variant == ParticleVariant::Sand {
                    // Clone for use in pixel tracking
                    let particle_under = &mut world[px].get(py + 1).cloned();
                    let is_below_free = particle_under.as_ref().is_some() && !particle_under.as_ref().unwrap().active;

                    // Check for a floor
                    if py32 < screen_height() - 1.0 && is_below_free {
                        // There's no floor nor any particles below, so fall!

                        // Swap the particles (TODO: optimise!)
                        world[px][py + 1].variant = world[px][py].variant.clone();
                        world[px][py + 1].active = true;
                        let id = world[px][py + 1].id;
                        world[px][py + 1].id = world[px][py].id;
                        world[px][py].id = id;
                        world[px][py].active = false;
                    } else {
                        // Check particle has hit a floor and is within the screen width bounds
                        if !is_below_free && px > 1 && px32 <= screen_width() - 1.0 {

                            // 50% chance per-frame of a sand particle moving left-right (if space allows!)
                            if rand::gen_range(0, 100) < 50 {
                                let x_new = px + rand::gen_range(-2, 2) as usize;

                                // Ensure a neighbouring particle doesn't exist (and that it's within screen width)
                                if x_new > 1 && x_new < screen_width() as usize && !world[x_new][py].active {
                                    // Swap the particles (TODO: optimise!)
                                    world[x_new][py].variant = world[px][py].variant.clone();
                                    world[x_new][py].active = true;
                                    let a = world[x_new][py].id;
                                    world[x_new][py].id = world[px][py].id;
                                    world[px][py].id = a;
                                    world[px][py].active = false;
                                }
                            }
                        }
                    }
                }

                // Compute the colour from the Variant
                let particle = &world[px][py];
                let colour = match particle.variant {
                    ParticleVariant::Sand  => Color::new(194.0, 178.0, 128.0, 1.0),
                    ParticleVariant::Brick => RED
                };

                // Render updated particle state
                draw_rectangle(px as f32, py as f32, 1.0, 1.0, colour);
            }
        }

        next_frame().await
    }
}