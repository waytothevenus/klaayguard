import React, { useEffect, useState } from "react";
import { useAuth } from "../../context/AuthContext";
import { notify } from "../../utils/utils";
import { useLocation, useNavigate } from "react-router-dom";

interface Account {
  id: string;
  type: string;
  attributes: {
    name: string;
  };
}

interface AccountSelectorProps {
  selectedAccountId?: string;
}

export const AccountSetupForm: React.FC<AccountSelectorProps> = ({
  selectedAccountId,
}) => {
  const [accounts, setAccounts] = useState<Account[]>([]);
  const [loading, setLoading] = useState(true);
  const [selected, setSelected] = useState<string | undefined>(
    selectedAccountId
  );

  const { token, authenticateUser } = useAuth();
  const location = useLocation();
  const navigate = useNavigate();

  const { username, password } = location.state || {};

  useEffect(() => {
    const fetchAccounts = async () => {
      setLoading(true);
      try {
        const res = await fetch("https://api.klaay.dev/accounts", {
          method: "GET",
          headers: {
            Authorization: `Bearer ${token}`,
          },
        });
        if (!res.ok) throw new Error("Failed to fetch accounts");
        const data = await res.json();
        setAccounts(data);
      } catch (err) {
        setAccounts([]);
      } finally {
        setLoading(false);
      }
    };
    fetchAccounts();
  }, [token]);

  const handleSignIn = async () => {
    if (!selected) {
      notify("Please select an account to continue.", "warning");
      return;
    }
    if (!username || !password) {
      notify("Missing username or password.", "error");
      return;
    }
    try {
      await authenticateUser(username, password, selected);
      navigate("/home");
    } catch (error) {
      notify(new String(error).toString(), "error");
    }
  };

  return (
    <div>
      <label className="block mb-2 font-medium">Select Account</label>
      {loading ? (
        <p>Loading accounts...</p>
      ) : (
        <>
          <select
            className="border px-3 py-2 rounded w-full"
            value={selected || ""}
            onChange={(e) => setSelected(e.target.value)}
          >
            <option value="">-- Select an account --</option>
            {accounts.map((account) => (
              <option key={account.id} value={account.id}>
                {account.attributes.name}
              </option>
            ))}
          </select>
          <button
            className="mt-4 bg-blue-600 text-white px-4 py-2 rounded disabled:opacity-50"
            onClick={handleSignIn}
            disabled={!selected}
            type="button"
          >
            Continue
          </button>
        </>
      )}
    </div>
  );
};
