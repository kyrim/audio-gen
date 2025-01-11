use std::sync::{Arc, Mutex};

use rodio::Sink;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod sine_wave;
mod amp_adsr;
mod traits;

mod polysynth;
use polysynth::PolySynth;

mod rodio_adapter;
use rodio_adapter::RodioAdapter;

fn main() {
    // 1) Create rodio output (unchanged)
    let (_stream, handle) = rodio::OutputStream::try_default()
        .expect("Failed to get default audio output");
    let sink = Sink::try_new(&handle).expect("Failed to create Sink");

    // 2) Create a poly synth
    let poly = PolySynth::new(44100, 3);
    let poly_arc = Arc::new(Mutex::new(poly));

    // 3) Wrap in RodioAdapter & append to sink
    let adapter = RodioAdapter::new(poly_arc.clone(), 44100);
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

    // 7) Run event loop.
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll; // Change to Wait if you prefer sleeping

        if let Event::WindowEvent { event, .. } = event { match event {
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

                // For demonstration we choose to print which key is pressed or released.
                match state {
                    ElementState::Pressed => {
                        println!("Pressed: {:?}", key_code);

                        // We use the ReceivedCharacter event for actual characters
                        // normally. However, here we map using VirtualKeyCode.
                        // Depending on your keyboard layout, you might prefer to handle
                        // note_on in ReceivedCharacter. Adjust as needed.
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
                            if let Some((_, freq)) = freq_map.iter().find(|(k, _)| *k == ch)
                            {
                                let mut locked = poly_arc.lock().unwrap();
                                locked.play(*freq);
                            }
                        }
                    }
                    ElementState::Released => {
                        println!("Released: {:?}", key_code);
                        // You can choose to release the note when key is released.
                        // If you want the note to be tied directly to the key press and release,
                        // call note_off() here.
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
                            if let Some((_, freq)) = freq_map.iter().find(|(k, _)| *k == ch)
                            {
                                let mut locked = poly_arc.lock().unwrap();
                                locked.stop(*freq);
                            }
                        }
                    }
                }
            }

            // Process text input events (if needed)
            WindowEvent::ReceivedCharacter(ch) => {
                // This is another way of handling actual character input.
                // For instance, if the user presses 'q' via text input.
                if ch == 'q' {
                    println!("Exiting. Goodbye!");
                    *control_flow = ControlFlow::Exit
                }
                // Optionally handle other characters here if necessary.
            }

            // Exit on window close request.
            WindowEvent::CloseRequested => {
                println!("Window close requested, exiting.");
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        } }
    });
}
