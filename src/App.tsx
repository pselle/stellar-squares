import "./App.module.css";
import { Routes, Route, Outlet } from "react-router-dom";
import Home from "./pages/Home";
import Home2 from "./pages/v2";

const AppLayout: React.FC = () => <Outlet />;

function App() {
  return (
    <Routes>
      <Route element={<AppLayout />}>
        <Route path="/" element={<Home />} />
        <Route path="/v2" element={<Home2 />} />
      </Route>
    </Routes>
  );
}

export default App;
