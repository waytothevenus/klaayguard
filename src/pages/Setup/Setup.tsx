import React, { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { invoke } from "@tauri-apps/api/core";

type InstallationState = "checking" | "installing" | "success" | "error";

const Setup: React.FC = () => {
  const navigate = useNavigate();
  const [installationState, setInstallationState] = useState<InstallationState>("checking");
  const [errorMessage, setErrorMessage] = useState<string>("");

  useEffect(() => {
    handleAutomaticInstallation();
  }, []);

  const handleAutomaticInstallation = async () => {
    try {
      setInstallationState("installing");
      setErrorMessage("");

      // Run the automatic installation
      await invoke("auto_install_osquery");
      
      setInstallationState("success");
      
      // Wait a moment to show success state, then redirect
      setTimeout(() => {
        navigate("/signin");
      }, 2000);
      
    } catch (error) {
      console.error("Installation failed:", error);
      setInstallationState("error");
      setErrorMessage(error instanceof Error ? error.message : "Installation failed");
    }
  };

  const handleRetry = () => {
    setInstallationState("checking");
    setErrorMessage("");
    handleAutomaticInstallation();
  };

  const renderContent = () => {
    switch (installationState) {
      case "checking":
        return (
          <div className="text-center">
            <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600 mx-auto mb-4"></div>
            <h2 className="text-xl font-semibold text-gray-800 mb-2">Checking System</h2>
            <p className="text-gray-600">Verifying osquery installation status...</p>
          </div>
        );

      case "installing":
        return (
          <div className="text-center">
            <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600 mx-auto mb-4"></div>
            <h2 className="text-xl font-semibold text-gray-800 mb-2">Installing osquery</h2>
            <p className="text-gray-600">Please wait while we install osquery on your system...</p>
            <p className="text-sm text-gray-500 mt-2">This may take a few minutes</p>
          </div>
        );

      case "success":
        return (
          <div className="text-center">
            <div className="w-12 h-12 bg-green-100 rounded-full flex items-center justify-center mx-auto mb-4">
              <svg className="w-6 h-6 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
              </svg>
            </div>
            <h2 className="text-xl font-semibold text-gray-800 mb-2">Installation Complete!</h2>
            <p className="text-gray-600">osquery has been successfully installed on your system.</p>
            <p className="text-sm text-gray-500 mt-2">Redirecting to sign in...</p>
          </div>
        );

      case "error":
        return (
          <div className="text-center">
            <div className="w-12 h-12 bg-red-100 rounded-full flex items-center justify-center mx-auto mb-4">
              <svg className="w-6 h-6 text-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
              </svg>
            </div>
            <h2 className="text-xl font-semibold text-gray-800 mb-2">Installation Failed</h2>
            <p className="text-gray-600 mb-4">{errorMessage}</p>
            <div className="space-y-2">
              <button
                onClick={handleRetry}
                className="bg-blue-600 text-white px-6 py-2 rounded-md hover:bg-blue-700 transition-colors"
              >
                Try Again
              </button>
              <div>
                <button
                  onClick={() => navigate("/signin")}
                  className="text-blue-600 hover:text-blue-700 text-sm"
                >
                  Continue without osquery
                </button>
              </div>
            </div>
          </div>
        );

      default:
        return null;
    }
  };

  return (
    <div className="min-h-screen bg-gray-50 flex items-center justify-center p-6">
      <div className="max-w-md w-full bg-white rounded-lg shadow-md p-8">
        <div className="text-center mb-6">
          <h1 className="text-2xl font-bold text-gray-900 mb-2">Welcome to KlaayGuard</h1>
          <p className="text-gray-600">
            We're setting up your system for security monitoring
          </p>
        </div>
        
        {renderContent()}
      </div>
    </div>
  );
};

export default Setup; 