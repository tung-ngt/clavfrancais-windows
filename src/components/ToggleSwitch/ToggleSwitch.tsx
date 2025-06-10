import { Component } from "solid-js";
import styles from "./toggle-switch.module.css";

interface ToggleSwitchProps {
    checked: boolean;
    name: string;
    onToggle: () => void;
}

const ToggleSwitch: Component<ToggleSwitchProps> = (props) => {
    return (
        <label class={styles.container}>
            {props.name}
            <span class={styles.switch}>
                <input
                    class={styles.input}
                    type="checkbox"
                    checked={props.checked}
                    onchange={props.onToggle}
                />
                <span class={styles.slider} />
            </span>
        </label>
    );
};

export default ToggleSwitch;
