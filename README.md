# DeFi Blackjack

A decentralized version of Blackjack where players interact via a WebSocket server. The core logic is implemented in Rust, with a frontend using React and Rust (WASM) for performance. Smart contracts, written in Solidity, manage the DeFi aspects of the game, handling bets, payouts, and game logic securely on the blockchain.

## Technologies

- **Rust**: Backend game logic, WebSocket server, and performance-critical frontend code (via WASM).
- **React**: Frontend UI for user interaction.
- **Solidity**: Smart contracts for DeFi functionality (bets, payouts).
- **Hardhat**: For Ethereum smart contract development.
- **Axum**: Web framework for the Rust WebSocket server.
- **Serde**: For serialization of game data.

## Project Structure

```
blackjack-defi/
│── Cargo.toml         # Workspace configuration
│── game_logic/        # Core game logic (Rust library)
│── server/            # WebSocket server (Rust binary)
│── contracts/         # Solidity smart contracts
│── frontend/          # React frontend with Rust WASM integration
│── blockchain/        # Ethereum deployment scripts (Hardhat)
```

## Overview

This project allows users to play a decentralized version of Blackjack. The backend server uses Rust to manage game state and WebSocket communication with clients. Players interact with the game through a React frontend, with critical game logic running via Rust and WebAssembly. The smart contracts on Ethereum ensure that the game's financial transactions (bets and payouts) are secure, transparent, and decentralized.
