import "./App.css";
import MainAppLayout from "./components/MainAppLayout/MainAppLayout";

function App() {
    document.addEventListener("contextmenu", (e) => {
        e.preventDefault();
    });
    return <MainAppLayout />;
}

export default App;
