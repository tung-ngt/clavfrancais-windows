import { Component } from "solid-js";
import styles from "./logo.module.css"

const Logo: Component<{}> = () => {
    return (
        <span class={styles.logoContainer}>
            <img class={styles.logo} src="/logo.png" alt="Clavfrancais" />
            <h1 class={styles.title}>Clavfrançais</h1>
        </span>
    );
};

export default Logo;
