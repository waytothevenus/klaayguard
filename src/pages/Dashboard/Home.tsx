import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { invoke } from "@tauri-apps/api/core";
import { useAuth } from "../../context/AuthContext";
import { CloseIcon } from "../../icons";

export const Home = () => {
  const navigate = useNavigate();
  const { token, logout } = useAuth();
  const [osqueryInstalled, setOsqueryInstalled] = useState<boolean | null>(
    null
  );
  const [config, setConfig] = useState<any>(null);
  const [error, setError] = useState("");

  useEffect(() => {
    checkInstallation();
    if(osqueryInstalled){

        fetchConfiguration();
        
        const interval = setInterval(() => {
            fetchConfiguration();
            postDataToApi();
        }, 15 * 60 * 1000); // 15 minutes
        
        return () => clearInterval(interval);
    }
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

  async function fetchConfiguration() {
    try {
      const response = await fetch(`https://api.klaay.dev/klaayguard/config`, {
        method: "GET",
        headers: {
          Authorization: `Bearer ${token}`,
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
          Authorization: `Bearer ${token}`,
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

  const handleLogout = () => {
    logout();
    navigate("/login");
  };

  return (
    <div className="home relative p-6 bg-gray-100 min-h-screen">
      {/* Logout Button */}
      <button
        onClick={handleLogout}
        className="absolute top-4 right-4 text-gray-600 hover:text-red-600"
        title="Logout"
      >
        <CloseIcon size={24} />
      </button>

      <h1 className="text-2xl font-bold text-gray-800 mb-4">
        Welcome to the Dashboard
      </h1>
      {error && <p className="text-red-500 mb-4">{error}</p>}
      {osqueryInstalled === null ? (
        <p>Checking osquery installation...</p>
      ) : osqueryInstalled ? (
        <>
          <p className="text-green-600 mb-4">Osquery is installed.</p>
          <h2 className="text-xl font-semibold text-gray-700 mb-2">
            Configuration
          </h2>
          <table className="table-auto w-full bg-white shadow-md rounded-lg mb-6">
            <thead>
              <tr className="bg-gray-200 text-gray-700">
                <th className="px-4 py-2">Type</th>
                <th className="px-4 py-2">ID</th>
              </tr>
            </thead>
            <tbody>
              {config?.data?.map((item: any, index: number) => (
                <tr
                  key={index}
                  className={`${index % 2 === 0 ? "bg-gray-100" : "bg-white"}`}
                >
                  <td className="border px-4 py-2">{item.type}</td>
                  <td className="border px-4 py-2">{item.id}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </>
      ) : (
        <button
          onClick={handleInstall}
          className="bg-blue-600 text-white px-4 py-2 rounded-md hover:bg-blue-700"
        >
          Install Osquery
        </button>
      )}
    </div>
  );
};
