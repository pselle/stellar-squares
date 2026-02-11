import "./App.module.css";
import { Routes, Route, Outlet } from "react-router-dom";
import Home from "./pages/Home";

const AppLayout: React.FC = () => <Outlet />;

function App() {
  return (
    <Routes>
      <Route element={<AppLayout />}>
        <Route path="/" element={<Home />} />
      </Route>
    </Routes>
  );
}

export default App;
