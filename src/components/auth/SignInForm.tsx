import { useEffect, useState } from "react";
import { Link } from "react-router-dom";
import { EyeCloseIcon, EyeIcon } from "../../icons";
import Label from "../form/Label";
import Input from "../form/input/InputField";
import Checkbox from "../form/input/Checkbox";
import Button from "../ui/button/Button";
import { useNavigate } from "react-router-dom";
import { useAuth } from "../../context/AuthContext";
import { notify } from "../../utils/utils";

export default function SignInForm() {
  const navigate = useNavigate();
  const { checkAuthentication, authenticateUser, error } = useAuth();
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [showPassword, setShowPassword] = useState(false);
  const [isChecked, setIsChecked] = useState(false);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [emailError, setEmailError] = useState("");
  const [passwordError, setPasswordError] = useState("");

  //   async function checkAuthentication() {
  //     try {
  //       const response = await fetch(`https://api.klaay.dev/me`, {
  //         method: "GET",
  //         headers: {
  //           Authorization: `Bearer ${jwtToken}`,
  //         },
  //       });

  //       if (response.ok) {
  //         setIsSignedIn(true);
  //       } else {
  //         setIsSignedIn(false);
  //       }
  //     } catch (err) {
  //       console.error("Error checking authentication:", err);
  //       setIsSignedIn(false);
  //     }
  //   }

  const validateInputs = () => {
    let isValid = true;
    if (!email) {
      setEmailError("Username is required.");
      isValid = false;
    } else {
      setEmailError("");
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
  //   async function authenticateUser() {
  //     if (!validateInputs()) return;
  //     try {
  //       const requestBody = {
  //         data: {
  //           type: "authorization",
  //           attributes: {
  //             email: username,
  //             password: password,
  //           },
  //           relationships: {
  //             account: {
  //               data: {
  //                 type: "account",
  //                 id: 264178000,
  //               },
  //             },
  //           },
  //         },
  //       };

  //       const response = await fetch(`https://api.klaay.dev/authenticate`, {
  //         method: "POST",
  //         headers: {
  //           "Content-Type": "application/vnd.api+json",
  //         },
  //         body: JSON.stringify(requestBody),
  //       });

  //       if (response.ok) {
  //         const responseData = await response.json();

  //         // Extract token from the response
  //         const token = responseData.data.attributes.token;
  //         setJwtToken(token);
  //         localStorage.setItem("jwtToken", token);

  //         // Extract user and account details for logging or further use
  //         const user = responseData.data.relationships.user.data;
  //         const account = responseData.data.relationships.account.data;

  //         console.log("Authenticated User:", user);
  //         console.log("Authenticated Account:", account);

  //         setIsSignedIn(true);
  //         setError("");
  //       } else {
  //         setError("Authentication failed. Please check your credentials.");
  //       }
  //     } catch (err) {
  //       console.error("Error authenticating user:", err);
  //       setError("An error occurred during authentication.");
  //     }
  //   }

  useEffect(() => {
    checkAuthentication();
  }, []);

  const handleSignIn = async () => {
    setIsSubmitting(true);
    try {
      if (!validateInputs()) return;
      await authenticateUser(email, password);
      if (!error) {
        notify("Sign in successful.", "success");
        navigate("/home");
      } else {
        notify(error, "error");
      }
    } catch (error) {
      notify(new String(error).toString(), "error");
    } finally {
      setIsSubmitting(false);
    }
  };

  return (
    <div className="flex flex-col flex-1">
      <div className="flex flex-col justify-center flex-1 w-full max-w-md mx-auto">
        <div>
          <div className="mb-5 sm:mb-8">
            <h1 className="mb-2 font-semibold text-gray-800 text-title-sm dark:text-white/90 sm:text-title-md">
              Sign In
            </h1>
            <p className="text-sm text-gray-500 dark:text-gray-400">
              Enter your email and password to sign in!
            </p>
          </div>
          <div>
            <form>
              <div className="space-y-6">
                <div>
                  <Label>
                    Email <span className="text-error-500">*</span>{" "}
                  </Label>
                  <Input
                    placeholder="info@gmail.com"
                    type="email"
                    value={email}
                    onChange={(e) => setEmail(e.target.value)}
                    error={!!emailError}
                  />
                </div>
                {emailError && (
                  <p className="text-error-500 text-sm mt-1">{emailError}</p>
                )}
                <div>
                  <Label>
                    Password <span className="text-error-500">*</span>{" "}
                  </Label>
                  <div className="relative">
                    <Input
                      type={showPassword ? "text" : "password"}
                      placeholder="Enter your password"
                      value={password}
                      onChange={(e) => setPassword(e.target.value)}
                        error={!!passwordError}
                    />
                    <span
                      onClick={() => setShowPassword(!showPassword)}
                      className="absolute z-30 -translate-y-1/2 cursor-pointer right-4 top-1/2"
                    >
                      {showPassword ? (
                        <EyeIcon className="fill-gray-500 dark:fill-gray-400 size-5" />
                      ) : (
                        <EyeCloseIcon className="fill-gray-500 dark:fill-gray-400 size-5" />
                      )}
                    </span>
                    {passwordError && (
                      <p className="text-error-500 text-sm mt-1">{passwordError}</p>
                    )}
                  </div>
                </div>
                <div className="flex items-center justify-between">
                  <div className="flex items-center gap-3">
                    <Checkbox checked={isChecked} onChange={setIsChecked} />
                    <span className="block font-normal text-gray-700 text-theme-sm dark:text-gray-400">
                      Keep me logged in
                    </span>
                  </div>
                  <Link
                    to="/forgot-password"
                    className="text-sm text-brand-500 hover:text-brand-600 dark:text-brand-400"
                  >
                    Forgot password?
                  </Link>
                </div>
                <div>
                  <Button
                    disabled={isSubmitting}
                    className="w-full"
                    size="sm"
                    onClick={handleSignIn}
                  >
                    Sign in
                  </Button>
                </div>
              </div>
            </form>

            <div className="mt-5">
              <p className="text-sm font-normal text-center text-gray-700 dark:text-gray-400 sm:text-start">
                Don&apos;t have an account? {""}
                <Link
                  to="/signup"
                  className="text-brand-500 hover:text-brand-600 dark:text-brand-400"
                >
                  Sign Up
                </Link>
              </p>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
