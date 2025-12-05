Author : Logan Callery <br>
Email : lcallery@pdx.edu <br>
Developed for CS523 - Rust Programming 


# Lord of the Lawn

Lord of the Lawn is a Rust-based clicker-style terminal game where you run a tiny lawn mowing business. Each “click” represents mowing a certain number of square feet of grass. As you complete jobs and earn money, you can buy upgrades that increase your mowing efficiency and eventually unlock an automower that cuts grass for you over time.

The project started as a simple terminal-only prototype so I could focus on core game logic, timing, and upgrades before worrying about any graphical UI.

---

## What Was Built

### Core Gameplay

- **Manual mowing loop**
  - The player mows lawns by pressing `Enter`.
  - Each press mows a certain number of square feet (`mower_efficiency`).
  - Each lawn has a **randomly generated size** in square feet.
  - When a lawn is completely mowed, the player gets paid for the full job.

- **Economy**
  - The player has a running total of **money earned**.
  - Lawns pay based on:
    - `lawn.size` (total square footage)
    - `payout_per_sqft` (price per square foot for that lawn)
  - The price per square foot is stored both:
    - Globally in the `Game` as `price_per_sqft` (current rate)
    - Per-lawn as `payout_per_sqft` so that existing lawns keep their original rate even if pricing changes later.

- **Upgrades**
  - **Manual mower upgrade (`U`)**
    - Increases `mower_efficiency` (square feet per click).
    - Each upgrade has a **cost** (`mower_upgrade_cost`).
    - After upgrading, the cost increases (currently doubles) so each subsequent upgrade is more expensive.
  - **Automower upgrade (`A`)**
    - Increases `auto_mower_rate` (square feet per second).
    - Each upgrade has a **cost** (`auto_mower_upgrade_cost`).
    - The cost increases after each upgrade.
    - The automower contributes progress automatically over time.

- **Terminal UI**
  - Displays current state on a single status line:
    - Current lawn progress: `mowed / size`
    - Total money
    - Price per square foot
    - Manual mower power and its upgrade cost
    - Automower rate and its upgrade cost
  - Key bindings:
    - `Enter` – mow manually
    - `U` – upgrade manual mower
    - `A` – upgrade automower
    - `Q` – quit the game

---

## How It Works (Implementation)

### Project Structure

```text
src/
├── main.rs   // Entry point, sets up and runs the Game
├── game.rs   // Core game loop, economy, upgrades, automower, UI
└── lawn.rs   // Lawn struct and its behavior, plus unit tests
```

### Things to Note

Chatgpt Model 5.1  was used to format the Rest Docs as well as generate ideas for function tests. 