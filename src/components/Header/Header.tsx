import { Component } from "solid-js";
import styles from "./header.module.css";
import Logo from "../Logo/Logo";
import QuitIcon from "../QuitIcon/QuitIcon";
import { invoke } from "@tauri-apps/api/core";

const Header: Component<{}> = () => {
    const quit = () => {
        invoke("quit_command");
    }
    return (
        <div class={styles.header}>
            <Logo />
            <button class={styles.quitButton} onclick={quit}>
                <QuitIcon width={24} height={24} color="#ff4b55"/>
            </button>
        </div>
    );
};

export default Header;
