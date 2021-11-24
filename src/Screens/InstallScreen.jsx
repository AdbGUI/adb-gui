import { useEffect, useState } from 'react';
import { Button, TextField } from '@mui/material';
import { AiOutlineArrowRight } from 'react-icons/ai'
import { open as openFileDialog } from '@tauri-apps/api/dialog';
import { event } from '@tauri-apps/api';
import '../Styles/InstallScreen.css';

// import images
import Bg01 from '../Assets/images/bg-01.jpg';
import Bg02 from '../Assets/images/bg-02.jpg';
import Bg03 from '../Assets/images/bg-03.jpg';

const SliderImages = ({ img }) => {
    return (
        <div className="slider-images">
            <div style={{ backgroundImage: `url(${img})` }} className="slider-images-item" />
            <div className="blur" />
        </div>
    )
}

const SelectPath = () => {
    const [path, setPath] = useState("");
    useEffect(() => {
        event.listen('install-path', (path) => {
            setPath(path.payload);
        });
    }, [path]);
    const openFileDialogHandler = async () => {
        return openFileDialog({
            directory: true,
            multiple: false
        }).then((filePath) => {
            if (filePath) {
                console.log(filePath);
                if (filePath.length > 0 && typeof (filePath) === "string")
                    return filePath;
                else if (filePath.length > 0 && typeof (filePath) === "object")
                    return filePath[0];
            }
            return '';
        })
    }
    return (
        <div className="install-path">
            <h3 className="install-path-title">path to install:</h3>
            <div className="install-path-input-container">
                <TextField
                    id="path-selection"
                    className="install-path-input"
                    label=""
                    variant="standard"
                    defaultValue=""
                    value={path}/>
                <Button className="install-path-input-button" onClick={async () => { setPath(await openFileDialogHandler()); }}
                    variant="contained"> select </Button>
            </div>
        </div>
    )
}

const NextButton = ({ showRounded = false }) => {
    return (
        <div className="next-button-container">
            {showRounded ?
                <Button className="next-button"
                    variant="contained"
                    size="large"
                >Continue<AiOutlineArrowRight className="next-button-icon" /></Button>
                :
                <Button className="next-button"
                    variant="contained"
                    size="large"
                >Continue<AiOutlineArrowRight className="next-button-icon" /></Button>
            }
        </div>
    )
}

const installSteps = [
    {
        title: 'Install Options',
        img: Bg01,
        element: <SelectPath />
    },
    {
        title: 'Install Options',
        img: Bg02,
        element: <SelectPath />
    },
    {
        title: 'Installing...',
        img: Bg03,
        element: <SelectPath />
    }
];

const InstallScreen = () => {

    const [installStep, setInstallStep] = useState(0);

    return (
        <div className="install-container">
            <header className="install-header">
                <SliderImages img={installSteps[installStep].img} />
                <h1 className="install-header-title">{installSteps[installStep].title}</h1>
            </header>
            <div className="install-content">
                {installSteps[installStep].element}
                <NextButton />
            </div>
        </div>
    )
}

export default InstallScreen;
