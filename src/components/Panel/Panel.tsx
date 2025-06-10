import { ParentComponent } from "solid-js";
import styles from "./panel.module.css";

interface PanelProps {
    maxWidth?: number;
    maxHeight?: number;
}

const Panel: ParentComponent<PanelProps> = (props) => {
    const maxWidth = props.maxWidth !== undefined ? `${props.maxWidth}px` : ``;
    const maxHeight = props.maxHeight !== undefined ? `${props.maxHeight}px` : ``;

    return (
        <div class={styles.panel} style={{ "max-width": maxWidth, "max-height": maxHeight }}>
            {props.children}
        </div>
    );
};

export default Panel;
