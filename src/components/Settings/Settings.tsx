import { Component, createEffect, createSignal } from "solid-js";
import Panel from "../Panel/Panel";
import ToggleSwitch from "../ToggleSwitch/ToggleSwitch";
import styles from "./settings.module.css";
import Selection from "../Selection/Selection";
import { invoke } from "@tauri-apps/api/core";
import RoundedButton from "../RoundedButton/RoundedButton";

enum ToggleShortcut {
    CtrlAlt = "CtrlAlt",
    AltZ = "AltZ",
}

interface Settings {
    runOnStartup: boolean;
    hideToTray: boolean;
    toggleShortcut: ToggleShortcut;
}

const toggleOptions = [
    ToggleShortcut.CtrlAlt,
    // ToggleShortcut.AltZ
];

const Settings: Component<{}> = () => {
    const [initialSettings, setInitialSettings] = createSignal<Settings>({
        runOnStartup: false,
        hideToTray: false,
        toggleShortcut: toggleOptions[0],
    });
    const [runOnStartUp, setRunOnStartUp] = createSignal(false);
    const [hideToTray, setHideToTray] = createSignal(false);
    const [toggleOption, setToggleOption] = createSignal(toggleOptions[0]);
    const [settingsChanged, setSettingsChanged] = createSignal(false);

    createEffect(async () => {
        const settings = await invoke<Settings>("get_settings_command");
        setInitialSettings(settings);
        setRunOnStartUp(settings.runOnStartup);
        setHideToTray(settings.hideToTray);
        setToggleOption(settings.toggleShortcut);
    });

    const saveSettings = () => {
        const settings = {
            runOnStartup: runOnStartUp(),
            hideToTray: hideToTray(),
            toggleShortcut: toggleOption(),
        };
        invoke("set_settings_command", { settings });
        setInitialSettings(settings);
        setSettingsChanged(false);
    };

    const cancelSettings = () => {
        let intial = initialSettings();
        setRunOnStartUp(intial.runOnStartup);
        setHideToTray(intial.hideToTray);
        setToggleOption(intial.toggleShortcut);
        setSettingsChanged(false);
    };

    const toggleRunOnStartUp = () => {
        setRunOnStartUp(!runOnStartUp());
        setSettingsChanged(true);
    };

    const toggleHideToTray = () => {
        setHideToTray(!hideToTray());
        setSettingsChanged(true);
    };

    const onChangeToggleOption = (option: string) => {
        setToggleOption(option as ToggleShortcut);
        setSettingsChanged(true);
    };

    return (
        <section class={styles.section}>
            <h2 class={styles.title}>Settings</h2>

            <Panel maxHeight={300} maxWidth={500}>
                <div class={styles.container}>
                    <ToggleSwitch
                        name="Run on startup"
                        onToggle={toggleRunOnStartUp}
                        checked={runOnStartUp()}
                    />
                    <ToggleSwitch
                        name="Hide to tray on start"
                        onToggle={toggleHideToTray}
                        checked={hideToTray()}
                    />
                    <Selection
                        name="Toggle language"
                        value={toggleOption()}
                        onchange={onChangeToggleOption}
                        options={toggleOptions}
                    />
                    <span>
                        <RoundedButton
                            disable={!settingsChanged()}
                            onClick={saveSettings}
                            text="Save"
                            variant="primary"
                        />
                        <RoundedButton
                            disable={!settingsChanged()}
                            onClick={cancelSettings}
                            text="Cancel"
                            variant="secondary"
                        />
                    </span>
                </div>
            </Panel>
        </section>
    );
};

export default Settings;
