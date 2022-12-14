use macroquad::prelude::*;

// NOTE: enable DEBUG and recompile for runtime stats / tracking / debugging helpers
static DEBUG: bool = false;

// Font size for the '{ParticleVariant} Selected' screen
static SELECTED_FONT_SIZE: f32 = 150.0;

#[derive(Clone, PartialEq, Eq)]
enum ParticleVariant {
    Sand,
    Dirt,
    Water,
    Brick
}

impl ParticleVariant {
    // Return a percentage (1-100) chance of this particle moving, based on it's variant
    fn get_movement_chance(&self) -> u8 {
        match self {
            ParticleVariant::Sand  => 50,
            ParticleVariant::Dirt  => 5,
            ParticleVariant::Water => 100,
            // Other particles (ie: brick) will default to being still
            _ => 0
        }
    }
}

impl std::fmt::Display for ParticleVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParticleVariant::Sand  => write!(f, "Sand"),
            ParticleVariant::Dirt  => write!(f, "Dirt"),
            ParticleVariant::Water => write!(f, "Water"),
            ParticleVariant::Brick => write!(f, "Brick")
        }
    }
}

#[derive(Clone)]
struct Particle {
    id: u32,
    variant: ParticleVariant,
    active: bool
}

impl Particle {
    fn new(id: u32, variant: ParticleVariant, active: bool) -> Particle {
        Particle { id, variant, active }
    }

    // Return a potential (non-guarenteed) movement delta for this particle, based on it's properties
    fn try_generate_movement(&self) -> usize {
        if rand::gen_range(0, 100) < self.variant.get_movement_chance() {
            rand::gen_range(-2, 2) as usize
        } else { 0 }
    }

    // Return a colour for this particle, based on it's properties
    // BUG (?): using a custom `Color::new(r, g, b, a);` doesn't seem to work here... so try to stick to defaults?
    fn get_colour(&self) -> Color {
        match self.variant {
            ParticleVariant::Sand  => BEIGE,
            ParticleVariant::Dirt  => DARKBROWN,
            ParticleVariant::Water => BLUE,
            ParticleVariant::Brick => RED
        }
    }
}

