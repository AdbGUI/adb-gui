import { useState } from 'react';
import { IoCheckmark } from "react-icons/io5";
import './ToggleCheckbox.css';

const ToggleCheckbox = ({ checked, onChange, className, label }) => {
    const [checkedState, setCheckedState] = useState(checked);
    return (
        <div className={`toggle-check ${className} ${checkedState ? "toggle-checked" : ""}`} onClick={() => { setCheckedState(!checkedState); onChange(!checkedState) }}>
            <div className="toggle-check-container-check" onClick={() => { setCheckedState(!checkedState); onChange(!checkedState) }}>
                {checkedState &&
                    <IoCheckmark className="toggle-check-icon"/>
                }
            </div>
            <label className="toggle-check-label">{label}</label>
        </div>
    );
}

export default ToggleCheckbox;
