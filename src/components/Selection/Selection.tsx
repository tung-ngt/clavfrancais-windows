import { Component, For, createSignal } from "solid-js";
import styles from "./selection.module.css";

interface SelectionProps {
    name: string;
    value: string;
    options: string[];
    onchange: (option: string) => void;
}

const Selection: Component<SelectionProps> = (props) => {
    const [open, setOpen] = createSignal(false);
    let dialogElement: HTMLDialogElement;

    const handleDropdown = () => {
        setOpen(!open());
    };

    const handleChangeOption = (option: string) => {
        dialogElement.close();
        if (option !== props.value) {
            props.onchange(option);
        }
    };

    return (
        <div class={styles.container}>
            {props.name}
            <button class={styles.dropdownButton} onclick={handleDropdown}>
                {props.value}
                <dialog class={styles.dropdown} ref={(el) => (dialogElement = el)} open={open()}>
                    <ul class={styles.list}>
                        <For each={props.options}>
                            {(option) => (
                                <li onclick={[handleChangeOption, option]} class={styles.item}>
                                    {option}
                                </li>
                            )}
                        </For>
                    </ul>
                </dialog>
            </button>
        </div>
    );
};

export default Selection;