#[macroquad::main("Rusty Sandbox")]
async fn main() {
    // The 2D world-space particle grid
    let mut world: Vec<Vec<Particle>> = Vec::new();

    // The last particle ID generated
    let mut last_id: u32 = 0;

    // The size (in pixels) of our paint radius
    let mut paint_radius: u16 = 1;

    // The zoom multiplyer
    let mut camera_zoom: u8 = 1;

    // The camera offsets (used to 'control' the camera's location on the grid via zoomed X/Y offset)
    let mut camera_offset_x: i16 = 0;
    let mut camera_offset_y: i16 = 0;

    // Flag to ensure paint 'smoothing' doesn't activate between clicks (individual paints)
    let mut is_drawing_secondary = false;

    // Trackers for mouse movements (used in 'smoothing' fast paints)
    let mut last_x: u16 = 0;
    let mut last_y: u16 = 0;

    // Flag lock to tell the engine when the user is hitting a GUI button
    let mut is_clicking_ui = false;

    // The current primary particle variant selected by the user
    let mut selected_variant = ParticleVariant::Sand;

    // The logic + renderer loop
    loop {
        clear_background(BLACK);

        // For every screen-height-pixel missing in world-space:
        for x in world.len()..screen_width() as usize {

            // Push the Y-axis particle vector
            let yvec: Vec<Particle> = Vec::new();
            world.push(yvec);

            // For every screen-width-pixel missing in world-space:
            for _y in world[x].len()..screen_height() as usize {

                // Generate a non-interactive placeholder particle
                last_id += 1;
                let air = Particle::new(
                    last_id,
                    ParticleVariant::Sand,
                    false
                );

                // Push the air particle
                world[x].push(air);
            }
        }

        // UI: Top-right
        if macroquad::ui::root_ui().button(vec2(25.0, 25.0), "Sand") {
            is_clicking_ui = true;
            selected_variant = ParticleVariant::Sand;
        }

        if macroquad::ui::root_ui().button(vec2(75.0, 25.0), "Dirt") {
            is_clicking_ui = true;
            selected_variant = ParticleVariant::Dirt;
        }

        if macroquad::ui::root_ui().button(vec2(125.0, 25.0), "Water") {
            is_clicking_ui = true;
            selected_variant = ParticleVariant::Water;
        }

        // UI: Top-Centre
        let selected_display_str = format!("{}", selected_variant);
        let selected_display_size = measure_text(selected_display_str.as_str(), None, SELECTED_FONT_SIZE as u16, 1.0);
        draw_text(selected_display_str.as_str(), (screen_width() / 2.0) - (selected_display_size.width / 2.0), 175.0, SELECTED_FONT_SIZE, Color::new(0.0, 0.47, 0.95, 0.275));

        // UI: Bottom-left
        draw_text(format!("Paint Size: {}px", paint_radius).as_str(), 25.0, screen_height() - 50.0, 50.0, BLUE);
        draw_text("Use the Numpad (+ and -) to increase/decrease size!", 25.0, screen_height() - 25.0, 20.0, BLUE);


        // Disable the mouse when clicking UI elements
        if !is_clicking_ui {
            // Control: left click for Sand
            if is_mouse_button_down(MouseButton::Left) {
                let (mouse_x, mouse_y) = mouse_position();
                let mouse_x = (mouse_x as u16 / camera_zoom as u16) - camera_offset_x as u16;
                let mouse_y = (mouse_y as u16 / camera_zoom as u16) - camera_offset_y as u16;

                // Fill an X/Y radius from the cursor with Sand particles
                for y in mouse_y..(mouse_y + paint_radius) {
                    for x in mouse_x - paint_radius..(mouse_x + paint_radius) {
                        // Note: macroquad doesn't like the mouse leaving the window when dragging.
                        // ... so make sure no crazy out-of-bounds happen!
                        if x > 0 && x < screen_width() as u16 && y > 0 && y < screen_height() as u16 {
                            let ptr = &mut world[x as usize][y as usize];
                            // If not occupied: assign Sand as the Variant and activate
                            if !ptr.active {
                                ptr.variant = selected_variant.clone();
                                ptr.active = true;
                            }
                        }
                    }
                }
            }

            // Control: right click for Brick
            if is_mouse_button_down(MouseButton::Right) {
                let (mouse_x, mouse_y) = mouse_position();
                let mouse_x = (mouse_x as u16 / camera_zoom as u16) - camera_offset_x as u16;
                let mouse_y = (mouse_y as u16 / camera_zoom as u16) - camera_offset_y as u16;
                // If the distance is large (e.g: a fast mouse flick) then we need to 'best-guess' the path of the cursor mid-frame
                // ... so that there's no gaps left between paint intersections, a nice touch for UX!
                if is_drawing_secondary {
                    // TODO: We can do a much better algorithm than this (perhaps linear interpolation?)
                    // While the X or Y coords of the last particle don't match the current mouse coords, pathfind our way to it!
                    while last_x != mouse_x || last_y != mouse_y {
                        if mouse_x > last_x { last_x += 1; }
                        if mouse_x < last_x { last_x -= 1; }
                        if mouse_y > last_y { last_y += 1; }
                        if mouse_y < last_y { last_y -= 1; }
                        // Note: macroquad doesn't like the mouse leaving the window when dragging.
                        // ... so make sure no crazy out-of-bounds happen!
                        if last_x > 0 && last_x < screen_width() as u16 && last_y > 0 && last_y < screen_height() as u16 {
                            // Place a particle along the path
                            let ptr = &mut world[last_x as usize][last_y as usize];
                            if !ptr.active {
                                ptr.variant = ParticleVariant::Brick;
                                ptr.active = true;
                            }
                        }
                    }
                } else {
                    // Reset X/Y tracking when we're not smoothing
                    last_x = mouse_x;
                    last_y = mouse_y;
                    // Switch the secondary draw on after one frame (to avoid the pathing system activating between 'paints')
                    is_drawing_secondary = true;
                }
            }
        }

        // Control release: Disable the secondary paint smoothing
        if is_mouse_button_released(MouseButton::Right) {
            is_drawing_secondary = false;
        }

        // Control: increase paint radius
        if is_key_pressed(KeyCode::KpAdd) {
            paint_radius += 1;
        }

        // Control: decrease paint radius
        if is_key_pressed(KeyCode::KpSubtract) && paint_radius > 1 {
            paint_radius -= 1;
        }

        // Control: rendering scale (zoom)
        let (_, scroll_y) = mouse_wheel();
        if scroll_y != 0.0 {
            if scroll_y > 0.0 {
                // Maximum zoom of 5x
                if camera_zoom < 5 {
                    camera_zoom += 1;
                }
            } else {
                // Minimum zoom of 1x (default)
                if camera_zoom > 1 {
                    camera_zoom -= 1;
                }
            }
        }

        // Control: WASD and Arrow Keys for camera 'offset' movement
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up)    { camera_offset_y += 1 }
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left)  { camera_offset_x += 1 }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down)  { camera_offset_y -= 1 }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) { camera_offset_x -= 1 }

        // Keep track of particle IDs that were modified within this frame.
        // ... this is to avoid 'infinite simulation' since gravity pulls them down the Y-axis progressively.
        let mut updated_ids: Vec<u32> = Vec::new();
        
        // Update the state of all particles + render
        let mut sand_count = 0;
        let mut dirt_count = 0;
        let mut water_count = 0;
        let mut brick_count = 0;
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
                if updated_ids.contains(&world[px][py].id) {
                    continue;
                }

                // Debugging: track pixel counts
                if DEBUG {
                    match world[px][py].variant {
                        ParticleVariant::Sand  => { sand_count  += 1 },
                        ParticleVariant::Dirt  => { dirt_count  += 1 },
                        ParticleVariant::Water => { water_count += 1 },
                        ParticleVariant::Brick => { brick_count += 1 },
                    }
                }

                // Only process Sand (and other future interactive particles) here
                if world[px][py].variant == ParticleVariant::Sand || world[px][py].variant == ParticleVariant::Dirt || world[px][py].variant == ParticleVariant::Water {
                    // Clone for use in pixel tracking
                    let particle_under = &mut world[px].get(py + 1).cloned();
                    let is_below_free = particle_under.as_ref().is_some() && !particle_under.as_ref().unwrap().active;

                    // Check for a floor
                    if py32 < screen_height() - 1.0 && is_below_free {
                        // There's no floor nor any particles below, so fall!

                        // Swap the particles (TODO: optimise!)
                        world[px][py + 1].variant = world[px][py].variant.clone();
                        world[px][py + 1].active = true;
                        let new_id = world[px][py + 1].id;
                        world[px][py + 1].id = world[px][py].id;
                        updated_ids.push(world[px][py + 1].id);
                        world[px][py].id = new_id;
                        world[px][py].active = false;
                    } else {
                        // Check particle has hit a floor and is within the screen width bounds
                        if !is_below_free && px > 0 && px32 < screen_width() {

                            // Compute the new X-axis based on Particle properties
                            let x_new = px + world[px][py].try_generate_movement();

                            // Ensure the new X-axis is valid
                            if x_new > 0 && x_new < screen_width() as usize {
                                // Generate some Y-axis entropy
                                let mut y_new = py;
                                let y_rand = py + rand::gen_range(0, 2) as usize;

                                // Ensure the new Y-axis is valid
                                if y_rand > 0 && y_rand < screen_height() as usize { y_new = y_rand; }

                                // Figure out some context data
                                let is_water = world[px][py].variant == ParticleVariant::Water;
                                let is_swapping_with_water = world[x_new][y_new].active && world[x_new][y_new].variant == ParticleVariant::Water && !is_water;

                                // 'Sinking' only applies when it's Solid <---> Liquid or physically dense elements
                                if !is_swapping_with_water { y_new = py; }

                                // Ensure a neighbouring solid particle doesn't exist
                                if  !world[x_new][y_new].active || is_swapping_with_water {
                                    // Swap the particles (TODO: optimise!)
                                    world[x_new][y_new].variant = world[px][py].variant.clone();
                                    world[x_new][y_new].active = true;
                                    let new_id = world[x_new][y_new].id;

                                    // Swap IDs and prevent further updates via vec tracker
                                    world[x_new][y_new].id = world[px][py].id;
                                    updated_ids.push(world[x_new][y_new].id);
                                    world[px][py].id = new_id;

                                    // If a solid particle swaps with water: then the prior solid position must be filled with water
                                    world[px][py].active = is_swapping_with_water;
                                    if is_swapping_with_water {
                                        world[px][py].variant = ParticleVariant::Water;
                                    }
                                }
                            }
                        }
                    }
                }

                // Render updated particle state
                let zoomf = camera_zoom as f32;
                draw_rectangle((px32 * zoomf) + (camera_offset_x as f32 * zoomf), (py32 * zoomf) + (camera_offset_y as f32 * zoomf), zoomf, zoomf, world[px][py].get_colour());
            }
        }

        // Disable the UI lock if buttons were released
        if is_mouse_button_released(MouseButton::Left) {
            is_clicking_ui = false;
        }

        // Debugging UI
        if DEBUG {
            draw_text(format!("Sand: {}, Dirt: {}, Water: {}, Brick: {}", sand_count, dirt_count, water_count, brick_count).as_str(), 25.0, screen_height() / 2.0, 20.0, BLUE);
        }

        next_frame().await
    }
}