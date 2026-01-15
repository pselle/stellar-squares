import { Layout } from "@stellar/design-system";
import "./App.module.css";
import ConnectAccount from "./components/ConnectAccount.tsx";
import { Routes, Route, Outlet } from "react-router-dom";
import Home from "./pages/Home";

const AppName = "Important Diagrams";
const AppLayout: React.FC = () => (
  <main>
    <Layout.Header
      projectId={AppName}
      projectTitle={AppName}
      contentRight={
        <>
          <ConnectAccount />
        </>
      }
    />
    <Outlet />
    <Layout.Footer hideLegalLinks={true}>
      <span>
        © {new Date().getFullYear()} {AppName}. Licensed under the{" "}
        <a
          href="http://www.apache.org/licenses/LICENSE-2.0"
          target="_blank"
          rel="noopener noreferrer"
        >
          Apache License, Version 2.0
        </a>
        .
      </span>
    </Layout.Footer>
  </main>
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
