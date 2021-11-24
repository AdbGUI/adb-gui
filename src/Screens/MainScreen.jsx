import { Sidebar } from '../Components/SideBar/SideBar';
import HomeScreen from './HomeScreen';
import '../Styles/Main.css';
import { useParams } from 'react-router-dom';

const MainScreen = () => {
    const { pageName } = useParams();
    const getPage = () => {
        console.log(pageName);
        switch (pageName) {
            case 'home':
                return <HomeScreen />;
            case 'apps':
                // return <AppsScreen />;
                break;
            case 'settings':
                // return <SettingsScreen />;
                break;
            default:
                return <HomeScreen />;
        }
    };
    return (
        <div className="main">
            <Sidebar />
            {getPage()}
        </div>
    );
};

export default MainScreen;
