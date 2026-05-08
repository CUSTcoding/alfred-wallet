import { Routes, Route } from "react-router-dom";

import AutoPage from "../features/AuthPage";
import Login from "../features/auth/login/Login";

import Register from "../features/auth/register/Register";


import Dashboard from "../features/dashboard/Dashboard";

function AppRoutes() {
  return (
    <Routes>
      <Route path="/" element={<AutoPage />} />
      <Route path="/login" element={<Login />} />
      <Route path="/restore" element={<Register />} />
       <Route path="/dashboard" element={<Dashboard />} />
    </Routes>
  );
}

export default AppRoutes;