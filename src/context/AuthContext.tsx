import React, { createContext, useState, useContext, useEffect } from "react";
import { useNavigate } from "react-router-dom";

type AuthContextType = {
  token: string | null;
  isAuthenticated: boolean;
  error: string;
  checkAuthentication: () => void;
  authenticateUser: (username: string, password: string) => void;
  logout: () => void;
};

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export const AuthProvider: React.FC<{ children: React.ReactNode }> = ({
  children,
}) => {
  const navigate = useNavigate();
  const [token, setToken] = useState<string | null>(null);
  const [error, setError] = useState<string>("");
  const [isAuthenticated, setIsAuthenticated] = useState(false);

  async function checkAuthentication() {
    try {
      const response = await fetch(`https://api.klaay.dev/me`, {
        method: "GET",
        headers: {
          Authorization: `Bearer ${token}`,
        },
      });

      if (response.ok) {
        setIsAuthenticated(true);
      } else {
        setIsAuthenticated(false);
      }
    } catch (err) {
      console.error("Error checking authentication:", err);
      setIsAuthenticated(false);
    }
  }
  

  async function authenticateUser(username: string, password: string) {
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

        const token = responseData.data.attributes.token;
        setToken(token);
        localStorage.setItem("jwtToken", token);

        const user = responseData.data.relationships.user.data;
        const account = responseData.data.relationships.account.data;

        console.log("Authenticated User:", user);
        console.log("Authenticated Account:", account);

        setIsAuthenticated(true);
        setError("");
      } else {
        setError("Authentication failed. Please check your credentials.");
      }
    } catch (err) {
      console.error("Error authenticating user:", err);
      setError("An error occurred during authentication.");
    }
  }

  useEffect(() => {
    const savedToken = localStorage.getItem("jwtToken");
    const currentPath = window.location.pathname;
    if (savedToken) {
      setToken(savedToken);
      setIsAuthenticated(true);
      if (currentPath == "/signin" || currentPath == "/signup") {
        navigate("/home");
      }
    } else {
      if (currentPath !== "/signup" && currentPath !== "/reset-password") {
        navigate("/signin");
      }
    }
  }, []);

  const logout = () => {
    setToken(null);
    setIsAuthenticated(false);
    localStorage.removeItem("jwtToken");
    navigate("/signin");
  };

  return (
    <AuthContext.Provider value={{ token, isAuthenticated, error, checkAuthentication, authenticateUser, logout }}>
      {children}
    </AuthContext.Provider>
  );
};

export const useAuth = (): AuthContextType => {
  const context = useContext(AuthContext);
  if (context === undefined) {
    throw new Error("useAuth must be used within an AuthProvider");
  }
  return context;
};
