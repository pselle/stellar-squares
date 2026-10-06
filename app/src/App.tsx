import "./App.module.css";
import { Routes, Route, Outlet } from "react-router-dom";
import Home from "./pages/Home";
import styles from "./App.module.css";

const AppLayout: React.FC = () => (
  <>
    <div className={styles.backgroundPattern}></div>
    <Outlet />
  </>
);

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
