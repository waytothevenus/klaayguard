import React, { createContext, useState, useContext, useEffect } from "react";
import { useNavigate } from "react-router-dom";

type AuthContextType = {
  token: string | null;
  isAuthenticated: boolean;
  isAccountConfigRequired: boolean;
  error: string;
  checkAuthentication: () => void;
  authenticateUser: (
    username: string,
    password: string,
    accountId: string | undefined
  ) => void;
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
  const [isAccountConfigRequired, setIsAccountConfigRequired] = useState(false);

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

  async function authenticateUser(
    username: string,
    password: string,
    accountId: string | undefined
  ) {
    try {
      const requestBody = accountId
        ? {
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
                    id: accountId,
                  },
                },
              },
            },
          }
        : {
            data: {
              type: "authorization",
              attributes: {
                email: username,
                password: password,
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

      if (response.status === 201) {
        const responseData = await response.json();

        const token = responseData.data.attributes.token;
        if (token) {
          setToken(token);
          localStorage.setItem("jwtToken", token);
        }

        const user = responseData.included?.[0]?.attributes;
        const account = responseData.data.relationships?.user?.data;

        if (account) {
          setIsAccountConfigRequired(true);
          navigate("/account-setup", {
            state: {
              username: username,
              password: password,
            },
          });
        } else {
          setIsAccountConfigRequired(false);
          navigate("/home");
        }

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

    console.log("Account Config Required:", isAccountConfigRequired);
    if (savedToken) {
      if (isAccountConfigRequired) {
        console.log("savedToken", savedToken);
        setToken(savedToken);
        setIsAuthenticated(true);
        if (currentPath == "/signin") {
          navigate("/account-setup");
        }
      } else {
        console.log("savedToken", savedToken);
        setToken(savedToken);
        setIsAuthenticated(true);
        if (currentPath == "/signin") {
          navigate("/home");
        }
      }
    } else {
      navigate("/signin");
    }
  }, []);

  const logout = () => {
    setToken(null);
    setIsAuthenticated(false);
    localStorage.removeItem("jwtToken");
    navigate("/signin");
  };

  return (
    <AuthContext.Provider
      value={{
        token,
        isAuthenticated,
        isAccountConfigRequired,
        error,
        checkAuthentication,
        authenticateUser,
        logout,
      }}
    >
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
