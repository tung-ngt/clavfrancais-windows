import { Component } from "solid-js";
import styles from "./flag-button.module.css";

interface FlagButtonProps {
    name: string;
    imagePath: string;
    isActive: boolean;
    onClick: () => void;
}

const FlagButton: Component<FlagButtonProps> = (props) => {
    return (
        <button
            classList={{
                [styles.button]: true,
                [styles.active]: props.isActive,
            }}
            onclick={props.onClick}
        >
            <span class={styles.tooltiptext}>{props.name}</span>
            <img class={styles.img} src={props.imagePath} alt={props.name} />
        </button>
    );
};

export default FlagButton;
