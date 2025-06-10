import { Component } from "solid-js";
import styles from "./footer.module.css";
import Panel from "../Panel/Panel";
import LinkIcon from "../LinkIcon/LinkIcon";

const Footer: Component<{}> = () => {
    return (
        <section class={styles.section}>
            <h2 class={styles.title}>Credit</h2>
            <Panel maxWidth={500}>
                <div class={styles.creditContainer}>
                    <p>Nguyễn Thanh Tùng</p>
                    <a
                        class={styles.link}
                        href="https://github.com/tung-ngt/clavfrancais-engine"
                        target="_blank"
                    >
                        Engine source code
                        <LinkIcon width={16} height={16} color="black" />
                    </a>
                    <a
                        class={styles.link}
                        href="https://github.com/tung-ngt/clavfrancais-engine"
                        target="_blank"
                    >
                        GUI source code
                        <LinkIcon width={16} height={16} color="black"/>
                    </a>
                    <p>tung.ngt.comp@gmail.com</p>
                </div>
            </Panel>
        </section>
    );
};

export default Footer;
