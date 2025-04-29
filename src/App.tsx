import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [isSignedIn, setIsSignedIn] = useState(false);
  const [jwtToken, setJwtToken] = useState("");
  const [config, setConfig] = useState(null);
  const [error, setError] = useState("");
  const [osqueryInstalled, setOsqueryInstalled] = useState<boolean | null>(
    null
  );
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [usernameError, setUsernameError] = useState("");
  const [passwordError, setPasswordError] = useState("");

  useEffect(() => {
    checkInstallation();
  }, []);

  const checkInstallation = async () => {
    try {
      const installed = await invoke<boolean>("check_osquery");
      setOsqueryInstalled(installed);
    } catch (err) {
      setError(`Error checking installation: ${err}`);
    }
  };

  const handleInstall = async () => {
    try {
      await invoke("install_osquery");
      await checkInstallation();
    } catch (err) {
      setError(`Installation failed: ${err}`);
    }
  };

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

  const validateInputs = () => {
    let isValid = true;
    if (!username) {
      setUsernameError("Username is required.");
      isValid = false;
    } else {
      setUsernameError("");
    }

    if (!password) {
      setPasswordError("Password is required.");
      isValid = false;
    } else {
      setPasswordError("");
    }

    return isValid;
  };

  // Step 2: Authenticate user
  async function authenticateUser() {
    if (!validateInputs()) return;
    try {
      const requestBody = {
        data: {
          type: "authorization",
          attributes: {
            email: username,
            password: password,
          },
          relationships: {
            account: {
              data: {
                type: "account",
                id: 264178000,
              },
            },
          },
        },
      };

      const response = await fetch(`https://api.klaay.dev/authenticate`, {
        method: "POST",
        headers: {
          "Content-Type": "application/vnd.api+json",
        },
        body: JSON.stringify(requestBody),
      });

      if (response.ok) {
        const responseData = await response.json();

        // Extract token from the response
        const token = responseData.data.attributes.token;
        setJwtToken(token);
        localStorage.setItem("jwtToken", token);

        // Extract user and account details for logging or further use
        const user = responseData.data.relationships.user.data;
        const account = responseData.data.relationships.account.data;

        console.log("Authenticated User:", user);
        console.log("Authenticated Account:", account);

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
        const tableNames = data.data.map((item: any) => item.id);
        console.log("Request Query for these tables: ", tableNames);
        await execute_query(tableNames);
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

  async function execute_query(tableNames: string[]) {
    try {
      const response = await invoke("execute_query", { tableNames });
      console.log("Query executed successfully:", response);
    } catch (error) {
      console.error("Error executing query:", error);
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

  if (osqueryInstalled === null) {
    return <div>Checking osquery installation...</div>;
  }

  if (!osqueryInstalled) {
    return (
      <div>
        <h2>Osquery not installed</h2>
        <button onClick={handleInstall}>Install Osquery</button>
        {error && <div className="error">{error}</div>}
      </div>
    );
  }

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
              className={`mt-1 block w-full px-4 py-2 border ${
                usernameError ? "border-red-500" : "border-gray-300"
              } rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 sm:text-sm`}
            />
            {usernameError && (
              <p className="text-red-500 text-sm mt-1">{usernameError}</p>
            )}
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
              className={`mt-1 block w-full px-4 py-2 border ${
                passwordError ? "border-red-500" : "border-gray-300"
              } rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 sm:text-sm`}
            />
            {passwordError && (
              <p className="text-red-500 text-sm mt-1">{passwordError}</p>
            )}
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
