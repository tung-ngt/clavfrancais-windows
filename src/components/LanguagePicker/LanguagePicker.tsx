import { Component, createEffect, createSignal, onCleanup } from "solid-js";
import styles from "./language-picker.module.css";
import FlagButton from "../FlagButton/FlagButton";
import Panel from "../Panel/Panel";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

enum Language {
    English = "English",
    French = "French",
}
const LanguagePicker: Component<{}> = () => {
    const [language, setLanguage] = createSignal<Language>(Language.English);

    createEffect(async () => {
        const value: string = await invoke("get_language_command");
        setLanguage(value as Language);
    });

    const eventHandler = async (event: KeyboardEvent) => {
        if (event.altKey && event.ctrlKey) {
            invoke("toggle_language_command");
        }
    };

    document.addEventListener("keydown", eventHandler); 

    const handleLanguageChange = (new_language: Language) => {
        if (new_language !== language()) {
            invoke("change_language_command", { language: new_language });
        }
    };

    listen("change_language", (event) => {
        const language = event.payload as Language;
        setLanguage(language);
    });

    return (
        <section class={styles.section}>
            <Panel maxWidth={500} maxHeight={300}>
                <div class={styles.flagContainer}>
                    <FlagButton
                        name="French"
                        imagePath="/france.png"
                        isActive={language() == Language.French}
                        onClick={() => handleLanguageChange(Language.French)}
                    ></FlagButton>
                    <FlagButton
                        name="English"
                        imagePath="/uk.png"
                        isActive={language() == Language.English}
                        onClick={() => handleLanguageChange(Language.English)}
                    ></FlagButton>
                </div>
            </Panel>
        </section>
    );
};

export default LanguagePicker;
