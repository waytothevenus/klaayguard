import { BrowserRouter, Routes, Route, Navigate } from "react-router-dom";
import SignIn from "./pages/AuthPages/SignIn";
import { Home } from "./pages/Dashboard/Home";
import { AuthProvider } from "./context/AuthContext";
import { ToastContainer } from "react-toastify";

export default function App() {
  return (
    <BrowserRouter>
      <ToastContainer />
      <AuthProvider>
        <Routes>
          <Route path="/" element={<Navigate to="/home" />} />
          <Route path="/home" element={<Home />} />
          <Route path="/signin" element={<SignIn />} />
        </Routes>
      </AuthProvider>
    </BrowserRouter>
  );
}