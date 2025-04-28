import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [isSignedIn, setIsSignedIn] = useState(false);
  const [jwtToken, setJwtToken] = useState("");
  const [config, setConfig] = useState(null);
  const [error, setError] = useState("");
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");

  // Step 1: Check if user is authenticated
  async function checkAuthentication() {
    try {
      const response = await fetch(`https://api.klaay.dev/me`, {
        method: "GET",
        headers: {
          Authorization: `Bearer ${jwtToken}`,
        },
      });

      if (response.ok) {
        setIsSignedIn(true);
      } else {
        setIsSignedIn(false);
      }
    } catch (err) {
      console.error("Error checking authentication:", err);
      setIsSignedIn(false);
    }
  }

  // Step 2: Authenticate user
  async function authenticateUser() {
    try {
      const response = await fetch(`https://api.klaay.dev/authenticate`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ username, password }),
      });

      if (response.ok) {
        const data = await response.json();
        setJwtToken(data.token);
        setIsSignedIn(true);
        setError("");
      } else {
        setError("Authentication failed. Please check your credentials.");
      }
    } catch (err) {
      console.error("Error authenticating user:", err);
      setError("An error occurred during authentication.");
    }
  }

  // Step 3: Fetch configuration every 15 minutes
  async function fetchConfiguration() {
    try {
      const response = await fetch(`https://api.klaay.dev/klaayguard/config`, {
        method: "GET",
        headers: {
          Authorization: `Bearer ${jwtToken}`,
        },
      });

      if (response.ok) {
        const data = await response.json();
        setConfig(data);
        await invoke("process_config", { config: data });
      } else {
        console.error("Failed to fetch configuration.");
      }
    } catch (err) {
      console.error("Error fetching configuration:", err);
    }
  }

  // Step 4: Post data to API
  async function postDataToApi() {
    try {
      const response = await fetch(`https://api.klaay.dev/klaayguard/data`, {
        method: "POST",
        headers: {
          Authorization: `Bearer ${jwtToken}`,
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ data: "osquery results" }),
      });

      if (!response.ok) {
        console.error("Failed to post data to API.");
      }
    } catch (err) {
      console.error("Error posting data to API:", err);
    }
  }

  useEffect(() => {
    if (isSignedIn) {
      fetchConfiguration();

      const interval = setInterval(() => {
        fetchConfiguration();
        postDataToApi();
      }, 15 * 60 * 1000); // 15 minutes

      return () => clearInterval(interval);
    }
  }, [isSignedIn]);

  useEffect(() => {
    setJwtToken(localStorage.getItem("jwtToken") || "");
    if (jwtToken) {
      checkAuthentication();
    }
  });

  return (
    <main className="flex items-center justify-center min-h-screen bg-gray-100">
      {!isSignedIn ? (
        <form
          className="bg-white shadow-lg rounded-lg p-8 w-full max-w-md"
          onSubmit={(e) => {
            e.preventDefault();
            authenticateUser();
          }}
        >
          <h1 className="text-2xl font-bold text-gray-800 mb-6 text-center">
            Sign In
          </h1>
          <div className="mb-4">
            <label
              htmlFor="username"
              className="block text-sm font-medium text-gray-700"
            >
              Username
            </label>
            <input
              id="username"
              type="text"
              placeholder="Enter your username"
              value={username}
              onChange={(e) => setUsername(e.target.value)}
              className="mt-1 block w-full px-4 py-2 border border-gray-300 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 sm:text-sm"
            />
          </div>
          <div className="mb-6">
            <label
              htmlFor="password"
              className="block text-sm font-medium text-gray-700"
            >
              Password
            </label>
            <input
              id="password"
              type="password"
              placeholder="Enter your password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              className="mt-1 block w-full px-4 py-2 border border-gray-300 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 sm:text-sm"
            />
          </div>
          {error && <p className="text-red-500 text-sm mb-4">{error}</p>}
          <button
            type="submit"
            className="w-full bg-blue-600 text-white py-2 px-4 rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
          >
            Sign In
          </button>
        </form>
      ) : (
        <div className="text-center">
          <h1 className="text-3xl font-bold text-gray-800 mb-4">Welcome!</h1>
          <p className="text-gray-600">
            Configuration:{" "}
            <span className="font-mono text-sm text-gray-800">
              {JSON.stringify(config)}
            </span>
          </p>
        </div>
      )}
    </main>
  );
}

export default App;
