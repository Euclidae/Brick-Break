Brick Breaker
A classic Breakout-style game built in Rust using the SDL3 library. Control a paddle to bounce a ball and destroy bricks to score points. The game features a simple yet engaging gameplay loop with lives, scoring, and victory conditions.
Features

Gameplay: Move the paddle with Left/A or Right/D keys, launch the ball with Space, and reset the game after victory or game over.
Physics: Smooth ball movement with collision detection for paddle, walls, and bricks.
Scoring: Earn points by destroying bricks, with higher rows worth more points.
Lives: Start with 3 lives; lose a life if the ball falls off the screen.
Visuals: Color-coded bricks for each row, a light gray paddle, and a white ball on a dark blue background.

Prerequisites

Rust: Install Rust via rustup with the stable-x86_64-pc-windows-msvc toolchain (or equivalent for your platform).
SDL3: Requires SDL3 development libraries (version 3.1.x or compatible with sdl3 = "0.14.31").
Windows (MSVC):
Download SDL3-devel-*-VC.zip from SDL3 GitHub releases.
Copy SDL3.lib to C:\Users\<YourUser>\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib.
Copy SDL3.dll to the project root or a directory in your system PATH (e.g., C:\Windows\System32).


Linux: Install libsdl3-dev via your package manager (e.g., sudo apt install libsdl3-dev on Ubuntu).
macOS: Install SDL3 via Homebrew (brew install sdl3).



Installation

Clone the repository:git clone https://github.com/Euclidae/brick_breaker.git
cd brick_breaker


Ensure SDL3 libraries are set up as described in Prerequisites.
Build and run the project:cargo build
cargo run



Controls

Left Arrow or A: Move paddle left.
Right Arrow or D: Move paddle right.
Space: Launch the ball from the paddle or reset the game after victory/game over.
Escape: Quit the game.
![image](https://github.com/user-attachments/assets/6fd4aa4c-c8b7-4e36-aea6-dc2f4c1f7684)

Project Structure

src/main.rs: Main game logic, including game loop, rendering, and input handling.
Cargo.toml: Project configuration with SDL3 dependency (sdl3 = "0.14.31").
LICENSE: MIT License.

Contributing
Contributions are welcome! Feel free to open issues or submit pull requests on GitHub.
License
This project is licensed under the MIT License. See the LICENSE file for details.
Author

GitHub: Euclidae

