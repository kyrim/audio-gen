use std::collections::HashSet;
use std::sync::{Arc, Mutex};

use rodio::Sink;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod sine_wave;
mod saw_wave;
mod square_wave;
mod adsr_envelope;
mod traits;

mod polysynth;
use polysynth::PolySynth;

mod voice;
mod gain;
mod ramp_envelope;

mod rodio_adapter;
use rodio_adapter::RodioAdapter;

fn main() {
    // 1) Create rodio output (unchanged)
    let (_stream, handle) = rodio::OutputStream::try_default()
        .expect("Failed to get default audio output");
    let sink = Sink::try_new(&handle).expect("Failed to create Sink");
    
    // 2) Create a poly synth
    let poly = PolySynth::new(48000, 3);
    let poly_arc = Arc::new(Mutex::new(poly));

    // 3) Wrap in RodioAdapter & append to sink
    let adapter = RodioAdapter::new(poly_arc.clone(), 48000);
    sink.append(adapter);
    sink.play();

    // 4) Print instructions in the console
    println!("Press keys [z, x, c, v, b, n, m] to play notes!");
    println!("Release the key to stop the note (or press 's' to trigger note_off).");
    println!("Press [q] or [Esc] to exit.");

    // 5) Key -> frequency mapping
    let freq_map = [
        ('z', 220.0),  // A3
        ('x', 246.94), // B3
        ('c', 261.63), // C4
        ('v', 293.66), // D4
        ('b', 329.63), // E4
        ('n', 349.23), // F4
        ('m', 392.00), // G4
    ];

    // 6) Create a winit event loop and window.
    let event_loop = EventLoop::new();
    let _window = WindowBuilder::new()
        .with_title("PolySynth")
        .build(&event_loop)
        .unwrap();

    // Track active keys
    let mut active_keys = HashSet::new();

    // 7) Run event loop.
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll; // Change to Wait if you prefer sleeping

        if let Event::WindowEvent { event, .. } = event {
            match event {
                // Process key input events
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state,
                            virtual_keycode: Some(key_code),
                            ..
                        },
                    ..
                } => {
                    // Exit if user presses q or Esc.
                    if key_code == VirtualKeyCode::Escape || key_code == VirtualKeyCode::Q {
                        println!("Exiting. Goodbye!");
                        *control_flow = ControlFlow::Exit;
                        return;
                    }

                    // Handle key press and release
                    let ch = match key_code {
                        VirtualKeyCode::Z => 'z',
                        VirtualKeyCode::X => 'x',
                        VirtualKeyCode::C => 'c',
                        VirtualKeyCode::V => 'v',
                        VirtualKeyCode::B => 'b',
                        VirtualKeyCode::N => 'n',
                        VirtualKeyCode::M => 'm',
                        _ => '\0',
                    };

                    if ch != '\0' {
                        match state {
                            ElementState::Pressed => {
                                if !active_keys.contains(&ch) {
                                    println!("Pressed: {:?}", key_code);
                                    if let Some((_, freq)) =
                                        freq_map.iter().find(|(k, _)| *k == ch)
                                    {
                                        let mut locked = poly_arc.lock().unwrap();
                                        locked.play(*freq);
                                    }
                                    active_keys.insert(ch);
                                }
                            }
                            ElementState::Released => {
                                if active_keys.contains(&ch) {
                                    println!("Released: {:?}", key_code);
                                    if let Some((_, freq)) =
                                        freq_map.iter().find(|(k, _)| *k == ch)
                                    {
                                        let mut locked = poly_arc.lock().unwrap();
                                        locked.stop(*freq);
                                    }
                                    active_keys.remove(&ch);
                                }
                            }
                        }
                    }
                }

                // Exit on window close request.
                WindowEvent::CloseRequested => {
                    println!("Window close requested, exiting.");
                    *control_flow = ControlFlow::Exit;
                }
                _ => {}
            }
        }
    });
}
