import { useEffect, useState } from 'react';
import '../Styles/SplashScreen.css'

// Components
import Fade from '../Components/Texts/Fade';

const tips = [
    "Not click the android button o.O",
    "Customize me like you want",
    "I'm a little bit slow",
    "I am supposed to be a tip",
];

const tipDuration = 4000;

const SplashScreen = ({ enableTips = true }) => {
    const [tip, setTip] = useState(tips[0]);

    useEffect(() => {
        if (enableTips) {
            const timeout = setInterval(() => {
                // Get random tip with no duplicates
                const randomTip = tips[Math.floor(Math.random() * tips.length)];
                if (randomTip !== tip) {
                    setTip(randomTip);
                }
            }, tipDuration);
            return () => clearInterval(timeout);
        }
    }, [tip, enableTips]);

    return (
        <div className="splash-screen">
            <div className="splash-screen-content">
                <div className="splash-screen-logo">
                    <svg width="30%" height="30%" viewBox="0 0 192 192" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <path className="android-path" d="M175.604 144H16.4C20.408 103.572 54.516 72 96 72C137.48 72 171.592 103.572 175.604 144V144Z" stroke="#494949" strokeWidth="4" strokeLinejoin="round" />
                        <path className="android-path" d="M56 79.9999L40 51.9999" stroke="#494949" strokeWidth="4" strokeLinecap="round" strokeLinejoin="round" />
                        <path className="android-path" d="M132 79.9999L148 51.9999" stroke="#494949" strokeWidth="4" strokeLinecap="round" strokeLinejoin="round" />
                        <path className="android-path" d="M60 124C64.4182 124 68 120.418 68 116C68 111.582 64.4182 108 60 108C55.5817 108 52 111.582 52 116C52 120.418 55.5817 124 60 124Z" fill="#494949" />
                        <path className="android-path" d="M132 124C136.418 124 140 120.418 140 116C140 111.582 136.418 108 132 108C127.582 108 124 111.582 124 116C124 120.418 127.582 124 132 124Z" fill="#494949" />
                    </svg>
                </div>
                {enableTips &&
                    <Fade className="splash-screen-text" content={tip} timeWait={(tipDuration / 2)} />
                }
            </div>
        </div>
    );
};

export default SplashScreen;
