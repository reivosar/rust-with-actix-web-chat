Real-Time Chat Application with Rust and React

This project is a real-time chat application utilizing Rust with Actix Web for the backend and React for the frontend. It demonstrates the use of WebSockets for bi-directional, real-time, event-based communication.

Features

	•	Real-time messaging between clients through a Rust-powered WebSocket server.
	•	Simple and intuitive React user interface.
	•	Docker-compose integration for easy setup and deployment.

Prerequisites

	•	Docker and Docker Compose
	•	Node.js and npm (for local development)

Getting Started

Using Docker Compose

	1.	Clone the repository:

git clone <repository-url>
cd <repository-directory>

	2.	Build and run the application using Docker Compose:

docker-compose up --build

This command builds the React app and Rust server from their respective Dockerfiles and starts them. The React app will be accessible on http://localhost:5173, and the Rust WebSocket server listens on ws://localhost:3030.

Development Setup

For development purposes, you might want to run the frontend and backend separately to take advantage of hot reloading.

Running the Backend

Navigate to the backend directory and run:

cd backend
cargo run

This will start the Rust WebSocket server on localhost:3030.

Running the Frontend

Navigate to the frontend directory, install dependencies, and start the development server:

cd frontend
npm install
npm start

This will start the React development server, which should automatically open http://localhost:5173 in your default web browser.

Usage

Once the application is running, open http://localhost:5173 in your web browser to access the chat interface. Type a message and press enter or click the send button to see real-time communication in action.
