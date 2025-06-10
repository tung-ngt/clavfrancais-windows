import { Component } from "solid-js";
import styles from "./rounded-button.module.css";

interface RoundedButtonProps {
    onClick: () => void;
    text: string;
    variant: "primary" | "secondary";
    disable?: boolean
}

const RoundedButton: Component<RoundedButtonProps> = (props) => {
    return (
        <button
            classList={{
                [styles.button]: true,
                [styles.primary]: props.variant == "primary",
                [styles.secondary]: props.variant == "secondary",
                [styles.disable]: props.disable,
            }}
            onclick={() => !props.disable && props.onClick()}
        >
            {props.text}
        </button>
    );
};

export default RoundedButton;
