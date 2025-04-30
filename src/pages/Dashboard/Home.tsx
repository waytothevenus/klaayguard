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
  interface ConfigData {
    type: string;
    id: string;
  }

  interface Config {
    data: ConfigData[];
  }

  type DeepRecord =
    | string
    | number
    | boolean
    | null
    | undefined
    | DeepRecord[]
    | { [key: string]: DeepRecord };

  const [config, setConfig] = useState<Config | null>(null);
  const [queryResult, setQueryResult] = useState<DeepRecord | null>(null);
  const [error, setError] = useState("");

  useEffect(() => {
    checkInstallation();
  }, []);

  useEffect(() => {
    fetchConfiguration();

    const interval = setInterval(() => {
      fetchConfiguration();
      postDataToApi();
    }, 15 * 60 * 1000); // 15 minutes

    return () => clearInterval(interval);
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
        const data = (await response.json()) as Config;
        setConfig(data);
        const tableNames = data.data.map((item) => item.id);
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
        body: JSON.stringify({ data: queryResult }),
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
      const response = await invoke<DeepRecord | null>("execute_query", {
        tableNames,
      });
      if (!response) {
        setError("No data received from the query.");
        return;
      }
      setQueryResult(response);
      console.log("Query executed successfully:", queryResult);
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
        <CloseIcon />
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
          <div className="overflow-x-auto bg-white shadow-md rounded-lg p-4 mb-6">
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
                {config?.data?.map((item, index: number) => (
                  <tr
                    key={index}
                    className={`${
                      index % 2 === 0 ? "bg-gray-100" : "bg-white"
                    }`}
                  >
                    <td className="border px-4 py-2">{item.type}</td>
                    <td className="border px-4 py-2">{item.id}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
          <div className="overflow-x-auto bg-white shadow-md rounded-lg p-4 mb-6">
            <h2 className="text-xl font-semibold text-gray-700 mb-2">
              Query Result
            </h2>
            {queryResult && typeof queryResult === "object" ? (
              Object.entries(queryResult).map(([tableName, tableData]) => (
                <div key={tableName} className="mb-8">
                  <h3 className="text-lg font-medium text-gray-800 mb-3 capitalize">
                    {tableName.replace(/_/g, " ")}
                  </h3>
                  {Array.isArray(tableData) && tableData.length > 0 ? (
                    <div className="overflow-x-auto">
                      <table className="min-w-full divide-y divide-gray-200">
                        <thead className="bg-gray-50">
                          <tr>
                            {tableData[0] &&
                              Object.keys(tableData[0]).map((key) => (
                                <th
                                  key={key}
                                  scope="col"
                                  className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                                >
                                  {key}
                                </th>
                              ))}
                          </tr>
                        </thead>
                        <tbody className="bg-white divide-y divide-gray-200">
                          {tableData.map((row, rowIndex) => (
                            <tr key={rowIndex}>
                              {row &&
                                Object.values(row).map((value, colIndex) => (
                                  <td
                                    key={colIndex}
                                    className="px-6 py-4 whitespace-nowrap text-sm text-gray-500"
                                  >
                                    {typeof value === "object" ? (
                                      <pre className="text-xs">
                                        {JSON.stringify(value, null, 2)}
                                      </pre>
                                    ) : (
                                      String(value)
                                    )}
                                  </td>
                                ))}
                            </tr>
                          ))}
                        </tbody>
                      </table>
                    </div>
                  ) : (
                    <p className="text-gray-500 italic">No data available</p>
                  )}
                </div>
              ))
            ) : (
              <pre className="whitespace-pre-wrap">
                {JSON.stringify(queryResult, null, 2)}
              </pre>
            )}
          </div>
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
