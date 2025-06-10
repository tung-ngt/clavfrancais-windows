import { Component } from "solid-js";
import Header from "../Header/Header";
import LanguagePicker from "../LanguagePicker/LanguagePicker";
import Settings from "../Settings/Settings";
import styles from "./main-app-layout.module.css"
import Footer from "../Footer/Footer";

const MainAppLayout: Component<{}> = () => {
    return (
        <main class={styles.main}>
            <Header />
            <LanguagePicker />
            <Settings />
            <Footer />
        </main>
    );
};

export default MainAppLayout;
