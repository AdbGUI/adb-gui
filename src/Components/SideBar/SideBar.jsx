import './SideBar.css';

import { GrAppsRounded, GrHomeRounded } from "react-icons/gr";
import { IoSettingsOutline } from "react-icons/io5";
import { Link } from 'react-router-dom';
import { IconContext } from 'react-icons/lib';

const SidebarButton = ({children, to}) => {
    return (
        <Link to={to} className="side-button">
            <IconContext.Provider value={{ className: "side-button-icon" }}>
                {children}
            </IconContext.Provider>
        </Link>
    );
}

const Sidebar = () => {
    return (
        <div className="sidebar">
            <SidebarButton to="/">
                <GrHomeRounded />
            </SidebarButton>
            <SidebarButton to="/apps">
                <GrAppsRounded />
            </SidebarButton>
            <SidebarButton to="/settings">
                <IoSettingsOutline />
            </SidebarButton>
        </div>
    );
}

export {
    Sidebar,
    SidebarButton,
}
